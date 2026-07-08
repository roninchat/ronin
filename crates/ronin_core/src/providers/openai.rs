//! HTTP adapter for OpenAI-compatible chat completion APIs.

use std::time::Duration;

use crate::error::{Result, RoninError};
use crate::providers::model_cache::{clear_model_cache, get_cached_models};
use crate::providers::{ChatProvider, ChatRequest, ChatStreamEvent, OllamaHealth, OllamaProvider};

/// OpenAI-compatible provider that queries a remote API.
#[derive(Clone)]
pub struct OpenAiCompatibleProvider {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl OpenAiCompatibleProvider {
    /// Creates a new provider targeting the given OpenAI-compatible base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    fn get_api_key(&self) -> Result<String> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                if let Some(key) = rt.block_on(async {
                    if let Ok(ss) =
                        secret_service::SecretService::connect(secret_service::EncryptionType::Dh)
                            .await
                    {
                        if let Ok(collection) = ss.get_default_collection().await {
                            if let Ok(items) = collection
                                .search_items(std::collections::HashMap::from([
                                    ("application", "ronin"),
                                    ("service", "openai"),
                                ]))
                                .await
                            {
                                if let Some(item) = items.first() {
                                    if let Ok(secret) = item.get_secret().await {
                                        if let Ok(key) = std::str::from_utf8(&secret) {
                                            return Some(key.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None
                }) {
                    return Ok(key);
                }
            }
        }

        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            return Ok(key);
        }

        Err(RoninError::Config(
            "No API key found. Set OPENAI_API_KEY or add a key in settings.".into(),
        ))
    }

    fn fetch_models_raw(&self) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct ModelEntry {
            id: String,
        }

        #[derive(serde::Deserialize)]
        struct ListResponse {
            data: Vec<ModelEntry>,
        }

        let key = self.get_api_key()?;

        let resp: ListResponse = self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", key))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .map_err(|source| RoninError::Provider(source.to_string()))?
            .json()
            .map_err(|source| RoninError::Provider(source.to_string()))?;

        Ok(resp.data.into_iter().map(|m| m.id).collect())
    }
}

impl ChatProvider for OpenAiCompatibleProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        #[derive(serde::Serialize)]
        struct OpenAiMessage<'a> {
            role: &'a str,
            content: &'a str,
        }

        #[derive(serde::Serialize)]
        struct OpenAiRequest<'a> {
            model: &'a str,
            messages: Vec<OpenAiMessage<'a>>,
            stream: bool,
        }

        #[derive(serde::Deserialize)]
        struct OpenAiResponse {
            choices: Option<Vec<OpenAiChoice>>,
            error: Option<OpenAiError>,
        }

        #[derive(serde::Deserialize)]
        struct OpenAiChoice {
            delta: Option<OpenAiDelta>,
        }

        #[derive(serde::Deserialize)]
        struct OpenAiDelta {
            content: Option<String>,
        }

        #[derive(serde::Deserialize)]
        struct OpenAiError {
            message: String,
        }

        let mut messages = Vec::new();
        if let Some(sys) = &request.system_prompt {
            messages.push(OpenAiMessage {
                role: "system",
                content: sys,
            });
        }
        for msg in &request.messages {
            messages.push(OpenAiMessage {
                role: &msg.role,
                content: &msg.content,
            });
        }

        let body = OpenAiRequest {
            model: &request.model,
            messages,
            stream: true,
        };

        let key = self.get_api_key()?;

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", key))
            .json(&body)
            .send()
            .map_err(|e| RoninError::Provider(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            let resp_text = resp
                .text()
                .map_err(|e| RoninError::Provider(e.to_string()))?;
            return Err(RoninError::Provider(format!(
                "openai returned {status}: {resp_text}"
            )));
        }

        let (tx, rx) = std::sync::mpsc::channel::<ChatStreamEvent>();
        std::thread::spawn(move || {
            use std::io::BufRead;
            let reader = std::io::BufReader::new(resp);
            for line_result in reader.lines() {
                let line = match line_result {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx.send(ChatStreamEvent::Error(format!(
                            "failed to read response: {e}"
                        )));
                        return;
                    }
                };
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        return;
                    }
                    match serde_json::from_str::<OpenAiResponse>(data) {
                        Ok(msg) => {
                            if let Some(error) = msg.error {
                                let _ = tx.send(ChatStreamEvent::Error(error.message));
                                return;
                            }
                            if let Some(choices) = msg.choices {
                                if let Some(choice) = choices.first() {
                                    if let Some(delta) = &choice.delta {
                                        if let Some(content) = &delta.content {
                                            if !content.is_empty()
                                                && tx
                                                    .send(ChatStreamEvent::Chunk(content.clone()))
                                                    .is_err()
                                            {
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(ChatStreamEvent::Error(format!(
                                "failed to parse openai response: {e} data: {data}"
                            )));
                            return;
                        }
                    }
                }
            }
        });

        Ok(Box::new(rx.into_iter()))
    }
}

impl OllamaProvider for OpenAiCompatibleProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn check_health(&self) -> OllamaHealth {
        let key = match self.get_api_key() {
            Ok(k) => k,
            Err(_) => {
                clear_model_cache(self.name());
                return OllamaHealth::Offline;
            }
        };

        let health = match self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", key))
            .timeout(std::time::Duration::from_secs(3))
            .send()
        {
            Ok(resp) if resp.status().is_success() => OllamaHealth::Online,
            _ => OllamaHealth::Offline,
        };

        if health == OllamaHealth::Offline {
            clear_model_cache(self.name());
        }

        health
    }

    fn list_models(&self) -> Result<Vec<String>> {
        let provider = self.clone();
        get_cached_models(
            self.name(),
            Duration::from_secs(240),
            Duration::from_secs(300),
            move || provider.fetch_models_raw(),
        )
    }
}
