//! Composer `@` attachment picker and `/` action picker presentation.

use ronin::composer_pickers::{
    at_attachment_catalog, detect_active_picker, move_picker_selection, slash_action_catalog,
    ActivePicker, AtAttachmentKind, PickerKind, SlashActionKind,
};

#[test]
fn at_catalog_should_list_required_attachment_sources() {
    let kinds: Vec<_> = at_attachment_catalog()
        .iter()
        .map(|i| i.at_kind().expect("at item"))
        .collect();
    assert!(kinds.contains(&AtAttachmentKind::File));
    assert!(kinds.contains(&AtAttachmentKind::Folder));
    assert!(kinds.contains(&AtAttachmentKind::Clipboard));
    assert!(kinds.contains(&AtAttachmentKind::Screenshot));
    assert!(kinds.contains(&AtAttachmentKind::Artifact));
    assert!(kinds.contains(&AtAttachmentKind::Memory));
}

#[test]
fn slash_catalog_should_list_initial_actions() {
    let kinds: Vec<_> = slash_action_catalog()
        .iter()
        .map(|i| i.slash_kind().expect("slash item"))
        .collect();
    assert!(kinds.contains(&SlashActionKind::NewThread));
    assert!(kinds.contains(&SlashActionKind::ClearComposer));
    assert!(kinds.contains(&SlashActionKind::SwitchModel));
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchToggle));
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchConfirm));
    assert!(kinds.contains(&SlashActionKind::ClipboardWatchDismiss));
}

#[test]
fn typing_at_should_open_attachment_picker_with_all_options() {
    let picker = detect_active_picker("@", 1).expect("picker");
    assert_eq!(picker.kind, PickerKind::AtAttachment);
    assert_eq!(picker.query, "");
    assert_eq!(picker.token_start, 0);
    assert_eq!(picker.items.len(), at_attachment_catalog().len());
}

#[test]
fn at_picker_should_filter_as_user_types() {
    let picker = detect_active_picker("note @fi", 8).expect("picker");
    assert_eq!(picker.kind, PickerKind::AtAttachment);
    assert_eq!(picker.query, "fi");
    assert_eq!(picker.items.len(), 1);
    assert_eq!(picker.items[0].insert, "@file:");
    assert_eq!(picker.items[0].at_kind(), Some(AtAttachmentKind::File));
}

#[test]
fn at_picker_should_not_open_mid_word() {
    assert_eq!(detect_active_picker("user@host", 9), None);
    assert_eq!(detect_active_picker("email@x", 7), None);
}

#[test]
fn at_picker_should_yield_to_file_memory_artifact_subflows() {
    assert_eq!(detect_active_picker("@file:/tmp", 10), None);
    assert_eq!(detect_active_picker("@folder:/tmp", 12), None);
    assert_eq!(detect_active_picker("@memory:abc", 11), None);
    assert_eq!(detect_active_picker("@artifact:x", 11), None);
}

#[test]
fn typing_slash_should_open_action_picker() {
    let picker = detect_active_picker("/", 1).expect("picker");
    assert_eq!(picker.kind, PickerKind::SlashAction);
    assert_eq!(picker.query, "");
    assert_eq!(picker.items.len(), slash_action_catalog().len());
}

#[test]
fn slash_picker_should_filter_as_user_types() {
    let picker = detect_active_picker("ready /ne", 9).expect("picker");
    assert_eq!(picker.kind, PickerKind::SlashAction);
    assert_eq!(picker.query, "ne");
    assert_eq!(picker.items.len(), 1);
    assert_eq!(picker.items[0].insert, "/new");
    assert_eq!(
        picker.items[0].slash_kind(),
        Some(SlashActionKind::NewThread)
    );
}

#[test]
fn slash_picker_should_not_open_mid_word() {
    assert_eq!(detect_active_picker("http://example", 14), None);
    assert_eq!(detect_active_picker("a/b", 3), None);
}

#[test]
fn backspacing_past_trigger_should_dismiss_picker() {
    assert!(detect_active_picker("@fi", 3).is_some());
    assert_eq!(
        detect_active_picker(" ", 1),
        None,
        "space-only token has no picker"
    );
    // After deleting `@`, cursor at end of prior word — no active picker
    assert_eq!(detect_active_picker("hello", 5), None);
}

#[test]
fn move_picker_selection_should_wrap_with_arrows() {
    assert_eq!(move_picker_selection(0, 3, -1), 2);
    assert_eq!(move_picker_selection(2, 3, 1), 0);
    assert_eq!(move_picker_selection(1, 3, 1), 2);
    assert_eq!(move_picker_selection(0, 0, 1), 0);
}

#[test]
fn active_picker_equality_should_support_ui_diffing() {
    let a = detect_active_picker("@", 1).unwrap();
    let b = detect_active_picker("@", 1).unwrap();
    assert_eq!(a, b);
    let ActivePicker { kind, .. } = a;
    assert_eq!(kind, PickerKind::AtAttachment);
}
