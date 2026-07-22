//! Explicit user context: `@file`, `@folder`, `@clipboard`, `@memory`, `@artifact`, `@screenshot`
//! parsing and context attachment drafts.

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

use ignore::gitignore::Gitignore;

use crate::domain::{Artifact, AttachmentKind, Memory};
use crate::folder_filter::{
    absolutize_path, folder_root_block_reason, load_gitignore_at, path_omitted_by_policy,
    FolderBlockReason, FolderListPolicy,
};

/// Default maximum number of files included in a folder listing (initial reveal).
///
/// Raised above the M2 shallow walk (200) so `@folder` reaches more of a project
/// before the user opts into progressive deepen or a lexical index.
pub const FOLDER_LIST_MAX_ENTRIES: usize = 500;

/// Default maximum directory nesting depth for folder listings (root = 0).
///
/// Raised above the M2 shallow walk (2) for deeper on-demand listing under caps.
pub const FOLDER_LIST_MAX_DEPTH: usize = 4;

/// Hard ceiling for progressive deepen of directory nesting (documented).
pub const FOLDER_LIST_DEPTH_CEILING: usize = 10;

/// Hard ceiling for progressive deepen of listing entry count (documented).
pub const FOLDER_LIST_ENTRIES_CEILING: usize = 2_000;

/// Step applied to [`FolderListOptions::max_depth`] on each progressive deepen.
pub const FOLDER_LIST_DEPTH_STEP: usize = 2;

/// Step applied to [`FolderListOptions::max_entries`] on each progressive deepen.
pub const FOLDER_LIST_ENTRIES_STEP: usize = 500;

/// Options for bounded / progressive folder listing walks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderListOptions {
    /// Maximum directory nesting depth (root = 0). Clamped to
    /// [`FOLDER_LIST_DEPTH_CEILING`].
    pub max_depth: usize,
    /// Maximum files collected. Clamped to [`FOLDER_LIST_ENTRIES_CEILING`].
    pub max_entries: usize,
    /// Optional case-insensitive substring filter on relative paths.
    ///
    /// When set, only matching files are collected (directories are still walked
    /// so deep matches remain reachable under caps).
    pub browse_filter: Option<String>,
}

impl Default for FolderListOptions {
    fn default() -> Self {
        Self {
            max_depth: FOLDER_LIST_MAX_DEPTH,
            max_entries: FOLDER_LIST_MAX_ENTRIES,
            browse_filter: None,
        }
    }
}

impl FolderListOptions {
    /// Clamps depth/entries to the documented progressive ceilings.
    #[must_use]
    pub fn clamp_to_ceilings(mut self) -> Self {
        self.max_depth = self.max_depth.min(FOLDER_LIST_DEPTH_CEILING);
        self.max_entries = self.max_entries.min(FOLDER_LIST_ENTRIES_CEILING);
        self
    }

    /// Whether another progressive deepen step can raise caps further.
    #[must_use]
    pub fn can_deepen(&self) -> bool {
        let depth = self.max_depth.min(FOLDER_LIST_DEPTH_CEILING);
        let entries = self.max_entries.min(FOLDER_LIST_ENTRIES_CEILING);
        depth < FOLDER_LIST_DEPTH_CEILING || entries < FOLDER_LIST_ENTRIES_CEILING
    }

    /// Next progressive reveal step toward the documented ceilings.
    #[must_use]
    pub fn deepen(&self) -> Self {
        Self {
            max_depth: self
                .max_depth
                .saturating_add(FOLDER_LIST_DEPTH_STEP)
                .min(FOLDER_LIST_DEPTH_CEILING),
            max_entries: self
                .max_entries
                .saturating_add(FOLDER_LIST_ENTRIES_STEP)
                .min(FOLDER_LIST_ENTRIES_CEILING),
            browse_filter: self.browse_filter.clone(),
        }
    }

    /// Sets a browse filter (empty / whitespace clears the filter).
    #[must_use]
    pub fn with_browse_filter(mut self, filter: impl Into<String>) -> Self {
        let filter = filter.into();
        let trimmed = filter.trim();
        self.browse_filter = if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        };
        self
    }
}

/// Returns whether `relative_path` matches an optional browse filter
/// (case-insensitive substring). Empty / whitespace filters match everything.
#[must_use]
pub fn folder_entry_matches_browse_filter(relative_path: &str, filter: Option<&str>) -> bool {
    let Some(filter) = filter.map(str::trim).filter(|f| !f.is_empty()) else {
        return true;
    };
    relative_path
        .to_ascii_lowercase()
        .contains(&filter.to_ascii_lowercase())
}

fn path_matches_browse_filter(relative_path: &str, filter: Option<&str>) -> bool {
    folder_entry_matches_browse_filter(relative_path, filter)
}

/// Default character threshold before attachment size warnings appear (~6k tokens).
pub const DEFAULT_ATTACHMENT_WARN_CHARS: usize = 24_000;

/// A parsed explicit context reference from the composer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextToolRef {
    /// User requested a file attachment by path.
    File(String),
    /// User requested a folder attachment by path (file selection follows).
    Folder(String),
    /// User requested a memory attachment by id.
    Memory(String),
    /// User requested an artifact attachment by id.
    Artifact(String),
    /// User requested current clipboard text.
    Clipboard,
    /// User requested a screenshot capture.
    Screenshot,
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

        if let Some(after_folder) = candidate.strip_prefix("@folder:") {
            let (path, consumed) = parse_file_ref(after_folder);
            if !path.is_empty() {
                refs.push(ContextToolRef::Folder(path));
                rest = &candidate["@folder:".len() + consumed..];
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

        if candidate.len() >= "@screenshot".len()
            && candidate[.."@screenshot".len()].eq_ignore_ascii_case("@screenshot")
            && is_ref_boundary(candidate["@screenshot".len()..].chars().next())
        {
            refs.push(ContextToolRef::Screenshot);
            rest = &candidate["@screenshot".len()..];
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
            || candidate.starts_with("@folder:")
            || candidate.starts_with("@memory:")
            || candidate.starts_with("@artifact:")
            || candidate
                .get(.."@clipboard".len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("@clipboard"))
            || candidate
                .get(.."@screenshot".len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("@screenshot"))
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

/// Base directory for relative `@file` / `@folder` resolution.
///
/// When `workspace_root` is set (explicit thread bind), it wins. Otherwise
/// `process_cwd` is used. Never auto-detects a git root or invents a workspace.
pub fn context_path_base<'a>(workspace_root: Option<&'a Path>, process_cwd: &'a Path) -> &'a Path {
    workspace_root.unwrap_or(process_cwd)
}

/// Resolves a `@file` / `@folder` path against an optional thread workspace root.
///
/// Absolute paths are returned unchanged. Relative paths use `workspace_root` when
/// set; otherwise they join against `process_cwd`. Binding a workspace never
/// attaches file contents by itself — callers still must read/list explicitly.
pub fn resolve_context_path(
    requested: impl AsRef<Path>,
    workspace_root: Option<&Path>,
    process_cwd: impl AsRef<Path>,
) -> PathBuf {
    let requested_path = requested.as_ref();
    if requested_path.is_absolute() {
        requested_path.to_path_buf()
    } else {
        context_path_base(workspace_root, process_cwd.as_ref()).join(requested_path)
    }
}

/// Maximum file attachment size in bytes.
pub const MAX_FILE_ATTACHMENT_BYTES: u64 = 1_048_576;

/// Maximum image / screenshot attachment size in bytes (10 MB).
pub const MAX_IMAGE_ATTACHMENT_BYTES: u64 = 10 * 1_048_576;

/// Returns the image MIME type for a supported image extension, if any.
pub fn image_mime_type(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

/// Whether `path` looks like a supported image attachment.
pub fn is_supported_image_path(path: &Path) -> bool {
    image_mime_type(path).is_some()
}

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
    /// User selected a file where a directory was required.
    #[error("path {path} is not a directory")]
    NotADirectory {
        /// User-visible path.
        path: PathBuf,
    },
    /// Folder attach confirmed with no files selected.
    #[error("no files selected from folder {path}")]
    EmptyFolderSelection {
        /// Folder path.
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
    /// Folder listing blocked by never-list or allowlist policy.
    #[error("folder {path} is blocked by privacy policy ({reason})")]
    FolderBlocked {
        /// User-visible folder path.
        path: PathBuf,
        /// Why listing was refused.
        reason: FolderBlockReason,
    },
}

/// Reads a file selected by explicit `@file` context (text or image).
///
/// Relative paths resolve against `workspace_root` when set, otherwise against
/// `process_cwd`. Absolute paths work with or without a workspace root.
pub fn read_file_attachment(
    path: impl AsRef<Path>,
    workspace_root: Option<&Path>,
    process_cwd: impl AsRef<Path>,
) -> std::result::Result<ContextAttachmentDraft, ContextToolError> {
    let requested_path = path.as_ref();
    let resolved_path = resolve_context_path(requested_path, workspace_root, process_cwd);

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

    let name = resolved_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attached file")
        .to_string();

    if let Some(mime_type) = image_mime_type(&resolved_path) {
        if metadata.len() > MAX_IMAGE_ATTACHMENT_BYTES {
            return Err(ContextToolError::FileTooLarge {
                path: requested_path.to_path_buf(),
            });
        }
        return Ok(ContextAttachmentDraft {
            kind: AttachmentKind::Image,
            name: name.clone(),
            mime_type: mime_type.to_string(),
            content: None,
            path: Some(resolved_path),
            context_block: format!("[Attached image: {name}]"),
            size_bytes: Some(metadata.len()),
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
    let mime_type = text_mime_type(&resolved_path);

    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::File,
        name: name.clone(),
        mime_type: mime_type.to_string(),
        content: None,
        path: Some(resolved_path),
        context_block: format!("[Attached file: {name}]\n{text}"),
        size_bytes: Some(metadata.len()),
    })
}

fn text_mime_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("rs") => "text/x-rust",
        Some("md") => "text/markdown",
        Some("json") => "application/json",
        Some("toml") => "application/toml",
        Some("html" | "htm") => "text/html",
        Some("css") => "text/css",
        Some("js" | "mjs") => "text/javascript",
        Some("ts") => "text/typescript",
        Some("py") => "text/x-python",
        Some("sh") => "text/x-shellscript",
        Some("csv") => "text/csv",
        Some("xml") => "application/xml",
        Some("yaml" | "yml") => "application/yaml",
        _ => "text/plain",
    }
}

/// Builds a screenshot attachment from a captured image path.
pub fn screenshot_attachment(
    path: impl AsRef<Path>,
) -> std::result::Result<ContextAttachmentDraft, ContextToolError> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path).map_err(|source| ContextToolError::FileMetadata {
        path: path.to_path_buf(),
        source,
    })?;
    if metadata.is_dir() {
        return Err(ContextToolError::IsDirectory {
            path: path.to_path_buf(),
        });
    }
    if metadata.len() > MAX_IMAGE_ATTACHMENT_BYTES {
        return Err(ContextToolError::FileTooLarge {
            path: path.to_path_buf(),
        });
    }
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("screenshot.png")
        .to_string();
    let mime_type = image_mime_type(path).unwrap_or("image/png").to_string();

    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::Screenshot,
        name: name.clone(),
        mime_type,
        content: None,
        path: Some(path.to_path_buf()),
        context_block: format!("[Screenshot: {name}]"),
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

/// One selectable file in a folder attachment listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderEntry {
    /// Path relative to the folder root (uses `/` separators).
    pub relative_path: String,
    /// File size in bytes.
    pub size_bytes: u64,
}

/// Bounded listing of files under a folder (depth + entry caps).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderListing {
    /// Absolute resolved folder path.
    pub root: PathBuf,
    /// Display name (folder basename).
    pub name: String,
    /// Listed files (not directories), sorted by relative path.
    pub entries: Vec<FolderEntry>,
    /// True when max depth or max entry count truncated the listing.
    pub truncated: bool,
    /// Options (clamped) that produced this listing — used for progressive deepen.
    pub list_options: FolderListOptions,
}

/// Lists files under `path` for folder attach (bounded depth + entry count).
///
/// Applies the default [`FolderListPolicy`] (gitignore + built-in deny).
/// Relative paths resolve against `workspace_root` when set, otherwise against
/// `process_cwd`. Absolute paths work with or without a workspace root.
pub fn list_folder_entries(
    path: impl AsRef<Path>,
    workspace_root: Option<&Path>,
    process_cwd: impl AsRef<Path>,
) -> std::result::Result<FolderListing, ContextToolError> {
    list_folder_entries_with_policy(
        path,
        workspace_root,
        process_cwd,
        &FolderListPolicy::default(),
    )
}

/// Lists files under `path` using an explicit [`FolderListPolicy`] and default
/// [`FolderListOptions`].
///
/// Listing never equals attaching — callers must still collect an explicit
/// selection before building a folder attachment draft.
pub fn list_folder_entries_with_policy(
    path: impl AsRef<Path>,
    workspace_root: Option<&Path>,
    process_cwd: impl AsRef<Path>,
    policy: &FolderListPolicy,
) -> std::result::Result<FolderListing, ContextToolError> {
    list_folder_entries_with_options(
        path,
        workspace_root,
        process_cwd,
        policy,
        &FolderListOptions::default(),
    )
}

/// Lists files under `path` with an explicit policy and listing options
/// (depth/entry caps + optional browse filter).
///
/// Listing never equals attaching — callers must still collect an explicit
/// selection before building a folder attachment draft. Ignore/deny/allow rules
/// from [`FolderListPolicy`] still apply.
pub fn list_folder_entries_with_options(
    path: impl AsRef<Path>,
    workspace_root: Option<&Path>,
    process_cwd: impl AsRef<Path>,
    policy: &FolderListPolicy,
    options: &FolderListOptions,
) -> std::result::Result<FolderListing, ContextToolError> {
    let requested_path = path.as_ref();
    let resolved = resolve_context_path(requested_path, workspace_root, process_cwd);
    let resolved = resolved
        .canonicalize()
        .unwrap_or_else(|_| absolutize_path(&resolved));

    let metadata =
        std::fs::metadata(&resolved).map_err(|source| ContextToolError::FileMetadata {
            path: requested_path.to_path_buf(),
            source,
        })?;
    if !metadata.is_dir() {
        return Err(ContextToolError::NotADirectory {
            path: requested_path.to_path_buf(),
        });
    }

    let never_list: Vec<PathBuf> = policy
        .never_list
        .iter()
        .map(|p| absolutize_path(p))
        .collect();
    let allowlist: Vec<PathBuf> = policy
        .allowlist
        .iter()
        .map(|p| absolutize_path(p))
        .collect();
    let policy = FolderListPolicy {
        honor_gitignore: policy.honor_gitignore,
        apply_built_in_deny: policy.apply_built_in_deny,
        never_list,
        allowlist_enabled: policy.allowlist_enabled,
        allowlist,
    };

    if let Some(reason) = folder_root_block_reason(&resolved, &policy) {
        return Err(ContextToolError::FolderBlocked {
            path: requested_path.to_path_buf(),
            reason,
        });
    }

    let name = resolved
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("folder")
        .to_string();

    let list_options = options.clone().clamp_to_ceilings();

    let mut gitignores = Vec::new();
    if policy.honor_gitignore {
        if let Some(gi) = load_gitignore_at(&resolved) {
            gitignores.push(gi);
        }
    }

    let mut entries = Vec::new();
    let mut truncated = false;
    {
        let mut walk = FolderWalkState {
            policy: &policy,
            options: &list_options,
            gitignores: &mut gitignores,
            entries: &mut entries,
            truncated: &mut truncated,
        };
        walk_folder(&resolved, &resolved, 0, &mut walk)?;
    }
    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(FolderListing {
        root: resolved,
        name,
        entries,
        truncated,
        list_options,
    })
}

struct FolderWalkState<'a> {
    policy: &'a FolderListPolicy,
    options: &'a FolderListOptions,
    gitignores: &'a mut Vec<Gitignore>,
    entries: &'a mut Vec<FolderEntry>,
    truncated: &'a mut bool,
}

fn walk_folder(
    root: &Path,
    dir: &Path,
    depth: usize,
    walk: &mut FolderWalkState<'_>,
) -> std::result::Result<(), ContextToolError> {
    let read = std::fs::read_dir(dir).map_err(|source| ContextToolError::ReadFile {
        path: dir.to_path_buf(),
        source,
    })?;

    let mut children: Vec<_> = read.filter_map(|e| e.ok()).collect();
    children.sort_by_key(|e| e.file_name());

    for child in children {
        if walk.entries.len() >= walk.options.max_entries {
            *walk.truncated = true;
            return Ok(());
        }
        let path = child.path();
        let meta = match child.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let is_dir = meta.is_dir();
        if path_omitted_by_policy(
            root,
            &path,
            is_dir,
            meta.len(),
            walk.policy,
            walk.gitignores,
        ) {
            continue;
        }
        if is_dir {
            if depth >= walk.options.max_depth {
                *walk.truncated = true;
                continue;
            }
            let mut pushed_gitignore = false;
            if walk.policy.honor_gitignore {
                if let Some(gi) = load_gitignore_at(&path) {
                    walk.gitignores.push(gi);
                    pushed_gitignore = true;
                }
            }
            walk_folder(root, &path, depth + 1, walk)?;
            if pushed_gitignore {
                walk.gitignores.pop();
            }
            if *walk.truncated && walk.entries.len() >= walk.options.max_entries {
                return Ok(());
            }
            continue;
        }
        if !meta.is_file() {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if !path_matches_browse_filter(&rel, walk.options.browse_filter.as_deref()) {
            continue;
        }
        walk.entries.push(FolderEntry {
            relative_path: rel,
            size_bytes: meta.len(),
        });
    }
    Ok(())
}

/// Builds a folder attachment draft from an explicit selection of relative paths.
pub fn folder_attachment_from_selection(
    listing: &FolderListing,
    selected: &[String],
) -> std::result::Result<ContextAttachmentDraft, ContextToolError> {
    let selected: BTreeSet<&str> = selected.iter().map(String::as_str).collect();
    if selected.is_empty() {
        return Err(ContextToolError::EmptyFolderSelection {
            path: listing.root.clone(),
        });
    }

    let mut blocks = Vec::new();
    let mut total_bytes = 0u64;
    for entry in &listing.entries {
        if !selected.contains(entry.relative_path.as_str()) {
            continue;
        }
        let full = listing.root.join(&entry.relative_path);
        match read_file_attachment(&full, None, &listing.root) {
            Ok(file_draft) => {
                total_bytes = total_bytes.saturating_add(entry.size_bytes);
                blocks.push(format!(
                    "[Folder file: {}/{}]\n{}",
                    listing.name,
                    entry.relative_path,
                    file_draft
                        .context_block
                        .lines()
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
                // For images, keep the short marker only.
                if matches!(file_draft.kind, AttachmentKind::Image) {
                    blocks.pop();
                    blocks.push(format!(
                        "[Folder file: {}/{} — image {}]",
                        listing.name, entry.relative_path, file_draft.name
                    ));
                }
            }
            Err(ContextToolError::BinaryFile { .. }) => {
                // Skip binaries silently in folder selection.
                continue;
            }
            Err(ContextToolError::FileTooLarge { .. }) => continue,
            Err(e) => return Err(e),
        }
    }

    if blocks.is_empty() {
        return Err(ContextToolError::EmptyFolderSelection {
            path: listing.root.clone(),
        });
    }

    let context_block = format!(
        "[Attached folder: {} — {} file(s)]\n{}",
        listing.name,
        blocks.len(),
        blocks.join("\n\n")
    );

    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::Folder,
        name: listing.name.clone(),
        mime_type: "text/plain".to_string(),
        content: None,
        path: Some(listing.root.clone()),
        context_block,
        size_bytes: Some(total_bytes),
    })
}

/// Character count contributed by an attachment draft toward context size.
pub fn attachment_content_chars(draft: &ContextAttachmentDraft) -> usize {
    draft.context_block.chars().count()
}

/// Sum of attachment context characters across drafts.
pub fn total_attachment_chars(drafts: &[ContextAttachmentDraft]) -> usize {
    drafts.iter().map(attachment_content_chars).sum()
}
