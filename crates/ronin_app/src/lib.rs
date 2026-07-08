#![deny(missing_docs)]

//! UI-facing application shell state for Ronin.
//!
//! Crate-internal modules split product behavior by concern: `shell` owns the
//! [`RoninShell`] controller and rendered [`ShellState`], `chat` assembles
//! provider requests with context caps, `status` probes provider health for
//! [`ProviderStatus`], and `tools` parses and executes assistant tool calls.

mod chat;
mod error;
mod shell;
mod status;
mod tools;

pub use error::{Result, RoninAppError};
pub use shell::{RoninShell, ShellState, StreamUpdate, VisualDirection, VisualReuseDecision};
pub use status::ProviderStatus;
