//! Window chrome metrics and overflow menus.
//!
//! Layout constants and menu catalogs only. No GPUI window.

use crate::icons::IconName;

/// Custom titlebar height in logical pixels.
pub const TITLEBAR_HEIGHT: f32 = 40.0;

/// Icon rail width in logical pixels.
pub const ICON_RAIL_WIDTH: f32 = 48.0;

/// Minimum window width in logical pixels.
pub const WINDOW_MIN_WIDTH: f32 = 800.0;

/// Minimum window height in logical pixels.
pub const WINDOW_MIN_HEIGHT: f32 = 560.0;

/// Which overflow (`⋯`) menu is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowMenu {
    /// Window / titlebar overflow.
    Window,
    /// Thread list row overflow.
    Thread,
    /// Message bubble overflow.
    Message,
}

/// One row in an overflow menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverflowItem {
    /// Stable action id (`"settings"`, `"copy"`, …).
    pub id: &'static str,
    /// User-visible label.
    pub label: &'static str,
    /// Leading icon.
    pub icon: IconName,
    /// Shortcut chord, when the action has one.
    pub keys: Option<&'static str>,
}

/// Window overflow: settings, shortcuts, zoom, sidebar.
pub fn window_overflow_items() -> &'static [OverflowItem] {
    &[
        OverflowItem {
            id: "settings",
            label: "Settings",
            icon: IconName::Settings,
            keys: Some("Ctrl+,"),
        },
        OverflowItem {
            id: "shortcuts",
            label: "Keyboard shortcuts",
            icon: IconName::Keyboard,
            keys: Some("Ctrl+/"),
        },
        OverflowItem {
            id: "zoom-in",
            label: "Zoom in",
            icon: IconName::ZoomIn,
            keys: Some("Ctrl+="),
        },
        OverflowItem {
            id: "zoom-out",
            label: "Zoom out",
            icon: IconName::ZoomOut,
            keys: Some("Ctrl+-"),
        },
        OverflowItem {
            id: "zoom-reset",
            label: "Reset zoom",
            icon: IconName::Maximize,
            keys: Some("Ctrl+0"),
        },
        OverflowItem {
            id: "toggle-sidebar",
            label: "Toggle sidebar",
            icon: IconName::PanelLeft,
            keys: Some("Ctrl+B"),
        },
    ]
}

/// Thread overflow: rename plus placeholders for later actions.
pub fn thread_overflow_items() -> &'static [OverflowItem] {
    &[
        OverflowItem {
            id: "rename",
            label: "Rename",
            icon: IconName::Pencil,
            keys: None,
        },
        OverflowItem {
            id: "export",
            label: "Export",
            icon: IconName::Copy,
            keys: None,
        },
        OverflowItem {
            id: "delete",
            label: "Delete",
            icon: IconName::X,
            keys: None,
        },
    ]
}

/// Message overflow using default flags from the message role/status.
///
/// Always includes copy. Assistant messages include save-memory and save-artifact.
/// Failed messages include retry. The last assistant message includes regenerate.
/// User messages include edit.
pub fn message_overflow_items(
    is_assistant: bool,
    is_failed: bool,
    is_last_assistant: bool,
) -> Vec<OverflowItem> {
    message_overflow_items_with_flags(
        is_assistant,
        is_failed,
        is_last_assistant,
        is_assistant,
        is_assistant,
        is_failed,
        is_last_assistant,
        !is_assistant,
    )
}

/// Message overflow with explicit capability flags.
pub fn message_overflow_items_with_flags(
    is_assistant: bool,
    is_failed: bool,
    is_last_assistant: bool,
    memories_enabled: bool,
    artifacts_enabled: bool,
    can_retry: bool,
    can_regenerate: bool,
    can_edit: bool,
) -> Vec<OverflowItem> {
    let mut items = vec![OverflowItem {
        id: "copy",
        label: "Copy",
        icon: IconName::Copy,
        keys: None,
    }];

    if is_assistant && memories_enabled {
        items.push(OverflowItem {
            id: "save-memory",
            label: "Save memory",
            icon: IconName::Sparkles,
            keys: None,
        });
    }
    if is_assistant && artifacts_enabled {
        items.push(OverflowItem {
            id: "save-artifact",
            label: "Save artifact",
            icon: IconName::Box,
            keys: None,
        });
    }
    if is_failed && can_retry {
        items.push(OverflowItem {
            id: "retry",
            label: "Retry",
            icon: IconName::Refresh,
            keys: None,
        });
    }
    if is_assistant && is_last_assistant && can_regenerate {
        items.push(OverflowItem {
            id: "regenerate",
            label: "Regenerate",
            icon: IconName::RotateCcw,
            keys: None,
        });
    }
    if can_edit {
        items.push(OverflowItem {
            id: "edit",
            label: "Edit",
            icon: IconName::Pencil,
            keys: None,
        });
    }

    items
}
