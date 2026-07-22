//! Extra dense public-seam corpus for thread workspace root (#70).
//! Exercises set/clear/show, resolve helpers, and negative auto-bind cases.

use std::path::{Path, PathBuf};

use ronin_core::{
    context_path_base, list_folder_entries, read_file_attachment, resolve_context_path, RoninPaths,
    RoninSession,
};
use tempfile::TempDir;

fn session_in(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap()
}

fn project_tree(temp: &TempDir, name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = temp.path().join(name);
    for (rel, body) in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, body).unwrap();
    }
    root
}

#[test]
fn set_clear_show_corpus_across_many_thread_bindings() {
    let temp = TempDir::new().unwrap();
    let session = session_in(&temp);
    let roots: Vec<PathBuf> = (1..=20)
        .map(|i| {
            let p = temp.path().join(format!("ws-{i}"));
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("marker.txt"), format!("ws-{i}")).unwrap();
            p
        })
        .collect();

    let mut thread_ids = Vec::new();
    for root in &roots {
        let thread = session.create_thread().unwrap();
        assert!(thread.workspace_root.is_none());
        session.set_thread_workspace_root(&thread.id, root).unwrap();
        thread_ids.push(thread.id);
    }

    let listed = session.list_threads().unwrap();
    for (id, root) in thread_ids.iter().zip(roots.iter()) {
        let t = listed.iter().find(|t| t.id == *id).unwrap();
        assert_eq!(
            t.workspace_root.as_deref(),
            Some(root.canonicalize().unwrap().as_path())
        );
    }

    for id in &thread_ids {
        session.clear_thread_workspace_root(id).unwrap();
    }
    for t in session.list_threads().unwrap() {
        if thread_ids.contains(&t.id) {
            assert!(t.workspace_root.is_none());
        }
    }
}

#[test]
fn relative_file_resolve_corpus_reads_expected_content() {
    let temp = TempDir::new().unwrap();
    let files = [
        ("a.txt", "alpha"),
        ("src/b.rs", "fn b() {}"),
        ("src/nested/c.md", "# c"),
        ("docs/d.txt", "delta"),
        ("e.toml", "[pkg]"),
        ("pkg/f/g.rs", "mod g;"),
        ("notes/h.txt", "eta"),
        ("i.json", "{}"),
        ("j/k/l.txt", "lambda"),
        ("m.rs", "fn m() {}"),
        ("n/o.txt", "omicron"),
        ("p/q/r/s.txt", "sigma"),
        ("t.md", "# tau"),
        ("u/v.rs", "fn v() {}"),
        ("w.txt", "omega"),
    ];
    let project = project_tree(&temp, "corpus-proj", &files);
    let elsewhere = temp.path().join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(elsewhere.join("a.txt"), "WRONG").unwrap();

    for (rel, body) in files {
        let draft = read_file_attachment(rel, Some(project.as_path()), &elsewhere).unwrap();
        assert!(
            draft.context_block.contains(body),
            "rel={rel} body missing in {}",
            draft.context_block
        );
        assert_eq!(draft.path.as_deref(), Some(project.join(rel).as_path()));
    }
}

#[test]
fn relative_folder_resolve_corpus_lists_expected_entries() {
    let temp = TempDir::new().unwrap();
    let project = project_tree(
        &temp,
        "folders",
        &[
            ("src/one.rs", "1"),
            ("src/two.rs", "2"),
            ("lib/three.rs", "3"),
            ("lib/four.rs", "4"),
            ("docs/a.md", "a"),
            ("docs/b.md", "b"),
        ],
    );
    let fallback = temp.path().join("fb");
    std::fs::create_dir_all(&fallback).unwrap();

    for (folder, expected) in [
        ("src", &["one.rs", "two.rs"][..]),
        ("lib", &["three.rs", "four.rs"][..]),
        ("docs", &["a.md", "b.md"][..]),
    ] {
        let listing = list_folder_entries(folder, Some(project.as_path()), &fallback).unwrap();
        for name in expected {
            assert!(
                listing.entries.iter().any(|e| e.relative_path == *name),
                "folder={folder} missing {name}"
            );
        }
    }
}

#[test]
fn absolute_paths_ignore_workspace_corpus_reads() {
    let temp = TempDir::new().unwrap();
    let ws = project_tree(&temp, "ws", &[("inside.txt", "inside")]);
    let other = project_tree(
        &temp,
        "other",
        &[
            ("o1.txt", "other-1"),
            ("o2.txt", "other-2"),
            ("o3.txt", "other-3"),
            ("o4.txt", "other-4"),
            ("o5.txt", "other-5"),
            ("o6.txt", "other-6"),
            ("o7.txt", "other-7"),
            ("o8.txt", "other-8"),
            ("o9.txt", "other-9"),
            ("o10.txt", "other-10"),
            ("o11.txt", "other-11"),
            ("o12.txt", "other-12"),
        ],
    );
    let fallback = temp.path().join("fb");
    std::fs::create_dir_all(&fallback).unwrap();

    for i in 1..=12 {
        let name = format!("o{i}.txt");
        let abs = other.join(&name);
        let draft = read_file_attachment(&abs, Some(ws.as_path()), &fallback).unwrap();
        assert!(draft.context_block.contains(&format!("other-{i}")));
        assert_eq!(draft.path.as_deref(), Some(abs.as_path()));
    }
}

#[test]
fn no_auto_bind_corpus_with_git_and_marker_files() {
    let temp = TempDir::new().unwrap();
    // Plant several trees that look like projects; none should bind.
    for i in 1..=15 {
        let p = temp.path().join(format!("looks-{i}"));
        std::fs::create_dir_all(p.join(".git")).unwrap();
        std::fs::write(p.join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
        std::fs::write(p.join("package.json"), "{}\n").unwrap();
    }
    let session = session_in(&temp);
    for _ in 0..15 {
        let thread = session.create_thread().unwrap();
        assert_eq!(thread.workspace_root, None);
    }
}

#[test]
fn resolve_helper_matrix_corpus() {
    let rows = [
        (Some("/w"), "/c", "a", "/w/a"),
        (Some("/w"), "/c", "a/b", "/w/a/b"),
        (Some("/w"), "/c", "a/b/c", "/w/a/b/c"),
        (None, "/c", "a", "/c/a"),
        (None, "/c", "a/b", "/c/a/b"),
        (Some("/proj"), "/tmp", "src/main.rs", "/proj/src/main.rs"),
        (Some("/proj"), "/tmp", "Cargo.toml", "/proj/Cargo.toml"),
        (None, "/tmp", "Cargo.toml", "/tmp/Cargo.toml"),
        (Some("/home/u/p"), "/home/u", "lib.rs", "/home/u/p/lib.rs"),
        (None, "/home/u", "lib.rs", "/home/u/lib.rs"),
        (Some("/r"), "/f", "x/y/z.md", "/r/x/y/z.md"),
        (None, "/f", "x/y/z.md", "/f/x/y/z.md"),
        (Some("/mono"), "/x", "apps/web/a.ts", "/mono/apps/web/a.ts"),
        (None, "/x", "apps/web/a.ts", "/x/apps/web/a.ts"),
        (Some("/w"), "/c", ".hidden", "/w/.hidden"),
        (None, "/c", ".hidden", "/c/.hidden"),
        (Some("/w"), "/c", "dir/", "/w/dir/"),
        (None, "/c", "dir/", "/c/dir/"),
        (Some("/alpha"), "/beta", "gamma", "/alpha/gamma"),
        (None, "/beta", "gamma", "/beta/gamma"),
        (Some("/1"), "/2", "3/4", "/1/3/4"),
        (None, "/2", "3/4", "/2/3/4"),
        (Some("/root"), "/cwd", "loop/PRD.md", "/root/loop/PRD.md"),
        (None, "/cwd", "loop/PRD.md", "/cwd/loop/PRD.md"),
        (Some("/ws"), "/fb", "fixtures/x.txt", "/ws/fixtures/x.txt"),
        (None, "/fb", "fixtures/x.txt", "/fb/fixtures/x.txt"),
        (
            Some("/p"),
            "/q",
            "crates/a/src/lib.rs",
            "/p/crates/a/src/lib.rs",
        ),
        (None, "/q", "crates/a/src/lib.rs", "/q/crates/a/src/lib.rs"),
        (
            Some("/p"),
            "/q",
            "docs/standards.md",
            "/p/docs/standards.md",
        ),
        (None, "/q", "docs/standards.md", "/q/docs/standards.md"),
    ];
    for (ws, cwd, rel, expected) in rows {
        let got = resolve_context_path(Path::new(rel), ws.map(Path::new), Path::new(cwd));
        assert_eq!(
            got,
            PathBuf::from(expected),
            "ws={ws:?} cwd={cwd} rel={rel}"
        );
        assert_eq!(
            context_path_base(ws.map(Path::new), Path::new(cwd)),
            Path::new(ws.unwrap_or(cwd))
        );
    }
}

#[test]
fn absolute_resolve_matrix_corpus() {
    let absolutes = [
        "/abs/1.rs",
        "/abs/2/3.rs",
        "/home/u/file.md",
        "/tmp/shot.png",
        "/var/log/x",
        "/etc/hosts",
        "/opt/a/b/c",
        "/srv/www/index.html",
        "/usr/share/doc/readme",
        "/root/secret",
        "/home/u/Projects/x/Cargo.toml",
        "/home/u/Projects/x/src/main.rs",
        "/tmp/with spaces/f.txt",
        "/abs/./n.rs",
        "/abs/../o.rs",
        "/long/path/to/deep/nested/file.txt",
        "/short",
        "/a/b",
        "/a/b/c/d/e",
        "/workspace-looking/path/file.rs",
    ];
    let ws = Path::new("/workspace");
    let cwd = Path::new("/cwd");
    for abs in absolutes {
        let p = Path::new(abs);
        assert_eq!(resolve_context_path(p, Some(ws), cwd), p);
        assert_eq!(resolve_context_path(p, None, cwd), p);
    }
}

#[test]
fn changing_workspace_then_resolving_uses_latest_root() {
    let temp = TempDir::new().unwrap();
    let a = project_tree(&temp, "a", &[("note.txt", "from-a")]);
    let b = project_tree(&temp, "b", &[("note.txt", "from-b")]);
    let session = session_in(&temp);
    let thread = session.create_thread().unwrap();
    session.set_thread_workspace_root(&thread.id, &a).unwrap();
    let root_a = session.thread_workspace_root(&thread.id).unwrap().unwrap();
    let draft_a = read_file_attachment("note.txt", Some(root_a.as_path()), temp.path()).unwrap();
    assert!(draft_a.context_block.contains("from-a"));

    session.set_thread_workspace_root(&thread.id, &b).unwrap();
    let root_b = session.thread_workspace_root(&thread.id).unwrap().unwrap();
    let draft_b = read_file_attachment("note.txt", Some(root_b.as_path()), temp.path()).unwrap();
    assert!(draft_b.context_block.contains("from-b"));
}

#[test]
fn clear_then_relative_falls_back_to_process_cwd() {
    let temp = TempDir::new().unwrap();
    let ws = project_tree(&temp, "ws", &[("only-ws.txt", "ws")]);
    let cwd = temp.path().join("cwd");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::write(cwd.join("only-cwd.txt"), "cwd").unwrap();

    let session = session_in(&temp);
    let thread = session.create_thread().unwrap();
    session.set_thread_workspace_root(&thread.id, &ws).unwrap();
    session.clear_thread_workspace_root(&thread.id).unwrap();
    assert!(session.thread_workspace_root(&thread.id).unwrap().is_none());

    let draft = read_file_attachment("only-cwd.txt", None, &cwd).unwrap();
    assert!(draft.context_block.contains("cwd"));
}

#[test]
fn launch_attach_intent_does_not_set_workspace_root() {
    // CLI --attach is an explicit file attach path; it must not become a workspace bind.
    let temp = TempDir::new().unwrap();
    let attach = temp.path().join("attached.md");
    std::fs::write(&attach, "attached body").unwrap();
    let other = temp.path().join("other-ws");
    std::fs::create_dir_all(&other).unwrap();
    std::fs::write(other.join("attached.md"), "wrong base").unwrap();

    let session = session_in(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &other)
        .unwrap();

    // Launch attach resolves with workspace=None (process CWD / absolute), even if a
    // thread workspace is bound — unchanged --attach intent.
    let draft = read_file_attachment(&attach, None, temp.path()).unwrap();
    assert!(draft.context_block.contains("attached body"));
    assert!(!draft.context_block.contains("wrong base"));
    assert!(
        session.thread_workspace_root(&thread.id).unwrap().is_some(),
        "workspace may be bound independently of --attach"
    );
}

#[test]
fn launch_attach_relative_path_uses_cwd_not_thread_workspace() {
    let temp = TempDir::new().unwrap();
    let cwd = temp.path().join("cwd");
    let ws = temp.path().join("ws");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(&ws).unwrap();
    std::fs::write(cwd.join("launch.md"), "from-cwd").unwrap();
    std::fs::write(ws.join("launch.md"), "from-ws").unwrap();

    let session = session_in(&temp);
    let thread = session.create_thread().unwrap();
    session.set_thread_workspace_root(&thread.id, &ws).unwrap();

    let draft = read_file_attachment("launch.md", None, &cwd).unwrap();
    assert!(
        draft.context_block.contains("from-cwd"),
        "relative --attach must use process CWD, not workspace"
    );
    assert!(!draft.context_block.contains("from-ws"));
}
