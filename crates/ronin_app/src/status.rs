//! Sidebar provider/model status and provider health probing.

use ronin_core::{OllamaHealth, OllamaProvider, RoninSession};

/// Maps a raw provider/config error into a clear, actionable user message.
///
/// `provider` is the active provider id (`"ollama"` / `"openai"`). Prefer this
/// over showing HTTP status codes, response dumps, or stack traces.
pub fn format_provider_error(provider: &str, raw: &str) -> String {
    let lower = raw.to_lowercase();

    if lower.contains("no api key")
        || lower.contains("unauthorized")
        || lower.contains("invalid api key")
        || lower.contains("incorrect api key")
    {
        return "Missing API key. Set the OPENAI_API_KEY environment variable, or add a key in provider settings (Secret Service).".to_string();
    }

    if lower.contains("rate limit")
        || lower.contains("rate_limit")
        || lower.contains("too many requests")
        || lower.contains("429")
    {
        if let Some(seconds) = extract_retry_seconds(&lower) {
            return format!(
                "Rate limited, try again in {seconds} seconds. Wait briefly, then retry the request."
            );
        }
        return "Rate limited. Try again in a few seconds.".to_string();
    }

    if lower.contains("context length")
        || lower.contains("context window")
        || lower.contains("maximum context")
        || lower.contains("prompt is too long")
        || lower.contains("input length exceeds")
        || (lower.contains("too long") && lower.contains("token"))
    {
        return "Message too long for the model's context window. Reduce the conversation length, remove large attachments, or switch to a model with a larger context.".to_string();
    }

    if lower.contains("model_not_found")
        || lower.contains("does not exist")
        || lower.contains("model not found")
        || (lower.contains("model") && lower.contains("not found"))
        || (lower.contains("404")
            && (lower.contains("model") || provider == "ollama" || provider == "openai"))
    {
        return if provider == "ollama" {
            "Model not found. Check the model name, or pull it with `ollama pull <model>`.".to_string()
        } else {
            "Model not found. Check the model name in provider settings, or pick an available model.".to_string()
        };
    }

    let looks_unreachable = lower.contains("connection refused")
        || lower.contains("tcp connect")
        || lower.contains("error sending request")
        || lower.contains("connect error")
        || lower.contains("timed out")
        || lower.contains("timeout")
        || lower.contains("dns error")
        || lower.contains("name or service not known")
        || lower.contains("502")
        || lower.contains("503")
        || lower.contains("504");

    if looks_unreachable {
        if provider == "ollama" || lower.contains("ollama") || lower.contains("11434") {
            return "Ollama is not running. Start it with `ollama serve`, or install Ollama from https://ollama.com and try again.".to_string();
        }
        return "Could not reach the OpenAI-compatible endpoint. Check the base URL in provider settings and your network connection.".to_string();
    }

    // Strip common "provider returned STATUS: " prefixes when nothing else matched.
    sanitize_generic_provider_dump(raw)
}

fn extract_retry_seconds(lower: &str) -> Option<u64> {
    // Prefer "retry after N seconds" / "try again in N seconds".
    for marker in ["retry after ", "try again in ", "retry in "] {
        if let Some(idx) = lower.find(marker) {
            let rest = &lower[idx + marker.len()..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse::<u64>() {
                if n > 0 {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn sanitize_generic_provider_dump(raw: &str) -> String {
    let trimmed = raw.trim();
    // Drop leading "ollama returned 500: " / "openai returned 400: " style prefixes.
    if let Some(idx) = trimmed.find(": ") {
        let (head, tail) = trimmed.split_at(idx);
        let head_l = head.to_lowercase();
        if (head_l.contains("returned") || head_l.contains("status"))
            && head.chars().any(|c| c.is_ascii_digit())
        {
            let body = tail.trim_start_matches(':').trim();
            if !body.is_empty() && body.len() < 400 && !body.starts_with('{') {
                return body.to_string();
            }
            return "Provider request failed. Check provider settings, then try again.".to_string();
        }
    }
    if trimmed.len() > 400 || trimmed.contains("stack backtrace") {
        return "Provider request failed. Check provider settings, then try again.".to_string();
    }
    trimmed.to_string()
}

/// Outcome of an explicit provider "Test Connection" check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionTestResult {
    /// Endpoint reachable and authentication (when required) succeeded.
    Success {
        /// User-facing success summary.
        message: String,
    },
    /// Reachability or authentication failed.
    Failure {
        /// Actionable failure details.
        message: String,
    },
}

impl ConnectionTestResult {
    /// User-facing message for either outcome.
    pub fn message(&self) -> &str {
        match self {
            Self::Success { message } | Self::Failure { message } => message,
        }
    }

    /// Whether the connection test succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
}

/// Verifies provider endpoint reachability and authentication (via model list).
pub fn run_connection_test(provider: &impl OllamaProvider) -> ConnectionTestResult {
    let name = provider.name();

    match provider.check_health() {
        OllamaHealth::Offline => {
            // Prefer auth/config errors from list_models when health is offline
            // solely because a key is missing.
            if let Err(err) = provider.list_models() {
                let raw = err.to_string();
                let lower = raw.to_lowercase();
                if lower.contains("no api key") || lower.contains("api key") {
                    return ConnectionTestResult::Failure {
                        message: format_provider_error(name, &raw),
                    };
                }
                return ConnectionTestResult::Failure {
                    message: format_provider_error(name, &raw),
                };
            }
            return ConnectionTestResult::Failure {
                message: format_provider_error(name, "connection refused"),
            };
        }
        OllamaHealth::Online => {}
    }

    match provider.list_models() {
        Ok(models) => {
            let count = models.len();
            let label = if name == "openai" { "OpenAI" } else { "Ollama" };
            let message = if count == 0 {
                format!(
                    "Connected to {label}, but no models are available. Pull or configure a model, then try again."
                )
            } else if count == 1 {
                format!("Successfully connected to {label} (1 model available).")
            } else {
                format!("Successfully connected to {label} ({count} models available).")
            };
            ConnectionTestResult::Success { message }
        }
        Err(err) => ConnectionTestResult::Failure {
            message: format_provider_error(name, &err.to_string()),
        },
    }
}

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
                ProviderStatus::OpenAiError {
                    message: format_provider_error("openai", &clean_msg),
                }
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
