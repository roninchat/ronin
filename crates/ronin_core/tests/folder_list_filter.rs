//! Ignore / deny / allow filters on folder listing walks (#71).
//!
//! Public seams: `FolderListPolicy`, `list_folder_entries` /
//! `list_folder_entries_with_policy`, and `RoninSession` never-list / allowlist APIs.

use std::path::Path;

use ronin_core::{
    folder_attachment_from_selection, list_folder_entries, list_folder_entries_with_policy,
    ContextToolError, FolderBlockReason, FolderListPolicy, RoninPaths, RoninSession,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open session")
}

fn rels(listing: &ronin_core::FolderListing) -> Vec<&str> {
    listing
        .entries
        .iter()
        .map(|e| e.relative_path.as_str())
        .collect()
}

fn write_tree(root: &Path, files: &[(&str, &str)]) {
    for (rel, body) in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, body).unwrap();
    }
}

#[test]
fn folder_listing_honors_gitignore_under_walked_root() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[
            (".gitignore", "secret.txt\nbuild/\n*.log\n"),
            ("README.md", "# hi\n"),
            ("secret.txt", "do-not-list\n"),
            ("build/out.txt", "artifact\n"),
            ("app.log", "noise\n"),
            ("src/main.rs", "fn main() {}\n"),
        ],
    );

    let listing = list_folder_entries(&root, None, temp.path()).expect("list");
    let paths = rels(&listing);
    assert!(paths.contains(&"README.md"));
    assert!(paths.contains(&"src/main.rs"));
    assert!(
        !paths
            .iter()
            .any(|p| *p == "secret.txt" || p.starts_with("build/") || p.ends_with(".log")),
        "gitignore matches must be omitted; got {paths:?}"
    );
}

#[test]
fn built_in_deny_omits_vcs_dirs_binaries_and_oversized_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[
            ("ok.txt", "fine\n"),
            (".git/HEAD", "ref: refs/heads/main\n"),
            (".git/config", "[core]\n"),
            ("lib.so", "binary-ish"),
            ("src/app.rs", "fn app() {}\n"),
        ],
    );
    // Oversized relative to attachment cap (1 MiB).
    let big = vec![b'x'; 1_048_576 + 64];
    std::fs::write(root.join("huge.bin.txt"), &big).unwrap();

    let listing = list_folder_entries(&root, None, temp.path()).expect("list");
    let paths = rels(&listing);
    assert!(paths.contains(&"ok.txt"));
    assert!(paths.contains(&"src/app.rs"));
    assert!(
        !paths
            .iter()
            .any(|p| p.starts_with(".git/") || *p == "lib.so"),
        "built-in deny must omit VCS/binaries; got {paths:?}"
    );
    assert!(
        !paths.contains(&"huge.bin.txt"),
        "oversized files must be omitted from listing"
    );
}

#[test]
fn never_list_path_blocks_listing_that_root_and_descendants() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let secrets = temp.path().join("secrets");
    let sibling = temp.path().join("ok-proj");
    write_tree(&secrets, &[("passwords.txt", "x\n")]);
    write_tree(&sibling, &[("readme.md", "ok\n")]);

    session
        .add_never_list_path(&secrets)
        .expect("mark never-list");

    let policy = session.folder_list_policy().expect("policy");
    let blocked = list_folder_entries_with_policy(&secrets, None, temp.path(), &policy);
    assert!(matches!(
        blocked,
        Err(ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NeverList,
            ..
        })
    ));

    let ok = list_folder_entries_with_policy(&sibling, None, temp.path(), &policy).expect("ok");
    assert!(rels(&ok).contains(&"readme.md"));
}

#[test]
fn never_list_omits_nested_denied_folder_during_walk() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let root = temp.path().join("proj");
    let private = root.join("private");
    write_tree(
        &root,
        &[("public.txt", "ok\n"), ("private/secret.txt", "nope\n")],
    );
    session.add_never_list_path(&private).unwrap();
    let policy = session.folder_list_policy().unwrap();

    let listing = list_folder_entries_with_policy(&root, None, temp.path(), &policy).unwrap();
    let paths = rels(&listing);
    assert!(paths.contains(&"public.txt"));
    assert!(
        !paths.iter().any(|p| p.starts_with("private/")),
        "never-list nested folder must be omitted; got {paths:?}"
    );
}

#[test]
fn allowlist_mode_restricts_eligible_roots() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let allowed = temp.path().join("allowed");
    let other = temp.path().join("other");
    write_tree(&allowed, &[("a.txt", "a\n")]);
    write_tree(&other, &[("b.txt", "b\n")]);

    session.set_folder_allowlist_enabled(true).unwrap();
    session.add_folder_allowlist_root(&allowed).unwrap();
    let policy = session.folder_list_policy().unwrap();

    let ok = list_folder_entries_with_policy(&allowed, None, temp.path(), &policy).unwrap();
    assert!(rels(&ok).contains(&"a.txt"));

    let denied = list_folder_entries_with_policy(&other, None, temp.path(), &policy);
    assert!(matches!(
        denied,
        Err(ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NotAllowlisted,
            ..
        })
    ));
}

#[test]
fn empty_selection_still_cannot_attach_whole_folder_after_filtering() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[(".gitignore", "*\n"), ("only.txt", "hidden by gitignore\n")],
    );
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    assert!(
        listing.entries.is_empty(),
        "fully ignored tree should list nothing"
    );
    let err = folder_attachment_from_selection(&listing, &[]).unwrap_err();
    assert!(matches!(err, ContextToolError::EmptyFolderSelection { .. }));
}

#[test]
fn session_never_list_and_allowlist_round_trip_in_config() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let never = temp.path().join("n");
    let allow = temp.path().join("a");
    std::fs::create_dir_all(&never).unwrap();
    std::fs::create_dir_all(&allow).unwrap();

    session.add_never_list_path(&never).unwrap();
    session.set_folder_allowlist_enabled(true).unwrap();
    session.add_folder_allowlist_root(&allow).unwrap();

    let reloaded = open_session(&temp);
    let never_paths = reloaded.list_never_list_paths().unwrap();
    assert!(never_paths.iter().any(|p| p.ends_with("n")));
    assert!(reloaded.folder_allowlist_enabled().unwrap());
    let allow_paths = reloaded.list_folder_allowlist_roots().unwrap();
    assert!(allow_paths.iter().any(|p| p.ends_with("a")));

    reloaded.remove_never_list_path(&never).unwrap();
    reloaded.remove_folder_allowlist_root(&allow).unwrap();
    reloaded.set_folder_allowlist_enabled(false).unwrap();
    assert!(reloaded.list_never_list_paths().unwrap().is_empty());
    assert!(!reloaded.folder_allowlist_enabled().unwrap());
}

#[test]
fn default_policy_does_not_require_allowlist() {
    let policy = FolderListPolicy::default();
    assert!(!policy.allowlist_enabled);
    assert!(policy.honor_gitignore);
    assert!(policy.apply_built_in_deny);
}

#[test]
fn policy_without_built_in_deny_still_lists_binary_names() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(&root, &[("lib.so", "x"), ("ok.txt", "y")]);
    let policy = FolderListPolicy {
        apply_built_in_deny: false,
        honor_gitignore: false,
        ..FolderListPolicy::default()
    };
    let listing = list_folder_entries_with_policy(&root, None, temp.path(), &policy).unwrap();
    let paths = rels(&listing);
    assert!(paths.contains(&"lib.so"));
    assert!(paths.contains(&"ok.txt"));
}
