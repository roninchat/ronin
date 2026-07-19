//! Public presentation seams for message edit + branch navigation chrome.

use ronin::message_branches::{
    branch_nav_label, edit_draft_commit, MessageEditDraft, MessageEditState,
};

#[test]
fn branch_nav_label_should_be_clear_for_users() {
    assert_eq!(branch_nav_label(0, 2), "1 / 2");
    assert_eq!(branch_nav_label(1, 3), "2 / 3");
}

#[test]
fn edit_draft_flow_should_commit_trimmed_text_or_cancel() {
    let mut state = MessageEditState::default();
    state.begin_edit("msg-1", "Hello");
    assert_eq!(
        state.editing(),
        Some(&MessageEditDraft {
            message_id: "msg-1".into(),
            draft: "Hello".into(),
        })
    );
    state.update_draft("  Hello edited  ");
    let committed = edit_draft_commit(&mut state).expect("commit");
    assert_eq!(committed.draft, "Hello edited");
    assert!(state.editing().is_none());

    state.begin_edit("msg-2", "x");
    state.cancel();
    assert!(state.editing().is_none());
    assert!(edit_draft_commit(&mut state).is_none());
}
