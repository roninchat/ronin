//! Pattern table: generation notification shaping across many inputs (#75).

use ronin_core::{
    may_inject_into_chat_request, notification_payload_origin, shape_generation_notification,
    GenerationNotifyInput, GenerationNotifyKind, NotificationPrefs, AMBIENT_REDACTED,
    FOCUS_THREAD_ACTION,
};

const SECRET_PATTERNS: &[&str] = &[
    "api_key=alpha",
    "api_key=bravo",
    "api_key=charlie",
    "api_key=delta",
    "api_key=echo",
    "token=foxtrot",
    "token=golf",
    "token=hotel",
    "token=india",
    "token=juliet",
    "key=kilo",
    "key=lima",
    "key=mike",
    "key=november",
    "key=oscar",
    "secret=papa",
    "secret=quebec",
    "secret=romeo",
    "secret=sierra",
    "secret=tango",
    "password=uniform",
    "password=victor",
    "password=whiskey",
    "password=xray",
    "password=yankee",
    "access_token=zulu",
    "access_token=one",
    "access_token=two",
    "access_token=three",
    "access_token=four",
    "Bearer sk-five",
    "Bearer sk-six",
    "Bearer sk-seven",
    "Bearer sk-eight",
    "Bearer sk-nine",
    "sk-tenABCDEF",
    "sk-elevenGHIJ",
    "sk-twelveKLMN",
    "sk-thirteenOP",
    "sk-fourteenQR",
    "api_key=\"quoted1\"",
    "api_key='quoted2'",
    "token=\"quoted3\"",
    "token='quoted4'",
    "password=\"quoted5\"",
    "password='quoted6'",
    "key=\"quoted7\"",
    "key='quoted8'",
    "secret=\"quoted9\"",
    "secret='quoted10'",
];

const LEAK_FRAGMENTS: &[&str] = &[
    "alpha",
    "bravo",
    "charlie",
    "delta",
    "echo",
    "foxtrot",
    "golf",
    "hotel",
    "india",
    "juliet",
    "kilo",
    "lima",
    "mike",
    "november",
    "oscar",
    "papa",
    "quebec",
    "romeo",
    "sierra",
    "tango",
    "uniform",
    "victor",
    "whiskey",
    "xray",
    "yankee",
    "zulu",
    "one",
    "two",
    "three",
    "four",
    "sk-five",
    "sk-six",
    "sk-seven",
    "sk-eight",
    "sk-nine",
    "tenABCDEF",
    "elevenGHIJ",
    "twelveKLMN",
    "thirteenOP",
    "fourteenQR",
    "quoted1",
    "quoted2",
    "quoted3",
    "quoted4",
    "quoted5",
    "quoted6",
    "quoted7",
    "quoted8",
    "quoted9",
    "quoted10",
];

#[test]
fn pattern_table_scrubs_every_secret_pattern_in_error_summary() {
    assert_eq!(SECRET_PATTERNS.len(), LEAK_FRAGMENTS.len());
    for (i, (pattern, leak)) in SECRET_PATTERNS
        .iter()
        .zip(LEAK_FRAGMENTS.iter())
        .enumerate()
    {
        let req = shape_generation_notification(
            &NotificationPrefs { enabled: true },
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Failed,
                thread_id: format!("pat-{i}"),
                thread_title: Some("Thread".into()),
                error_summary: Some((*pattern).into()),
            },
        )
        .unwrap_or_else(|| panic!("shape {pattern}"));
        assert!(
            req.body.contains(AMBIENT_REDACTED),
            "pattern {pattern} missing redaction: {}",
            req.body
        );
        assert!(
            !req.body.contains(leak),
            "pattern {pattern} leaked {leak} in {}",
            req.body
        );
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert!(!may_inject_into_chat_request(notification_payload_origin()));
    }
}

#[test]
fn pattern_table_scrubs_every_secret_pattern_in_thread_title() {
    for (i, (pattern, leak)) in SECRET_PATTERNS
        .iter()
        .zip(LEAK_FRAGMENTS.iter())
        .enumerate()
    {
        let req = shape_generation_notification(
            &NotificationPrefs { enabled: true },
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Completed,
                thread_id: format!("title-pat-{i}"),
                thread_title: Some((*pattern).into()),
                error_summary: None,
            },
        )
        .unwrap_or_else(|| panic!("shape title {pattern}"));
        assert!(
            req.body.contains(AMBIENT_REDACTED),
            "title pattern {pattern} missing redaction: {}",
            req.body
        );
        assert!(
            !req.body.contains(leak),
            "title pattern {pattern} leaked {leak} in {}",
            req.body
        );
    }
}

#[test]
fn pattern_table_disable_gate_for_each_secret_pattern() {
    for (i, pattern) in SECRET_PATTERNS.iter().enumerate() {
        let shaped = shape_generation_notification(
            &NotificationPrefs { enabled: false },
            &GenerationNotifyInput {
                kind: if i % 2 == 0 {
                    GenerationNotifyKind::Completed
                } else {
                    GenerationNotifyKind::Failed
                },
                thread_id: format!("off-{i}"),
                thread_title: Some((*pattern).into()),
                error_summary: Some((*pattern).into()),
            },
        );
        assert!(shaped.is_none(), "disabled must suppress {pattern}");
    }
}

#[test]
fn pattern_table_completed_and_failed_titles_for_benign_names() {
    let titles = [
        "Design notes",
        "Bug triage",
        "Release checklist",
        "Customer email draft",
        "Local build log",
        "Portal research",
        "Notification UX",
        "Focus action wiring",
        "Config defaults",
        "Secret scrub audit",
        "Thread A",
        "Thread B",
        "Thread C",
        "Thread D",
        "Thread E",
        "Morning standup",
        "Afternoon review",
        "Evening wrap",
        "Weekend plan",
        "Monday goals",
        "Infra checklist",
        "Docs pass",
        "Clippy cleanup",
        "Fmt only",
        "Test densify",
        "M3.0 desktop",
        "Issue 75",
        "PR body",
        "Loop progress",
        "Frontier ticket",
    ];
    for (i, title) in titles.iter().enumerate() {
        for kind in [
            GenerationNotifyKind::Completed,
            GenerationNotifyKind::Failed,
        ] {
            let req = shape_generation_notification(
                &NotificationPrefs::default(),
                &GenerationNotifyInput {
                    kind,
                    thread_id: format!("benign-{i}-{:?}", kind),
                    thread_title: Some((*title).into()),
                    error_summary: if kind == GenerationNotifyKind::Failed {
                        Some("network blip".into())
                    } else {
                        None
                    },
                },
            )
            .expect("shaped");
            assert!(req.body.contains(title));
            assert_eq!(req.kind, kind);
            assert_eq!(req.buttons[0].action_id, FOCUS_THREAD_ACTION);
            assert!(!may_inject_into_chat_request(notification_payload_origin()));
        }
    }
}

#[test]
fn pattern_table_empty_and_whitespace_thread_ids_rejected_for_many_titles() {
    let titles = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for title in titles {
        for thread_id in ["", " ", "  ", "\t", "\n"] {
            for kind in [
                GenerationNotifyKind::Completed,
                GenerationNotifyKind::Failed,
            ] {
                assert!(
                    shape_generation_notification(
                        &NotificationPrefs::default(),
                        &GenerationNotifyInput {
                            kind,
                            thread_id: thread_id.into(),
                            thread_title: Some(title.into()),
                            error_summary: Some("e".into()),
                        },
                    )
                    .is_none(),
                    "thread_id={thread_id:?} title={title}"
                );
            }
        }
    }
}

#[test]
fn pattern_table_id_prefix_and_kind_markers() {
    for i in 0..40 {
        let completed = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Completed,
                thread_id: format!("idc-{i}"),
                thread_title: None,
                error_summary: None,
            },
        )
        .expect("completed");
        assert!(completed.id.contains("done"));
        assert!(completed.id.contains("chat.ronin.generation"));

        let failed = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Failed,
                thread_id: format!("idf-{i}"),
                thread_title: None,
                error_summary: Some("e".into()),
            },
        )
        .expect("failed");
        assert!(failed.id.contains("fail"));
        assert!(failed.id.contains("chat.ronin.generation"));
    }
}
