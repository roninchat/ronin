//! HTTP adapter for the local Ollama REST API.

use std::time::Duration;

use crate::error::{Result, RoninError};
use crate::providers::model_cache::{clear_model_cache, get_cached_models};
use crate::providers::{
    ChatMessage, ChatProvider, ChatRequest, ChatStreamEvent, OllamaHealth, OllamaProvider,
};

/// HTTP-based Ollama provider that queries the Ollama REST API.
#[derive(Clone)]
pub struct HttpOllamaProvider {
    base_url: String,
}

impl HttpOllamaProvider {
    /// Creates a new provider targeting the given Ollama base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    fn fetch_models_raw(&self) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct ModelEntry {
            name: String,
        }

        #[derive(serde::Deserialize)]
        struct ListResponse {
            models: Vec<ModelEntry>,
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|source| RoninError::Provider(source.to_string()))?;

        let resp: ListResponse = client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .map_err(|source| RoninError::Provider(source.to_string()))?
            .json()
            .map_err(|source| RoninError::Provider(source.to_string()))?;

        Ok(resp.models.into_iter().map(|m| m.name).collect())
    }
}

impl ChatProvider for HttpOllamaProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        #[derive(serde::Serialize)]
        struct OllamaChatRequest<'a> {
            model: &'a str,
            messages: &'a [ChatMessage],
            stream: bool,
        }

        #[derive(serde::Deserialize)]
        struct OllamaChatResponse {
            message: Option<OllamaChatMessage>,
            done: Option<bool>,
            error: Option<String>,
        }

        #[derive(serde::Deserialize)]
        struct OllamaChatMessage {
            content: String,
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| RoninError::Provider(e.to_string()))?;

        let body = OllamaChatRequest {
            model: &request.model,
            messages: &request.messages,
            stream: true,
        };

        let resp = client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .map_err(|e| RoninError::Provider(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            let resp_text = resp
                .text()
                .map_err(|e| RoninError::Provider(e.to_string()))?;
            return Err(RoninError::Provider(format!(
                "ollama returned {status}: {resp_text}"
            )));
        }

        // Spawn a thread to read the response body line-by-line (true streaming)
        // and send parsed chunks through a channel. The returned iterator reads
        // from the channel receiver, blocking only until the next chunk arrives.
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
                match serde_json::from_str::<OllamaChatResponse>(line) {
                    Ok(msg) => {
                        if let Some(error) = msg.error {
                            let _ = tx.send(ChatStreamEvent::Error(error));
                            return;
                        }
                        if let Some(message) = msg.message {
                            let content = message.content;
                            if !content.is_empty()
                                && tx.send(ChatStreamEvent::Chunk(content)).is_err()
                            {
                                return; // Receiver dropped
                            }
                        }
                        if msg.done == Some(true) {
                            return;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(ChatStreamEvent::Error(format!(
                            "failed to parse ollama response: {e}"
                        )));
                        return;
                    }
                }
            }
            // If we get here, the stream ended without a `done: true` marker.
            // This is fine — the loop simply ends.
        });

        Ok(Box::new(rx.into_iter()))
    }
}

impl OllamaProvider for HttpOllamaProvider {
    fn check_health(&self) -> OllamaHealth {
        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                clear_model_cache(self.name());
                return OllamaHealth::Offline;
            }
        };

        let health = match client.get(format!("{}/api/version", self.base_url)).send() {
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
