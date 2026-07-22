//! Focus-action and host-port seam coverage for desktop notifications (#75).

use ronin_core::{
    interpret_notification_action, shape_generation_notification, DesktopNotifier,
    GenerationNotifyInput, GenerationNotifyKind, NotificationPrefs, NullDesktopNotifier,
    RecordingDesktopNotifier, FOCUS_THREAD_ACTION,
};

#[test]
fn poll_focus_thread_defaults_none_on_recording_and_null_notifiers() {
    assert!(RecordingDesktopNotifier::new()
        .poll_focus_thread()
        .is_none());
    assert!(NullDesktopNotifier.poll_focus_thread().is_none());
}

#[test]
fn interpret_focus_action_matrix_for_many_thread_ids() {
    for i in 0..80 {
        let thread = format!("focus-seam-{i:03}");
        let action =
            interpret_notification_action(FOCUS_THREAD_ACTION, &thread).expect("focus action");
        assert_eq!(action.thread_id, thread);
        assert!(interpret_notification_action("app.open", &thread).is_none());
        assert!(interpret_notification_action("dismiss", &thread).is_none());
    }
}

#[test]
fn shaped_requests_always_carry_focus_default_and_button() {
    for i in 0..60 {
        for kind in [
            GenerationNotifyKind::Completed,
            GenerationNotifyKind::Failed,
        ] {
            let req = shape_generation_notification(
                &NotificationPrefs::default(),
                &GenerationNotifyInput {
                    kind,
                    thread_id: format!("focus-req-{i}"),
                    thread_title: Some(format!("T{i}")),
                    error_summary: Some("e".into()),
                },
            )
            .expect("shaped");
            assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
            assert_eq!(req.buttons.len(), 1);
            assert_eq!(req.buttons[0].action_id, FOCUS_THREAD_ACTION);
            assert_eq!(req.buttons[0].label, "Open thread");
            let focus = interpret_notification_action(
                req.default_action.as_deref().expect("default"),
                &req.thread_id,
            )
            .expect("interpret");
            assert_eq!(focus.thread_id, req.thread_id);
        }
    }
}

#[test]
fn recording_notifier_preserves_focus_metadata_on_delivery() {
    let notifier = RecordingDesktopNotifier::new();
    for i in 0..40 {
        let req = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Completed,
                thread_id: format!("rec-focus-{i}"),
                thread_title: None,
                error_summary: None,
            },
        )
        .expect("shaped");
        notifier.notify(&req).expect("notify");
    }
    let sent = notifier.take_sent();
    assert_eq!(sent.len(), 40);
    for (i, req) in sent.iter().enumerate() {
        assert_eq!(req.thread_id, format!("rec-focus-{i}"));
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert!(notifier.poll_focus_thread().is_none());
    }
}
