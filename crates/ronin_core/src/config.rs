//! TOML configuration types for Ronin preferences and provider settings.

/// Minimum sidebar width in pixels.
pub const SIDEBAR_WIDTH_MIN: f32 = 180.0;
/// Maximum sidebar width in pixels.
pub const SIDEBAR_WIDTH_MAX: f32 = 480.0;
/// Default sidebar width in pixels.
pub const SIDEBAR_WIDTH_DEFAULT: f32 = 280.0;

/// Clamps a preferred sidebar width into the supported range.
pub fn clamp_sidebar_width(width: f32) -> f32 {
    if !width.is_finite() {
        return SIDEBAR_WIDTH_DEFAULT;
    }
    width.clamp(SIDEBAR_WIDTH_MIN, SIDEBAR_WIDTH_MAX)
}

/// Width used for layout: `0` when collapsed, otherwise the clamped preference.
pub fn effective_sidebar_width(preferred: f32, collapsed: bool) -> f32 {
    if collapsed {
        0.0
    } else {
        clamp_sidebar_width(preferred)
    }
}

/// User theme preference from `config.toml`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    /// Force the polished light theme.
    Light,
    /// Force the polished dark theme.
    Dark,
    /// Follow the desktop color-scheme preference.
    #[default]
    System,
}

/// Resolved light or dark color scheme applied to the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    /// Polished light theme.
    Light,
    /// Polished dark theme.
    Dark,
}

/// Resolves a theme preference against the desktop color scheme.
pub fn resolve_color_scheme(preference: ThemePreference, system: ColorScheme) -> ColorScheme {
    match preference {
        ThemePreference::Light => ColorScheme::Light,
        ThemePreference::Dark => ColorScheme::Dark,
        ThemePreference::System => system,
    }
}

/// Configuration for the OpenAI provider.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct OpenAiConfig {
    /// Base URL for the OpenAI-compatible API endpoint.
    pub base_url: Option<String>,
}

/// General preferences.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct GeneralConfig {
    /// Default AI provider.
    pub default_provider: Option<String>,
    /// Default model name.
    pub default_model: Option<String>,
    /// When true, request a concise model-generated title after the first exchange.
    #[serde(default = "default_auto_title")]
    pub auto_title: bool,
    /// Character threshold for attachment size warnings before send.
    #[serde(default = "default_attachment_warn_chars")]
    pub attachment_warn_chars: usize,
}

fn default_auto_title() -> bool {
    true
}

fn default_attachment_warn_chars() -> usize {
    crate::context::DEFAULT_ATTACHMENT_WARN_CHARS
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_provider: None,
            default_model: None,
            auto_title: true,
            attachment_warn_chars: default_attachment_warn_chars(),
        }
    }
}

/// Ollama provider config.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct OllamaConfig {
    /// Base URL for Ollama.
    #[serde(default = "default_ollama_base_url")]
    pub base_url: String,
}

fn default_ollama_base_url() -> String {
    "http://localhost:11434".to_string()
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: default_ollama_base_url(),
        }
    }
}

/// UI chrome preferences (sidebar layout, etc.).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UiConfig {
    /// Preferred sidebar width in pixels (clamped when applied).
    #[serde(default = "default_sidebar_width")]
    pub sidebar_width: f32,
    /// Whether the sidebar is fully collapsed.
    #[serde(default)]
    pub sidebar_collapsed: bool,
}

fn default_sidebar_width() -> f32 {
    SIDEBAR_WIDTH_DEFAULT
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            sidebar_width: SIDEBAR_WIDTH_DEFAULT,
            sidebar_collapsed: false,
        }
    }
}

/// How custom persona text combines with the built-in Ronin system prompt.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PersonaMode {
    /// Append custom persona text after the built-in Ronin prompt (default).
    #[default]
    Append,
    /// Replace the built-in prompt with custom persona text only.
    Replace,
}

/// User persona / system-prompt customization stored in `config.toml`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PersonaConfig {
    /// Whether custom text appends to or replaces the built-in prompt.
    #[serde(default)]
    pub mode: PersonaMode,
    /// User-authored persona / system-prompt text.
    #[serde(default)]
    pub text: String,
}

/// Persistent file-logging preferences from `config.toml`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoggingConfig {
    /// When true, write rotated diagnostic logs under the cache logs directory.
    #[serde(default)]
    pub file_enabled: bool,
    /// Rotate the active log file after this many bytes (default 5 MiB).
    #[serde(default = "default_max_log_file_bytes")]
    pub max_file_bytes: u64,
}

fn default_max_log_file_bytes() -> u64 {
    5 * 1024 * 1024
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            file_enabled: false,
            max_file_bytes: default_max_log_file_bytes(),
        }
    }
}

/// Local-knowledge privacy preferences for folder listing / indexing.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalKnowledgeConfig {
    /// Absolute paths that must never be listed or indexed.
    #[serde(default)]
    pub never_list: Vec<String>,
    /// When true, only [`Self::allowlist`] roots are eligible for listing/indexing.
    #[serde(default)]
    pub allowlist_enabled: bool,
    /// Approved roots when allowlist mode is enabled.
    #[serde(default)]
    pub allowlist: Vec<String>,
}

/// The root configuration object loaded from config.toml.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct RoninConfig {
    /// Theme preference (`light`, `dark`, or `system`).
    #[serde(default)]
    pub theme: ThemePreference,
    /// General preferences.
    #[serde(default)]
    pub general: GeneralConfig,
    /// Ollama provider configuration.
    #[serde(default)]
    pub ollama: OllamaConfig,
    /// OpenAI provider configuration.
    pub openai: Option<OpenAiConfig>,
    /// UI chrome preferences (sidebar width / collapse).
    #[serde(default)]
    pub ui: UiConfig,
    /// Persona / system-prompt customization.
    #[serde(default)]
    pub persona: PersonaConfig,
    /// Persistent file logging preferences.
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Folder listing / local-knowledge privacy controls.
    #[serde(default)]
    pub local_knowledge: LocalKnowledgeConfig,
}

/// Portable provider settings for import/export (never includes secrets).
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
pub struct ProviderConfigExport {
    /// Default provider and model selection.
    #[serde(default)]
    pub general: GeneralConfig,
    /// Ollama base URL and related non-secret settings.
    #[serde(default)]
    pub ollama: OllamaConfig,
    /// OpenAI-compatible base URL (API keys are never included).
    #[serde(default)]
    pub openai: Option<OpenAiConfig>,
}

impl RoninConfig {
    /// Builds a portable provider-settings bundle with no secrets.
    pub fn provider_export(&self) -> ProviderConfigExport {
        ProviderConfigExport {
            general: self.general.clone(),
            ollama: self.ollama.clone(),
            openai: self.openai.clone(),
        }
    }
}

/// Serializes non-secret provider settings to TOML for file export.
pub fn export_provider_config_toml(config: &RoninConfig) -> Result<String, String> {
    let bundle = config.provider_export();
    toml::to_string_pretty(&bundle).map_err(|e| format!("serialize provider config: {e}"))
}

/// Validates a portable provider-settings bundle.
pub fn validate_provider_config_export(bundle: &ProviderConfigExport) -> Result<(), String> {
    if bundle.ollama.base_url.trim().is_empty() {
        return Err("invalid provider config: ollama.base_url must not be empty".to_string());
    }
    if let Some(openai) = &bundle.openai {
        if let Some(url) = &openai.base_url {
            if url.trim().is_empty() {
                return Err(
                    "invalid provider config: openai.base_url must not be empty".to_string()
                );
            }
        }
    }
    Ok(())
}

/// Parses and validates a provider-config TOML string, merging into `current`.
///
/// Theme, UI, and persona settings on `current` are preserved. Only provider
/// settings (`general`, `ollama`, `openai`) are replaced from the import.
pub fn import_provider_config_toml(
    current: &RoninConfig,
    toml_text: &str,
) -> Result<RoninConfig, String> {
    let bundle: ProviderConfigExport = toml::from_str(toml_text)
        .map_err(|e| format!("failed to parse provider config TOML: {e}"))?;
    validate_provider_config_export(&bundle)?;

    let mut merged = current.clone();
    merged.general = bundle.general;
    merged.ollama = bundle.ollama;
    merged.openai = bundle.openai;
    Ok(merged)
}
