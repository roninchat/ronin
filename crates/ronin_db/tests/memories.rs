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

    // 4. Delete
    db.delete_memory(&memory.id).expect("delete memory");
    let fetched_after_delete = db.get_memory(&memory.id).expect("get memory after delete");
    assert!(fetched_after_delete.is_none());
}
