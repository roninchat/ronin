//! Typed errors for Ronin's session boundary.

use std::io;
use std::path::PathBuf;

use ronin_db::RoninDbError;

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

    /// Provider operation failed.
    #[error("provider error: {0}")]
    Provider(String),

    /// Ronin configuration read/write failed.
    #[error("config error: {0}")]
    Config(String),

    /// Workspace root path is missing or not a directory.
    #[error("workspace root must be an existing directory: {path}")]
    InvalidWorkspaceRoot {
        /// Path the caller attempted to bind.
        path: PathBuf,
    },

    /// Never-list / allowlist path is missing or not a directory.
    #[error("privacy path must be an existing directory: {path}")]
    InvalidPrivacyPath {
        /// Path the caller attempted to register.
        path: PathBuf,
    },

    /// Lexical workspace index operation failed.
    #[error("workspace index error: {0}")]
    WorkspaceIndex(String),

    /// Internal lock for workspace index cancel flags was poisoned.
    #[error("workspace index cancel lock poisoned")]
    WorkspaceIndexCancelLock,
}
