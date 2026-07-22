//! Dense shell search/attach assertions (#74).

use ronin_app::RoninShell;
use ronin_core::{may_inject_into_chat_request, RoninPaths, WorkspaceIndexPhase};
use tempfile::TempDir;

fn indexed(temp: &TempDir) -> (RoninShell, String) {
    let root = temp.path().join("ws");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..20 {
        std::fs::write(
            root.join(format!("f{i}.txt")),
            format!("shellterm{i} shared\n"),
        )
        .unwrap();
    }
    let mut shell = RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap();
    let t = shell.create_new_thread().unwrap();
    shell.set_thread_workspace_root(&t.id, &root).unwrap();
    assert_eq!(
        shell.build_workspace_index(&t.id).unwrap().phase,
        WorkspaceIndexPhase::Done
    );
    (shell, t.id)
}

#[test]
fn shell_search_unique_terms() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = indexed(&temp);
    for i in 0..20 {
        let hits = shell
            .search_workspace_index(&tid, &format!("shellterm{i}"))
            .unwrap();
        assert!(!hits.is_empty());
        for h in hits {
            assert!(!may_inject_into_chat_request(h.context_origin()));
        }
    }
}

#[test]
fn shell_attach_each_file() {
    let temp = TempDir::new().unwrap();
    let (shell, tid) = indexed(&temp);
    for i in 0..20 {
        let path = format!("f{i}.txt");
        let drafts = shell
            .attach_workspace_index_hits(&tid, std::slice::from_ref(&path))
            .unwrap();
        assert_eq!(drafts.len(), 1);
        assert!(drafts[0].context_block.contains(&path));
    }
}

#[test]
fn shell_search_absent_threads_error_table() {
    let temp = TempDir::new().unwrap();
    let mut shell = RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap();
    for i in 0..40 {
        let t = shell.create_new_thread().unwrap();
        let err = shell.search_workspace_index(&t.id, "x").unwrap_err();
        assert!(err.to_string().contains("not ready"), "thread {i}");
    }
}
