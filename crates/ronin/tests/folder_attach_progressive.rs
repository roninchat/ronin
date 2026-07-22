//! Dense FolderAttachState browse-filter / reveal seams (#72).

use ronin::folder_attach::FolderAttachState;
use ronin_core::{
    list_folder_entries, list_folder_entries_with_options, FolderListOptions, FolderListPolicy,
};
use tempfile::TempDir;

fn tree(files: &[&str]) -> (tempfile::TempDir, std::path::PathBuf) {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("root");
    for rel in files {
        let path = root.join(rel);
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(&path, "x").unwrap();
    }
    (temp, root)
}

#[test]
fn attach_browse_filter_unique_needles() {
    let files: &[&str] = &[
        "p/file_000.rs",
        "p/file_001.rs",
        "p/file_002.rs",
        "p/file_003.rs",
        "p/file_004.rs",
        "p/file_005.rs",
        "p/file_006.rs",
        "p/file_007.rs",
        "p/file_008.rs",
        "p/file_009.rs",
        "p/file_010.rs",
        "p/file_011.rs",
        "p/file_012.rs",
        "p/file_013.rs",
        "p/file_014.rs",
        "p/file_015.rs",
        "p/file_016.rs",
        "p/file_017.rs",
        "p/file_018.rs",
        "p/file_019.rs",
        "p/file_020.rs",
        "p/file_021.rs",
        "p/file_022.rs",
        "p/file_023.rs",
        "p/file_024.rs",
        "p/file_025.rs",
        "p/file_026.rs",
        "p/file_027.rs",
        "p/file_028.rs",
        "p/file_029.rs",
        "p/file_030.rs",
        "p/file_031.rs",
        "p/file_032.rs",
        "p/file_033.rs",
        "p/file_034.rs",
        "p/file_035.rs",
        "p/file_036.rs",
        "p/file_037.rs",
        "p/file_038.rs",
        "p/file_039.rs",
        "p/file_040.rs",
        "p/file_041.rs",
        "p/file_042.rs",
        "p/file_043.rs",
        "p/file_044.rs",
        "p/file_045.rs",
        "p/file_046.rs",
        "p/file_047.rs",
        "p/file_048.rs",
        "p/file_049.rs",
        "p/file_050.rs",
        "p/file_051.rs",
        "p/file_052.rs",
        "p/file_053.rs",
        "p/file_054.rs",
        "p/file_055.rs",
        "p/file_056.rs",
        "p/file_057.rs",
        "p/file_058.rs",
        "p/file_059.rs",
        "p/file_060.rs",
        "p/file_061.rs",
        "p/file_062.rs",
        "p/file_063.rs",
        "p/file_064.rs",
        "p/file_065.rs",
        "p/file_066.rs",
        "p/file_067.rs",
        "p/file_068.rs",
        "p/file_069.rs",
        "p/file_070.rs",
        "p/file_071.rs",
        "p/file_072.rs",
        "p/file_073.rs",
        "p/file_074.rs",
        "p/file_075.rs",
        "p/file_076.rs",
        "p/file_077.rs",
        "p/file_078.rs",
        "p/file_079.rs",
    ];
    let (temp, root) = tree(files);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    assert_eq!(state.visible_entries().len(), 80);
    for i in 0..80 {
        let needle = format!("file_{i:03}");
        state.set_browse_filter(&needle);
        let vis: Vec<_> = state
            .visible_entries()
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        let expect = format!("p/file_{i:03}.rs");
        assert_eq!(vis, vec![expect.as_str()], "{needle}");
    }
    state.clear_browse_filter();
    assert_eq!(state.visible_entries().len(), 80);
}

#[test]
fn attach_select_all_visible_unique_filters() {
    let (temp, root) = tree(&[
        "keep/a.rs",
        "keep/b.rs",
        "drop/c.rs",
        "drop/d.rs",
        "keep/nested/e.rs",
    ]);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    state.clear_selection();
    state.set_browse_filter("keep");
    state.select_all_visible();
    assert_eq!(state.selected_count(), 3);
    assert!(state.is_selected("keep/a.rs"));
    assert!(state.is_selected("keep/b.rs"));
    assert!(state.is_selected("keep/nested/e.rs"));
    assert!(!state.is_selected("drop/c.rs"));
    assert!(state.to_context_draft().is_ok());
}

#[test]
fn attach_replace_listing_preserves_selection_matrix() {
    let keepers: &[&str] = &[
        "top_00.txt",
        "top_01.txt",
        "top_02.txt",
        "top_03.txt",
        "top_04.txt",
        "top_05.txt",
        "top_06.txt",
        "top_07.txt",
        "top_08.txt",
        "top_09.txt",
        "top_10.txt",
        "top_11.txt",
        "top_12.txt",
        "top_13.txt",
        "top_14.txt",
        "top_15.txt",
        "top_16.txt",
        "top_17.txt",
        "top_18.txt",
        "top_19.txt",
        "top_20.txt",
        "top_21.txt",
        "top_22.txt",
        "top_23.txt",
        "top_24.txt",
        "top_25.txt",
        "top_26.txt",
        "top_27.txt",
        "top_28.txt",
        "top_29.txt",
        "top_30.txt",
        "top_31.txt",
        "top_32.txt",
        "top_33.txt",
        "top_34.txt",
        "top_35.txt",
        "top_36.txt",
        "top_37.txt",
        "top_38.txt",
        "top_39.txt",
    ];
    let (temp, root) = tree(keepers);
    // add deeper files beyond shallow cap
    let mut cur = root.join("deep");
    for i in 0..6 {
        cur = cur.join(format!("d{i}"));
        std::fs::create_dir_all(&cur).unwrap();
        std::fs::write(cur.join(format!("deep{i}.txt")), "d").unwrap();
    }
    let shallow = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions {
            max_depth: 1,
            max_entries: 100,
            browse_filter: None,
        },
    )
    .unwrap();
    let mut state = FolderAttachState::from_listing(shallow);
    state.clear_selection();
    for k in keepers.iter().take(10) {
        state.toggle_file(k);
        assert!(state.is_selected(k));
    }
    let deeper = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &state.deepen_options(),
    )
    .unwrap();
    let before = state.selected_count();
    state.replace_listing(deeper);
    assert_eq!(state.selected_count(), before);
    for k in keepers.iter().take(10) {
        assert!(state.is_selected(k), "{k}");
    }
}

#[test]
fn attach_can_reveal_more_matrix() {
    // tiny tree: not truncated
    let (temp, root) = tree(&["a.txt"]);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let state = FolderAttachState::from_listing(listing);
    assert!(!state.can_reveal_more());

    // truncated with headroom
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("big");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..80 {
        std::fs::write(root.join(format!("f{i}.txt")), "x").unwrap();
    }
    let listing = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions {
            max_depth: 2,
            max_entries: 10,
            browse_filter: None,
        },
    )
    .unwrap();
    assert!(listing.truncated);
    let state = FolderAttachState::from_listing(listing);
    assert!(state.can_reveal_more());
}

#[test]
fn attach_filter_whitespace_clears_visibility() {
    let (temp, root) = tree(&["a.txt", "b.txt"]);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    let mut state = FolderAttachState::from_listing(listing);
    state.set_browse_filter("a");
    assert_eq!(state.visible_entries().len(), 1);
    for raw in ["", " ", "   ", "\t", "\n"] {
        state.set_browse_filter(raw);
        // empty/whitespace still stored but visible_entries trims
        assert_eq!(state.visible_entries().len(), 2, "raw={raw:?}");
    }
}
