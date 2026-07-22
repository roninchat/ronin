//! User-triggered one-shot lexical workspace index (M3.0 #73 / #74).
//!
//! Build / rebuild / cancel / delete are explicit session actions. Collection
//! reuses [`FolderListPolicy`] ignore/deny/allow rules. Search returns
//! candidates only; attaching selected hits (or a visible per-send include)
//! is required before any hit may enter provider chat assembly.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ignore::gitignore::Gitignore;

use crate::context::ContextAttachmentDraft;
use crate::domain::AttachmentKind;
use crate::folder_filter::{
    absolutize_path, folder_root_block_reason, load_gitignore_at, path_omitted_by_policy,
    FolderBlockReason, FolderListPolicy,
};
use crate::trust::{may_inject_into_chat_request, scrub_ambient_payload, ContextOrigin};

/// Default maximum files admitted into one index build.
pub const WORKSPACE_INDEX_MAX_ENTRIES: usize = 5_000;

/// Default maximum total UTF-8 body bytes admitted into one index build.
pub const WORKSPACE_INDEX_MAX_BYTES: u64 = 32 * 1024 * 1024;

/// Default maximum directory nesting depth for index walks (root = 0).
pub const WORKSPACE_INDEX_MAX_DEPTH: usize = 20;

/// Default per-file read cap (aligned with attach size hygiene).
pub const WORKSPACE_INDEX_MAX_FILE_BYTES: u64 = crate::context::MAX_FILE_ATTACHMENT_BYTES;

/// Default wall-clock budget for one index build.
pub const WORKSPACE_INDEX_MAX_DURATION: Duration = Duration::from_secs(60);

/// Lifecycle phase of a thread's lexical workspace index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceIndexPhase {
    /// No index metadata / corpus for this thread.
    Absent,
    /// A user-triggered build is in progress.
    Running,
    /// Last build finished successfully (may still be truncated by caps).
    Done,
    /// Last build failed.
    Failed,
    /// Last build was cancelled by the user.
    Cancelled,
}

impl WorkspaceIndexPhase {
    /// Stable wire / DB label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Running => "running",
            Self::Done => "done",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Parse a persisted phase label.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "absent" => Some(Self::Absent),
            "running" => Some(Self::Running),
            "done" => Some(Self::Done),
            "failed" => Some(Self::Failed),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

/// Caps for a one-shot lexical index walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexCaps {
    /// Maximum file entries to index.
    pub max_entries: usize,
    /// Maximum total body bytes to index.
    pub max_bytes: u64,
    /// Maximum directory nesting depth (root = 0).
    pub max_depth: usize,
    /// Skip individual files larger than this.
    pub max_file_bytes: u64,
    /// Wall-clock budget; exceeding marks the build truncated/cancelled by time.
    pub max_duration: Duration,
}

impl Default for WorkspaceIndexCaps {
    fn default() -> Self {
        Self {
            max_entries: WORKSPACE_INDEX_MAX_ENTRIES,
            max_bytes: WORKSPACE_INDEX_MAX_BYTES,
            max_depth: WORKSPACE_INDEX_MAX_DEPTH,
            max_file_bytes: WORKSPACE_INDEX_MAX_FILE_BYTES,
            max_duration: WORKSPACE_INDEX_MAX_DURATION,
        }
    }
}

/// Status snapshot for a thread's lexical workspace index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexInfo {
    /// Current lifecycle phase.
    pub phase: WorkspaceIndexPhase,
    /// Workspace root the index was built against, if any.
    pub workspace_root: Option<PathBuf>,
    /// Number of file documents stored in the corpus.
    pub entry_count: u64,
    /// Total body bytes stored in the corpus.
    pub byte_count: u64,
    /// Whether the build stopped early due to caps or cancel.
    pub truncated: bool,
    /// Failure message when [`WorkspaceIndexPhase::Failed`].
    pub error_message: Option<String>,
    /// Absolute path to the on-disk lexical store, when present.
    pub storage_path: Option<PathBuf>,
    /// Build completion time as UTC Unix milliseconds, when known.
    pub built_at_ms: Option<i64>,
}

impl WorkspaceIndexInfo {
    /// Empty / absent index info.
    pub fn absent() -> Self {
        Self {
            phase: WorkspaceIndexPhase::Absent,
            workspace_root: None,
            entry_count: 0,
            byte_count: 0,
            truncated: false,
            error_message: None,
            storage_path: None,
            built_at_ms: None,
        }
    }
}

/// One text document collected for the lexical corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexDocument {
    /// Path relative to the workspace root (forward slashes).
    pub relative_path: String,
    /// Scrubbed UTF-8 body text.
    pub body: String,
    /// Byte length of the stored body.
    pub byte_len: u64,
}

/// Result of a cancellable index document walk (no persistence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexCollectResult {
    /// Documents collected before stop.
    pub documents: Vec<WorkspaceIndexDocument>,
    /// Total body bytes across [`Self::documents`].
    pub byte_count: u64,
    /// Whether caps or cancel stopped the walk early.
    pub truncated: bool,
    /// Whether the caller cancel flag aborted the walk.
    pub cancelled: bool,
    /// Optional error message when the walk failed hard.
    pub error_message: Option<String>,
}

/// Why an index build could not start against a root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceIndexBlock {
    /// Folder privacy policy blocked the root.
    Folder(FolderBlockReason),
    /// Root path is missing or not a directory.
    InvalidRoot,
}

/// Relative directory under the session data dir for per-thread index DBs.
pub const WORKSPACE_INDEX_STORAGE_DIR: &str = "workspace_indexes";

/// Absolute path for a thread's lexical index SQLite file under `data_dir`.
pub fn workspace_index_storage_path(data_dir: &Path, thread_id: &str) -> PathBuf {
    data_dir
        .join(WORKSPACE_INDEX_STORAGE_DIR)
        .join(format!("{thread_id}.db"))
}

/// Walk `root` and collect lexical documents under `policy` + `caps`.
///
/// Honors ignore/deny/allow. Checks `cancel` between files. Does not write disk
/// and does not attach anything to chat.
pub fn collect_workspace_index_documents(
    root: &Path,
    policy: &FolderListPolicy,
    caps: &WorkspaceIndexCaps,
    cancel: &AtomicBool,
) -> WorkspaceIndexCollectResult {
    let resolved = match root.canonicalize() {
        Ok(p) => p,
        Err(_) => absolutize_path(root),
    };

    if !resolved.is_dir() {
        return WorkspaceIndexCollectResult {
            documents: Vec::new(),
            byte_count: 0,
            truncated: false,
            cancelled: false,
            error_message: Some("workspace root must be an existing directory".into()),
        };
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
        return WorkspaceIndexCollectResult {
            documents: Vec::new(),
            byte_count: 0,
            truncated: false,
            cancelled: false,
            error_message: Some(format!("workspace root blocked: {reason}")),
        };
    }

    let mut gitignores = Vec::new();
    if policy.honor_gitignore {
        if let Some(gi) = load_gitignore_at(&resolved) {
            gitignores.push(gi);
        }
    }

    let started = Instant::now();
    let mut documents = Vec::new();
    let mut byte_count = 0_u64;
    let mut truncated = false;
    let mut cancelled = false;

    let mut walk = IndexWalkState {
        policy: &policy,
        caps,
        cancel,
        gitignores: &mut gitignores,
        documents: &mut documents,
        byte_count: &mut byte_count,
        truncated: &mut truncated,
        cancelled: &mut cancelled,
        started,
    };

    if let Err(msg) = walk_index_folder(&resolved, &resolved, 0, &mut walk) {
        return WorkspaceIndexCollectResult {
            documents,
            byte_count,
            truncated,
            cancelled,
            error_message: Some(msg),
        };
    }

    documents.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    WorkspaceIndexCollectResult {
        documents,
        byte_count,
        truncated,
        cancelled,
        error_message: None,
    }
}

struct IndexWalkState<'a> {
    policy: &'a FolderListPolicy,
    caps: &'a WorkspaceIndexCaps,
    cancel: &'a AtomicBool,
    gitignores: &'a mut Vec<Gitignore>,
    documents: &'a mut Vec<WorkspaceIndexDocument>,
    byte_count: &'a mut u64,
    truncated: &'a mut bool,
    cancelled: &'a mut bool,
    started: Instant,
}

fn walk_index_folder(
    root: &Path,
    dir: &Path,
    depth: usize,
    walk: &mut IndexWalkState<'_>,
) -> Result<(), String> {
    if walk.cancel.load(Ordering::SeqCst) {
        *walk.cancelled = true;
        *walk.truncated = true;
        return Ok(());
    }
    if walk.started.elapsed() > walk.caps.max_duration {
        *walk.truncated = true;
        return Ok(());
    }

    let read = std::fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    let mut children: Vec<_> = read.filter_map(|e| e.ok()).collect();
    children.sort_by_key(|e| e.file_name());

    for child in children {
        if walk.cancel.load(Ordering::SeqCst) {
            *walk.cancelled = true;
            *walk.truncated = true;
            return Ok(());
        }
        if walk.started.elapsed() > walk.caps.max_duration {
            *walk.truncated = true;
            return Ok(());
        }
        if walk.documents.len() >= walk.caps.max_entries {
            *walk.truncated = true;
            return Ok(());
        }
        if *walk.byte_count >= walk.caps.max_bytes {
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
            if depth >= walk.caps.max_depth {
                *walk.truncated = true;
                continue;
            }
            let mut pushed = false;
            if walk.policy.honor_gitignore {
                if let Some(gi) = load_gitignore_at(&path) {
                    walk.gitignores.push(gi);
                    pushed = true;
                }
            }
            walk_index_folder(root, &path, depth + 1, walk)?;
            if pushed {
                walk.gitignores.pop();
            }
            if *walk.cancelled
                || *walk.truncated && walk.documents.len() >= walk.caps.max_entries
                || *walk.byte_count >= walk.caps.max_bytes
            {
                return Ok(());
            }
            continue;
        }

        if !meta.is_file() {
            continue;
        }
        if meta.len() > walk.caps.max_file_bytes {
            continue;
        }

        let rel = match path.strip_prefix(root) {
            Ok(r) => r.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };

        let raw = match std::fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        if raw.contains(&0) {
            continue;
        }
        let Ok(text) = String::from_utf8(raw) else {
            continue;
        };
        let body = scrub_ambient_payload(&text);
        let byte_len = body.len() as u64;
        if *walk.byte_count + byte_len > walk.caps.max_bytes {
            *walk.truncated = true;
            return Ok(());
        }

        *walk.byte_count += byte_len;
        walk.documents.push(WorkspaceIndexDocument {
            relative_path: rel,
            body,
            byte_len,
        });
    }

    Ok(())
}

/// Classify whether `root` may be indexed under `policy`.
pub fn workspace_index_root_block(
    root: &Path,
    policy: &FolderListPolicy,
) -> Option<WorkspaceIndexBlock> {
    let resolved = root
        .canonicalize()
        .unwrap_or_else(|_| absolutize_path(root));
    if !resolved.is_dir() {
        return Some(WorkspaceIndexBlock::InvalidRoot);
    }
    folder_root_block_reason(&resolved, policy).map(WorkspaceIndexBlock::Folder)
}

/// Default cap on lexical search hits returned to the UI.
pub const WORKSPACE_INDEX_SEARCH_DEFAULT_LIMIT: usize = 50;

/// Hard ceiling on lexical search hits in one query.
pub const WORKSPACE_INDEX_SEARCH_MAX_LIMIT: usize = 200;

/// Visible label for the optional per-send include control.
pub const WORKSPACE_INDEX_INCLUDE_GATE_LABEL: &str = "Include selected search hits in this send";

/// One lexical search candidate (path + snippet). Never auto-merged into chat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexHit {
    /// Path relative to the workspace root.
    pub relative_path: String,
    /// Short excerpt around the match.
    pub snippet: String,
}

impl WorkspaceIndexHit {
    /// Trust origin for a bare search candidate (must not inject).
    pub fn context_origin(&self) -> ContextOrigin {
        ContextOrigin::IndexSearchHit
    }
}

/// Explicit user selection of search hits awaiting attach / include.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspaceIndexHitSelection {
    paths: Vec<String>,
}

impl WorkspaceIndexHitSelection {
    /// Empty selection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace selection with `paths` (deduped, order-preserving).
    pub fn set_paths(&mut self, paths: impl IntoIterator<Item = impl Into<String>>) {
        let mut seen = std::collections::HashSet::new();
        self.paths = paths
            .into_iter()
            .map(Into::into)
            .filter(|p| seen.insert(p.clone()))
            .collect();
    }

    /// Add one relative path if not already selected.
    pub fn select(&mut self, relative_path: impl Into<String>) {
        let path = relative_path.into();
        if !self.paths.iter().any(|p| p == &path) {
            self.paths.push(path);
        }
    }

    /// Remove one relative path from the selection.
    pub fn deselect(&mut self, relative_path: &str) {
        self.paths.retain(|p| p != relative_path);
    }

    /// Clear all selected paths.
    pub fn clear(&mut self) {
        self.paths.clear();
    }

    /// Currently selected relative paths.
    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    /// Whether any path is selected.
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

/// Optional per-send include control for selected search hits.
///
/// Labeled, visible, and **off by default**. Enabling it is an explicit user
/// action equivalent to confirming include for the current send only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceIndexIncludeGate {
    enabled: bool,
}

impl Default for WorkspaceIndexIncludeGate {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceIndexIncludeGate {
    /// New gate: disabled (safe default).
    pub fn new() -> Self {
        Self { enabled: false }
    }

    /// User-visible control label.
    pub fn label(&self) -> &'static str {
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL
    }

    /// Whether include is currently on for this send.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Explicitly enable or disable the gate.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Trust origin when this gate is considered for injection.
    pub fn context_origin(&self) -> ContextOrigin {
        if self.enabled {
            ContextOrigin::VisiblePerSendInclude
        } else {
            ContextOrigin::IndexSearchHit
        }
    }
}

/// Clamp a requested search limit into the documented range.
pub fn clamp_workspace_index_search_limit(limit: usize) -> usize {
    limit.clamp(1, WORKSPACE_INDEX_SEARCH_MAX_LIMIT)
}

/// Build a context attachment draft from an explicitly chosen index hit body.
///
/// Callers must only invoke this after an explicit attach action. The draft is
/// tagged as [`ContextOrigin::ExplicitAttachment`] for injection checks.
pub fn workspace_index_hit_attachment(relative_path: &str, body: &str) -> ContextAttachmentDraft {
    let scrubbed = scrub_ambient_payload(body);
    let name = relative_path
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(relative_path)
        .to_string();
    let size_bytes = Some(scrubbed.len() as u64);
    ContextAttachmentDraft {
        kind: AttachmentKind::File,
        name,
        mime_type: "text/plain".to_string(),
        content: Some(scrubbed.clone()),
        path: None,
        context_block: format!("[Attached workspace file: {relative_path}]\n{scrubbed}"),
        size_bytes,
    }
}

/// Trust origin for drafts produced by [`workspace_index_hit_attachment`].
pub fn workspace_index_hit_attachment_origin() -> ContextOrigin {
    ContextOrigin::ExplicitAttachment
}

/// Drafts that may enter chat when the include gate is on and selection is non-empty.
///
/// Returns an empty list when the gate is off or nothing is selected — search
/// hits never silent-merge.
pub fn drafts_for_workspace_index_include(
    gate: &WorkspaceIndexIncludeGate,
    selected_drafts: &[ContextAttachmentDraft],
) -> Vec<ContextAttachmentDraft> {
    if !gate.is_enabled() || selected_drafts.is_empty() {
        return Vec::new();
    }
    selected_drafts.to_vec()
}

/// Whether a workspace-index-related origin may enter provider chat assembly.
pub fn workspace_index_origin_may_inject(origin: ContextOrigin) -> bool {
    may_inject_into_chat_request(origin)
}
