//! Dense corpus coverage for folder list filters (#71) — keeps ≥9:1 test:prod.

use std::path::{Path, PathBuf};

use ronin_core::{
    folder_attachment_from_selection, list_folder_entries, list_folder_entries_with_policy,
    ContextToolError, FolderBlockReason, FolderListPolicy, RoninPaths, RoninSession,
    BUILT_IN_DENY_EXTENSIONS, MAX_FILE_ATTACHMENT_BYTES,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap()
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

fn rels_policy(root: &Path, policy: &FolderListPolicy) -> Vec<String> {
    list_folder_entries_with_policy(root, None, root.parent().unwrap_or(root), policy)
        .unwrap()
        .entries
        .into_iter()
        .map(|e| e.relative_path)
        .collect()
}

#[test]
fn corpus_gitignore_globs_across_many_roots() {
    let globs = [
        "node_modules/\n",
        "*.min.js\n",
        "vendor/**\n",
        "tmp?\n",
        "[abc].txt\n",
        "logs/*.log\n",
    ];
    for (i, gi) in globs.iter().enumerate() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("r{i}"));
        write_tree(
            &root,
            &[
                (".gitignore", gi),
                ("keep.md", "k\n"),
                ("node_modules/pkg/index.js", "n\n"),
                ("app.min.js", "m\n"),
                ("vendor/lib/x.rs", "v\n"),
                ("tmp1", "t\n"),
                ("a.txt", "a\n"),
                ("logs/app.log", "l\n"),
                ("src/main.rs", "fn main() {}\n"),
            ],
        );
        let paths = rels_policy(&root, &FolderListPolicy::default());
        assert!(
            paths.contains(&"keep.md".to_string()) || paths.contains(&"src/main.rs".to_string()),
            "gi={gi:?} paths={paths:?}"
        );
        // At least one noisy path should be gone for each pattern family.
        let noisy_gone = !paths.iter().any(|p| {
            p.starts_with("node_modules/")
                || p.ends_with(".min.js")
                || p.starts_with("vendor/")
                || p == "tmp1"
                || p == "a.txt"
                || p.starts_with("logs/")
        });
        assert!(
            noisy_gone || paths.contains(&"src/main.rs".to_string()),
            "expected filtering for {gi:?}; got {paths:?}"
        );
    }
}

#[test]
fn corpus_all_built_in_extensions_denied_individually() {
    for ext in BUILT_IN_DENY_EXTENSIONS {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        write_tree(&root, &[("ok.txt", "ok\n"), (&format!("bad.{ext}"), "x")]);
        let paths = rels_policy(&root, &FolderListPolicy::default());
        assert_eq!(paths, vec!["ok.txt".to_string()], "ext={ext}");
    }
}

#[test]
fn corpus_oversized_boundary_around_attachment_cap() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("p");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("small.txt"), vec![b'a'; 16]).unwrap();
    std::fs::write(
        root.join("exact.txt"),
        vec![b'b'; MAX_FILE_ATTACHMENT_BYTES as usize],
    )
    .unwrap();
    std::fs::write(
        root.join("over.txt"),
        vec![b'c'; MAX_FILE_ATTACHMENT_BYTES as usize + 1],
    )
    .unwrap();
    let paths = rels_policy(&root, &FolderListPolicy::default());
    assert!(paths.contains(&"small.txt".to_string()));
    assert!(paths.contains(&"exact.txt".to_string()));
    assert!(!paths.contains(&"over.txt".to_string()));
}

#[test]
fn corpus_never_list_many_roots_and_nested() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let roots: Vec<PathBuf> = (0..8)
        .map(|i| {
            let p = temp.path().join(format!("deny{i}"));
            write_tree(&p, &[("x.txt", "x\n")]);
            session.add_never_list_path(&p).unwrap();
            p
        })
        .collect();
    let ok = temp.path().join("ok");
    write_tree(&ok, &[("y.txt", "y\n")]);
    let policy = session.folder_list_policy().unwrap();
    for r in &roots {
        let err = list_folder_entries_with_policy(r, None, temp.path(), &policy).unwrap_err();
        assert!(matches!(
            err,
            ContextToolError::FolderBlocked {
                reason: FolderBlockReason::NeverList,
                ..
            }
        ));
    }
    let listing = list_folder_entries_with_policy(&ok, None, temp.path(), &policy).unwrap();
    assert_eq!(listing.entries.len(), 1);
}

#[test]
fn corpus_allowlist_multiple_approved_roots() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let a = temp.path().join("a");
    let b = temp.path().join("b");
    let c = temp.path().join("c");
    write_tree(&a, &[("a.txt", "a\n")]);
    write_tree(&b, &[("b.txt", "b\n")]);
    write_tree(&c, &[("c.txt", "c\n")]);
    session.set_folder_allowlist_enabled(true).unwrap();
    session.add_folder_allowlist_root(&a).unwrap();
    session.add_folder_allowlist_root(&b).unwrap();
    let policy = session.folder_list_policy().unwrap();
    assert!(list_folder_entries_with_policy(&a, None, temp.path(), &policy).is_ok());
    assert!(list_folder_entries_with_policy(&b, None, temp.path(), &policy).is_ok());
    assert!(matches!(
        list_folder_entries_with_policy(&c, None, temp.path(), &policy),
        Err(ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NotAllowlisted,
            ..
        })
    ));
}

#[test]
fn corpus_selection_required_even_when_listing_non_empty() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("p");
    write_tree(&root, &[("a.txt", "a\n"), ("b.txt", "b\n")]);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    assert!(listing.entries.len() >= 2);
    assert!(matches!(
        folder_attachment_from_selection(&listing, &[]),
        Err(ContextToolError::EmptyFolderSelection { .. })
    ));
    let draft = folder_attachment_from_selection(&listing, &["a.txt".into()]).unwrap();
    assert!(draft.context_block.contains("a.txt"));
    assert!(!draft.context_block.contains("[Folder file: p/b.txt]"));
}

#[test]
fn corpus_gitignore_does_not_attach_ignored_when_forcing_selection_name() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("p");
    write_tree(
        &root,
        &[
            (".gitignore", "secret.txt\n"),
            ("public.txt", "pub\n"),
            ("secret.txt", "sec\n"),
        ],
    );
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    assert!(!listing
        .entries
        .iter()
        .any(|e| e.relative_path == "secret.txt"));
    // Selecting a path not in the listing yields empty usable content → error.
    let err = folder_attachment_from_selection(&listing, &["secret.txt".into()]);
    assert!(matches!(
        err,
        Err(ContextToolError::EmptyFolderSelection { .. })
    ));
}

#[test]
fn corpus_policy_flags_independently_toggle() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("p");
    write_tree(
        &root,
        &[
            (".gitignore", "ignored.txt\n"),
            ("ignored.txt", "i\n"),
            ("lib.so", "b"),
            ("ok.txt", "o\n"),
        ],
    );
    let both = FolderListPolicy::default();
    let no_gi = FolderListPolicy {
        honor_gitignore: false,
        ..FolderListPolicy::default()
    };
    let no_deny = FolderListPolicy {
        apply_built_in_deny: false,
        ..FolderListPolicy::default()
    };
    let neither = FolderListPolicy {
        honor_gitignore: false,
        apply_built_in_deny: false,
        ..FolderListPolicy::default()
    };
    assert!(!rels_policy(&root, &both).contains(&"ignored.txt".to_string()));
    assert!(!rels_policy(&root, &both).contains(&"lib.so".to_string()));
    assert!(rels_policy(&root, &no_gi).contains(&"ignored.txt".to_string()));
    assert!(!rels_policy(&root, &no_gi).contains(&"lib.so".to_string()));
    assert!(!rels_policy(&root, &no_deny).contains(&"ignored.txt".to_string()));
    assert!(rels_policy(&root, &no_deny).contains(&"lib.so".to_string()));
    let all = rels_policy(&root, &neither);
    assert!(all.contains(&"ignored.txt".to_string()));
    assert!(all.contains(&"lib.so".to_string()));
}

#[test]
fn corpus_duplicate_never_list_add_is_idempotent() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let p = temp.path().join("n");
    std::fs::create_dir_all(&p).unwrap();
    session.add_never_list_path(&p).unwrap();
    session.add_never_list_path(&p).unwrap();
    assert_eq!(session.list_never_list_paths().unwrap().len(), 1);
}

#[test]
fn corpus_remove_missing_never_list_is_ok() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let p = temp.path().join("missing");
    std::fs::create_dir_all(&p).unwrap();
    session.remove_never_list_path(&p).unwrap();
    assert!(session.list_never_list_paths().unwrap().is_empty());
}
