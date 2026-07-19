//! Attachment preview: image thumbnails and improved file metadata.

use std::path::PathBuf;

use ronin::attachment_preview::{
    format_size_bytes, preview_from_attachment, preview_from_draft, AttachmentPreview,
    FILE_SNIPPET_CHARS,
};
use ronin_core::{
    read_file_attachment, screenshot_attachment, Attachment, AttachmentId, AttachmentKind,
    ContextAttachmentDraft,
};
use tempfile::TempDir;

#[test]
fn preview_from_draft_should_expose_image_thumbnail_path_for_images() {
    let draft = ContextAttachmentDraft {
        kind: AttachmentKind::Image,
        name: "photo.png".into(),
        mime_type: "image/png".into(),
        content: None,
        path: Some(PathBuf::from("/tmp/photo.png")),
        context_block: "[Attached image: photo.png]".into(),
        size_bytes: Some(2048),
    };

    match preview_from_draft(&draft) {
        AttachmentPreview::Image {
            name,
            kind,
            mime_type,
            path,
            size_bytes,
        } => {
            assert_eq!(name, "photo.png");
            assert_eq!(kind, AttachmentKind::Image);
            assert_eq!(mime_type, "image/png");
            assert_eq!(path, PathBuf::from("/tmp/photo.png"));
            assert_eq!(size_bytes, Some(2048));
        }
        other => panic!("expected image preview, got {other:?}"),
    }
}

#[test]
fn preview_from_draft_should_expose_screenshot_as_image_preview() {
    let draft = ContextAttachmentDraft {
        kind: AttachmentKind::Screenshot,
        name: "shot.png".into(),
        mime_type: "image/png".into(),
        content: None,
        path: Some(PathBuf::from("/tmp/shot.png")),
        context_block: "[Screenshot: shot.png]".into(),
        size_bytes: Some(100),
    };

    match preview_from_draft(&draft) {
        AttachmentPreview::Image { kind, path, .. } => {
            assert_eq!(kind, AttachmentKind::Screenshot);
            assert_eq!(path, PathBuf::from("/tmp/shot.png"));
        }
        other => panic!("expected image preview, got {other:?}"),
    }
}

#[test]
fn preview_from_draft_should_show_size_type_and_snippet_for_text_files() {
    let temp = TempDir::new().expect("temp");
    let path = temp.path().join("notes.txt");
    let body = "hello world\nsecond line";
    std::fs::write(&path, body).expect("write");

    let draft = read_file_attachment(&path, temp.path()).expect("read");
    match preview_from_draft(&draft) {
        AttachmentPreview::File {
            name,
            mime_type,
            size_label,
            snippet,
            path: preview_path,
        } => {
            assert_eq!(name, "notes.txt");
            assert_eq!(mime_type, "text/plain");
            assert_eq!(size_label, format_size_bytes(body.len() as u64));
            assert_eq!(snippet.as_deref(), Some(body));
            assert_eq!(preview_path.as_deref(), Some(path.as_path()));
        }
        other => panic!("expected file preview, got {other:?}"),
    }
}

#[test]
fn file_snippet_should_truncate_long_text() {
    let temp = TempDir::new().expect("temp");
    let path = temp.path().join("long.txt");
    let body: String = "x".repeat(FILE_SNIPPET_CHARS + 40);
    std::fs::write(&path, &body).expect("write");

    let draft = read_file_attachment(&path, temp.path()).expect("read");
    match preview_from_draft(&draft) {
        AttachmentPreview::File {
            snippet: Some(snippet),
            ..
        } => {
            assert_eq!(snippet.chars().count(), FILE_SNIPPET_CHARS + 1);
            assert!(snippet.ends_with('…'));
        }
        other => panic!("expected truncated file snippet, got {other:?}"),
    }
}

#[test]
fn preview_from_attachment_should_show_inline_image_for_persisted_rows() {
    let attachment = Attachment {
        id: AttachmentId("a1".into()),
        message_id: "m1".into(),
        kind: AttachmentKind::Image,
        name: "pic.webp".into(),
        mime_type: "image/webp".into(),
        content: None,
        path: Some("/data/pic.webp".into()),
        created_at: 0,
    };

    match preview_from_attachment(&attachment) {
        AttachmentPreview::Image {
            name,
            path,
            mime_type,
            ..
        } => {
            assert_eq!(name, "pic.webp");
            assert_eq!(mime_type, "image/webp");
            assert_eq!(path, PathBuf::from("/data/pic.webp"));
        }
        other => panic!("expected image preview, got {other:?}"),
    }
}

#[test]
fn screenshot_and_image_attachment_flow_through_draft_builders() {
    let temp = TempDir::new().expect("temp");
    let shot = temp.path().join("cap.png");
    let img = temp.path().join("logo.svg");
    std::fs::write(&shot, b"\x89PNG\r\n\x1a\n").expect("png");
    std::fs::write(&img, b"<svg></svg>").expect("svg");

    let shot_draft = screenshot_attachment(&shot).expect("screenshot");
    let img_draft = read_file_attachment(&img, temp.path()).expect("image");

    assert!(matches!(
        preview_from_draft(&shot_draft),
        AttachmentPreview::Image {
            kind: AttachmentKind::Screenshot,
            ..
        }
    ));
    assert!(matches!(
        preview_from_draft(&img_draft),
        AttachmentPreview::Image {
            kind: AttachmentKind::Image,
            ..
        }
    ));
}
