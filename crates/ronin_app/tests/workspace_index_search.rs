//! Shell seam: lexical search + attach gate (#74).

use ronin_app::RoninShell;
use ronin_core::{
    drafts_for_workspace_index_include, may_inject_into_chat_request, ContextOrigin, RoninPaths,
    WorkspaceIndexIncludeGate, WorkspaceIndexPhase,
};
use tempfile::TempDir;

fn open_shell(temp: &TempDir) -> RoninShell {
    RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open shell")
}

fn seed_and_index(temp: &TempDir) -> (RoninShell, String) {
    let root = temp.path().join("ws");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src/main.rs"), "fn main() { /* zeta */ }\n").unwrap();
    std::fs::write(root.join("NOTES.md"), "zeta notes\n").unwrap();
    let mut shell = open_shell(temp);
    let thread = shell.create_new_thread().unwrap();
    shell.set_thread_workspace_root(&thread.id, &root).unwrap();
    let info = shell.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    (shell, thread.id)
}

#[test]
fn shell_search_returns_hits() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = seed_and_index(&temp);
    let hits = shell.search_workspace_index(&tid, "zeta").unwrap();
    assert!(!hits.is_empty());
    assert!(hits.iter().any(|h| !h.snippet.is_empty()));
}

#[test]
fn shell_search_does_not_auto_attach() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = seed_and_index(&temp);
    let hits = shell.search_workspace_index(&tid, "zeta").unwrap();
    assert!(!hits.is_empty());
    for hit in &hits {
        assert!(!may_inject_into_chat_request(hit.context_origin()));
    }
    assert!(shell.session().list_messages(&tid).unwrap().is_empty());
}

#[test]
fn shell_attach_selected_paths() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = seed_and_index(&temp);
    let hits = shell.search_workspace_index(&tid, "zeta").unwrap();
    let paths: Vec<String> = hits
        .iter()
        .take(1)
        .map(|h| h.relative_path.clone())
        .collect();
    let drafts = shell.attach_workspace_index_hits(&tid, &paths).unwrap();
    assert_eq!(drafts.len(), 1);
    assert!(drafts[0].context_block.contains("Attached workspace file"));
}

#[test]
fn shell_attach_empty_is_noop() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = seed_and_index(&temp);
    assert!(shell
        .attach_workspace_index_hits(&tid, &[] as &[&str])
        .unwrap()
        .is_empty());
}

#[test]
fn shell_search_before_index_errors() {
    let temp = TempDir::new().unwrap();
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    assert!(shell.search_workspace_index(&thread.id, "x").is_err());
}

#[test]
fn shell_include_gate_off_by_default() {
    let gate = WorkspaceIndexIncludeGate::default();
    assert!(!gate.is_enabled());
    assert_eq!(gate.context_origin(), ContextOrigin::IndexSearchHit);
    let temp = TempDir::new().unwrap();
    let (shell, tid) = seed_and_index(&temp);
    let hits = shell.search_workspace_index(&tid, "zeta").unwrap();
    let paths: Vec<_> = hits.iter().map(|h| h.relative_path.clone()).collect();
    let drafts = shell.attach_workspace_index_hits(&tid, &paths).unwrap();
    assert!(drafts_for_workspace_index_include(&gate, &drafts).is_empty());
}
