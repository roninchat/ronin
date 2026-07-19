use ronin_core::{
    resolve_color_scheme, ColorScheme, RoninConfig, RoninPaths, RoninSession, ThemePreference,
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
fn load_config_should_parse_theme_light() {
    let (_temp, session) = session_with_toml(r#"theme = "light""#);
    let config = session.load_config().expect("load config");
    assert_eq!(config.theme, ThemePreference::Light);
}

#[test]
fn load_config_should_parse_theme_dark() {
    let (_temp, session) = session_with_toml(r#"theme = "dark""#);
    let config = session.load_config().expect("load config");
    assert_eq!(config.theme, ThemePreference::Dark);
}

#[test]
fn load_config_should_parse_theme_system() {
    let (_temp, session) = session_with_toml(r#"theme = "system""#);
    let config = session.load_config().expect("load config");
    assert_eq!(config.theme, ThemePreference::System);
}

#[test]
fn load_config_should_default_theme_to_system_when_missing() {
    let (_temp, session) = session_with_toml("");
    let config = session.load_config().expect("load config");
    assert_eq!(config.theme, ThemePreference::System);
}

#[test]
fn resolve_color_scheme_should_force_dark_regardless_of_system() {
    assert_eq!(
        resolve_color_scheme(ThemePreference::Dark, ColorScheme::Light),
        ColorScheme::Dark
    );
    assert_eq!(
        resolve_color_scheme(ThemePreference::Dark, ColorScheme::Dark),
        ColorScheme::Dark
    );
}

#[test]
fn resolve_color_scheme_should_force_light_regardless_of_system() {
    assert_eq!(
        resolve_color_scheme(ThemePreference::Light, ColorScheme::Dark),
        ColorScheme::Light
    );
    assert_eq!(
        resolve_color_scheme(ThemePreference::Light, ColorScheme::Light),
        ColorScheme::Light
    );
}

#[test]
fn resolve_color_scheme_should_follow_system_when_preference_is_system() {
    assert_eq!(
        resolve_color_scheme(ThemePreference::System, ColorScheme::Light),
        ColorScheme::Light
    );
    assert_eq!(
        resolve_color_scheme(ThemePreference::System, ColorScheme::Dark),
        ColorScheme::Dark
    );
}

#[test]
fn theme_preference_should_persist_across_config_reload() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open session");
    session
        .save_config(&RoninConfig {
            theme: ThemePreference::Light,
            ..RoninConfig::default()
        })
        .expect("save config");

    let reloaded = RoninSession::open(paths).expect("reopen session");
    let config = reloaded.load_config().expect("load config");
    assert_eq!(config.theme, ThemePreference::Light);
}

#[test]
fn switching_theme_in_config_should_apply_on_reload() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();
    std::fs::write(paths.config_dir.join("config.toml"), r#"theme = "dark""#).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open session");
    assert_eq!(
        session.load_config().expect("load").theme,
        ThemePreference::Dark
    );

    std::fs::write(paths.config_dir.join("config.toml"), r#"theme = "light""#).unwrap();
    assert_eq!(
        session.load_config().expect("reload").theme,
        ThemePreference::Light
    );
}
