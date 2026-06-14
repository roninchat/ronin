use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, MessageStatus, RoninPaths,
};
use tempfile::TempDir;

struct FakeProvider {
    chunks: Vec<ChatStreamEvent>,
}

impl ChatProvider for FakeProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(self.chunks.clone().into_iter()))
    }
}

fn setup_shell() -> (RoninShell, String, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("selected thread");
    (shell, thread_id, temp)
}

#[test]
fn retry_message_should_create_new_user_message_and_stream() {
    let (mut shell, thread_id, _temp) = setup_shell();

    let fail_provider = FakeProvider {
        chunks: vec![ChatStreamEvent::Error("offline".into())],
    };

    shell
        .begin_streaming(&thread_id, Some("Hello"), Box::new(fail_provider), "test-model")
        .expect("begin streaming");

    thread::sleep(Duration::from_millis(100));
    while shell.poll_streaming() {}

    let msgs = shell.state().messages.as_ref().unwrap();
    assert_eq!(msgs.len(), 2);
    let failed_msg_id = msgs[1].id.clone();
    assert_eq!(msgs[1].status, MessageStatus::Failed);

    let success_provider = FakeProvider {
        chunks: vec![
            ChatStreamEvent::Chunk("Hi ".into()),
            ChatStreamEvent::Chunk("there".into()),
        ],
    };

    shell
        .retry_message(&failed_msg_id, Box::new(success_provider), "test-model")
        .expect("retry message");

    thread::sleep(Duration::from_millis(100));
    while shell.poll_streaming() {}

    let msgs = shell.state().messages.as_ref().unwrap();
    assert_eq!(msgs.len(), 4);

    assert_eq!(msgs[0].role, MessageRole::User);
    assert_eq!(msgs[0].content, "Hello");

    assert_eq!(msgs[1].id, failed_msg_id);
    assert_eq!(msgs[1].status, MessageStatus::Failed);

    assert_eq!(msgs[2].role, MessageRole::User);
    assert_eq!(msgs[2].content, "Hello");

    assert_eq!(msgs[3].role, MessageRole::Assistant);
    assert_eq!(msgs[3].content, "Hi there");
    assert_eq!(msgs[3].status, MessageStatus::Complete);
}

#[test]
fn regenerate_last_assistant_should_cancel_and_stream_new_response() {
    let (mut shell, thread_id, _temp) = setup_shell();

    // Setup: User message and Complete assistant message
    let success_provider = FakeProvider {
        chunks: vec![ChatStreamEvent::Chunk("Old response".into())],
    };

    shell
        .begin_streaming(
            &thread_id,
            Some("Hello"),
            Box::new(success_provider),
            "test-model",
        )
        .expect("begin streaming");

    thread::sleep(Duration::from_millis(100));
    while shell.poll_streaming() {}

    let msgs = shell.state().messages.as_ref().unwrap();
    assert_eq!(msgs.len(), 2);
    let first_assistant_id = msgs[1].id.clone();
    assert_eq!(msgs[1].status, MessageStatus::Complete);

    // Act: Regenerate
    let regen_provider = FakeProvider {
        chunks: vec![ChatStreamEvent::Chunk("New response".into())],
    };

    shell
        .regenerate_last_assistant(&thread_id, Box::new(regen_provider), "test-model")
        .expect("regenerate");

    thread::sleep(Duration::from_millis(100));
    while shell.poll_streaming() {}

    let msgs = shell.state().messages.as_ref().unwrap();
    // 1 user, 1 new complete assistant (old assistant is deleted)
    assert_eq!(msgs.len(), 2);

    assert_eq!(msgs[0].role, MessageRole::User);
    assert_eq!(msgs[0].content, "Hello");

    assert_eq!(msgs[1].role, MessageRole::Assistant);
    assert_eq!(msgs[1].content, "New response");
    assert_eq!(msgs[1].status, MessageStatus::Complete);
    assert_ne!(msgs[1].id, first_assistant_id);
}
