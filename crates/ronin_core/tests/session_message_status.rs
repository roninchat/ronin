use ronin_core::{MessageRole, MessageStatus, RoninPaths, RoninSession};
use tempfile::TempDir;

fn setup_session() -> (RoninSession, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    (session, temp)
}

#[test]
fn cancel_message_should_set_status_to_cancelled_with_partial_content() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().expect("create thread");
    let msg = session
        .create_streaming_message(&thread.id, "initial")
        .expect("create message");

    session
        .cancel_message(&msg.id, "partial output")
        .expect("cancel message");

    let msgs = session.list_messages(&thread.id).expect("list messages");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].content, "partial output");
    assert_eq!(msgs[0].status, MessageStatus::Cancelled);
    assert_eq!(msgs[0].error_message, None);
}

#[test]
fn fail_message_should_set_status_to_failed_with_error_message() {
    let (session, _temp) = setup_session();
    let thread = session.create_thread().expect("create thread");
    let msg = session
        .create_streaming_message(&thread.id, "initial")
        .expect("create message");

    session
        .fail_message(&msg.id, "partial", "an error occurred")
        .expect("fail message");

    let msgs = session.list_messages(&thread.id).expect("list messages");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].content, "partial");
    assert_eq!(msgs[0].status, MessageStatus::Failed);
    assert_eq!(msgs[0].error_message.as_deref(), Some("an error occurred"));
}

#[test]
fn repair_stale_streaming_should_mark_streaming_messages_as_failed_on_startup() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    // First session creates some messages
    {
        let session = RoninSession::open(paths.clone()).expect("open session");
        let thread = session.create_thread().expect("create thread");

        session
            .create_message(&thread.id, MessageRole::User, "hi")
            .expect("user msg");
        session
            .create_streaming_message(&thread.id, "interrupted")
            .expect("streaming msg");
    }

    // Reopen session - this should trigger repair
    let session = RoninSession::open(paths).expect("reopen session");

    let threads = session.list_threads().expect("list threads");
    let msgs = session
        .list_messages(&threads[0].id)
        .expect("list messages");

    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[1].content, "interrupted");
    assert_eq!(msgs[1].status, MessageStatus::Failed);
    assert_eq!(
        msgs[1].error_message.as_deref(),
        Some("Generation interrupted because Ronin exited before the response completed.")
    );
}
