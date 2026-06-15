use ronin_core::{parse_context_tools, read_file_attachment, ContextToolError, ContextToolRef};
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
fn read_file_attachment_should_resolve_relative_text_file_and_format_context() {
    let temp = TempDir::new().expect("temp dir");
    let file_path = temp.path().join("notes.txt");
    std::fs::write(&file_path, "alpha\nbeta").expect("write fixture");

    let draft = read_file_attachment("notes.txt", temp.path()).expect("read attachment");

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

    let error = read_file_attachment("large.txt", temp.path()).expect_err("large file rejected");

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

    let error = read_file_attachment("image.bin", temp.path()).expect_err("binary rejected");

    assert!(matches!(error, ContextToolError::BinaryFile { .. }));
    assert_eq!(error.to_string(), "file image.bin appears to be binary");
}
