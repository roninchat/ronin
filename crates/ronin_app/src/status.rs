//! Sidebar provider/model status and provider health probing.

use ronin_core::{OllamaHealth, OllamaProvider, RoninSession};

/// Basic provider/model status shown in the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderStatus {
    /// No provider or model is configured yet.
    NotConfigured,
    /// Ollama provider is selected but could not be reached.
    OllamaOffline,
    /// Ollama is reachable and a model is selected.
    OllamaOnline {
        /// Name of the selected model.
        model: String,
    },
    /// Ollama is reachable but no models are installed.
    OllamaNoModels,
    /// OpenAI is configured, reachable and a model is selected.
    OpenAiReady {
        /// Name of the selected model.
        model: String,
    },
    /// OpenAI provider health check or model list failed.
    OpenAiError {
        /// Sanitized error message.
        message: String,
    },
    /// OpenAI is selected but no API key has been configured.
    OpenAiNotConfigured,
}

/// Selects the active model: the saved choice when still available, otherwise
/// the first listed model. Persists the selection back to config.
fn select_and_save_model(session: &RoninSession, models: &[String]) -> String {
    let saved = session.load_selected_model().unwrap_or(None);
    let model = match saved {
        Some(m) if models.contains(&m) => m,
        _ => models[0].clone(),
    };
    let _ = session.save_selected_model(&model);
    model
}

/// Probes an OpenAI-compatible provider and maps the outcome to a status.
pub(crate) fn probe_openai_status(
    provider: &impl OllamaProvider,
    session: &RoninSession,
) -> ProviderStatus {
    match provider.list_models() {
        Ok(models) if !models.is_empty() => {
            let model = select_and_save_model(session, &models);
            ProviderStatus::OpenAiReady { model }
        }
        Err(err) => {
            let msg = err.to_string();
            if msg.contains("No API key found") {
                ProviderStatus::OpenAiNotConfigured
            } else {
                let clean_msg = match &err {
                    ronin_core::RoninError::Provider(inner) => inner.clone(),
                    _ => msg,
                };
                ProviderStatus::OpenAiError { message: clean_msg }
            }
        }
        _ => ProviderStatus::OpenAiError {
            message: "No models returned from OpenAI".to_string(),
        },
    }
}

/// Probes an Ollama provider and maps the outcome to a status.
pub(crate) fn probe_ollama_status(
    provider: &impl OllamaProvider,
    session: &RoninSession,
) -> ProviderStatus {
    match provider.check_health() {
        OllamaHealth::Online => match provider.list_models() {
            Ok(models) if !models.is_empty() => {
                let model = select_and_save_model(session, &models);
                ProviderStatus::OllamaOnline { model }
            }
            _ => ProviderStatus::OllamaNoModels,
        },
        OllamaHealth::Offline => ProviderStatus::OllamaOffline,
    }
}

/// Probes any provider, routing on its reported name.
pub(crate) fn probe_provider_status(
    provider: &impl OllamaProvider,
    session: &RoninSession,
) -> ProviderStatus {
    if provider.name() == "openai" {
        probe_openai_status(provider, session)
    } else {
        probe_ollama_status(provider, session)
    }
}
