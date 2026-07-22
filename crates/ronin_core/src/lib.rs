#![deny(missing_docs)]

//! Public application/session boundary for Ronin.
//!
//! Modules follow the product boundaries in `docs/standards.md`:
//!
//! - [`domain`]: pure domain types (threads, messages, memories, artifacts).
//! - [`config`]: TOML preference/provider configuration types.
//! - [`context`]: explicit user context parsing and attachment drafts.
//! - [`screenshot`]: screenshot capture abstraction (portal / fallback).
//! - [`notification`]: desktop notification request shaping (host port; no D-Bus).
//! - [`providers`]: Ronin-owned provider traits and HTTP adapters.
//! - [`session`]: filesystem/database-backed application session.
//! - [`trust`]: host-enforced capability boundary and silent-context gates.

pub mod config;
pub mod context;
pub mod domain;
pub mod error;
pub mod notification;
pub mod providers;
pub mod screenshot;
pub mod session;
pub mod trust;

pub use config::{
    clamp_sidebar_width, effective_sidebar_width, export_provider_config_toml,
    import_provider_config_toml, resolve_color_scheme, validate_provider_config_export,
    ColorScheme, GeneralConfig, LoggingConfig, NotificationsConfig, OllamaConfig, OpenAiConfig,
    PersonaConfig, PersonaMode, ProviderConfigExport, RoninConfig, ThemePreference, UiConfig,
    SIDEBAR_WIDTH_DEFAULT, SIDEBAR_WIDTH_MAX, SIDEBAR_WIDTH_MIN,
};
pub use context::{
    artifact_attachment, attachment_content_chars, clipboard_attachment,
    folder_attachment_from_selection, image_mime_type, is_supported_image_path,
    list_folder_entries, memory_attachment, parse_context_tools, read_file_attachment,
    screenshot_attachment, total_attachment_chars, ContextAttachmentDraft, ContextToolError,
    ContextToolRef, FolderEntry, FolderListing, ParsedContextTools, DEFAULT_ATTACHMENT_WARN_CHARS,
    FOLDER_LIST_MAX_DEPTH, FOLDER_LIST_MAX_ENTRIES, MAX_FILE_ATTACHMENT_BYTES,
    MAX_IMAGE_ATTACHMENT_BYTES,
};
pub use domain::{
    Artifact, ArtifactId, Attachment, AttachmentId, AttachmentKind, Memory, MemoryId, Message,
    MessageRole, MessageStatus, RoninPaths, Thread,
};
pub use error::{Result, RoninError};
pub use notification::{
    interpret_notification_action, notification_may_inject_into_chat_request,
    notification_payload_origin, shape_generation_notification, DesktopNotificationRequest,
    DesktopNotifier, GenerationNotifyInput, GenerationNotifyKind, NotificationButton,
    NotificationError, NotificationFocusAction, NotificationPrefs, NullDesktopNotifier,
    RecordingDesktopNotifier, FOCUS_THREAD_ACTION, GENERATION_NOTIFICATION_ID_PREFIX,
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
