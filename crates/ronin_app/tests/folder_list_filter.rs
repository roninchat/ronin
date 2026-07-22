//! Shell seam: folder_list_policy reads never-list / allowlist from session (#71).

use ronin_app::RoninShell;
use ronin_core::{
    list_folder_entries_with_policy, ContextToolError, FolderBlockReason, RoninPaths,
};
use tempfile::TempDir;

fn open_shell(temp: &TempDir) -> RoninShell {
    RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open shell")
}

#[test]
fn shell_folder_list_policy_reflects_session_never_list() {
    let temp = TempDir::new().unwrap();
    let shell = open_shell(&temp);
    let denied = temp.path().join("private");
    std::fs::create_dir_all(&denied).unwrap();
    std::fs::write(denied.join("x.txt"), "x").unwrap();
    shell.session().add_never_list_path(&denied).unwrap();
    let policy = shell.folder_list_policy().unwrap();
    let err = list_folder_entries_with_policy(&denied, None, temp.path(), &policy).unwrap_err();
    assert!(matches!(
        err,
        ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NeverList,
            ..
        }
    ));
}

#[test]
fn shell_folder_list_policy_reflects_allowlist_mode() {
    let temp = TempDir::new().unwrap();
    let shell = open_shell(&temp);
    let root = temp.path().join("ok");
    let other = temp.path().join("other");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::create_dir_all(&other).unwrap();
    shell.session().set_folder_allowlist_enabled(true).unwrap();
    shell.session().add_folder_allowlist_root(&root).unwrap();
    let policy = shell.folder_list_policy().unwrap();
    assert!(policy.allowlist_enabled);
    assert!(list_folder_entries_with_policy(&root, None, temp.path(), &policy).is_ok());
    assert!(matches!(
        list_folder_entries_with_policy(&other, None, temp.path(), &policy),
        Err(ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NotAllowlisted,
            ..
        })
    ));
}
