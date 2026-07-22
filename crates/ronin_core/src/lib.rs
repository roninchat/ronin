#![deny(missing_docs)]

//! Public application/session boundary for Ronin.
//!
//! Modules follow the product boundaries in `docs/standards.md`:
//!
//! - [`domain`]: pure domain types (threads, messages, memories, artifacts).
//! - [`config`]: TOML preference/provider configuration types.
//! - [`context`]: explicit user context parsing and attachment drafts.
//! - [`folder_filter`]: ignore/deny/allow policy for folder listing walks.
//! - [`workspace_index`]: user-triggered one-shot lexical workspace index + search/attach gate.
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
pub mod workspace_index;

pub use config::{
    clamp_sidebar_width, effective_sidebar_width, export_provider_config_toml,
    import_provider_config_toml, resolve_color_scheme, validate_provider_config_export,
    ColorScheme, GeneralConfig, LocalKnowledgeConfig, LoggingConfig, OllamaConfig, OpenAiConfig,
    PersonaConfig, PersonaMode, ProviderConfigExport, RoninConfig, ThemePreference, UiConfig,
    SIDEBAR_WIDTH_DEFAULT, SIDEBAR_WIDTH_MAX, SIDEBAR_WIDTH_MIN,
};
pub use context::{
    artifact_attachment, attachment_content_chars, clipboard_attachment, context_path_base,
    folder_attachment_from_selection, folder_entry_matches_browse_filter, image_mime_type,
    is_supported_image_path, list_folder_entries, list_folder_entries_with_options,
    list_folder_entries_with_policy, memory_attachment, parse_context_tools, read_file_attachment,
    resolve_context_path, screenshot_attachment, total_attachment_chars, ContextAttachmentDraft,
    ContextToolError, ContextToolRef, FolderEntry, FolderListOptions, FolderListing,
    ParsedContextTools, DEFAULT_ATTACHMENT_WARN_CHARS, FOLDER_LIST_DEPTH_CEILING,
    FOLDER_LIST_DEPTH_STEP, FOLDER_LIST_ENTRIES_CEILING, FOLDER_LIST_ENTRIES_STEP,
    FOLDER_LIST_MAX_DEPTH, FOLDER_LIST_MAX_ENTRIES, MAX_FILE_ATTACHMENT_BYTES,
    MAX_IMAGE_ATTACHMENT_BYTES,
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
pub use workspace_index::{
    clamp_workspace_index_search_limit, collect_workspace_index_documents,
    drafts_for_workspace_index_include, workspace_index_hit_attachment,
    workspace_index_hit_attachment_origin, workspace_index_origin_may_inject,
    workspace_index_root_block, workspace_index_storage_path, WorkspaceIndexBlock,
    WorkspaceIndexCaps, WorkspaceIndexCollectResult, WorkspaceIndexDocument, WorkspaceIndexHit,
    WorkspaceIndexHitSelection, WorkspaceIndexIncludeGate, WorkspaceIndexInfo, WorkspaceIndexPhase,
    WORKSPACE_INDEX_INCLUDE_GATE_LABEL, WORKSPACE_INDEX_MAX_BYTES, WORKSPACE_INDEX_MAX_DEPTH,
    WORKSPACE_INDEX_MAX_DURATION, WORKSPACE_INDEX_MAX_ENTRIES, WORKSPACE_INDEX_MAX_FILE_BYTES,
    WORKSPACE_INDEX_SEARCH_DEFAULT_LIMIT, WORKSPACE_INDEX_SEARCH_MAX_LIMIT,
    WORKSPACE_INDEX_STORAGE_DIR,
};
