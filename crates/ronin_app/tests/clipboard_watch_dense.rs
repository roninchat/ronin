//! Dense shell clipboard-watch lifecycle (#77).

use ronin_app::RoninShell;
use ronin_core::{ClipboardObserveOutcome, RoninPaths};
use tempfile::TempDir;

fn open_shell() -> (RoninShell, TempDir) {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "").unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    (RoninShell::open(paths).unwrap(), temp)
}

#[test]
fn dense_shell_enable_confirm_disable_cycles() {
    for i in 0..60usize {
        let (mut shell, _t) = open_shell();
        assert!(!shell.clipboard_watch_enabled());
        shell.set_clipboard_watch_enabled(true, Some("s")).unwrap();
        let text = format!("dense-shell-{i}");
        assert_eq!(
            shell.observe_clipboard_text(&text),
            ClipboardObserveOutcome::Proposed
        );
        if i % 2 == 0 {
            let d = shell.confirm_clipboard_attach_proposal().unwrap();
            assert_eq!(d.content.as_deref(), Some(text.as_str()));
        } else {
            shell.dismiss_clipboard_attach_proposal();
        }
        shell.set_clipboard_watch_enabled(false, None).unwrap();
        assert!(!shell.clipboard_watch_enabled());
        assert!(shell.pending_clipboard_attach_proposal().is_none());
    }
}

#[test]
fn dense_shell_disabled_observe_matrix() {
    for i in 0..40usize {
        let (mut shell, _t) = open_shell();
        assert!(!shell.clipboard_watch_enabled());
        assert_eq!(
            shell.observe_clipboard_text(&format!("dense-off-{i}")),
            ClipboardObserveOutcome::IgnoredDisabled
        );
        assert!(shell.pending_clipboard_attach_proposal().is_none());
    }
}
