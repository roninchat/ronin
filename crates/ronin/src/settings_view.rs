//! Settings overlay sections and field labels.
//!
//! Presentation only. No GPUI.

/// Sidebar section in the settings overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsSection {
    /// Memories, artifacts, hints, notifications, auto-title.
    General,
    /// Theme and UI scale.
    Appearance,
    /// Provider and model defaults.
    Models,
    /// Data retention and export.
    Privacy,
    /// Shortcut reference.
    Keyboard,
    /// Developer / advanced toggles.
    Advanced,
}

/// Open section and search query for the settings overlay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsState {
    open: bool,
    section: SettingsSection,
    search: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsState {
    /// Closed overlay on the General section.
    pub fn new() -> Self {
        Self {
            open: false,
            section: SettingsSection::General,
            search: String::new(),
        }
    }

    /// Shows the overlay.
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Hides the overlay.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Flips overlay visibility.
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Whether the overlay is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Active section.
    pub fn section(&self) -> SettingsSection {
        self.section
    }

    /// Selects a section.
    pub fn set_section(&mut self, section: SettingsSection) {
        self.section = section;
    }

    /// Section filter text.
    pub fn search(&self) -> &str {
        &self.search
    }

    /// Replaces the section filter.
    pub fn set_search(&mut self, search: impl Into<String>) {
        self.search = search.into();
    }
}

/// Sections whose labels match `query` (case-insensitive substring).
///
/// Empty query returns every section.
pub fn visible_sections(query: &str) -> Vec<SettingsSection> {
    let all = [
        SettingsSection::General,
        SettingsSection::Appearance,
        SettingsSection::Models,
        SettingsSection::Privacy,
        SettingsSection::Keyboard,
        SettingsSection::Advanced,
    ];
    let q = query.trim();
    if q.is_empty() {
        return all.to_vec();
    }
    let q = q.to_lowercase();
    all.into_iter()
        .filter(|section| section_label(*section).to_lowercase().contains(&q))
        .collect()
}

/// Sidebar label for a settings section.
pub fn section_label(section: SettingsSection) -> &'static str {
    match section {
        SettingsSection::General => "General",
        SettingsSection::Appearance => "Appearance",
        SettingsSection::Models => "Models",
        SettingsSection::Privacy => "Privacy",
        SettingsSection::Keyboard => "Keyboard",
        SettingsSection::Advanced => "Advanced",
    }
}

/// General: enable memories.
pub fn memories_toggle_label() -> &'static str {
    "Memories"
}

/// General: enable artifacts.
pub fn artifacts_toggle_label() -> &'static str {
    "Artifacts"
}

/// General: show shortcut hints and the first-run coach.
pub fn shortcut_hints_toggle_label() -> &'static str {
    "Shortcut hints"
}

/// General: desktop notifications when generation finishes.
pub fn notifications_toggle_label() -> &'static str {
    "Desktop notifications"
}

/// General: auto-generate thread titles after the first reply.
pub fn auto_title_toggle_label() -> &'static str {
    "Auto-generate thread titles"
}

/// Appearance: color scheme preference.
pub fn theme_label() -> &'static str {
    "Theme"
}

/// Appearance: UI scale.
pub fn scale_label() -> &'static str {
    "Scale"
}
