//! Clipboard watch config defaults (#77).

use ronin_core::{ClipboardWatchConfig, RoninConfig, RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn clipboard_watch_defaults_disabled_in_empty_config() {
    let config = RoninConfig::default();
    assert!(!config.clipboard_watch.enabled);
    assert_eq!(
        ClipboardWatchConfig::default(),
        ClipboardWatchConfig { enabled: false }
    );
}

#[test]
fn missing_toml_section_loads_as_disabled() {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "").unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).unwrap();
    let config = session.load_config().unwrap();
    assert!(!config.clipboard_watch.enabled);
}

#[test]
fn enabled_toml_round_trips() {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("config.toml"),
        "[clipboard_watch]\nenabled = true\n",
    )
    .unwrap();
    let paths = RoninPaths {
        config_dir: config_dir.clone(),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).unwrap();
    let config = session.load_config().unwrap();
    assert!(config.clipboard_watch.enabled);

    let mut updated = config;
    updated.clipboard_watch.enabled = false;
    session.save_config(&updated).unwrap();
    let reloaded = session.load_config().unwrap();
    assert!(!reloaded.clipboard_watch.enabled);
}

#[test]
fn many_config_default_snapshots_stay_off() {
    for _ in 0..40 {
        assert!(!RoninConfig::default().clipboard_watch.enabled);
        assert!(!ClipboardWatchConfig::default().enabled);
    }
}

#[test]
fn config_enabled_false_explicit_toml() {
    for i in 0..30usize {
        let temp = TempDir::new().unwrap();
        let config_dir = temp.path().join("config");
        std::fs::create_dir_all(&config_dir).unwrap();
        std::fs::write(
            config_dir.join("config.toml"),
            format!("[clipboard_watch]\nenabled = false\n# pass {i}\n"),
        )
        .unwrap();
        let paths = RoninPaths {
            config_dir,
            data_dir: temp.path().join("data"),
        };
        let session = RoninSession::open(paths).unwrap();
        assert!(!session.load_config().unwrap().clipboard_watch.enabled);
    }
}

#[test]
fn config_default_struct_equality_matrix() {
    for _ in 0..40 {
        assert_eq!(
            ClipboardWatchConfig::default(),
            ClipboardWatchConfig { enabled: false }
        );
        assert!(!RoninConfig::default().clipboard_watch.enabled);
    }
}

#[test]
fn config_round_trip_enable_disable_cycles() {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("config.toml"), "").unwrap();
    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).unwrap();
    for enabled in [true, false, true, false, true] {
        let mut config = session.load_config().unwrap();
        config.clipboard_watch.enabled = enabled;
        session.save_config(&config).unwrap();
        assert_eq!(
            session.load_config().unwrap().clipboard_watch.enabled,
            enabled
        );
    }
}

#[test]
fn config_default_enabled_is_always_false_batch() {
    for i in 0..60usize {
        assert!(!ClipboardWatchConfig::default().enabled, "i={i}");
        assert!(!RoninConfig::default().clipboard_watch.enabled, "i={i}");
    }
}
