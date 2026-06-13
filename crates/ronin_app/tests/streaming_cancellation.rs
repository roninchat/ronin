use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, MessageStatus, RoninPaths,
};
use tempfile::TempDir;

/// Provider that blocks until cancelled.
struct InfiniteProvider;
impl ChatProvider for InfiniteProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(std::iter::from_fn(|| {
            std::thread::sleep(Duration::from_millis(100));
            Some(ChatStreamEvent::Chunk("tick ".into()))
        })))
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
fn cancel_streaming_should_stop_appending_chunks_and_mark_cancelled() {
    let (mut shell, thread_id, _temp) = setup_shell();

    shell
        .begin_streaming(
            &thread_id,
            "start",
            Box::new(InfiniteProvider),
            "test-model",
        )
        .expect("begin streaming");

    // Let some chunks arrive
    thread::sleep(Duration::from_millis(300));
    shell.poll_streaming(); // drain initial ticks

    // Ensure it's actively streaming
    assert!(shell.is_generation_active());

    // Cancel
    shell.cancel_streaming().expect("cancel streaming");

    // Must immediately be inactive
    assert!(!shell.is_generation_active());

    // Wait a bit to ensure background thread drops/stops sending
    thread::sleep(Duration::from_millis(200));

    // Poll again, should return false (not streaming)
    let active = shell.poll_streaming();
    assert!(!active);

    // Verify status is cancelled
    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");

    assert_eq!(assistant.status, MessageStatus::Cancelled);
}

#[test]
fn cancel_streaming_should_allow_new_generation_after_cancel() {
    let (mut shell, thread_id, _temp) = setup_shell();

    shell
        .begin_streaming(
            &thread_id,
            "start",
            Box::new(InfiniteProvider),
            "test-model",
        )
        .expect("begin streaming");

    // Should error if we try to start another stream while active
    let err = shell
        .begin_streaming(
            &thread_id,
            "again",
            Box::new(InfiniteProvider),
            "test-model",
        )
        .unwrap_err();
    assert!(matches!(
        err,
        ronin_app::RoninAppError::GenerationInProgress
    ));

    shell.cancel_streaming().expect("cancel streaming");

    // Now we can send again
    shell
        .send_message(&thread_id, "after cancel")
        .expect("send after cancel");
}

#[test]
fn shell_open_should_repair_stale_streaming_messages_on_startup() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    // Open a shell and begin an infinite stream (will create a streaming msg in DB)
    let mut shell = RoninShell::open(paths.clone()).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    shell
        .begin_streaming(&thread_id, "run", Box::new(InfiniteProvider), "test-model")
        .expect("begin streaming");

    // Drop the shell simulating a crash/exit without completion
    drop(shell);

    // Reopen the shell
    let mut reopened_shell = RoninShell::open(paths).expect("reopen shell");
    reopened_shell
        .select_thread(&thread_id)
        .expect("select thread");

    let msgs = reopened_shell.state().messages.as_ref().unwrap();
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant msg");

    // It should have been repaired to Failed
    assert_eq!(assistant.status, MessageStatus::Failed);
    assert_eq!(
        assistant.error_message.as_deref(),
        Some("Generation interrupted because Ronin exited before the response completed.")
    );
}

#[test]
fn cancelled_message_should_not_be_resumable() {
    // This is basically verified by the status being Cancelled and having no Resume method,
    // but just checking the struct state.
    let (mut shell, thread_id, _temp) = setup_shell();

    shell
        .begin_streaming(
            &thread_id,
            "start",
            Box::new(InfiniteProvider),
            "test-model",
        )
        .expect("begin streaming");
    shell.cancel_streaming().expect("cancel streaming");

    let msgs = shell.state().messages.as_ref().expect("messages");
    let assistant = msgs
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .unwrap();
    assert_eq!(assistant.status, MessageStatus::Cancelled);
}
