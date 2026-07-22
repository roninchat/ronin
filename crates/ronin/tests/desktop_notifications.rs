//! Host portal notification helpers (#75).

use ronin::desktop_notifications::{focus_thread_id_from_action, PortalDesktopNotifier};
use ronin_core::{
    shape_generation_notification, DesktopNotifier, GenerationNotifyInput, GenerationNotifyKind,
    NotificationPrefs, NullDesktopNotifier, RecordingDesktopNotifier, FOCUS_THREAD_ACTION,
};

#[test]
fn portal_notifier_type_implements_desktop_notifier_trait() {
    let _notifier: Box<dyn DesktopNotifier + Send> = Box::new(PortalDesktopNotifier::new());
    let _null: Box<dyn DesktopNotifier + Send> = Box::new(NullDesktopNotifier);
    let _rec: Box<dyn DesktopNotifier + Send> = Box::new(RecordingDesktopNotifier::new());
}

#[test]
fn focus_thread_id_from_action_reads_request_thread() {
    let req = shape_generation_notification(
        &NotificationPrefs::default(),
        &GenerationNotifyInput {
            kind: GenerationNotifyKind::Completed,
            thread_id: "thread-focus-1".into(),
            thread_title: Some("T".into()),
            error_summary: None,
        },
    )
    .expect("shaped");
    assert_eq!(
        focus_thread_id_from_action(FOCUS_THREAD_ACTION, &req).as_deref(),
        Some("thread-focus-1")
    );
    assert!(focus_thread_id_from_action("dismiss", &req).is_none());
}

#[test]
fn portal_notifier_poll_focus_defaults_empty_before_actions() {
    let notifier = PortalDesktopNotifier::new();
    assert!(notifier.poll_focus_thread().is_none());
}

#[test]
fn recording_notifier_poll_focus_is_none_by_default() {
    let notifier = RecordingDesktopNotifier::new();
    assert!(notifier.poll_focus_thread().is_none());
}

#[test]
fn recording_notifier_can_stand_in_for_portal_in_host_tests() {
    let notifier = RecordingDesktopNotifier::new();
    let req = shape_generation_notification(
        &NotificationPrefs { enabled: true },
        &GenerationNotifyInput {
            kind: GenerationNotifyKind::Failed,
            thread_id: "host-1".into(),
            thread_title: None,
            error_summary: Some("boom".into()),
        },
    )
    .expect("shaped");
    notifier.notify(&req).expect("deliver");
    let sent = notifier.take_sent();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].kind, GenerationNotifyKind::Failed);
    assert_eq!(
        focus_thread_id_from_action(FOCUS_THREAD_ACTION, &sent[0]).as_deref(),
        Some("host-1")
    );
}

#[test]
fn disabled_shaping_means_host_has_nothing_to_deliver() {
    let notifier = RecordingDesktopNotifier::new();
    let shaped = shape_generation_notification(
        &NotificationPrefs { enabled: false },
        &GenerationNotifyInput {
            kind: GenerationNotifyKind::Completed,
            thread_id: "x".into(),
            thread_title: None,
            error_summary: None,
        },
    );
    assert!(shaped.is_none());
    assert!(notifier.is_empty());
}
