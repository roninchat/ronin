//! Composer context/token size estimation and indicator presentation.
//!
//! Public seams for approximate token counts, fill levels, and omission
//! messaging — testable without GPUI pixels.

use gpui::Hsla;
use ronin_core::ColorScheme;

/// Visual urgency band for the context indicator (green → yellow → red).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextFillLevel {
    /// Plenty of room remaining (< 60% of the displayed limit).
    Comfortable,
    /// Getting full (60–85%).
    Elevated,
    /// Near or over the limit (≥ 85%), or omission is imminent.
    Critical,
}

/// Inputs used to project the next provider request's context size.
#[derive(Debug, Clone, Copy)]
pub struct ContextEstimateInput<'a> {
    /// Thread message bodies in chronological order (oldest first).
    pub message_contents: &'a [String],
    /// Current composer draft text (not yet sent).
    pub composer_text: &'a str,
    /// Approximate character size of pending attachment context blocks.
    pub attachment_chars: usize,
    /// Character length of the effective system prompt that will be sent.
    pub system_prompt_chars: usize,
    /// Active model name, when known (used to resolve context windows).
    pub model_name: Option<&'a str>,
    /// Max conversation messages included in a request (app cap).
    pub max_messages: usize,
    /// Max conversation characters included in a request (app cap).
    pub max_chars: usize,
}

/// Presentation model for the composer context/token indicator.
#[derive(Debug, Clone, PartialEq)]
pub struct ContextIndicator {
    /// Approximate tokens that would be included in the next request.
    pub estimated_tokens: usize,
    /// Characters counted toward the estimate (system + included history + pending).
    pub used_chars: usize,
    /// Model context window in tokens when known.
    pub limit_tokens: Option<usize>,
    /// Effective character budget used for fill ratio (app cap or window-derived).
    pub limit_chars: usize,
    /// `used_chars / limit_chars` (may exceed 1.0).
    pub fill_ratio: f32,
    /// Color-band urgency.
    pub level: ContextFillLevel,
    /// True when older history would be dropped by message/char caps.
    pub messages_omitted: bool,
    /// Compact label shown near the composer (e.g. `~1.2k / 128k tokens`).
    pub summary_label: String,
    /// Optional omission hint when history will be truncated.
    pub omission_label: Option<&'static str>,
}

/// Rough token estimate: ~4 characters per token (ceiling).
pub fn estimate_tokens_from_chars(chars: usize) -> usize {
    chars.div_ceil(4)
}

/// Maps a fill ratio to the green → yellow → red urgency band.
pub fn fill_level_for_ratio(ratio: f32) -> ContextFillLevel {
    if ratio >= 0.85 {
        ContextFillLevel::Critical
    } else if ratio >= 0.60 {
        ContextFillLevel::Elevated
    } else {
        ContextFillLevel::Comfortable
    }
}

/// Compact display for token counts (`850`, `1.2k`, `128k`).
pub fn format_token_count(tokens: usize) -> String {
    if tokens < 1_000 {
        tokens.to_string()
    } else if tokens < 10_000 {
        let tenths = (tokens + 50) / 100;
        if tenths % 10 == 0 {
            format!("{}k", tenths / 10)
        } else {
            format!("{}.{}k", tenths / 10, tenths % 10)
        }
    } else if tokens < 1_000_000 {
        format!("{}k", (tokens + 500) / 1_000)
    } else {
        format!("{}M", (tokens + 500_000) / 1_000_000)
    }
}

/// Resolves a known context window (tokens) for common Ollama / OpenAI model names.
///
/// Returns [`None`] when the window is unknown so callers can fall back to the
/// app character cap.
pub fn resolve_model_context_window(model_name: &str) -> Option<usize> {
    let name = model_name
        .split([':', '/', '@'])
        .next()
        .unwrap_or(model_name)
        .to_ascii_lowercase();

    // OpenAI-compatible
    if name.starts_with("gpt-4o") || name.starts_with("chatgpt-4o") {
        return Some(128_000);
    }
    if name.starts_with("gpt-4-turbo") || name.starts_with("gpt-4.1") {
        return Some(128_000);
    }
    if name.starts_with("gpt-4") {
        return Some(8_192);
    }
    if name.starts_with("gpt-3.5") {
        return Some(16_385);
    }
    if name.starts_with("o1") || name.starts_with("o3") || name.starts_with("o4") {
        return Some(200_000);
    }

    // Common local / Ollama families (approximate defaults)
    if name.starts_with("llama3.2") || name.starts_with("llama3.1") || name.starts_with("llama3") {
        return Some(128_000);
    }
    if name.starts_with("llama2") {
        return Some(4_096);
    }
    if name.starts_with("qwen2.5") || name.starts_with("qwen2") || name.starts_with("qwen") {
        return Some(32_768);
    }
    if name.starts_with("mistral") || name.starts_with("mixtral") {
        return Some(32_768);
    }
    if name.starts_with("gemma2") || name.starts_with("gemma") {
        return Some(8_192);
    }
    if name.starts_with("phi3") || name.starts_with("phi-3") || name.starts_with("phi4") {
        return Some(16_384);
    }
    if name.starts_with("deepseek") {
        return Some(64_000);
    }
    if name.starts_with("codellama") {
        return Some(16_384);
    }

    None
}

/// Theme-aware color for the indicator fill level (green → yellow → red).
pub fn fill_level_color(level: ContextFillLevel, scheme: ColorScheme) -> Hsla {
    match (level, scheme) {
        (ContextFillLevel::Comfortable, ColorScheme::Dark) => {
            gpui::hsla(120. / 360., 0.55, 0.65, 1.0) // soft green
        }
        (ContextFillLevel::Comfortable, ColorScheme::Light) => {
            gpui::hsla(130. / 360., 0.55, 0.38, 1.0)
        }
        (ContextFillLevel::Elevated, ColorScheme::Dark) => {
            gpui::hsla(45. / 360., 0.70, 0.65, 1.0) // amber
        }
        (ContextFillLevel::Elevated, ColorScheme::Light) => {
            gpui::hsla(40. / 360., 0.75, 0.42, 1.0)
        }
        (ContextFillLevel::Critical, ColorScheme::Dark) => {
            gpui::hsla(350. / 360., 0.65, 0.70, 1.0) // rose / red
        }
        (ContextFillLevel::Critical, ColorScheme::Light) => {
            gpui::hsla(350. / 360., 0.75, 0.45, 1.0)
        }
    }
}

fn count_included_history_chars(
    message_contents: &[String],
    pending_chars: usize,
    max_messages: usize,
    max_chars: usize,
) -> (usize, bool) {
    // Pending draft counts as the newest message toward both caps.
    let mut included_msgs = if pending_chars > 0 { 1usize } else { 0 };
    let mut total_chars = pending_chars;
    let mut omitted = pending_chars > max_chars;

    for content in message_contents.iter().rev() {
        if included_msgs >= max_messages {
            omitted = true;
            break;
        }
        let msg_chars = content.chars().count();
        if total_chars + msg_chars > max_chars {
            omitted = true;
            break;
        }
        total_chars += msg_chars;
        included_msgs += 1;
    }

    let history_included = included_msgs.saturating_sub(usize::from(pending_chars > 0));
    if history_included < message_contents.len() {
        omitted = true;
    }

    (total_chars, omitted)
}

/// Projects context usage for the next request and builds indicator presentation.
pub fn project_context_indicator(input: ContextEstimateInput<'_>) -> ContextIndicator {
    let system_chars = input.system_prompt_chars;
    let pending_chars = input.composer_text.chars().count() + input.attachment_chars;
    let (history_and_pending_chars, messages_omitted) = count_included_history_chars(
        input.message_contents,
        pending_chars,
        input.max_messages,
        input.max_chars,
    );

    let used_chars = system_chars + history_and_pending_chars;
    let estimated_tokens = estimate_tokens_from_chars(used_chars);

    let limit_tokens = input
        .model_name
        .and_then(resolve_model_context_window);
    let app_limit_tokens = estimate_tokens_from_chars(input.max_chars + system_chars);
    let display_limit_tokens = limit_tokens.unwrap_or(app_limit_tokens);
    let limit_chars = display_limit_tokens.saturating_mul(4).max(1);

    let fill_ratio = used_chars as f32 / limit_chars as f32;
    let mut level = fill_level_for_ratio(fill_ratio);
    if messages_omitted && level == ContextFillLevel::Comfortable {
        // Omission is a strong signal even when the model window is large.
        level = ContextFillLevel::Elevated;
    }

    let summary_label = format!(
        "~{} / {} tokens",
        format_token_count(estimated_tokens),
        format_token_count(display_limit_tokens)
    );

    let omission_label = if messages_omitted {
        Some("Older messages will be omitted")
    } else {
        None
    };

    ContextIndicator {
        estimated_tokens,
        used_chars,
        limit_tokens,
        limit_chars,
        fill_ratio,
        level,
        messages_omitted,
        summary_label,
        omission_label,
    }
}
