use ronin_app::{ProviderStatus, RoninShell, VisualReuseDecision};
use ronin_core::{MessageRole, OllamaHealth, OllamaProvider, RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn shell_should_create_usable_selected_thread_when_none_exists() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let shell = RoninShell::open(paths).expect("open shell");
    let state = shell.state();

    assert_eq!(state.threads.len(), 1);
    assert_eq!(state.threads[0].title, "New Chat");
    assert_eq!(
        state.selected_thread_id.as_deref(),
        Some(state.threads[0].id.as_str())
    );
}

#[test]
fn shell_should_create_and_select_new_thread_from_sidebar_action() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths).expect("open shell");
    let initial_thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("initial selected thread");

    let created = shell.create_new_thread().expect("create new thread");

    assert_ne!(created.id, initial_thread_id);
    assert_eq!(shell.state().threads.len(), 2);
    assert_eq!(
        shell.state().selected_thread_id.as_deref(),
        Some(created.id.as_str())
    );
}

#[test]
fn shell_should_create_and_select_new_empty_thread_on_new_launch() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths.clone()).expect("open setup session");
    let existing = session.create_thread().expect("create existing thread");
    drop(session);

    let shell = RoninShell::open_with_new_thread(paths).expect("open shell with new thread");
    let state = shell.state();
    let selected_id = state
        .selected_thread_id
        .as_deref()
        .expect("selected thread id");
    let selected = state
        .threads
        .iter()
        .find(|thread| thread.id == selected_id)
        .expect("selected thread exists");

    assert_eq!(state.threads.len(), 2);
    assert!(state.threads.iter().any(|thread| thread == &existing));
    assert_ne!(selected.id, existing.id);
    assert_eq!(selected.title, "New Chat");
}

#[test]
fn shell_should_select_thread_from_sidebar_action() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths).expect("open shell");
    let created = shell.create_new_thread().expect("create new thread");
    let first_id = shell.state().threads[0].id.clone();

    shell.select_thread(&first_id).expect("select first thread");

    assert_eq!(
        shell.state().selected_thread_id.as_deref(),
        Some(first_id.as_str())
    );
    assert_ne!(
        shell.state().selected_thread_id.as_deref(),
        Some(created.id.as_str())
    );
}

#[test]
fn shell_should_restore_persisted_threads_and_select_first_thread() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths.clone()).expect("open setup session");
    let first = session.create_thread().expect("create first thread");
    let second = session.create_thread().expect("create second thread");
    drop(session);

    let shell = RoninShell::open(paths).expect("open shell");
    let state = shell.state();

    assert_eq!(state.window_title, "Ronin");
    assert_eq!(state.threads, vec![first, second]);
    assert_eq!(
        state.selected_thread_id.as_deref(),
        Some(state.threads[0].id.as_str())
    );
    assert_eq!(state.provider_status, ProviderStatus::NotConfigured);
}

#[test]
fn shell_should_expose_m0_visual_direction_for_design_checkpoint() {
    let direction = RoninShell::m0_visual_direction();

    assert_eq!(
        direction.assessment_axes,
        ["rounded", "soft", "premium", "Linux-native", "Zed-grade"]
    );
    assert!(direction.required_changes_before_deeper_ui.is_empty());
    assert!(matches!(
        direction.reuse_decision,
        VisualReuseDecision::CustomGpui { .. }
    ));
}

#[test]
fn shell_should_set_ollama_offline_when_opened_with_ollama_provider() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let provider = FakeOllama {
        health: ronin_core::OllamaHealth::Offline,
        models: vec![],
    };
    let shell = RoninShell::open_with_ollama_provider(paths, provider).expect("open shell");
    let state = shell.state();

    assert_eq!(state.provider_status, ProviderStatus::OllamaOffline);
}

struct FakeOllama {
    health: OllamaHealth,
    models: Vec<String>,
}

impl OllamaProvider for FakeOllama {
    fn check_health(&self) -> OllamaHealth {
        self.health.clone()
    }

    fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
        Ok(self.models.clone())
    }
}

#[test]
fn shell_should_select_first_model_when_ollama_online_with_models() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let provider = FakeOllama {
        health: OllamaHealth::Online,
        models: vec!["llama3.2".into(), "codellama".into()],
    };

    let shell =
        RoninShell::open_with_ollama_provider(paths, provider).expect("open shell with ollama");
    let state = shell.state();

    assert_eq!(
        state.provider_status,
        ProviderStatus::OllamaOnline {
            model: "llama3.2".into()
        }
    );
}

#[test]
fn shell_should_show_no_models_when_ollama_online_but_empty() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let provider = FakeOllama {
        health: OllamaHealth::Online,
        models: vec![],
    };

    let shell =
        RoninShell::open_with_ollama_provider(paths, provider).expect("open shell with ollama");
    let state = shell.state();

    assert_eq!(state.provider_status, ProviderStatus::OllamaNoModels);
}

#[test]
fn shell_should_restore_previously_selected_model_from_config() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    // First open: select codellama (second model)
    {
        let provider = FakeOllama {
            health: OllamaHealth::Online,
            models: vec!["llama3.2".into(), "codellama".into()],
        };
        let mut shell =
            RoninShell::open_with_ollama_provider(paths.clone(), provider).expect("first open");
        let state = shell.state();
        assert_eq!(
            state.provider_status,
            ProviderStatus::OllamaOnline {
                model: "llama3.2".into()
            }
        );
        // Select codellama explicitly
        shell.select_model("codellama").expect("select codellama");
        drop(shell);
    }

    // Re-open: should restore previously selected model from config
    {
        let provider = FakeOllama {
            health: OllamaHealth::Online,
            models: vec!["llama3.2".into(), "codellama".into()],
        };
        let shell = RoninShell::open_with_ollama_provider(paths, provider).expect("re-open");
        let state = shell.state();
        assert_eq!(
            state.provider_status,
            ProviderStatus::OllamaOnline {
                model: "codellama".into()
            }
        );
    }
}

#[test]
fn shell_should_report_openai_ready_when_online_with_models() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    struct FakeOpenAi;
    impl OllamaProvider for FakeOpenAi {
        fn name(&self) -> &'static str {
            "openai"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Online
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Ok(vec!["gpt-4o".into()])
        }
    }

    let shell = RoninShell::open_with_ollama_provider(paths, FakeOpenAi).expect("open shell");
    let state = shell.state();

    assert_eq!(
        state.provider_status,
        ProviderStatus::OpenAiReady {
            model: "gpt-4o".into()
        }
    );
}

#[test]
fn shell_should_report_openai_not_configured_when_no_api_key() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    struct FakeOpenAiNoKey;
    impl OllamaProvider for FakeOpenAiNoKey {
        fn name(&self) -> &'static str {
            "openai"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Offline
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Err(ronin_core::RoninError::Provider(
                "No API key found. Set OPENAI_API_KEY or add a key in settings.".into(),
            ))
        }
    }

    let shell = RoninShell::open_with_ollama_provider(paths, FakeOpenAiNoKey).expect("open shell");
    let state = shell.state();

    assert_eq!(state.provider_status, ProviderStatus::OpenAiNotConfigured);
}

#[test]
fn shell_should_report_openai_error_when_list_models_fails_with_other_error() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    struct FakeOpenAiError;
    impl OllamaProvider for FakeOpenAiError {
        fn name(&self) -> &'static str {
            "openai"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Offline
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Err(ronin_core::RoninError::Provider(
                "API Connection timeout".into(),
            ))
        }
    }

    let shell = RoninShell::open_with_ollama_provider(paths, FakeOpenAiError).expect("open shell");
    let state = shell.state();

    assert_eq!(
        state.provider_status,
        ProviderStatus::OpenAiError {
            message: "Could not reach the OpenAI-compatible endpoint. Check the base URL in provider settings and your network connection.".into()
        }
    );
}

#[test]
fn shell_should_resolve_thread_provider_and_model_falling_back_to_config_defaults() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    std::fs::create_dir_all(&paths.config_dir).unwrap();
    let toml_content = r#"
[general]
default_provider = "openai"
default_model = "gpt-4o"
"#;
    std::fs::write(paths.config_dir.join("config.toml"), toml_content).unwrap();

    let mut shell = RoninShell::open(paths.clone()).expect("open shell");

    let thread1 = shell.create_new_thread().expect("create thread 1");
    let (p1, m1) = shell
        .resolve_thread_provider_and_model(&thread1.id)
        .expect("resolve 1");
    assert_eq!(p1, "openai");
    assert_eq!(m1, "gpt-4o");

    let thread2 = shell.create_new_thread().expect("create thread 2");
    shell
        .session()
        .set_thread_provider(&thread2.id, "ollama")
        .unwrap();
    shell
        .session()
        .set_thread_model(&thread2.id, "llama3")
        .unwrap();

    // Create a new shell to load the threads with updated database fields
    let shell2 = RoninShell::open(paths.clone()).expect("open shell 2");

    let (p2, m2) = shell2
        .resolve_thread_provider_and_model(&thread2.id)
        .expect("resolve 2");
    assert_eq!(p2, "ollama");
    assert_eq!(m2, "llama3");
}

#[test]
fn shell_should_store_connection_test_result_on_state() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    struct Offline;
    impl OllamaProvider for Offline {
        fn name(&self) -> &'static str {
            "ollama"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Offline
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Err(ronin_core::RoninError::Provider(
                "connection refused".into(),
            ))
        }
    }

    let mut shell = RoninShell::open(paths).expect("open shell");
    assert!(shell.state().connection_test.is_none());

    let result = shell.record_connection_test(&Offline);
    assert!(!result.is_success());
    let stored = shell
        .state()
        .connection_test
        .as_ref()
        .expect("connection test stored");
    assert_eq!(stored, &result);
    assert!(stored.message().to_lowercase().contains("ollama"));
}

#[test]
fn shell_should_refresh_provider_status_based_on_thread_settings() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    std::fs::create_dir_all(&paths.config_dir).unwrap();
    let toml_content = r#"
[general]
default_provider = "openai"
default_model = "gpt-4o"
"#;
    std::fs::write(paths.config_dir.join("config.toml"), toml_content).unwrap();

    let mut shell = RoninShell::open(paths).expect("open shell");
    assert_eq!(shell.state().provider_status, ProviderStatus::NotConfigured);

    shell.refresh_provider_status().unwrap();
    assert_eq!(
        shell.state().provider_status,
        ProviderStatus::OpenAiNotConfigured
    );
}

#[test]
fn shell_artifact_crud() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open shell");

    let thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("selected thread");
    let msg = shell
        .session()
        .create_message(&thread_id, MessageRole::User, "hello")
        .expect("create message");

    // Create
    let artifact = shell
        .create_artifact(&thread_id, &msg.id, "My Artifact", "content")
        .expect("create artifact");
    assert_eq!(artifact.title, "My Artifact");
    assert_eq!(artifact.content, "content");

    // List all
    let all = shell.list_all_artifacts().expect("list all artifacts");
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, artifact.id);

    // Delete
    shell
        .delete_artifact(&artifact.id)
        .expect("delete artifact");
    let after = shell.list_all_artifacts().expect("list after delete");
    assert!(after.is_empty());
}

#[test]
fn shell_should_rename_and_edit_artifact_and_list_reflects_change() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open shell");

    let thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("selected thread");
    let msg = shell
        .session()
        .create_message(&thread_id, MessageRole::User, "hello")
        .expect("create message");
    let artifact = shell
        .create_artifact(&thread_id, &msg.id, "Draft", "v1")
        .expect("create artifact");

    shell
        .update_artifact(&artifact.id, "Final Title", "v2 body")
        .expect("update artifact");

    let listed = shell.list_all_artifacts().expect("list after update");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].title, "Final Title");
    assert_eq!(listed[0].content, "v2 body");
}
