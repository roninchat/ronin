#![deny(missing_docs)]

//! Public application/session boundary for Ronin.
//!
//! Modules follow the product boundaries in `docs/standards.md`:
//!
//! - [`domain`]: pure domain types (threads, messages, memories, artifacts).
//! - [`config`]: TOML preference/provider configuration types.
//! - [`context`]: explicit user context parsing and attachment drafts.
//! - [`providers`]: Ronin-owned provider traits and HTTP adapters.
//! - [`session`]: filesystem/database-backed application session.

pub mod config;
pub mod context;
pub mod domain;
pub mod error;
pub mod providers;
pub mod session;

pub use config::{GeneralConfig, OllamaConfig, OpenAiConfig, RoninConfig};
pub use context::{
    artifact_attachment, clipboard_attachment, memory_attachment, parse_context_tools,
    read_file_attachment, ContextAttachmentDraft, ContextToolError, ContextToolRef,
    ParsedContextTools, MAX_FILE_ATTACHMENT_BYTES,
};
pub use domain::{
    Artifact, ArtifactId, Attachment, AttachmentId, AttachmentKind, Memory, MemoryId, Message,
    MessageRole, MessageStatus, RoninPaths, Thread,
};
pub use error::{Result, RoninError};
pub use providers::{
    clear_model_cache, get_cached_models, get_model_cache, CachedModels, ChatMessage, ChatProvider,
    ChatRequest, ChatStreamEvent, HttpOllamaProvider, OllamaHealth, OllamaProvider,
    OpenAiCompatibleProvider, RONIN_SYSTEM_PROMPT,
};
pub use session::RoninSession;
