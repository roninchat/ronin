//! Folder attach: listing, file selection, and draft conversion.

use ronin::folder_attach::{folder_attach_from_listing, FolderAttachState};
use ronin_core::{list_folder_entries, AttachmentKind};
use tempfile::TempDir;

#[test]
fn folder_attach_should_default_select_all_and_toggle_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("pkg");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("a.txt"), "aaa").unwrap();
    std::fs::write(root.join("b.txt"), "bbb").unwrap();
    std::fs::write(root.join("c.txt"), "ccc").unwrap();

    let listing = list_folder_entries(&root, temp.path()).unwrap();
    let mut state = folder_attach_from_listing(listing);
    assert_eq!(state.selected_count(), 3);
    assert!(state.is_selected("a.txt"));

    state.toggle_file("b.txt");
    assert!(!state.is_selected("b.txt"));
    assert_eq!(state.selected_count(), 2);

    let draft = state.to_context_draft().expect("draft");
    assert_eq!(draft.kind, AttachmentKind::Folder);
    assert!(draft.context_block.contains("aaa"));
    assert!(!draft.context_block.contains("bbb"));
    assert!(draft.context_block.contains("ccc"));
}

#[test]
fn folder_attach_should_require_at_least_one_selected_file() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("empty-sel");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("only.txt"), "x").unwrap();
    let listing = list_folder_entries(&root, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    state.clear_selection();
    assert!(state.to_context_draft().is_err());
}
