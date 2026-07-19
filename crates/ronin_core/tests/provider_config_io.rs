//! Provider config import/export (portable TOML, no secrets).

use ronin_core::{
    export_provider_config_toml, import_provider_config_toml, GeneralConfig, OllamaConfig,
    OpenAiConfig, RoninConfig, RoninPaths, RoninSession,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();
    RoninSession::open(paths).expect("open session")
}

#[test]
fn export_provider_config_should_include_base_urls_and_model_selection() {
    let config = RoninConfig {
        general: GeneralConfig {
            default_provider: Some("ollama".into()),
            default_model: Some("llama3.2".into()),
            auto_title: true,
        },
        ollama: OllamaConfig {
            base_url: "http://192.168.1.50:11434".into(),
        },
        openai: Some(OpenAiConfig {
            base_url: Some("https://api.example.com/v1".into()),
        }),
        ..RoninConfig::default()
    };

    let toml = export_provider_config_toml(&config).expect("export");

    assert!(toml.contains("llama3.2"), "export should include default model");
    assert!(
        toml.contains("http://192.168.1.50:11434"),
        "export should include ollama base URL"
    );
    assert!(
        toml.contains("https://api.example.com/v1"),
        "export should include openai base URL"
    );
    assert!(
        toml.contains("ollama"),
        "export should include default provider"
    );
}

#[test]
fn export_provider_config_should_exclude_secrets() {
    let config = RoninConfig {
        openai: Some(OpenAiConfig {
            base_url: Some("https://api.openai.com/v1".into()),
        }),
        ..RoninConfig::default()
    };

    let toml = export_provider_config_toml(&config).expect("export");
    let lower = toml.to_lowercase();

    assert!(
        !lower.contains("api_key")
            && !lower.contains("apikey")
            && !lower.contains("secret")
            && !lower.contains("token")
            && !lower.contains("password")
            && !lower.contains("bearer"),
        "export must not contain secret field names: {toml}"
    );
}

#[test]
fn import_provider_config_should_restore_settings_into_live_config() {
    let temp = TempDir::new().expect("temp");
    let session = open_session(&temp);

    session
        .save_config(&RoninConfig {
            ollama: OllamaConfig {
                base_url: "http://localhost:11434".into(),
            },
            ..RoninConfig::default()
        })
        .expect("seed");

    let imported = r#"
[general]
default_provider = "openai"
default_model = "gpt-4o"

[ollama]
base_url = "http://10.0.0.2:11434"

[openai]
base_url = "https://api.openai.com/v1"
"#;

    let merged = import_provider_config_toml(&session.load_config().unwrap(), imported)
        .expect("import");
    session.save_config(&merged).expect("save");

    let loaded = session.load_config().expect("reload");
    assert_eq!(loaded.general.default_provider.as_deref(), Some("openai"));
    assert_eq!(loaded.general.default_model.as_deref(), Some("gpt-4o"));
    assert_eq!(loaded.ollama.base_url, "http://10.0.0.2:11434");
    assert_eq!(
        loaded.openai.as_ref().and_then(|o| o.base_url.as_deref()),
        Some("https://api.openai.com/v1")
    );
}

#[test]
fn import_provider_config_should_report_clear_error_for_invalid_toml() {
    let current = RoninConfig::default();
    let err = import_provider_config_toml(&current, "[[[not valid")
        .expect_err("invalid toml must fail");
    let lower = err.to_lowercase();
    assert!(
        lower.contains("parse") || lower.contains("invalid") || lower.contains("toml"),
        "error should be actionable: {err}"
    );
}

#[test]
fn import_provider_config_should_reject_empty_ollama_base_url() {
    let current = RoninConfig::default();
    let err = import_provider_config_toml(
        &current,
        r#"
[ollama]
base_url = ""
"#,
    )
    .expect_err("empty base_url must fail");
    assert!(
        err.to_lowercase().contains("base_url"),
        "error should mention base_url: {err}"
    );
}

#[test]
fn session_should_export_and_import_provider_config_files() {
    let temp = TempDir::new().expect("temp");
    let session = open_session(&temp);

    session
        .save_config(&RoninConfig {
            general: GeneralConfig {
                default_provider: Some("ollama".into()),
                default_model: Some("mistral".into()),
                auto_title: true,
            },
            ollama: OllamaConfig {
                base_url: "http://export-host:11434".into(),
            },
            ..RoninConfig::default()
        })
        .expect("seed");

    let export_path = temp.path().join("provider-export.toml");
    session
        .export_provider_config_to_file(&export_path)
        .expect("export file");

    let exported = std::fs::read_to_string(&export_path).expect("read export");
    assert!(exported.contains("mistral"));
    assert!(exported.contains("http://export-host:11434"));
    assert!(!exported.to_lowercase().contains("api_key"));

    // Reset local config, then import.
    session
        .save_config(&RoninConfig::default())
        .expect("reset");
    session
        .import_provider_config_from_file(&export_path)
        .expect("import file");

    let loaded = session.load_config().expect("load");
    assert_eq!(loaded.general.default_model.as_deref(), Some("mistral"));
    assert_eq!(loaded.ollama.base_url, "http://export-host:11434");
}
