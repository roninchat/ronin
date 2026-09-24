//! Command palette catalogs, filter, and selection clamp.

use ronin::command_palette::{
    command_catalog, filter_items, quick_items, thread_items, CommandPaletteState, PaletteAction,
    PaletteMode,
};

#[test]
fn filter_items_should_match_label_substring_case_insensitively() {
    let catalog = command_catalog();
    let hits = filter_items(&catalog, "ZOOM");
    assert!(hits.iter().any(|item| item.action == PaletteAction::ZoomIn));
    assert!(hits
        .iter()
        .any(|item| item.action == PaletteAction::ZoomOut));
    assert!(hits
        .iter()
        .any(|item| item.action == PaletteAction::ZoomReset));
    assert!(!hits
        .iter()
        .any(|item| item.action == PaletteAction::NewThread));
}

#[test]
fn filter_items_should_return_all_when_query_empty() {
    let catalog = command_catalog();
    let hits = filter_items(&catalog, "  ");
    assert_eq!(hits.len(), catalog.len());
}

#[test]
fn open_quick_and_close_should_toggle_visibility_and_mode() {
    let mut palette = CommandPaletteState::new();
    assert!(!palette.is_open());
    palette.open_quick();
    assert!(palette.is_open());
    assert_eq!(palette.mode(), PaletteMode::Quick);
    assert_eq!(palette.query(), "");
    assert_eq!(palette.selected_index(), 0);
    palette.set_query("set");
    assert_eq!(palette.query(), "set");
    palette.close();
    assert!(!palette.is_open());
    assert_eq!(palette.query(), "");
}

#[test]
fn open_commands_should_switch_mode_and_reset_query() {
    let mut palette = CommandPaletteState::new();
    palette.open_quick();
    palette.set_query("hello");
    palette.open_commands();
    assert!(palette.is_open());
    assert_eq!(palette.mode(), PaletteMode::Commands);
    assert_eq!(palette.query(), "");
    assert_eq!(palette.selected_index(), 0);
}

#[test]
fn move_sel_should_clamp_to_item_count() {
    let mut palette = CommandPaletteState::new();
    palette.open_commands();
    palette.move_sel(100, 3);
    assert_eq!(palette.selected_index(), 2);
    palette.move_sel(-100, 3);
    assert_eq!(palette.selected_index(), 0);
    palette.move_sel(1, 0);
    assert_eq!(palette.selected_index(), 0);
}

#[test]
fn thread_items_should_carry_select_action_and_index() {
    let items = thread_items(&[(2, "Alpha"), (5, "Beta")]);
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].action, PaletteAction::SelectThread);
    assert_eq!(items[0].thread_index, Some(2));
    assert_eq!(items[0].label, "Alpha");
    assert_eq!(items[1].thread_index, Some(5));
}

#[test]
fn quick_items_should_list_threads_then_new_search_settings() {
    let items = quick_items(&[(0, "Today")]);
    let actions: Vec<_> = items.iter().map(|item| item.action).collect();
    assert_eq!(actions[0], PaletteAction::SelectThread);
    assert!(actions.contains(&PaletteAction::NewThread));
    assert!(actions.contains(&PaletteAction::OpenSearch));
    assert!(actions.contains(&PaletteAction::OpenSettings));
    assert!(!actions.contains(&PaletteAction::ZoomIn));
}

#[test]
fn command_catalog_should_include_core_actions() {
    let actions: Vec<_> = command_catalog()
        .into_iter()
        .map(|item| item.action)
        .collect();
    assert!(actions.contains(&PaletteAction::NewThread));
    assert!(actions.contains(&PaletteAction::ToggleSidebar));
    assert!(actions.contains(&PaletteAction::OpenSettings));
    assert!(actions.contains(&PaletteAction::Screenshot));
    assert!(actions.contains(&PaletteAction::ScreenshotWindow));
    assert!(!actions.contains(&PaletteAction::SelectThread));
}
