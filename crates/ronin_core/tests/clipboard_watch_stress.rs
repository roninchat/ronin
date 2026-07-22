//! Lifecycle stress table for clipboard watch (#77).

use ronin_core::{ClipboardObserveOutcome, ClipboardWatchController, ClipboardWatchPrefs};

#[test]
fn stress_prefs_toggle_and_observe() {
    let samples: &[&str] = &[
        "stress-sample-000",
        "stress-sample-001",
        "stress-sample-002",
        "stress-sample-003",
        "stress-sample-004",
        "stress-sample-005",
        "stress-sample-006",
        "stress-sample-007",
        "stress-sample-008",
        "stress-sample-009",
        "stress-sample-010",
        "stress-sample-011",
        "stress-sample-012",
        "stress-sample-013",
        "stress-sample-014",
        "stress-sample-015",
        "stress-sample-016",
        "stress-sample-017",
        "stress-sample-018",
        "stress-sample-019",
        "stress-sample-020",
        "stress-sample-021",
        "stress-sample-022",
        "stress-sample-023",
        "stress-sample-024",
        "stress-sample-025",
        "stress-sample-026",
        "stress-sample-027",
        "stress-sample-028",
        "stress-sample-029",
        "stress-sample-030",
        "stress-sample-031",
        "stress-sample-032",
        "stress-sample-033",
        "stress-sample-034",
        "stress-sample-035",
        "stress-sample-036",
        "stress-sample-037",
        "stress-sample-038",
        "stress-sample-039",
        "stress-sample-040",
        "stress-sample-041",
        "stress-sample-042",
        "stress-sample-043",
        "stress-sample-044",
        "stress-sample-045",
        "stress-sample-046",
        "stress-sample-047",
        "stress-sample-048",
        "stress-sample-049",
        "stress-sample-050",
        "stress-sample-051",
        "stress-sample-052",
        "stress-sample-053",
        "stress-sample-054",
        "stress-sample-055",
        "stress-sample-056",
        "stress-sample-057",
        "stress-sample-058",
        "stress-sample-059",
        "stress-sample-060",
        "stress-sample-061",
        "stress-sample-062",
        "stress-sample-063",
        "stress-sample-064",
        "stress-sample-065",
        "stress-sample-066",
        "stress-sample-067",
        "stress-sample-068",
        "stress-sample-069",
        "stress-sample-070",
        "stress-sample-071",
        "stress-sample-072",
        "stress-sample-073",
        "stress-sample-074",
        "stress-sample-075",
        "stress-sample-076",
        "stress-sample-077",
        "stress-sample-078",
        "stress-sample-079",
        "stress-sample-080",
        "stress-sample-081",
        "stress-sample-082",
        "stress-sample-083",
        "stress-sample-084",
        "stress-sample-085",
        "stress-sample-086",
        "stress-sample-087",
        "stress-sample-088",
        "stress-sample-089",
        "stress-sample-090",
        "stress-sample-091",
        "stress-sample-092",
        "stress-sample-093",
        "stress-sample-094",
        "stress-sample-095",
        "stress-sample-096",
        "stress-sample-097",
        "stress-sample-098",
        "stress-sample-099",
        "stress-sample-100",
        "stress-sample-101",
        "stress-sample-102",
        "stress-sample-103",
        "stress-sample-104",
        "stress-sample-105",
        "stress-sample-106",
        "stress-sample-107",
        "stress-sample-108",
        "stress-sample-109",
        "stress-sample-110",
        "stress-sample-111",
        "stress-sample-112",
        "stress-sample-113",
        "stress-sample-114",
        "stress-sample-115",
        "stress-sample-116",
        "stress-sample-117",
        "stress-sample-118",
        "stress-sample-119",
        "stress-sample-120",
        "stress-sample-121",
        "stress-sample-122",
        "stress-sample-123",
        "stress-sample-124",
        "stress-sample-125",
        "stress-sample-126",
        "stress-sample-127",
        "stress-sample-128",
        "stress-sample-129",
        "stress-sample-130",
        "stress-sample-131",
        "stress-sample-132",
        "stress-sample-133",
        "stress-sample-134",
        "stress-sample-135",
        "stress-sample-136",
        "stress-sample-137",
        "stress-sample-138",
        "stress-sample-139",
        "stress-sample-140",
        "stress-sample-141",
        "stress-sample-142",
        "stress-sample-143",
        "stress-sample-144",
        "stress-sample-145",
        "stress-sample-146",
        "stress-sample-147",
        "stress-sample-148",
        "stress-sample-149",
        "stress-sample-150",
        "stress-sample-151",
        "stress-sample-152",
        "stress-sample-153",
        "stress-sample-154",
        "stress-sample-155",
        "stress-sample-156",
        "stress-sample-157",
        "stress-sample-158",
        "stress-sample-159",
        "stress-sample-160",
        "stress-sample-161",
        "stress-sample-162",
        "stress-sample-163",
        "stress-sample-164",
        "stress-sample-165",
        "stress-sample-166",
        "stress-sample-167",
        "stress-sample-168",
        "stress-sample-169",
        "stress-sample-170",
        "stress-sample-171",
        "stress-sample-172",
        "stress-sample-173",
        "stress-sample-174",
        "stress-sample-175",
        "stress-sample-176",
        "stress-sample-177",
        "stress-sample-178",
        "stress-sample-179",
    ];
    for (i, sample) in samples.iter().enumerate() {
        let mut watch = ClipboardWatchController::new();
        assert!(!ClipboardWatchPrefs::default().enabled);
        watch.apply_prefs(&ClipboardWatchPrefs { enabled: true }, Some("stress-base"));
        assert!(watch.is_enabled());
        assert_eq!(
            watch.observe_text("stress-base"),
            ClipboardObserveOutcome::Unchanged
        );
        assert_eq!(
            watch.observe_text(sample),
            ClipboardObserveOutcome::Proposed
        );
        if i % 2 == 0 {
            watch.dismiss_pending();
        } else {
            assert!(watch.confirm_pending().is_some());
        }
        watch.apply_prefs(&ClipboardWatchPrefs { enabled: false }, None);
        assert!(!watch.is_enabled());
        assert_eq!(
            watch.observe_text("nope"),
            ClipboardObserveOutcome::IgnoredDisabled
        );
    }
}

#[test]
fn stress_empty_and_whitespace_ignored() {
    let empties: &[&str] = &[
        "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t",
        "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ",
        "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ",
        "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t",
        "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ",
        "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ",
        "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t",
        "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ",
        "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ",
        "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t",
        "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ",
        "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ",
        "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t",
        "\n", " \t ", "   ", "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ",
        "\t\t", " \n ", "    ", "", " ", "  ", "\t", "\n", " \t ", "   ", "\t\t", " \n ", "    ",
    ];
    for empty in empties {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("keep"));
        assert_eq!(
            watch.observe_text(empty),
            ClipboardObserveOutcome::IgnoredEmpty
        );
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn stress_reenable_clears_old_proposal() {
    for i in 0..100usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("e0"));
        watch.observe_text(&format!("old-{i}"));
        assert!(watch.pending_proposal().is_some());
        watch.enable(Some("e1"));
        assert!(watch.pending_proposal().is_none());
        assert_eq!(watch.observe_text("e1"), ClipboardObserveOutcome::Unchanged);
    }
}
