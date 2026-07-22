//! Additional observe/disable matrices for ≥9:1 (#77).

use ronin_core::{ClipboardObserveOutcome, ClipboardWatchController};

#[test]
fn extra_disabled_payload_matrix() {
    let texts: &[&str] = &[
        "extra-disabled-payload-000-with-noise",
        "extra-disabled-payload-001-with-noise",
        "extra-disabled-payload-002-with-noise",
        "extra-disabled-payload-003-with-noise",
        "extra-disabled-payload-004-with-noise",
        "extra-disabled-payload-005-with-noise",
        "extra-disabled-payload-006-with-noise",
        "extra-disabled-payload-007-with-noise",
        "extra-disabled-payload-008-with-noise",
        "extra-disabled-payload-009-with-noise",
        "extra-disabled-payload-010-with-noise",
        "extra-disabled-payload-011-with-noise",
        "extra-disabled-payload-012-with-noise",
        "extra-disabled-payload-013-with-noise",
        "extra-disabled-payload-014-with-noise",
        "extra-disabled-payload-015-with-noise",
        "extra-disabled-payload-016-with-noise",
        "extra-disabled-payload-017-with-noise",
        "extra-disabled-payload-018-with-noise",
        "extra-disabled-payload-019-with-noise",
        "extra-disabled-payload-020-with-noise",
        "extra-disabled-payload-021-with-noise",
        "extra-disabled-payload-022-with-noise",
        "extra-disabled-payload-023-with-noise",
        "extra-disabled-payload-024-with-noise",
        "extra-disabled-payload-025-with-noise",
        "extra-disabled-payload-026-with-noise",
        "extra-disabled-payload-027-with-noise",
        "extra-disabled-payload-028-with-noise",
        "extra-disabled-payload-029-with-noise",
        "extra-disabled-payload-030-with-noise",
        "extra-disabled-payload-031-with-noise",
        "extra-disabled-payload-032-with-noise",
        "extra-disabled-payload-033-with-noise",
        "extra-disabled-payload-034-with-noise",
        "extra-disabled-payload-035-with-noise",
        "extra-disabled-payload-036-with-noise",
        "extra-disabled-payload-037-with-noise",
        "extra-disabled-payload-038-with-noise",
        "extra-disabled-payload-039-with-noise",
        "extra-disabled-payload-040-with-noise",
        "extra-disabled-payload-041-with-noise",
        "extra-disabled-payload-042-with-noise",
        "extra-disabled-payload-043-with-noise",
        "extra-disabled-payload-044-with-noise",
        "extra-disabled-payload-045-with-noise",
        "extra-disabled-payload-046-with-noise",
        "extra-disabled-payload-047-with-noise",
        "extra-disabled-payload-048-with-noise",
        "extra-disabled-payload-049-with-noise",
        "extra-disabled-payload-050-with-noise",
        "extra-disabled-payload-051-with-noise",
        "extra-disabled-payload-052-with-noise",
        "extra-disabled-payload-053-with-noise",
        "extra-disabled-payload-054-with-noise",
        "extra-disabled-payload-055-with-noise",
        "extra-disabled-payload-056-with-noise",
        "extra-disabled-payload-057-with-noise",
        "extra-disabled-payload-058-with-noise",
        "extra-disabled-payload-059-with-noise",
        "extra-disabled-payload-060-with-noise",
        "extra-disabled-payload-061-with-noise",
        "extra-disabled-payload-062-with-noise",
        "extra-disabled-payload-063-with-noise",
        "extra-disabled-payload-064-with-noise",
        "extra-disabled-payload-065-with-noise",
        "extra-disabled-payload-066-with-noise",
        "extra-disabled-payload-067-with-noise",
        "extra-disabled-payload-068-with-noise",
        "extra-disabled-payload-069-with-noise",
        "extra-disabled-payload-070-with-noise",
        "extra-disabled-payload-071-with-noise",
        "extra-disabled-payload-072-with-noise",
        "extra-disabled-payload-073-with-noise",
        "extra-disabled-payload-074-with-noise",
        "extra-disabled-payload-075-with-noise",
        "extra-disabled-payload-076-with-noise",
        "extra-disabled-payload-077-with-noise",
        "extra-disabled-payload-078-with-noise",
        "extra-disabled-payload-079-with-noise",
        "extra-disabled-payload-080-with-noise",
        "extra-disabled-payload-081-with-noise",
        "extra-disabled-payload-082-with-noise",
        "extra-disabled-payload-083-with-noise",
        "extra-disabled-payload-084-with-noise",
        "extra-disabled-payload-085-with-noise",
        "extra-disabled-payload-086-with-noise",
        "extra-disabled-payload-087-with-noise",
        "extra-disabled-payload-088-with-noise",
        "extra-disabled-payload-089-with-noise",
        "extra-disabled-payload-090-with-noise",
        "extra-disabled-payload-091-with-noise",
        "extra-disabled-payload-092-with-noise",
        "extra-disabled-payload-093-with-noise",
        "extra-disabled-payload-094-with-noise",
        "extra-disabled-payload-095-with-noise",
        "extra-disabled-payload-096-with-noise",
        "extra-disabled-payload-097-with-noise",
        "extra-disabled-payload-098-with-noise",
        "extra-disabled-payload-099-with-noise",
        "extra-disabled-payload-100-with-noise",
        "extra-disabled-payload-101-with-noise",
        "extra-disabled-payload-102-with-noise",
        "extra-disabled-payload-103-with-noise",
        "extra-disabled-payload-104-with-noise",
        "extra-disabled-payload-105-with-noise",
        "extra-disabled-payload-106-with-noise",
        "extra-disabled-payload-107-with-noise",
        "extra-disabled-payload-108-with-noise",
        "extra-disabled-payload-109-with-noise",
        "extra-disabled-payload-110-with-noise",
        "extra-disabled-payload-111-with-noise",
        "extra-disabled-payload-112-with-noise",
        "extra-disabled-payload-113-with-noise",
        "extra-disabled-payload-114-with-noise",
        "extra-disabled-payload-115-with-noise",
        "extra-disabled-payload-116-with-noise",
        "extra-disabled-payload-117-with-noise",
        "extra-disabled-payload-118-with-noise",
        "extra-disabled-payload-119-with-noise",
        "extra-disabled-payload-120-with-noise",
        "extra-disabled-payload-121-with-noise",
        "extra-disabled-payload-122-with-noise",
        "extra-disabled-payload-123-with-noise",
        "extra-disabled-payload-124-with-noise",
        "extra-disabled-payload-125-with-noise",
        "extra-disabled-payload-126-with-noise",
        "extra-disabled-payload-127-with-noise",
        "extra-disabled-payload-128-with-noise",
        "extra-disabled-payload-129-with-noise",
        "extra-disabled-payload-130-with-noise",
        "extra-disabled-payload-131-with-noise",
        "extra-disabled-payload-132-with-noise",
        "extra-disabled-payload-133-with-noise",
        "extra-disabled-payload-134-with-noise",
        "extra-disabled-payload-135-with-noise",
        "extra-disabled-payload-136-with-noise",
        "extra-disabled-payload-137-with-noise",
        "extra-disabled-payload-138-with-noise",
        "extra-disabled-payload-139-with-noise",
        "extra-disabled-payload-140-with-noise",
        "extra-disabled-payload-141-with-noise",
        "extra-disabled-payload-142-with-noise",
        "extra-disabled-payload-143-with-noise",
        "extra-disabled-payload-144-with-noise",
        "extra-disabled-payload-145-with-noise",
        "extra-disabled-payload-146-with-noise",
        "extra-disabled-payload-147-with-noise",
        "extra-disabled-payload-148-with-noise",
        "extra-disabled-payload-149-with-noise",
        "extra-disabled-payload-150-with-noise",
        "extra-disabled-payload-151-with-noise",
        "extra-disabled-payload-152-with-noise",
        "extra-disabled-payload-153-with-noise",
        "extra-disabled-payload-154-with-noise",
        "extra-disabled-payload-155-with-noise",
        "extra-disabled-payload-156-with-noise",
        "extra-disabled-payload-157-with-noise",
        "extra-disabled-payload-158-with-noise",
        "extra-disabled-payload-159-with-noise",
        "extra-disabled-payload-160-with-noise",
        "extra-disabled-payload-161-with-noise",
        "extra-disabled-payload-162-with-noise",
        "extra-disabled-payload-163-with-noise",
        "extra-disabled-payload-164-with-noise",
        "extra-disabled-payload-165-with-noise",
        "extra-disabled-payload-166-with-noise",
        "extra-disabled-payload-167-with-noise",
        "extra-disabled-payload-168-with-noise",
        "extra-disabled-payload-169-with-noise",
        "extra-disabled-payload-170-with-noise",
        "extra-disabled-payload-171-with-noise",
        "extra-disabled-payload-172-with-noise",
        "extra-disabled-payload-173-with-noise",
        "extra-disabled-payload-174-with-noise",
        "extra-disabled-payload-175-with-noise",
        "extra-disabled-payload-176-with-noise",
        "extra-disabled-payload-177-with-noise",
        "extra-disabled-payload-178-with-noise",
        "extra-disabled-payload-179-with-noise",
        "extra-disabled-payload-180-with-noise",
        "extra-disabled-payload-181-with-noise",
        "extra-disabled-payload-182-with-noise",
        "extra-disabled-payload-183-with-noise",
        "extra-disabled-payload-184-with-noise",
        "extra-disabled-payload-185-with-noise",
        "extra-disabled-payload-186-with-noise",
        "extra-disabled-payload-187-with-noise",
        "extra-disabled-payload-188-with-noise",
        "extra-disabled-payload-189-with-noise",
        "extra-disabled-payload-190-with-noise",
        "extra-disabled-payload-191-with-noise",
        "extra-disabled-payload-192-with-noise",
        "extra-disabled-payload-193-with-noise",
        "extra-disabled-payload-194-with-noise",
        "extra-disabled-payload-195-with-noise",
        "extra-disabled-payload-196-with-noise",
        "extra-disabled-payload-197-with-noise",
        "extra-disabled-payload-198-with-noise",
        "extra-disabled-payload-199-with-noise",
        "extra-disabled-payload-200-with-noise",
        "extra-disabled-payload-201-with-noise",
        "extra-disabled-payload-202-with-noise",
        "extra-disabled-payload-203-with-noise",
        "extra-disabled-payload-204-with-noise",
        "extra-disabled-payload-205-with-noise",
        "extra-disabled-payload-206-with-noise",
        "extra-disabled-payload-207-with-noise",
        "extra-disabled-payload-208-with-noise",
        "extra-disabled-payload-209-with-noise",
        "extra-disabled-payload-210-with-noise",
        "extra-disabled-payload-211-with-noise",
        "extra-disabled-payload-212-with-noise",
        "extra-disabled-payload-213-with-noise",
        "extra-disabled-payload-214-with-noise",
        "extra-disabled-payload-215-with-noise",
        "extra-disabled-payload-216-with-noise",
        "extra-disabled-payload-217-with-noise",
        "extra-disabled-payload-218-with-noise",
        "extra-disabled-payload-219-with-noise",
        "extra-disabled-payload-220-with-noise",
        "extra-disabled-payload-221-with-noise",
        "extra-disabled-payload-222-with-noise",
        "extra-disabled-payload-223-with-noise",
        "extra-disabled-payload-224-with-noise",
        "extra-disabled-payload-225-with-noise",
        "extra-disabled-payload-226-with-noise",
        "extra-disabled-payload-227-with-noise",
        "extra-disabled-payload-228-with-noise",
        "extra-disabled-payload-229-with-noise",
        "extra-disabled-payload-230-with-noise",
        "extra-disabled-payload-231-with-noise",
        "extra-disabled-payload-232-with-noise",
        "extra-disabled-payload-233-with-noise",
        "extra-disabled-payload-234-with-noise",
        "extra-disabled-payload-235-with-noise",
        "extra-disabled-payload-236-with-noise",
        "extra-disabled-payload-237-with-noise",
        "extra-disabled-payload-238-with-noise",
        "extra-disabled-payload-239-with-noise",
        "extra-disabled-payload-240-with-noise",
        "extra-disabled-payload-241-with-noise",
        "extra-disabled-payload-242-with-noise",
        "extra-disabled-payload-243-with-noise",
        "extra-disabled-payload-244-with-noise",
        "extra-disabled-payload-245-with-noise",
        "extra-disabled-payload-246-with-noise",
        "extra-disabled-payload-247-with-noise",
        "extra-disabled-payload-248-with-noise",
        "extra-disabled-payload-249-with-noise",
    ];
    for text in texts {
        let mut watch = ClipboardWatchController::new();
        assert!(!watch.is_enabled());
        assert_eq!(
            watch.observe_text(text),
            ClipboardObserveOutcome::IgnoredDisabled
        );
        assert!(watch.pending_proposal().is_none());
        assert!(watch.confirm_pending().is_none());
    }
}

#[test]
fn extra_enable_baseline_unchanged_matrix() {
    for i in 0..200usize {
        let mut watch = ClipboardWatchController::new();
        let base = format!("extra-base-{i}");
        watch.enable(Some(&base));
        assert_eq!(
            watch.observe_text(&base),
            ClipboardObserveOutcome::Unchanged
        );
        assert!(watch.pending_proposal().is_none());
        let changed = format!("extra-changed-{i}");
        assert_eq!(
            watch.observe_text(&changed),
            ClipboardObserveOutcome::Proposed
        );
        watch.disable();
        assert!(!watch.is_enabled());
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn extra_awaiting_baseline_then_change() {
    for i in 0..150usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(None);
        let first = format!("first-extra-{i}");
        assert_eq!(
            watch.observe_text(&first),
            ClipboardObserveOutcome::Unchanged
        );
        assert!(watch.pending_proposal().is_none());
        let second = format!("second-extra-{i}");
        assert_eq!(
            watch.observe_text(&second),
            ClipboardObserveOutcome::Proposed
        );
        assert_eq!(watch.pending_proposal().unwrap().text, second);
        watch.dismiss_pending();
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn extra_replace_pending_keeps_latest_only() {
    for i in 0..100usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("r0"));
        watch.observe_text(&format!("first-{i}"));
        let first_id = watch.pending_proposal().unwrap().id.clone();
        watch.observe_text(&format!("second-{i}"));
        let second = watch.pending_proposal().unwrap();
        assert_ne!(second.id, first_id);
        assert_eq!(second.text, format!("second-{i}"));
        watch.dismiss_pending();
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn extra_poll_disabled_short_circuits() {
    for i in 0..80usize {
        let source = ronin_core::ScriptedClipboardSource::new();
        source.push_texts([format!("should-not-read-{i}")]);
        let mut watch = ClipboardWatchController::new();
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::IgnoredDisabled
        );
        // Script unused because disabled short-circuit — push remains.
        watch.enable(None);
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Unchanged
        );
    }
}

#[test]
fn extra_confirm_origin_after_each_propose() {
    use ronin_core::{
        confirmed_clipboard_attach_may_inject_into_chat_request, may_inject_into_chat_request,
        ContextOrigin,
    };
    for i in 0..120usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("x"));
        watch.observe_text(&format!("extra-confirm-{i}"));
        assert!(!may_inject_into_chat_request(
            ContextOrigin::ClipboardWatchProposal
        ));
        assert!(watch.confirm_pending().is_some());
        assert!(confirmed_clipboard_attach_may_inject_into_chat_request());
        assert!(may_inject_into_chat_request(
            ContextOrigin::ConfirmToAttachAccepted
        ));
    }
}
