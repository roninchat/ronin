//! Provider settings: Test Connection, import/export, and persona presentation seams.

use ronin::provider_settings::{
    built_in_system_prompt_label, built_in_system_prompt_text, connection_test_is_success,
    export_excludes_secrets_notice, export_provider_config_button_label,
    format_connection_test_result, format_import_config_error, import_provider_config_button_label,
    no_hidden_instructions_notice, persona_mode_append_label, persona_mode_replace_label,
    system_prompt_editor_view, test_connection_button_label, SystemPromptEditorView,
};
use ronin_app::ConnectionTestResult;
use ronin_core::{effective_system_prompt, PersonaConfig, PersonaMode, RONIN_SYSTEM_PROMPT};

#[test]
fn test_connection_button_should_have_clear_label() {
    assert_eq!(test_connection_button_label(), "Test Connection");
}

#[test]
fn connection_test_success_should_format_with_check_mark() {
    let result = ConnectionTestResult::Success {
        message: "Successfully connected to Ollama (1 model available).".into(),
    };
    let text = format_connection_test_result(&result);
    assert!(text.starts_with('✓'));
    assert!(text.to_lowercase().contains("success"));
    assert!(connection_test_is_success(&result));
}

#[test]
fn connection_test_failure_should_format_with_cross_and_details() {
    let result = ConnectionTestResult::Failure {
        message: "Ollama is not running. Start it with `ollama serve`.".into(),
    };
    let text = format_connection_test_result(&result);
    assert!(text.starts_with('✗'));
    assert!(text.to_lowercase().contains("ollama"));
    assert!(!connection_test_is_success(&result));
}

#[test]
fn import_export_buttons_should_have_clear_labels_and_secret_notice() {
    assert_eq!(
        export_provider_config_button_label(),
        "Export Provider Config"
    );
    assert_eq!(
        import_provider_config_button_label(),
        "Import Provider Config"
    );
    let notice = export_excludes_secrets_notice().to_lowercase();
    assert!(notice.contains("api key") || notice.contains("api keys"));
    assert!(notice.contains("never") || notice.contains("not"));
}

#[test]
fn system_prompt_editor_should_label_built_in_and_show_effective_prompt() {
    let persona = PersonaConfig {
        mode: PersonaMode::Append,
        text: "Be brief.".into(),
    };
    let effective = effective_system_prompt(&persona);
    let view = system_prompt_editor_view(&persona, &effective);

    assert_eq!(view.built_in_label, built_in_system_prompt_label());
    assert!(view.built_in_label.to_lowercase().contains("built-in"));
    assert!(view.built_in_label.to_lowercase().contains("default"));
    assert_eq!(view.built_in_text, built_in_system_prompt_text());
    assert_eq!(view.built_in_text, RONIN_SYSTEM_PROMPT);
    assert_eq!(view.mode, PersonaMode::Append);
    assert_eq!(view.append_label, persona_mode_append_label());
    assert_eq!(view.replace_label, persona_mode_replace_label());
    assert_eq!(view.custom_text, "Be brief.");
    assert_eq!(view.effective_text, effective);
    assert!(view.effective_text.contains("Ronin"));
    assert!(view.effective_text.contains("Be brief."));
    assert!(
        view.transparency_notice
            .to_lowercase()
            .contains("no hidden")
            || no_hidden_instructions_notice()
                .to_lowercase()
                .contains("no hidden")
    );
}

#[test]
fn system_prompt_editor_replace_mode_should_preview_custom_only() {
    let persona = PersonaConfig {
        mode: PersonaMode::Replace,
        text: "You are a thesaurus.".into(),
    };
    let effective = effective_system_prompt(&persona);
    let view: SystemPromptEditorView = system_prompt_editor_view(&persona, &effective);
    assert_eq!(view.effective_text, "You are a thesaurus.");
    assert!(!view.effective_text.contains("You are Ronin"));
    // Built-in remains visible as the labeled default even in replace mode.
    assert_eq!(view.built_in_text, RONIN_SYSTEM_PROMPT);
}

#[test]
fn import_config_error_should_be_prefixed_for_display() {
    let formatted = format_import_config_error("failed to parse provider config TOML: …");
    assert!(formatted.starts_with("Import failed:"));
    assert!(formatted.contains("parse"));
}
