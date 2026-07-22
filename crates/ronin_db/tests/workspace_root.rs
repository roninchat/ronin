//! Persistence seam tests for thread `workspace_root` (#70).

use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn create_thread_workspace_root_is_none_by_default() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().unwrap();
    assert!(thread.workspace_root.is_none());
}

#[test]
fn update_thread_workspace_root_round_trips() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().unwrap();

    db.update_thread_workspace_root(&thread.id, Some("/home/u/proj"))
        .unwrap();

    let listed = db.list_threads().unwrap();
    assert_eq!(listed[0].workspace_root.as_deref(), Some("/home/u/proj"));
}

#[test]
fn update_thread_workspace_root_can_clear() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().unwrap();
    db.update_thread_workspace_root(&thread.id, Some("/ws"))
        .unwrap();
    db.update_thread_workspace_root(&thread.id, None).unwrap();

    assert!(db.list_threads().unwrap()[0].workspace_root.is_none());
}

#[test]
fn workspace_root_independent_per_thread_rows() {
    let (db, _temp) = open_test_db();
    let a = db.create_thread().unwrap();
    let b = db.create_thread().unwrap();
    db.update_thread_workspace_root(&a.id, Some("/a")).unwrap();
    db.update_thread_workspace_root(&b.id, Some("/b")).unwrap();

    let threads = db.list_threads().unwrap();
    let ta = threads.iter().find(|t| t.id == a.id).unwrap();
    let tb = threads.iter().find(|t| t.id == b.id).unwrap();
    assert_eq!(ta.workspace_root.as_deref(), Some("/a"));
    assert_eq!(tb.workspace_root.as_deref(), Some("/b"));
}

#[test]
fn migration_adds_workspace_root_column_on_fresh_db() {
    let (db, _temp) = open_test_db();
    // Opening applies migrations; create + update proves column exists.
    let thread = db.create_thread().unwrap();
    db.update_thread_workspace_root(&thread.id, Some("/migrated"))
        .unwrap();
    assert_eq!(
        db.list_threads().unwrap()[0].workspace_root.as_deref(),
        Some("/migrated")
    );
}
