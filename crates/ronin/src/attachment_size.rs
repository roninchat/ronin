//! Attachment size warnings before send.

use ronin_core::{total_attachment_chars, ContextAttachmentDraft};

use crate::context_indicator::estimate_tokens_from_chars;

/// Default character threshold before attachment size warnings appear.
pub const DEFAULT_ATTACHMENT_WARN_CHARS: usize = ronin_core::DEFAULT_ATTACHMENT_WARN_CHARS;

/// Snapshot describing an attachment size warning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentSizeWarning {
    /// Total attachment context characters.
    pub total_chars: usize,
    /// Approximate tokens (`ceil(chars / 4)`).
    pub estimated_tokens: usize,
    /// Configured warning threshold in characters.
    pub threshold_chars: usize,
    /// User-visible warning copy.
    pub message: String,
}

/// Builds a warning when `total_chars` exceeds `threshold_chars`.
pub fn attachment_size_warning(
    total_chars: usize,
    threshold_chars: usize,
) -> Option<AttachmentSizeWarning> {
    if total_chars <= threshold_chars {
        return None;
    }
    let estimated_tokens = estimate_tokens_from_chars(total_chars);
    let threshold_tokens = estimate_tokens_from_chars(threshold_chars);
    Some(AttachmentSizeWarning {
        total_chars,
        estimated_tokens,
        threshold_chars,
        message: format!(
            "Attachments are large (~{estimated_tokens} tokens / {total_chars} chars; warning at ~{threshold_tokens} tokens). Proceed, or remove attachments to free context."
        ),
    })
}

/// Evaluates size warning across all pending attachment drafts.
pub fn attachment_size_warning_for_drafts(
    drafts: &[ContextAttachmentDraft],
    threshold_chars: usize,
) -> Option<AttachmentSizeWarning> {
    attachment_size_warning(total_attachment_chars(drafts), threshold_chars)
}

/// UI state for size warning acknowledge / clear flow.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttachmentSizeWarnState {
    warning: Option<AttachmentSizeWarning>,
    acknowledged: bool,
}

impl AttachmentSizeWarnState {
    /// Recomputes the warning from current drafts.
    pub fn evaluate(&mut self, drafts: &[ContextAttachmentDraft], threshold_chars: usize) {
        let next = attachment_size_warning_for_drafts(drafts, threshold_chars);
        if next.as_ref().map(|w| w.total_chars) != self.warning.as_ref().map(|w| w.total_chars) {
            self.acknowledged = false;
        }
        self.warning = next;
    }

    /// Current warning, if any.
    pub fn warning(&self) -> Option<&AttachmentSizeWarning> {
        self.warning.as_ref()
    }

    /// Whether send should be blocked until the user proceeds or removes attachments.
    pub fn should_block_send(&self) -> bool {
        self.warning.is_some() && !self.acknowledged
    }

    /// User chose to proceed despite the warning.
    pub fn acknowledge_and_proceed(&mut self) {
        if self.warning.is_some() {
            self.acknowledged = true;
        }
    }

    /// Clears warning state (e.g. after attachments were removed).
    pub fn clear(&mut self) {
        self.warning = None;
        self.acknowledged = false;
    }
}

/// Fallback threshold when config is unavailable.
pub fn default_warn_chars() -> usize {
    DEFAULT_ATTACHMENT_WARN_CHARS
}
