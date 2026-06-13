use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn update_message_status_should_set_status_and_error_message() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");
    let msg = db
        .create_message(&thread.id, "assistant", "partial content", "streaming")
        .expect("create message");

    db.update_message_status(
        &msg.id,
        "failed",
        Some("Generation interrupted because Ronin exited before the response completed."),
    )
    .expect("update status");

    let messages = db
        .list_messages_for_thread(&thread.id)
        .expect("list messages");
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].status, "failed");
    assert_eq!(
        messages[0].content, "partial content",
        "content should not change"
    );
    assert_eq!(
        messages[0].error_message.as_deref(),
        Some("Generation interrupted because Ronin exited before the response completed.")
    );
}

#[test]
fn update_message_content_and_status_should_set_all_fields() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");
    let msg = db
        .create_message(&thread.id, "assistant", "", "streaming")
        .expect("create message");

    db.update_message_content_and_status(&msg.id, "partial output", "cancelled", None)
        .expect("update content and status");

    let messages = db
        .list_messages_for_thread(&thread.id)
        .expect("list messages");
    assert_eq!(messages[0].content, "partial output");
    assert_eq!(messages[0].status, "cancelled");
    assert_eq!(messages[0].error_message, None);
}

#[test]
fn find_stale_streaming_messages_should_return_only_streaming_status() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");

    // Create messages with various statuses
    db.create_message(&thread.id, "user", "hello", "complete")
        .expect("create user msg");
    let streaming1 = db
        .create_message(&thread.id, "assistant", "partial", "streaming")
        .expect("create streaming msg 1");
    db.create_message(&thread.id, "assistant", "done", "complete")
        .expect("create complete msg");

    // Create another thread with a streaming message
    let thread2 = db.create_thread().expect("create thread 2");
    let streaming2 = db
        .create_message(&thread2.id, "assistant", "also partial", "streaming")
        .expect("create streaming msg 2");

    let stale = db.find_stale_streaming_messages().expect("find stale");
    assert_eq!(stale.len(), 2);

    let stale_ids: Vec<&str> = stale.iter().map(|m| m.id.as_str()).collect();
    assert!(stale_ids.contains(&streaming1.id.as_str()));
    assert!(stale_ids.contains(&streaming2.id.as_str()));
}
