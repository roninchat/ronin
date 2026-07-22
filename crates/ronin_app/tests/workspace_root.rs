//! Shell seam: set/clear/show thread workspace root (#70).

use ronin_app::RoninShell;
use ronin_core::RoninPaths;
use tempfile::TempDir;

fn open_shell(temp: &TempDir) -> RoninShell {
    RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open shell")
}

#[test]
fn shell_set_and_clear_workspace_root_visible_on_thread_state() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    std::fs::create_dir_all(&root).unwrap();

    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().expect("create");
    assert_eq!(
        shell
            .state()
            .threads
            .iter()
            .find(|t| t.id == thread.id)
            .and_then(|t| t.workspace_root.clone()),
        None
    );

    shell
        .set_thread_workspace_root(&thread.id, &root)
        .expect("set");
    let shown = shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread.id)
        .and_then(|t| t.workspace_root.clone());
    assert_eq!(
        shown.as_deref(),
        Some(root.canonicalize().unwrap().as_path())
    );

    shell
        .clear_thread_workspace_root(&thread.id)
        .expect("clear");
    assert!(shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread.id)
        .unwrap()
        .workspace_root
        .is_none());
}

#[test]
fn shell_create_thread_does_not_auto_bind_workspace() {
    let temp = TempDir::new().unwrap();
    let gitty = temp.path().join("repo");
    std::fs::create_dir_all(gitty.join(".git")).unwrap();
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    assert!(shell
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread.id)
        .unwrap()
        .workspace_root
        .is_none());
}

#[test]
fn shell_thread_workspace_root_helper_reads_state() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    std::fs::create_dir_all(&root).unwrap();
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    assert!(shell.thread_workspace_root(&thread.id).is_none());
    shell.set_thread_workspace_root(&thread.id, &root).unwrap();
    assert_eq!(
        shell.thread_workspace_root(&thread.id).as_deref(),
        Some(root.canonicalize().unwrap().as_path())
    );
}
