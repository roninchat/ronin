//! Opt-in clipboard watch → confirm-to-attach proposals (M3.0 #77).
//!
//! Watching is **off by default**. When enabled, clipboard text *changes* stage a
//! pending attach proposal that requires explicit confirm. Proposals never merge
//! into provider chat assembly until the user confirms (trust
//! [`ContextOrigin::ConfirmToAttachAccepted`]). On-demand `@clipboard` remains a
//! separate explicit path.

use crate::context::{clipboard_attachment, ContextAttachmentDraft};
use crate::trust::{may_inject_into_chat_request, scrub_ambient_payload, ContextOrigin};

/// Maximum characters kept in the user-visible proposal preview.
pub const CLIPBOARD_PROPOSAL_PREVIEW_CHARS: usize = 240;

/// Errors from a [`ClipboardTextSource`] backend.
#[derive(Debug, thiserror::Error)]
pub enum ClipboardWatchError {
    /// Backend could not read clipboard text.
    #[error("clipboard read failed: {0}")]
    ReadFailed(String),
}

/// Thin host port: read current clipboard text (arboard / test double).
pub trait ClipboardTextSource {
    /// Returns current clipboard text, or an empty string when unavailable.
    fn read_text(&self) -> Result<String, ClipboardWatchError>;
}

/// Test double that returns a scripted clipboard sequence (FIFO).
#[derive(Debug, Default)]
pub struct ScriptedClipboardSource {
    next: std::sync::Mutex<std::collections::VecDeque<String>>,
}

impl ScriptedClipboardSource {
    /// Creates an empty script (reads return empty string).
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues texts returned by successive [`ClipboardTextSource::read_text`] calls.
    pub fn push_texts<I, S>(&self, texts: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.next
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .extend(texts.into_iter().map(Into::into));
    }
}

impl ClipboardTextSource for ScriptedClipboardSource {
    fn read_text(&self) -> Result<String, ClipboardWatchError> {
        let mut q = self.next.lock().unwrap_or_else(|p| p.into_inner());
        Ok(q.pop_front().unwrap_or_default())
    }
}

/// User preference gate for clipboard watching (default **off**).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClipboardWatchPrefs {
    /// When false, observe/poll never stages proposals.
    pub enabled: bool,
}

/// Outcome of feeding observed clipboard text into the watcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardObserveOutcome {
    /// Watcher is disabled — no proposal.
    IgnoredDisabled,
    /// Text unchanged vs baseline / last seen.
    Unchanged,
    /// Empty / whitespace-only clipboard ignored.
    IgnoredEmpty,
    /// A new (or replaced) pending confirm-to-attach proposal is ready.
    Proposed,
}

/// Pending confirm-to-attach draft staged by the clipboard watcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardAttachProposal {
    /// Opaque proposal id (stable for this pending draft).
    pub id: String,
    /// Full clipboard text to attach on confirm (not ambient-injected).
    pub text: String,
    /// Scrubbed, truncated preview for UI chrome.
    pub preview: String,
}

/// Opt-in clipboard watch state machine (confirm required; never silent-attach).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardWatchController {
    enabled: bool,
    last_seen: Option<String>,
    pending: Option<ClipboardAttachProposal>,
    next_id: u64,
    /// When true, the next observe seeds baseline without proposing.
    awaiting_baseline: bool,
}

impl Default for ClipboardWatchController {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardWatchController {
    /// Creates a disabled watcher with no pending proposal.
    pub fn new() -> Self {
        Self {
            enabled: false,
            last_seen: None,
            pending: None,
            next_id: 1,
            awaiting_baseline: false,
        }
    }

    /// Whether watching is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Pending confirm-to-attach proposal, if any.
    pub fn pending_proposal(&self) -> Option<&ClipboardAttachProposal> {
        self.pending.as_ref()
    }

    /// Enables watching, clearing any prior proposal and seeding the baseline.
    ///
    /// `current_clipboard` becomes the baseline so existing clipboard contents
    /// do not immediately become a proposal — only *changes* after enable do.
    /// When `None`, the next [`Self::observe_text`] call seeds without proposing.
    pub fn enable(&mut self, current_clipboard: Option<&str>) {
        self.enabled = true;
        self.pending = None;
        match current_clipboard {
            Some(text) => {
                self.last_seen = Some(text.to_string());
                self.awaiting_baseline = false;
            }
            None => {
                self.last_seen = None;
                self.awaiting_baseline = true;
            }
        }
    }

    /// Disables watching, clears proposals, and stops comparing clipboard text.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.pending = None;
        self.last_seen = None;
        self.awaiting_baseline = false;
    }

    /// Applies prefs: enable with optional baseline, or disable and clear.
    pub fn apply_prefs(&mut self, prefs: &ClipboardWatchPrefs, current_clipboard: Option<&str>) {
        if prefs.enabled {
            if !self.enabled {
                self.enable(current_clipboard);
            }
        } else if self.enabled {
            self.disable();
        }
    }

    /// Observes clipboard text. Stages a proposal only when enabled and changed.
    pub fn observe_text(&mut self, text: &str) -> ClipboardObserveOutcome {
        if !self.enabled {
            return ClipboardObserveOutcome::IgnoredDisabled;
        }
        if self.awaiting_baseline {
            self.awaiting_baseline = false;
            if text.trim().is_empty() {
                self.last_seen = None;
                return ClipboardObserveOutcome::IgnoredEmpty;
            }
            self.last_seen = Some(text.to_string());
            return ClipboardObserveOutcome::Unchanged;
        }
        if text.trim().is_empty() {
            return ClipboardObserveOutcome::IgnoredEmpty;
        }
        if self.last_seen.as_deref() == Some(text) {
            return ClipboardObserveOutcome::Unchanged;
        }
        self.last_seen = Some(text.to_string());
        let id = format!("clipboard-proposal-{}", self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.pending = Some(ClipboardAttachProposal {
            id,
            text: text.to_string(),
            preview: proposal_preview(text),
        });
        ClipboardObserveOutcome::Proposed
    }

    /// Confirms the pending proposal into an explicit clipboard attachment draft.
    ///
    /// Returns [`None`] when nothing is pending. Cleared after confirm.
    pub fn confirm_pending(&mut self) -> Option<ContextAttachmentDraft> {
        let proposal = self.pending.take()?;
        Some(clipboard_attachment(&proposal.text))
    }

    /// Dismisses / ignores the pending proposal without attaching.
    pub fn dismiss_pending(&mut self) {
        self.pending = None;
    }

    /// Reads from `source` when enabled and observes the result.
    pub fn poll_source(
        &mut self,
        source: &dyn ClipboardTextSource,
    ) -> Result<ClipboardObserveOutcome, ClipboardWatchError> {
        if !self.enabled {
            return Ok(ClipboardObserveOutcome::IgnoredDisabled);
        }
        let text = source.read_text()?;
        Ok(self.observe_text(&text))
    }
}

/// Context origin for a clipboard-watch proposal — never silent model context.
pub fn clipboard_watch_proposal_origin() -> ContextOrigin {
    ContextOrigin::ClipboardWatchProposal
}

/// Whether a clipboard-watch proposal may merge into a provider chat request.
pub fn clipboard_watch_proposal_may_inject_into_chat_request() -> bool {
    may_inject_into_chat_request(clipboard_watch_proposal_origin())
}

/// Context origin after the user confirms a clipboard-watch proposal.
pub fn confirmed_clipboard_attach_origin() -> ContextOrigin {
    ContextOrigin::ConfirmToAttachAccepted
}

/// Whether a confirmed clipboard-watch attach may merge into a chat request.
pub fn confirmed_clipboard_attach_may_inject_into_chat_request() -> bool {
    may_inject_into_chat_request(confirmed_clipboard_attach_origin())
}

/// Builds a scrubbed, truncated preview for proposal chrome.
pub fn proposal_preview(text: &str) -> String {
    let scrubbed = scrub_ambient_payload(text);
    if scrubbed.chars().count() <= CLIPBOARD_PROPOSAL_PREVIEW_CHARS {
        return scrubbed;
    }
    let truncated: String = scrubbed
        .chars()
        .take(CLIPBOARD_PROPOSAL_PREVIEW_CHARS)
        .collect();
    format!("{truncated}…")
}
