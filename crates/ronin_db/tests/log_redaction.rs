//! Secret/prompt/payload/URL redaction for log output.

use ronin_db::{redact_log_text, REDACTED_PLACEHOLDER};

#[test]
fn redact_should_strip_api_keys_and_bearer_tokens() {
    let input = "Authorization: Bearer sk-abc123XYZ_secret key=sk-proj-deadbeef";
    let out = redact_log_text(input);
    assert!(!out.contains("sk-abc123XYZ_secret"), "{out}");
    assert!(!out.contains("sk-proj-deadbeef"), "{out}");
    assert!(!out.contains("Bearer sk-"), "{out}");
    assert!(out.contains(REDACTED_PLACEHOLDER), "{out}");
}

#[test]
fn redact_should_strip_password_token_and_secret_assignments() {
    let input = "password=hunter2 api_key=supersecret token=tok_123 secret=shh";
    let out = redact_log_text(input);
    assert!(!out.contains("hunter2"), "{out}");
    assert!(!out.contains("supersecret"), "{out}");
    assert!(!out.contains("tok_123"), "{out}");
    assert!(!out.contains("shh"), "{out}");
    assert!(out.contains(REDACTED_PLACEHOLDER), "{out}");
}

#[test]
fn redact_should_strip_prompt_and_message_content_fields() {
    let input =
        r#"prompt="Tell me your secrets" content="User message body" message="hello there""#;
    let out = redact_log_text(input);
    assert!(!out.contains("Tell me your secrets"), "{out}");
    assert!(!out.contains("User message body"), "{out}");
    assert!(!out.contains("hello there"), "{out}");
    assert!(out.contains(REDACTED_PLACEHOLDER), "{out}");
}

#[test]
fn redact_should_strip_raw_provider_json_payloads() {
    let input = r#"provider payload: {"model":"gpt","messages":[{"role":"user","content":"hi"}],"choices":[{"text":"yo"}]}"#;
    let out = redact_log_text(input);
    assert!(!out.contains(r#""messages""#), "{out}");
    assert!(!out.contains(r#""choices""#), "{out}");
    assert!(!out.contains("\"hi\""), "{out}");
    assert!(out.contains(REDACTED_PLACEHOLDER), "{out}");
}

#[test]
fn redact_should_strip_credentials_from_urls() {
    let input = "fetch https://user:p@ss@api.example.com/v1?api_key=sk-live-999&token=abc";
    let out = redact_log_text(input);
    assert!(!out.contains("user:p@ss"), "{out}");
    assert!(!out.contains("sk-live-999"), "{out}");
    assert!(!out.contains("token=abc"), "{out}");
    assert!(
        out.contains(REDACTED_PLACEHOLDER) || out.contains("api.example.com"),
        "{out}"
    );
}

#[test]
fn redact_should_preserve_safe_diagnostic_text() {
    let input = "ronin shell selected thread thread_id=019f-test provider=ollama model=llama3.2";
    let out = redact_log_text(input);
    assert_eq!(out, input);
}
