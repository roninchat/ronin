//! Provider settings: Test Connection, import/export, and persona presentation.
//!
//! Public seams — testable without GPUI.

use ronin_app::ConnectionTestResult;
use ronin_core::{PersonaConfig, PersonaMode, RONIN_SYSTEM_PROMPT};

/// Label for the Test Connection action in provider settings.
pub fn test_connection_button_label() -> &'static str {
    "Test Connection"
}

/// Formats a connection-test outcome for display under the button.
pub fn format_connection_test_result(result: &ConnectionTestResult) -> String {
    match result {
        ConnectionTestResult::Success { message } => format!("✓ {message}"),
        ConnectionTestResult::Failure { message } => format!("✗ {message}"),
    }
}

/// Whether the result row should use the success (accent) tone vs error tone.
pub fn connection_test_is_success(result: &ConnectionTestResult) -> bool {
    result.is_success()
}

/// Label for exporting provider settings to a TOML file.
pub fn export_provider_config_button_label() -> &'static str {
    "Export Provider Config"
}

/// Label for importing provider settings from a TOML file.
pub fn import_provider_config_button_label() -> &'static str {
    "Import Provider Config"
}

/// Clarifies that exports never include API keys or other secrets.
pub fn export_excludes_secrets_notice() -> &'static str {
    "Exports include base URLs and model defaults only — API keys and tokens are never exported."
}

/// Section heading for the system-prompt / persona editor.
pub fn system_prompt_section_title() -> &'static str {
    "System Prompt"
}

/// Label for the built-in Ronin capability-boundary prompt (always shown).
pub fn built_in_system_prompt_label() -> &'static str {
    "Built-in Ronin (default)"
}

/// Returns the built-in Ronin system prompt text for display.
pub fn built_in_system_prompt_text() -> &'static str {
    RONIN_SYSTEM_PROMPT
}

/// Label for the append mode option.
pub fn persona_mode_append_label() -> &'static str {
    "Append to built-in"
}

/// Label for the replace mode option.
pub fn persona_mode_replace_label() -> &'static str {
    "Replace built-in"
}

/// Label for the custom persona text field.
pub fn custom_persona_field_label() -> &'static str {
    "Custom persona"
}

/// Label for the inspectable effective prompt preview.
pub fn effective_system_prompt_label() -> &'static str {
    "Effective system prompt (sent to the model)"
}

/// Notice that only this visible prompt is sent as the system message.
pub fn no_hidden_instructions_notice() -> &'static str {
    "No hidden system instructions are sent beyond what is shown above. Explicit attachments and enabled profile memories appear as separate, visible context."
}

/// Presentation model for the system-prompt settings editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemPromptEditorView {
    /// Section title.
    pub section_title: &'static str,
    /// Label for the built-in prompt block.
    pub built_in_label: &'static str,
    /// Full built-in prompt text.
    pub built_in_text: &'static str,
    /// Current persona mode.
    pub mode: PersonaMode,
    /// Append option label.
    pub append_label: &'static str,
    /// Replace option label.
    pub replace_label: &'static str,
    /// Custom persona field label.
    pub custom_label: &'static str,
    /// Current custom persona text.
    pub custom_text: String,
    /// Effective prompt preview label.
    pub effective_label: &'static str,
    /// Full effective prompt that will be sent.
    pub effective_text: String,
    /// Transparency notice (no hidden instructions).
    pub transparency_notice: &'static str,
}

/// Builds the system-prompt editor view from current persona settings.
pub fn system_prompt_editor_view(
    persona: &PersonaConfig,
    effective_prompt: &str,
) -> SystemPromptEditorView {
    SystemPromptEditorView {
        section_title: system_prompt_section_title(),
        built_in_label: built_in_system_prompt_label(),
        built_in_text: built_in_system_prompt_text(),
        mode: persona.mode,
        append_label: persona_mode_append_label(),
        replace_label: persona_mode_replace_label(),
        custom_label: custom_persona_field_label(),
        custom_text: persona.text.clone(),
        effective_label: effective_system_prompt_label(),
        effective_text: effective_prompt.to_string(),
        transparency_notice: no_hidden_instructions_notice(),
    }
}

/// Formats a provider-config import failure for display.
pub fn format_import_config_error(error: &str) -> String {
    format!("Import failed: {error}")
}
