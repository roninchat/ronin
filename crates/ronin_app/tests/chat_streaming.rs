use ronin_app::RoninShell;
use ronin_core::{ChatProvider, ChatRequest, ChatStreamEvent, RoninPaths};
use tempfile::TempDir;

struct FakeChatProvider {
    chunks: Vec<ChatStreamEvent>,
}

impl ChatProvider for FakeChatProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(self.chunks.clone().into_iter()))
    }
}

#[test]
fn shell_should_stream_assistant_response_through_fake_provider() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths.clone()).expect("open shell");
    let thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("selected thread id");

    let provider = FakeChatProvider {
        chunks: vec![
            ChatStreamEvent::Chunk("Hello, ".into()),
            ChatStreamEvent::Chunk("I am an assistant.".into()),
        ],
    };

    shell
        .send_message_with_provider(
            &thread_id,
            "Tell me about yourself.",
            &provider,
            "test-model",
        )
        .expect("send message with provider");

    // Verify thread messages in shell state
    let state = shell.state();
    assert!(
        state.messages.is_some(),
        "messages should be loaded in shell state"
    );
    let msgs = state.messages.as_ref().expect("messages");

    assert_eq!(msgs.len(), 2, "should have user and assistant messages");
    assert_eq!(msgs[0].role, ronin_core::MessageRole::User);
    assert_eq!(msgs[0].content, "Tell me about yourself.");
    assert_eq!(msgs[0].status, ronin_core::MessageStatus::Complete);

    assert_eq!(msgs[1].role, ronin_core::MessageRole::Assistant);
    assert_eq!(msgs[1].content, "Hello, I am an assistant.");
    assert_eq!(msgs[1].status, ronin_core::MessageStatus::Complete);

    // Thread title should be derived
    let thread = state
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .expect("thread");
    assert_eq!(thread.title, "Tell me about yourself.");

    // Reopen and verify persistence
    drop(shell);

    let reopened = RoninShell::open(paths).expect("reopen shell");
    let reopened_state = reopened.state();
    let reopened_msgs = reopened_state
        .messages
        .as_ref()
        .expect("messages after reopen");
    assert_eq!(
        reopened_msgs.len(),
        2,
        "messages should persist across sessions"
    );
    assert_eq!(reopened_msgs[0].content, "Tell me about yourself.");
    assert_eq!(reopened_msgs[1].content, "Hello, I am an assistant.");
}
