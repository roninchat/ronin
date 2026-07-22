//! Shell clipboard watch enable/confirm/dismiss/disable (#77).

use ronin_app::RoninShell;
use ronin_core::{
    clipboard_watch_proposal_may_inject_into_chat_request, may_inject_into_chat_request,
    ClipboardObserveOutcome, ContextOrigin, RoninPaths, ScriptedClipboardSource,
};
use tempfile::TempDir;

fn open_shell(toml: &str) -> (RoninShell, TempDir) {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), toml).unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    (RoninShell::open(paths).unwrap(), temp)
}

#[test]
fn shell_watcher_disabled_by_default() {
    let (shell, _temp) = open_shell("");
    assert!(!shell.clipboard_watch_enabled());
    assert!(shell.pending_clipboard_attach_proposal().is_none());
}

#[test]
fn shell_enable_observe_confirm_never_auto_injects() {
    let (mut shell, _temp) = open_shell("");
    shell
        .set_clipboard_watch_enabled(true, Some("seed"))
        .unwrap();
    assert!(shell.clipboard_watch_enabled());
    assert_eq!(
        shell.observe_clipboard_text("seed"),
        ClipboardObserveOutcome::Unchanged
    );
    assert_eq!(
        shell.observe_clipboard_text("fresh"),
        ClipboardObserveOutcome::Proposed
    );
    assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        ContextOrigin::ClipboardWatchProposal
    ));
    let draft = shell.confirm_clipboard_attach_proposal().expect("draft");
    assert_eq!(draft.content.as_deref(), Some("fresh"));
    assert!(may_inject_into_chat_request(
        ContextOrigin::ConfirmToAttachAccepted
    ));
    assert!(shell.pending_clipboard_attach_proposal().is_none());
}

#[test]
fn shell_dismiss_and_disable_clear_proposals() {
    let (mut shell, _temp) = open_shell("");
    shell.set_clipboard_watch_enabled(true, Some("a")).unwrap();
    shell.observe_clipboard_text("b");
    assert!(shell.pending_clipboard_attach_proposal().is_some());
    shell.dismiss_clipboard_attach_proposal();
    assert!(shell.pending_clipboard_attach_proposal().is_none());

    shell.observe_clipboard_text("c");
    assert!(shell.pending_clipboard_attach_proposal().is_some());
    shell.set_clipboard_watch_enabled(false, None).unwrap();
    assert!(!shell.clipboard_watch_enabled());
    assert!(shell.pending_clipboard_attach_proposal().is_none());
    assert_eq!(
        shell.observe_clipboard_text("d"),
        ClipboardObserveOutcome::IgnoredDisabled
    );
}

#[test]
fn shell_persists_enabled_flag() {
    let (mut shell, temp) = open_shell("");
    shell.set_clipboard_watch_enabled(true, Some("x")).unwrap();
    drop(shell);
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let reopened = RoninShell::open(paths).unwrap();
    assert!(reopened.clipboard_watch_enabled());
}

#[test]
fn shell_poll_clipboard_watch_with_scripted_source() {
    let (mut shell, _temp) = open_shell("");
    let source = ScriptedClipboardSource::new();
    source.push_texts(["one", "two"]);
    assert_eq!(
        shell.poll_clipboard_watch(&source).unwrap(),
        ClipboardObserveOutcome::IgnoredDisabled
    );
    shell.set_clipboard_watch_enabled(true, None).unwrap();
    assert_eq!(
        shell.poll_clipboard_watch(&source).unwrap(),
        ClipboardObserveOutcome::Unchanged
    );
    assert_eq!(
        shell.poll_clipboard_watch(&source).unwrap(),
        ClipboardObserveOutcome::Proposed
    );
    assert_eq!(
        shell
            .pending_clipboard_attach_proposal()
            .map(|p| p.text.as_str()),
        Some("two")
    );
}

#[test]
fn shell_lifecycle_matrix_dense() {
    for i in 0..40usize {
        let (mut shell, _temp) = open_shell("");
        shell
            .set_clipboard_watch_enabled(true, Some("base"))
            .unwrap();
        let text = format!("shell-payload-{i}");
        assert_eq!(
            shell.observe_clipboard_text(&text),
            ClipboardObserveOutcome::Proposed
        );
        if i % 2 == 0 {
            shell.dismiss_clipboard_attach_proposal();
        } else {
            let _ = shell.confirm_clipboard_attach_proposal();
        }
        shell.set_clipboard_watch_enabled(false, None).unwrap();
        assert!(!shell.clipboard_watch_enabled());
    }
}

#[test]
fn shell_observe_without_enable_is_ignored_matrix() {
    for i in 0..50usize {
        let (mut shell, _temp) = open_shell("");
        assert_eq!(
            shell.observe_clipboard_text(&format!("no-watch-{i}")),
            ClipboardObserveOutcome::IgnoredDisabled
        );
        assert!(shell.pending_clipboard_attach_proposal().is_none());
    }
}

#[test]
fn shell_confirm_none_when_no_proposal_matrix() {
    for _ in 0..40usize {
        let (mut shell, _temp) = open_shell("");
        assert!(shell.confirm_clipboard_attach_proposal().is_none());
        shell.dismiss_clipboard_attach_proposal();
        assert!(shell.pending_clipboard_attach_proposal().is_none());
    }
}
