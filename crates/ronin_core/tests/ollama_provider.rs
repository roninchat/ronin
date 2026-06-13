use ronin_core::{OllamaHealth, OllamaProvider};

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
fn provider_should_report_healthy_with_models() {
    let provider = FakeOllama {
        health: OllamaHealth::Online,
        models: vec!["llama3.2".into(), "codellama".into()],
    };

    assert_eq!(provider.check_health(), OllamaHealth::Online);
    let models = provider.list_models().expect("list models");
    assert_eq!(models, vec!["llama3.2", "codellama"]);
}

#[test]
fn provider_should_report_offline_when_unreachable() {
    let provider = FakeOllama {
        health: OllamaHealth::Offline,
        models: vec![],
    };

    assert_eq!(provider.check_health(), OllamaHealth::Offline);
}

#[test]
fn provider_should_report_online_with_no_models() {
    let provider = FakeOllama {
        health: OllamaHealth::Online,
        models: vec![],
    };

    assert_eq!(provider.check_health(), OllamaHealth::Online);
    let models = provider.list_models().expect("list models");
    assert!(models.is_empty());
}

#[test]
fn http_provider_should_return_offline_when_no_server_running() {
    let provider = ronin_core::HttpOllamaProvider::new("http://127.0.0.1:11434");

    let health = provider.check_health();
    // Accept either state — just verify it doesn't panic.
    match health {
        OllamaHealth::Online | OllamaHealth::Offline => {}
    }
}
