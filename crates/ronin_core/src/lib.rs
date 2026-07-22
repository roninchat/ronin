#![deny(missing_docs)]

//! Public application/session boundary for Ronin.
//!
//! Modules follow the product boundaries in `docs/standards.md`:
//!
//! - [`domain`]: pure domain types (threads, messages, memories, artifacts).
//! - [`config`]: TOML preference/provider configuration types.
//! - [`context`]: explicit user context parsing and attachment drafts.
//! - [`folder_filter`]: ignore/deny/allow policy for folder listing walks.
//! - [`screenshot`]: screenshot capture abstraction (portal / fallback).
//! - [`providers`]: Ronin-owned provider traits and HTTP adapters.
//! - [`session`]: filesystem/database-backed application session.
//! - [`trust`]: host-enforced capability boundary and silent-context gates.

pub mod config;
pub mod context;
pub mod domain;
pub mod error;
pub mod folder_filter;
pub mod providers;
pub mod screenshot;
pub mod session;
pub mod trust;

pub use config::{
    clamp_sidebar_width, effective_sidebar_width, export_provider_config_toml,
    import_provider_config_toml, resolve_color_scheme, validate_provider_config_export,
    ColorScheme, GeneralConfig, LocalKnowledgeConfig, LoggingConfig, OllamaConfig, OpenAiConfig,
    PersonaConfig, PersonaMode, ProviderConfigExport, RoninConfig, ThemePreference, UiConfig,
    SIDEBAR_WIDTH_DEFAULT, SIDEBAR_WIDTH_MAX, SIDEBAR_WIDTH_MIN,
};
pub use context::{
    artifact_attachment, attachment_content_chars, clipboard_attachment, context_path_base,
    folder_attachment_from_selection, image_mime_type, is_supported_image_path,
    list_folder_entries, list_folder_entries_with_policy, memory_attachment, parse_context_tools,
    read_file_attachment, resolve_context_path, screenshot_attachment, total_attachment_chars,
    ContextAttachmentDraft, ContextToolError, ContextToolRef, FolderEntry, FolderListing,
    ParsedContextTools, DEFAULT_ATTACHMENT_WARN_CHARS, FOLDER_LIST_MAX_DEPTH,
    FOLDER_LIST_MAX_ENTRIES, MAX_FILE_ATTACHMENT_BYTES, MAX_IMAGE_ATTACHMENT_BYTES,
};
pub use domain::{
    Artifact, ArtifactId, Attachment, AttachmentId, AttachmentKind, Memory, MemoryId, Message,
    MessageRole, MessageStatus, RoninPaths, Thread,
};
pub use error::{Result, RoninError};
pub use folder_filter::{
    absolutize_path, folder_root_block_reason, path_is_under, FolderBlockReason, FolderListPolicy,
    BUILT_IN_DENY_DIR_NAMES, BUILT_IN_DENY_EXTENSIONS, BUILT_IN_DENY_FILE_NAMES,
};
pub use providers::{
    clear_model_cache, effective_system_prompt, get_cached_models, get_model_cache, CachedModels,
    ChatMessage, ChatProvider, ChatRequest, ChatStreamEvent, HttpOllamaProvider, OllamaHealth,
    OllamaProvider, OpenAiCompatibleProvider, RONIN_SYSTEM_PROMPT,
};
pub use screenshot::{FakeScreenshotCapturer, ScreenshotCapturer, ScreenshotError};
pub use session::RoninSession;
pub use trust::{
    may_auto_execute, may_inject_into_chat_request, resolve_marker_tool, scrub_ambient_payload,
    AllowedTool, ContextOrigin, ToolDisposition, AMBIENT_REDACTED, FORBIDDEN_AGENCY_TOOL_NAMES,
};
