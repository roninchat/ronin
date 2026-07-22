//! Dense corpus for desktop notification shaping / scrub / inject gates (#75).

use ronin_core::{
    interpret_notification_action, may_inject_into_chat_request,
    notification_may_inject_into_chat_request, notification_payload_origin,
    shape_generation_notification, DesktopNotifier, GenerationNotifyInput, GenerationNotifyKind,
    NotificationPrefs, NullDesktopNotifier, RecordingDesktopNotifier, AMBIENT_REDACTED,
    FOCUS_THREAD_ACTION,
};

fn prefs(enabled: bool) -> NotificationPrefs {
    NotificationPrefs { enabled }
}

fn input(
    kind: GenerationNotifyKind,
    thread_id: &str,
    title: Option<&str>,
    err: Option<&str>,
) -> GenerationNotifyInput {
    GenerationNotifyInput {
        kind,
        thread_id: thread_id.into(),
        thread_title: title.map(str::to_string),
        error_summary: err.map(str::to_string),
    }
}

#[test]
fn corpus_completed_titles_produce_focusable_notifications() {
    let titles = [
        "Alpha",
        "Beta plan",
        "γ-thread",
        "Project / notes",
        "2026-07-23 standup",
        "Quotes \"inside\"",
        "Path-like ~/src/ronin",
        "Empty-looking",
        "Z",
        "Long title with many words about the weekly planning session",
        "Another sufficiently distinct thread title for coverage",
    ];
    for (i, title) in titles.iter().enumerate() {
        let thread_id = format!("c-{i}");
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                GenerationNotifyKind::Completed,
                &thread_id,
                Some(title),
                None,
            ),
        )
        .unwrap_or_else(|| panic!("expected shaped for {title}"));
        assert_eq!(req.kind, GenerationNotifyKind::Completed);
        assert_eq!(req.thread_id, thread_id);
        assert!(req.body.contains(title.trim()) || title.trim().is_empty());
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert!(!may_inject_into_chat_request(notification_payload_origin()));
    }
}

#[test]
fn corpus_failed_errors_scrub_secret_bearing_fragments() {
    let secrets: &[(&str, &str)] = &[
        ("api_key=aaaa", "aaaa"),
        ("token=bbbb", "bbbb"),
        ("key=cccc", "cccc"),
        ("secret=dddd", "dddd"),
        ("password=eeee", "eeee"),
        ("access_token=ffff", "ffff"),
        ("Bearer sk-gggg", "sk-gggg"),
        ("sk-hhhhIIII", "hhhhIIII"),
        ("api_key=\"jjjj\"", "jjjj"),
        ("token='kkkk'", "kkkk"),
        ("password=\"p w\"", "p w"),
        ("key=mmmm&more", "mmmm"),
        ("secret=nnnn,trail", "nnnn"),
        ("ACCESS_TOKEN=OOOO", "OOOO"),
        ("Api_Key=PPPP", "PPPP"),
        ("TOKEN=QQQQ", "QQQQ"),
        ("Bearer sk-rrrr", "sk-rrrr"),
        ("api_key=ssss token=tttt", "ssss"),
        ("prefix api_key=uuuu suffix", "uuuu"),
        ("err: password=vvvv please", "vvvv"),
    ];
    for (i, (err, leak)) in secrets.iter().enumerate() {
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                GenerationNotifyKind::Failed,
                &format!("s-{i}"),
                Some("Job"),
                Some(err),
            ),
        )
        .expect("shaped");
        assert!(
            req.body.contains(AMBIENT_REDACTED),
            "expected redaction in {err:?} → {}",
            req.body
        );
        assert!(
            !req.body.contains(leak),
            "leak {leak:?} still present in {}",
            req.body
        );
        assert!(!notification_may_inject_into_chat_request());
    }
}

#[test]
fn corpus_disabled_never_shapes_across_many_threads() {
    for i in 0..40 {
        let id = format!("quiet-{i}");
        assert!(shape_generation_notification(
            &prefs(false),
            &input(GenerationNotifyKind::Completed, &id, Some("T"), None),
        )
        .is_none());
        assert!(shape_generation_notification(
            &prefs(false),
            &input(GenerationNotifyKind::Failed, &id, None, Some("e")),
        )
        .is_none());
    }
}

#[test]
fn corpus_recording_notifier_preserves_request_order() {
    let notifier = RecordingDesktopNotifier::new();
    for i in 0..25 {
        let kind = if i % 2 == 0 {
            GenerationNotifyKind::Completed
        } else {
            GenerationNotifyKind::Failed
        };
        let req = shape_generation_notification(
            &prefs(true),
            &input(kind, &format!("ord-{i}"), Some("T"), Some("e")),
        )
        .expect("shaped");
        notifier.notify(&req).expect("deliver");
    }
    assert_eq!(notifier.len(), 25);
    let sent = notifier.take_sent();
    assert_eq!(sent.len(), 25);
    for (i, req) in sent.iter().enumerate() {
        assert_eq!(req.thread_id, format!("ord-{i}"));
        if i % 2 == 0 {
            assert_eq!(req.kind, GenerationNotifyKind::Completed);
        } else {
            assert_eq!(req.kind, GenerationNotifyKind::Failed);
        }
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
    }
}

#[test]
fn corpus_null_notifier_swallows_deliveries() {
    let notifier = NullDesktopNotifier;
    for i in 0..20 {
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                GenerationNotifyKind::Completed,
                &format!("n-{i}"),
                None,
                None,
            ),
        )
        .expect("shaped");
        notifier.notify(&req).expect("ok");
    }
}

#[test]
fn corpus_focus_action_interpretation_matrix() {
    for i in 0..30 {
        let thread = format!("focus-{i}");
        let ok = interpret_notification_action(FOCUS_THREAD_ACTION, &thread).expect("focus");
        assert_eq!(ok.thread_id, thread);
        assert!(interpret_notification_action("other", &thread).is_none());
        assert!(interpret_notification_action("", &thread).is_none());
    }
}

#[test]
fn corpus_notification_origin_blocked_for_every_shaped_payload() {
    assert!(!notification_may_inject_into_chat_request());
    for i in 0..50 {
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                if i % 2 == 0 {
                    GenerationNotifyKind::Completed
                } else {
                    GenerationNotifyKind::Failed
                },
                &format!("inj-{i}"),
                Some(&format!("title-{i}")),
                Some(&format!("err-{i}")),
            ),
        )
        .expect("shaped");
        // Payload text is ambient; trust gate stays closed regardless of content.
        assert!(!may_inject_into_chat_request(notification_payload_origin()));
        assert!(!req.title.is_empty());
        assert!(!req.body.is_empty());
        assert!(req.buttons.iter().all(|b| !b.action_id.is_empty()));
    }
}

#[test]
fn corpus_id_sanitization_for_messy_thread_ids() {
    let messy = [
        "plain",
        "with space",
        "path/like",
        "dot.dot",
        "under_score",
        "dash-ok",
        "unicode-αβ",
        "emoji-🙂",
        "quote\"here",
        "colon:here",
        "at@here",
        "hash#here",
        "percent%20",
        "plus+plus",
        "q?mark",
        "amp&ersand",
        "brace{x}",
        "bracket[x]",
        "pipe|x",
        "slash\\x",
    ];
    for (i, id) in messy.iter().enumerate() {
        let req = shape_generation_notification(
            &prefs(true),
            &input(GenerationNotifyKind::Completed, id, Some("T"), None),
        )
        .unwrap_or_else(|| panic!("shape {id}"));
        assert!(
            req.id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')),
            "unsafe id chars for {id:?}: {}",
            req.id
        );
        assert!(req.id.contains("done") || req.id.contains("fail"));
        assert_eq!(req.thread_id, *id);
        let _ = i;
    }
}

#[test]
fn corpus_enabled_toggle_round_trips_per_kind() {
    for enabled in [true, false] {
        for kind in [
            GenerationNotifyKind::Completed,
            GenerationNotifyKind::Failed,
        ] {
            for i in 0..15 {
                let shaped = shape_generation_notification(
                    &prefs(enabled),
                    &input(kind, &format!("tog-{i}"), Some("Title"), Some("Err")),
                );
                assert_eq!(
                    shaped.is_some(),
                    enabled,
                    "enabled={enabled} kind={kind:?} i={i}"
                );
                if let Some(req) = shaped {
                    assert_eq!(req.kind, kind);
                    assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
                }
            }
        }
    }
}

#[test]
fn corpus_secret_in_thread_title_is_scrubbed_from_body() {
    let titled = [
        "api_key=title-secret-1",
        "token=title-secret-2",
        "Bearer sk-title-3",
        "password=title-secret-4",
        "key=title-secret-5",
        "secret=title-secret-6",
        "access_token=title-secret-7",
        "sk-title-secret-8",
        "api_key=\"title-secret-9\"",
        "token='title-secret-10'",
    ];
    for (i, title) in titled.iter().enumerate() {
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                GenerationNotifyKind::Completed,
                &format!("tt-{i}"),
                Some(title),
                None,
            ),
        )
        .expect("shaped");
        assert!(
            req.body.contains(AMBIENT_REDACTED),
            "title={title:?} body={}",
            req.body
        );
        for leak in [
            "title-secret-1",
            "title-secret-2",
            "sk-title-3",
            "title-secret-4",
            "title-secret-5",
            "title-secret-6",
            "title-secret-7",
            "title-secret-8",
            "title-secret-9",
            "title-secret-10",
        ] {
            if title.contains(leak) || title.contains(&leak.replace("sk-", "")) {
                assert!(!req.body.contains(leak), "leak {leak} in {}", req.body);
            }
        }
    }
}

#[test]
fn corpus_button_labels_stable_across_requests() {
    for i in 0..20 {
        let req = shape_generation_notification(
            &prefs(true),
            &input(
                GenerationNotifyKind::Failed,
                &format!("btn-{i}"),
                None,
                Some("e"),
            ),
        )
        .expect("shaped");
        assert_eq!(req.buttons.len(), 1);
        assert_eq!(req.buttons[0].label, "Open thread");
        assert_eq!(req.buttons[0].action_id, FOCUS_THREAD_ACTION);
    }
}
