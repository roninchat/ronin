//! Settings overlay section filter and field labels.

use ronin::settings_view::{
    artifacts_toggle_label, auto_title_toggle_label, memories_toggle_label,
    notifications_toggle_label, scale_label, section_label, shortcut_hints_toggle_label,
    theme_label, visible_sections, SettingsSection, SettingsState,
};

#[test]
fn open_close_toggle_should_change_visibility() {
    let mut settings = SettingsState::new();
    assert!(!settings.is_open());
    settings.open();
    assert!(settings.is_open());
    settings.close();
    assert!(!settings.is_open());
    settings.toggle();
    assert!(settings.is_open());
    settings.toggle();
    assert!(!settings.is_open());
}

#[test]
fn set_section_and_search_should_round_trip() {
    let mut settings = SettingsState::new();
    assert_eq!(settings.section(), SettingsSection::General);
    settings.set_section(SettingsSection::Appearance);
    settings.set_search("theme");
    assert_eq!(settings.section(), SettingsSection::Appearance);
    assert_eq!(settings.search(), "theme");
}

#[test]
fn visible_sections_should_filter_by_label_substring() {
    let all = visible_sections("");
    assert_eq!(all.len(), 6);
    assert!(all.contains(&SettingsSection::General));
    let models = visible_sections("mod");
    assert_eq!(models, vec![SettingsSection::Models]);
    let empty = visible_sections("no-such-section");
    assert!(empty.is_empty());
}

#[test]
fn section_and_field_labels_should_be_non_empty() {
    assert_eq!(section_label(SettingsSection::Keyboard), "Keyboard");
    assert!(!memories_toggle_label().is_empty());
    assert!(!artifacts_toggle_label().is_empty());
    assert!(!shortcut_hints_toggle_label().is_empty());
    assert!(!notifications_toggle_label().is_empty());
    assert!(!auto_title_toggle_label().is_empty());
    assert!(!theme_label().is_empty());
    assert!(!scale_label().is_empty());
}
