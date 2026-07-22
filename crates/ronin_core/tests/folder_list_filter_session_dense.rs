//! Dense session persistence cases for never-list / allowlist (#71).

use ronin_core::{
    list_folder_entries_with_policy, ContextToolError, FolderBlockReason, LocalKnowledgeConfig,
    RoninConfig, RoninPaths, RoninSession,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap()
}

#[test]
fn dense_session_never_list_round_trips_many_paths() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let mut paths = Vec::new();
    for i in 0..40 {
        let p = temp.path().join(format!("n{i}"));
        std::fs::create_dir_all(&p).unwrap();
        session.add_never_list_path(&p).unwrap();
        paths.push(p);
    }
    assert_eq!(session.list_never_list_paths().unwrap().len(), 40);
    let policy = session.folder_list_policy().unwrap();
    for p in &paths {
        assert!(matches!(
            list_folder_entries_with_policy(p, None, temp.path(), &policy),
            Err(ContextToolError::FolderBlocked {
                reason: FolderBlockReason::NeverList,
                ..
            })
        ));
    }
    for p in &paths {
        session.remove_never_list_path(p).unwrap();
    }
    assert!(session.list_never_list_paths().unwrap().is_empty());
}

#[test]
fn dense_session_allowlist_round_trips_many_roots() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    session.set_folder_allowlist_enabled(true).unwrap();
    let mut allowed = Vec::new();
    for i in 0..35 {
        let p = temp.path().join(format!("a{i}"));
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("f.txt"), "x").unwrap();
        session.add_folder_allowlist_root(&p).unwrap();
        allowed.push(p);
    }
    assert_eq!(session.list_folder_allowlist_roots().unwrap().len(), 35);
    let policy = session.folder_list_policy().unwrap();
    for p in &allowed {
        assert!(list_folder_entries_with_policy(p, None, temp.path(), &policy).is_ok());
    }
    let outsider = temp.path().join("outsider");
    std::fs::create_dir_all(&outsider).unwrap();
    assert!(matches!(
        list_folder_entries_with_policy(&outsider, None, temp.path(), &policy),
        Err(ContextToolError::FolderBlocked {
            reason: FolderBlockReason::NotAllowlisted,
            ..
        })
    ));
    for p in &allowed {
        session.remove_folder_allowlist_root(p).unwrap();
    }
    session.set_folder_allowlist_enabled(false).unwrap();
    assert!(session.list_folder_allowlist_roots().unwrap().is_empty());
    assert!(!session.folder_allowlist_enabled().unwrap());
}

#[test]
fn dense_session_rejects_non_directory_never_list() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let file = temp.path().join("not-a-dir.txt");
    std::fs::write(&file, "x").unwrap();
    assert!(session.add_never_list_path(&file).is_err());
    assert!(session.add_folder_allowlist_root(&file).is_err());
}

#[test]
fn dense_local_knowledge_config_defaults_empty() {
    for _ in 0..40 {
        let temp = TempDir::new().unwrap();
        let session = open_session(&temp);
        let cfg = session.load_config().unwrap();
        assert!(cfg.local_knowledge.never_list.is_empty());
        assert!(!cfg.local_knowledge.allowlist_enabled);
        assert!(cfg.local_knowledge.allowlist.is_empty());
    }
}

#[test]
fn dense_session_survives_reload_after_config_write() {
    for i in 0..30 {
        let temp = TempDir::new().unwrap();
        let session = open_session(&temp);
        let never = temp.path().join(format!("n{i}"));
        let allow = temp.path().join(format!("a{i}"));
        std::fs::create_dir_all(&never).unwrap();
        std::fs::create_dir_all(&allow).unwrap();
        session.add_never_list_path(&never).unwrap();
        session.set_folder_allowlist_enabled(true).unwrap();
        session.add_folder_allowlist_root(&allow).unwrap();

        let reloaded = open_session(&temp);
        assert!(reloaded.folder_allowlist_enabled().unwrap());
        assert_eq!(reloaded.list_never_list_paths().unwrap().len(), 1);
        assert_eq!(reloaded.list_folder_allowlist_roots().unwrap().len(), 1);
        let policy = reloaded.folder_list_policy().unwrap();
        assert!(matches!(
            list_folder_entries_with_policy(&never, None, temp.path(), &policy),
            Err(ContextToolError::FolderBlocked {
                reason: FolderBlockReason::NeverList,
                ..
            })
        ));
    }
}

#[test]
fn dense_direct_local_knowledge_config_save_load() {
    let samples: &[LocalKnowledgeConfig] = &[
        LocalKnowledgeConfig::default(),
        LocalKnowledgeConfig {
            never_list: vec!["/tmp/a".into()],
            allowlist_enabled: false,
            allowlist: vec![],
        },
        LocalKnowledgeConfig {
            never_list: vec!["/tmp/a".into(), "/tmp/b".into()],
            allowlist_enabled: true,
            allowlist: vec!["/home/u/code".into()],
        },
        LocalKnowledgeConfig {
            never_list: (0..20).map(|i| format!("/n{i}")).collect(),
            allowlist_enabled: true,
            allowlist: (0..15).map(|i| format!("/a{i}")).collect(),
        },
    ];
    for sample in samples {
        for _ in 0..10 {
            let temp = TempDir::new().unwrap();
            let session = open_session(&temp);
            let cfg = RoninConfig {
                local_knowledge: sample.clone(),
                ..RoninConfig::default()
            };
            session.save_config(&cfg).unwrap();
            let loaded = open_session(&temp).load_config().unwrap();
            assert_eq!(loaded.local_knowledge, *sample);
        }
    }
}

#[test]
fn dense_never_list_then_clear_restores_listing() {
    for i in 0..25 {
        let temp = TempDir::new().unwrap();
        let session = open_session(&temp);
        let p = temp.path().join(format!("p{i}"));
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("x.txt"), "x").unwrap();
        session.add_never_list_path(&p).unwrap();
        let blocked = session.folder_list_policy().unwrap();
        assert!(list_folder_entries_with_policy(&p, None, temp.path(), &blocked).is_err());
        session.remove_never_list_path(&p).unwrap();
        let open = session.folder_list_policy().unwrap();
        let listing = list_folder_entries_with_policy(&p, None, temp.path(), &open).unwrap();
        assert_eq!(listing.entries.len(), 1);
    }
}
