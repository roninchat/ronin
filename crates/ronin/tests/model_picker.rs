//! Public seams for the model picker: grouping, capabilities, keyboard nav.

use ronin::model_picker::{
    build_picker_entries, entries_from_listed_providers, format_capability_summary,
    group_entries_by_provider, infer_model_capabilities, open_picker_at_active,
    picker_row_colors, refresh_picker_entries, ModelPickerAction, ModelPickerKey,
    ModelPickerState, ModelProviderKind, PickerRowTone,
};
use ronin_core::ColorScheme;

#[test]
fn capabilities_should_include_context_window_when_known() {
    let caps = infer_model_capabilities("gpt-4o");
    assert_eq!(caps.context_window_tokens, Some(128_000));
    assert!(caps.supports_vision);
    assert!(caps.summary_parts().iter().any(|p| p.contains("128")));
}

#[test]
fn capabilities_should_detect_vision_for_llava_and_gpt4o() {
    assert!(infer_model_capabilities("llava").supports_vision);
    assert!(infer_model_capabilities("llava:13b").supports_vision);
    assert!(infer_model_capabilities("gpt-4o-mini").supports_vision);
    assert!(!infer_model_capabilities("llama3.2").supports_vision);
}

#[test]
fn unknown_model_capabilities_should_omit_window() {
    let caps = infer_model_capabilities("totally-unknown-xyz");
    assert!(caps.context_window_tokens.is_none());
    assert!(!caps.supports_vision);
}

#[test]
fn build_picker_entries_should_group_by_provider_with_name_and_provider() {
    let entries = build_picker_entries(&[
        (
            ModelProviderKind::Ollama,
            vec!["llama3.2".into(), "llava".into()],
        ),
        (ModelProviderKind::OpenAi, vec!["gpt-4o".into()]),
    ]);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].provider, ModelProviderKind::Ollama);
    assert_eq!(entries[0].model_name, "llama3.2");
    assert_eq!(entries[0].provider_label, "Ollama");
    assert_eq!(entries[2].provider, ModelProviderKind::OpenAi);
    assert_eq!(entries[2].model_name, "gpt-4o");
    assert_eq!(entries[2].provider_label, "OpenAI");
    assert!(entries[1].capabilities.supports_vision);
}

#[test]
fn group_entries_should_preserve_provider_sections() {
    let entries = build_picker_entries(&[
        (ModelProviderKind::OpenAi, vec!["gpt-4o".into()]),
        (ModelProviderKind::Ollama, vec!["mistral".into()]),
    ]);
    let grouped = group_entries_by_provider(&entries);
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped[0].0, ModelProviderKind::Ollama);
    assert_eq!(grouped[0].1[0].model_name, "mistral");
    assert_eq!(grouped[1].0, ModelProviderKind::OpenAi);
}

#[test]
fn format_capability_summary_should_join_known_parts() {
    let caps = infer_model_capabilities("gpt-4o");
    let summary = format_capability_summary(&caps);
    assert!(summary.contains("ctx"));
    assert!(summary.to_lowercase().contains("vision"));
}

#[test]
fn picker_keyboard_should_navigate_select_and_dismiss() {
    let mut state = ModelPickerState::default();
    assert!(!state.is_open());
    state.open_with_count(3);
    assert!(state.is_open());
    assert_eq!(state.selected(), 0);

    assert_eq!(
        state.handle_key(ModelPickerKey::Down, 3),
        ModelPickerAction::HighlightChanged { index: 1 }
    );
    assert_eq!(
        state.handle_key(ModelPickerKey::Down, 3),
        ModelPickerAction::HighlightChanged { index: 2 }
    );
    assert_eq!(
        state.handle_key(ModelPickerKey::Up, 3),
        ModelPickerAction::HighlightChanged { index: 1 }
    );
    assert_eq!(
        state.handle_key(ModelPickerKey::Enter, 3),
        ModelPickerAction::Select { index: 1 }
    );
    assert!(!state.is_open());

    state.open_with_count(2);
    assert_eq!(
        state.handle_key(ModelPickerKey::Escape, 2),
        ModelPickerAction::Dismiss
    );
    assert!(!state.is_open());
}

#[test]
fn picker_should_mark_active_entry() {
    let entries = build_picker_entries(&[(ModelProviderKind::Ollama, vec!["a".into(), "b".into()])]);
    let marked = ronin::model_picker::mark_active_entry(&entries, "ollama", "b");
    assert!(!marked[0].is_active);
    assert!(marked[1].is_active);
}

#[test]
fn open_picker_should_highlight_active_model() {
    let entries = build_picker_entries(&[(
        ModelProviderKind::Ollama,
        vec!["llama3.2".into(), "mistral".into(), "llava".into()],
    )]);
    let marked = ronin::model_picker::mark_active_entry(&entries, "ollama", "mistral");
    let mut state = ModelPickerState::default();
    open_picker_at_active(&mut state, &marked);
    assert!(state.is_open());
    assert_eq!(state.selected(), 1);
}

#[test]
fn entries_from_listed_providers_should_group_and_mark_active() {
    let listed = vec![
        ("openai".to_string(), vec!["gpt-4o".into()]),
        ("ollama".to_string(), vec!["llama3.2".into(), "llava".into()]),
        ("unknown".to_string(), vec!["skip-me".into()]),
    ];
    let entries = entries_from_listed_providers(&listed, Some("openai"), Some("gpt-4o"));
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].provider, ModelProviderKind::Ollama);
    assert_eq!(entries[0].model_name, "llama3.2");
    assert!(!entries[0].is_active);
    assert_eq!(entries[2].provider, ModelProviderKind::OpenAi);
    assert!(entries[2].is_active);
    assert!(entries[1].capabilities.supports_vision);
}

#[test]
fn refresh_picker_entries_should_add_new_models_and_keep_highlight() {
    let mut state = ModelPickerState::default();
    let initial = build_picker_entries(&[(ModelProviderKind::Ollama, vec!["llama3.2".into()])]);
    open_picker_at_active(&mut state, &initial);
    assert_eq!(state.selected(), 0);

    let refreshed = refresh_picker_entries(
        &mut state,
        &[(
            ModelProviderKind::Ollama,
            vec!["llama3.2".into(), "mistral".into()],
        )],
        "ollama",
        "llama3.2",
    );
    assert_eq!(refreshed.len(), 2);
    assert_eq!(refreshed[1].model_name, "mistral");
    assert!(refreshed[0].is_active);
    assert_eq!(state.selected(), 0);
}

#[test]
fn refresh_picker_entries_should_clamp_when_highlighted_model_removed() {
    let mut state = ModelPickerState::default();
    let initial = build_picker_entries(&[(
        ModelProviderKind::Ollama,
        vec!["a".into(), "b".into(), "c".into()],
    )]);
    state.open_with_count(3);
    assert_eq!(
        state.handle_key(ModelPickerKey::Down, 3),
        ModelPickerAction::HighlightChanged { index: 1 }
    );
    assert_eq!(
        state.handle_key(ModelPickerKey::Down, 3),
        ModelPickerAction::HighlightChanged { index: 2 }
    );

    let refreshed = refresh_picker_entries(
        &mut state,
        &[(ModelProviderKind::Ollama, vec!["a".into()])],
        "ollama",
        "a",
    );
    assert_eq!(refreshed.len(), 1);
    assert_eq!(state.selected(), 0);
}

#[test]
fn picker_row_colors_should_differ_between_themes() {
    let light = picker_row_colors(ColorScheme::Light, PickerRowTone::Highlighted);
    let dark = picker_row_colors(ColorScheme::Dark, PickerRowTone::Highlighted);
    assert_ne!(light.background, dark.background);
    assert_ne!(light.text, dark.text);

    let active_light = picker_row_colors(ColorScheme::Light, PickerRowTone::Active);
    let default_light = picker_row_colors(ColorScheme::Light, PickerRowTone::Default);
    assert_ne!(active_light.background, default_light.background);
}

#[test]
fn picker_row_tone_should_prefer_highlight_over_active() {
    assert_eq!(
        ronin::model_picker::picker_row_tone(true, true),
        PickerRowTone::Highlighted
    );
    assert_eq!(
        ronin::model_picker::picker_row_tone(false, true),
        PickerRowTone::Active
    );
    assert_eq!(
        ronin::model_picker::picker_row_tone(false, false),
        PickerRowTone::Default
    );
}
