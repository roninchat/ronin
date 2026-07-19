//! Typed errors for Ronin's UI-facing app boundary.

use ronin_core::RoninError;

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

    /// Generation is already in progress.
    #[error("generation in progress")]
    GenerationInProgress,

    /// Action cannot be performed on the target message.
    #[error("invalid message for action")]
    InvalidMessage,

    /// Thread title is empty or whitespace-only.
    #[error("thread title must not be empty")]
    InvalidThreadTitle,
}
