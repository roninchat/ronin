//! Concurrent per-thread generations through public shell APIs.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, MessageStatus, RoninPaths,
};
use tempfile::TempDir;

/// Slow provider: emits `prefix` chunks until `stop` is set, then finishes.
struct SlowMarkerProvider {
    prefix: String,
    stop: Arc<Mutex<bool>>,
}

impl ChatProvider for SlowMarkerProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        let prefix = self.prefix.clone();
        let stop = Arc::clone(&self.stop);
        let mut n = 0u32;
        Ok(Box::new(std::iter::from_fn(move || {
            if *stop.lock().unwrap() {
                return None;
            }
            n += 1;
            thread::sleep(Duration::from_millis(40));
            Some(ChatStreamEvent::Chunk(format!("{prefix}{n} ")))
        })))
    }
}

fn open_shell() -> (TempDir, RoninShell) {
    let temp = TempDir::new().expect("temp");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open");
    (temp, shell)
}

#[test]
fn user_can_start_generation_in_thread_a_then_thread_b_concurrently() {
    let (_temp, mut shell) = open_shell();

    let thread_a = shell.state().selected_thread_id.clone().expect("thread a");
    let thread_b = shell.create_new_thread().expect("thread b").id;

    let stop_a = Arc::new(Mutex::new(false));
    let stop_b = Arc::new(Mutex::new(false));

    shell.select_thread(&thread_a).expect("select a");
    shell
        .begin_streaming(
            &thread_a,
            Some("prompt A"),
            Box::new(SlowMarkerProvider {
                prefix: "A".into(),
                stop: Arc::clone(&stop_a),
            }),
            "model-a",
        )
        .expect("start A");

    assert!(
        shell.is_thread_generating(&thread_a),
        "thread A should be generating"
    );

    // Switch to B and start another generation while A is still active.
    shell.select_thread(&thread_b).expect("select b");
    shell
        .begin_streaming(
            &thread_b,
            Some("prompt B"),
            Box::new(SlowMarkerProvider {
                prefix: "B".into(),
                stop: Arc::clone(&stop_b),
            }),
            "model-b",
        )
        .expect("start B while A still generating");

    assert!(shell.is_thread_generating(&thread_a));
    assert!(shell.is_thread_generating(&thread_b));
    assert!(
        shell.is_generation_active(),
        "current thread (B) should report generation active"
    );

    // Both streams should produce content independently.
    thread::sleep(Duration::from_millis(200));
    shell.poll_streaming(); // drains all; updates selected (B)

    shell.select_thread(&thread_a).expect("back to a");
    shell.poll_streaming();

    let msgs_a = shell.state().messages.as_ref().expect("msgs a");
    let assistant_a = msgs_a
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant a");
    assert!(
        assistant_a.content.contains('A'),
        "thread A stream should contain A chunks: {}",
        assistant_a.content
    );

    shell.select_thread(&thread_b).expect("back to b");
    shell.poll_streaming();
    let msgs_b = shell.state().messages.as_ref().expect("msgs b");
    let assistant_b = msgs_b
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant b");
    assert!(
        assistant_b.content.contains('B'),
        "thread B stream should contain B chunks: {}",
        assistant_b.content
    );
    assert!(
        !assistant_b.content.contains('A'),
        "thread B must not receive A's chunks"
    );

    *stop_a.lock().unwrap() = true;
    *stop_b.lock().unwrap() = true;
    for _ in 0..50 {
        if !shell.is_thread_generating(&thread_a) && !shell.is_thread_generating(&thread_b) {
            break;
        }
        shell.poll_streaming();
        thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn same_thread_cannot_start_second_generation_while_active() {
    let (_temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    let stop = Arc::new(Mutex::new(false));

    shell
        .begin_streaming(
            &thread_id,
            Some("first"),
            Box::new(SlowMarkerProvider {
                prefix: "X".into(),
                stop: Arc::clone(&stop),
            }),
            "model",
        )
        .expect("first");

    let err = shell
        .begin_streaming(
            &thread_id,
            Some("second"),
            Box::new(SlowMarkerProvider {
                prefix: "Y".into(),
                stop: Arc::clone(&stop),
            }),
            "model",
        )
        .expect_err("same thread must reject concurrent start");
    assert!(matches!(
        err,
        ronin_app::RoninAppError::GenerationInProgress
    ));

    *stop.lock().unwrap() = true;
    shell.cancel_streaming().ok();
}

#[test]
fn sequential_sends_should_still_succeed_on_one_thread() {
    let (_temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().expect("thread");

    struct InstantProvider;
    impl ChatProvider for InstantProvider {
        fn stream_chat(
            &self,
            _request: &ChatRequest,
        ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
            Ok(Box::new(
                vec![ChatStreamEvent::Chunk("ok".into())].into_iter(),
            ))
        }
    }

    let provider = InstantProvider;
    for i in 0..3 {
        shell
            .send_message_with_provider(&thread_id, &format!("Message {i}"), &provider, "test")
            .expect("send should succeed");
    }

    let state = shell.state();
    let msgs = state.messages.as_ref().expect("messages");
    assert_eq!(msgs.len(), 6);
}

#[test]
fn cancel_should_affect_only_currently_viewed_thread() {
    let (_temp, mut shell) = open_shell();
    let thread_a = shell.state().selected_thread_id.clone().expect("a");
    let thread_b = shell.create_new_thread().expect("b").id;
    let stop_a = Arc::new(Mutex::new(false));
    let stop_b = Arc::new(Mutex::new(false));

    shell.select_thread(&thread_a).expect("select a");
    shell
        .begin_streaming(
            &thread_a,
            Some("A"),
            Box::new(SlowMarkerProvider {
                prefix: "A".into(),
                stop: Arc::clone(&stop_a),
            }),
            "m",
        )
        .expect("start a");

    shell.select_thread(&thread_b).expect("select b");
    shell
        .begin_streaming(
            &thread_b,
            Some("B"),
            Box::new(SlowMarkerProvider {
                prefix: "B".into(),
                stop: Arc::clone(&stop_b),
            }),
            "m",
        )
        .expect("start b");

    // Cancel while viewing B — A must keep generating.
    shell.cancel_streaming().expect("cancel b");
    assert!(!shell.is_thread_generating(&thread_b));
    assert!(shell.is_thread_generating(&thread_a));

    let msgs_b = shell.state().messages.as_ref().expect("b msgs");
    let assistant_b = msgs_b
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("assistant b");
    assert_eq!(assistant_b.status, MessageStatus::Cancelled);

    shell.select_thread(&thread_a).expect("select a");
    assert!(shell.is_generation_active());
    assert_eq!(
        shell
            .state()
            .messages
            .as_ref()
            .unwrap()
            .iter()
            .find(|m| m.role == MessageRole::Assistant)
            .unwrap()
            .status,
        MessageStatus::Streaming
    );

    *stop_a.lock().unwrap() = true;
    *stop_b.lock().unwrap() = true;
    for _ in 0..50 {
        if !shell.is_thread_generating(&thread_a) {
            break;
        }
        shell.poll_streaming();
        thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn sidebar_should_report_which_threads_have_active_generations() {
    let (_temp, mut shell) = open_shell();
    let thread_a = shell.state().selected_thread_id.clone().expect("a");
    let thread_b = shell.create_new_thread().expect("b").id;
    let stop = Arc::new(Mutex::new(false));

    shell.select_thread(&thread_a).unwrap();
    shell
        .begin_streaming(
            &thread_a,
            Some("A"),
            Box::new(SlowMarkerProvider {
                prefix: "A".into(),
                stop: Arc::clone(&stop),
            }),
            "m",
        )
        .unwrap();
    shell.select_thread(&thread_b).unwrap();
    shell
        .begin_streaming(
            &thread_b,
            Some("B"),
            Box::new(SlowMarkerProvider {
                prefix: "B".into(),
                stop: Arc::clone(&stop),
            }),
            "m",
        )
        .unwrap();

    let active = shell.active_generating_thread_ids();
    assert_eq!(active.len(), 2);
    assert!(active.contains(&thread_a));
    assert!(active.contains(&thread_b));

    *stop.lock().unwrap() = true;
    shell.cancel_streaming().ok();
    shell.select_thread(&thread_a).ok();
    shell.cancel_streaming().ok();
}

#[test]
fn switching_to_generating_thread_shows_live_stream() {
    let (_temp, mut shell) = open_shell();
    let thread_a = shell.state().selected_thread_id.clone().expect("a");
    let thread_b = shell.create_new_thread().expect("b").id;
    let stop_a = Arc::new(Mutex::new(false));

    shell.select_thread(&thread_a).unwrap();
    shell
        .begin_streaming(
            &thread_a,
            Some("A"),
            Box::new(SlowMarkerProvider {
                prefix: "LIVE".into(),
                stop: Arc::clone(&stop_a),
            }),
            "m",
        )
        .unwrap();

    // Leave A generating; view B.
    shell.select_thread(&thread_b).unwrap();
    thread::sleep(Duration::from_millis(180));
    shell.poll_streaming();

    // Return to A — should show live streamed content.
    shell.select_thread(&thread_a).unwrap();
    shell.poll_streaming();
    let content = shell
        .state()
        .messages
        .as_ref()
        .unwrap()
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .unwrap()
        .content
        .clone();
    assert!(
        content.contains("LIVE"),
        "switching back should show live stream: {content}"
    );
    assert_eq!(
        shell
            .state()
            .messages
            .as_ref()
            .unwrap()
            .iter()
            .find(|m| m.role == MessageRole::Assistant)
            .unwrap()
            .status,
        MessageStatus::Streaming
    );

    *stop_a.lock().unwrap() = true;
    for _ in 0..50 {
        if !shell.is_thread_generating(&thread_a) {
            break;
        }
        shell.poll_streaming();
        thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn concurrent_generations_should_persist_independently_without_corruption() {
    let (_temp, mut shell) = open_shell();
    let thread_a = shell.state().selected_thread_id.clone().expect("a");
    let thread_b = shell.create_new_thread().expect("b").id;

    struct FinishProvider {
        text: String,
    }
    impl ChatProvider for FinishProvider {
        fn stream_chat(
            &self,
            _request: &ChatRequest,
        ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
            let text = self.text.clone();
            Ok(Box::new(
                vec![
                    ChatStreamEvent::Chunk(text.clone()),
                    ChatStreamEvent::Chunk(" done".into()),
                ]
                .into_iter(),
            ))
        }
    }

    shell.select_thread(&thread_a).unwrap();
    shell
        .begin_streaming(
            &thread_a,
            Some("prompt-a"),
            Box::new(FinishProvider {
                text: "alpha-response".into(),
            }),
            "m",
        )
        .unwrap();
    shell.select_thread(&thread_b).unwrap();
    shell
        .begin_streaming(
            &thread_b,
            Some("prompt-b"),
            Box::new(FinishProvider {
                text: "beta-response".into(),
            }),
            "m",
        )
        .unwrap();

    for _ in 0..100 {
        if !shell.is_thread_generating(&thread_a) && !shell.is_thread_generating(&thread_b) {
            break;
        }
        shell.poll_streaming();
        thread::sleep(Duration::from_millis(10));
    }

    let msgs_a = shell.session().list_messages(&thread_a).expect("a");
    let msgs_b = shell.session().list_messages(&thread_b).expect("b");
    let a = msgs_a
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("a asst");
    let b = msgs_b
        .iter()
        .find(|m| m.role == MessageRole::Assistant)
        .expect("b asst");

    assert_eq!(a.status, MessageStatus::Complete);
    assert_eq!(b.status, MessageStatus::Complete);
    assert!(a.content.contains("alpha-response"), "{}", a.content);
    assert!(b.content.contains("beta-response"), "{}", b.content);
    assert!(!a.content.contains("beta"), "A must not contain B content");
    assert!(!b.content.contains("alpha"), "B must not contain A content");
    // clone_session must not have repaired live streams to Failed.
    assert!(a.error_message.is_none());
    assert!(b.error_message.is_none());
}
