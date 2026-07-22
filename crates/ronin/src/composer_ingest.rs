//! Drag-and-drop file ingest and clipboard image paste → attachment drafts.
//!
//! Public seams for path lists and image bytes, testable without GPUI.

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::folder_attach::{folder_attach_from_listing, FolderAttachState};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use ronin_core::{
    list_folder_entries_with_policy, read_file_attachment, AttachmentKind, ContextAttachmentDraft,
    ContextToolError, FolderListPolicy, MAX_IMAGE_ATTACHMENT_BYTES,
};

/// Outcome of ingesting one or more dropped filesystem paths.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DropIngestResult {
    /// Successfully built attachment drafts.
    pub drafts: Vec<ContextAttachmentDraft>,
    /// Folder attaches awaiting file selection.
    pub folders: Vec<FolderAttachState>,
    /// User-visible errors for paths that could not be attached.
    pub errors: Vec<String>,
}

/// Formats a drop/ingest failure into a clear, actionable message.
pub fn format_drop_error(path: &Path, err: &ContextToolError) -> String {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("file"));
    match err {
        ContextToolError::BinaryFile { .. } => {
            format!("Unsupported file type: {name} appears to be binary and cannot be attached as text. Use an image (png/jpg/gif/webp/svg) or a text file.")
        }
        ContextToolError::FileTooLarge { .. } => {
            format!("File too large: {name} exceeds the attachment size limit.")
        }
        ContextToolError::IsDirectory { .. } => {
            format!("Cannot attach directory: {name}. Drop individual files instead.")
        }
        other => format!("Could not attach {name}: {other}"),
    }
}

/// Builds attachment drafts (and folder selection states) from dropped paths.
///
/// Relative paths resolve against `workspace_root` when the active thread has an
/// explicit bind; otherwise against `cwd`. Absolute drops are unchanged.
/// Folder walks honor `policy` (gitignore / deny / never-list / allowlist).
pub fn ingest_dropped_paths(
    paths: &[PathBuf],
    workspace_root: Option<&Path>,
    cwd: &Path,
    policy: &FolderListPolicy,
) -> DropIngestResult {
    let mut drafts = Vec::new();
    let mut folders = Vec::new();
    let mut errors = Vec::new();
    for path in paths {
        let resolved = if path.is_absolute() {
            path.clone()
        } else {
            cwd.join(path)
        };
        if resolved.is_dir() {
            match list_folder_entries_with_policy(&resolved, workspace_root, cwd, policy) {
                Ok(listing) => folders.push(folder_attach_from_listing(listing)),
                Err(err) => errors.push(format_drop_error(path, &err)),
            }
            continue;
        }
        match read_file_attachment(path, workspace_root, cwd) {
            Ok(draft) => drafts.push(draft),
            Err(err) => errors.push(format_drop_error(path, &err)),
        }
    }
    DropIngestResult {
        drafts,
        folders,
        errors,
    }
}

/// Whether the drop-target overlay should be painted.
pub fn drop_overlay_should_show(dragging_files: bool) -> bool {
    dragging_files
}

fn unique_paste_name(ext: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("paste-{millis}.{ext}")
}

fn extension_for_mime(mime_type: &str) -> Option<&'static str> {
    match mime_type {
        "image/png" => Some("png"),
        "image/jpeg" | "image/jpg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

/// Persists encoded clipboard image bytes and returns an [`AttachmentKind::Image`] draft.
pub fn paste_image_bytes(
    encoded_bytes: &[u8],
    mime_type: &str,
    dest_dir: &Path,
) -> Result<ContextAttachmentDraft, String> {
    if encoded_bytes.is_empty() {
        return Err("Pasted image is empty.".into());
    }
    if encoded_bytes.len() as u64 > MAX_IMAGE_ATTACHMENT_BYTES {
        return Err(format!(
            "Pasted image exceeds the {} MB attachment size limit.",
            MAX_IMAGE_ATTACHMENT_BYTES / 1_048_576
        ));
    }
    let ext = extension_for_mime(mime_type)
        .ok_or_else(|| format!("Unsupported pasted image type: {mime_type}"))?;
    fs::create_dir_all(dest_dir).map_err(|e| format!("failed to create paste dir: {e}"))?;
    let name = unique_paste_name(ext);
    let path = dest_dir.join(&name);
    fs::write(&path, encoded_bytes).map_err(|e| format!("failed to save pasted image: {e}"))?;
    let size_bytes = encoded_bytes.len() as u64;
    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::Image,
        name: name.clone(),
        mime_type: mime_type.to_string(),
        content: None,
        path: Some(path),
        context_block: format!("[Attached image: {name}]"),
        size_bytes: Some(size_bytes),
    })
}

/// Encodes raw RGBA8 pixels as PNG, then builds an image attachment draft.
pub fn paste_rgba_image(
    width: usize,
    height: usize,
    rgba: &[u8],
    dest_dir: &Path,
) -> Result<ContextAttachmentDraft, String> {
    let expected = width
        .checked_mul(height)
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| "Invalid pasted image dimensions.".to_string())?;
    if rgba.len() != expected {
        return Err(format!(
            "Pasted image pixel data length mismatch (expected {expected} bytes, got {}).",
            rgba.len()
        ));
    }
    if expected as u64 > MAX_IMAGE_ATTACHMENT_BYTES {
        return Err(format!(
            "Pasted image exceeds the {} MB attachment size limit.",
            MAX_IMAGE_ATTACHMENT_BYTES / 1_048_576
        ));
    }

    let mut png_bytes = Vec::new();
    {
        let encoder = PngEncoder::new(Cursor::new(&mut png_bytes));
        encoder
            .write_image(rgba, width as u32, height as u32, ExtendedColorType::Rgba8)
            .map_err(|e| format!("failed to encode pasted image as PNG: {e}"))?;
    }
    paste_image_bytes(&png_bytes, "image/png", dest_dir)
}
