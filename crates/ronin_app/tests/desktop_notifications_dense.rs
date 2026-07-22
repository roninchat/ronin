//! Dense shell notification enqueue cases (#75).

use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    may_inject_into_chat_request, notification_payload_origin, ChatProvider, ChatRequest,
    ChatStreamEvent, GenerationNotifyKind, RoninPaths, FOCUS_THREAD_ACTION,
};
use tempfile::TempDir;

struct OkProvider;

impl ChatProvider for OkProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(std::iter::once(ChatStreamEvent::Chunk(
            "ok".into(),
        ))))
    }
}

struct ErrProvider {
    message: String,
}

impl ChatProvider for ErrProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(std::iter::once(ChatStreamEvent::Error(
            self.message.clone(),
        ))))
    }
}

fn open_shell(toml: &str) -> (RoninShell, String, TempDir) {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), toml).unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).unwrap();
    let id = shell.state().selected_thread_id.clone().unwrap();
    (shell, id, temp)
}

fn wait_idle(shell: &mut RoninShell) {
    for _ in 0..60 {
        thread::sleep(Duration::from_millis(15));
        let active = shell.poll_streaming();
        if !active && !shell.is_generation_active() {
            return;
        }
    }
}

#[test]
fn dense_completed_notifications_for_many_prompts() {
    let prompts = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    for (i, prompt) in prompts.iter().enumerate() {
        let (mut shell, thread_id, _temp) = open_shell("");
        shell
            .begin_streaming(thread_id.as_str(), Some(prompt), Box::new(OkProvider), "m")
            .unwrap();
        wait_idle(&mut shell);
        let pending = shell.drain_pending_desktop_notifications();
        assert_eq!(pending.len(), 1, "i={i} prompt={prompt}");
        assert_eq!(pending[0].kind, GenerationNotifyKind::Completed);
        assert_eq!(
            pending[0].default_action.as_deref(),
            Some(FOCUS_THREAD_ACTION)
        );
        assert!(!may_inject_into_chat_request(notification_payload_origin()));
    }
}

#[test]
fn dense_failed_notifications_scrub_varied_provider_errors() {
    let errors = [
        "api_key=shell-secret-a",
        "token=shell-secret-b",
        "password=shell-secret-c",
        "Bearer sk-shell-d",
        "key=shell-secret-e",
        "secret=shell-secret-f",
        "access_token=shell-secret-g",
        "sk-shell-secret-h",
        "api_key=\"shell-secret-i\"",
        "token='shell-secret-j'",
        "plain failure without secrets",
        "timeout waiting for upstream",
        "connection reset by peer",
        "model not found",
        "rate limited",
    ];
    for (i, err) in errors.iter().enumerate() {
        let (mut shell, thread_id, _temp) = open_shell("");
        shell
            .begin_streaming(
                thread_id.as_str(),
                Some("hi"),
                Box::new(ErrProvider {
                    message: (*err).into(),
                }),
                "m",
            )
            .unwrap();
        wait_idle(&mut shell);
        let pending = shell.drain_pending_desktop_notifications();
        assert_eq!(pending.len(), 1, "i={i}");
        assert_eq!(pending[0].kind, GenerationNotifyKind::Failed);
        for leak in [
            "shell-secret-a",
            "shell-secret-b",
            "shell-secret-c",
            "sk-shell-d",
            "shell-secret-e",
            "shell-secret-f",
            "shell-secret-g",
            "shell-secret-h",
            "shell-secret-i",
            "shell-secret-j",
        ] {
            if err.contains(leak) || err.contains(&leak.replace("sk-", "")) {
                assert!(
                    !pending[0].body.contains(leak),
                    "leak {leak} in {}",
                    pending[0].body
                );
            }
        }
    }
}

#[test]
fn dense_disabled_config_suppresses_varied_completions() {
    for i in 0..12 {
        let (mut shell, thread_id, _temp) = open_shell(
            r#"
[notifications]
enabled = false
"#,
        );
        shell
            .begin_streaming(
                thread_id.as_str(),
                Some(&format!("prompt-{i}")),
                Box::new(OkProvider),
                "m",
            )
            .unwrap();
        wait_idle(&mut shell);
        assert!(
            shell.drain_pending_desktop_notifications().is_empty(),
            "i={i}"
        );
    }
}

#[test]
fn dense_failed_also_suppressed_when_disabled() {
    for i in 0..10 {
        let (mut shell, thread_id, _temp) = open_shell(
            r#"
[notifications]
enabled = false
"#,
        );
        shell
            .begin_streaming(
                thread_id.as_str(),
                Some("x"),
                Box::new(ErrProvider {
                    message: format!("err-{i}"),
                }),
                "m",
            )
            .unwrap();
        wait_idle(&mut shell);
        assert!(shell.drain_pending_desktop_notifications().is_empty());
    }
}
