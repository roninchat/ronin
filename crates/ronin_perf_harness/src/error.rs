//! Harness errors.

use thiserror::Error;

/// Errors from the Perf Harness runner.
#[derive(Debug, Error)]
pub enum HarnessError {
    /// Official Judgment Profile was not satisfied.
    #[error("judgment profile: {0}")]
    JudgmentProfile(String),
    /// Drive Smoke failed (window not operable).
    #[error("drive smoke failed: {0}")]
    DriveSmokeFailed(String),
    /// Baseline load/compare problem.
    #[error("baseline: {0}")]
    Baseline(String),
    /// Scenario content problem.
    #[error("scenario: {0}")]
    Scenario(String),
    /// Isolation / path problem.
    #[error("isolation: {0}")]
    Isolation(String),
    /// I/O failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// JSON failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Session/DB failure.
    #[error("session: {0}")]
    Session(String),
}
