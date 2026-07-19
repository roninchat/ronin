//! Public seams for inline thread rename presentation.

use ronin::thread_titles::{
    format_sidebar_thread_title, title_generation_status_label, ThreadRenameDraft,
    ThreadRenameState, TITLE_GENERATING_HINT,
};

#[test]
fn rename_flow_should_begin_update_commit_and_cancel() {
    let mut state = ThreadRenameState::default();
    assert!(state.editing().is_none());

    state.begin_rename("thread-1", "Old Title");
    let draft = state.editing().expect("editing");
    assert_eq!(draft.thread_id, "thread-1");
    assert_eq!(draft.draft, "Old Title");

    state.update_draft("New Title");
    assert_eq!(state.editing().unwrap().draft, "New Title");

    let committed = state.commit().expect("commit");
    assert_eq!(
        committed,
        ThreadRenameDraft {
            thread_id: "thread-1".into(),
            draft: "New Title".into(),
        }
    );
    assert!(state.editing().is_none());

    state.begin_rename("thread-2", "Again");
    state.update_draft("Nope");
    state.cancel();
    assert!(state.editing().is_none());
}

#[test]
fn commit_should_trim_and_reject_empty_titles() {
    let mut state = ThreadRenameState::default();
    state.begin_rename("t1", "Keep");
    state.update_draft("   ");
    assert!(state.commit().is_none(), "whitespace-only rejected");
    assert!(
        state.editing().is_some(),
        "stay in edit mode on empty commit"
    );

    state.update_draft("  Trimmed Name  ");
    let committed = state.commit().expect("ok");
    assert_eq!(committed.draft, "Trimmed Name");
}

#[test]
fn title_generation_status_should_disclose_extra_model_request() {
    assert!(title_generation_status_label(false).is_none());
    let label = title_generation_status_label(true).expect("hint");
    assert!(
        label.to_lowercase().contains("title")
            && (label.to_lowercase().contains("model")
                || label.to_lowercase().contains("request")
                || label.to_lowercase().contains("generat")),
        "unclear: {label}"
    );
    assert_eq!(label, TITLE_GENERATING_HINT);
}

#[test]
fn sidebar_thread_title_should_mark_active_generations() {
    assert_eq!(format_sidebar_thread_title("Chat", false), "Chat");
    let marked = format_sidebar_thread_title("Chat", true);
    assert!(marked.contains('●'));
    assert!(marked.contains("Chat"));
}
