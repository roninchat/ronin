//! Sidebar width clamping and config persistence.

use ronin_core::{
    clamp_sidebar_width, effective_sidebar_width, RoninConfig, RoninPaths, RoninSession, UiConfig,
    SIDEBAR_WIDTH_DEFAULT, SIDEBAR_WIDTH_MAX, SIDEBAR_WIDTH_MIN,
};
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
fn clamp_sidebar_width_should_enforce_min_and_max() {
    assert_eq!(
        clamp_sidebar_width(SIDEBAR_WIDTH_MIN - 40.0),
        SIDEBAR_WIDTH_MIN
    );
    assert_eq!(
        clamp_sidebar_width(SIDEBAR_WIDTH_MAX + 120.0),
        SIDEBAR_WIDTH_MAX
    );
    assert_eq!(clamp_sidebar_width(320.0), 320.0);
}

#[test]
fn clamp_sidebar_width_should_fallback_for_non_finite() {
    assert_eq!(clamp_sidebar_width(f32::NAN), SIDEBAR_WIDTH_DEFAULT);
    assert_eq!(clamp_sidebar_width(f32::INFINITY), SIDEBAR_WIDTH_DEFAULT);
}

#[test]
fn effective_sidebar_width_should_be_zero_when_collapsed() {
    assert_eq!(effective_sidebar_width(320.0, true), 0.0);
    assert_eq!(
        effective_sidebar_width(320.0, false),
        clamp_sidebar_width(320.0)
    );
}

#[test]
fn load_config_should_default_sidebar_layout_when_missing() {
    let (_temp, session) = session_with_toml("");
    let config = session.load_config().expect("load config");
    assert_eq!(config.ui.sidebar_width, SIDEBAR_WIDTH_DEFAULT);
    assert!(!config.ui.sidebar_collapsed);
}

#[test]
fn load_config_should_parse_sidebar_width_and_collapsed() {
    let (_temp, session) = session_with_toml(
        r#"
[ui]
sidebar_width = 360.0
sidebar_collapsed = true
"#,
    );
    let config = session.load_config().expect("load config");
    assert_eq!(config.ui.sidebar_width, 360.0);
    assert!(config.ui.sidebar_collapsed);
}

#[test]
fn sidebar_layout_should_persist_across_config_reload() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open session");
    session
        .save_config(&RoninConfig {
            ui: UiConfig {
                sidebar_width: 300.0,
                sidebar_collapsed: true,
            },
            ..RoninConfig::default()
        })
        .expect("save config");

    let reloaded = RoninSession::open(paths).expect("reopen session");
    let config = reloaded.load_config().expect("load config");
    assert_eq!(config.ui.sidebar_width, 300.0);
    assert!(config.ui.sidebar_collapsed);
}
