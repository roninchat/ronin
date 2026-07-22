use ronin_core::{
    artifact_attachment, parse_context_tools, read_file_attachment, Artifact, ArtifactId,
    AttachmentKind, ContextToolError, ContextToolRef,
};
use tempfile::TempDir;

#[test]
fn parse_context_tools_should_find_file_and_clipboard_refs_and_strip_visible_message() {
    let parsed = parse_context_tools(
        r#"Review @file:"./fixtures/with spaces.txt" and @clipboard before replying"#,
    );

    assert_eq!(
        parsed.visible_message, "Review and before replying",
        "visible message should omit explicit context refs"
    );
    assert_eq!(
        parsed.refs,
        vec![
            ContextToolRef::File("./fixtures/with spaces.txt".into()),
            ContextToolRef::Clipboard,
        ]
    );
}

#[test]
fn parse_context_tools_should_find_artifact_ref_and_strip_visible_message() {
    let parsed = parse_context_tools("Refactor @artifact:abc-123 for clarity");

    assert_eq!(
        parsed.visible_message, "Refactor for clarity",
        "visible message should omit explicit artifact ref"
    );
    assert_eq!(
        parsed.refs,
        vec![ContextToolRef::Artifact("abc-123".into())]
    );
}

#[test]
fn parse_context_tools_should_find_screenshot_ref_and_strip_visible_message() {
    let parsed = parse_context_tools("Look at @screenshot please");

    assert_eq!(
        parsed.visible_message, "Look at please",
        "visible message should omit explicit screenshot ref"
    );
    assert_eq!(parsed.refs, vec![ContextToolRef::Screenshot]);
}

#[test]
fn read_file_attachment_should_resolve_relative_text_file_and_format_context() {
    let temp = TempDir::new().expect("temp dir");
    let file_path = temp.path().join("notes.txt");
    std::fs::write(&file_path, "alpha\nbeta").expect("write fixture");

    let draft = read_file_attachment("notes.txt", None, temp.path()).expect("read attachment");

    assert_eq!(draft.name, "notes.txt");
    assert_eq!(
        draft.context_block,
        "[Attached file: notes.txt]\nalpha\nbeta"
    );
    assert_eq!(draft.path.as_deref(), Some(file_path.as_path()));
    assert_eq!(draft.content, None);
}

#[test]
fn read_file_attachment_should_reject_files_over_one_mb() {
    let temp = TempDir::new().expect("temp dir");
    std::fs::write(temp.path().join("large.txt"), vec![b'a'; 1_048_577]).expect("write fixture");

    let error =
        read_file_attachment("large.txt", None, temp.path()).expect_err("large file rejected");

    assert!(matches!(error, ContextToolError::FileTooLarge { .. }));
    assert_eq!(
        error.to_string(),
        "file large.txt exceeds 1 MB attachment limit"
    );
}

#[test]
fn read_file_attachment_should_reject_binary_files_with_clear_error() {
    let temp = TempDir::new().expect("temp dir");
    std::fs::write(temp.path().join("image.bin"), b"abc\0def").expect("write fixture");

    let error = read_file_attachment("image.bin", None, temp.path()).expect_err("binary rejected");

    assert!(matches!(error, ContextToolError::BinaryFile { .. }));
    assert_eq!(error.to_string(), "file image.bin appears to be binary");
}

#[test]
fn artifact_attachment_should_build_correct_context_draft() {
    let artifact = Artifact {
        id: ArtifactId("art-001".into()),
        thread_id: "thread-1".into(),
        message_id: "msg-1".into(),
        title: "My Refactored Code".into(),
        content: "fn main() {}".into(),
        kind: "document".into(),
        language: None,
        created_at: 0,
    };

    let draft = artifact_attachment(&artifact);

    assert_eq!(draft.kind, AttachmentKind::Artifact);
    assert_eq!(draft.name, "artifact:My Refactored Code");
    assert_eq!(draft.mime_type, "text/plain");
    assert_eq!(draft.content.as_deref(), Some("fn main() {}"));
    assert_eq!(
        draft.context_block,
        "[Artifact: My Refactored Code]\nfn main() {}"
    );
    assert!(draft.path.is_none());
}

#[test]
fn screenshot_attachment_should_build_draft_from_captured_path() {
    use ronin_core::screenshot_attachment;
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("shot.png");
    std::fs::write(&path, b"\x89PNG\r\n\x1a\nfake").expect("write png");

    let draft = screenshot_attachment(&path).expect("screenshot attachment");

    assert_eq!(draft.kind, AttachmentKind::Screenshot);
    assert_eq!(draft.name, "shot.png");
    assert_eq!(draft.mime_type, "image/png");
    assert_eq!(draft.path.as_deref(), Some(path.as_path()));
    assert_eq!(draft.content, None);
    assert_eq!(draft.size_bytes, Some(path.metadata().unwrap().len()));
    assert!(draft.context_block.contains("[Screenshot: shot.png]"));
}

#[test]
fn fake_screenshot_capturer_should_return_path_for_attachment_flow() {
    use ronin_core::{screenshot_attachment, FakeScreenshotCapturer, ScreenshotCapturer};
    let temp = TempDir::new().expect("temp dir");
    let fixture = temp.path().join("portal-shot.png");
    std::fs::write(&fixture, b"\x89PNG\r\n\x1a\n").expect("write");

    let capturer = FakeScreenshotCapturer::new(fixture.clone());
    let captured = capturer
        .capture(temp.path())
        .expect("fake capture succeeds");
    assert_eq!(captured, fixture);

    let draft = screenshot_attachment(&captured).expect("attach screenshot");
    assert_eq!(draft.kind, AttachmentKind::Screenshot);
    assert_eq!(draft.path.as_deref(), Some(fixture.as_path()));
}

#[test]
fn read_file_attachment_should_attach_image_files_with_image_kind() {
    let temp = TempDir::new().expect("temp dir");
    let path = temp.path().join("photo.jpg");
    // JPEG SOI marker + null byte (would fail binary text check)
    std::fs::write(&path, b"\xff\xd8\xff\xe0\x00\x10JFIF\0rest").expect("write jpg");

    let draft = read_file_attachment("photo.jpg", None, temp.path()).expect("read image");

    assert_eq!(draft.kind, AttachmentKind::Image);
    assert_eq!(draft.name, "photo.jpg");
    assert_eq!(draft.mime_type, "image/jpeg");
    assert_eq!(draft.path.as_deref(), Some(path.as_path()));
    assert_eq!(draft.content, None);
    assert!(draft.context_block.contains("[Attached image: photo.jpg]"));
}

#[test]
fn read_file_attachment_should_accept_supported_image_extensions() {
    let temp = TempDir::new().expect("temp dir");
    for (name, mime) in [
        ("a.png", "image/png"),
        ("b.gif", "image/gif"),
        ("c.webp", "image/webp"),
        ("d.svg", "image/svg+xml"),
        ("e.JPG", "image/jpeg"),
    ] {
        let path = temp.path().join(name);
        std::fs::write(&path, b"\0img").expect("write");
        let draft = read_file_attachment(name, None, temp.path()).expect(name);
        assert_eq!(draft.kind, AttachmentKind::Image, "{name}");
        assert_eq!(draft.mime_type, mime, "{name}");
    }
}

#[test]
fn parse_context_tools_should_recognize_folder_ref() {
    let parsed = parse_context_tools("see @folder:src/lib then ask");
    assert_eq!(parsed.refs, vec![ContextToolRef::Folder("src/lib".into())]);
    assert_eq!(parsed.visible_message, "see then ask");
}

#[test]
fn list_folder_entries_should_list_files_and_allow_selection_to_draft() {
    use ronin_core::{
        folder_attachment_from_selection, list_folder_entries, FOLDER_LIST_MAX_ENTRIES,
    };

    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("proj");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("README.md"), "# hi").unwrap();
    std::fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn x() {}").unwrap();

    let listing = list_folder_entries(&root, None, temp.path()).expect("list");
    assert_eq!(listing.name, "proj");
    assert!(!listing.truncated);
    let rels: Vec<_> = listing
        .entries
        .iter()
        .map(|e| e.relative_path.as_str())
        .collect();
    assert!(rels.contains(&"README.md"));
    assert!(rels.contains(&"src/main.rs"));
    assert!(rels.contains(&"src/lib.rs"));
    assert!(listing.entries.len() <= FOLDER_LIST_MAX_ENTRIES);

    let draft =
        folder_attachment_from_selection(&listing, &["README.md".into(), "src/main.rs".into()])
            .expect("draft");
    assert_eq!(draft.kind, AttachmentKind::Folder);
    assert!(draft.context_block.contains("README.md"));
    assert!(draft.context_block.contains("# hi"));
    assert!(draft.context_block.contains("src/main.rs"));
    assert!(draft.context_block.contains("fn main()"));
    assert!(!draft.context_block.contains("pub fn x"));
}

#[test]
fn list_folder_entries_should_bound_deep_listings() {
    use ronin_core::{list_folder_entries, FOLDER_LIST_MAX_DEPTH, FOLDER_LIST_MAX_ENTRIES};

    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("deep");
    // Build deeper than max depth and more files than max entries.
    let mut cur = root.clone();
    for i in 0..=FOLDER_LIST_MAX_DEPTH + 2 {
        cur = cur.join(format!("d{i}"));
        std::fs::create_dir_all(&cur).unwrap();
        std::fs::write(cur.join(format!("f{i}.txt")), format!("file {i}")).unwrap();
    }
    for i in 0..FOLDER_LIST_MAX_ENTRIES + 20 {
        std::fs::write(root.join(format!("top-{i}.txt")), "x").unwrap();
    }

    let listing = list_folder_entries(&root, None, temp.path()).expect("list");
    assert!(
        listing.truncated,
        "large/deep folders must report truncation"
    );
    assert!(listing.entries.len() <= FOLDER_LIST_MAX_ENTRIES);
    let has_too_deep = listing
        .entries
        .iter()
        .any(|e| e.relative_path.matches('/').count() > FOLDER_LIST_MAX_DEPTH);
    assert!(
        !has_too_deep,
        "entries should stay within max depth; got {:?}",
        listing
            .entries
            .iter()
            .map(|e| &e.relative_path)
            .collect::<Vec<_>>()
    );
}
