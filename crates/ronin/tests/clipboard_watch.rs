//! Host clipboard watch reader + on-demand path coexistence (#77).

use ronin::clipboard_watch::{read_clipboard_text_lossy, ArboardClipboardSource};
use ronin_core::{
    clipboard_attachment, parse_context_tools, ClipboardObserveOutcome, ClipboardTextSource,
    ClipboardWatchController, ClipboardWatchError,
};

struct FailingSource;

impl ClipboardTextSource for FailingSource {
    fn read_text(&self) -> Result<String, ClipboardWatchError> {
        Err(ClipboardWatchError::ReadFailed("boom".into()))
    }
}

#[test]
fn lossy_read_swallows_source_errors() {
    assert_eq!(read_clipboard_text_lossy(&FailingSource), "");
}

#[test]
fn arboard_source_implements_trait_object_safely() {
    let source: &dyn ClipboardTextSource = &ArboardClipboardSource::new();
    // May succeed or fail depending on session clipboard; must not panic.
    let _ = source.read_text();
    let _ = read_clipboard_text_lossy(source);
}

#[test]
fn on_demand_clipboard_attachment_unaffected_by_disabled_watch() {
    let mut watch = ClipboardWatchController::new();
    assert!(!watch.is_enabled());
    let parsed = parse_context_tools("attach @clipboard now");
    assert!(parsed
        .refs
        .iter()
        .any(|r| matches!(r, ronin_core::ContextToolRef::Clipboard)));
    let draft = clipboard_attachment("from @clipboard");
    assert!(draft.context_block.contains("from @clipboard"));
    assert_eq!(
        watch.observe_text("ambient should ignore"),
        ClipboardObserveOutcome::IgnoredDisabled
    );
}

#[test]
fn host_watch_controller_confirm_gate_matrix() {
    for i in 0..30usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("h"));
        watch.observe_text(&format!("host-{i}"));
        assert!(watch.pending_proposal().is_some());
        if i % 2 == 0 {
            assert!(watch.confirm_pending().is_some());
        } else {
            watch.dismiss_pending();
        }
    }
}

#[test]
fn host_ondemand_matrix_stays_independent() {
    for i in 0..40usize {
        let mut watch = ClipboardWatchController::new();
        assert!(!watch.is_enabled());
        let parsed = parse_context_tools(&format!("note @clipboard {i}"));
        assert!(parsed
            .refs
            .iter()
            .any(|r| matches!(r, ronin_core::ContextToolRef::Clipboard)));
        let draft = clipboard_attachment(&format!("clip-{i}"));
        assert!(draft.context_block.contains(&format!("clip-{i}")));
        assert_eq!(
            watch.observe_text(&format!("ambient-{i}")),
            ClipboardObserveOutcome::IgnoredDisabled
        );
    }
}

#[test]
fn arboard_lossy_read_stable_across_calls() {
    let source = ArboardClipboardSource::new();
    for _ in 0..20 {
        let _ = read_clipboard_text_lossy(&source);
    }
}

#[test]
fn host_confirm_dismiss_disable_grid() {
    for i in 0..25usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("h0"));
        watch.observe_text(&format!("host-grid-{i}"));
        if i % 3 == 0 {
            watch.dismiss_pending();
        } else if i % 3 == 1 {
            assert!(watch.confirm_pending().is_some());
        } else {
            watch.disable();
            assert!(!watch.is_enabled());
        }
    }
}

#[test]
fn slash_catalog_exposes_clipboard_watch_actions() {
    use ronin::composer_pickers::{slash_action_catalog, SlashActionKind};
    let kinds: Vec<_> = slash_action_catalog()
        .iter()
        .filter_map(|i| i.slash_kind())
        .collect();
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchToggle));
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchConfirm));
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchDismiss));
    // Insert tokens are stable public seams.
    let inserts: Vec<_> = slash_action_catalog().iter().map(|i| i.insert).collect();
    assert!(inserts.contains(&"/clipboard-watch"));
    assert!(inserts.contains(&"/clipboard-confirm"));
    assert!(inserts.contains(&"/clipboard-dismiss"));
}

#[test]
fn slash_action_filter_finds_clipboard_watch() {
    use ronin::composer_pickers::{filter_picker_items, slash_action_catalog};
    for q in ["clip", "clipboard", "confirm", "dismiss", "watch"] {
        let filtered = filter_picker_items(slash_action_catalog(), q);
        assert!(
            !filtered.is_empty(),
            "expected clipboard-watch slash hits for query {q}"
        );
    }
}

#[test]
fn slash_tokens_stable_matrix() {
    use ronin::composer_pickers::slash_action_catalog;
    for _ in 0..40 {
        let inserts: Vec<_> = slash_action_catalog().iter().map(|i| i.insert).collect();
        assert!(inserts.contains(&"/clipboard-watch"));
        assert!(inserts.contains(&"/clipboard-confirm"));
        assert!(inserts.contains(&"/clipboard-dismiss"));
    }
}
