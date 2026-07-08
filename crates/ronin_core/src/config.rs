//! TOML configuration types for Ronin preferences and provider settings.

/// Configuration for the OpenAI provider.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct OpenAiConfig {
    /// Base URL for the OpenAI-compatible API endpoint.
    pub base_url: Option<String>,
}

/// General preferences.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct GeneralConfig {
    /// Default AI provider.
    pub default_provider: Option<String>,
    /// Default model name.
    pub default_model: Option<String>,
}

/// Ollama provider config.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

/// The root configuration object loaded from config.toml.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct RoninConfig {
    /// General preferences.
    #[serde(default)]
    pub general: GeneralConfig,
    /// Ollama provider configuration.
    #[serde(default)]
    pub ollama: OllamaConfig,
    /// OpenAI provider configuration.
    pub openai: Option<OpenAiConfig>,
}
