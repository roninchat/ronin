//! Thread workspace root: set/clear/show + relative `@file`/`@folder` resolve (#70).
//!
//! Public seams: `RoninSession` workspace APIs, `context_path_base` /
//! `resolve_context_path`, and `read_file_attachment` / `list_folder_entries`.

use std::path::{Path, PathBuf};

use ronin_core::{
    context_path_base, list_folder_entries, read_file_attachment, resolve_context_path, RoninPaths,
    RoninSession,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open session")
}

fn make_project(temp: &TempDir, name: &str) -> PathBuf {
    let root = temp.path().join(name);
    std::fs::create_dir_all(root.join("src")).expect("mkdir src");
    std::fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("write main");
    std::fs::write(root.join("README.md"), "# project\n").expect("write readme");
    root
}

#[test]
fn new_thread_has_no_workspace_root() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    assert_eq!(thread.workspace_root, None);
}

#[test]
fn set_and_show_workspace_root_round_trips_on_thread() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();

    session
        .set_thread_workspace_root(&thread.id, &project)
        .expect("set workspace");

    let listed = session.list_threads().unwrap();
    let got = listed.iter().find(|t| t.id == thread.id).unwrap();
    assert_eq!(
        got.workspace_root.as_deref(),
        Some(project.canonicalize().unwrap().as_path())
    );
}

#[test]
fn clear_workspace_root_removes_binding() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();

    session
        .clear_thread_workspace_root(&thread.id)
        .expect("clear");

    let listed = session.list_threads().unwrap();
    assert_eq!(listed[0].workspace_root, None);
}

#[test]
fn change_workspace_root_replaces_previous() {
    let temp = TempDir::new().unwrap();
    let a = make_project(&temp, "a");
    let b = make_project(&temp, "b");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();

    session.set_thread_workspace_root(&thread.id, &a).unwrap();
    session.set_thread_workspace_root(&thread.id, &b).unwrap();

    let got = &session.list_threads().unwrap()[0];
    assert_eq!(
        got.workspace_root.as_deref(),
        Some(b.canonicalize().unwrap().as_path())
    );
}

#[test]
fn workspace_root_persists_across_session_reopen() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "persist-proj");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let thread_id = {
        let session = RoninSession::open(paths.clone()).unwrap();
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &project)
            .unwrap();
        thread.id
    };

    let reopened = RoninSession::open(paths).unwrap();
    let thread = reopened
        .list_threads()
        .unwrap()
        .into_iter()
        .find(|t| t.id == thread_id)
        .unwrap();
    assert_eq!(
        thread.workspace_root.as_deref(),
        Some(project.canonicalize().unwrap().as_path())
    );
}

#[test]
fn set_workspace_root_rejects_missing_path() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    let missing = temp.path().join("no-such-dir");

    let err = session
        .set_thread_workspace_root(&thread.id, &missing)
        .expect_err("missing path");
    assert!(
        err.to_string().contains("workspace") || err.to_string().contains("directory"),
        "unexpected error: {err}"
    );
    assert_eq!(session.list_threads().unwrap()[0].workspace_root, None);
}

#[test]
fn set_workspace_root_rejects_file_path() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("not-a-dir.txt");
    std::fs::write(&file, "x").unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();

    let err = session
        .set_thread_workspace_root(&thread.id, &file)
        .expect_err("file path");
    assert!(
        err.to_string().contains("directory") || err.to_string().contains("workspace"),
        "unexpected error: {err}"
    );
}

#[test]
fn relative_file_resolves_against_workspace_root() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let unrelated = temp.path().join("elsewhere");
    std::fs::create_dir_all(&unrelated).unwrap();

    let draft = read_file_attachment("src/main.rs", Some(project.as_path()), &unrelated)
        .expect("read via workspace");

    assert_eq!(draft.name, "main.rs");
    assert!(draft.context_block.contains("fn main()"));
    assert_eq!(
        draft.path.as_deref(),
        Some(project.join("src/main.rs").as_path())
    );
}

#[test]
fn relative_folder_resolves_against_workspace_root() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let fallback = temp.path().join("fallback-cwd");
    std::fs::create_dir_all(&fallback).unwrap();

    let listing =
        list_folder_entries("src", Some(project.as_path()), &fallback).expect("list via workspace");

    assert_eq!(listing.root, project.join("src"));
    assert!(listing.entries.iter().any(|e| e.relative_path == "main.rs"));
}

#[test]
fn absolute_file_works_without_workspace_root() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let abs = project.join("README.md");
    let fallback = temp.path().join("unused-cwd");
    std::fs::create_dir_all(&fallback).unwrap();

    let resolved = resolve_context_path(&abs, None, &fallback);
    assert_eq!(resolved, abs);

    let draft = read_file_attachment(&abs, None, &fallback).expect("absolute read");
    assert!(draft.context_block.contains("# project"));
}

#[test]
fn absolute_file_works_with_workspace_root_set() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let other = make_project(&temp, "other");
    let abs = other.join("README.md");

    let resolved = resolve_context_path(&abs, Some(&project), Path::new("/tmp"));
    assert_eq!(resolved, abs);

    let draft = read_file_attachment(&abs, Some(project.as_path()), Path::new("/tmp"))
        .expect("absolute ignores workspace");
    assert!(draft.context_block.contains("# project"));
    assert_eq!(draft.path.as_deref(), Some(abs.as_path()));
}

#[test]
fn no_silent_auto_bind_from_process_cwd() {
    let temp = TempDir::new().unwrap();
    let _project = make_project(&temp, "looks-like-cwd");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    assert_eq!(
        thread.workspace_root, None,
        "workspace must not auto-bind from CWD"
    );
}

#[test]
fn no_silent_auto_bind_from_git_root() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "gitty");
    std::fs::create_dir_all(project.join(".git")).unwrap();
    std::fs::write(project.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();

    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    assert_eq!(
        thread.workspace_root, None,
        "workspace must not auto-bind from git root"
    );
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    assert!(session.list_threads().unwrap()[0].workspace_root.is_some());
}

#[test]
fn context_path_base_prefers_workspace_over_fallback() {
    let workspace = Path::new("/home/user/project");
    let fallback = Path::new("/tmp/cwd");
    assert_eq!(context_path_base(Some(workspace), fallback), workspace);
    assert_eq!(context_path_base(None, fallback), fallback);
}

#[test]
fn resolve_context_path_joins_relative_to_workspace() {
    let workspace = PathBuf::from("/ws");
    let fallback = PathBuf::from("/cwd");
    assert_eq!(
        resolve_context_path(Path::new("src/lib.rs"), Some(&workspace), &fallback),
        PathBuf::from("/ws/src/lib.rs")
    );
}

#[test]
fn resolve_context_path_joins_relative_to_fallback_without_workspace() {
    let fallback = PathBuf::from("/cwd");
    assert_eq!(
        resolve_context_path(Path::new("notes.txt"), None, &fallback),
        PathBuf::from("/cwd/notes.txt")
    );
}

#[test]
fn resolve_context_path_keeps_absolute_with_or_without_workspace() {
    let abs = PathBuf::from("/abs/file.rs");
    let workspace = PathBuf::from("/ws");
    let fallback = PathBuf::from("/cwd");
    assert_eq!(resolve_context_path(&abs, Some(&workspace), &fallback), abs);
    assert_eq!(resolve_context_path(&abs, None, &fallback), abs);
}

#[test]
fn clearing_workspace_does_not_inject_path_into_model_context() {
    let temp = TempDir::new().unwrap();
    let project = make_project(&temp, "proj");
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &project)
        .unwrap();
    session.clear_thread_workspace_root(&thread.id).unwrap();

    let messages = session.list_messages(&thread.id).unwrap();
    assert!(
        messages.is_empty(),
        "workspace bind/clear must not create chat messages or silent context"
    );
}

#[test]
fn workspace_binding_is_per_thread() {
    let temp = TempDir::new().unwrap();
    let a = make_project(&temp, "a");
    let b = make_project(&temp, "b");
    let session = open_session(&temp);
    let t1 = session.create_thread().unwrap();
    let t2 = session.create_thread().unwrap();

    session.set_thread_workspace_root(&t1.id, &a).unwrap();
    session.set_thread_workspace_root(&t2.id, &b).unwrap();

    let threads = session.list_threads().unwrap();
    let one = threads.iter().find(|t| t.id == t1.id).unwrap();
    let two = threads.iter().find(|t| t.id == t2.id).unwrap();
    assert_eq!(
        one.workspace_root.as_deref(),
        Some(a.canonicalize().unwrap().as_path())
    );
    assert_eq!(
        two.workspace_root.as_deref(),
        Some(b.canonicalize().unwrap().as_path())
    );
}
