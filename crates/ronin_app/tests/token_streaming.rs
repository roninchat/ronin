use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, MessageStatus, RoninPaths,
};
use tempfile::TempDir;

/// Fake provider that yields individual token-sized chunks.
struct TokenProvider {
    tokens: Vec<String>,
}

impl ChatProvider for TokenProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(
            self.tokens
                .iter()
                .map(|t| ChatStreamEvent::Chunk(t.clone())),
        ))
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
fn poll_streaming_should_drain_all_available_chunks_and_append_deltas() {
    let (mut shell, thread_id, _temp) = setup_shell();

    let provider = TokenProvider {
        tokens: vec!["Hello".into(), ", ".into(), "world".into()],
    };

    shell
        .begin_streaming(&thread_id, "hi", Box::new(provider), "test-model")
        .expect("begin streaming");

    // Let bg thread finish sending all chunks + Done.
    thread::sleep(Duration::from_millis(100));

    // Single poll should drain all chunks AND the Done message.
    let active = shell.poll_streaming();
    assert!(
        !active,
        "should be done — all chunks and Done drained in one call"
    );

    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");

    // Content should be all deltas concatenated via push_str.
    assert_eq!(
        assistant.content, "Hello, world",
        "all token deltas should be appended"
    );
    assert_eq!(
        assistant.status,
        MessageStatus::Complete,
        "status should be complete after Done"
    );
}

#[test]
fn poll_streaming_should_return_true_when_chunks_still_arriving() {
    let (mut shell, thread_id, _temp) = setup_shell();

    // Use a provider with a slow iterator that blocks between chunks.
    struct SlowProvider;
    impl ChatProvider for SlowProvider {
        fn stream_chat(
            &self,
            _request: &ChatRequest,
        ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
            Ok(Box::new(
                vec![
                    ChatStreamEvent::Chunk("first".into()),
                    // The bg thread will send "first", then block on next().
                    // We poll before the second chunk arrives.
                ]
                .into_iter()
                .chain(
                    std::iter::from_fn(|| {
                        // Simulate a slow model — block for 200ms then yield second chunk.
                        std::thread::sleep(Duration::from_millis(200));
                        Some(ChatStreamEvent::Chunk(" second".into()))
                    })
                    .take(1),
                ),
            ))
        }
    }

    shell
        .begin_streaming(&thread_id, "hi", Box::new(SlowProvider), "test-model")
        .expect("begin streaming");

    // Wait just long enough for first chunk but not second.
    thread::sleep(Duration::from_millis(50));

    // Poll should drain "first" and hit Empty — return true (still active).
    let active = shell.poll_streaming();
    assert!(
        active,
        "should still be active — second chunk hasn't arrived"
    );

    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");
    assert_eq!(assistant.content, "first", "should have first chunk only");

    // Wait for second chunk + Done.
    thread::sleep(Duration::from_millis(300));

    // Next poll drains second chunk + Done.
    let active = shell.poll_streaming();
    assert!(!active, "should be done after second chunk + Done");

    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");
    assert_eq!(assistant.content, "first second");
    assert_eq!(assistant.status, MessageStatus::Complete);
}

#[test]
fn poll_streaming_should_handle_error_during_stream() {
    let (mut shell, thread_id, _temp) = setup_shell();

    struct ErrorAfterOneProvider;
    impl ChatProvider for ErrorAfterOneProvider {
        fn stream_chat(
            &self,
            _request: &ChatRequest,
        ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
            Ok(Box::new(
                vec![
                    ChatStreamEvent::Chunk("partial".into()),
                    ChatStreamEvent::Error("connection lost".into()),
                ]
                .into_iter(),
            ))
        }
    }

    shell
        .begin_streaming(
            &thread_id,
            "hi",
            Box::new(ErrorAfterOneProvider),
            "test-model",
        )
        .expect("begin streaming");

    thread::sleep(Duration::from_millis(100));

    // Drains partial chunk + error in one call.
    let active = shell.poll_streaming();
    assert!(!active, "should stop after error");
    assert!(
        !shell.is_generation_active(),
        "generation should be inactive after error"
    );

    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");
    assert_eq!(
        assistant.content, "partial",
        "should retain content received before error"
    );
}
