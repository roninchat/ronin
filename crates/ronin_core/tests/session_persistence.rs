use ronin_core::{RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn first_launch_creates_state_and_reopen_restores_created_thread() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let session = RoninSession::open(paths.clone()).expect("open session on empty paths");

    assert!(paths.config_dir.is_dir());
    assert!(paths.data_dir.is_dir());
    assert!(paths.data_dir.join("ronin.db").is_file());

    let thread = session.create_thread().expect("create thread");

    assert!(!thread.id.trim().is_empty());
    assert_eq!(thread.title, "New Chat");
    assert!(thread.created_at > 0);
    assert!(thread.updated_at >= thread.created_at);
    assert!(!thread.archived);

    drop(session);

    let reopened = RoninSession::open(paths).expect("reopen session");
    let threads = reopened.list_threads().expect("list threads");

    assert_eq!(threads, vec![thread]);
}
