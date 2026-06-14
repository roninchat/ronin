use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn artifact_crud_round_trip() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");
    let msg = db
        .create_message(&thread.id, "user", "create artifact", "complete")
        .expect("create message");

    // 1. Create
    let artifact = db
        .create_artifact(&thread.id, &msg.id, "My Artifact", "some content")
        .expect("create artifact");

    assert_eq!(artifact.thread_id, thread.id);
    assert_eq!(artifact.message_id, msg.id);
    assert_eq!(artifact.title, "My Artifact");
    assert_eq!(artifact.content, "some content");

    // 2. Get by ID
    let fetched = db
        .get_artifact(&artifact.id)
        .expect("get artifact")
        .expect("artifact should exist");
    assert_eq!(fetched, artifact);

    // 3. List by thread
    let artifacts = db
        .list_artifacts_for_thread(&thread.id)
        .expect("list artifacts");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0], artifact);

    // 4. Delete
    db.delete_artifact(&artifact.id).expect("delete artifact");
    let fetched_after_delete = db
        .get_artifact(&artifact.id)
        .expect("get artifact after delete");
    assert!(fetched_after_delete.is_none());
}
