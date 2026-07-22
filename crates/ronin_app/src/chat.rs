//! Chat request assembly with context caps.

use ronin_core::{ChatMessage, ChatRequest, ChatStreamEvent, Message, MessageRole};

/// Maximum number of conversation messages included in a provider request.
pub const MAX_MESSAGES: usize = 40;
/// Maximum total characters of conversation content included in a request.
pub const MAX_CHARS: usize = 80_000;

/// A provider request built from persisted messages with context caps applied.
pub(crate) struct CappedChatRequest {
    /// The assembled request.
    pub request: ChatRequest,
    /// Whether messages or context were omitted due to caps.
    pub truncated: bool,
}

/// Builds a provider chat request from persisted thread messages.
///
/// Keeps the most recent messages within [`MAX_MESSAGES`]/[`MAX_CHARS`],
/// skipping the streaming placeholder message, and prepends the provided
/// system prompt plus an optional explicit-attachment context block.
pub(crate) fn build_capped_chat_request(
    model: &str,
    all_msgs: &[Message],
    skip_message_id: &str,
    system_prompt: &str,
    attachment_context: Option<&str>,
) -> CappedChatRequest {
    let mut truncated = false;

    let mut included = Vec::new();
    let mut total_chars = 0usize;
    for msg in all_msgs.iter().rev() {
        if msg.id == skip_message_id {
            continue;
        }
        if included.len() >= MAX_MESSAGES {
            truncated = true;
            break;
        }
        let msg_chars = msg.content.chars().count();
        if total_chars + msg_chars > MAX_CHARS {
            truncated = true;
            break;
        }
        total_chars += msg_chars;
        included.push(msg);
    }
    included.reverse();

    let mut chat_messages = vec![ChatMessage {
        role: "system".to_string(),
        content: system_prompt.to_string(),
    }];
    if let Some(context) = attachment_context {
        if context.chars().count() > MAX_CHARS {
            truncated = true;
        }
        chat_messages.push(ChatMessage {
            role: "system".to_string(),
            content: context.to_string(),
        });
    }
    chat_messages.extend(included.into_iter().map(|m| ChatMessage {
        role: match m.role {
            MessageRole::User => "user".to_string(),
            MessageRole::Assistant => "assistant".to_string(),
            MessageRole::System => "system".to_string(),
        },
        content: m.content.clone(),
    }));

    CappedChatRequest {
        request: ChatRequest {
            model: model.to_string(),
            messages: chat_messages,
            system_prompt: Some(system_prompt.to_string()),
        },
        truncated,
    }
}

/// Derives a display title from the first non-empty line of a prompt.
///
/// Trims whitespace, collapses repeated whitespace/newlines into single
/// spaces, and truncates to approximately 60 characters.
pub fn derive_thread_title(prompt: &str) -> String {
    let first_line = prompt.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let collapsed: String = first_line.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= 60 {
        collapsed
    } else {
        let mut truncated = collapsed.chars().take(57).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}

const TITLE_GEN_SYSTEM: &str = "You invent short chat thread titles. Reply with the title only — at most 8 words, no quotes, no trailing punctuation, no explanation.";

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{truncated}…")
    } else {
        truncated
    }
}

/// Builds a lightweight chat request that asks the model for a thread title.
pub fn build_title_generation_request(
    model: &str,
    user_message: &str,
    assistant_message: &str,
) -> ChatRequest {
    let user_excerpt = truncate_chars(user_message, 400);
    let assistant_excerpt = truncate_chars(assistant_message, 400);
    let user_content = format!(
        "User message:\n{user_excerpt}\n\nAssistant reply:\n{assistant_excerpt}\n\nThread title:"
    );
    ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: TITLE_GEN_SYSTEM.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
        system_prompt: Some(TITLE_GEN_SYSTEM.to_string()),
    }
}

/// Joins streamed title-generation chunks into a raw response string.
pub fn collect_streamed_title(events: impl IntoIterator<Item = ChatStreamEvent>) -> String {
    let mut out = String::new();
    for event in events {
        match event {
            ChatStreamEvent::Chunk(chunk) => out.push_str(&chunk),
            ChatStreamEvent::Error(_) => break,
        }
    }
    out
}

/// Cleans a model title response into a persistable thread title.
pub fn sanitize_generated_title(raw: &str) -> Option<String> {
    let mut line = raw
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())?
        .trim_matches(|c| c == '"' || c == '\'' || c == '`')
        .trim()
        .to_string();
    if let Some(rest) = line
        .strip_prefix("Title:")
        .or_else(|| line.strip_prefix("title:"))
    {
        line = rest.trim().to_string();
    }
    let collapsed: String = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        return None;
    }
    Some(derive_thread_title(&collapsed))
}

/// Whether an auto-generated title may replace the current thread title.
///
/// Allows replacement while the title is still the default `New Chat` or the
/// provisional first-line derive from `first_user_message`. Manual renames and
/// already-custom titles are left alone.
pub fn may_apply_auto_title(current_title: &str, first_user_message: &str, manual: bool) -> bool {
    if manual {
        return false;
    }
    if current_title == "New Chat" {
        return true;
    }
    current_title == derive_thread_title(first_user_message)
}
