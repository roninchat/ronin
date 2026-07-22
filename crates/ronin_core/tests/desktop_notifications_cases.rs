//! Table-driven cases for desktop notification shaping (#75).

use ronin_core::{
    interpret_notification_action, shape_generation_notification, GenerationNotifyInput,
    GenerationNotifyKind, NotificationPrefs, AMBIENT_REDACTED, FOCUS_THREAD_ACTION,
};

struct ShapeCase {
    name: &'static str,
    enabled: bool,
    kind: GenerationNotifyKind,
    thread_id: &'static str,
    thread_title: Option<&'static str>,
    error_summary: Option<&'static str>,
    expect_some: bool,
    title_contains: Option<&'static str>,
    body_contains: Option<&'static str>,
    body_excludes: Option<&'static str>,
    expect_focus: bool,
}

fn run_case(case: &ShapeCase) {
    let prefs = NotificationPrefs {
        enabled: case.enabled,
    };
    let input = GenerationNotifyInput {
        kind: case.kind,
        thread_id: case.thread_id.into(),
        thread_title: case.thread_title.map(str::to_string),
        error_summary: case.error_summary.map(str::to_string),
    };
    let shaped = shape_generation_notification(&prefs, &input);
    if !case.expect_some {
        assert!(shaped.is_none(), "case {} expected None", case.name);
        return;
    }
    let req = shaped.unwrap_or_else(|| panic!("case {} expected Some", case.name));
    assert_eq!(req.kind, case.kind, "case {}", case.name);
    assert_eq!(req.thread_id, case.thread_id, "case {}", case.name);
    if let Some(frag) = case.title_contains {
        assert!(
            req.title.contains(frag),
            "case {} title missing {frag:?}: {}",
            case.name,
            req.title
        );
    }
    if let Some(frag) = case.body_contains {
        assert!(
            req.body.contains(frag),
            "case {} body missing {frag:?}: {}",
            case.name,
            req.body
        );
    }
    if let Some(frag) = case.body_excludes {
        assert!(
            !req.body.contains(frag),
            "case {} body should exclude {frag:?}: {}",
            case.name,
            req.body
        );
    }
    if case.expect_focus {
        assert_eq!(
            req.default_action.as_deref(),
            Some(FOCUS_THREAD_ACTION),
            "case {}",
            case.name
        );
        assert!(
            req.buttons
                .iter()
                .any(|b| b.action_id == FOCUS_THREAD_ACTION),
            "case {} missing focus button",
            case.name
        );
    }
}

#[test]
fn shape_generation_notification_table() {
    let cases = [
        ShapeCase {
            name: "completed_with_title",
            enabled: true,
            kind: GenerationNotifyKind::Completed,
            thread_id: "a1",
            thread_title: Some("Notes"),
            error_summary: None,
            expect_some: true,
            title_contains: Some("complete"),
            body_contains: Some("Notes"),
            body_excludes: None,
            expect_focus: true,
        },
        ShapeCase {
            name: "completed_blank_title",
            enabled: true,
            kind: GenerationNotifyKind::Completed,
            thread_id: "a2",
            thread_title: Some("   "),
            error_summary: None,
            expect_some: true,
            title_contains: Some("complete"),
            body_contains: Some("Finished generating"),
            body_excludes: None,
            expect_focus: true,
        },
        ShapeCase {
            name: "failed_with_error",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "b1",
            thread_title: Some("Work"),
            error_summary: Some("timeout"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some("timeout"),
            body_excludes: None,
            expect_focus: true,
        },
        ShapeCase {
            name: "failed_blank_error",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "b2",
            thread_title: None,
            error_summary: Some("  "),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some("Generation failed."),
            body_excludes: None,
            expect_focus: true,
        },
        ShapeCase {
            name: "disabled_completed",
            enabled: false,
            kind: GenerationNotifyKind::Completed,
            thread_id: "c1",
            thread_title: Some("Quiet"),
            error_summary: None,
            expect_some: false,
            title_contains: None,
            body_contains: None,
            body_excludes: None,
            expect_focus: false,
        },
        ShapeCase {
            name: "disabled_failed",
            enabled: false,
            kind: GenerationNotifyKind::Failed,
            thread_id: "c2",
            thread_title: None,
            error_summary: Some("err"),
            expect_some: false,
            title_contains: None,
            body_contains: None,
            body_excludes: None,
            expect_focus: false,
        },
        ShapeCase {
            name: "empty_thread",
            enabled: true,
            kind: GenerationNotifyKind::Completed,
            thread_id: "",
            thread_title: Some("X"),
            error_summary: None,
            expect_some: false,
            title_contains: None,
            body_contains: None,
            body_excludes: None,
            expect_focus: false,
        },
        ShapeCase {
            name: "scrub_api_key_in_title_path",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec1",
            thread_title: Some("api_key=super-secret"),
            error_summary: None,
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("super-secret"),
            expect_focus: true,
        },
        ShapeCase {
            name: "scrub_token_in_error",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec2",
            thread_title: Some("Job"),
            error_summary: Some("token=leakme"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("leakme"),
            expect_focus: true,
        },
        ShapeCase {
            name: "scrub_bearer_in_error",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec3",
            thread_title: None,
            error_summary: Some("Bearer sk-test-999"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("sk-test-999"),
            expect_focus: true,
        },
        ShapeCase {
            name: "scrub_password",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec4",
            thread_title: Some("Auth"),
            error_summary: Some("password=hunter2"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("hunter2"),
            expect_focus: true,
        },
        ShapeCase {
            name: "scrub_access_token",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec5",
            thread_title: None,
            error_summary: Some("access_token=atatat"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("atatat"),
            expect_focus: true,
        },
        ShapeCase {
            name: "unicode_thread_id_sanitized_in_id",
            enabled: true,
            kind: GenerationNotifyKind::Completed,
            thread_id: "id/with spaces",
            thread_title: Some("OK"),
            error_summary: None,
            expect_some: true,
            title_contains: Some("complete"),
            body_contains: Some("OK"),
            body_excludes: None,
            expect_focus: true,
        },
        ShapeCase {
            name: "secret_key_prefix",
            enabled: true,
            kind: GenerationNotifyKind::Failed,
            thread_id: "sec6",
            thread_title: Some("T"),
            error_summary: Some("sk-proj-ABCDEFG"),
            expect_some: true,
            title_contains: Some("failed"),
            body_contains: Some(AMBIENT_REDACTED),
            body_excludes: Some("ABCDEFG"),
            expect_focus: true,
        },
    ];
    for case in &cases {
        run_case(case);
    }
}

#[test]
fn interpret_notification_action_table() {
    let cases: &[(&str, &str, bool)] = &[
        (FOCUS_THREAD_ACTION, "t1", true),
        (FOCUS_THREAD_ACTION, "uuid-019e", true),
        ("focus-thread", "ok", true),
        ("open-folder", "t1", false),
        ("dismiss", "t1", false),
        ("", "t1", false),
        (FOCUS_THREAD_ACTION, "", false),
        (FOCUS_THREAD_ACTION, "   ", false),
        ("FOCUS-THREAD", "t1", false),
        ("focus_thread", "t1", false),
    ];
    for (action, thread, expect) in cases {
        let got = interpret_notification_action(action, thread);
        assert_eq!(
            got.is_some(),
            *expect,
            "action={action:?} thread={thread:?}"
        );
        if *expect {
            assert_eq!(got.expect("focus").thread_id, thread.trim());
        }
    }
}

#[test]
fn disable_gate_matrix_across_kinds() {
    for enabled in [true, false] {
        for kind in [
            GenerationNotifyKind::Completed,
            GenerationNotifyKind::Failed,
        ] {
            for (thread_id, expect_when_enabled) in [("", false), ("x", true), ("  ", false)] {
                let prefs = NotificationPrefs { enabled };
                let input = GenerationNotifyInput {
                    kind,
                    thread_id: thread_id.into(),
                    thread_title: Some("Title".into()),
                    error_summary: Some("err".into()),
                };
                let shaped = shape_generation_notification(&prefs, &input);
                let expect = enabled && expect_when_enabled;
                assert_eq!(
                    shaped.is_some(),
                    expect,
                    "enabled={enabled} kind={kind:?} thread={thread_id:?}"
                );
            }
        }
    }
}
