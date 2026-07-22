//! Attachment size warnings across attachment kinds.

use ronin::attachment_size::{
    attachment_size_warning, attachment_size_warning_for_drafts, AttachmentSizeWarnState,
    DEFAULT_ATTACHMENT_WARN_CHARS,
};
use ronin::context_indicator::estimate_tokens_from_chars;
use ronin_core::{clipboard_attachment, AttachmentKind, ContextAttachmentDraft};

fn draft(kind: AttachmentKind, name: &str, block: &str) -> ContextAttachmentDraft {
    ContextAttachmentDraft {
        kind,
        name: name.into(),
        mime_type: "text/plain".into(),
        content: Some(block.into()),
        path: None,
        context_block: block.into(),
        size_bytes: Some(block.len() as u64),
    }
}

#[test]
fn size_warning_should_trigger_above_configurable_threshold() {
    let threshold = 100;
    let big = "x".repeat(120);
    let warn = attachment_size_warning(big.chars().count(), threshold).expect("warn");
    assert_eq!(warn.total_chars, 120);
    assert_eq!(warn.threshold_chars, threshold);
    assert_eq!(warn.estimated_tokens, estimate_tokens_from_chars(120));
    assert!(warn.message.contains("120") || warn.message.to_lowercase().contains("token"));
    assert!(attachment_size_warning(50, threshold).is_none());
}

#[test]
fn size_warning_should_sum_all_attachment_kinds() {
    let drafts = vec![
        draft(AttachmentKind::File, "a.txt", &"a".repeat(40)),
        draft(AttachmentKind::Clipboard, "clip", &"b".repeat(40)),
        draft(
            AttachmentKind::Image,
            "pic.png",
            "[Attached image: pic.png]",
        ),
        draft(AttachmentKind::Folder, "src", &"c".repeat(40)),
    ];
    let threshold = 100;
    let warn = attachment_size_warning_for_drafts(&drafts, threshold).expect("warn");
    assert!(warn.total_chars > threshold);
    assert_eq!(
        warn.estimated_tokens,
        estimate_tokens_from_chars(warn.total_chars)
    );
}

#[test]
fn size_warning_state_should_allow_proceed_or_clear_after_warn() {
    let mut state = AttachmentSizeWarnState::default();
    assert!(!state.should_block_send());
    let drafts = vec![clipboard_attachment(
        &"z".repeat(DEFAULT_ATTACHMENT_WARN_CHARS + 10),
    )];
    state.evaluate(&drafts, DEFAULT_ATTACHMENT_WARN_CHARS);
    assert!(state.warning().is_some());
    assert!(state.should_block_send());

    state.acknowledge_and_proceed();
    assert!(!state.should_block_send());
    assert!(state.warning().is_some()); // still visible, but send allowed

    state.clear();
    assert!(state.warning().is_none());
    assert!(!state.should_block_send());
}

#[test]
fn default_warn_threshold_should_be_positive_and_configurable_via_constant() {
    const {
        assert!(DEFAULT_ATTACHMENT_WARN_CHARS >= 8_000);
    };
}
