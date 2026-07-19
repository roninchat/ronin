//! Persistent file logging: enablement, rotation, and redaction on write.

use ronin_db::{
    default_log_dir, redact_log_text, FileLogOptions, RotatingLogWriter, REDACTED_PLACEHOLDER,
};
use tempfile::TempDir;

#[test]
fn default_log_dir_should_be_under_cache_ronin_logs() {
    let dir = default_log_dir(std::path::Path::new("/home/user/.cache"));
    assert_eq!(
        dir.to_string_lossy(),
        "/home/user/.cache/ronin/logs"
    );
}

#[test]
fn rotating_writer_should_append_redacted_lines_when_enabled() {
    let temp = TempDir::new().unwrap();
    let log_dir = temp.path().join("logs");
    let writer = RotatingLogWriter::open(&log_dir, 1024 * 1024).expect("open");

    writer
        .append_line("auth Bearer sk-secretVALUE password=hunter2")
        .expect("append");

    let contents = std::fs::read_to_string(writer.active_path()).expect("read");
    assert!(contents.contains(REDACTED_PLACEHOLDER), "{contents}");
    assert!(!contents.contains("sk-secretVALUE"), "{contents}");
    assert!(!contents.contains("hunter2"), "{contents}");
}

#[test]
fn rotating_writer_should_rotate_when_max_size_exceeded() {
    let temp = TempDir::new().unwrap();
    let log_dir = temp.path().join("logs");
    let writer = RotatingLogWriter::open(&log_dir, 64).expect("open");

    for i in 0..20 {
        writer
            .append_line(&format!("safe diagnostic line number {i} with padding ----"))
            .expect("append");
    }

    assert!(writer.active_path().is_file());
    assert!(
        log_dir.join("ronin.log.1").is_file(),
        "rotated file should exist after exceeding max size"
    );
}

#[test]
fn file_log_options_default_should_be_disabled() {
    let opts = FileLogOptions::default();
    assert!(!opts.enabled);
}

#[test]
fn redact_then_write_should_match_public_redact_api() {
    let sample = r#"prompt="secret prompt" https://u:p@host/v1?api_key=abc"#;
    assert_eq!(
        redact_log_text(sample),
        redact_log_text(sample),
        "redaction must be deterministic"
    );
    let temp = TempDir::new().unwrap();
    let writer = RotatingLogWriter::open(temp.path(), 10_000).unwrap();
    writer.append_line(sample).unwrap();
    let contents = std::fs::read_to_string(writer.active_path()).unwrap();
    assert!(!contents.contains("secret prompt"));
    assert!(!contents.contains("u:p@"));
    assert!(!contents.contains("api_key=abc"));
}
