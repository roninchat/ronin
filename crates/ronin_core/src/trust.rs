//! Host-enforced trust foundation for M3.0 (capability boundary + silent-context).

/// Placeholder for secret-bearing ambient payload fragments.
pub const AMBIENT_REDACTED: &str = "[REDACTED]";

/// Agency tool names M3.0 must never execute (mutate / shell / browser).
pub const FORBIDDEN_AGENCY_TOOL_NAMES: &[&str] = &[
    "write_file",
    "create_file",
    "edit_file",
    "delete_file",
    "run_shell",
    "shell",
    "bash",
    "exec",
    "browser",
    "browse_url",
    "open_url",
    "web_search",
];

/// Host-allowlisted M3.0 tools (read-only memory).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedTool {
    /// List memory ids and titles.
    ListMemories,
    /// Fetch one memory by id.
    GetMemory {
        /// Opaque memory id.
        id: String,
    },
}

/// Host disposition for a `[TOOL_CALL: …]` marker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolDisposition {
    /// Allowlisted; may auto-execute.
    Allow(AllowedTool),
    /// Forbidden agency surface; never execute.
    Refuse {
        /// Tool name.
        name: String,
    },
    /// Unrecognized; never execute.
    Unknown {
        /// Tool name.
        name: String,
    },
}

/// Origin of candidate content for provider chat assembly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextOrigin {
    /// Composer message body.
    ComposerText,
    /// Explicit user attach (`@…`, CLI `--attach`, …).
    ExplicitAttachment,
    /// Confirmed confirm-to-attach draft.
    ConfirmToAttachAccepted,
    /// Visible per-send include of selected candidates.
    VisiblePerSendInclude,
    /// Enabled profile memories (indicator required elsewhere).
    EnabledProfileMemory,
    /// Index search hit — candidate only.
    IndexSearchHit,
    /// Persisted index corpus — never auto-merge.
    WorkspaceIndexCorpus,
    /// Clipboard-watch proposal — never silent-attach.
    ClipboardWatchProposal,
    /// Notification title/body — not model context.
    NotificationPayload,
    /// Other ambient desktop events.
    AmbientDesktopEvent,
}

/// Resolve the last unexecuted `[TOOL_CALL: …]` against the host allowlist.
pub fn resolve_marker_tool(text: &str) -> Option<ToolDisposition> {
    let (name, args) = parse_pending_tool_marker(text)?;
    Some(disposition_for_tool(&name, &args))
}

/// Whether the host may auto-execute this disposition (M3.0: allowlisted only).
pub fn may_auto_execute(disposition: &ToolDisposition) -> bool {
    matches!(disposition, ToolDisposition::Allow(_))
}

/// Whether this origin may merge into provider [`crate::ChatRequest`] assembly.
pub fn may_inject_into_chat_request(origin: ContextOrigin) -> bool {
    matches!(
        origin,
        ContextOrigin::ComposerText
            | ContextOrigin::ExplicitAttachment
            | ContextOrigin::ConfirmToAttachAccepted
            | ContextOrigin::VisiblePerSendInclude
            | ContextOrigin::EnabledProfileMemory
    )
}

/// Scrub secrets from ambient payloads (notifications, index metadata, events).
pub fn scrub_ambient_payload(input: &str) -> String {
    let mut out = scrub_keyed_values(input, &["bearer ", "sk-"], false);
    out = scrub_keyed_values(
        &out,
        &[
            "api_key=",
            "token=",
            "key=",
            "secret=",
            "password=",
            "access_token=",
        ],
        true,
    );
    out
}

fn disposition_for_tool(name: &str, args: &[String]) -> ToolDisposition {
    let lower = name.to_ascii_lowercase();
    if FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(&lower))
    {
        return ToolDisposition::Refuse { name: lower };
    }
    match lower.as_str() {
        "list_memories" => ToolDisposition::Allow(AllowedTool::ListMemories),
        "get_memory" => {
            let id = args
                .iter()
                .find_map(|p| p.strip_prefix("id:"))
                .map(|r| r.trim().trim_matches('"').trim_matches('\'').to_string())
                .unwrap_or_default();
            ToolDisposition::Allow(AllowedTool::GetMemory { id })
        }
        _ => ToolDisposition::Unknown { name: lower },
    }
}

fn parse_pending_tool_marker(text: &str) -> Option<(String, Vec<String>)> {
    let marker = "[TOOL_CALL:";
    let pos = text.rfind(marker)?;
    let rest = &text[pos..];
    if rest.contains("[TOOL_RESULT:") {
        return None;
    }
    let body = &text[pos + marker.len()..];
    let end = body.find(']')?;
    let parts: Vec<&str> = body[..end].split(',').map(str::trim).collect();
    if parts.is_empty() || parts[0].is_empty() {
        return None;
    }
    Some((
        parts[0].to_string(),
        parts[1..].iter().map(|s| (*s).to_string()).collect(),
    ))
}

fn scrub_keyed_values(input: &str, keys: &[&str], keep_key: bool) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    loop {
        let lower = rest.to_ascii_lowercase();
        let mut best: Option<(usize, &str)> = None;
        for key in keys {
            if let Some(idx) = lower.find(key) {
                if best.map(|(b, _)| idx < b).unwrap_or(true) {
                    best = Some((idx, *key));
                }
            }
        }
        let Some((idx, key)) = best else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..idx]);
        if keep_key {
            out.push_str(&rest[idx..idx + key.len()]);
            rest = &rest[idx + key.len()..];
        } else {
            rest = &rest[idx..];
        }
        out.push_str(AMBIENT_REDACTED);
        let skip = scrub_value_len(rest);
        rest = &rest[skip..];
    }
    out
}

fn scrub_value_len(value: &str) -> usize {
    let mut chars = value.chars();
    match chars.next() {
        Some('"' | '\'') => {
            let quote = value.as_bytes()[0] as char;
            1 + value[1..]
                .find(quote)
                .map(|i| i + 1)
                .unwrap_or(value[1..].len())
        }
        _ => value
            .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ',' | '&'))
            .unwrap_or(value.len()),
    }
}
