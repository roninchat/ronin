#![deny(missing_docs)]

//! Public application/session boundary for Ronin.

use std::fs;
use std::io;
use std::path::PathBuf;

use ronin_db::{init_tracing, DbThread, RoninDb, RoninDbError};

/// Result type returned by `ronin_core` operations.
pub type Result<T> = std::result::Result<T, RoninError>;

/// Errors returned by Ronin's public session boundary.
#[derive(Debug, thiserror::Error)]
pub enum RoninError {
    /// Ronin could not create or access its configuration directory.
    #[error("failed to create Ronin config directory at {path}")]
    CreateConfigDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin could not create or access its data directory.
    #[error("failed to create Ronin data directory at {path}")]
    CreateDataDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin's SQLite persistence layer failed.
    #[error(transparent)]
    Db(#[from] RoninDbError),
}

/// Filesystem locations used by a Ronin app session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoninPaths {
    /// Directory for user-editable or generated configuration files.
    pub config_dir: PathBuf,
    /// Directory for local application data such as `ronin.db`.
    pub data_dir: PathBuf,
}

/// User-visible conversation thread metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    /// App-generated opaque thread identifier.
    pub id: String,
    /// User-visible title. New M0 threads start as `New Chat`.
    pub title: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
    /// Whether the thread is archived. M0 stores this but has no UI for it yet.
    pub archived: bool,
}

/// Open Ronin application session backed by local filesystem state.
pub struct RoninSession {
    db: RoninDb,
}

impl RoninSession {
    /// Opens a Ronin session against the provided paths.
    ///
    /// Creates the config and data directories when they do not already exist,
    /// opens `ronin.db` in the data directory, and applies pending migrations.
    pub fn open(paths: RoninPaths) -> Result<Self> {
        init_tracing();
        tracing::info!("opening ronin session");
        fs::create_dir_all(&paths.config_dir).map_err(|source| RoninError::CreateConfigDir {
            path: paths.config_dir.clone(),
            source,
        })?;
        tracing::info!(config_dir = %paths.config_dir.display(), "ronin config directory ready");

        fs::create_dir_all(&paths.data_dir).map_err(|source| RoninError::CreateDataDir {
            path: paths.data_dir.clone(),
            source,
        })?;
        tracing::info!(data_dir = %paths.data_dir.display(), "ronin data directory ready");

        let db = RoninDb::open(paths.data_dir.join("ronin.db"))?;
        Ok(Self { db })
    }

    /// Creates a new user-visible thread titled `New Chat` and persists it.
    pub fn create_thread(&self) -> Result<Thread> {
        self.db
            .create_thread()
            .map(Thread::from)
            .map_err(Into::into)
    }

    /// Lists persisted threads in stable creation order.
    pub fn list_threads(&self) -> Result<Vec<Thread>> {
        self.db
            .list_threads()
            .map(|threads| threads.into_iter().map(Thread::from).collect())
            .map_err(Into::into)
    }
}

impl From<DbThread> for Thread {
    fn from(thread: DbThread) -> Self {
        Self {
            id: thread.id,
            title: thread.title,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            archived: thread.archived,
        }
    }
}
