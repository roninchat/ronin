//! Persistence seam tests for workspace index metadata (#73).

use ronin_db::{DbWorkspaceIndexMeta, RoninDb, WorkspaceLexicalStore};
use tempfile::TempDir;

fn open_db(temp: &TempDir) -> RoninDb {
    RoninDb::open(temp.path().join("ronin.db")).unwrap()
}

#[test]
fn migration_creates_workspace_indexes_table() {
    let temp = TempDir::new().unwrap();
    let db = open_db(&temp);
    let thread = db.create_thread().unwrap();
    assert!(db.get_workspace_index_meta(&thread.id).unwrap().is_none());
}

#[test]
fn upsert_and_get_workspace_index_meta_round_trip() {
    let temp = TempDir::new().unwrap();
    let db = open_db(&temp);
    let thread = db.create_thread().unwrap();
    let meta = DbWorkspaceIndexMeta {
        thread_id: thread.id.clone(),
        phase: "done".into(),
        workspace_root: Some("/ws".into()),
        entry_count: 3,
        byte_count: 42,
        truncated: true,
        error_message: None,
        storage_relpath: Some("workspace_indexes/x.db".into()),
        built_at: Some(123),
        updated_at: 456,
    };
    db.upsert_workspace_index_meta(&meta).unwrap();
    let got = db.get_workspace_index_meta(&thread.id).unwrap().unwrap();
    assert_eq!(got.phase, "done");
    assert_eq!(got.entry_count, 3);
    assert_eq!(got.byte_count, 42);
    assert!(got.truncated);
    assert_eq!(got.workspace_root.as_deref(), Some("/ws"));
}

#[test]
fn delete_workspace_index_meta_removes_row() {
    let temp = TempDir::new().unwrap();
    let db = open_db(&temp);
    let thread = db.create_thread().unwrap();
    db.upsert_workspace_index_meta(&DbWorkspaceIndexMeta {
        thread_id: thread.id.clone(),
        phase: "running".into(),
        workspace_root: None,
        entry_count: 0,
        byte_count: 0,
        truncated: false,
        error_message: None,
        storage_relpath: None,
        built_at: None,
        updated_at: 1,
    })
    .unwrap();
    db.delete_workspace_index_meta(&thread.id).unwrap();
    assert!(db.get_workspace_index_meta(&thread.id).unwrap().is_none());
}

#[test]
fn lexical_store_replace_and_contains_path() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("idx.db");
    let store = WorkspaceLexicalStore::open(&path).unwrap();
    store
        .replace_documents(&[ronin_db::LexicalIndexDocument {
            relative_path: "a.rs".into(),
            body: "fn a() {}".into(),
            byte_len: 9,
        }])
        .unwrap();
    assert_eq!(store.entry_count().unwrap(), 1);
    assert!(store.contains_path("a.rs").unwrap());
    assert!(!store.contains_path("missing.rs").unwrap());
}

#[test]
fn lexical_store_clear_empties_corpus() {
    let temp = TempDir::new().unwrap();
    let store = WorkspaceLexicalStore::open(temp.path().join("idx.db")).unwrap();
    store
        .replace_documents(&[ronin_db::LexicalIndexDocument {
            relative_path: "a.rs".into(),
            body: "x".into(),
            byte_len: 1,
        }])
        .unwrap();
    store.clear().unwrap();
    assert_eq!(store.entry_count().unwrap(), 0);
}

#[test]
fn upsert_phases_table() {
    let phases = ["absent", "running", "done", "failed", "cancelled"];
    let temp = TempDir::new().unwrap();
    let db = open_db(&temp);
    let thread = db.create_thread().unwrap();
    for (i, phase) in phases.iter().enumerate() {
        db.upsert_workspace_index_meta(&DbWorkspaceIndexMeta {
            thread_id: thread.id.clone(),
            phase: (*phase).into(),
            workspace_root: Some(format!("/ws/{i}")),
            entry_count: i as i64,
            byte_count: (i * 10) as i64,
            truncated: i % 2 == 0,
            error_message: if *phase == "failed" {
                Some("boom".into())
            } else {
                None
            },
            storage_relpath: Some(format!("workspace_indexes/{i}.db")),
            built_at: Some(i as i64),
            updated_at: i as i64 + 100,
        })
        .unwrap();
        let got = db.get_workspace_index_meta(&thread.id).unwrap().unwrap();
        assert_eq!(got.phase, *phase);
    }
}

#[test]
fn meta_phase_labels_persist_matrix() {
    let labels: &[&str] = &[
        "absent",    // 00
        "running",   // 01
        "done",      // 02
        "failed",    // 03
        "cancelled", // 04
        "absent",    // 05
        "running",   // 06
        "done",      // 07
        "failed",    // 08
        "cancelled", // 09
        "absent",    // 10
        "running",   // 11
        "done",      // 12
        "failed",    // 13
        "cancelled", // 14
        "absent",    // 15
        "running",   // 16
        "done",      // 17
        "failed",    // 18
        "cancelled", // 19
        "absent",    // 20
        "running",   // 21
        "done",      // 22
        "failed",    // 23
        "cancelled", // 24
        "absent",    // 25
        "running",   // 26
        "done",      // 27
        "failed",    // 28
        "cancelled", // 29
        "absent",    // 30
        "running",   // 31
        "done",      // 32
        "failed",    // 33
        "cancelled", // 34
        "absent",    // 35
        "running",   // 36
        "done",      // 37
        "failed",    // 38
        "cancelled", // 39
        "absent",    // 40
        "running",   // 41
        "done",      // 42
        "failed",    // 43
        "cancelled", // 44
        "absent",    // 45
        "running",   // 46
        "done",      // 47
        "failed",    // 48
        "cancelled", // 49
        "absent",    // 50
        "running",   // 51
        "done",      // 52
        "failed",    // 53
        "cancelled", // 54
        "absent",    // 55
        "running",   // 56
        "done",      // 57
        "failed",    // 58
        "cancelled", // 59
        "absent",    // 60
        "running",   // 61
        "done",      // 62
        "failed",    // 63
        "cancelled", // 64
        "absent",    // 65
        "running",   // 66
        "done",      // 67
        "failed",    // 68
        "cancelled", // 69
        "absent",    // 70
        "running",   // 71
        "done",      // 72
        "failed",    // 73
        "cancelled", // 74
        "absent",    // 75
        "running",   // 76
        "done",      // 77
        "failed",    // 78
        "cancelled", // 79
    ];
    let temp = TempDir::new().unwrap();
    let db = open_db(&temp);
    let thread = db.create_thread().unwrap();
    for (i, phase) in labels.iter().enumerate() {
        db.upsert_workspace_index_meta(&DbWorkspaceIndexMeta {
            thread_id: thread.id.clone(),
            phase: (*phase).into(),
            workspace_root: Some(format!("/ws/{i}")),
            entry_count: i as i64,
            byte_count: (i * 3) as i64,
            truncated: i % 2 == 0,
            error_message: None,
            storage_relpath: Some(format!("workspace_indexes/{i}.db")),
            built_at: Some(i as i64),
            updated_at: 1000 + i as i64,
        })
        .unwrap();
        let got = db.get_workspace_index_meta(&thread.id).unwrap().unwrap();
        assert_eq!(&got.phase, phase, "i={i}");
        assert_eq!(got.entry_count, i as i64);
    }
}
