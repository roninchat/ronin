//! Ctrl+P quick switcher and Ctrl+Shift+P command palette.
//!
//! Testable without GPUI: catalogs, substring filter, open/close, selection clamp.

use crate::icons::IconName;

/// Which palette was opened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteMode {
    /// Ctrl+P: threads plus a few commands.
    Quick,
    /// Ctrl+Shift+P: full command catalog.
    Commands,
}

/// Action dispatched when a palette row is confirmed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteAction {
    /// Create a new thread.
    NewThread,
    /// Collapse or expand the sidebar.
    ToggleSidebar,
    /// Open settings.
    OpenSettings,
    /// Open global search.
    OpenSearch,
    /// Capture a screenshot (interactive / default).
    Screenshot,
    /// Capture the current window (falls back when unsupported).
    ScreenshotWindow,
    /// Focus the composer.
    FocusComposer,
    /// Toggle keyboard shortcut help.
    ToggleHelp,
    /// Increase UI zoom.
    ZoomIn,
    /// Decrease UI zoom.
    ZoomOut,
    /// Restore default zoom.
    ZoomReset,
    /// Open the memories panel.
    OpenMemories,
    /// Open the artifacts panel.
    OpenArtifacts,
    /// Select a thread. Index is on [`PaletteItem::thread_index`].
    SelectThread,
}

/// One palette row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaletteItem {
    /// User-visible label (thread title or command name).
    pub label: String,
    /// Shortcut chord, when the command has one.
    pub keys: Option<&'static str>,
    /// Leading icon.
    pub icon: IconName,
    /// Action to run on confirm.
    pub action: PaletteAction,
    /// Thread list index when [`PaletteAction::SelectThread`].
    pub thread_index: Option<usize>,
}

/// Open/query/selection state for the palette overlay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandPaletteState {
    open: bool,
    mode: PaletteMode,
    query: String,
    selected: usize,
}

impl Default for CommandPaletteState {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPaletteState {
    /// Closed palette, quick mode, empty query.
    pub fn new() -> Self {
        Self {
            open: false,
            mode: PaletteMode::Quick,
            query: String::new(),
            selected: 0,
        }
    }

    /// Opens Ctrl+P (threads + a few commands) and clears the query.
    pub fn open_quick(&mut self) {
        self.open = true;
        self.mode = PaletteMode::Quick;
        self.query.clear();
        self.selected = 0;
    }

    /// Opens Ctrl+Shift+P (commands) and clears the query.
    pub fn open_commands(&mut self) {
        self.open = true;
        self.mode = PaletteMode::Commands;
        self.query.clear();
        self.selected = 0;
    }

    /// Dismisses the palette.
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.selected = 0;
    }

    /// Whether the overlay is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Quick vs commands.
    pub fn mode(&self) -> PaletteMode {
        self.mode
    }

    /// Current filter text.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Replaces the filter and resets selection to the first row.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected = 0;
    }

    /// Moves the highlight by `delta`, clamping to `[0, item_count)`.
    pub fn move_sel(&mut self, delta: i32, item_count: usize) {
        if item_count == 0 {
            self.selected = 0;
            return;
        }
        let max = (item_count - 1) as i64;
        let next = (self.selected as i64 + i64::from(delta)).clamp(0, max);
        self.selected = next as usize;
    }

    /// Highlighted row index (not yet clamped to a live item list).
    pub fn selected_index(&self) -> usize {
        self.selected
    }
}

/// Static command catalog (no threads).
pub fn command_catalog() -> Vec<PaletteItem> {
    vec![
        item(
            "New chat",
            Some("Ctrl+N"),
            IconName::MessageSquarePlus,
            PaletteAction::NewThread,
        ),
        item(
            "Search",
            Some("Ctrl+F"),
            IconName::Search,
            PaletteAction::OpenSearch,
        ),
        item(
            "Settings",
            Some("Ctrl+,"),
            IconName::Settings,
            PaletteAction::OpenSettings,
        ),
        item(
            "Toggle sidebar",
            Some("Ctrl+B"),
            IconName::PanelLeft,
            PaletteAction::ToggleSidebar,
        ),
        item(
            "Focus composer",
            Some("Ctrl+L"),
            IconName::Pencil,
            PaletteAction::FocusComposer,
        ),
        item(
            "Capture screenshot",
            None,
            IconName::Camera,
            PaletteAction::Screenshot,
        ),
        item(
            "Capture window",
            None,
            IconName::Maximize,
            PaletteAction::ScreenshotWindow,
        ),
        item(
            "Keyboard shortcuts",
            Some("Ctrl+/"),
            IconName::Keyboard,
            PaletteAction::ToggleHelp,
        ),
        item(
            "Zoom in",
            Some("Ctrl+="),
            IconName::ZoomIn,
            PaletteAction::ZoomIn,
        ),
        item(
            "Zoom out",
            Some("Ctrl+-"),
            IconName::ZoomOut,
            PaletteAction::ZoomOut,
        ),
        item(
            "Reset zoom",
            Some("Ctrl+0"),
            IconName::Maximize,
            PaletteAction::ZoomReset,
        ),
        item(
            "Open memories",
            None,
            IconName::Sparkles,
            PaletteAction::OpenMemories,
        ),
        item(
            "Open artifacts",
            None,
            IconName::Box,
            PaletteAction::OpenArtifacts,
        ),
    ]
}

/// Case-insensitive substring filter on [`PaletteItem::label`].
///
/// Empty or whitespace-only queries return every item.
pub fn filter_items(items: &[PaletteItem], query: &str) -> Vec<PaletteItem> {
    let q = query.trim();
    if q.is_empty() {
        return items.to_vec();
    }
    let q = q.to_lowercase();
    items
        .iter()
        .filter(|item| item.label.to_lowercase().contains(&q))
        .cloned()
        .collect()
}

/// Thread rows for Ctrl+P. `titles` is `(index, title)`.
pub fn thread_items(titles: &[(usize, &str)]) -> Vec<PaletteItem> {
    titles
        .iter()
        .map(|(index, title)| PaletteItem {
            label: (*title).to_string(),
            keys: None,
            icon: IconName::File,
            action: PaletteAction::SelectThread,
            thread_index: Some(*index),
        })
        .collect()
}

/// Ctrl+P rows: threads, then new chat, search, and settings.
pub fn quick_items(titles: &[(usize, &str)]) -> Vec<PaletteItem> {
    let mut items = thread_items(titles);
    items.extend(command_catalog().into_iter().filter(|item| {
        matches!(
            item.action,
            PaletteAction::NewThread | PaletteAction::OpenSearch | PaletteAction::OpenSettings
        )
    }));
    items
}

fn item(
    label: &str,
    keys: Option<&'static str>,
    icon: IconName,
    action: PaletteAction,
) -> PaletteItem {
    PaletteItem {
        label: label.to_string(),
        keys,
        icon,
        action,
        thread_index: None,
    }
}
