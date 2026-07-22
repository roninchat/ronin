//! Attachment preview models for composer and message rendering.

use std::path::{Path, PathBuf};

use ronin_core::{Attachment, AttachmentKind, ContextAttachmentDraft};

/// Maximum characters in a text-file content snippet preview.
pub const FILE_SNIPPET_CHARS: usize = 120;

/// Preview shown for an attachment in the composer or a sent message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachmentPreview {
    /// Image or screenshot with a path suitable for thumbnail rendering.
    Image {
        /// Display name.
        name: String,
        /// Attachment kind (`Image` or `Screenshot`).
        kind: AttachmentKind,
        /// MIME type.
        mime_type: String,
        /// Filesystem path to the image.
        path: PathBuf,
        /// Size in bytes when known.
        size_bytes: Option<u64>,
    },
    /// Non-image file with size, type indicator, and optional text snippet.
    File {
        /// Display name.
        name: String,
        /// MIME type / type indicator.
        mime_type: String,
        /// Human-readable size label (e.g. `"1.2 KB"`).
        size_label: String,
        /// Short content snippet for text files.
        snippet: Option<String>,
        /// Source path when known.
        path: Option<PathBuf>,
    },
    /// Clipboard / memory / artifact text attachments.
    Text {
        /// Display name.
        name: String,
        /// Attachment kind.
        kind: AttachmentKind,
        /// Optional short snippet of content.
        snippet: Option<String>,
    },
}

/// Builds a composer/message preview from a context attachment draft.
pub fn preview_from_draft(draft: &ContextAttachmentDraft) -> AttachmentPreview {
    match draft.kind {
        AttachmentKind::Image | AttachmentKind::Screenshot => AttachmentPreview::Image {
            name: draft.name.clone(),
            kind: draft.kind,
            mime_type: draft.mime_type.clone(),
            path: draft
                .path
                .clone()
                .unwrap_or_else(|| PathBuf::from(&draft.name)),
            size_bytes: draft.size_bytes,
        },
        AttachmentKind::File | AttachmentKind::Folder => {
            let snippet = draft
                .path
                .as_ref()
                .and_then(|p| text_file_snippet(p, FILE_SNIPPET_CHARS));
            AttachmentPreview::File {
                name: draft.name.clone(),
                mime_type: draft.mime_type.clone(),
                size_label: format_size_bytes(draft.size_bytes.unwrap_or(0)),
                snippet: snippet
                    .or_else(|| Some(truncate_chars(&draft.context_block, FILE_SNIPPET_CHARS))),
                path: draft.path.clone(),
            }
        }
        AttachmentKind::Clipboard | AttachmentKind::Memory | AttachmentKind::Artifact => {
            let snippet = draft
                .content
                .as_ref()
                .map(|c| truncate_chars(c, FILE_SNIPPET_CHARS));
            AttachmentPreview::Text {
                name: draft.name.clone(),
                kind: draft.kind,
                snippet,
            }
        }
    }
}

/// Builds a preview from a persisted attachment row.
pub fn preview_from_attachment(attachment: &Attachment) -> AttachmentPreview {
    match attachment.kind {
        AttachmentKind::Image | AttachmentKind::Screenshot => AttachmentPreview::Image {
            name: attachment.name.clone(),
            kind: attachment.kind,
            mime_type: attachment.mime_type.clone(),
            path: PathBuf::from(attachment.path.as_deref().unwrap_or(&attachment.name)),
            size_bytes: None,
        },
        AttachmentKind::File | AttachmentKind::Folder => {
            let path = attachment.path.as_ref().map(PathBuf::from);
            let size_bytes = path
                .as_ref()
                .and_then(|p| std::fs::metadata(p).ok())
                .map(|m| m.len())
                .unwrap_or(0);
            let snippet = path
                .as_ref()
                .and_then(|p| text_file_snippet(p, FILE_SNIPPET_CHARS))
                .or_else(|| {
                    attachment
                        .content
                        .as_ref()
                        .map(|c| truncate_chars(c, FILE_SNIPPET_CHARS))
                });
            AttachmentPreview::File {
                name: attachment.name.clone(),
                mime_type: attachment.mime_type.clone(),
                size_label: format_size_bytes(size_bytes),
                snippet,
                path,
            }
        }
        AttachmentKind::Clipboard | AttachmentKind::Memory | AttachmentKind::Artifact => {
            let snippet = attachment
                .content
                .as_ref()
                .map(|c| truncate_chars(c, FILE_SNIPPET_CHARS));
            AttachmentPreview::Text {
                name: attachment.name.clone(),
                kind: attachment.kind,
                snippet,
            }
        }
    }
}

/// Formats a byte count as a short human-readable label.
pub fn format_size_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

fn text_file_snippet(path: &Path, max_chars: usize) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.iter().take(8 * 1024).any(|b| *b == 0) {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    Some(truncate_chars(&text, max_chars))
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let snippet: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{snippet}…")
    } else {
        snippet
    }
}
