//! Drag-and-drop path ingest and clipboard image paste → attachment drafts.

use std::path::PathBuf;

use ronin::composer_ingest::{
    drop_overlay_should_show, ingest_dropped_paths, paste_image_bytes, paste_rgba_image,
    DropIngestResult,
};
use ronin_core::{AttachmentKind, FolderListPolicy, MAX_IMAGE_ATTACHMENT_BYTES};
use tempfile::TempDir;

fn default_policy() -> FolderListPolicy {
    FolderListPolicy::default()
}

fn write_text(dir: &std::path::Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, body).unwrap();
    path
}

fn write_bytes(dir: &std::path::Path, name: &str, bytes: &[u8]) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn ingest_dropped_paths_should_attach_multiple_supported_files() {
    let temp = TempDir::new().unwrap();
    let a = write_text(temp.path(), "a.txt", "hello");
    let b = write_bytes(temp.path(), "b.png", b"\x89PNG\r\n\x1a\nfake");
    // tiny valid-enough png header; read_file_attachment only checks extension + size for images

    let result = ingest_dropped_paths(
        &[a.clone(), b.clone()],
        None,
        temp.path(),
        &default_policy(),
    );
    assert_eq!(result.drafts.len(), 2, "errors: {:?}", result.errors);
    assert!(result.errors.is_empty());
    assert_eq!(result.drafts[0].kind, AttachmentKind::File);
    assert_eq!(result.drafts[0].name, "a.txt");
    assert_eq!(result.drafts[1].kind, AttachmentKind::Image);
    assert_eq!(result.drafts[1].mime_type, "image/png");
}

#[test]
fn ingest_dropped_paths_should_report_unsupported_types_clearly() {
    let temp = TempDir::new().unwrap();
    let bin = write_bytes(temp.path(), "blob.bin", &[0, 1, 2, 3, 255, 0]);
    let result = ingest_dropped_paths(&[bin], None, temp.path(), &default_policy());
    assert!(result.drafts.is_empty());
    assert_eq!(result.errors.len(), 1);
    let msg = result.errors[0].to_lowercase();
    assert!(
        msg.contains("binary") || msg.contains("unsupported") || msg.contains("blob.bin"),
        "unclear error: {}",
        result.errors[0]
    );
}

#[test]
fn ingest_dropped_paths_should_partial_succeed_when_some_fail() {
    let temp = TempDir::new().unwrap();
    let ok = write_text(temp.path(), "ok.md", "# hi");
    let bad = write_bytes(temp.path(), "x.bin", b"abc\0def\xff");
    let DropIngestResult { drafts, errors, .. } =
        ingest_dropped_paths(&[ok, bad], None, temp.path(), &default_policy());
    assert_eq!(drafts.len(), 1);
    assert_eq!(errors.len(), 1);
    assert_eq!(drafts[0].name, "ok.md");
}

#[test]
fn ingest_dropped_folder_should_open_folder_attach_selection() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path().join("docs");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.md"), "one").unwrap();
    std::fs::write(dir.join("b.md"), "two").unwrap();

    let result = ingest_dropped_paths(&[dir], None, temp.path(), &default_policy());
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.drafts.is_empty());
    assert_eq!(result.folders.len(), 1);
    assert_eq!(result.folders[0].selected_count(), 2);
}

#[test]
fn ingest_dropped_folder_honors_never_list_policy() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path().join("secrets");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("x.txt"), "nope").unwrap();
    let policy = FolderListPolicy {
        never_list: vec![dir.clone()],
        ..FolderListPolicy::default()
    };
    let result = ingest_dropped_paths(&[dir], None, temp.path(), &policy);
    assert!(result.folders.is_empty());
    assert_eq!(result.errors.len(), 1);
    assert!(
        result.errors[0].to_lowercase().contains("blocked")
            || result.errors[0].to_lowercase().contains("never"),
        "unexpected: {}",
        result.errors[0]
    );
}

#[test]
fn drop_overlay_should_show_only_while_dragging_files() {
    assert!(drop_overlay_should_show(true));
    assert!(!drop_overlay_should_show(false));
}

#[test]
fn paste_image_bytes_should_create_image_attachment_on_disk() {
    let temp = TempDir::new().unwrap();
    // Minimal 1x1 PNG
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0x02, 0xFE, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let draft = paste_image_bytes(png, "image/png", temp.path()).expect("paste");
    assert_eq!(draft.kind, AttachmentKind::Image);
    assert_eq!(draft.mime_type, "image/png");
    let path = draft.path.expect("path");
    assert!(path.exists());
    assert!(path.starts_with(temp.path()));
    assert!(draft.size_bytes.unwrap() > 0);
    assert!(draft.context_block.to_lowercase().contains("image") || draft.name.contains("paste"));
}

#[test]
fn paste_image_bytes_should_reject_oversized_payload() {
    let temp = TempDir::new().unwrap();
    let huge = vec![0u8; (MAX_IMAGE_ATTACHMENT_BYTES as usize) + 8];
    let err = paste_image_bytes(&huge, "image/png", temp.path()).expect_err("too large");
    assert!(
        err.to_lowercase().contains("limit") || err.to_lowercase().contains("large"),
        "unclear: {err}"
    );
}

#[test]
fn paste_rgba_image_should_encode_png_attachment() {
    let temp = TempDir::new().unwrap();
    // 2x2 solid red RGBA
    let rgba = [
        255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255,
    ];
    let draft = paste_rgba_image(2, 2, &rgba, temp.path()).expect("rgba paste");
    assert_eq!(draft.kind, AttachmentKind::Image);
    assert_eq!(draft.mime_type, "image/png");
    let path = draft.path.expect("path");
    assert!(path.exists());
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");
}
