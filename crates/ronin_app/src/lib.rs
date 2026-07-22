#![deny(missing_docs)]

//! UI-facing application shell state for Ronin.
//!
//! Crate-internal modules split product behavior by concern: `shell` owns the
//! [`RoninShell`] controller and rendered [`ShellState`], `chat` assembles
//! provider requests with context caps, `status` probes provider health for
//! [`ProviderStatus`], formats actionable provider errors, and runs
//! [`run_connection_test`], and `tools` parses and executes assistant tool calls.

mod branches;
mod chat;
mod error;
mod shell;
mod status;
mod tools;

pub use branches::{
    leaf_under_root, resolve_active_path, sibling_branch_nav, BranchNav, MessageNode,
};
pub use chat::{
    build_title_generation_request, collect_streamed_title, derive_thread_title,
    may_apply_auto_title, sanitize_generated_title, MAX_CHARS, MAX_MESSAGES,
};
pub use error::{Result, RoninAppError};
pub use shell::{RoninShell, ShellState, StreamUpdate, VisualDirection, VisualReuseDecision};
pub use status::{
    format_provider_error, run_connection_test, ConnectionTestResult, ProviderStatus,
};
