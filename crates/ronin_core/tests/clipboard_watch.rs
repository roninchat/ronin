//! Opt-in clipboard watch → confirm-to-attach (#77).
//!
//! Public seams:
//! - [`ronin_core::ClipboardWatchController`]
//! - [`ronin_core::clipboard_watch_proposal_may_inject_into_chat_request`]
//! - [`ronin_core::confirmed_clipboard_attach_may_inject_into_chat_request`]
//! - [`ronin_core::clipboard_attachment`] (on-demand `@clipboard` unchanged)

use ronin_core::{
    clipboard_attachment, clipboard_watch_proposal_may_inject_into_chat_request,
    clipboard_watch_proposal_origin, confirmed_clipboard_attach_may_inject_into_chat_request,
    confirmed_clipboard_attach_origin, may_inject_into_chat_request, parse_context_tools,
    proposal_preview, scrub_ambient_payload, ClipboardObserveOutcome, ClipboardWatchController,
    ClipboardWatchPrefs, ScriptedClipboardSource, AMBIENT_REDACTED,
    CLIPBOARD_PROPOSAL_PREVIEW_CHARS,
};

#[test]
fn watcher_disabled_by_default() {
    let watch = ClipboardWatchController::new();
    assert!(!watch.is_enabled());
    assert!(watch.pending_proposal().is_none());
    assert_eq!(
        ClipboardWatchPrefs::default(),
        ClipboardWatchPrefs { enabled: false }
    );
}

#[test]
fn disabled_observe_never_stages_proposal() {
    let mut watch = ClipboardWatchController::new();
    assert_eq!(
        watch.observe_text("secret paste"),
        ClipboardObserveOutcome::IgnoredDisabled
    );
    assert!(watch.pending_proposal().is_none());
    assert!(!may_inject_into_chat_request(
        clipboard_watch_proposal_origin()
    ));
}

#[test]
fn enable_with_baseline_only_proposes_on_change() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("already on clipboard"));
    assert!(watch.is_enabled());
    assert_eq!(
        watch.observe_text("already on clipboard"),
        ClipboardObserveOutcome::Unchanged
    );
    assert!(watch.pending_proposal().is_none());

    assert_eq!(
        watch.observe_text("fresh copy"),
        ClipboardObserveOutcome::Proposed
    );
    let proposal = watch.pending_proposal().expect("proposal");
    assert_eq!(proposal.text, "fresh copy");
    assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        clipboard_watch_proposal_origin()
    ));
}

#[test]
fn enable_without_baseline_seeds_first_observe() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(None);
    assert_eq!(
        watch.observe_text("existing"),
        ClipboardObserveOutcome::Unchanged
    );
    assert!(watch.pending_proposal().is_none());
    assert_eq!(
        watch.observe_text("changed later"),
        ClipboardObserveOutcome::Proposed
    );
    assert_eq!(
        watch.pending_proposal().map(|p| p.text.as_str()),
        Some("changed later")
    );
}

#[test]
fn confirm_builds_clipboard_attachment_and_clears_pending() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("base"));
    assert_eq!(
        watch.observe_text("attach me"),
        ClipboardObserveOutcome::Proposed
    );
    let draft = watch.confirm_pending().expect("draft");
    assert_eq!(draft.name, "clipboard");
    assert_eq!(draft.content.as_deref(), Some("attach me"));
    assert!(draft.context_block.contains("attach me"));
    assert!(watch.pending_proposal().is_none());
    assert!(confirmed_clipboard_attach_may_inject_into_chat_request());
    assert!(may_inject_into_chat_request(
        confirmed_clipboard_attach_origin()
    ));
}

#[test]
fn dismiss_clears_proposal_without_attach() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("base"));
    watch.observe_text("ignore me");
    assert!(watch.pending_proposal().is_some());
    watch.dismiss_pending();
    assert!(watch.pending_proposal().is_none());
    assert!(watch.confirm_pending().is_none());
}

#[test]
fn disable_clears_proposals_and_stops_watching() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("base"));
    watch.observe_text("pending");
    assert!(watch.pending_proposal().is_some());
    watch.disable();
    assert!(!watch.is_enabled());
    assert!(watch.pending_proposal().is_none());
    assert_eq!(
        watch.observe_text("after disable"),
        ClipboardObserveOutcome::IgnoredDisabled
    );
    assert!(watch.pending_proposal().is_none());
}

#[test]
fn empty_clipboard_ignored_when_enabled() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("base"));
    assert_eq!(
        watch.observe_text("   "),
        ClipboardObserveOutcome::IgnoredEmpty
    );
    assert!(watch.pending_proposal().is_none());
}

#[test]
fn proposal_preview_scrubs_secrets_and_truncates() {
    let preview = proposal_preview("token=super-secret-value and more text");
    assert!(preview.contains(AMBIENT_REDACTED));
    assert!(!preview.contains("super-secret-value"));
    assert_eq!(preview, scrub_ambient_payload(&preview));

    let long = "x".repeat(CLIPBOARD_PROPOSAL_PREVIEW_CHARS + 40);
    let truncated = proposal_preview(&long);
    assert!(truncated.ends_with('…'));
    assert!(truncated.chars().count() <= CLIPBOARD_PROPOSAL_PREVIEW_CHARS + 1);
}

#[test]
fn on_demand_clipboard_path_unchanged_when_watcher_off() {
    let watch = ClipboardWatchController::new();
    assert!(!watch.is_enabled());
    // Explicit @clipboard parse still works independently of the watcher.
    let parsed = parse_context_tools("see @clipboard please");
    assert!(parsed
        .refs
        .iter()
        .any(|r| matches!(r, ronin_core::ContextToolRef::Clipboard)));
    let draft = clipboard_attachment("explicit paste");
    assert_eq!(draft.content.as_deref(), Some("explicit paste"));
    // Watcher still has no proposal / inject path.
    assert!(watch.pending_proposal().is_none());
    assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
}

#[test]
fn poll_source_respects_enabled_and_scripts() {
    let source = ScriptedClipboardSource::new();
    source.push_texts(["base", "base", "next"]);
    let mut watch = ClipboardWatchController::new();
    assert_eq!(
        watch.poll_source(&source).expect("poll"),
        ClipboardObserveOutcome::IgnoredDisabled
    );
    watch.enable(None);
    assert_eq!(
        watch.poll_source(&source).expect("baseline"),
        ClipboardObserveOutcome::Unchanged
    );
    assert_eq!(
        watch.poll_source(&source).expect("unchanged"),
        ClipboardObserveOutcome::Unchanged
    );
    assert_eq!(
        watch.poll_source(&source).expect("change"),
        ClipboardObserveOutcome::Proposed
    );
    assert_eq!(
        watch.pending_proposal().map(|p| p.text.as_str()),
        Some("next")
    );
}

#[test]
fn replacing_proposal_keeps_only_latest() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("0"));
    watch.observe_text("one");
    let first_id = watch.pending_proposal().unwrap().id.clone();
    watch.observe_text("two");
    let second = watch.pending_proposal().unwrap();
    assert_ne!(second.id, first_id);
    assert_eq!(second.text, "two");
}
