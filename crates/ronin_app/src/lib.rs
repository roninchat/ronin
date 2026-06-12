#![deny(missing_docs)]

//! UI-facing application shell state for Ronin.

use ronin_core::{RoninError, RoninPaths, RoninSession, Thread};

/// Result type returned by `ronin_app` operations.
pub type Result<T> = std::result::Result<T, RoninAppError>;

/// Errors returned by Ronin's UI-facing app boundary.
#[derive(Debug, thiserror::Error)]
pub enum RoninAppError {
    /// Ronin session operation failed.
    #[error(transparent)]
    Session(#[from] RoninError),

    /// Requested thread is not loaded in shell state.
    #[error("thread {thread_id} is not loaded")]
    ThreadNotLoaded {
        /// Thread id requested by the UI.
        thread_id: String,
    },
}

/// Basic provider/model status shown in the sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderStatus {
    /// No provider or model is configured yet.
    NotConfigured,
}

/// Snapshot of state rendered by the native shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellState {
    /// Native window title.
    pub window_title: String,
    /// Persisted threads visible in sidebar.
    pub threads: Vec<Thread>,
    /// Currently selected thread id, if any.
    pub selected_thread_id: Option<String>,
    /// Sidebar provider/model status.
    pub provider_status: ProviderStatus,
}

/// UI-facing shell controller backed by `RoninSession` behavior.
pub struct RoninShell {
    session: RoninSession,
    state: ShellState,
}

impl RoninShell {
    /// Opens the shell state from persisted Ronin paths.
    pub fn open(paths: RoninPaths) -> Result<Self> {
        let session = RoninSession::open(paths)?;
        let mut threads = session.list_threads()?;
        if threads.is_empty() {
            threads.push(session.create_thread()?);
            tracing::info!("ronin shell created initial thread");
        }
        let selected_thread_id = threads.first().map(|thread| thread.id.clone());
        tracing::info!(thread_count = threads.len(), "ronin shell state restored");

        Ok(Self {
            session,
            state: ShellState {
                window_title: "Ronin".to_string(),
                threads,
                selected_thread_id,
                provider_status: ProviderStatus::NotConfigured,
            },
        })
    }

    /// Creates a new thread from the sidebar action and selects it.
    pub fn create_new_thread(&mut self) -> Result<Thread> {
        let thread = self.session.create_thread()?;
        self.state.selected_thread_id = Some(thread.id.clone());
        self.state.threads.push(thread.clone());
        tracing::info!(thread_id = %thread.id, "ronin shell created and selected thread");
        Ok(thread)
    }

    /// Selects a loaded thread from the sidebar.
    pub fn select_thread(&mut self, thread_id: &str) -> Result<()> {
        let exists = self
            .state
            .threads
            .iter()
            .any(|thread| thread.id.as_str() == thread_id);
        if !exists {
            return Err(RoninAppError::ThreadNotLoaded {
                thread_id: thread_id.to_string(),
            });
        }

        self.state.selected_thread_id = Some(thread_id.to_string());
        tracing::info!(thread_id, "ronin shell selected thread");
        Ok(())
    }

    /// Returns current shell state.
    pub fn state(&self) -> &ShellState {
        &self.state
    }
}
