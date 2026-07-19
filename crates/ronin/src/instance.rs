//! Single-instance enforcement and Unix-socket IPC for CLI intent routing.
//!
//! Primary process holds an exclusive lock under the runtime directory and
//! listens on a Unix domain socket. Secondary processes connect, send their
//! [`LaunchIntent`], request focus, and exit. Stale sockets left after a crash
//! are cleaned up when a new primary acquires the lock.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use crate::{LaunchIntent, LauncherError};

const LOCK_NAME: &str = "ronin.lock";
const SOCK_NAME: &str = "ronin.sock";
const IPC_MAGIC: &str = "ronin-ipc-v1";

/// Errors from single-instance lock / IPC operations.
#[derive(Debug, thiserror::Error)]
pub enum InstanceError {
    /// Filesystem or socket I/O failed.
    #[error("instance ipc error: {0}")]
    Io(#[from] std::io::Error),

    /// Lock could not be acquired or released.
    #[error("instance lock error: {0}")]
    Lock(String),

    /// Wire protocol was invalid or incomplete.
    #[error("invalid ipc message: {0}")]
    Protocol(String),
}

/// Outcome of attempting to become the single Ronin instance.
pub enum InstanceAcquire {
    /// This process owns the lock and should run the UI / accept IPC.
    Primary(InstancePrimary),
    /// Intent was delivered to the running instance; this process should exit.
    HandedOff,
}

impl InstanceAcquire {
    /// Whether this process is the primary (UI) instance.
    pub fn is_primary(&self) -> bool {
        matches!(self, Self::Primary(_))
    }

    /// Whether the intent was handed off to an existing instance.
    pub fn is_handed_off(&self) -> bool {
        matches!(self, Self::HandedOff)
    }

    /// Returns the primary handle when this process owns the instance.
    pub fn into_primary(self) -> Option<InstancePrimary> {
        match self {
            Self::Primary(p) => Some(p),
            Self::HandedOff => None,
        }
    }
}

/// Intent delivered from a secondary process to the primary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingLaunch {
    /// CLI intent to apply in the running instance.
    pub intent: LaunchIntent,
    /// Whether the primary should raise/focus its window.
    pub focus: bool,
}

/// Primary instance: holds the lock file and receives IPC on a background thread.
pub struct InstancePrimary {
    _lock_file: File,
    incoming_rx: mpsc::Receiver<IncomingLaunch>,
    /// When dropped / set, the accept loop exits.
    shutdown_tx: Option<mpsc::Sender<()>>,
    sock_path: PathBuf,
}

impl InstancePrimary {
    /// Non-blocking poll for a secondary launch intent.
    pub fn try_recv(&mut self) -> Result<Option<IncomingLaunch>, InstanceError> {
        match self.incoming_rx.try_recv() {
            Ok(incoming) => Ok(Some(incoming)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Ok(None),
        }
    }

    /// Path to the Unix socket used for IPC (tests / diagnostics).
    pub fn socket_path(&self) -> &Path {
        &self.sock_path
    }
}

impl Drop for InstancePrimary {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        let _ = fs::remove_file(&self.sock_path);
        // flock is released when `_lock_file` is closed.
    }
}

/// Attempts to become the primary instance, or routes `intent` to the existing one.
///
/// `runtime_dir` is typically `$XDG_RUNTIME_DIR/ronin` (or a test temp path).
pub fn acquire_instance(
    runtime_dir: &Path,
    intent: &LaunchIntent,
) -> Result<InstanceAcquire, InstanceError> {
    fs::create_dir_all(runtime_dir)?;
    let lock_path = runtime_dir.join(LOCK_NAME);
    let sock_path = runtime_dir.join(SOCK_NAME);

    let lock_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)?;

    match try_flock_exclusive(&lock_file) {
        Ok(()) => {
            // We own the lock. Remove any stale socket from a crashed primary.
            let _ = fs::remove_file(&sock_path);
            let listener = UnixListener::bind(&sock_path)?;
            listener.set_nonblocking(false)?;

            let (incoming_tx, incoming_rx) = mpsc::channel();
            let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
            let accept_sock = sock_path.clone();

            std::thread::spawn(move || {
                // Wake accept loop by connecting to ourselves on shutdown.
                loop {
                    if shutdown_rx.try_recv().is_ok() {
                        break;
                    }
                    listener.set_nonblocking(true).ok();
                    match listener.accept() {
                        Ok((mut stream, _)) => {
                            listener.set_nonblocking(false).ok();
                            stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
                            match read_incoming(&mut stream) {
                                Ok(incoming) => {
                                    let _ = writeln!(stream, "ok");
                                    if incoming_tx.send(incoming).is_err() {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!(error = %e, "failed to read ipc intent");
                                }
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            if shutdown_rx.try_recv().is_ok() {
                                break;
                            }
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(_) => break,
                    }
                }
                let _ = accept_sock;
            });

            Ok(InstanceAcquire::Primary(InstancePrimary {
                _lock_file: lock_file,
                incoming_rx,
                shutdown_tx: Some(shutdown_tx),
                sock_path,
            }))
        }
        Err(e) if is_would_block(&e) => {
            hand_off_intent(&sock_path, intent)?;
            Ok(InstanceAcquire::HandedOff)
        }
        Err(e) => Err(e),
    }
}

/// Resolves the default runtime directory for Ronin IPC (`$XDG_RUNTIME_DIR/ronin`).
pub fn instance_runtime_dir() -> Result<PathBuf, LauncherError> {
    let base = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".ronin-runtime"))
                .map_err(|_| LauncherError::MissingHome)
        })?;
    Ok(base.join("ronin"))
}

fn try_flock_exclusive(file: &File) -> Result<(), InstanceError> {
    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if rc == 0 {
        Ok(())
    } else {
        let err = std::io::Error::last_os_error();
        if err.kind() == std::io::ErrorKind::WouldBlock
            || err.raw_os_error() == Some(libc::EWOULDBLOCK)
            || err.raw_os_error() == Some(libc::EAGAIN)
        {
            Err(InstanceError::Lock("would block".into()))
        } else {
            Err(InstanceError::Lock(err.to_string()))
        }
    }
}

fn is_would_block(err: &InstanceError) -> bool {
    matches!(err, InstanceError::Lock(msg) if msg.contains("would block"))
        || matches!(err, InstanceError::Io(e) if e.kind() == std::io::ErrorKind::WouldBlock)
}

fn hand_off_intent(sock_path: &Path, intent: &LaunchIntent) -> Result<(), InstanceError> {
    let mut last_err = None;
    for _ in 0..40 {
        match UnixStream::connect(sock_path) {
            Ok(mut stream) => {
                stream.set_write_timeout(Some(Duration::from_secs(2)))?;
                stream.set_read_timeout(Some(Duration::from_secs(2)))?;
                write_incoming(&mut stream, intent, true)?;
                stream.shutdown(std::net::Shutdown::Write)?;
                let mut ack = String::new();
                let _ = stream.read_to_string(&mut ack);
                return Ok(());
            }
            Err(e) => {
                last_err = Some(e);
                std::thread::sleep(Duration::from_millis(25));
            }
        }
    }
    Err(InstanceError::Io(last_err.unwrap_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "could not connect to primary ronin instance",
        )
    })))
}

fn write_incoming(
    stream: &mut UnixStream,
    intent: &LaunchIntent,
    focus: bool,
) -> Result<(), InstanceError> {
    let (kind, paths) = match intent {
        LaunchIntent::OpenPersisted { attach_paths } => ("open", attach_paths.as_slice()),
        LaunchIntent::NewThread { attach_paths } => ("new", attach_paths.as_slice()),
        LaunchIntent::OpenWithOllama { attach_paths } => ("ollama", attach_paths.as_slice()),
        LaunchIntent::Quick { attach_paths } => ("quick", attach_paths.as_slice()),
    };
    writeln!(stream, "{IPC_MAGIC}")?;
    writeln!(stream, "{kind}")?;
    writeln!(stream, "{}", if focus { "focus" } else { "nofocus" })?;
    for path in paths {
        writeln!(stream, "{}", path.display())?;
    }
    writeln!(stream, ".")?;
    stream.flush()?;
    Ok(())
}

fn read_incoming(stream: &mut UnixStream) -> Result<IncomingLaunch, InstanceError> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;
    let mut lines = buf.lines();
    let magic = lines
        .next()
        .ok_or_else(|| InstanceError::Protocol("missing magic".into()))?;
    if magic != IPC_MAGIC {
        return Err(InstanceError::Protocol(format!("bad magic: {magic}")));
    }
    let kind = lines
        .next()
        .ok_or_else(|| InstanceError::Protocol("missing kind".into()))?;
    let focus_line = lines
        .next()
        .ok_or_else(|| InstanceError::Protocol("missing focus".into()))?;
    let focus = focus_line == "focus";

    let mut attach_paths = Vec::new();
    for line in lines {
        if line == "." {
            break;
        }
        if !line.is_empty() {
            attach_paths.push(PathBuf::from(line));
        }
    }

    let intent = match kind {
        "open" => LaunchIntent::OpenPersisted { attach_paths },
        "new" => LaunchIntent::NewThread { attach_paths },
        "ollama" => LaunchIntent::OpenWithOllama { attach_paths },
        "quick" => LaunchIntent::Quick { attach_paths },
        other => {
            return Err(InstanceError::Protocol(format!("unknown kind: {other}")));
        }
    };

    Ok(IncomingLaunch { intent, focus })
}

/// Describes how a primary instance should apply a remote launch intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedIntent {
    /// Create and select a new thread.
    pub create_new_thread: bool,
    /// Open the compact quick-mode overlay (not the full shell chrome).
    pub open_quick_overlay: bool,
    /// Prefer the Ollama provider path when opening.
    pub prefer_ollama: bool,
    /// Paths to attach into the composer / pending attachments.
    pub attach_paths: Vec<PathBuf>,
    /// Raise/focus the application window.
    pub focus_window: bool,
}

/// Maps an incoming IPC launch to concrete primary-side actions.
pub fn plan_incoming_launch(incoming: &IncomingLaunch) -> AppliedIntent {
    match &incoming.intent {
        LaunchIntent::OpenPersisted { attach_paths } => AppliedIntent {
            create_new_thread: !attach_paths.is_empty(),
            open_quick_overlay: false,
            prefer_ollama: false,
            attach_paths: attach_paths.clone(),
            focus_window: incoming.focus,
        },
        LaunchIntent::NewThread { attach_paths } => AppliedIntent {
            create_new_thread: true,
            open_quick_overlay: false,
            prefer_ollama: false,
            attach_paths: attach_paths.clone(),
            focus_window: incoming.focus,
        },
        LaunchIntent::Quick { attach_paths } => AppliedIntent {
            create_new_thread: false,
            open_quick_overlay: true,
            prefer_ollama: false,
            attach_paths: attach_paths.clone(),
            focus_window: incoming.focus,
        },
        LaunchIntent::OpenWithOllama { attach_paths } => AppliedIntent {
            create_new_thread: false,
            open_quick_overlay: false,
            prefer_ollama: true,
            attach_paths: attach_paths.clone(),
            focus_window: incoming.focus,
        },
    }
}
