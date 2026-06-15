#![deny(missing_docs)]

//! Public application/session boundary for Ronin.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ronin_db::{
    init_tracing, DbArtifact, DbAttachment, DbMemory, DbMessage, DbThread, RoninDb, RoninDbError,
};

/// Result type returned by `ronin_core` operations.
pub type Result<T> = std::result::Result<T, RoninError>;

/// Ollama server health status reported through the provider boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaHealth {
    /// Ollama server is reachable and responding.
    Online,
    /// Ollama server is not reachable.
    Offline,
}

/// Provider boundary for querying Ollama status and available models.
pub trait OllamaProvider {
    /// Opaque name for identifying this provider type (e.g. "ollama", "openai").
    fn name(&self) -> &'static str {
        "ollama"
    }
    /// Checks whether the Ollama server is reachable.
    fn check_health(&self) -> OllamaHealth;
    /// Lists available model names from the provider.
    fn list_models(&self) -> Result<Vec<String>>;
}

/// A request to send to a chat provider.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    /// Model name to use for this request.
    pub model: String,
    /// Conversation messages to include as context.
    pub messages: Vec<ChatMessage>,
    /// Optional system prompt prepended to the request (not persisted).
    pub system_prompt: Option<String>,
}

/// A message in a chat request (role + content pairs).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ChatMessage {
    /// Message role for the provider.
    pub role: String,
    /// Message content.
    pub content: String,
}

/// A parsed explicit context reference from the composer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextToolRef {
    /// User requested a file attachment by path.
    File(String),
    /// User requested a memory attachment by id.
    Memory(String),
    /// User requested an artifact attachment by id.
    Artifact(String),
    /// User requested current clipboard text.
    Clipboard,
}

/// Parsed composer text and explicit context references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedContextTools {
    /// User-visible prompt after context references are removed.
    pub visible_message: String,
    /// Explicit context references found in source order.
    pub refs: Vec<ContextToolRef>,
}

/// Parses explicit `@file:<path>` and `@clipboard` context refs from composer text.
pub fn parse_context_tools(input: &str) -> ParsedContextTools {
    let mut refs = Vec::new();
    let mut visible = String::new();
    let mut rest = input;

    while let Some(at) = find_next_context_ref(rest) {
        visible.push_str(&rest[..at]);
        let candidate = &rest[at..];

        if let Some(after_file) = candidate.strip_prefix("@file:") {
            let (path, consumed) = parse_file_ref(after_file);
            if !path.is_empty() {
                refs.push(ContextToolRef::File(path));
                rest = &candidate["@file:".len() + consumed..];
                continue;
            }
        }

        if let Some(after_memory) = candidate.strip_prefix("@memory:") {
            let (id, consumed) = parse_file_ref(after_memory);
            if !id.is_empty() {
                refs.push(ContextToolRef::Memory(id));
                rest = &candidate["@memory:".len() + consumed..];
                continue;
            }
        }

        if let Some(after_artifact) = candidate.strip_prefix("@artifact:") {
            let (id, consumed) = parse_file_ref(after_artifact);
            if !id.is_empty() {
                refs.push(ContextToolRef::Artifact(id));
                rest = &candidate["@artifact:".len() + consumed..];
                continue;
            }
        }

        if candidate.len() >= "@clipboard".len()
            && candidate[.."@clipboard".len()].eq_ignore_ascii_case("@clipboard")
            && is_ref_boundary(candidate["@clipboard".len()..].chars().next())
        {
            refs.push(ContextToolRef::Clipboard);
            rest = &candidate["@clipboard".len()..];
            continue;
        }

        visible.push('@');
        rest = &candidate['@'.len_utf8()..];
    }

    visible.push_str(rest);

    ParsedContextTools {
        visible_message: visible.split_whitespace().collect::<Vec<_>>().join(" "),
        refs,
    }
}

fn find_next_context_ref(input: &str) -> Option<usize> {
    input.match_indices('@').find_map(|(idx, _)| {
        let candidate = &input[idx..];
        if candidate.starts_with("@file:")
            || candidate.starts_with("@memory:")
            || candidate.starts_with("@artifact:")
            || candidate
                .get(.."@clipboard".len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("@clipboard"))
        {
            Some(idx)
        } else {
            None
        }
    })
}

fn parse_file_ref(input: &str) -> (String, usize) {
    if let Some(quoted) = input.strip_prefix('"') {
        if let Some(end) = quoted.find('"') {
            return (quoted[..end].to_string(), end + 2);
        }
        return (quoted.to_string(), input.len());
    }

    let consumed = input
        .char_indices()
        .find_map(|(idx, ch)| ch.is_whitespace().then_some(idx))
        .unwrap_or(input.len());
    (input[..consumed].to_string(), consumed)
}

fn is_ref_boundary(next: Option<char>) -> bool {
    next.is_none_or(char::is_whitespace)
}

/// Maximum file attachment size in bytes.
pub const MAX_FILE_ATTACHMENT_BYTES: u64 = 1_048_576;

/// Context attachment prepared from an explicit user action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextAttachmentDraft {
    /// Attachment kind.
    pub kind: AttachmentKind,
    /// Display name shown to users and persisted with metadata.
    pub name: String,
    /// MIME type if known; text attachments default to `text/plain`.
    pub mime_type: String,
    /// Clipboard text content; file content is not persisted here.
    pub content: Option<String>,
    /// Source file path for file attachments.
    pub path: Option<PathBuf>,
    /// Provider context block generated from this attachment.
    pub context_block: String,
    /// File size in bytes when attachment came from disk.
    pub size_bytes: Option<u64>,
}

/// Errors produced while resolving explicit context attachments.
#[derive(Debug, thiserror::Error)]
pub enum ContextToolError {
    /// File metadata could not be read.
    #[error("failed to read file metadata for {path}: {source}")]
    FileMetadata {
        /// User-visible file path.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// User selected a directory instead of a regular file.
    #[error("file {path} is a directory")]
    IsDirectory {
        /// User-visible file path.
        path: PathBuf,
    },
    /// File exceeds configured size limit.
    #[error("file {path} exceeds 1 MB attachment limit")]
    FileTooLarge {
        /// User-visible file path.
        path: PathBuf,
    },
    /// File appears binary and should not be injected into prompt context.
    #[error("file {path} appears to be binary")]
    BinaryFile {
        /// User-visible file path.
        path: PathBuf,
    },
    /// File content could not be read as text.
    #[error("failed to read file {path}: {source}")]
    ReadFile {
        /// User-visible file path.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },
}

/// Reads a text file selected by explicit `@file` context.
pub fn read_file_attachment(
    path: impl AsRef<Path>,
    cwd: impl AsRef<Path>,
) -> std::result::Result<ContextAttachmentDraft, ContextToolError> {
    let requested_path = path.as_ref();
    let resolved_path = if requested_path.is_absolute() {
        requested_path.to_path_buf()
    } else {
        cwd.as_ref().join(requested_path)
    };

    let metadata =
        std::fs::metadata(&resolved_path).map_err(|source| ContextToolError::FileMetadata {
            path: requested_path.to_path_buf(),
            source,
        })?;

    if metadata.is_dir() {
        return Err(ContextToolError::IsDirectory {
            path: requested_path.to_path_buf(),
        });
    }

    if metadata.len() > MAX_FILE_ATTACHMENT_BYTES {
        return Err(ContextToolError::FileTooLarge {
            path: requested_path.to_path_buf(),
        });
    }

    let bytes = std::fs::read(&resolved_path).map_err(|source| ContextToolError::ReadFile {
        path: requested_path.to_path_buf(),
        source,
    })?;

    if bytes.iter().take(8 * 1024).any(|byte| *byte == 0) {
        return Err(ContextToolError::BinaryFile {
            path: requested_path.to_path_buf(),
        });
    }

    let text = String::from_utf8_lossy(&bytes).into_owned();
    let name = resolved_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attached file")
        .to_string();

    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::File,
        name: name.clone(),
        mime_type: "text/plain".to_string(),
        content: None,
        path: Some(resolved_path),
        context_block: format!("[Attached file: {name}]\n{text}"),
        size_bytes: Some(metadata.len()),
    })
}

/// Builds a clipboard context attachment from text read by the UI boundary.
pub fn clipboard_attachment(text: &str) -> ContextAttachmentDraft {
    ContextAttachmentDraft {
        kind: AttachmentKind::Clipboard,
        name: "clipboard".to_string(),
        mime_type: "text/plain".to_string(),
        content: Some(text.to_string()),
        path: None,
        context_block: format!("[Clipboard content]\n{text}"),
        size_bytes: Some(text.len() as u64),
    }
}

/// Builds a memory context attachment from a memory object.
pub fn memory_attachment(memory: &Memory) -> ContextAttachmentDraft {
    ContextAttachmentDraft {
        kind: AttachmentKind::Memory,
        name: format!("memory:{}", memory.title),
        mime_type: "text/plain".to_string(),
        content: Some(memory.content.clone()),
        path: None,
        context_block: format!("[Memory: {}]\n{}", memory.title, memory.content),
        size_bytes: Some(memory.content.len() as u64),
    }
}

/// Builds an artifact context attachment from an artifact object.
pub fn artifact_attachment(artifact: &Artifact) -> ContextAttachmentDraft {
    ContextAttachmentDraft {
        kind: AttachmentKind::Artifact,
        name: format!("artifact:{}", artifact.title),
        mime_type: "text/plain".to_string(),
        content: Some(artifact.content.clone()),
        path: None,
        context_block: format!("[Artifact: {}]\n{}", artifact.title, artifact.content),
        size_bytes: Some(artifact.content.len() as u64),
    }
}

/// An event emitted during a streaming chat response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatStreamEvent {
    /// A partial chunk of the assistant response.
    Chunk(String),
    /// The stream encountered an error.
    Error(String),
}

/// Provider boundary for streaming chat requests.
pub trait ChatProvider {
    /// Initiates a streaming chat request.
    ///
    /// Returns an iterator of stream events. Callers should drain the iterator
    /// to receive all chunks, then finalize the response.
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>>;
}

/// System prompt describing Ronin's capability boundary.
pub const RONIN_SYSTEM_PROMPT: &str = r#"You are Ronin, a local AI assistant on Linux.
Answer questions directly, concisely, and truthfully.
You do not have user memories in your context by default.
Instead, you can use these tools to search and fetch user memories:

- `[TOOL_CALL: list_memories]`: Returns a list of all memory IDs and titles. Use this to find what memories exist.
- `[TOOL_CALL: get_memory, id: "<id>"]`: Returns the content of a specific memory by ID. Use this to read the details of a memory.

When you call a tool, stop generation immediately. The system will append the tool results as `[TOOL_RESULT: ...]`. You must then continue generation in your next turn using the fetched information.

Examples:
1. User: "What is my name?"
You: "Let me check your memories. [TOOL_CALL: list_memories]"
System: "[TOOL_RESULT: list_memories, result: "ID, Title\n019ecc48, User's Name\n"]"
You: "I found a memory about your name. Let me fetch it. [TOOL_CALL: get_memory, id: "019ecc48"]"
System: "[TOOL_RESULT: get_memory, result: "Alice"]"
You: "Your name is Alice."

2. User: "Do I like coffee?"
You: "Let me check your memories. [TOOL_CALL: list_memories]"
System: "[TOOL_RESULT: list_memories, result: "ID, Title\n019ecc48, Food Preferences\n"]"
You: "Let me fetch your food preferences. [TOOL_CALL: get_memory, id: "019ecc48"]"
System: "[TOOL_RESULT: get_memory, result: "Prefers tea over coffee"]"
You: "No, according to your preferences, you prefer tea over coffee.""#;

/// HTTP-based Ollama provider that queries the Ollama REST API.
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
            Err(_) => return OllamaHealth::Offline,
        };

        match client.get(format!("{}/api/version", self.base_url)).send() {
            Ok(resp) if resp.status().is_success() => OllamaHealth::Online,
            _ => OllamaHealth::Offline,
        }
    }

    fn list_models(&self) -> Result<Vec<String>> {
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

/// OpenAI-compatible provider that queries a remote API.
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
            Err(_) => return OllamaHealth::Offline,
        };

        match self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", key))
            .timeout(std::time::Duration::from_secs(3))
            .send()
        {
            Ok(resp) if resp.status().is_success() => OllamaHealth::Online,
            _ => OllamaHealth::Offline,
        }
    }

    fn list_models(&self) -> Result<Vec<String>> {
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

/// Errors returned by Ronin's public session boundary.
#[derive(Debug, thiserror::Error)]
pub enum RoninError {
    /// Ronin could not create or access its configuration directory.
    #[error("failed to create Ronin config directory at {path}")]
    CreateConfigDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin could not create or access its data directory.
    #[error("failed to create Ronin data directory at {path}")]
    CreateDataDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin's SQLite persistence layer failed.
    #[error(transparent)]
    Db(#[from] RoninDbError),

    /// Provider operation failed.
    #[error("provider error: {0}")]
    Provider(String),

    /// Ronin configuration read/write failed.
    #[error("config error: {0}")]
    Config(String),
}

/// Filesystem locations used by a Ronin app session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoninPaths {
    /// Directory for user-editable or generated configuration files.
    pub config_dir: PathBuf,
    /// Directory for local application data such as `ronin.db`.
    pub data_dir: PathBuf,
}

/// User-visible conversation thread metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    /// App-generated opaque thread identifier.
    pub id: String,
    /// User-visible title. New M0 threads start as `New Chat`.
    pub title: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
    /// Whether the thread is archived. M0 stores this but has no UI for it yet.
    pub archived: bool,
    /// Opaque identifier for the AI provider (e.g. "ollama", "openai").
    pub provider: Option<String>,
    /// Selected model name.
    pub model: Option<String>,
}

/// Role of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    /// Message sent by the user.
    User,
    /// Response generated by the assistant.
    Assistant,
    /// System-level instruction (not persisted).
    System,
}

/// Lifecycle status of an assistant message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    /// Message is fully written and complete.
    Complete,
    /// Message is being generated; content may be partial.
    Streaming,
    /// Generation failed or was cancelled.
    Error,
    /// User cancelled the generation.
    Cancelled,
    /// Generation was interrupted before completion (e.g. app exited).
    Failed,
}

/// A chat message within a thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// App-generated opaque message identifier.
    pub id: String,
    /// Owning thread identifier.
    pub thread_id: String,
    /// Message role.
    pub role: MessageRole,
    /// Message body content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Lifecycle status.
    pub status: MessageStatus,
    /// Sanitized failure reason when status is `Error`.
    pub error_message: Option<String>,
}

/// Opaque identifier for an Artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactId(pub String);

/// Opaque identifier for a Memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryId(pub String);

/// Opaque identifier for an Attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentId(pub String);

/// Kind of an attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentKind {
    /// A file path attachment.
    File,
    /// A pasted clipboard attachment.
    Clipboard,
    /// A memory attachment.
    Memory,
    /// An artifact attachment.
    Artifact,
}

/// A persisted artifact linked to a specific message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    /// App-generated opaque artifact identifier.
    pub id: ArtifactId,
    /// Owning thread identifier.
    pub thread_id: String,
    /// Originating message identifier.
    pub message_id: String,
    /// Artifact title.
    pub title: String,
    /// Artifact content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
}

/// A persisted core memory piece.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    /// App-generated opaque memory identifier.
    pub id: MemoryId,
    /// Memory title.
    pub title: String,
    /// Memory content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
}

/// A persisted attachment linked to a specific message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    /// App-generated opaque attachment identifier.
    pub id: AttachmentId,
    /// Owning message identifier.
    pub message_id: String,
    /// Kind of attachment.
    pub kind: AttachmentKind,
    /// Attachment filename or paste name.
    pub name: String,
    /// MIME type of the attachment.
    pub mime_type: String,
    /// Text content for clipboard attachments.
    pub content: Option<String>,
    /// Path for file attachments.
    pub path: Option<String>,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
}

/// Open Ronin application session backed by local filesystem state.
pub struct RoninSession {
    db: RoninDb,
    paths: RoninPaths,
}

impl RoninSession {
    /// Opens a Ronin session against the provided paths.
    ///
    /// Creates the config and data directories when they do not already exist,
    /// opens `ronin.db` in the data directory, and applies pending migrations.
    pub fn open(paths: RoninPaths) -> Result<Self> {
        init_tracing();
        tracing::info!("opening ronin session");
        fs::create_dir_all(&paths.config_dir).map_err(|source| RoninError::CreateConfigDir {
            path: paths.config_dir.clone(),
            source,
        })?;
        tracing::info!(config_dir = %paths.config_dir.display(), "ronin config directory ready");

        fs::create_dir_all(&paths.data_dir).map_err(|source| RoninError::CreateDataDir {
            path: paths.data_dir.clone(),
            source,
        })?;
        tracing::info!(data_dir = %paths.data_dir.display(), "ronin data directory ready");

        let db = RoninDb::open(paths.data_dir.join("ronin.db"))?;
        let session = Self { db, paths };
        session.repair_stale_streaming_messages()?;
        Ok(session)
    }

    fn repair_stale_streaming_messages(&self) -> Result<()> {
        let stale_msgs = self.db.find_stale_streaming_messages()?;
        for msg in stale_msgs {
            tracing::info!(message_id = %msg.id, "repairing stale streaming message on startup");
            self.db.update_message_status(
                &msg.id,
                "failed",
                Some("Generation interrupted because Ronin exited before the response completed."),
            )?;
        }
        Ok(())
    }

    /// Creates a new user-visible thread titled `New Chat` and persists it.
    pub fn create_thread(&self) -> Result<Thread> {
        let config = self.load_config()?;
        self.db
            .create_thread_with_provider(
                config.general.default_provider.as_deref(),
                config.general.default_model.as_deref(),
            )
            .map(Thread::from)
            .map_err(Into::into)
    }

    /// Lists persisted threads in stable creation order.
    pub fn list_threads(&self) -> Result<Vec<Thread>> {
        self.db
            .list_threads()
            .map(|threads| threads.into_iter().map(Thread::from).collect())
            .map_err(Into::into)
    }

    /// Updates a thread's title and bumps its updated_at timestamp.
    pub fn update_thread_title(&self, thread_id: &str, title: &str) -> Result<()> {
        self.db
            .update_thread_title(thread_id, title)
            .map_err(Into::into)
    }

    /// Updates a thread's provider and bumps its updated_at timestamp.
    pub fn set_thread_provider(&self, thread_id: &str, provider: &str) -> Result<()> {
        self.db
            .update_thread_provider(thread_id, Some(provider))
            .map_err(Into::into)
    }

    /// Updates a thread's model and bumps its updated_at timestamp.
    pub fn set_thread_model(&self, thread_id: &str, model: &str) -> Result<()> {
        self.db
            .update_thread_model(thread_id, Some(model))
            .map_err(Into::into)
    }

    /// Creates and persists a new message in the given thread.
    pub fn create_message(
        &self,
        thread_id: &str,
        role: MessageRole,
        content: &str,
    ) -> Result<Message> {
        let db_role = match role {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::System => "system",
        };
        self.db
            .create_message(thread_id, db_role, content, "complete")
            .map(Message::from)
            .map_err(Into::into)
    }

    /// Creates and persists a streaming assistant message placeholder.
    pub fn create_streaming_message(&self, thread_id: &str, content: &str) -> Result<Message> {
        self.db
            .create_message(thread_id, "assistant", content, "streaming")
            .map(Message::from)
            .map_err(Into::into)
    }

    /// Lists messages for a thread in creation order.
    pub fn list_messages(&self, thread_id: &str) -> Result<Vec<Message>> {
        self.db
            .list_messages_for_thread(thread_id)
            .map(|msgs| msgs.into_iter().map(Message::from).collect())
            .map_err(Into::into)
    }

    /// Replaces an assistant message's content and sets status to complete.
    pub fn complete_message(&self, message_id: &str, content: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "complete", None)
            .map_err(Into::into)
    }

    /// Cancels a streaming message, saving partial output.
    pub fn cancel_message(&self, message_id: &str, content: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "cancelled", None)
            .map_err(Into::into)
    }

    /// Deletes a message.
    pub fn delete_message(&self, message_id: &str) -> Result<()> {
        self.db.delete_message(message_id).map_err(Into::into)
    }

    /// Fails a message with an error.
    pub fn fail_message(&self, message_id: &str, content: &str, error_message: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "failed", Some(error_message))
            .map_err(Into::into)
    }

    /// Loads the previously selected Ollama model from config, if any.
    pub fn load_selected_model(&self) -> Result<Option<String>> {
        let config = self.load_config()?;
        Ok(config.general.default_model)
    }

    /// Creates an independent session handle pointing at the same database.
    ///
    /// Opens a separate SQLite connection so the caller can write from a
    /// background thread without blocking the main session.
    pub fn clone_session(&self) -> Result<Self> {
        Self::open(self.paths.clone())
    }

    /// Saves the selected Ollama model to config.
    pub fn save_selected_model(&self, model: &str) -> Result<()> {
        let mut config = self.load_config()?;
        config.general.default_model = Some(model.to_string());
        let config_path = self.paths.config_dir.join("config.toml");
        let data = toml::to_string_pretty(&config)
            .map_err(|e| RoninError::Config(format!("serialize config: {e}")))?;
        fs::write(&config_path, data)
            .map_err(|e| RoninError::Config(format!("write config: {e}")))?;
        tracing::info!(model = %model, "saved selected model to config");
        Ok(())
    }

    /// Loads the config.toml file if it exists.
    pub fn load_config(&self) -> Result<RoninConfig> {
        let config_path = self.paths.config_dir.join("config.toml");
        if !config_path.is_file() {
            return Ok(RoninConfig::default());
        }
        let data = fs::read_to_string(&config_path)
            .map_err(|e| RoninError::Config(format!("read config.toml: {e}")))?;
        toml::from_str(&data).map_err(|e| RoninError::Config(format!("parse config.toml: {e}")))
    }

    /// Creates a new artifact.
    pub fn create_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
    ) -> Result<Artifact> {
        self.db
            .create_artifact(thread_id, message_id, title, content)
            .map(Artifact::from)
            .map_err(Into::into)
    }

    /// Lists artifacts for a thread.
    pub fn list_artifacts(&self, thread_id: &str) -> Result<Vec<Artifact>> {
        self.db
            .list_artifacts_for_thread(thread_id)
            .map(|artifacts| artifacts.into_iter().map(Artifact::from).collect())
            .map_err(Into::into)
    }

    /// Lists all artifacts across all threads, newest first.
    pub fn list_all_artifacts(&self) -> Result<Vec<Artifact>> {
        self.db
            .list_all_artifacts()
            .map(|artifacts| artifacts.into_iter().map(Artifact::from).collect())
            .map_err(Into::into)
    }

    /// Deletes an artifact.
    pub fn delete_artifact(&self, id: &ArtifactId) -> Result<()> {
        self.db.delete_artifact(&id.0).map_err(Into::into)
    }

    /// Creates a new memory.
    pub fn create_memory(&self, title: &str, content: &str) -> Result<Memory> {
        self.db
            .create_memory(title, content)
            .map(Memory::from)
            .map_err(Into::into)
    }

    /// Lists all memories.
    pub fn list_memories(&self) -> Result<Vec<Memory>> {
        self.db
            .list_all_memories()
            .map(|memories| memories.into_iter().map(Memory::from).collect())
            .map_err(Into::into)
    }

    /// Updates a memory.
    pub fn update_memory(&self, id: &MemoryId, title: &str, content: &str) -> Result<()> {
        self.db
            .update_memory(&id.0, title, content)
            .map_err(Into::into)
    }

    /// Deletes a memory.
    pub fn delete_memory(&self, id: &MemoryId) -> Result<()> {
        self.db.delete_memory(&id.0).map_err(Into::into)
    }

    /// Creates a new attachment.
    pub fn create_attachment(
        &self,
        message_id: &str,
        kind: AttachmentKind,
        name: &str,
        mime_type: &str,
        content: Option<&str>,
        path: Option<&str>,
    ) -> Result<Attachment> {
        let db_kind = match kind {
            AttachmentKind::File => "file",
            AttachmentKind::Clipboard => "clipboard",
            AttachmentKind::Memory => "memory",
            AttachmentKind::Artifact => "artifact",
        };
        self.db
            .create_attachment(message_id, db_kind, name, mime_type, content, path)
            .map(Attachment::from)
            .map_err(Into::into)
    }

    /// Lists attachments for a message.
    pub fn list_attachments(&self, message_id: &str) -> Result<Vec<Attachment>> {
        self.db
            .list_attachments_for_message(message_id)
            .map(|attachments| attachments.into_iter().map(Attachment::from).collect())
            .map_err(Into::into)
    }

    /// Deletes an attachment.
    pub fn delete_attachment(&self, id: &AttachmentId) -> Result<()> {
        self.db.delete_attachment(&id.0).map_err(Into::into)
    }
}

impl From<DbThread> for Thread {
    fn from(thread: DbThread) -> Self {
        Self {
            id: thread.id,
            title: thread.title,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            archived: thread.archived,
            provider: thread.provider,
            model: thread.model,
        }
    }
}

impl From<DbMessage> for Message {
    fn from(msg: DbMessage) -> Self {
        Self {
            id: msg.id,
            thread_id: msg.thread_id,
            role: match msg.role.as_str() {
                "user" => MessageRole::User,
                "assistant" => MessageRole::Assistant,
                _ => MessageRole::System,
            },
            content: msg.content,
            created_at: msg.created_at,
            status: match msg.status.as_str() {
                "streaming" => MessageStatus::Streaming,
                "error" => MessageStatus::Error,
                "cancelled" => MessageStatus::Cancelled,
                "failed" => MessageStatus::Failed,
                _ => MessageStatus::Complete,
            },
            error_message: msg.error_message,
        }
    }
}

impl From<DbArtifact> for Artifact {
    fn from(artifact: DbArtifact) -> Self {
        Self {
            id: ArtifactId(artifact.id),
            thread_id: artifact.thread_id,
            message_id: artifact.message_id,
            title: artifact.title,
            content: artifact.content,
            created_at: artifact.created_at,
        }
    }
}

impl From<DbMemory> for Memory {
    fn from(memory: DbMemory) -> Self {
        Self {
            id: MemoryId(memory.id),
            title: memory.title,
            content: memory.content,
            created_at: memory.created_at,
            updated_at: memory.updated_at,
        }
    }
}

impl From<DbAttachment> for Attachment {
    fn from(attachment: DbAttachment) -> Self {
        Self {
            id: AttachmentId(attachment.id),
            message_id: attachment.message_id,
            kind: match attachment.kind.as_str() {
                "clipboard" => AttachmentKind::Clipboard,
                "memory" => AttachmentKind::Memory,
                _ => AttachmentKind::File,
            },
            name: attachment.name,
            mime_type: attachment.mime_type,
            content: attachment.content,
            path: attachment.path,
            created_at: attachment.created_at,
        }
    }
}
