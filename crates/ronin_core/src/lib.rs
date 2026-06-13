#![deny(missing_docs)]

//! Public application/session boundary for Ronin.

use std::fs;
use std::io;
use std::path::PathBuf;
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
pub const RONIN_SYSTEM_PROMPT: &str = "\
You are Ronin, a local AI assistant running on Linux. \
You run offline via Ollama and have no internet access. \
You cannot browse the web, fetch URLs, run shell commands, \
or access files on the user's system unless they paste content \
into the chat. Answer concisely and truthfully. \
When you don't know something, say so.";

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
        self.db
            .create_thread()
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

    /// Fails a message with an error.
    pub fn fail_message(&self, message_id: &str, content: &str, error_message: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "failed", Some(error_message))
            .map_err(Into::into)
    }

    /// Loads the previously selected Ollama model from config, if any.
    pub fn load_selected_model(&self) -> Result<Option<String>> {
        let config_path = self.paths.config_dir.join("ronin_config.json");
        if !config_path.is_file() {
            return Ok(None);
        }
        let data = fs::read_to_string(&config_path)
            .map_err(|e| RoninError::Config(format!("read config: {e}")))?;
        #[derive(serde::Deserialize)]
        struct Config {
            selected_model: Option<String>,
        }
        let config: Config = serde_json::from_str(&data)
            .map_err(|e| RoninError::Config(format!("parse config: {e}")))?;
        Ok(config.selected_model)
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
        let config_path = self.paths.config_dir.join("ronin_config.json");
        #[derive(serde::Serialize)]
        struct Config {
            selected_model: String,
        }
        let config = Config {
            selected_model: model.to_string(),
        };
        let data = serde_json::to_string_pretty(&config)
            .map_err(|e| RoninError::Config(format!("serialize config: {e}")))?;
        fs::write(&config_path, data)
            .map_err(|e| RoninError::Config(format!("write config: {e}")))?;
        tracing::info!(model = %model, "saved selected model to config");
        Ok(())
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
            .map(|artifacts| artifacts.into_iter().map(|a| Artifact::from(a)).collect())
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
            .map(|memories| memories.into_iter().map(|m| Memory::from(m)).collect())
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
            .map(|attachments| attachments.into_iter().map(|a| Attachment::from(a)).collect())
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
