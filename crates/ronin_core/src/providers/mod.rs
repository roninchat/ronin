//! Ronin-owned provider traits and concrete provider adapters.

mod model_cache;
mod ollama;
mod openai;

pub use model_cache::{clear_model_cache, get_cached_models, get_model_cache, CachedModels};
pub use ollama::HttpOllamaProvider;
pub use openai::OpenAiCompatibleProvider;

use crate::error::Result;

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

/// Resolves the system prompt that will be sent to the model.
///
/// - [`PersonaMode::Append`]: built-in Ronin prompt, then optional custom text.
/// - [`PersonaMode::Replace`]: custom text only; empty/whitespace falls back to built-in.
pub fn effective_system_prompt(persona: &crate::config::PersonaConfig) -> String {
    let custom = persona.text.trim();
    match persona.mode {
        crate::config::PersonaMode::Append => {
            if custom.is_empty() {
                RONIN_SYSTEM_PROMPT.to_string()
            } else {
                format!("{RONIN_SYSTEM_PROMPT}\n\n{custom}")
            }
        }
        crate::config::PersonaMode::Replace => {
            if custom.is_empty() {
                RONIN_SYSTEM_PROMPT.to_string()
            } else {
                custom.to_string()
            }
        }
    }
}
