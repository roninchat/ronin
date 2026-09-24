//! Window chrome metrics and overflow menu catalogs.

use ronin::chrome::{
    message_overflow_items, message_overflow_items_with_flags, thread_overflow_items,
    window_overflow_items, ICON_RAIL_WIDTH, TITLEBAR_HEIGHT, WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH,
};

fn ids(items: &[ronin::chrome::OverflowItem]) -> Vec<&'static str> {
    items.iter().map(|item| item.id).collect()
}

#[test]
fn chrome_metrics_should_match_shell_constants() {
    assert_eq!(TITLEBAR_HEIGHT, 40.0);
    assert_eq!(ICON_RAIL_WIDTH, 48.0);
    assert_eq!(WINDOW_MIN_WIDTH, 800.0);
    assert_eq!(WINDOW_MIN_HEIGHT, 560.0);
}

#[test]
fn window_overflow_should_include_settings_shortcuts_zoom_and_sidebar() {
    let present = ids(window_overflow_items());
    assert!(present.contains(&"settings"));
    assert!(present.contains(&"shortcuts"));
    assert!(present.contains(&"zoom-in"));
    assert!(present.contains(&"zoom-out"));
    assert!(present.contains(&"zoom-reset"));
    assert!(present.contains(&"toggle-sidebar"));
}

#[test]
fn thread_overflow_should_include_rename() {
    assert!(ids(thread_overflow_items()).contains(&"rename"));
}

#[test]
fn message_overflow_should_always_include_copy() {
    let user = message_overflow_items(false, false, false);
    let assistant = message_overflow_items(true, false, true);
    assert!(ids(&user).contains(&"copy"));
    assert!(ids(&assistant).contains(&"copy"));
}

#[test]
fn message_overflow_flags_should_gate_memory_artifact_retry_regenerate_edit() {
    let gated =
        message_overflow_items_with_flags(true, true, true, false, false, false, false, false);
    assert_eq!(ids(&gated), vec!["copy"]);

    let full = message_overflow_items_with_flags(true, true, true, true, true, true, true, true);
    let present = ids(&full);
    assert!(present.contains(&"copy"));
    assert!(present.contains(&"save-memory"));
    assert!(present.contains(&"save-artifact"));
    assert!(present.contains(&"retry"));
    assert!(present.contains(&"regenerate"));
    assert!(present.contains(&"edit"));
}

#[test]
fn user_message_overflow_should_include_edit_not_save_memory() {
    let items = message_overflow_items(false, false, false);
    let present = ids(&items);
    assert!(present.contains(&"edit"));
    assert!(!present.contains(&"save-memory"));
    assert!(!present.contains(&"save-artifact"));
}
