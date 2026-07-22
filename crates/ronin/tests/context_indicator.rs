//! Public seams for composer context/token size estimation and indicator presentation.

use ronin::context_indicator::{
    estimate_tokens_from_chars, fill_level_for_ratio, format_token_count,
    project_context_indicator, resolve_model_context_window, ContextEstimateInput,
    ContextFillLevel,
};
use ronin_app::{MAX_CHARS, MAX_MESSAGES};

#[test]
fn estimate_tokens_from_chars_should_use_four_chars_per_token() {
    assert_eq!(estimate_tokens_from_chars(0), 0);
    assert_eq!(estimate_tokens_from_chars(1), 1);
    assert_eq!(estimate_tokens_from_chars(4), 1);
    assert_eq!(estimate_tokens_from_chars(4000), 1000);
    assert_eq!(estimate_tokens_from_chars(4001), 1001);
}

#[test]
fn fill_level_should_shift_comfortable_elevated_critical() {
    assert_eq!(fill_level_for_ratio(0.0), ContextFillLevel::Comfortable);
    assert_eq!(fill_level_for_ratio(0.59), ContextFillLevel::Comfortable);
    assert_eq!(fill_level_for_ratio(0.60), ContextFillLevel::Elevated);
    assert_eq!(fill_level_for_ratio(0.84), ContextFillLevel::Elevated);
    assert_eq!(fill_level_for_ratio(0.85), ContextFillLevel::Critical);
    assert_eq!(fill_level_for_ratio(1.2), ContextFillLevel::Critical);
}

#[test]
fn project_should_estimate_small_thread_without_omission() {
    let messages = vec!["Hello".to_string(), "Hi there".to_string()];
    let indicator = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: "Next question",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });

    assert!(!indicator.messages_omitted);
    assert!(indicator.estimated_tokens > 0);
    assert!(indicator.used_chars > 0);
    assert_eq!(indicator.level, ContextFillLevel::Comfortable);
    assert!(
        indicator.summary_label.contains('~') || indicator.summary_label.contains("token"),
        "label should show approx tokens: {}",
        indicator.summary_label
    );
    assert!(indicator.omission_label.is_none());
}

#[test]
fn project_should_flag_omission_when_message_cap_exceeded() {
    let messages: Vec<String> = (0..MAX_MESSAGES + 5)
        .map(|i| format!("message {i}"))
        .collect();
    let indicator = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: "overflow",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });

    assert!(indicator.messages_omitted);
    assert_eq!(
        indicator.omission_label,
        Some("Older messages will be omitted")
    );
}

#[test]
fn project_should_flag_omission_when_char_cap_exceeded() {
    let long = "A".repeat(MAX_CHARS / 2 + 100);
    let messages = vec![long.clone(), long];
    let indicator = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: "",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });

    assert!(indicator.messages_omitted);
    assert!(
        matches!(
            indicator.level,
            ContextFillLevel::Elevated | ContextFillLevel::Critical
        ),
        "omission should raise urgency, got {:?}",
        indicator.level
    );
    assert_eq!(
        indicator.omission_label,
        Some("Older messages will be omitted")
    );
}

#[test]
fn project_should_include_composer_and_attachment_chars() {
    let messages = vec!["hi".to_string()];
    let base = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: "",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });
    let with_pending = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: &"x".repeat(400),
        attachment_chars: 400,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });

    assert!(with_pending.used_chars > base.used_chars);
    assert!(with_pending.estimated_tokens > base.estimated_tokens);
}

#[test]
fn project_should_show_model_window_when_known() {
    let messages = vec!["hello".to_string()];
    let indicator = project_context_indicator(ContextEstimateInput {
        message_contents: &messages,
        composer_text: "world",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: Some("gpt-4o"),
        max_messages: MAX_MESSAGES,
        max_chars: MAX_CHARS,
    });

    let window = resolve_model_context_window("gpt-4o").expect("known model");
    assert_eq!(indicator.limit_tokens, Some(window));
    assert!(
        indicator
            .summary_label
            .contains(&format_token_count(window))
            || indicator.summary_label.contains('/'),
        "should show limit in label: {}",
        indicator.summary_label
    );
}

#[test]
fn resolve_model_context_window_should_cover_ollama_and_openai_names() {
    assert!(resolve_model_context_window("gpt-4o").is_some());
    assert!(resolve_model_context_window("gpt-4o-mini").is_some());
    assert!(resolve_model_context_window("llama3.2").is_some());
    assert!(resolve_model_context_window("llama3.2:latest").is_some());
    assert!(resolve_model_context_window("qwen2.5").is_some());
    assert!(resolve_model_context_window("totally-unknown-model-xyz").is_none());
}

#[test]
fn fill_ratio_should_rise_toward_critical_as_context_grows() {
    let small = project_context_indicator(ContextEstimateInput {
        message_contents: &["hi".into()],
        composer_text: "",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: 1_000, // tight cap for the test
    });
    let large_msg = "B".repeat(900);
    let large = project_context_indicator(ContextEstimateInput {
        message_contents: &[large_msg],
        composer_text: "",
        attachment_chars: 0,
        system_prompt_chars: 100,
        model_name: None,
        max_messages: MAX_MESSAGES,
        max_chars: 1_000,
    });

    assert!(large.fill_ratio > small.fill_ratio);
    assert_eq!(large.level, ContextFillLevel::Critical);
}
