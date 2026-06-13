use ronin_core::{MessageRole, RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn messages_should_be_persisted_and_restored_when_reopening_session() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let session = RoninSession::open(paths.clone()).expect("open session");
    let thread = session.create_thread().expect("create thread");

    let user_msg = session
        .create_message(&thread.id, MessageRole::User, "Hello, Ronin!")
        .expect("create user message");

    assert_eq!(user_msg.role, MessageRole::User);
    assert_eq!(user_msg.content, "Hello, Ronin!");
    assert_eq!(user_msg.thread_id, thread.id);
    assert!(!user_msg.id.trim().is_empty());
    assert!(user_msg.created_at > 0);

    let messages = session.list_messages(&thread.id).expect("list messages");
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0], user_msg);

    drop(session);

    let reopened = RoninSession::open(paths).expect("reopen session");
    let messages = reopened
        .list_messages(&thread.id)
        .expect("list messages after reopen");
    assert_eq!(messages, vec![user_msg]);
}
