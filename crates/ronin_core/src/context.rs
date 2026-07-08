//! Explicit user context: `@file`, `@clipboard`, `@memory`, `@artifact` parsing
//! and context attachment drafts.

use std::io;
use std::path::{Path, PathBuf};

use crate::domain::{Artifact, AttachmentKind, Memory};

/// A parsed explicit context reference from the composer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextToolRef {
    /// User requested a file attachment by path.
    File(String),
    /// User requested a memory attachment by id.
    Memory(String),
    /// User requested an artifact attachment by id.
    Artifact(String),
    /// User requested current clipboard text.
    Clipboard,
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
            || candidate.starts_with("@memory:")
            || candidate.starts_with("@artifact:")
            || candidate
                .get(.."@clipboard".len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("@clipboard"))
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

/// Maximum file attachment size in bytes.
pub const MAX_FILE_ATTACHMENT_BYTES: u64 = 1_048_576;

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
}

/// Reads a text file selected by explicit `@file` context.
pub fn read_file_attachment(
    path: impl AsRef<Path>,
    cwd: impl AsRef<Path>,
) -> std::result::Result<ContextAttachmentDraft, ContextToolError> {
    let requested_path = path.as_ref();
    let resolved_path = if requested_path.is_absolute() {
        requested_path.to_path_buf()
    } else {
        cwd.as_ref().join(requested_path)
    };

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
    let name = resolved_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attached file")
        .to_string();

    Ok(ContextAttachmentDraft {
        kind: AttachmentKind::File,
        name: name.clone(),
        mime_type: "text/plain".to_string(),
        content: None,
        path: Some(resolved_path),
        context_block: format!("[Attached file: {name}]\n{text}"),
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
