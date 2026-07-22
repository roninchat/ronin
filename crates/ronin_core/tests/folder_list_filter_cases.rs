//! Table-driven cases for folder list ignore / deny / allow (#71).

use std::path::{Path, PathBuf};

use ronin_core::{
    folder_root_block_reason, list_folder_entries, list_folder_entries_with_policy, path_is_under,
    FolderBlockReason, FolderListPolicy, BUILT_IN_DENY_DIR_NAMES, BUILT_IN_DENY_EXTENSIONS,
};
use tempfile::TempDir;

fn write_tree(root: &Path, files: &[(&str, &str)]) {
    for (rel, body) in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, body).unwrap();
    }
}

fn listed(root: &Path, policy: &FolderListPolicy) -> Vec<String> {
    list_folder_entries_with_policy(root, None, root.parent().unwrap_or(root), policy)
        .unwrap()
        .entries
        .into_iter()
        .map(|e| e.relative_path)
        .collect()
}

#[test]
fn path_is_under_cases() {
    let cases: &[(&str, &str, bool)] = &[
        ("/a", "/a", true),
        ("/a/b", "/a", true),
        ("/a/b/c", "/a/b", true),
        ("/ab", "/a", false),
        ("/a", "/a/b", false),
        ("/x", "/y", false),
    ];
    for (path, ancestor, expect) in cases {
        assert_eq!(
            path_is_under(Path::new(path), Path::new(ancestor)),
            *expect,
            "{path} under {ancestor}"
        );
    }
}

#[test]
fn folder_root_block_reason_cases() {
    let never = PathBuf::from("/home/u/secrets");
    let allow = PathBuf::from("/home/u/code");
    let cases: &[(PathBuf, FolderListPolicy, Option<FolderBlockReason>)] = &[
        (
            PathBuf::from("/home/u/proj"),
            FolderListPolicy::default(),
            None,
        ),
        (
            never.clone(),
            FolderListPolicy {
                never_list: vec![never.clone()],
                ..FolderListPolicy::default()
            },
            Some(FolderBlockReason::NeverList),
        ),
        (
            never.join("deep"),
            FolderListPolicy {
                never_list: vec![never.clone()],
                ..FolderListPolicy::default()
            },
            Some(FolderBlockReason::NeverList),
        ),
        (
            PathBuf::from("/home/u/other"),
            FolderListPolicy {
                allowlist_enabled: true,
                allowlist: vec![allow.clone()],
                ..FolderListPolicy::default()
            },
            Some(FolderBlockReason::NotAllowlisted),
        ),
        (
            allow.clone(),
            FolderListPolicy {
                allowlist_enabled: true,
                allowlist: vec![allow.clone()],
                ..FolderListPolicy::default()
            },
            None,
        ),
        (
            allow.join("crate"),
            FolderListPolicy {
                allowlist_enabled: true,
                allowlist: vec![allow.clone()],
                ..FolderListPolicy::default()
            },
            None,
        ),
        (
            PathBuf::from("/anywhere"),
            FolderListPolicy {
                allowlist_enabled: true,
                allowlist: vec![],
                ..FolderListPolicy::default()
            },
            Some(FolderBlockReason::NotAllowlisted),
        ),
    ];
    for (root, policy, expect) in cases {
        assert_eq!(
            folder_root_block_reason(root, policy),
            *expect,
            "root={}",
            root.display()
        );
    }
}

#[test]
fn gitignore_pattern_cases_omit_expected_paths() {
    let patterns: &[(&str, &[&str], &[&str])] = &[
        ("secret.env\n", &["secret.env"], &["ok.txt"]),
        ("*.tmp\n", &["a.tmp", "b.tmp"], &["keep.txt"]),
        ("dist/\n", &["dist/out.js"], &["src/app.rs"]),
        ("/rooted.txt\n", &["rooted.txt"], &["nested/rooted.txt"]),
        (
            "*.log\n!keep.log\n",
            &["noise.log"],
            &["keep.log", "ok.txt"],
        ),
    ];
    for (gitignore, omitted, kept) in patterns {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("proj");
        let mut files = vec![(".gitignore", *gitignore), ("ok.txt", "x\n")];
        for path in omitted.iter().chain(kept.iter()) {
            if *path == "ok.txt" {
                continue;
            }
            files.push((*path, "body\n"));
        }
        // Ensure nested dirs for nested cases.
        write_tree(&root, &files);
        if kept.contains(&"nested/rooted.txt") {
            write_tree(&root, &[("nested/rooted.txt", "body\n")]);
        }
        if kept.contains(&"keep.log") {
            write_tree(&root, &[("keep.log", "body\n")]);
        }
        if omitted.iter().any(|p| p.contains('/')) || kept.iter().any(|p| p.contains('/')) {
            // already written via write_tree when path has /
        }
        let paths = listed(&root, &FolderListPolicy::default());
        for o in *omitted {
            assert!(
                !paths.iter().any(|p| p == o),
                "gitignore={gitignore:?} should omit {o}; got {paths:?}"
            );
        }
        for k in *kept {
            assert!(
                paths.iter().any(|p| p == k),
                "gitignore={gitignore:?} should keep {k}; got {paths:?}"
            );
        }
    }
}

#[test]
fn built_in_deny_extension_cases() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("keep.rs"), "fn k() {}").unwrap();
    for ext in BUILT_IN_DENY_EXTENSIONS.iter().take(12) {
        std::fs::write(root.join(format!("x.{ext}")), "bin").unwrap();
    }
    let paths = listed(&root, &FolderListPolicy::default());
    assert!(paths.iter().any(|p| p == "keep.rs"));
    for ext in BUILT_IN_DENY_EXTENSIONS.iter().take(12) {
        assert!(
            !paths.iter().any(|p| p == &format!("x.{ext}")),
            "extension .{ext} must be denied; got {paths:?}"
        );
    }
}

#[test]
fn built_in_deny_vcs_dir_cases() {
    for name in BUILT_IN_DENY_DIR_NAMES {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("proj");
        write_tree(
            &root,
            &[
                ("readme.md", "ok\n"),
                (&format!("{name}/objects/pack"), "x\n"),
            ],
        );
        let paths = listed(&root, &FolderListPolicy::default());
        assert!(paths.contains(&"readme.md".to_string()));
        assert!(
            !paths.iter().any(|p| p.starts_with(&format!("{name}/"))),
            "{name} must be omitted; got {paths:?}"
        );
    }
}

#[test]
fn nested_gitignore_under_subdirectory_is_honored() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[
            ("top.txt", "keep\n"),
            ("sub/.gitignore", "hidden.txt\n"),
            ("sub/visible.txt", "yes\n"),
            ("sub/hidden.txt", "no\n"),
        ],
    );
    let paths = listed(&root, &FolderListPolicy::default());
    assert!(paths.contains(&"top.txt".to_string()));
    assert!(paths.contains(&"sub/visible.txt".to_string()));
    assert!(!paths.contains(&"sub/hidden.txt".to_string()));
}

#[test]
fn allowlist_disabled_lists_outside_allowlist_entries() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("anywhere");
    write_tree(&root, &[("a.txt", "a\n")]);
    let policy = FolderListPolicy {
        allowlist_enabled: false,
        allowlist: vec![temp.path().join("other")],
        ..FolderListPolicy::default()
    };
    let paths = listed(&root, &policy);
    assert!(paths.contains(&"a.txt".to_string()));
}

#[test]
fn default_list_folder_entries_matches_default_policy() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[
            (".gitignore", "skip.txt\n"),
            ("keep.txt", "k\n"),
            ("skip.txt", "s\n"),
            ("blob.so", "b"),
        ],
    );
    let a = list_folder_entries(&root, None, temp.path()).unwrap();
    let b = list_folder_entries_with_policy(&root, None, temp.path(), &FolderListPolicy::default())
        .unwrap();
    assert_eq!(a.entries, b.entries);
}
