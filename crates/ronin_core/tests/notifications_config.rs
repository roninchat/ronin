//! NotificationsConfig persistence and defaults (#75).

use ronin_core::{
    shape_generation_notification, GenerationNotifyInput, GenerationNotifyKind, NotificationPrefs,
    NotificationsConfig, RoninConfig, RoninPaths, RoninSession,
};
use tempfile::TempDir;

fn session_with_toml(toml_content: &str) -> (TempDir, RoninSession) {
    let temp = TempDir::new().expect("temp");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), toml_content).unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open");
    (temp, session)
}

#[test]
fn load_config_should_default_notifications_to_enabled() {
    let (_temp, session) = session_with_toml("");
    let config = session.load_config().expect("load");
    assert!(config.notifications.enabled);
}

#[test]
fn load_config_should_parse_notifications_disabled() {
    let (_temp, session) = session_with_toml(
        r#"
[notifications]
enabled = false
"#,
    );
    let config = session.load_config().expect("load");
    assert!(!config.notifications.enabled);
}

#[test]
fn load_config_should_parse_notifications_enabled_explicitly() {
    let (_temp, session) = session_with_toml(
        r#"
[notifications]
enabled = true
"#,
    );
    let config = session.load_config().expect("load");
    assert!(config.notifications.enabled);
}

#[test]
fn notifications_config_should_persist_across_reload() {
    let temp = TempDir::new().unwrap();
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();
    let session = RoninSession::open(paths.clone()).unwrap();
    session
        .save_config(&RoninConfig {
            notifications: NotificationsConfig { enabled: false },
            ..RoninConfig::default()
        })
        .unwrap();

    let reloaded = RoninSession::open(paths).unwrap();
    let config = reloaded.load_config().unwrap();
    assert!(!config.notifications.enabled);
}

#[test]
fn notifications_config_default_matches_shaping_prefs_default() {
    assert!(NotificationsConfig::default().enabled);
    assert!(NotificationPrefs::default().enabled);
}

#[test]
fn session_notifications_flag_gates_shaping() {
    let (_temp, session) = session_with_toml(
        r#"
[notifications]
enabled = false
"#,
    );
    let enabled = session.load_config().unwrap().notifications.enabled;
    let shaped = shape_generation_notification(
        &NotificationPrefs { enabled },
        &GenerationNotifyInput {
            kind: GenerationNotifyKind::Completed,
            thread_id: "t".into(),
            thread_title: Some("T".into()),
            error_summary: None,
        },
    );
    assert!(shaped.is_none());
}

#[test]
fn provider_import_preserves_notifications_setting() {
    use ronin_core::import_provider_config_toml;
    let current = RoninConfig {
        notifications: NotificationsConfig { enabled: false },
        ..RoninConfig::default()
    };
    let imported = import_provider_config_toml(
        &current,
        r#"
[general]
default_provider = "ollama"
default_model = "llama"
auto_title = true
attachment_warn_chars = 12000

[ollama]
base_url = "http://localhost:11434"
"#,
    )
    .expect("import");
    assert!(
        !imported.notifications.enabled,
        "provider import must not reset notifications"
    );
}

#[test]
fn notifications_enabled_true_allows_shaping_from_session() {
    let (_temp, session) = session_with_toml(
        r#"
[notifications]
enabled = true
"#,
    );
    let enabled = session.load_config().unwrap().notifications.enabled;
    let shaped = shape_generation_notification(
        &NotificationPrefs { enabled },
        &GenerationNotifyInput {
            kind: GenerationNotifyKind::Failed,
            thread_id: "fail-1".into(),
            thread_title: None,
            error_summary: Some("boom".into()),
        },
    );
    assert!(shaped.is_some());
}
