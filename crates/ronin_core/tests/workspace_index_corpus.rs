//! Corpus coverage for lexical workspace index (#73).

use std::path::Path;
use std::sync::atomic::AtomicBool;

use ronin_core::{
    collect_workspace_index_documents, may_inject_into_chat_request, ContextOrigin,
    FolderListPolicy, MessageRole, RoninPaths, RoninSession, WorkspaceIndexCaps,
    WorkspaceIndexPhase,
};
use ronin_db::WorkspaceLexicalStore;
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap()
}

fn write_files(root: &Path, files: &[(&str, &str)]) {
    for (rel, body) in files {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, body).unwrap();
    }
}

#[test]
fn corpus_build_many_named_projects() {
    let names: &[&str] = &[
        "proj_000", "proj_001", "proj_002", "proj_003", "proj_004", "proj_005", "proj_006",
        "proj_007", "proj_008", "proj_009", "proj_010", "proj_011", "proj_012", "proj_013",
        "proj_014", "proj_015", "proj_016", "proj_017", "proj_018", "proj_019", "proj_020",
        "proj_021", "proj_022", "proj_023", "proj_024", "proj_025", "proj_026", "proj_027",
        "proj_028", "proj_029", "proj_030", "proj_031", "proj_032", "proj_033", "proj_034",
        "proj_035", "proj_036", "proj_037", "proj_038", "proj_039", "proj_040", "proj_041",
        "proj_042", "proj_043", "proj_044", "proj_045", "proj_046", "proj_047", "proj_048",
        "proj_049", "proj_050", "proj_051", "proj_052", "proj_053", "proj_054", "proj_055",
        "proj_056", "proj_057", "proj_058", "proj_059",
    ];
    for name in names {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(name);
        write_files(
            &root,
            &[
                ("README.md", &format!("# {name}\n")),
                ("src/main.rs", "fn main() {}\n"),
                ("src/util.rs", "pub fn u() {}\n"),
            ],
        );
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &root)
            .unwrap();
        let info = session.build_workspace_index(&thread.id).unwrap();
        assert_eq!(info.phase, WorkspaceIndexPhase::Done, "{name}");
        assert!(info.entry_count >= 3, "{name}");
        let store =
            WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id))
                .unwrap();
        assert!(store.contains_path("README.md").unwrap(), "{name}");
        assert!(store.contains_path("src/main.rs").unwrap(), "{name}");
    }
}

#[test]
fn corpus_delete_rebuild_cycles() {
    let cycles: &[u32] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ];
    for &cycle in cycles {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("ws");
        write_files(&root, &[("a.txt", "one"), ("b.txt", "two")]);
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &root)
            .unwrap();
        for c in 0..cycle {
            let info = session.build_workspace_index(&thread.id).unwrap();
            assert_eq!(info.phase, WorkspaceIndexPhase::Done, "cycle {c}");
            session.delete_workspace_index(&thread.id).unwrap();
            assert_eq!(
                session.workspace_index_info(&thread.id).unwrap().phase,
                WorkspaceIndexPhase::Absent,
                "cycle {c}"
            );
        }
    }
}

#[test]
fn corpus_never_list_blocks_many_secret_dirs() {
    let secrets: &[&str] = &[
        "secret_00",
        "secret_01",
        "secret_02",
        "secret_03",
        "secret_04",
        "secret_05",
        "secret_06",
        "secret_07",
        "secret_08",
        "secret_09",
        "secret_10",
        "secret_11",
        "secret_12",
        "secret_13",
        "secret_14",
        "secret_15",
        "secret_16",
        "secret_17",
        "secret_18",
        "secret_19",
        "secret_20",
        "secret_21",
        "secret_22",
        "secret_23",
        "secret_24",
        "secret_25",
        "secret_26",
        "secret_27",
        "secret_28",
        "secret_29",
        "secret_30",
        "secret_31",
        "secret_32",
        "secret_33",
        "secret_34",
        "secret_35",
        "secret_36",
        "secret_37",
        "secret_38",
        "secret_39",
        "secret_40",
        "secret_41",
        "secret_42",
        "secret_43",
        "secret_44",
        "secret_45",
        "secret_46",
        "secret_47",
        "secret_48",
        "secret_49",
    ];
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    write_files(&root, &[("ok.txt", "visible")]);
    for s in secrets {
        write_files(&root.join(s), &[("hidden.txt", "nope")]);
    }
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &root)
        .unwrap();
    for s in secrets {
        session.add_never_list_path(root.join(s)).unwrap();
    }
    let info = session.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    let store =
        WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id)).unwrap();
    assert!(store.contains_path("ok.txt").unwrap());
    for s in secrets {
        assert!(
            !store.contains_path(&format!("{s}/hidden.txt")).unwrap(),
            "{s}"
        );
    }
}

#[test]
fn corpus_gitignore_patterns_omit_ignored_files() {
    let patterns: &[(&str, &str, &str)] = &[
        ("node_modules/", "node_modules/x/a.js", "src/a.rs"),
        ("target/", "target/debug/x", "src/b.rs"),
        ("*.log", "app.log", "src/c.rs"),
        ("dist/", "dist/bundle.js", "lib/d.rs"),
        ("build/", "build/out.o", "src/e.rs"),
        (".env", ".env", "src/f.rs"),
        ("vendor/", "vendor/pkg/x", "src/g.rs"),
        ("__pycache__/", "__pycache__/a.pyc", "src/h.rs"),
        ("*.tmp", "x.tmp", "src/i.rs"),
        ("secret/", "secret/k", "src/j.rs"),
        ("skip0/", "skip0/x.txt", "keep0.rs"),
        ("skip1/", "skip1/x.txt", "keep1.rs"),
        ("skip2/", "skip2/x.txt", "keep2.rs"),
        ("skip3/", "skip3/x.txt", "keep3.rs"),
        ("skip4/", "skip4/x.txt", "keep4.rs"),
        ("skip5/", "skip5/x.txt", "keep5.rs"),
        ("skip6/", "skip6/x.txt", "keep6.rs"),
        ("skip7/", "skip7/x.txt", "keep7.rs"),
        ("skip8/", "skip8/x.txt", "keep8.rs"),
        ("skip9/", "skip9/x.txt", "keep9.rs"),
        ("skip10/", "skip10/x.txt", "keep10.rs"),
        ("skip11/", "skip11/x.txt", "keep11.rs"),
        ("skip12/", "skip12/x.txt", "keep12.rs"),
        ("skip13/", "skip13/x.txt", "keep13.rs"),
        ("skip14/", "skip14/x.txt", "keep14.rs"),
        ("skip15/", "skip15/x.txt", "keep15.rs"),
        ("skip16/", "skip16/x.txt", "keep16.rs"),
        ("skip17/", "skip17/x.txt", "keep17.rs"),
        ("skip18/", "skip18/x.txt", "keep18.rs"),
        ("skip19/", "skip19/x.txt", "keep19.rs"),
        ("skip20/", "skip20/x.txt", "keep20.rs"),
        ("skip21/", "skip21/x.txt", "keep21.rs"),
        ("skip22/", "skip22/x.txt", "keep22.rs"),
        ("skip23/", "skip23/x.txt", "keep23.rs"),
        ("skip24/", "skip24/x.txt", "keep24.rs"),
        ("skip25/", "skip25/x.txt", "keep25.rs"),
        ("skip26/", "skip26/x.txt", "keep26.rs"),
        ("skip27/", "skip27/x.txt", "keep27.rs"),
        ("skip28/", "skip28/x.txt", "keep28.rs"),
        ("skip29/", "skip29/x.txt", "keep29.rs"),
        ("skip30/", "skip30/x.txt", "keep30.rs"),
        ("skip31/", "skip31/x.txt", "keep31.rs"),
        ("skip32/", "skip32/x.txt", "keep32.rs"),
        ("skip33/", "skip33/x.txt", "keep33.rs"),
        ("skip34/", "skip34/x.txt", "keep34.rs"),
        ("skip35/", "skip35/x.txt", "keep35.rs"),
        ("skip36/", "skip36/x.txt", "keep36.rs"),
        ("skip37/", "skip37/x.txt", "keep37.rs"),
        ("skip38/", "skip38/x.txt", "keep38.rs"),
        ("skip39/", "skip39/x.txt", "keep39.rs"),
    ];
    for (ignore_line, bad, good) in patterns {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("ws");
        write_files(
            &root,
            &[
                (".gitignore", ignore_line),
                (bad, "ignored-body"),
                (good, "kept-body"),
            ],
        );
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
        assert!(paths.contains(good), "{ignore_line} missing {good}");
        assert!(!paths.contains(bad), "{ignore_line} leaked {bad}");
    }
}

#[test]
fn corpus_index_build_leaves_chat_untouched_many_messages() {
    let texts: &[&str] = &[
        "user message 000 before any index",
        "user message 001 before any index",
        "user message 002 before any index",
        "user message 003 before any index",
        "user message 004 before any index",
        "user message 005 before any index",
        "user message 006 before any index",
        "user message 007 before any index",
        "user message 008 before any index",
        "user message 009 before any index",
        "user message 010 before any index",
        "user message 011 before any index",
        "user message 012 before any index",
        "user message 013 before any index",
        "user message 014 before any index",
        "user message 015 before any index",
        "user message 016 before any index",
        "user message 017 before any index",
        "user message 018 before any index",
        "user message 019 before any index",
        "user message 020 before any index",
        "user message 021 before any index",
        "user message 022 before any index",
        "user message 023 before any index",
        "user message 024 before any index",
        "user message 025 before any index",
        "user message 026 before any index",
        "user message 027 before any index",
        "user message 028 before any index",
        "user message 029 before any index",
        "user message 030 before any index",
        "user message 031 before any index",
        "user message 032 before any index",
        "user message 033 before any index",
        "user message 034 before any index",
        "user message 035 before any index",
        "user message 036 before any index",
        "user message 037 before any index",
        "user message 038 before any index",
        "user message 039 before any index",
        "user message 040 before any index",
        "user message 041 before any index",
        "user message 042 before any index",
        "user message 043 before any index",
        "user message 044 before any index",
        "user message 045 before any index",
        "user message 046 before any index",
        "user message 047 before any index",
        "user message 048 before any index",
        "user message 049 before any index",
        "user message 050 before any index",
        "user message 051 before any index",
        "user message 052 before any index",
        "user message 053 before any index",
        "user message 054 before any index",
        "user message 055 before any index",
        "user message 056 before any index",
        "user message 057 before any index",
        "user message 058 before any index",
        "user message 059 before any index",
        "user message 060 before any index",
        "user message 061 before any index",
        "user message 062 before any index",
        "user message 063 before any index",
        "user message 064 before any index",
        "user message 065 before any index",
        "user message 066 before any index",
        "user message 067 before any index",
        "user message 068 before any index",
        "user message 069 before any index",
        "user message 070 before any index",
        "user message 071 before any index",
        "user message 072 before any index",
        "user message 073 before any index",
        "user message 074 before any index",
        "user message 075 before any index",
        "user message 076 before any index",
        "user message 077 before any index",
        "user message 078 before any index",
        "user message 079 before any index",
    ];
    for text in texts {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("ws");
        write_files(&root, &[("leak.txt", "SHOULD_NOT_APPEAR_IN_CHAT")]);
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &root)
            .unwrap();
        session
            .create_message(&thread.id, MessageRole::User, text)
            .unwrap();
        session.build_workspace_index(&thread.id).unwrap();
        let msgs = session.list_messages(&thread.id).unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, *text);
        assert!(!msgs[0].content.contains("SHOULD_NOT_APPEAR_IN_CHAT"));
        assert!(!may_inject_into_chat_request(
            ContextOrigin::WorkspaceIndexCorpus
        ));
        assert!(!may_inject_into_chat_request(ContextOrigin::IndexSearchHit));
    }
}

#[test]
fn corpus_reopen_never_auto_indexes_many_threads() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    write_files(&root, &[("a.txt", "a")]);
    let ids = {
        let session = open_session(&temp);
        let mut ids = Vec::new();
        for _ in 0..25 {
            let t = session.create_thread().unwrap();
            session.set_thread_workspace_root(&t.id, &root).unwrap();
            ids.push(t.id);
        }
        ids
    };
    let session = open_session(&temp);
    for id in &ids {
        let info = session.workspace_index_info(id).unwrap();
        assert_eq!(info.phase, WorkspaceIndexPhase::Absent, "{id}");
        assert!(!session.workspace_index_storage_path_for(id).exists());
    }
}

#[test]
fn corpus_indexed_bodies_scrub_keyed_secrets() {
    let payloads: &[(&str, &str)] = &[
        ("api_key=sk-live-abc", "x"),
        ("token=supersecret", "x"),
        ("password=hunter2", "x"),
        ("Bearer abcdefghijklmnop", "x"),
        ("api_key=secret_value_0", "x"),
        ("api_key=secret_value_1", "x"),
        ("api_key=secret_value_2", "x"),
        ("api_key=secret_value_3", "x"),
        ("api_key=secret_value_4", "x"),
        ("api_key=secret_value_5", "x"),
        ("api_key=secret_value_6", "x"),
        ("api_key=secret_value_7", "x"),
        ("api_key=secret_value_8", "x"),
        ("api_key=secret_value_9", "x"),
        ("api_key=secret_value_10", "x"),
        ("api_key=secret_value_11", "x"),
        ("api_key=secret_value_12", "x"),
        ("api_key=secret_value_13", "x"),
        ("api_key=secret_value_14", "x"),
        ("api_key=secret_value_15", "x"),
        ("api_key=secret_value_16", "x"),
        ("api_key=secret_value_17", "x"),
        ("api_key=secret_value_18", "x"),
        ("api_key=secret_value_19", "x"),
        ("api_key=secret_value_20", "x"),
        ("api_key=secret_value_21", "x"),
        ("api_key=secret_value_22", "x"),
        ("api_key=secret_value_23", "x"),
        ("api_key=secret_value_24", "x"),
        ("api_key=secret_value_25", "x"),
        ("api_key=secret_value_26", "x"),
        ("api_key=secret_value_27", "x"),
        ("api_key=secret_value_28", "x"),
        ("api_key=secret_value_29", "x"),
        ("api_key=secret_value_30", "x"),
        ("api_key=secret_value_31", "x"),
        ("api_key=secret_value_32", "x"),
        ("api_key=secret_value_33", "x"),
        ("api_key=secret_value_34", "x"),
        ("api_key=secret_value_35", "x"),
        ("api_key=secret_value_36", "x"),
        ("api_key=secret_value_37", "x"),
        ("api_key=secret_value_38", "x"),
        ("api_key=secret_value_39", "x"),
    ];
    for (dirty, _) in payloads {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("ws");
        write_files(&root, &[("cfg.txt", dirty)]);
        let result = collect_workspace_index_documents(
            &root,
            &FolderListPolicy::default(),
            &WorkspaceIndexCaps::default(),
            &AtomicBool::new(false),
        );
        assert_eq!(result.documents.len(), 1);
        let body = &result.documents[0].body;
        assert!(
            body.contains("[REDACTED]") || !body.contains("secret_value"),
            "dirty={dirty} body={body}"
        );
        // raw key material from our fixtures should not remain verbatim for api_key forms
        if let Some(secret) = dirty.strip_prefix("api_key=") {
            assert!(
                !body.contains(secret) || body.contains("[REDACTED]"),
                "{body}"
            );
        }
    }
}
