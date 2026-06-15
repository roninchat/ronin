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

#[test]
fn list_all_artifacts_should_return_artifacts_across_threads_newest_first() {
    let (db, _temp) = open_test_db();

    let thread1 = db.create_thread().expect("create thread 1");
    let msg1 = db
        .create_message(&thread1.id, "user", "hello", "complete")
        .expect("create message 1");

    let thread2 = db.create_thread().expect("create thread 2");
    let msg2 = db
        .create_message(&thread2.id, "user", "world", "complete")
        .expect("create message 2");

    // Create artifacts — second one is older
    let art1 = db
        .create_artifact(&thread1.id, &msg1.id, "Artifact from thread 1", "content 1")
        .expect("create artifact 1");
    let art2 = db
        .create_artifact(&thread2.id, &msg2.id, "Artifact from thread 2", "content 2")
        .expect("create artifact 2");

    let all = db.list_all_artifacts().expect("list all artifacts");
    assert_eq!(all.len(), 2, "should return artifacts from both threads");
    // newest first (art2 was created after art1)
    assert_eq!(all[0].id, art2.id);
    assert_eq!(all[1].id, art1.id);
}
