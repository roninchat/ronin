//! Marker-based tool call parsing and execution against a Ronin session.

use ronin_core::RoninSession;

/// A tool call requested by the assistant through inline markers.
pub(crate) enum ToolCall {
    ListMemories,
    GetMemory(String),
}

/// Finds the last `[TOOL_CALL: ...]` marker that has no following result.
pub(crate) fn parse_unexecuted_tool_call(text: &str) -> Option<ToolCall> {
    let marker = "[TOOL_CALL:";
    if let Some(pos) = text.rfind(marker) {
        let rest = &text[pos..];
        if rest.contains("[TOOL_RESULT:") {
            return None;
        }

        let call_content_str = &text[pos + marker.len()..];
        if let Some(end_pos) = call_content_str.find(']') {
            let call_content = &call_content_str[..end_pos];
            let parts: Vec<&str> = call_content.split(',').map(|s| s.trim()).collect();
            if !parts.is_empty() {
                let tool_name = parts[0].to_lowercase();
                if tool_name == "list_memories" {
                    return Some(ToolCall::ListMemories);
                } else if tool_name == "get_memory" {
                    for part in &parts[1..] {
                        if let Some(stripped) = part.strip_prefix("id:") {
                            let id = stripped.trim().trim_matches('"').trim_matches('\'');
                            return Some(ToolCall::GetMemory(id.to_string()));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Executes a tool call and formats its `[TOOL_RESULT: ...]` block.
pub(crate) fn execute_tool_call(session: &RoninSession, tool_call: &ToolCall) -> String {
    match tool_call {
        ToolCall::ListMemories => match session.list_memories() {
            Ok(mems) => {
                let mut res = String::from("ID, Title\n");
                for m in mems {
                    res.push_str(&format!("{}, {}\n", m.id.0, m.title));
                }
                format!("[TOOL_RESULT: list_memories, result: {:?}]", res)
            }
            Err(e) => format!("[TOOL_RESULT: list_memories, error: {:?}]", e.to_string()),
        },
        ToolCall::GetMemory(id) => match session.list_memories() {
            Ok(mems) => {
                if let Some(m) = mems.into_iter().find(|m| &m.id.0 == id) {
                    format!("[TOOL_RESULT: get_memory, result: {:?}]", m.content)
                } else {
                    String::from("[TOOL_RESULT: get_memory, error: \"Memory not found\"]")
                }
            }
            Err(e) => format!("[TOOL_RESULT: get_memory, error: {:?}]", e.to_string()),
        },
    }
}
