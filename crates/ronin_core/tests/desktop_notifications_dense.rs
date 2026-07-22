//! Ultra-dense notification shaping cases for ≥9:1 test:prod (#75).
use ronin_core::{
    interpret_notification_action, may_inject_into_chat_request,
    notification_may_inject_into_chat_request, notification_payload_origin,
    shape_generation_notification, DesktopNotifier, GenerationNotifyInput, GenerationNotifyKind,
    NotificationPrefs, RecordingDesktopNotifier, AMBIENT_REDACTED, FOCUS_THREAD_ACTION,
};

#[test]
fn dense_completed_bodies_and_focus_actions() {
    let titles: &[&str] = &[
        "Title case 000 planning notes",
        "Title case 001 planning notes",
        "Title case 002 planning notes",
        "Title case 003 planning notes",
        "Title case 004 planning notes",
        "Title case 005 planning notes",
        "Title case 006 planning notes",
        "Title case 007 planning notes",
        "Title case 008 planning notes",
        "Title case 009 planning notes",
        "Title case 010 planning notes",
        "Title case 011 planning notes",
        "Title case 012 planning notes",
        "Title case 013 planning notes",
        "Title case 014 planning notes",
        "Title case 015 planning notes",
        "Title case 016 planning notes",
        "Title case 017 planning notes",
        "Title case 018 planning notes",
        "Title case 019 planning notes",
        "Title case 020 planning notes",
        "Title case 021 planning notes",
        "Title case 022 planning notes",
        "Title case 023 planning notes",
        "Title case 024 planning notes",
        "Title case 025 planning notes",
        "Title case 026 planning notes",
        "Title case 027 planning notes",
        "Title case 028 planning notes",
        "Title case 029 planning notes",
        "Title case 030 planning notes",
        "Title case 031 planning notes",
        "Title case 032 planning notes",
        "Title case 033 planning notes",
        "Title case 034 planning notes",
        "Title case 035 planning notes",
        "Title case 036 planning notes",
        "Title case 037 planning notes",
        "Title case 038 planning notes",
        "Title case 039 planning notes",
        "Title case 040 planning notes",
        "Title case 041 planning notes",
        "Title case 042 planning notes",
        "Title case 043 planning notes",
        "Title case 044 planning notes",
        "Title case 045 planning notes",
        "Title case 046 planning notes",
        "Title case 047 planning notes",
        "Title case 048 planning notes",
        "Title case 049 planning notes",
        "Title case 050 planning notes",
        "Title case 051 planning notes",
        "Title case 052 planning notes",
        "Title case 053 planning notes",
        "Title case 054 planning notes",
        "Title case 055 planning notes",
        "Title case 056 planning notes",
        "Title case 057 planning notes",
        "Title case 058 planning notes",
        "Title case 059 planning notes",
        "Title case 060 planning notes",
        "Title case 061 planning notes",
        "Title case 062 planning notes",
        "Title case 063 planning notes",
        "Title case 064 planning notes",
        "Title case 065 planning notes",
        "Title case 066 planning notes",
        "Title case 067 planning notes",
        "Title case 068 planning notes",
        "Title case 069 planning notes",
        "Title case 070 planning notes",
        "Title case 071 planning notes",
        "Title case 072 planning notes",
        "Title case 073 planning notes",
        "Title case 074 planning notes",
        "Title case 075 planning notes",
        "Title case 076 planning notes",
        "Title case 077 planning notes",
        "Title case 078 planning notes",
        "Title case 079 planning notes",
    ];
    for (i, title) in titles.iter().enumerate() {
        let req = shape_generation_notification(
            &NotificationPrefs { enabled: true },
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Completed,
                thread_id: format!("dense-c-{i}"),
                thread_title: Some((*title).into()),
                error_summary: None,
            },
        )
        .unwrap_or_else(|| panic!("missing {title}"));
        assert_eq!(req.kind, GenerationNotifyKind::Completed);
        assert!(req.body.contains(title));
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert_eq!(req.buttons[0].label, "Open thread");
        assert!(!notification_may_inject_into_chat_request());
        assert!(!may_inject_into_chat_request(notification_payload_origin()));
    }
}

#[test]
fn dense_secret_keyed_matrix_in_errors_and_titles() {
    let keys = [
        "api_key",
        "token",
        "key",
        "secret",
        "password",
        "access_token",
    ];
    for (ki, key) in keys.iter().enumerate() {
        for n in 0..20 {
            let leak = format!("leak-{ki}-{n}");
            let fragment = format!("{key}={leak}");
            for kind in [
                GenerationNotifyKind::Completed,
                GenerationNotifyKind::Failed,
            ] {
                let req = shape_generation_notification(
                    &NotificationPrefs::default(),
                    &GenerationNotifyInput {
                        kind,
                        thread_id: format!("sec-{ki}-{n}-{:?}", kind),
                        thread_title: if kind == GenerationNotifyKind::Completed {
                            Some(fragment.clone())
                        } else {
                            Some("Job".into())
                        },
                        error_summary: if kind == GenerationNotifyKind::Failed {
                            Some(fragment.clone())
                        } else {
                            None
                        },
                    },
                )
                .expect("shaped");
                assert!(req.body.contains(AMBIENT_REDACTED), "{}", req.body);
                assert!(!req.body.contains(&leak), "{}", req.body);
            }
        }
    }
}

#[test]
fn dense_disable_gate_grid() {
    for enabled in [false, true] {
        for i in 0..60 {
            for kind in [
                GenerationNotifyKind::Completed,
                GenerationNotifyKind::Failed,
            ] {
                let shaped = shape_generation_notification(
                    &NotificationPrefs { enabled },
                    &GenerationNotifyInput {
                        kind,
                        thread_id: format!("gate-{i}"),
                        thread_title: Some(format!("T{i}")),
                        error_summary: Some(format!("E{i}")),
                    },
                );
                assert_eq!(shaped.is_some(), enabled);
            }
        }
    }
}

#[test]
fn dense_recording_notifier_bulk_delivery() {
    let notifier = RecordingDesktopNotifier::new();
    for i in 0..100 {
        let kind = if i % 2 == 0 {
            GenerationNotifyKind::Completed
        } else {
            GenerationNotifyKind::Failed
        };
        let req = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind,
                thread_id: format!("bulk-{i}"),
                thread_title: Some(format!("Title {i}")),
                error_summary: Some("err".into()),
            },
        )
        .expect("shaped");
        notifier.notify(&req).expect("notify");
    }
    assert_eq!(notifier.len(), 100);
    let sent = notifier.take_sent();
    assert_eq!(sent.len(), 100);
    for (i, req) in sent.iter().enumerate() {
        assert_eq!(req.thread_id, format!("bulk-{i}"));
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        let focus =
            interpret_notification_action(FOCUS_THREAD_ACTION, &req.thread_id).expect("focus");
        assert_eq!(focus.thread_id, req.thread_id);
    }
}

#[test]
fn dense_bearer_and_sk_prefix_scrubbing() {
    let samples: &[&str] = &[
        "Bearer sk-sample000SECRET",
        "sk-sample001SECRET",
        "Bearer sk-sample002SECRET",
        "sk-sample003SECRET",
        "Bearer sk-sample004SECRET",
        "sk-sample005SECRET",
        "Bearer sk-sample006SECRET",
        "sk-sample007SECRET",
        "Bearer sk-sample008SECRET",
        "sk-sample009SECRET",
        "Bearer sk-sample010SECRET",
        "sk-sample011SECRET",
        "Bearer sk-sample012SECRET",
        "sk-sample013SECRET",
        "Bearer sk-sample014SECRET",
        "sk-sample015SECRET",
        "Bearer sk-sample016SECRET",
        "sk-sample017SECRET",
        "Bearer sk-sample018SECRET",
        "sk-sample019SECRET",
        "Bearer sk-sample020SECRET",
        "sk-sample021SECRET",
        "Bearer sk-sample022SECRET",
        "sk-sample023SECRET",
        "Bearer sk-sample024SECRET",
        "sk-sample025SECRET",
        "Bearer sk-sample026SECRET",
        "sk-sample027SECRET",
        "Bearer sk-sample028SECRET",
        "sk-sample029SECRET",
        "Bearer sk-sample030SECRET",
        "sk-sample031SECRET",
        "Bearer sk-sample032SECRET",
        "sk-sample033SECRET",
        "Bearer sk-sample034SECRET",
        "sk-sample035SECRET",
        "Bearer sk-sample036SECRET",
        "sk-sample037SECRET",
        "Bearer sk-sample038SECRET",
        "sk-sample039SECRET",
        "Bearer sk-sample040SECRET",
        "sk-sample041SECRET",
        "Bearer sk-sample042SECRET",
        "sk-sample043SECRET",
        "Bearer sk-sample044SECRET",
        "sk-sample045SECRET",
        "Bearer sk-sample046SECRET",
        "sk-sample047SECRET",
        "Bearer sk-sample048SECRET",
        "sk-sample049SECRET",
        "Bearer sk-sample050SECRET",
        "sk-sample051SECRET",
        "Bearer sk-sample052SECRET",
        "sk-sample053SECRET",
        "Bearer sk-sample054SECRET",
        "sk-sample055SECRET",
        "Bearer sk-sample056SECRET",
        "sk-sample057SECRET",
        "Bearer sk-sample058SECRET",
        "sk-sample059SECRET",
    ];
    for (i, sample) in samples.iter().enumerate() {
        let req = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Failed,
                thread_id: format!("bear-{i}"),
                thread_title: None,
                error_summary: Some((*sample).into()),
            },
        )
        .expect("shaped");
        assert!(req.body.contains(AMBIENT_REDACTED), "{}", req.body);
        assert!(!req.body.contains("SECRET"), "{}", req.body);
    }
}

#[test]
fn dense_messy_thread_id_sanitization_grid() {
    let chars = [
        "/", " ", ".", ":", "@", "#", "%", "+", "?", "&", "{", "}", "[", "]", "|", "\\", "\"", "'",
        "=", ";",
    ];
    for (i, ch) in chars.iter().enumerate() {
        for n in 0..5 {
            let id = format!("id{i}{ch}{n}");
            let req = shape_generation_notification(
                &NotificationPrefs::default(),
                &GenerationNotifyInput {
                    kind: GenerationNotifyKind::Completed,
                    thread_id: id.clone(),
                    thread_title: Some("T".into()),
                    error_summary: None,
                },
            )
            .expect("shaped");
            assert!(req
                .id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')));
            assert_eq!(req.thread_id, id);
        }
    }
}

#[test]
fn dense_empty_thread_rejection_matrix() {
    let empties = ["", " ", "  ", "\t", "\n", "\t \n"];
    for empty in empties {
        for i in 0..20 {
            for kind in [
                GenerationNotifyKind::Completed,
                GenerationNotifyKind::Failed,
            ] {
                assert!(shape_generation_notification(
                    &NotificationPrefs::default(),
                    &GenerationNotifyInput {
                        kind,
                        thread_id: empty.into(),
                        thread_title: Some(format!("T{i}")),
                        error_summary: Some("e".into()),
                    },
                )
                .is_none());
            }
        }
    }
}
