//! Dense trust-foundation regression corpus (#69).
#![allow(clippy::useless_format, clippy::cognitive_complexity)]
use ronin_core::{
    may_auto_execute, may_inject_into_chat_request, resolve_marker_tool, scrub_ambient_payload,
    AllowedTool, ContextOrigin, ToolDisposition, AMBIENT_REDACTED, FORBIDDEN_AGENCY_TOOL_NAMES,
};

#[test]
fn refuse_write_file_bare() {
    let name = "write_file";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_write_file_prose() {
    let name = "write_file";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_write_file_args() {
    let name = "write_file";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_write_file_upper() {
    let name = "write_file";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_write_file_inline() {
    let name = "write_file";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_write_file_fence() {
    let name = "write_file";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_bare() {
    let name = "create_file";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_prose() {
    let name = "create_file";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_args() {
    let name = "create_file";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_upper() {
    let name = "create_file";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_inline() {
    let name = "create_file";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_create_file_fence() {
    let name = "create_file";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_bare() {
    let name = "edit_file";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_prose() {
    let name = "edit_file";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_args() {
    let name = "edit_file";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_upper() {
    let name = "edit_file";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_inline() {
    let name = "edit_file";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_edit_file_fence() {
    let name = "edit_file";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_bare() {
    let name = "delete_file";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_prose() {
    let name = "delete_file";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_args() {
    let name = "delete_file";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_upper() {
    let name = "delete_file";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_inline() {
    let name = "delete_file";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_delete_file_fence() {
    let name = "delete_file";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_bare() {
    let name = "run_shell";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_prose() {
    let name = "run_shell";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_args() {
    let name = "run_shell";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_upper() {
    let name = "run_shell";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_inline() {
    let name = "run_shell";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_run_shell_fence() {
    let name = "run_shell";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_bare() {
    let name = "shell";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_prose() {
    let name = "shell";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_args() {
    let name = "shell";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_upper() {
    let name = "shell";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_inline() {
    let name = "shell";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_shell_fence() {
    let name = "shell";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_bare() {
    let name = "bash";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_prose() {
    let name = "bash";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_args() {
    let name = "bash";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_upper() {
    let name = "bash";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_inline() {
    let name = "bash";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_bash_fence() {
    let name = "bash";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_bare() {
    let name = "exec";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_prose() {
    let name = "exec";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_args() {
    let name = "exec";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_upper() {
    let name = "exec";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_inline() {
    let name = "exec";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_exec_fence() {
    let name = "exec";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_bare() {
    let name = "browser";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_prose() {
    let name = "browser";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_args() {
    let name = "browser";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_upper() {
    let name = "browser";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_inline() {
    let name = "browser";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browser_fence() {
    let name = "browser";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_bare() {
    let name = "browse_url";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_prose() {
    let name = "browse_url";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_args() {
    let name = "browse_url";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_upper() {
    let name = "browse_url";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_inline() {
    let name = "browse_url";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_browse_url_fence() {
    let name = "browse_url";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_bare() {
    let name = "open_url";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_prose() {
    let name = "open_url";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_args() {
    let name = "open_url";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_upper() {
    let name = "open_url";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_inline() {
    let name = "open_url";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_open_url_fence() {
    let name = "open_url";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_bare() {
    let name = "web_search";
    let marker = format!("[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_prose() {
    let name = "web_search";
    let marker = format!("I will proceed.\n[TOOL_CALL: {name}]");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_args() {
    let name = "web_search";
    let marker = format!(r#"[TOOL_CALL: {name}, arg: \"x\"]"#);
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_upper() {
    let name = "web_search";
    let marker = format!("[TOOL_CALL: {}]", name.to_ascii_uppercase());
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_inline() {
    let name = "web_search";
    let marker = format!("ok [TOOL_CALL: {name}] end");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn refuse_web_search_fence() {
    let name = "web_search";
    let marker = format!("```\n[TOOL_CALL: {name}]\n```");
    let d = resolve_marker_tool(&marker).expect("disposition");
    assert!(matches!(d, ToolDisposition::Refuse { .. }), "{d:?}");
    assert!(!may_auto_execute(&d));
    assert!(FORBIDDEN_AGENCY_TOOL_NAMES
        .iter()
        .any(|f| f.eq_ignore_ascii_case(name)));
}

#[test]
fn unknown_apply_patch_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: apply_patch]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: apply_patch, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_run_python_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: run_python]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: run_python, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_sudo_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: sudo]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: sudo, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_curl_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: curl]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: curl, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_wget_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: wget]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: wget, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_ftp_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: ftp]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: ftp, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_scp_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: scp]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: scp, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_ssh_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: ssh]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: ssh, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_docker_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: docker]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: docker, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_kubectl_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: kubectl]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: kubectl, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_npm_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: npm]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: npm, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_pip_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: pip]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: pip, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_systemctl_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: systemctl]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: systemctl, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_osascript_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: osascript]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: osascript, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_powershell_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: powershell]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: powershell, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_cmd_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: cmd]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: cmd, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_eval_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: eval]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: eval, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_send_email_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: send_email]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: send_email, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_post_slack_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: post_slack]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: post_slack, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_tweet_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: tweet]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: tweet, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_calendar_create_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: calendar_create]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: calendar_create, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_file_move_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: file_move]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: file_move, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_file_copy_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: file_copy]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: file_copy, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_mkdir_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: mkdir]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: mkdir, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_rmdir_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: rmdir]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: rmdir, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_chmod_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: chmod]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: chmod, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_chown_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: chown]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: chown, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_tar_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: tar]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: tar, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_unzip_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: unzip]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: unzip, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_deploy_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: deploy]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: deploy, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_terraform_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: terraform]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: terraform, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_ansible_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: ansible]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: ansible, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_mcp_write_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: mcp_write]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: mcp_write, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_mcp_shell_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: mcp_shell]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: mcp_shell, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_tool_router_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: tool_router]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: tool_router, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_agent_delegate_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: agent_delegate]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: agent_delegate, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_compile_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: compile]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: compile, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn unknown_rm_rf_never_auto_executes() {
    let d = resolve_marker_tool("[TOOL_CALL: rm_rf]").unwrap();
    assert!(matches!(d, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d));
    let d2 = resolve_marker_tool("try [TOOL_CALL: rm_rf, x: 1]").unwrap();
    assert!(matches!(d2, ToolDisposition::Unknown { .. }));
    assert!(!may_auto_execute(&d2));
}

#[test]
fn scrub_secret_case_01() {
    let dirty = "event-1 key=sk-live-S001 api_key=K001 token=T001";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S001"), "{clean}");
    assert!(!clean.contains("K001"), "{clean}");
    assert!(!clean.contains("T001"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-1"));
}

#[test]
fn scrub_secret_case_02() {
    let dirty = "event-2 key=sk-live-S002 api_key=K002 token=T002";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S002"), "{clean}");
    assert!(!clean.contains("K002"), "{clean}");
    assert!(!clean.contains("T002"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-2"));
}

#[test]
fn scrub_secret_case_03() {
    let dirty = "event-3 key=sk-live-S003 api_key=K003 token=T003";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S003"), "{clean}");
    assert!(!clean.contains("K003"), "{clean}");
    assert!(!clean.contains("T003"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-3"));
}

#[test]
fn scrub_secret_case_04() {
    let dirty = "event-4 key=sk-live-S004 api_key=K004 token=T004";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S004"), "{clean}");
    assert!(!clean.contains("K004"), "{clean}");
    assert!(!clean.contains("T004"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-4"));
}

#[test]
fn scrub_secret_case_05() {
    let dirty = "event-5 key=sk-live-S005 api_key=K005 token=T005";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S005"), "{clean}");
    assert!(!clean.contains("K005"), "{clean}");
    assert!(!clean.contains("T005"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-5"));
}

#[test]
fn scrub_secret_case_06() {
    let dirty = "event-6 key=sk-live-S006 api_key=K006 token=T006";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S006"), "{clean}");
    assert!(!clean.contains("K006"), "{clean}");
    assert!(!clean.contains("T006"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-6"));
}

#[test]
fn scrub_secret_case_07() {
    let dirty = "event-7 key=sk-live-S007 api_key=K007 token=T007";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S007"), "{clean}");
    assert!(!clean.contains("K007"), "{clean}");
    assert!(!clean.contains("T007"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-7"));
}

#[test]
fn scrub_secret_case_08() {
    let dirty = "event-8 key=sk-live-S008 api_key=K008 token=T008";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S008"), "{clean}");
    assert!(!clean.contains("K008"), "{clean}");
    assert!(!clean.contains("T008"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-8"));
}

#[test]
fn scrub_secret_case_09() {
    let dirty = "event-9 key=sk-live-S009 api_key=K009 token=T009";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S009"), "{clean}");
    assert!(!clean.contains("K009"), "{clean}");
    assert!(!clean.contains("T009"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-9"));
}

#[test]
fn scrub_secret_case_10() {
    let dirty = "event-10 key=sk-live-S010 api_key=K010 token=T010";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S010"), "{clean}");
    assert!(!clean.contains("K010"), "{clean}");
    assert!(!clean.contains("T010"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-10"));
}

#[test]
fn scrub_secret_case_11() {
    let dirty = "event-11 key=sk-live-S011 api_key=K011 token=T011";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S011"), "{clean}");
    assert!(!clean.contains("K011"), "{clean}");
    assert!(!clean.contains("T011"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-11"));
}

#[test]
fn scrub_secret_case_12() {
    let dirty = "event-12 key=sk-live-S012 api_key=K012 token=T012";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S012"), "{clean}");
    assert!(!clean.contains("K012"), "{clean}");
    assert!(!clean.contains("T012"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-12"));
}

#[test]
fn scrub_secret_case_13() {
    let dirty = "event-13 key=sk-live-S013 api_key=K013 token=T013";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S013"), "{clean}");
    assert!(!clean.contains("K013"), "{clean}");
    assert!(!clean.contains("T013"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-13"));
}

#[test]
fn scrub_secret_case_14() {
    let dirty = "event-14 key=sk-live-S014 api_key=K014 token=T014";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S014"), "{clean}");
    assert!(!clean.contains("K014"), "{clean}");
    assert!(!clean.contains("T014"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-14"));
}

#[test]
fn scrub_secret_case_15() {
    let dirty = "event-15 key=sk-live-S015 api_key=K015 token=T015";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S015"), "{clean}");
    assert!(!clean.contains("K015"), "{clean}");
    assert!(!clean.contains("T015"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-15"));
}

#[test]
fn scrub_secret_case_16() {
    let dirty = "event-16 key=sk-live-S016 api_key=K016 token=T016";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S016"), "{clean}");
    assert!(!clean.contains("K016"), "{clean}");
    assert!(!clean.contains("T016"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-16"));
}

#[test]
fn scrub_secret_case_17() {
    let dirty = "event-17 key=sk-live-S017 api_key=K017 token=T017";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S017"), "{clean}");
    assert!(!clean.contains("K017"), "{clean}");
    assert!(!clean.contains("T017"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-17"));
}

#[test]
fn scrub_secret_case_18() {
    let dirty = "event-18 key=sk-live-S018 api_key=K018 token=T018";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S018"), "{clean}");
    assert!(!clean.contains("K018"), "{clean}");
    assert!(!clean.contains("T018"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-18"));
}

#[test]
fn scrub_secret_case_19() {
    let dirty = "event-19 key=sk-live-S019 api_key=K019 token=T019";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S019"), "{clean}");
    assert!(!clean.contains("K019"), "{clean}");
    assert!(!clean.contains("T019"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-19"));
}

#[test]
fn scrub_secret_case_20() {
    let dirty = "event-20 key=sk-live-S020 api_key=K020 token=T020";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S020"), "{clean}");
    assert!(!clean.contains("K020"), "{clean}");
    assert!(!clean.contains("T020"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-20"));
}

#[test]
fn scrub_secret_case_21() {
    let dirty = "event-21 key=sk-live-S021 api_key=K021 token=T021";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S021"), "{clean}");
    assert!(!clean.contains("K021"), "{clean}");
    assert!(!clean.contains("T021"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-21"));
}

#[test]
fn scrub_secret_case_22() {
    let dirty = "event-22 key=sk-live-S022 api_key=K022 token=T022";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S022"), "{clean}");
    assert!(!clean.contains("K022"), "{clean}");
    assert!(!clean.contains("T022"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-22"));
}

#[test]
fn scrub_secret_case_23() {
    let dirty = "event-23 key=sk-live-S023 api_key=K023 token=T023";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S023"), "{clean}");
    assert!(!clean.contains("K023"), "{clean}");
    assert!(!clean.contains("T023"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-23"));
}

#[test]
fn scrub_secret_case_24() {
    let dirty = "event-24 key=sk-live-S024 api_key=K024 token=T024";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S024"), "{clean}");
    assert!(!clean.contains("K024"), "{clean}");
    assert!(!clean.contains("T024"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-24"));
}

#[test]
fn scrub_secret_case_25() {
    let dirty = "event-25 key=sk-live-S025 api_key=K025 token=T025";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S025"), "{clean}");
    assert!(!clean.contains("K025"), "{clean}");
    assert!(!clean.contains("T025"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-25"));
}

#[test]
fn scrub_secret_case_26() {
    let dirty = "event-26 key=sk-live-S026 api_key=K026 token=T026";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S026"), "{clean}");
    assert!(!clean.contains("K026"), "{clean}");
    assert!(!clean.contains("T026"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-26"));
}

#[test]
fn scrub_secret_case_27() {
    let dirty = "event-27 key=sk-live-S027 api_key=K027 token=T027";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S027"), "{clean}");
    assert!(!clean.contains("K027"), "{clean}");
    assert!(!clean.contains("T027"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-27"));
}

#[test]
fn scrub_secret_case_28() {
    let dirty = "event-28 key=sk-live-S028 api_key=K028 token=T028";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S028"), "{clean}");
    assert!(!clean.contains("K028"), "{clean}");
    assert!(!clean.contains("T028"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-28"));
}

#[test]
fn scrub_secret_case_29() {
    let dirty = "event-29 key=sk-live-S029 api_key=K029 token=T029";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S029"), "{clean}");
    assert!(!clean.contains("K029"), "{clean}");
    assert!(!clean.contains("T029"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-29"));
}

#[test]
fn scrub_secret_case_30() {
    let dirty = "event-30 key=sk-live-S030 api_key=K030 token=T030";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S030"), "{clean}");
    assert!(!clean.contains("K030"), "{clean}");
    assert!(!clean.contains("T030"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-30"));
}

#[test]
fn scrub_secret_case_31() {
    let dirty = "event-31 key=sk-live-S031 api_key=K031 token=T031";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S031"), "{clean}");
    assert!(!clean.contains("K031"), "{clean}");
    assert!(!clean.contains("T031"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-31"));
}

#[test]
fn scrub_secret_case_32() {
    let dirty = "event-32 key=sk-live-S032 api_key=K032 token=T032";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S032"), "{clean}");
    assert!(!clean.contains("K032"), "{clean}");
    assert!(!clean.contains("T032"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-32"));
}

#[test]
fn scrub_secret_case_33() {
    let dirty = "event-33 key=sk-live-S033 api_key=K033 token=T033";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S033"), "{clean}");
    assert!(!clean.contains("K033"), "{clean}");
    assert!(!clean.contains("T033"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-33"));
}

#[test]
fn scrub_secret_case_34() {
    let dirty = "event-34 key=sk-live-S034 api_key=K034 token=T034";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S034"), "{clean}");
    assert!(!clean.contains("K034"), "{clean}");
    assert!(!clean.contains("T034"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-34"));
}

#[test]
fn scrub_secret_case_35() {
    let dirty = "event-35 key=sk-live-S035 api_key=K035 token=T035";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S035"), "{clean}");
    assert!(!clean.contains("K035"), "{clean}");
    assert!(!clean.contains("T035"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-35"));
}

#[test]
fn scrub_secret_case_36() {
    let dirty = "event-36 key=sk-live-S036 api_key=K036 token=T036";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S036"), "{clean}");
    assert!(!clean.contains("K036"), "{clean}");
    assert!(!clean.contains("T036"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-36"));
}

#[test]
fn scrub_secret_case_37() {
    let dirty = "event-37 key=sk-live-S037 api_key=K037 token=T037";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S037"), "{clean}");
    assert!(!clean.contains("K037"), "{clean}");
    assert!(!clean.contains("T037"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-37"));
}

#[test]
fn scrub_secret_case_38() {
    let dirty = "event-38 key=sk-live-S038 api_key=K038 token=T038";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S038"), "{clean}");
    assert!(!clean.contains("K038"), "{clean}");
    assert!(!clean.contains("T038"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-38"));
}

#[test]
fn scrub_secret_case_39() {
    let dirty = "event-39 key=sk-live-S039 api_key=K039 token=T039";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S039"), "{clean}");
    assert!(!clean.contains("K039"), "{clean}");
    assert!(!clean.contains("T039"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-39"));
}

#[test]
fn scrub_secret_case_40() {
    let dirty = "event-40 key=sk-live-S040 api_key=K040 token=T040";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S040"), "{clean}");
    assert!(!clean.contains("K040"), "{clean}");
    assert!(!clean.contains("T040"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-40"));
}

#[test]
fn scrub_secret_case_41() {
    let dirty = "event-41 key=sk-live-S041 api_key=K041 token=T041";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S041"), "{clean}");
    assert!(!clean.contains("K041"), "{clean}");
    assert!(!clean.contains("T041"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-41"));
}

#[test]
fn scrub_secret_case_42() {
    let dirty = "event-42 key=sk-live-S042 api_key=K042 token=T042";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S042"), "{clean}");
    assert!(!clean.contains("K042"), "{clean}");
    assert!(!clean.contains("T042"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-42"));
}

#[test]
fn scrub_secret_case_43() {
    let dirty = "event-43 key=sk-live-S043 api_key=K043 token=T043";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S043"), "{clean}");
    assert!(!clean.contains("K043"), "{clean}");
    assert!(!clean.contains("T043"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-43"));
}

#[test]
fn scrub_secret_case_44() {
    let dirty = "event-44 key=sk-live-S044 api_key=K044 token=T044";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S044"), "{clean}");
    assert!(!clean.contains("K044"), "{clean}");
    assert!(!clean.contains("T044"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-44"));
}

#[test]
fn scrub_secret_case_45() {
    let dirty = "event-45 key=sk-live-S045 api_key=K045 token=T045";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S045"), "{clean}");
    assert!(!clean.contains("K045"), "{clean}");
    assert!(!clean.contains("T045"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-45"));
}

#[test]
fn scrub_secret_case_46() {
    let dirty = "event-46 key=sk-live-S046 api_key=K046 token=T046";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S046"), "{clean}");
    assert!(!clean.contains("K046"), "{clean}");
    assert!(!clean.contains("T046"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-46"));
}

#[test]
fn scrub_secret_case_47() {
    let dirty = "event-47 key=sk-live-S047 api_key=K047 token=T047";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S047"), "{clean}");
    assert!(!clean.contains("K047"), "{clean}");
    assert!(!clean.contains("T047"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-47"));
}

#[test]
fn scrub_secret_case_48() {
    let dirty = "event-48 key=sk-live-S048 api_key=K048 token=T048";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S048"), "{clean}");
    assert!(!clean.contains("K048"), "{clean}");
    assert!(!clean.contains("T048"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-48"));
}

#[test]
fn scrub_secret_case_49() {
    let dirty = "event-49 key=sk-live-S049 api_key=K049 token=T049";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S049"), "{clean}");
    assert!(!clean.contains("K049"), "{clean}");
    assert!(!clean.contains("T049"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-49"));
}

#[test]
fn scrub_secret_case_50() {
    let dirty = "event-50 key=sk-live-S050 api_key=K050 token=T050";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S050"), "{clean}");
    assert!(!clean.contains("K050"), "{clean}");
    assert!(!clean.contains("T050"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-50"));
}

#[test]
fn scrub_secret_case_51() {
    let dirty = "event-51 key=sk-live-S051 api_key=K051 token=T051";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S051"), "{clean}");
    assert!(!clean.contains("K051"), "{clean}");
    assert!(!clean.contains("T051"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-51"));
}

#[test]
fn scrub_secret_case_52() {
    let dirty = "event-52 key=sk-live-S052 api_key=K052 token=T052";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S052"), "{clean}");
    assert!(!clean.contains("K052"), "{clean}");
    assert!(!clean.contains("T052"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-52"));
}

#[test]
fn scrub_secret_case_53() {
    let dirty = "event-53 key=sk-live-S053 api_key=K053 token=T053";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S053"), "{clean}");
    assert!(!clean.contains("K053"), "{clean}");
    assert!(!clean.contains("T053"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-53"));
}

#[test]
fn scrub_secret_case_54() {
    let dirty = "event-54 key=sk-live-S054 api_key=K054 token=T054";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S054"), "{clean}");
    assert!(!clean.contains("K054"), "{clean}");
    assert!(!clean.contains("T054"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-54"));
}

#[test]
fn scrub_secret_case_55() {
    let dirty = "event-55 key=sk-live-S055 api_key=K055 token=T055";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S055"), "{clean}");
    assert!(!clean.contains("K055"), "{clean}");
    assert!(!clean.contains("T055"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-55"));
}

#[test]
fn scrub_secret_case_56() {
    let dirty = "event-56 key=sk-live-S056 api_key=K056 token=T056";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S056"), "{clean}");
    assert!(!clean.contains("K056"), "{clean}");
    assert!(!clean.contains("T056"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-56"));
}

#[test]
fn scrub_secret_case_57() {
    let dirty = "event-57 key=sk-live-S057 api_key=K057 token=T057";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S057"), "{clean}");
    assert!(!clean.contains("K057"), "{clean}");
    assert!(!clean.contains("T057"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-57"));
}

#[test]
fn scrub_secret_case_58() {
    let dirty = "event-58 key=sk-live-S058 api_key=K058 token=T058";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S058"), "{clean}");
    assert!(!clean.contains("K058"), "{clean}");
    assert!(!clean.contains("T058"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-58"));
}

#[test]
fn scrub_secret_case_59() {
    let dirty = "event-59 key=sk-live-S059 api_key=K059 token=T059";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S059"), "{clean}");
    assert!(!clean.contains("K059"), "{clean}");
    assert!(!clean.contains("T059"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-59"));
}

#[test]
fn scrub_secret_case_60() {
    let dirty = "event-60 key=sk-live-S060 api_key=K060 token=T060";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S060"), "{clean}");
    assert!(!clean.contains("K060"), "{clean}");
    assert!(!clean.contains("T060"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-60"));
}

#[test]
fn scrub_secret_case_61() {
    let dirty = "event-61 key=sk-live-S061 api_key=K061 token=T061";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S061"), "{clean}");
    assert!(!clean.contains("K061"), "{clean}");
    assert!(!clean.contains("T061"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-61"));
}

#[test]
fn scrub_secret_case_62() {
    let dirty = "event-62 key=sk-live-S062 api_key=K062 token=T062";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S062"), "{clean}");
    assert!(!clean.contains("K062"), "{clean}");
    assert!(!clean.contains("T062"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-62"));
}

#[test]
fn scrub_secret_case_63() {
    let dirty = "event-63 key=sk-live-S063 api_key=K063 token=T063";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S063"), "{clean}");
    assert!(!clean.contains("K063"), "{clean}");
    assert!(!clean.contains("T063"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-63"));
}

#[test]
fn scrub_secret_case_64() {
    let dirty = "event-64 key=sk-live-S064 api_key=K064 token=T064";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S064"), "{clean}");
    assert!(!clean.contains("K064"), "{clean}");
    assert!(!clean.contains("T064"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-64"));
}

#[test]
fn scrub_secret_case_65() {
    let dirty = "event-65 key=sk-live-S065 api_key=K065 token=T065";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S065"), "{clean}");
    assert!(!clean.contains("K065"), "{clean}");
    assert!(!clean.contains("T065"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-65"));
}

#[test]
fn scrub_secret_case_66() {
    let dirty = "event-66 key=sk-live-S066 api_key=K066 token=T066";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S066"), "{clean}");
    assert!(!clean.contains("K066"), "{clean}");
    assert!(!clean.contains("T066"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-66"));
}

#[test]
fn scrub_secret_case_67() {
    let dirty = "event-67 key=sk-live-S067 api_key=K067 token=T067";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S067"), "{clean}");
    assert!(!clean.contains("K067"), "{clean}");
    assert!(!clean.contains("T067"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-67"));
}

#[test]
fn scrub_secret_case_68() {
    let dirty = "event-68 key=sk-live-S068 api_key=K068 token=T068";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S068"), "{clean}");
    assert!(!clean.contains("K068"), "{clean}");
    assert!(!clean.contains("T068"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-68"));
}

#[test]
fn scrub_secret_case_69() {
    let dirty = "event-69 key=sk-live-S069 api_key=K069 token=T069";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S069"), "{clean}");
    assert!(!clean.contains("K069"), "{clean}");
    assert!(!clean.contains("T069"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-69"));
}

#[test]
fn scrub_secret_case_70() {
    let dirty = "event-70 key=sk-live-S070 api_key=K070 token=T070";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S070"), "{clean}");
    assert!(!clean.contains("K070"), "{clean}");
    assert!(!clean.contains("T070"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-70"));
}

#[test]
fn scrub_secret_case_71() {
    let dirty = "event-71 key=sk-live-S071 api_key=K071 token=T071";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S071"), "{clean}");
    assert!(!clean.contains("K071"), "{clean}");
    assert!(!clean.contains("T071"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-71"));
}

#[test]
fn scrub_secret_case_72() {
    let dirty = "event-72 key=sk-live-S072 api_key=K072 token=T072";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S072"), "{clean}");
    assert!(!clean.contains("K072"), "{clean}");
    assert!(!clean.contains("T072"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-72"));
}

#[test]
fn scrub_secret_case_73() {
    let dirty = "event-73 key=sk-live-S073 api_key=K073 token=T073";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S073"), "{clean}");
    assert!(!clean.contains("K073"), "{clean}");
    assert!(!clean.contains("T073"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-73"));
}

#[test]
fn scrub_secret_case_74() {
    let dirty = "event-74 key=sk-live-S074 api_key=K074 token=T074";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S074"), "{clean}");
    assert!(!clean.contains("K074"), "{clean}");
    assert!(!clean.contains("T074"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-74"));
}

#[test]
fn scrub_secret_case_75() {
    let dirty = "event-75 key=sk-live-S075 api_key=K075 token=T075";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S075"), "{clean}");
    assert!(!clean.contains("K075"), "{clean}");
    assert!(!clean.contains("T075"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-75"));
}

#[test]
fn scrub_secret_case_76() {
    let dirty = "event-76 key=sk-live-S076 api_key=K076 token=T076";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S076"), "{clean}");
    assert!(!clean.contains("K076"), "{clean}");
    assert!(!clean.contains("T076"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-76"));
}

#[test]
fn scrub_secret_case_77() {
    let dirty = "event-77 key=sk-live-S077 api_key=K077 token=T077";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S077"), "{clean}");
    assert!(!clean.contains("K077"), "{clean}");
    assert!(!clean.contains("T077"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-77"));
}

#[test]
fn scrub_secret_case_78() {
    let dirty = "event-78 key=sk-live-S078 api_key=K078 token=T078";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S078"), "{clean}");
    assert!(!clean.contains("K078"), "{clean}");
    assert!(!clean.contains("T078"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-78"));
}

#[test]
fn scrub_secret_case_79() {
    let dirty = "event-79 key=sk-live-S079 api_key=K079 token=T079";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S079"), "{clean}");
    assert!(!clean.contains("K079"), "{clean}");
    assert!(!clean.contains("T079"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-79"));
}

#[test]
fn scrub_secret_case_80() {
    let dirty = "event-80 key=sk-live-S080 api_key=K080 token=T080";
    let clean = scrub_ambient_payload(dirty);
    assert!(!clean.contains("sk-live-S080"), "{clean}");
    assert!(!clean.contains("K080"), "{clean}");
    assert!(!clean.contains("T080"), "{clean}");
    assert!(clean.contains(AMBIENT_REDACTED));
    assert!(clean.contains("event-80"));
}

#[test]
fn scrub_preserves_benign_01() {
    let safe = "Generation finished for thread workspace-1";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_02() {
    let safe = "Generation finished for thread workspace-2";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_03() {
    let safe = "Generation finished for thread workspace-3";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_04() {
    let safe = "Generation finished for thread workspace-4";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_05() {
    let safe = "Generation finished for thread workspace-5";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_06() {
    let safe = "Generation finished for thread workspace-6";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_07() {
    let safe = "Generation finished for thread workspace-7";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_08() {
    let safe = "Generation finished for thread workspace-8";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_09() {
    let safe = "Generation finished for thread workspace-9";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_10() {
    let safe = "Generation finished for thread workspace-10";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_11() {
    let safe = "Generation finished for thread workspace-11";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_12() {
    let safe = "Generation finished for thread workspace-12";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_13() {
    let safe = "Generation finished for thread workspace-13";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_14() {
    let safe = "Generation finished for thread workspace-14";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_15() {
    let safe = "Generation finished for thread workspace-15";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_16() {
    let safe = "Generation finished for thread workspace-16";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_17() {
    let safe = "Generation finished for thread workspace-17";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_18() {
    let safe = "Generation finished for thread workspace-18";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_19() {
    let safe = "Generation finished for thread workspace-19";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_20() {
    let safe = "Generation finished for thread workspace-20";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_21() {
    let safe = "Generation finished for thread workspace-21";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_22() {
    let safe = "Generation finished for thread workspace-22";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_23() {
    let safe = "Generation finished for thread workspace-23";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_24() {
    let safe = "Generation finished for thread workspace-24";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_25() {
    let safe = "Generation finished for thread workspace-25";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_26() {
    let safe = "Generation finished for thread workspace-26";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_27() {
    let safe = "Generation finished for thread workspace-27";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_28() {
    let safe = "Generation finished for thread workspace-28";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_29() {
    let safe = "Generation finished for thread workspace-29";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_30() {
    let safe = "Generation finished for thread workspace-30";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_31() {
    let safe = "Generation finished for thread workspace-31";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_32() {
    let safe = "Generation finished for thread workspace-32";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_33() {
    let safe = "Generation finished for thread workspace-33";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_34() {
    let safe = "Generation finished for thread workspace-34";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_35() {
    let safe = "Generation finished for thread workspace-35";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_36() {
    let safe = "Generation finished for thread workspace-36";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_37() {
    let safe = "Generation finished for thread workspace-37";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_38() {
    let safe = "Generation finished for thread workspace-38";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_39() {
    let safe = "Generation finished for thread workspace-39";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn scrub_preserves_benign_40() {
    let safe = "Generation finished for thread workspace-40";
    assert_eq!(scrub_ambient_payload(safe), safe);
}

#[test]
fn origin_index_search_hit_blocked_from_chat_request() {
    assert!(!may_inject_into_chat_request(ContextOrigin::IndexSearchHit));
}

#[test]
fn origin_workspace_index_corpus_blocked_from_chat_request() {
    assert!(!may_inject_into_chat_request(
        ContextOrigin::WorkspaceIndexCorpus
    ));
}

#[test]
fn origin_clipboard_watch_proposal_blocked_from_chat_request() {
    assert!(!may_inject_into_chat_request(
        ContextOrigin::ClipboardWatchProposal
    ));
}

#[test]
fn origin_notification_payload_blocked_from_chat_request() {
    assert!(!may_inject_into_chat_request(
        ContextOrigin::NotificationPayload
    ));
}

#[test]
fn origin_ambient_desktop_event_blocked_from_chat_request() {
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn origin_composer_text_allowed_into_chat_request() {
    assert!(may_inject_into_chat_request(ContextOrigin::ComposerText));
}

#[test]
fn origin_explicit_attachment_allowed_into_chat_request() {
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn origin_confirm_to_attach_accepted_allowed_into_chat_request() {
    assert!(may_inject_into_chat_request(
        ContextOrigin::ConfirmToAttachAccepted
    ));
}

#[test]
fn origin_visible_per_send_include_allowed_into_chat_request() {
    assert!(may_inject_into_chat_request(
        ContextOrigin::VisiblePerSendInclude
    ));
}

#[test]
fn origin_enabled_profile_memory_allowed_into_chat_request() {
    assert!(may_inject_into_chat_request(
        ContextOrigin::EnabledProfileMemory
    ));
}

#[test]
fn allow_get_memory_id_case_01() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "a", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "a".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_02() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "b", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "b".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_03() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "c", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "c".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_04() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "d", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "d".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_05() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "e", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "e".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_06() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "mem-1", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "mem-1".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_07() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "mem-2", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "mem-2".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_08() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "019ecc48", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "019ecc48".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_09() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "019ecc49", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "019ecc49".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_10() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "uuid-1", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "uuid-1".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_11() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "uuid-2", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "uuid-2".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_12() {
    let marker = concat!(
        "[TOOL_CALL: get_memory, id: \"",
        "id_with_underscore",
        "\"]"
    );
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "id_with_underscore".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_13() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "id-with-dash", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "id-with-dash".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_14() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "Z", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "Z".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_15() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "0", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "0".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}

#[test]
fn allow_get_memory_id_case_16() {
    let marker = concat!("[TOOL_CALL: get_memory, id: \"", "abc123", "\"]");
    assert_eq!(
        resolve_marker_tool(marker),
        Some(ToolDisposition::Allow(AllowedTool::GetMemory {
            id: "abc123".into()
        }))
    );
    assert!(may_auto_execute(&resolve_marker_tool(marker).unwrap()));
}
