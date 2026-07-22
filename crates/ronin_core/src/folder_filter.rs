//! Folder listing privacy: gitignore-style ignores, built-in deny hygiene,
//! never-list paths, and optional allowlist roots.

use std::path::{Path, PathBuf};

use ignore::gitignore::{Gitignore, GitignoreBuilder};

/// Why a folder root was refused for listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderBlockReason {
    /// Path is marked never-list / never-index (or under such a path).
    NeverList,
    /// Allowlist mode is on and the root is outside approved trees.
    NotAllowlisted,
}

impl FolderBlockReason {
    /// Stable reason label for errors and logs.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NeverList => "never-list",
            Self::NotAllowlisted => "not-allowlisted",
        }
    }
}

impl std::fmt::Display for FolderBlockReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Policy applied when walking a folder for attach listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderListPolicy {
    /// When true, honor `.gitignore` files under the walked root.
    pub honor_gitignore: bool,
    /// When true, apply built-in VCS / binary / oversized hygiene.
    pub apply_built_in_deny: bool,
    /// Absolute paths that must never be listed (roots or nested dirs).
    pub never_list: Vec<PathBuf>,
    /// When true, only roots under [`Self::allowlist`] may be listed.
    pub allowlist_enabled: bool,
    /// Approved roots when allowlist mode is enabled.
    pub allowlist: Vec<PathBuf>,
}

impl Default for FolderListPolicy {
    fn default() -> Self {
        Self {
            honor_gitignore: true,
            apply_built_in_deny: true,
            never_list: Vec::new(),
            allowlist_enabled: false,
            allowlist: Vec::new(),
        }
    }
}

/// Directory basenames always omitted by built-in deny hygiene.
pub const BUILT_IN_DENY_DIR_NAMES: &[&str] = &[".git", ".hg", ".svn"];

/// File basenames always omitted by built-in deny hygiene.
pub const BUILT_IN_DENY_FILE_NAMES: &[&str] = &[
    ".gitignore",
    ".ignore",
    ".rgignore",
    ".fdignore",
    ".gitattributes",
];

/// File extensions always omitted by built-in deny hygiene (lowercase, no dot).
pub const BUILT_IN_DENY_EXTENSIONS: &[&str] = &[
    "a", "o", "obj", "so", "dylib", "dll", "exe", "bin", "dat", "iso", "dmg", "wasm", "class",
    "pyc", "pyo", "png", "jpg", "jpeg", "gif", "webp", "ico", "bmp", "pdf", "zip", "tar", "gz",
    "tgz", "bz2", "xz", "7z", "rar", "woff", "woff2", "ttf", "otf", "eot", "mp3", "mp4", "avi",
    "mkv", "mov", "wav", "flac",
];

/// Returns whether `path` is equal to or nested under `ancestor`.
pub fn path_is_under(path: &Path, ancestor: &Path) -> bool {
    if path == ancestor {
        return true;
    }
    path.strip_prefix(ancestor).is_ok()
}

/// Whether a resolved folder root may be listed under `policy`.
pub fn folder_root_block_reason(
    root: &Path,
    policy: &FolderListPolicy,
) -> Option<FolderBlockReason> {
    for never in &policy.never_list {
        if path_is_under(root, never) {
            return Some(FolderBlockReason::NeverList);
        }
    }
    if policy.allowlist_enabled {
        let allowed = policy
            .allowlist
            .iter()
            .any(|allowed| path_is_under(root, allowed));
        if !allowed {
            return Some(FolderBlockReason::NotAllowlisted);
        }
    }
    None
}

/// Whether a path (file or directory) under a listing root should be omitted.
pub fn path_omitted_by_policy(
    root: &Path,
    path: &Path,
    is_dir: bool,
    size_bytes: u64,
    policy: &FolderListPolicy,
    gitignores: &[Gitignore],
) -> bool {
    for never in &policy.never_list {
        if path_is_under(path, never) {
            return true;
        }
    }

    if policy.apply_built_in_deny && built_in_deny_omits(path, is_dir, size_bytes) {
        return true;
    }

    if policy.honor_gitignore {
        for gi in gitignores {
            if gi.matched(path, is_dir).is_ignore() {
                return true;
            }
            // Paths are absolute; also try relative to listing root.
            if let Ok(rel) = path.strip_prefix(root) {
                if gi.matched(rel, is_dir).is_ignore() {
                    return true;
                }
            }
        }
    }

    false
}

fn built_in_deny_omits(path: &Path, is_dir: bool, size_bytes: u64) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if is_dir && BUILT_IN_DENY_DIR_NAMES.contains(&name) {
            return true;
        }
        if !is_dir {
            if BUILT_IN_DENY_FILE_NAMES.contains(&name) {
                return true;
            }
            if let Some(ext) = Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_ascii_lowercase())
            {
                if BUILT_IN_DENY_EXTENSIONS.contains(&ext.as_str()) {
                    return true;
                }
            }
            if size_bytes > crate::context::MAX_FILE_ATTACHMENT_BYTES {
                return true;
            }
        }
    }
    false
}

/// Loads `.gitignore` at `dir` (if present) scoped to that directory.
pub fn load_gitignore_at(dir: &Path) -> Option<Gitignore> {
    let gi_path = dir.join(".gitignore");
    if !gi_path.is_file() {
        return None;
    }
    let mut builder = GitignoreBuilder::new(dir);
    builder.add(&gi_path);
    builder.build().ok()
}

/// Canonicalize when possible; otherwise absolutize against the process CWD.
pub fn absolutize_path(path: &Path) -> PathBuf {
    if let Ok(canon) = path.canonicalize() {
        return canon;
    }
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
