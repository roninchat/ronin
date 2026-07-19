//! Logging preferences in config.toml.

use ronin_core::{LoggingConfig, RoninConfig, RoninPaths, RoninSession};
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
fn load_config_should_default_file_logging_to_disabled() {
    let (_temp, session) = session_with_toml("");
    let config = session.load_config().expect("load");
    assert!(!config.logging.file_enabled);
    assert_eq!(config.logging.max_file_bytes, 5 * 1024 * 1024);
}

#[test]
fn load_config_should_parse_logging_file_enabled() {
    let (_temp, session) = session_with_toml(
        r#"
[logging]
file_enabled = true
max_file_bytes = 1048576
"#,
    );
    let config = session.load_config().expect("load");
    assert!(config.logging.file_enabled);
    assert_eq!(config.logging.max_file_bytes, 1_048_576);
}

#[test]
fn logging_config_should_persist_across_reload() {
    let temp = TempDir::new().unwrap();
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();
    let session = RoninSession::open(paths.clone()).unwrap();
    session
        .save_config(&RoninConfig {
            logging: LoggingConfig {
                file_enabled: true,
                max_file_bytes: 2_000_000,
            },
            ..RoninConfig::default()
        })
        .unwrap();

    let reloaded = RoninSession::open(paths).unwrap();
    let config = reloaded.load_config().unwrap();
    assert!(config.logging.file_enabled);
    assert_eq!(config.logging.max_file_bytes, 2_000_000);
}
