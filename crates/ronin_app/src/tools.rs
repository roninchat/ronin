//! Marker-based tool call parsing and execution against a Ronin session.
//!
//! Execution is gated by [`ronin_core::trust`] — mutate/shell/browser markers are
//! refused by the host allowlist even when system-prompt text is replaced.

use ronin_core::{resolve_marker_tool, AllowedTool, RoninSession, ToolDisposition};

/// A tool call that the host allowlist permits to auto-execute.
pub(crate) enum ToolCall {
    /// List memories.
    ListMemories,
    /// Fetch one memory by id.
    GetMemory(String),
}

/// Finds the last `[TOOL_CALL: …]` marker and returns an allowlisted call only.
///
/// Forbidden and unknown tools yield [`None`] here; callers that need an explicit
/// refusal string should use [`refusal_tool_result`].
pub(crate) fn parse_unexecuted_tool_call(text: &str) -> Option<ToolCall> {
    match resolve_marker_tool(text)? {
        ToolDisposition::Allow(AllowedTool::ListMemories) => Some(ToolCall::ListMemories),
        ToolDisposition::Allow(AllowedTool::GetMemory { id }) => Some(ToolCall::GetMemory(id)),
        ToolDisposition::Refuse { .. } | ToolDisposition::Unknown { .. } => None,
    }
}

/// When the latest marker is refused or unknown, returns a `[TOOL_RESULT: …]` that
/// records host refusal without executing any agency side effect.
pub(crate) fn refusal_tool_result(text: &str) -> Option<String> {
    match resolve_marker_tool(text)? {
        ToolDisposition::Allow(_) => None,
        ToolDisposition::Refuse { name } => Some(format!(
            "[TOOL_RESULT: {name}, error: \"refused by host capability boundary\"]"
        )),
        ToolDisposition::Unknown { name } => Some(format!(
            "[TOOL_RESULT: {name}, error: \"unknown tool; not registered\"]"
        )),
    }
}

/// Resolves the pending marker to either an allowlisted execution result or a
/// host refusal/unknown result. Returns `None` when no pending marker exists.
pub(crate) fn next_tool_result(session: &RoninSession, text: &str) -> Option<String> {
    if let Some(tool_call) = parse_unexecuted_tool_call(text) {
        return Some(execute_tool_call(session, &tool_call));
    }
    refusal_tool_result(text)
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
