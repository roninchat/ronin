//! Desktop notifications — request shaping + silent-context gates (#75).
//!
//! Public seams:
//! - [`ronin_core::shape_generation_notification`]
//! - [`ronin_core::interpret_notification_action`]
//! - [`ronin_core::notification_may_inject_into_chat_request`]
//! - [`ronin_core::RecordingDesktopNotifier`]

use ronin_core::{
    interpret_notification_action, may_inject_into_chat_request,
    notification_may_inject_into_chat_request, notification_payload_origin, scrub_ambient_payload,
    shape_generation_notification, DesktopNotifier, GenerationNotifyInput, GenerationNotifyKind,
    NotificationPrefs, RecordingDesktopNotifier, AMBIENT_REDACTED, FOCUS_THREAD_ACTION,
    GENERATION_NOTIFICATION_ID_PREFIX,
};

fn completed(thread_id: &str, title: Option<&str>) -> GenerationNotifyInput {
    GenerationNotifyInput {
        kind: GenerationNotifyKind::Completed,
        thread_id: thread_id.into(),
        thread_title: title.map(str::to_string),
        error_summary: None,
    }
}

fn failed(thread_id: &str, title: Option<&str>, err: Option<&str>) -> GenerationNotifyInput {
    GenerationNotifyInput {
        kind: GenerationNotifyKind::Failed,
        thread_id: thread_id.into(),
        thread_title: title.map(str::to_string),
        error_summary: err.map(str::to_string),
    }
}

#[test]
fn shape_completed_notification_includes_focus_action_and_stable_id() {
    let req = shape_generation_notification(
        &NotificationPrefs::default(),
        &completed("thr-1", Some("Plan")),
    )
    .expect("shaped");
    assert_eq!(req.kind, GenerationNotifyKind::Completed);
    assert_eq!(req.thread_id, "thr-1");
    assert!(req.id.starts_with(GENERATION_NOTIFICATION_ID_PREFIX));
    assert!(req.id.contains("done"));
    assert!(req.id.contains("thr-1"));
    assert_eq!(req.title, "Ronin — generation complete");
    assert!(req.body.contains("Plan"));
    assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
    assert_eq!(req.buttons.len(), 1);
    assert_eq!(req.buttons[0].action_id, FOCUS_THREAD_ACTION);
    assert_eq!(req.buttons[0].label, "Open thread");
}

#[test]
fn shape_failed_notification_includes_error_summary() {
    let req = shape_generation_notification(
        &NotificationPrefs::default(),
        &failed("t2", Some("Debug"), Some("connection refused")),
    )
    .expect("shaped");
    assert_eq!(req.kind, GenerationNotifyKind::Failed);
    assert!(req.id.contains("fail"));
    assert_eq!(req.title, "Ronin — generation failed");
    assert!(req.body.contains("Debug"));
    assert!(req.body.contains("connection refused"));
    assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
}

#[test]
fn disabled_prefs_suppress_all_notification_shaping() {
    let prefs = NotificationPrefs { enabled: false };
    assert!(shape_generation_notification(&prefs, &completed("x", None)).is_none());
    assert!(shape_generation_notification(&prefs, &failed("x", None, Some("e"))).is_none());
}

#[test]
fn empty_thread_id_is_rejected() {
    assert!(
        shape_generation_notification(&NotificationPrefs::default(), &completed("", None))
            .is_none()
    );
    assert!(
        shape_generation_notification(&NotificationPrefs::default(), &completed("   ", None))
            .is_none()
    );
}

#[test]
fn notification_payload_never_injects_into_chat_request() {
    assert!(!notification_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(notification_payload_origin()));
    let req = shape_generation_notification(
        &NotificationPrefs::default(),
        &failed("leak", Some("api_key=sk-secret"), Some("token=abc123")),
    )
    .expect("shaped");
    assert!(!may_inject_into_chat_request(notification_payload_origin()));
    assert!(req.body.contains(AMBIENT_REDACTED));
    assert!(!req.body.contains("sk-secret"));
    assert!(!req.body.contains("abc123"));
}

#[test]
fn shaped_bodies_scrub_bearer_and_keyed_secrets() {
    let req = shape_generation_notification(
        &NotificationPrefs::default(),
        &failed(
            "sec",
            Some("Bearer sk-live-ABCDEF"),
            Some("password=hunter2 and key=xyz"),
        ),
    )
    .expect("shaped");
    assert_eq!(req.body, scrub_ambient_payload(&req.body));
    assert!(req.body.contains(AMBIENT_REDACTED));
    assert!(!req.body.contains("sk-live-ABCDEF"));
    assert!(!req.body.contains("hunter2"));
    assert!(!req.body.contains("xyz"));
}

#[test]
fn recording_notifier_captures_shaped_requests_only() {
    let notifier = RecordingDesktopNotifier::new();
    assert!(notifier.is_empty());
    let req = shape_generation_notification(&NotificationPrefs::default(), &completed("n1", None))
        .expect("shaped");
    notifier.notify(&req).expect("deliver");
    assert_eq!(notifier.len(), 1);
    let sent = notifier.take_sent();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].thread_id, "n1");
    assert!(notifier.is_empty());
}

#[test]
fn interpret_focus_action_only_for_known_action_id() {
    let focus = interpret_notification_action(FOCUS_THREAD_ACTION, "thread-9").expect("focus");
    assert_eq!(focus.thread_id, "thread-9");
    assert!(interpret_notification_action("dismiss", "thread-9").is_none());
    assert!(interpret_notification_action(FOCUS_THREAD_ACTION, "").is_none());
    assert!(interpret_notification_action(FOCUS_THREAD_ACTION, "  ").is_none());
}

#[test]
fn completed_body_without_title_uses_generic_copy() {
    let req = shape_generation_notification(&NotificationPrefs::default(), &completed("g", None))
        .expect("shaped");
    assert_eq!(req.body, "Finished generating a reply.");
}

#[test]
fn failed_body_without_title_or_error_uses_generic_copy() {
    let req =
        shape_generation_notification(&NotificationPrefs::default(), &failed("g", None, None))
            .expect("shaped");
    assert_eq!(req.body, "Generation failed.");
}
