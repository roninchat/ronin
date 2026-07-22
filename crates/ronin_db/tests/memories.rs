use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn memory_crud_round_trip() {
    let (db, _temp) = open_test_db();

    // 1. Create
    let memory = db
        .create_memory("User Preference", "User prefers Rust for backend")
        .expect("create memory");

    assert_eq!(memory.title, "User Preference");
    assert_eq!(memory.content, "User prefers Rust for backend");
    assert!(memory.created_at > 0);
    assert_eq!(memory.updated_at, memory.created_at);
    assert!(memory.enabled);
    assert!(!memory.is_profile);

    // 2. Get by ID
    let fetched = db
        .get_memory(&memory.id)
        .expect("get memory")
        .expect("memory should exist");
    assert_eq!(fetched, memory);

    // 3. List all
    let memories = db.list_all_memories().expect("list memories");
    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0], memory);

    // 4. Update
    db.update_memory(&memory.id, "New Title", "New Content")
        .expect("update memory");
    let fetched_after_update = db.get_memory(&memory.id).expect("get memory").unwrap();
    assert_eq!(fetched_after_update.title, "New Title");
    assert_eq!(fetched_after_update.content, "New Content");
    assert!(fetched_after_update.updated_at >= memory.updated_at);

    // 5. Delete
    db.delete_memory(&memory.id).expect("delete memory");
    let fetched_after_delete = db.get_memory(&memory.id).expect("get memory after delete");
    assert!(fetched_after_delete.is_none());
}

#[test]
fn memory_enabled_and_profile_should_persist() {
    let (db, temp) = open_test_db();
    let memory = db
        .create_memory("Prefs", "likes tea")
        .expect("create memory");
    assert!(memory.enabled);
    assert!(!memory.is_profile);

    db.set_memory_enabled(&memory.id, false)
        .expect("disable memory");
    db.set_memory_profile(&memory.id, true)
        .expect("mark profile");

    let fetched = db.get_memory(&memory.id).expect("get").expect("exists");
    assert!(!fetched.enabled);
    assert!(fetched.is_profile);

    // Reopen DB to prove persistence across connections.
    let path = temp.path().join("test.db");
    drop(db);
    let reopened = RoninDb::open(&path).expect("reopen");
    let again = reopened
        .get_memory(&memory.id)
        .expect("get")
        .expect("exists");
    assert!(!again.enabled);
    assert!(again.is_profile);
}

#[test]
fn create_profile_memory_should_be_enabled_profile() {
    let (db, _temp) = open_test_db();
    let memory = db
        .create_memory_with_flags("Role", "Staff eng", true, true)
        .expect("create profile");
    assert!(memory.enabled);
    assert!(memory.is_profile);
}
