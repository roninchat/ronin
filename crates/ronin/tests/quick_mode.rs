//! Quick mode overlay: compact one-shot Q&A surface.

use ronin::quick_mode::{
    copy_answer_label, dismiss_hint, open_in_main_label, quick_window_size, save_to_thread_label,
    QuickModeState, QuickPhase, QuickStreamEvent, QUICK_WINDOW_HEIGHT, QUICK_WINDOW_WIDTH,
};

#[test]
fn quick_window_should_be_compact_not_full_shell_sized() {
    let (w, h) = quick_window_size();
    assert_eq!(w, QUICK_WINDOW_WIDTH);
    assert_eq!(h, QUICK_WINDOW_HEIGHT);
    assert!(w < 800.0, "overlay should be narrower than main shell");
    assert!(h < 600.0, "overlay should be shorter than main shell");
}

#[test]
fn quick_mode_should_stream_answer_then_allow_copy() {
    let mut quick = QuickModeState::new();
    quick.set_question("What is Ronin?");
    assert_eq!(quick.phase(), QuickPhase::Composing);
    assert_eq!(quick.question(), "What is Ronin?");

    quick.begin_streaming();
    assert_eq!(quick.phase(), QuickPhase::Streaming);
    quick.append_chunk("A local ");
    quick.append_chunk("AI assistant.");
    quick.finish_streaming();
    assert_eq!(quick.phase(), QuickPhase::Complete);
    assert_eq!(quick.answer(), "A local AI assistant.");

    let copied = quick.copy_answer().expect("copy");
    assert_eq!(copied, "A local AI assistant.");
    assert_eq!(copy_answer_label(), "Copy");
}

#[test]
fn esc_should_dismiss_quick_mode() {
    let mut quick = QuickModeState::new();
    assert!(!quick.is_dismissed());
    quick.dismiss();
    assert!(quick.is_dismissed());
    assert_eq!(dismiss_hint(), "Esc to dismiss");
}

#[test]
fn save_and_open_in_main_should_require_completed_answer() {
    let mut quick = QuickModeState::new();
    quick.set_question("Hi");
    assert!(!quick.can_save());
    assert!(!quick.can_open_in_main());

    quick.begin_streaming();
    quick.append_chunk("Hello");
    quick.finish_streaming();
    assert!(quick.can_save());
    assert!(!quick.can_open_in_main());

    quick.mark_saved("thread-123");
    assert_eq!(quick.saved_thread_id(), Some("thread-123"));
    assert!(quick.can_open_in_main());
    assert_eq!(save_to_thread_label(), "Save to thread");
    assert_eq!(open_in_main_label(), "Open in Ronin");
}

#[test]
fn quick_mode_should_track_failed_generation() {
    let mut quick = QuickModeState::new();
    quick.set_question("Hi");
    quick.begin_streaming();
    quick.fail("provider offline");
    assert_eq!(
        quick.phase(),
        QuickPhase::Failed {
            message: "provider offline".into()
        }
    );
    assert!(!quick.can_save());
}

#[test]
fn apply_stream_event_should_drive_quick_phase() {
    let mut quick = QuickModeState::new();
    quick.set_question("Hi");
    quick.begin_streaming();
    quick.apply_stream_event(QuickStreamEvent::Chunk("Hello".into()));
    quick.apply_stream_event(QuickStreamEvent::Done);
    assert_eq!(quick.phase(), QuickPhase::Complete);
    assert_eq!(quick.answer(), "Hello");
}

#[test]
fn build_quick_chat_request_should_be_one_shot_user_message() {
    use ronin::quick_mode::build_quick_chat_request;

    let req = build_quick_chat_request("Ping", "llama3", "You are Ronin.");
    assert_eq!(req.model, "llama3");
    assert_eq!(req.system_prompt.as_deref(), Some("You are Ronin."));
    assert_eq!(req.messages.len(), 1);
    assert_eq!(req.messages[0].role, "user");
    assert_eq!(req.messages[0].content, "Ping");
}

#[test]
fn quick_launch_plan_should_open_overlay_not_full_shell_thread() {
    use ronin::{plan_incoming_launch, IncomingLaunch, LaunchIntent};

    let plan = plan_incoming_launch(&IncomingLaunch {
        intent: LaunchIntent::Quick {
            attach_paths: vec!["/tmp/note.md".into()],
        },
        focus: true,
    });
    assert!(
        plan.open_quick_overlay,
        "quick intent must open the compact overlay"
    );
    assert!(
        !plan.create_new_thread,
        "quick should not create a full-shell thread until the user saves"
    );
    assert!(plan.focus_window);
    assert_eq!(
        plan.attach_paths,
        vec![std::path::PathBuf::from("/tmp/note.md")]
    );
}
