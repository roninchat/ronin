use ronin_app::RoninShell;
use ronin_core::RoninPaths;
use tempfile::TempDir;

#[test]
fn shell_should_derive_thread_title_from_first_user_message() {
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
    assert_eq!(shell.state().threads[0].title, "New Chat", "initial title");

    shell
        .send_message(&thread_id, "Hello, Ronin! This is a test prompt.")
        .expect("send message");

    let state = shell.state();
    let thread = state
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .expect("thread exists");
    assert_eq!(thread.title, "Hello, Ronin! This is a test prompt.");

    drop(shell);

    let reopened = RoninShell::open(paths).expect("reopen shell");
    let reopened_state = reopened.state();
    let reopened_thread = reopened_state
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .expect("thread exists after reopen");
    assert_eq!(
        reopened_thread.title,
        "Hello, Ronin! This is a test prompt."
    );
}

#[test]
fn thread_title_should_collapse_whitespace_and_truncate() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let long_prompt = "  Hello,   Ronin!  This is a very long prompt that should be truncated to about sixty characters total.  ";
    shell
        .send_message(&thread_id, long_prompt)
        .expect("send message");

    let title = shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .map(|t| t.title.as_str())
        .expect("thread");

    assert!(!title.contains("  "), "collapsed double spaces: {title}");
    assert!(!title.contains('\n'), "collapsed newlines: {title}");
    assert!(
        title.len() <= 60,
        "truncated to ~60 chars, got {}: {title}",
        title.len()
    );
    assert!(
        title.starts_with("Hello, Ronin!"),
        "preserved prefix: {title}"
    );
}

#[test]
fn thread_title_should_not_change_after_first_user_message() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");
    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    shell
        .send_message(&thread_id, "First message for title")
        .expect("first send");

    let first_title = shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .map(|t| t.title.clone())
        .expect("thread");

    shell
        .send_message(&thread_id, "Second message should not change title")
        .expect("second send");

    let title_after_second = shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .map(|t| t.title.as_str())
        .expect("thread");

    assert_eq!(title_after_second, first_title.as_str());
}
