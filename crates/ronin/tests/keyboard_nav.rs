//! Keyboard-first navigation state machine and shortcut catalog.

use ronin::keyboard_nav::{
    shortcut_catalog, FocusRegion, KeyInput, KeyboardNavState, NavAction, ScrollDirection,
};

fn key(key: &str) -> KeyInput<'_> {
    KeyInput {
        key,
        control: false,
        shift: false,
        alt: false,
    }
}

fn ctrl(key: &str) -> KeyInput<'_> {
    KeyInput {
        key,
        control: true,
        shift: false,
        alt: false,
    }
}

fn shift_tab() -> KeyInput<'static> {
    KeyInput {
        key: "tab",
        control: false,
        shift: true,
        alt: false,
    }
}

#[test]
fn default_focus_should_be_composer() {
    let nav = KeyboardNavState::new();
    assert_eq!(nav.focus(), FocusRegion::Composer);
    assert_eq!(nav.thread_highlight(), None);
    assert!(!nav.help_visible());
}

#[test]
fn tab_should_cycle_focus_sidebar_messages_composer() {
    let mut nav = KeyboardNavState::new();
    let (consumed, action) = nav.handle_key(key("tab"), 3);
    assert!(consumed);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Sidebar));
    assert_eq!(nav.focus(), FocusRegion::Sidebar);
    assert_eq!(nav.thread_highlight(), Some(0));

    let (_, action) = nav.handle_key(key("tab"), 3);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Messages));

    let (_, action) = nav.handle_key(key("tab"), 3);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Composer));
}

#[test]
fn shift_tab_should_cycle_focus_backwards() {
    let mut nav = KeyboardNavState::new();
    let (_, action) = nav.handle_key(shift_tab(), 2);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Messages));
    let (_, action) = nav.handle_key(shift_tab(), 2);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Sidebar));
}

#[test]
fn ctrl_1_should_focus_sidebar_even_from_composer() {
    let mut nav = KeyboardNavState::new();
    let (consumed, action) = nav.handle_key(ctrl("1"), 4);
    assert!(consumed);
    assert_eq!(action, NavAction::FocusChanged(FocusRegion::Sidebar));
    assert_eq!(nav.focus(), FocusRegion::Sidebar);
    assert_eq!(nav.thread_highlight(), Some(0));
}

#[test]
fn arrow_keys_should_move_thread_highlight_when_sidebar_focused() {
    let mut nav = KeyboardNavState::new();
    nav.set_focus(FocusRegion::Sidebar, 3);
    assert_eq!(nav.thread_highlight(), Some(0));

    let (consumed, action) = nav.handle_key(key("down"), 3);
    assert!(consumed);
    assert_eq!(action, NavAction::ThreadHighlightChanged { index: 1 });
    assert_eq!(nav.thread_highlight(), Some(1));

    let (_, action) = nav.handle_key(key("down"), 3);
    assert_eq!(action, NavAction::ThreadHighlightChanged { index: 2 });

    let (_, action) = nav.handle_key(key("down"), 3);
    assert_eq!(
        action,
        NavAction::ThreadHighlightChanged { index: 2 },
        "clamp at last thread"
    );

    let (_, action) = nav.handle_key(key("up"), 3);
    assert_eq!(action, NavAction::ThreadHighlightChanged { index: 1 });
}

#[test]
fn enter_should_select_highlighted_thread_when_sidebar_focused() {
    let mut nav = KeyboardNavState::new();
    nav.set_focus(FocusRegion::Sidebar, 3);
    nav.handle_key(key("down"), 3);
    let (consumed, action) = nav.handle_key(key("enter"), 3);
    assert!(consumed);
    assert_eq!(action, NavAction::SelectThread { index: 1 });
}

#[test]
fn page_keys_should_scroll_messages_when_messages_focused() {
    let mut nav = KeyboardNavState::new();
    nav.set_focus(FocusRegion::Messages, 0);

    let (consumed, action) = nav.handle_key(key("pageup"), 0);
    assert!(consumed);
    assert_eq!(action, NavAction::ScrollMessages(ScrollDirection::Up));

    let (consumed, action) = nav.handle_key(key("pagedown"), 0);
    assert!(consumed);
    assert_eq!(action, NavAction::ScrollMessages(ScrollDirection::Down));
}

#[test]
fn ctrl_slash_should_toggle_help_overlay() {
    let mut nav = KeyboardNavState::new();
    let (consumed, action) = nav.handle_key(ctrl("/"), 0);
    assert!(consumed);
    assert_eq!(action, NavAction::ToggleHelp);
    assert!(nav.help_visible());

    let (_, action) = nav.handle_key(ctrl("/"), 0);
    assert_eq!(action, NavAction::ToggleHelp);
    assert!(!nav.help_visible());
}

#[test]
fn escape_should_dismiss_help_when_visible() {
    let mut nav = KeyboardNavState::new();
    nav.handle_key(ctrl("/"), 0);
    assert!(nav.help_visible());
    let (consumed, action) = nav.handle_key(key("escape"), 0);
    assert!(consumed);
    assert_eq!(action, NavAction::ToggleHelp);
    assert!(!nav.help_visible());
}

#[test]
fn arrow_keys_should_not_consume_when_composer_focused() {
    let mut nav = KeyboardNavState::new();
    assert_eq!(nav.focus(), FocusRegion::Composer);
    let (consumed, action) = nav.handle_key(key("up"), 3);
    assert!(!consumed);
    assert_eq!(action, NavAction::None);
}

#[test]
fn shortcut_catalog_should_include_m0_and_navigation_shortcuts() {
    let catalog = shortcut_catalog();
    assert!(catalog.len() >= 8);

    let blob: String = catalog
        .iter()
        .flat_map(|h| [h.keys, h.action])
        .collect::<Vec<_>>()
        .join(" ");

    for needle in [
        "Enter",
        "Shift+Enter",
        "Esc",
        "Ctrl+N",
        "Ctrl+1",
        "Tab",
        "Page",
        "Ctrl+/",
        "Ctrl+F",
    ] {
        assert!(
            blob.contains(needle),
            "catalog missing discoverable hint for {needle}: {blob}"
        );
    }
}

#[test]
fn ctrl_f_should_toggle_search() {
    let mut nav = KeyboardNavState::new();
    let (consumed, action) = nav.handle_key(
        KeyInput {
            key: "f",
            control: true,
            shift: false,
            alt: false,
        },
        0,
    );
    assert!(consumed);
    assert_eq!(action, NavAction::ToggleSearch);

    let (consumed, action) = nav.handle_key(
        KeyInput {
            key: "f",
            control: true,
            shift: true,
            alt: false,
        },
        0,
    );
    assert!(consumed);
    assert_eq!(action, NavAction::ToggleSearch);
}
