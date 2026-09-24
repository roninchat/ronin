//! FeaturesConfig defaults and TOML persistence.

use ronin_core::{FeaturesConfig, RoninConfig, RoninPaths, RoninSession};
use tempfile::TempDir;

fn session_with_toml(toml_content: &str) -> (TempDir, RoninSession) {
    let temp = TempDir::new().expect("temp dir");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), toml_content).unwrap();

    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    (temp, session)
}

#[test]
fn features_config_should_default_memories_and_artifacts_off() {
    let config = RoninConfig::default();
    assert!(!config.features.memories);
    assert!(!config.features.artifacts);
    assert_eq!(
        FeaturesConfig::default(),
        FeaturesConfig {
            memories: false,
            artifacts: false,
        }
    );
}

#[test]
fn load_config_should_default_features_when_section_missing() {
    let (_temp, session) = session_with_toml("");
    let config = session.load_config().expect("load config");
    assert!(!config.features.memories);
    assert!(!config.features.artifacts);
}

#[test]
fn load_config_should_parse_features_section() {
    let (_temp, session) = session_with_toml(
        r#"
[features]
memories = true
artifacts = true
"#,
    );
    let config = session.load_config().expect("load config");
    assert!(config.features.memories);
    assert!(config.features.artifacts);
}

#[test]
fn features_config_should_round_trip_via_toml() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open session");
    session
        .save_config(&RoninConfig {
            features: FeaturesConfig {
                memories: true,
                artifacts: true,
            },
            ..RoninConfig::default()
        })
        .expect("save config");

    let reloaded = RoninSession::open(paths).expect("reopen session");
    let config = reloaded.load_config().expect("load config");
    assert!(config.features.memories);
    assert!(config.features.artifacts);
}
