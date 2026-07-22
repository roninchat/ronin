//! User-triggered one-shot lexical workspace index (#73).
//!
//! Public seams: `RoninSession` build/rebuild/cancel/delete/status,
//! `collect_workspace_index_documents`, storage under data_dir/workspace_indexes.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use ronin_core::{
    collect_workspace_index_documents, may_inject_into_chat_request, workspace_index_storage_path,
    ContextOrigin, FolderListPolicy, MessageRole, RoninPaths, RoninSession, WorkspaceIndexCaps,
    WorkspaceIndexPhase,
};
use ronin_db::WorkspaceLexicalStore;
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open session")
}

fn make_project(temp: &TempDir, name: &str) -> std::path::PathBuf {
    let root = temp.path().join(name);
    std::fs::create_dir_all(root.join("src")).expect("mkdir");
    std::fs::write(root.join("src/main.rs"), "fn main() { /* alpha */ }\n").unwrap();
    std::fs::write(root.join("README.md"), "# project beta\n").unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn helper() {}\n").unwrap();
    root
}

#[test]
fn new_thread_has_absent_workspace_index() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    let info = session.workspace_index_info(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Absent);
    assert_eq!(info.entry_count, 0);
}

#[test]
fn session_open_does_not_start_indexing() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();

    // Re-open: must not create index store or flip phase.
    let session2 = open_session(&temp);
    let info = session2.workspace_index_info(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Absent);
    assert!(!session2
        .workspace_index_storage_path_for(&thread.id)
        .exists());
}

#[test]
fn build_requires_workspace_root() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    let err = session.build_workspace_index(&thread.id).unwrap_err();
    assert!(err.to_string().contains("workspace root"));
}

#[test]
fn build_indexes_workspace_files_and_reports_done() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();

    let info = session.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    assert!(info.entry_count >= 3);
    assert!(info.byte_count > 0);
    assert!(!info.truncated);
    assert!(info.storage_path.as_ref().unwrap().exists());
    assert!(info
        .storage_path
        .unwrap()
        .starts_with(temp.path().join("data")));

    let store =
        WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id)).unwrap();
    assert!(store.contains_path("src/main.rs").unwrap());
    assert!(store.contains_path("README.md").unwrap());
}

#[test]
fn rebuild_replaces_prior_corpus() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session.build_workspace_index(&thread.id).unwrap();

    std::fs::write(project.join("new_file.txt"), "fresh omega content\n").unwrap();
    let info = session.rebuild_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    let store =
        WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id)).unwrap();
    assert!(store.contains_path("new_file.txt").unwrap());
}

#[test]
fn delete_removes_metadata_and_store_file() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session.build_workspace_index(&thread.id).unwrap();
    let path = session.workspace_index_storage_path_for(&thread.id);
    assert!(path.exists());

    session.delete_workspace_index(&thread.id).unwrap();
    assert!(!path.exists());
    assert_eq!(
        session.workspace_index_info(&thread.id).unwrap().phase,
        WorkspaceIndexPhase::Absent
    );
}

#[test]
fn cancel_flag_aborts_build_as_cancelled() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();

    let cancel = Arc::new(AtomicBool::new(true));
    let info = session
        .build_workspace_index_cancellable(
            &thread.id,
            &WorkspaceIndexCaps::default(),
            Arc::clone(&cancel),
        )
        .unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Cancelled);
    assert!(info.truncated);
}

#[test]
fn collect_cancel_stops_mid_walk() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("tree");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..40 {
        std::fs::write(root.join(format!("f{i:02}.txt")), format!("body {i}")).unwrap();
    }
    let cancel = AtomicBool::new(true);
    let result = collect_workspace_index_documents(
        &root,
        &FolderListPolicy::default(),
        &WorkspaceIndexCaps::default(),
        &cancel,
    );
    assert!(result.cancelled);
    assert!(result.documents.len() < 40);
}

#[test]
fn collect_honors_gitignore_and_deny() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("priv");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
    std::fs::create_dir_all(root.join(".git")).unwrap();
    std::fs::write(root.join(".gitignore"), "node_modules/\n").unwrap();
    std::fs::write(root.join("src/ok.rs"), "ok\n").unwrap();
    std::fs::write(root.join("node_modules/pkg/x.js"), "secret\n").unwrap();
    std::fs::write(root.join(".git/config"), "git\n").unwrap();
    std::fs::write(root.join("blob.so"), "binaryish\n").unwrap();

    let result = collect_workspace_index_documents(
        &root,
        &FolderListPolicy::default(),
        &WorkspaceIndexCaps::default(),
        &AtomicBool::new(false),
    );
    let paths: Vec<_> = result
        .documents
        .iter()
        .map(|d| d.relative_path.as_str())
        .collect();
    assert!(paths.contains(&"src/ok.rs"));
    assert!(!paths.iter().any(|p| p.contains("node_modules")));
    assert!(!paths.iter().any(|p| p.contains(".git")));
    assert!(!paths.iter().any(|p| p.ends_with(".so")));
}

#[test]
fn collect_respects_entry_and_byte_caps_with_truncation() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("big");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..30 {
        std::fs::write(root.join(format!("a{i:02}.txt")), "xxxxxxxx").unwrap();
    }
    let caps = WorkspaceIndexCaps {
        max_entries: 5,
        max_bytes: WORKSPACE_INDEX_MAX_BYTES_LOCAL,
        max_depth: 4,
        max_file_bytes: 1_048_576,
        max_duration: Duration::from_secs(30),
    };
    let result = collect_workspace_index_documents(
        &root,
        &FolderListPolicy::default(),
        &caps,
        &AtomicBool::new(false),
    );
    assert!(result.truncated);
    assert_eq!(result.documents.len(), 5);
}

const WORKSPACE_INDEX_MAX_BYTES_LOCAL: u64 = 32 * 1024 * 1024;

#[test]
fn build_never_injects_corpus_into_chat_messages() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session
        .create_message(&thread.id, MessageRole::User, "hello before index")
        .unwrap();

    session.build_workspace_index(&thread.id).unwrap();

    let messages = session.list_messages(&thread.id).unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "hello before index");
    assert!(!messages[0].content.contains("alpha"));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::WorkspaceIndexCorpus
    ));
}

#[test]
fn storage_path_is_under_session_data_dir() {
    let temp = TempDir::new().unwrap();
    let data = temp.path().join("data");
    let path = workspace_index_storage_path(&data, "thread-xyz");
    assert_eq!(path, data.join("workspace_indexes").join("thread-xyz.db"));
}

#[test]
fn build_honors_never_list_via_session_policy() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let secret = project.join("secret_dir");
    std::fs::create_dir_all(&secret).unwrap();
    std::fs::write(secret.join("passwords.txt"), "hunter2\n").unwrap();

    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session.add_never_list_path(&secret).unwrap();

    let info = session.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    let store =
        WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id)).unwrap();
    assert!(!store.contains_path("secret_dir/passwords.txt").unwrap());
    assert!(store.contains_path("README.md").unwrap());
}

#[test]
fn build_with_entry_cap_marks_truncated() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    for i in 0..20 {
        std::fs::write(project.join(format!("extra{i}.txt")), "x").unwrap();
    }
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    let caps = WorkspaceIndexCaps {
        max_entries: 3,
        ..WorkspaceIndexCaps::default()
    };
    let info = session
        .build_workspace_index_with_caps(&thread.id, &caps)
        .unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    assert!(info.truncated);
    assert_eq!(info.entry_count, 3);
}

#[test]
fn cancel_preserves_prior_done_corpus_on_disk() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session.build_workspace_index(&thread.id).unwrap();
    let store_path = session.workspace_index_storage_path_for(&thread.id);
    let store = WorkspaceLexicalStore::open(&store_path).unwrap();
    assert!(store.contains_path("README.md").unwrap());

    let cancel = Arc::new(AtomicBool::new(true));
    let info = session
        .build_workspace_index_cancellable(&thread.id, &WorkspaceIndexCaps::default(), cancel)
        .unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Cancelled);
    let store = WorkspaceLexicalStore::open(&store_path).unwrap();
    assert!(
        store.contains_path("README.md").unwrap(),
        "prior Done corpus must survive cancel"
    );
}

#[test]
fn index_status_round_trips_across_reopen() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "persist");
    {
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &project)
            .unwrap();
        session.build_workspace_index(&thread.id).unwrap();
        std::fs::write(temp.path().join("tid"), &thread.id).unwrap();
    }
    let tid = std::fs::read_to_string(temp.path().join("tid")).unwrap();
    let session = open_session(&temp);
    let info = session.workspace_index_info(tid.trim()).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    assert!(info.entry_count >= 3);
}
