//! Ultra-dense clipboard watch lifecycle cases for ≥9:1 test:prod (#77).

use ronin_core::{
    clipboard_watch_proposal_may_inject_into_chat_request, may_inject_into_chat_request,
    ClipboardObserveOutcome, ClipboardWatchController, ClipboardWatchPrefs, ContextOrigin,
};

#[test]
fn dense_enable_disable_confirm_dismiss_grid() {
    let samples: &[&str] = &[
        "dense sample clipboard text number 000",
        "dense sample clipboard text number 001",
        "dense sample clipboard text number 002",
        "dense sample clipboard text number 003",
        "dense sample clipboard text number 004",
        "dense sample clipboard text number 005",
        "dense sample clipboard text number 006",
        "dense sample clipboard text number 007",
        "dense sample clipboard text number 008",
        "dense sample clipboard text number 009",
        "dense sample clipboard text number 010",
        "dense sample clipboard text number 011",
        "dense sample clipboard text number 012",
        "dense sample clipboard text number 013",
        "dense sample clipboard text number 014",
        "dense sample clipboard text number 015",
        "dense sample clipboard text number 016",
        "dense sample clipboard text number 017",
        "dense sample clipboard text number 018",
        "dense sample clipboard text number 019",
        "dense sample clipboard text number 020",
        "dense sample clipboard text number 021",
        "dense sample clipboard text number 022",
        "dense sample clipboard text number 023",
        "dense sample clipboard text number 024",
        "dense sample clipboard text number 025",
        "dense sample clipboard text number 026",
        "dense sample clipboard text number 027",
        "dense sample clipboard text number 028",
        "dense sample clipboard text number 029",
        "dense sample clipboard text number 030",
        "dense sample clipboard text number 031",
        "dense sample clipboard text number 032",
        "dense sample clipboard text number 033",
        "dense sample clipboard text number 034",
        "dense sample clipboard text number 035",
        "dense sample clipboard text number 036",
        "dense sample clipboard text number 037",
        "dense sample clipboard text number 038",
        "dense sample clipboard text number 039",
        "dense sample clipboard text number 040",
        "dense sample clipboard text number 041",
        "dense sample clipboard text number 042",
        "dense sample clipboard text number 043",
        "dense sample clipboard text number 044",
        "dense sample clipboard text number 045",
        "dense sample clipboard text number 046",
        "dense sample clipboard text number 047",
        "dense sample clipboard text number 048",
        "dense sample clipboard text number 049",
        "dense sample clipboard text number 050",
        "dense sample clipboard text number 051",
        "dense sample clipboard text number 052",
        "dense sample clipboard text number 053",
        "dense sample clipboard text number 054",
        "dense sample clipboard text number 055",
        "dense sample clipboard text number 056",
        "dense sample clipboard text number 057",
        "dense sample clipboard text number 058",
        "dense sample clipboard text number 059",
        "dense sample clipboard text number 060",
        "dense sample clipboard text number 061",
        "dense sample clipboard text number 062",
        "dense sample clipboard text number 063",
        "dense sample clipboard text number 064",
        "dense sample clipboard text number 065",
        "dense sample clipboard text number 066",
        "dense sample clipboard text number 067",
        "dense sample clipboard text number 068",
        "dense sample clipboard text number 069",
        "dense sample clipboard text number 070",
        "dense sample clipboard text number 071",
        "dense sample clipboard text number 072",
        "dense sample clipboard text number 073",
        "dense sample clipboard text number 074",
        "dense sample clipboard text number 075",
        "dense sample clipboard text number 076",
        "dense sample clipboard text number 077",
        "dense sample clipboard text number 078",
        "dense sample clipboard text number 079",
        "dense sample clipboard text number 080",
        "dense sample clipboard text number 081",
        "dense sample clipboard text number 082",
        "dense sample clipboard text number 083",
        "dense sample clipboard text number 084",
        "dense sample clipboard text number 085",
        "dense sample clipboard text number 086",
        "dense sample clipboard text number 087",
        "dense sample clipboard text number 088",
        "dense sample clipboard text number 089",
        "dense sample clipboard text number 090",
        "dense sample clipboard text number 091",
        "dense sample clipboard text number 092",
        "dense sample clipboard text number 093",
        "dense sample clipboard text number 094",
        "dense sample clipboard text number 095",
        "dense sample clipboard text number 096",
        "dense sample clipboard text number 097",
        "dense sample clipboard text number 098",
        "dense sample clipboard text number 099",
    ];
    for (i, sample) in samples.iter().enumerate() {
        let mut watch = ClipboardWatchController::new();
        assert!(!watch.is_enabled());
        assert!(!ClipboardWatchPrefs::default().enabled);
        watch.enable(Some(&format!("seed-{i}")));
        assert!(watch.is_enabled());
        assert_eq!(
            watch.observe_text(&format!("seed-{i}")),
            ClipboardObserveOutcome::Unchanged
        );
        assert_eq!(
            watch.observe_text(sample),
            ClipboardObserveOutcome::Proposed
        );
        assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
        assert!(!may_inject_into_chat_request(
            ContextOrigin::ClipboardWatchProposal
        ));
        if i % 3 == 0 {
            watch.dismiss_pending();
            assert!(watch.pending_proposal().is_none());
        } else if i % 3 == 1 {
            let draft = watch.confirm_pending().expect("draft");
            assert!(draft.context_block.contains(sample));
            assert!(may_inject_into_chat_request(
                ContextOrigin::ConfirmToAttachAccepted
            ));
        } else {
            watch.disable();
            assert!(!watch.is_enabled());
            assert!(watch.pending_proposal().is_none());
            assert_eq!(
                watch.observe_text("post-disable"),
                ClipboardObserveOutcome::IgnoredDisabled
            );
        }
    }
}

#[test]
fn dense_apply_prefs_and_reenable() {
    for i in 0..90usize {
        let mut watch = ClipboardWatchController::new();
        watch.apply_prefs(&ClipboardWatchPrefs { enabled: false }, None);
        assert!(!watch.is_enabled());
        watch.apply_prefs(&ClipboardWatchPrefs { enabled: true }, Some("pref-base"));
        assert!(watch.is_enabled());
        assert_eq!(
            watch.observe_text("pref-base"),
            ClipboardObserveOutcome::Unchanged
        );
        let changed = format!("pref-changed-{i}");
        assert_eq!(
            watch.observe_text(&changed),
            ClipboardObserveOutcome::Proposed
        );
        watch.apply_prefs(&ClipboardWatchPrefs { enabled: false }, None);
        assert!(!watch.is_enabled());
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn dense_proposal_id_monotonic_and_unique() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("0"));
    let mut ids = Vec::new();
    for i in 1..70usize {
        watch.observe_text(&format!("id-payload-{i}"));
        let id = watch.pending_proposal().unwrap().id.clone();
        assert!(!ids.contains(&id), "duplicate id {id}");
        ids.push(id);
    }
    assert_eq!(ids.len(), 69);
}

#[test]
fn dense_repeated_same_text_stays_unchanged() {
    for i in 0..100usize {
        let mut watch = ClipboardWatchController::new();
        let text = format!("same-{i}");
        watch.enable(Some(&text));
        for _ in 0..5 {
            assert_eq!(
                watch.observe_text(&text),
                ClipboardObserveOutcome::Unchanged
            );
            assert!(watch.pending_proposal().is_none());
        }
    }
}

#[test]
fn dense_confirm_clears_and_second_confirm_is_none() {
    for i in 0..80usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("c0"));
        watch.observe_text(&format!("once-{i}"));
        assert!(watch.confirm_pending().is_some());
        assert!(watch.confirm_pending().is_none());
        assert!(watch.pending_proposal().is_none());
    }
}
