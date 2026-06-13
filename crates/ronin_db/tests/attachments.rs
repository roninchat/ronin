use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn attachment_crud_round_trip() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");
    let msg = db.create_message(&thread.id, "user", "create attachment", "complete").expect("create message");

    // 1. Create file attachment
    let file_att = db.create_attachment(
        &msg.id,
        "file",
        "main.rs",
        "text/rust",
        None,
        Some("/path/to/main.rs")
    ).expect("create file attachment");

    assert_eq!(file_att.message_id, msg.id);
    assert_eq!(file_att.kind, "file");
    assert_eq!(file_att.name, "main.rs");
    assert_eq!(file_att.mime_type, "text/rust");
    assert_eq!(file_att.content, None);
    assert_eq!(file_att.path.as_deref(), Some("/path/to/main.rs"));

    // Create clipboard attachment
    let clip_att = db.create_attachment(
        &msg.id,
        "clipboard",
        "pasted_code",
        "text/plain",
        Some("fn main() {}"),
        None
    ).expect("create clipboard attachment");

    // 2. Get by ID
    let fetched = db.get_attachment(&file_att.id).expect("get attachment").expect("attachment should exist");
    assert_eq!(fetched, file_att);

    // 3. List by message
    let attachments = db.list_attachments_for_message(&msg.id).expect("list attachments");
    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0], file_att);
    assert_eq!(attachments[1], clip_att);

    // 4. Delete
    db.delete_attachment(&file_att.id).expect("delete attachment");
    let fetched_after_delete = db.get_attachment(&file_att.id).expect("get attachment after delete");
    assert!(fetched_after_delete.is_none());
}
