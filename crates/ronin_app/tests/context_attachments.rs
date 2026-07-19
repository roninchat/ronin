use std::cell::RefCell;

use ronin_app::RoninShell;
use ronin_core::{
    clipboard_attachment, read_file_attachment, ChatProvider, ChatRequest, ChatStreamEvent,
    MessageRole, RoninPaths, RoninSession,
};
use tempfile::TempDir;

struct CapturingProvider {
    captured: RefCell<Option<ChatRequest>>,
}

impl CapturingProvider {
    fn new() -> Self {
        Self {
            captured: RefCell::new(None),
        }
    }
}

impl ChatProvider for CapturingProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        *self.captured.borrow_mut() = Some(request.clone());
        Ok(Box::new([ChatStreamEvent::Chunk("ok".into())].into_iter()))
    }
}

#[test]
fn send_message_with_provider_and_attachments_should_persist_and_inject_context() {
    let temp = TempDir::new().expect("temp dir");
    let attached_file = temp.path().join("notes.txt");
    std::fs::write(&attached_file, "file body").expect("write fixture");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths.clone()).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");
    let file = read_file_attachment(&attached_file, temp.path()).expect("file attachment");
    let clipboard = clipboard_attachment("clip body");
    let provider = CapturingProvider::new();

    shell
        .send_message_with_provider_and_attachments(
            &thread_id,
            "Summarize these",
            &[file, clipboard],
            &provider,
            "test-model",
        )
        .expect("send message");

    let captured = provider.captured.borrow();
    let request = captured.as_ref().expect("captured request");
    assert_eq!(request.messages[0].role, "system");
    assert!(request.messages[1]
        .content
        .contains("[Attached file: notes.txt]\nfile body"));
    assert!(request.messages[1]
        .content
        .contains("[Clipboard content]\nclip body"));
    assert_eq!(request.messages[2].role, "user");
    assert_eq!(request.messages[2].content, "Summarize these");

    drop(shell);
    let session = RoninSession::open(paths).expect("open session");
    let user_message = session
        .list_messages(&thread_id)
        .expect("messages")
        .into_iter()
        .find(|message| message.role == MessageRole::User)
        .expect("user message");
    let attachments = session
        .list_attachments(&user_message.id)
        .expect("attachments");

    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0].name, "notes.txt");
    assert_eq!(
        attachments[0].path.as_deref(),
        Some(attached_file.to_string_lossy().as_ref())
    );
    assert_eq!(attachments[0].content, None);
    assert_eq!(attachments[1].name, "clipboard");
    assert_eq!(attachments[1].content.as_deref(), Some("clip body"));
}

#[test]
fn send_message_with_provider_and_attachments_should_allow_attachment_only_prompt() {
    let temp = TempDir::new().expect("temp dir");
    let attached_file = temp.path().join("notes.txt");
    std::fs::write(&attached_file, "file body").expect("write fixture");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths.clone()).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");
    let file = read_file_attachment(&attached_file, temp.path()).expect("file attachment");
    let provider = CapturingProvider::new();

    shell
        .send_message_with_provider_and_attachments(
            &thread_id,
            "",
            &[file],
            &provider,
            "test-model",
        )
        .expect("send attachment-only message");

    let captured = provider.captured.borrow();
    let request = captured.as_ref().expect("captured request");
    assert!(request.messages[1]
        .content
        .contains("[Attached file: notes.txt]\nfile body"));
    assert_eq!(request.messages[2].role, "user");
    assert_eq!(request.messages[2].content, "See attached context.");
}

#[test]
fn send_message_should_persist_image_and_screenshot_attachment_metadata() {
    use ronin_core::{
        screenshot_attachment, AttachmentKind, FakeScreenshotCapturer, ScreenshotCapturer,
    };

    let temp = TempDir::new().expect("temp dir");
    let image_path = temp.path().join("diagram.png");
    std::fs::write(&image_path, b"\x89PNG\r\n\x1a\nfake").expect("write image");
    let shot_path = temp.path().join("portal.png");
    std::fs::write(&shot_path, b"\x89PNG\r\n\x1a\nshot").expect("write shot");

    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths.clone()).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let image = read_file_attachment(&image_path, temp.path()).expect("image");
    let captured = FakeScreenshotCapturer::new(shot_path.clone())
        .capture(temp.path())
        .expect("capture");
    let screenshot = screenshot_attachment(&captured).expect("screenshot draft");
    let provider = CapturingProvider::new();

    shell
        .send_message_with_provider_and_attachments(
            &thread_id,
            "Review visuals",
            &[image, screenshot],
            &provider,
            "test-model",
        )
        .expect("send");

    drop(shell);
    let session = RoninSession::open(paths).expect("reopen");
    let user_message = session
        .list_messages(&thread_id)
        .expect("messages")
        .into_iter()
        .find(|m| m.role == MessageRole::User)
        .expect("user message");
    let attachments = session
        .list_attachments(&user_message.id)
        .expect("attachments");

    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0].kind, AttachmentKind::Image);
    assert_eq!(attachments[0].mime_type, "image/png");
    assert_eq!(
        attachments[0].path.as_deref(),
        Some(image_path.to_string_lossy().as_ref())
    );
    assert_eq!(attachments[1].kind, AttachmentKind::Screenshot);
    assert_eq!(attachments[1].mime_type, "image/png");
    assert_eq!(
        attachments[1].path.as_deref(),
        Some(shot_path.to_string_lossy().as_ref())
    );
}
