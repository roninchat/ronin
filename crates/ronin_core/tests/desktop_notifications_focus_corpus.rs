//! Focus-action corpus for notification requests (#75).

use ronin_core::{
    interpret_notification_action, shape_generation_notification, GenerationNotifyInput,
    GenerationNotifyKind, NotificationPrefs, FOCUS_THREAD_ACTION,
};

#[test]
fn focus_corpus_completed_failed_and_disabled() {
    for i in 0..120 {
        for enabled in [true, false] {
            for kind in [
                GenerationNotifyKind::Completed,
                GenerationNotifyKind::Failed,
            ] {
                let shaped = shape_generation_notification(
                    &NotificationPrefs { enabled },
                    &GenerationNotifyInput {
                        kind,
                        thread_id: format!("fc-{i}"),
                        thread_title: Some(format!("Title {i}")),
                        error_summary: Some("err".into()),
                    },
                );
                assert_eq!(shaped.is_some(), enabled);
                if let Some(req) = shaped {
                    assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
                    let focus =
                        interpret_notification_action(FOCUS_THREAD_ACTION, &req.thread_id).unwrap();
                    assert_eq!(focus.thread_id, req.thread_id);
                }
            }
        }
    }
}

#[test]
fn focus_corpus_explicit_thread_table() {
    let threads: &[&str] = &[
        "explicit-focus-thread-000",
        "explicit-focus-thread-001",
        "explicit-focus-thread-002",
        "explicit-focus-thread-003",
        "explicit-focus-thread-004",
        "explicit-focus-thread-005",
        "explicit-focus-thread-006",
        "explicit-focus-thread-007",
        "explicit-focus-thread-008",
        "explicit-focus-thread-009",
        "explicit-focus-thread-010",
        "explicit-focus-thread-011",
        "explicit-focus-thread-012",
        "explicit-focus-thread-013",
        "explicit-focus-thread-014",
        "explicit-focus-thread-015",
        "explicit-focus-thread-016",
        "explicit-focus-thread-017",
        "explicit-focus-thread-018",
        "explicit-focus-thread-019",
        "explicit-focus-thread-020",
        "explicit-focus-thread-021",
        "explicit-focus-thread-022",
        "explicit-focus-thread-023",
        "explicit-focus-thread-024",
        "explicit-focus-thread-025",
        "explicit-focus-thread-026",
        "explicit-focus-thread-027",
        "explicit-focus-thread-028",
        "explicit-focus-thread-029",
        "explicit-focus-thread-030",
        "explicit-focus-thread-031",
        "explicit-focus-thread-032",
        "explicit-focus-thread-033",
        "explicit-focus-thread-034",
        "explicit-focus-thread-035",
        "explicit-focus-thread-036",
        "explicit-focus-thread-037",
        "explicit-focus-thread-038",
        "explicit-focus-thread-039",
        "explicit-focus-thread-040",
        "explicit-focus-thread-041",
        "explicit-focus-thread-042",
        "explicit-focus-thread-043",
        "explicit-focus-thread-044",
        "explicit-focus-thread-045",
        "explicit-focus-thread-046",
        "explicit-focus-thread-047",
        "explicit-focus-thread-048",
        "explicit-focus-thread-049",
        "explicit-focus-thread-050",
        "explicit-focus-thread-051",
        "explicit-focus-thread-052",
        "explicit-focus-thread-053",
        "explicit-focus-thread-054",
        "explicit-focus-thread-055",
        "explicit-focus-thread-056",
        "explicit-focus-thread-057",
        "explicit-focus-thread-058",
        "explicit-focus-thread-059",
        "explicit-focus-thread-060",
        "explicit-focus-thread-061",
        "explicit-focus-thread-062",
        "explicit-focus-thread-063",
        "explicit-focus-thread-064",
        "explicit-focus-thread-065",
        "explicit-focus-thread-066",
        "explicit-focus-thread-067",
        "explicit-focus-thread-068",
        "explicit-focus-thread-069",
        "explicit-focus-thread-070",
        "explicit-focus-thread-071",
        "explicit-focus-thread-072",
        "explicit-focus-thread-073",
        "explicit-focus-thread-074",
        "explicit-focus-thread-075",
        "explicit-focus-thread-076",
        "explicit-focus-thread-077",
        "explicit-focus-thread-078",
        "explicit-focus-thread-079",
        "explicit-focus-thread-080",
        "explicit-focus-thread-081",
        "explicit-focus-thread-082",
        "explicit-focus-thread-083",
        "explicit-focus-thread-084",
        "explicit-focus-thread-085",
        "explicit-focus-thread-086",
        "explicit-focus-thread-087",
        "explicit-focus-thread-088",
        "explicit-focus-thread-089",
        "explicit-focus-thread-090",
        "explicit-focus-thread-091",
        "explicit-focus-thread-092",
        "explicit-focus-thread-093",
        "explicit-focus-thread-094",
        "explicit-focus-thread-095",
        "explicit-focus-thread-096",
        "explicit-focus-thread-097",
        "explicit-focus-thread-098",
        "explicit-focus-thread-099",
        "explicit-focus-thread-100",
        "explicit-focus-thread-101",
        "explicit-focus-thread-102",
        "explicit-focus-thread-103",
        "explicit-focus-thread-104",
        "explicit-focus-thread-105",
        "explicit-focus-thread-106",
        "explicit-focus-thread-107",
        "explicit-focus-thread-108",
        "explicit-focus-thread-109",
        "explicit-focus-thread-110",
        "explicit-focus-thread-111",
        "explicit-focus-thread-112",
        "explicit-focus-thread-113",
        "explicit-focus-thread-114",
        "explicit-focus-thread-115",
        "explicit-focus-thread-116",
        "explicit-focus-thread-117",
        "explicit-focus-thread-118",
        "explicit-focus-thread-119",
        "explicit-focus-thread-120",
        "explicit-focus-thread-121",
        "explicit-focus-thread-122",
        "explicit-focus-thread-123",
        "explicit-focus-thread-124",
        "explicit-focus-thread-125",
        "explicit-focus-thread-126",
        "explicit-focus-thread-127",
        "explicit-focus-thread-128",
        "explicit-focus-thread-129",
        "explicit-focus-thread-130",
        "explicit-focus-thread-131",
        "explicit-focus-thread-132",
        "explicit-focus-thread-133",
        "explicit-focus-thread-134",
        "explicit-focus-thread-135",
        "explicit-focus-thread-136",
        "explicit-focus-thread-137",
        "explicit-focus-thread-138",
        "explicit-focus-thread-139",
        "explicit-focus-thread-140",
        "explicit-focus-thread-141",
        "explicit-focus-thread-142",
        "explicit-focus-thread-143",
        "explicit-focus-thread-144",
        "explicit-focus-thread-145",
        "explicit-focus-thread-146",
        "explicit-focus-thread-147",
        "explicit-focus-thread-148",
        "explicit-focus-thread-149",
        "explicit-focus-thread-150",
        "explicit-focus-thread-151",
        "explicit-focus-thread-152",
        "explicit-focus-thread-153",
        "explicit-focus-thread-154",
        "explicit-focus-thread-155",
        "explicit-focus-thread-156",
        "explicit-focus-thread-157",
        "explicit-focus-thread-158",
        "explicit-focus-thread-159",
        "explicit-focus-thread-160",
        "explicit-focus-thread-161",
        "explicit-focus-thread-162",
        "explicit-focus-thread-163",
        "explicit-focus-thread-164",
        "explicit-focus-thread-165",
        "explicit-focus-thread-166",
        "explicit-focus-thread-167",
        "explicit-focus-thread-168",
        "explicit-focus-thread-169",
        "explicit-focus-thread-170",
        "explicit-focus-thread-171",
        "explicit-focus-thread-172",
        "explicit-focus-thread-173",
        "explicit-focus-thread-174",
        "explicit-focus-thread-175",
        "explicit-focus-thread-176",
        "explicit-focus-thread-177",
        "explicit-focus-thread-178",
        "explicit-focus-thread-179",
        "explicit-focus-thread-180",
        "explicit-focus-thread-181",
        "explicit-focus-thread-182",
        "explicit-focus-thread-183",
        "explicit-focus-thread-184",
        "explicit-focus-thread-185",
        "explicit-focus-thread-186",
        "explicit-focus-thread-187",
        "explicit-focus-thread-188",
        "explicit-focus-thread-189",
        "explicit-focus-thread-190",
        "explicit-focus-thread-191",
        "explicit-focus-thread-192",
        "explicit-focus-thread-193",
        "explicit-focus-thread-194",
        "explicit-focus-thread-195",
        "explicit-focus-thread-196",
        "explicit-focus-thread-197",
        "explicit-focus-thread-198",
        "explicit-focus-thread-199",
    ];
    for thread in threads {
        let req = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Completed,
                thread_id: (*thread).into(),
                thread_title: Some("Focus me".into()),
                error_summary: None,
            },
        )
        .expect("shaped");
        assert_eq!(req.thread_id, *thread);
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert_eq!(req.buttons[0].action_id, FOCUS_THREAD_ACTION);
        assert_eq!(
            interpret_notification_action(FOCUS_THREAD_ACTION, thread)
                .unwrap()
                .thread_id,
            *thread
        );
        assert!(interpret_notification_action("noop", thread).is_none());
    }
}

#[test]
fn focus_corpus_failed_kind_also_offers_open_thread() {
    for i in 0..40 {
        let req = shape_generation_notification(
            &NotificationPrefs::default(),
            &GenerationNotifyInput {
                kind: GenerationNotifyKind::Failed,
                thread_id: format!("fail-focus-{i}"),
                thread_title: Some(format!("Fail {i}")),
                error_summary: Some("boom".into()),
            },
        )
        .expect("shaped");
        assert_eq!(req.kind, GenerationNotifyKind::Failed);
        assert_eq!(req.default_action.as_deref(), Some(FOCUS_THREAD_ACTION));
        assert_eq!(req.buttons[0].label, "Open thread");
        assert_eq!(
            interpret_notification_action(FOCUS_THREAD_ACTION, &req.thread_id)
                .unwrap()
                .thread_id,
            req.thread_id
        );
    }
}
