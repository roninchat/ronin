//! Shell queues desktop notifications on generation done/fail (#75).

use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    may_inject_into_chat_request, notification_payload_origin, ChatProvider, ChatRequest,
    ChatStreamEvent, GenerationNotifyKind, NotificationsConfig, RoninConfig, RoninPaths,
    FOCUS_THREAD_ACTION,
};
use tempfile::TempDir;

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

struct FailingProvider;

impl ChatProvider for FailingProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(std::iter::once(ChatStreamEvent::Error(
            "provider exploded api_key=should-not-leak".into(),
        ))))
    }
}

fn setup_shell_with_toml(toml: &str) -> (RoninShell, String, TempDir) {
    let temp = TempDir::new().expect("temp");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), toml).unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    (shell, thread_id, temp)
}

fn poll_until_idle(shell: &mut RoninShell) {
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(20));
        let active = shell.poll_streaming();
        if !active && !shell.is_generation_active() {
            break;
        }
    }
}

#[test]
fn generation_complete_enqueues_desktop_notification_with_focus_action() {
    let (mut shell, thread_id, _temp) = setup_shell_with_toml("");
    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(TokenProvider {
                tokens: vec!["Hi".into()],
            }),
            "test-model",
        )
        .expect("begin");
    poll_until_idle(&mut shell);

    let pending = shell.drain_pending_desktop_notifications();
    assert_eq!(pending.len(), 1, "expected one completion notification");
    assert_eq!(pending[0].kind, GenerationNotifyKind::Completed);
    assert_eq!(pending[0].thread_id, thread_id);
    assert_eq!(
        pending[0].default_action.as_deref(),
        Some(FOCUS_THREAD_ACTION)
    );
    assert!(!may_inject_into_chat_request(notification_payload_origin()));
}

#[test]
fn generation_failure_enqueues_failed_notification_and_scrubs_secrets() {
    let (mut shell, thread_id, _temp) = setup_shell_with_toml("");
    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(FailingProvider),
            "test-model",
        )
        .expect("begin");
    poll_until_idle(&mut shell);

    let pending = shell.drain_pending_desktop_notifications();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].kind, GenerationNotifyKind::Failed);
    assert_eq!(pending[0].thread_id, thread_id);
    assert!(
        !pending[0].body.contains("should-not-leak"),
        "secret leaked in {}",
        pending[0].body
    );
}

#[test]
fn disabled_notifications_produce_no_pending_requests() {
    let (mut shell, thread_id, _temp) = setup_shell_with_toml(
        r#"
[notifications]
enabled = false
"#,
    );
    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(TokenProvider {
                tokens: vec!["Hi".into()],
            }),
            "test-model",
        )
        .expect("begin");
    poll_until_idle(&mut shell);
    assert!(shell.drain_pending_desktop_notifications().is_empty());
}

#[test]
fn drain_clears_pending_queue() {
    let (mut shell, thread_id, _temp) = setup_shell_with_toml("");
    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(TokenProvider {
                tokens: vec!["A".into()],
            }),
            "test-model",
        )
        .expect("begin");
    poll_until_idle(&mut shell);
    assert_eq!(shell.drain_pending_desktop_notifications().len(), 1);
    assert!(shell.drain_pending_desktop_notifications().is_empty());
}

#[test]
fn cancel_does_not_enqueue_completion_notification() {
    let (mut shell, thread_id, _temp) = setup_shell_with_toml("");
    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(TokenProvider {
                tokens: vec!["slow".into()],
            }),
            "test-model",
        )
        .expect("begin");
    // Cancel before polling Done.
    shell.cancel_streaming().expect("cancel");
    let _ = shell.poll_streaming();
    assert!(
        shell.drain_pending_desktop_notifications().is_empty(),
        "cancel must not notify completion"
    );
}

#[test]
fn saving_disabled_config_suppresses_subsequent_notifications() {
    let temp = TempDir::new().unwrap();
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();
    let mut shell = RoninShell::open(paths).expect("open");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    shell
        .session()
        .save_config(&RoninConfig {
            notifications: NotificationsConfig { enabled: false },
            ..RoninConfig::default()
        })
        .expect("save");

    shell
        .begin_streaming(
            &thread_id,
            Some("hello"),
            Box::new(TokenProvider {
                tokens: vec!["Hi".into()],
            }),
            "test-model",
        )
        .expect("begin");
    poll_until_idle(&mut shell);
    assert!(shell.drain_pending_desktop_notifications().is_empty());
}
