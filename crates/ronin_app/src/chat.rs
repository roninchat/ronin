//! Chat request assembly with context caps.

use ronin_core::{ChatMessage, ChatRequest, Message, MessageRole, RONIN_SYSTEM_PROMPT};

/// Maximum number of conversation messages included in a provider request.
pub(crate) const MAX_MESSAGES: usize = 40;
/// Maximum total characters of conversation content included in a request.
pub(crate) const MAX_CHARS: usize = 80_000;

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
/// skipping the streaming placeholder message, and prepends Ronin's system
/// prompt plus an optional explicit-attachment context block.
pub(crate) fn build_capped_chat_request(
    model: &str,
    all_msgs: &[Message],
    skip_message_id: &str,
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
        content: RONIN_SYSTEM_PROMPT.to_string(),
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
            system_prompt: Some(RONIN_SYSTEM_PROMPT.to_string()),
        },
        truncated,
    }
}

/// Derives a display title from the first non-empty line of a prompt.
///
/// Trims whitespace, collapses repeated whitespace/newlines into single
/// spaces, and truncates to approximately 60 characters.
pub(crate) fn derive_thread_title(prompt: &str) -> String {
    let first_line = prompt.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let collapsed: String = first_line.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.len() <= 60 {
        collapsed
    } else {
        let mut truncated = collapsed.chars().take(57).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}
