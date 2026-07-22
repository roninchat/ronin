//! Folder attach: browse filter + progressive deepen seams (#72).

use ronin::folder_attach::{
    folder_attach_from_listing, folder_browse_filter_placeholder, folder_reveal_more_label,
    folder_truncated_hint, FolderAttachState,
};
use ronin_core::{
    list_folder_entries, list_folder_entries_with_options, AttachmentKind, FolderListOptions,
    FolderListPolicy, FOLDER_LIST_MAX_DEPTH,
};
use tempfile::TempDir;

#[test]
fn folder_attach_should_default_select_all_and_toggle_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("pkg");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("a.txt"), "aaa").unwrap();
    std::fs::write(root.join("b.txt"), "bbb").unwrap();
    std::fs::write(root.join("c.txt"), "ccc").unwrap();

    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
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
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    state.clear_selection();
    assert!(state.to_context_draft().is_err());
}

#[test]
fn browse_filter_narrows_visible_entries_before_selection() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("filter-ui");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src/main.rs"), "fn main(){}").unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn x(){}").unwrap();
    std::fs::write(root.join("README.md"), "# hi").unwrap();

    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    assert_eq!(state.visible_entries().len(), 3);

    state.set_browse_filter("main");
    let visible: Vec<_> = state
        .visible_entries()
        .iter()
        .map(|e| e.relative_path.as_str())
        .collect();
    assert_eq!(visible, vec!["src/main.rs"]);
    // Filter does not clear prior selection of hidden files.
    assert!(state.is_selected("README.md"));
    assert_eq!(state.browse_filter(), "main");

    state.clear_selection();
    state.select_all_visible();
    assert_eq!(state.selected_count(), 1);
    assert!(state.is_selected("src/main.rs"));
    assert!(!state.is_selected("README.md"));

    state.clear_browse_filter();
    assert_eq!(state.visible_entries().len(), 3);
}

#[test]
fn replace_listing_keeps_existing_selection_and_does_not_auto_select_new() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("prog");
    let mut cur = root.clone();
    for i in 0..=FOLDER_LIST_MAX_DEPTH + 2 {
        cur = cur.join(format!("d{i}"));
        std::fs::create_dir_all(&cur).unwrap();
        std::fs::write(cur.join(format!("f{i}.txt")), "x").unwrap();
    }

    let shallow = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions {
            max_depth: 2,
            max_entries: 50,
            browse_filter: None,
        },
    )
    .unwrap();
    let mut state = FolderAttachState::from_listing(shallow);
    state.clear_selection();
    let keep = state.listing().entries[0].relative_path.clone();
    state.toggle_file(&keep);
    assert!(state.is_selected(&keep));

    let deeper = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &state.deepen_options(),
    )
    .unwrap();
    assert!(deeper.entries.len() > state.listing().entries.len());
    let new_only: Vec<_> = deeper
        .entries
        .iter()
        .map(|e| e.relative_path.clone())
        .filter(|p| {
            !state
                .listing()
                .entries
                .iter()
                .any(|e| e.relative_path == *p)
        })
        .collect();
    assert!(!new_only.is_empty());

    state.replace_listing(deeper);
    assert!(state.is_selected(&keep));
    for path in &new_only {
        assert!(
            !state.is_selected(path),
            "new paths must not auto-select; {path} was selected"
        );
    }
    assert!(state.to_context_draft().is_ok());
}

#[test]
fn can_reveal_more_requires_truncation_and_headroom() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("small");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("a.txt"), "a").unwrap();
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let state = FolderAttachState::from_listing(listing);
    assert!(!state.listing().truncated);
    assert!(!state.can_reveal_more());
}

#[test]
fn labels_document_browse_and_reveal_controls() {
    assert!(!folder_truncated_hint().is_empty());
    assert!(!folder_reveal_more_label().is_empty());
    assert!(!folder_browse_filter_placeholder().is_empty());
}
