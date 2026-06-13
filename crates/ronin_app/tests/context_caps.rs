use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, RoninPaths, RoninSession,
};
use std::cell::RefCell;
use tempfile::TempDir;

struct CapturingFakeProvider {
    chunks: Vec<ChatStreamEvent>,
    captured: RefCell<Option<ChatRequest>>,
}

impl CapturingFakeProvider {
    fn new(chunks: Vec<ChatStreamEvent>) -> Self {
        Self {
            chunks,
            captured: RefCell::new(None),
        }
    }
}

impl ChatProvider for CapturingFakeProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        *self.captured.borrow_mut() = Some(request.clone());
        Ok(Box::new(self.chunks.clone().into_iter()))
    }
}

#[test]
fn provider_request_should_include_system_prompt_and_exclude_streaming_placeholder() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let provider = CapturingFakeProvider::new(vec![ChatStreamEvent::Chunk("response".into())]);

    shell
        .send_message_with_provider(&thread_id, "Hello, system!", &provider, "test-model")
        .expect("send message");

    let captured = provider.captured.borrow();
    let request = captured.as_ref().expect("request was captured");

    // System prompt is prepended as first message.
    assert_eq!(request.messages.len(), 2, "system prompt + user message");
    assert_eq!(request.messages[0].role, "system");
    assert!(
        request.messages[0].content.contains("Ronin"),
        "system prompt should describe Ronin"
    );
    assert_eq!(request.messages[1].role, "user");
    assert_eq!(request.messages[1].content, "Hello, system!");
    assert_eq!(request.model, "test-model");
}

#[test]
fn truncation_notice_should_be_false_for_small_threads() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let provider = CapturingFakeProvider::new(vec![ChatStreamEvent::Chunk("ok".into())]);

    shell
        .send_message_with_provider(&thread_id, "Hi", &provider, "test")
        .expect("send");

    // With only 2 messages, no truncation needed
    assert!(!shell.state().truncation_notice);
}

#[test]
fn truncation_notice_should_be_true_when_context_exceeds_cap() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    // Setup: use RoninSession directly to populate many messages.
    let session = RoninSession::open(paths.clone()).expect("open session");
    let thread = session.create_thread().expect("create thread");
    let thread_id = thread.id.clone();

    let long_content = "A".repeat(2000); // ~2k chars each, 45 * 2k = 90k chars > 80k cap
    for i in 0..45 {
        let role = if i % 2 == 0 {
            MessageRole::User
        } else {
            MessageRole::Assistant
        };
        session
            .create_message(&thread_id, role, &long_content)
            .expect("create message");
    }
    drop(session);

    // Now open shell and send a message to trigger context cap logic.
    let mut shell = RoninShell::open(paths).expect("open shell");
    // Select the thread that has 45 messages
    shell.select_thread(&thread_id).expect("select thread");

    let provider =
        CapturingFakeProvider::new(vec![ChatStreamEvent::Chunk("truncated response".into())]);

    shell
        .send_message_with_provider(&thread_id, "Final message", &provider, "test")
        .expect("send");

    let captured = provider.captured.borrow();
    let request = captured.as_ref().expect("request captured");

    // Should be capped at 40 messages + 1 system prompt.
    assert!(
        request.messages.len() <= 41,
        "messages capped at ~40 + system prompt, got {}",
        request.messages.len()
    );

    let total_chars: usize = request.messages.iter().map(|m| m.content.len()).sum();
    let system_prompt_chars = request
        .messages
        .first()
        .map(|m| m.content.len())
        .unwrap_or(0);
    assert!(
        total_chars - system_prompt_chars <= 80_000,
        "chars capped at 80k (excluding system prompt), got {}",
        total_chars - system_prompt_chars
    );

    assert!(
        shell.state().truncation_notice,
        "truncation notice should be set"
    );
}
