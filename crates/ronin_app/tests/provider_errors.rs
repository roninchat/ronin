//! Provider user-facing error messaging and connection test seams.

use ronin_app::format_provider_error;

#[test]
fn missing_api_key_error_should_include_setup_instructions() {
    let msg = format_provider_error(
        "openai",
        "No API key found. Set OPENAI_API_KEY or add a key in settings.",
    );
    let lower = msg.to_lowercase();
    assert!(lower.contains("api key"));
    assert!(
        lower.contains("openai_api_key") || lower.contains("settings"),
        "should tell user how to configure: {msg}"
    );
    assert!(
        !msg.contains("401") && !msg.contains("stack"),
        "must not expose raw HTTP/status dumps: {msg}"
    );
}

#[test]
fn ollama_offline_error_should_include_start_instructions() {
    let samples = [
        "error sending request for url (http://localhost:11434/api/tags)",
        "connection refused",
        "ollama returned 502: Bad Gateway",
        "tcp connect error",
    ];
    for raw in samples {
        let msg = format_provider_error("ollama", raw);
        let lower = msg.to_lowercase();
        assert!(
            lower.contains("ollama")
                && (lower.contains("not running") || lower.contains("offline")),
            "expected offline guidance for `{raw}`, got: {msg}"
        );
        assert!(
            lower.contains("start") || lower.contains("install") || lower.contains("ollama serve"),
            "should include start/install instructions: {msg}"
        );
        assert!(
            !msg.contains("502") && !msg.contains("tcp connect"),
            "must not expose raw transport detail: {msg}"
        );
    }
}

#[test]
fn openai_unreachable_error_should_be_actionable_without_ollama_copy() {
    let msg = format_provider_error(
        "openai",
        "error sending request for url (https://api.openai.com/v1/models): connection refused",
    );
    let lower = msg.to_lowercase();
    assert!(lower.contains("reach") || lower.contains("connect") || lower.contains("endpoint"));
    assert!(!lower.contains("ollama serve"));
    assert!(!msg.contains("connection refused"));
}

#[test]
fn invalid_model_error_should_suggest_checking_or_pulling() {
    let samples = [
        ("ollama", "model 'llama3.9' not found"),
        ("ollama", "ollama returned 404: {\"error\":\"model 'foo' not found\"}"),
        ("openai", "openai returned 404: The model `gpt-9` does not exist"),
        ("openai", "invalid_request_error: model_not_found"),
    ];
    for (provider, raw) in samples {
        let msg = format_provider_error(provider, raw);
        let lower = msg.to_lowercase();
        assert!(
            lower.contains("model") && lower.contains("not found"),
            "expected model-not-found for `{raw}`, got: {msg}"
        );
        assert!(
            lower.contains("check") || lower.contains("pull") || lower.contains("name"),
            "should suggest checking name or pulling: {msg}"
        );
        assert!(!msg.contains("404"), "must not expose raw status: {msg}");
    }
}

#[test]
fn rate_limit_error_should_show_retry_timing_when_available() {
    let with_retry = format_provider_error(
        "openai",
        "openai returned 429: Rate limit exceeded. Please retry after 12 seconds.",
    );
    let lower = with_retry.to_lowercase();
    assert!(lower.contains("rate limit") || lower.contains("rate limited"));
    assert!(
        lower.contains("12") && lower.contains("second"),
        "should surface retry timing: {with_retry}"
    );
    assert!(!with_retry.contains("429"));

    let without_retry = format_provider_error("openai", "Too Many Requests");
    let lower = without_retry.to_lowercase();
    assert!(lower.contains("rate limit") || lower.contains("rate limited"));
    assert!(lower.contains("try again") || lower.contains("retry"));
}

#[test]
fn context_too_large_error_should_explain_limit_and_suggest_reducing() {
    let samples = [
        "context length exceeded",
        "maximum context length is 8192 tokens",
        "prompt is too long",
        "this model's maximum context length is 128000 tokens",
        "ollama returned 400: {\"error\":\"the input length exceeds the context window\"}",
    ];
    for raw in samples {
        let msg = format_provider_error("ollama", raw);
        let lower = msg.to_lowercase();
        assert!(
            lower.contains("too long") || lower.contains("context"),
            "expected context guidance for `{raw}`, got: {msg}"
        );
        assert!(
            lower.contains("reduc") || lower.contains("shorter") || lower.contains("fewer"),
            "should suggest reducing context: {msg}"
        );
        assert!(!msg.contains("400"), "must not expose raw status: {msg}");
    }
}

#[test]
fn connection_test_should_succeed_when_provider_is_reachable_and_authenticated() {
    use ronin_app::{run_connection_test, ConnectionTestResult};
    use ronin_core::{OllamaHealth, OllamaProvider};

    struct OkProvider;
    impl OllamaProvider for OkProvider {
        fn name(&self) -> &'static str {
            "ollama"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Online
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Ok(vec!["llama3.2".into()])
        }
    }

    let result = run_connection_test(&OkProvider);
    match result {
        ConnectionTestResult::Success { message } => {
            let lower = message.to_lowercase();
            assert!(lower.contains("success") || lower.contains("connected"));
            assert!(lower.contains("ollama") || lower.contains("1 model") || lower.contains("model"));
        }
        ConnectionTestResult::Failure { message } => {
            panic!("expected success, got failure: {message}");
        }
    }
}

#[test]
fn connection_test_should_fail_with_actionable_message_when_ollama_offline() {
    use ronin_app::{run_connection_test, ConnectionTestResult};
    use ronin_core::{OllamaHealth, OllamaProvider};

    struct Offline;
    impl OllamaProvider for Offline {
        fn name(&self) -> &'static str {
            "ollama"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Offline
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Err(ronin_core::RoninError::Provider("connection refused".into()))
        }
    }

    match run_connection_test(&Offline) {
        ConnectionTestResult::Failure { message } => {
            let lower = message.to_lowercase();
            assert!(lower.contains("ollama"));
            assert!(lower.contains("not running") || lower.contains("offline"));
            assert!(lower.contains("start") || lower.contains("install") || lower.contains("serve"));
        }
        ConnectionTestResult::Success { message } => {
            panic!("expected failure, got success: {message}");
        }
    }
}

#[test]
fn connection_test_should_fail_when_openai_missing_api_key() {
    use ronin_app::{run_connection_test, ConnectionTestResult};
    use ronin_core::{OllamaHealth, OllamaProvider};

    struct NoKey;
    impl OllamaProvider for NoKey {
        fn name(&self) -> &'static str {
            "openai"
        }
        fn check_health(&self) -> OllamaHealth {
            OllamaHealth::Offline
        }
        fn list_models(&self) -> Result<Vec<String>, ronin_core::RoninError> {
            Err(ronin_core::RoninError::Config(
                "No API key found. Set OPENAI_API_KEY or add a key in settings.".into(),
            ))
        }
    }

    match run_connection_test(&NoKey) {
        ConnectionTestResult::Failure { message } => {
            let lower = message.to_lowercase();
            assert!(lower.contains("api key"));
            assert!(lower.contains("openai_api_key") || lower.contains("settings"));
        }
        ConnectionTestResult::Success { message } => {
            panic!("expected failure, got success: {message}");
        }
    }
}
