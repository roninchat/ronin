use ronin_core::{RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn thread_struct_should_contain_provider_and_model() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let thread = session.create_thread().expect("create thread");

    // By default they should be None
    assert_eq!(thread.provider, None);
    assert_eq!(thread.model, None);
}

#[test]
fn load_config_should_load_general_settings_when_present() {
    let temp = TempDir::new().expect("temp dir");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let toml_content = r#"
[general]
default_provider = "openai"
default_model = "gpt-4o"
"#;
    std::fs::write(config_dir.join("config.toml"), toml_content).unwrap();

    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let config = session.load_config().expect("load config");

    assert_eq!(config.general.default_provider.as_deref(), Some("openai"));
    assert_eq!(config.general.default_model.as_deref(), Some("gpt-4o"));
}

#[test]
fn load_config_should_load_ollama_settings_when_present() {
    let temp = TempDir::new().expect("temp dir");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let toml_content = r#"
[ollama]
base_url = "http://192.168.1.100:11434"
"#;
    std::fs::write(config_dir.join("config.toml"), toml_content).unwrap();

    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let config = session.load_config().expect("load config");

    assert_eq!(config.ollama.base_url, "http://192.168.1.100:11434");
}

#[test]
fn load_config_should_use_sensible_defaults_when_missing() {
    let temp = TempDir::new().expect("temp dir");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Write empty toml config
    std::fs::write(config_dir.join("config.toml"), "").unwrap();

    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let config = session.load_config().expect("load config");

    assert_eq!(config.general.default_provider, None);
    assert_eq!(config.general.default_model, None);
    assert_eq!(config.ollama.base_url, "http://localhost:11434");
}

#[test]
fn create_thread_should_inherit_global_defaults_when_configured() {
    let temp = TempDir::new().expect("temp dir");
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let toml_content = r#"
[general]
default_provider = "ollama"
default_model = "llama3"
"#;
    std::fs::write(config_dir.join("config.toml"), toml_content).unwrap();

    let paths = RoninPaths {
        config_dir,
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let thread = session.create_thread().expect("create thread");

    assert_eq!(thread.provider.as_deref(), Some("ollama"));
    assert_eq!(thread.model.as_deref(), Some("llama3"));
}

#[test]
fn set_thread_provider_should_update_and_persist_provider() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let thread = session.create_thread().expect("create thread");

    session
        .set_thread_provider(&thread.id, "openai")
        .expect("set provider");

    let threads = session.list_threads().expect("list threads");
    assert_eq!(threads[0].provider.as_deref(), Some("openai"));
}

#[test]
fn set_thread_model_should_update_and_persist_model() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths).expect("open session");
    let thread = session.create_thread().expect("create thread");

    session
        .set_thread_model(&thread.id, "gpt-4o")
        .expect("set model");

    let threads = session.list_threads().expect("list threads");
    assert_eq!(threads[0].model.as_deref(), Some("gpt-4o"));
}

#[test]
fn save_and_load_selected_model_should_use_config_toml_instead_of_json() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    std::fs::create_dir_all(&paths.config_dir).unwrap();

    let initial_toml = r#"
[ollama]
base_url = "http://localhost:11434"
"#;
    std::fs::write(paths.config_dir.join("config.toml"), initial_toml).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open session");

    assert_eq!(session.load_selected_model().unwrap(), None);

    session.save_selected_model("llama3.2").unwrap();

    assert_eq!(
        session.load_selected_model().unwrap().as_deref(),
        Some("llama3.2")
    );

    let toml_data = std::fs::read_to_string(paths.config_dir.join("config.toml")).unwrap();
    assert!(toml_data.contains("default_model = \"llama3.2\""));
    assert!(toml_data.contains("base_url = \"http://localhost:11434\""));

    assert!(!paths.config_dir.join("ronin_config.json").is_file());
}
