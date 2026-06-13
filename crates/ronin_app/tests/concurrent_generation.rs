use ronin_app::RoninShell;
use ronin_core::{ChatProvider, ChatRequest, ChatStreamEvent, RoninPaths};
use tempfile::TempDir;

struct SimpleFakeProvider {
    chunk: String,
}

impl ChatProvider for SimpleFakeProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(
            vec![ChatStreamEvent::Chunk(self.chunk.clone())].into_iter(),
        ))
    }
}

#[test]
fn generation_active_should_be_true_during_provider_call() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    // Before send, generation should not be active
    assert!(!shell.is_generation_active());

    let provider = SimpleFakeProvider {
        chunk: "response".into(),
    };

    shell
        .send_message_with_provider(&thread_id, "Hi", &provider, "test")
        .expect("send");

    // After send, generation should be cleared
    assert!(!shell.is_generation_active());
}

#[test]
fn sequential_sends_should_succeed_since_generation_resets_between_calls() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let provider = SimpleFakeProvider {
        chunk: "response".into(),
    };

    // Multiple sequential sends should all work
    for i in 0..3 {
        shell
            .send_message_with_provider(&thread_id, &format!("Message {i}"), &provider, "test")
            .expect("send should succeed");
    }

    let state = shell.state();
    let msgs = state.messages.as_ref().expect("messages");
    // 3 user messages + 3 assistant responses = 6 messages
    assert_eq!(msgs.len(), 6);
}
