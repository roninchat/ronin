//! Deeper progressive folder listing + browse filter (#72).
//!
//! Public seams: `FolderListOptions`, `list_folder_entries_with_options`,
//! raised default caps, progressive `deepen`, and browse-filter narrowing.

use std::path::Path;

use ronin_core::{
    folder_attachment_from_selection, list_folder_entries, list_folder_entries_with_options,
    list_folder_entries_with_policy, FolderListOptions, FolderListPolicy,
    FOLDER_LIST_DEPTH_CEILING, FOLDER_LIST_ENTRIES_CEILING, FOLDER_LIST_MAX_DEPTH,
    FOLDER_LIST_MAX_ENTRIES,
};
use tempfile::TempDir;

fn write_tree(root: &Path, rels: &[&str]) {
    for rel in rels {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, format!("body:{rel}")).unwrap();
    }
}

fn rels(listing: &ronin_core::FolderListing) -> Vec<&str> {
    listing
        .entries
        .iter()
        .map(|e| e.relative_path.as_str())
        .collect()
}

#[test]
fn default_caps_are_deeper_than_m2_shallow_walk() {
    // M2 shipped depth 2 / 200 entries; M3.0 deepen raises the documented defaults.
    const {
        assert!(FOLDER_LIST_MAX_DEPTH > 2);
        assert!(FOLDER_LIST_MAX_ENTRIES > 200);
        assert!(FOLDER_LIST_DEPTH_CEILING >= FOLDER_LIST_MAX_DEPTH);
        assert!(FOLDER_LIST_ENTRIES_CEILING >= FOLDER_LIST_MAX_ENTRIES);
    };
}

#[test]
fn default_listing_reaches_deeper_than_two_directory_levels() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("proj");
    write_tree(
        &root,
        &[
            "README.md",
            "a/one.txt",
            "a/b/two.txt",
            "a/b/c/three.txt",
            "a/b/c/d/four.txt",
        ],
    );

    let listing = list_folder_entries(&root, None, temp.path()).expect("list");
    let paths = rels(&listing);
    assert!(paths.contains(&"README.md"));
    assert!(paths.contains(&"a/one.txt"));
    assert!(paths.contains(&"a/b/two.txt"));
    assert!(
        paths.contains(&"a/b/c/three.txt"),
        "default deepen should reach depth>2; got {paths:?}"
    );
    assert_eq!(listing.list_options.max_depth, FOLDER_LIST_MAX_DEPTH);
    assert_eq!(listing.list_options.max_entries, FOLDER_LIST_MAX_ENTRIES);
}

#[test]
fn progressive_deepen_reveals_more_under_documented_ceilings() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("deep");
    // Build a chain deeper than the initial default but within the ceiling.
    let mut cur = root.clone();
    let mut expected_deep = String::new();
    for i in 0..FOLDER_LIST_DEPTH_CEILING {
        cur = cur.join(format!("d{i}"));
        std::fs::create_dir_all(&cur).unwrap();
        let name = format!("f{i}.txt");
        std::fs::write(cur.join(&name), "x").unwrap();
        if i + 1 == FOLDER_LIST_MAX_DEPTH + 2 {
            expected_deep = cur
                .strip_prefix(&root)
                .unwrap()
                .join(&name)
                .to_string_lossy()
                .replace('\\', "/");
        }
    }

    let shallow = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions::default(),
    )
    .unwrap();
    assert!(
        !rels(&shallow).contains(&expected_deep.as_str()),
        "initial options should miss deep file {expected_deep}"
    );
    assert!(shallow.truncated || shallow.list_options.can_deepen());

    let deeper_opts = shallow.list_options.deepen();
    assert!(
        deeper_opts.max_depth > shallow.list_options.max_depth
            || deeper_opts.max_entries > shallow.list_options.max_entries
    );
    let deeper = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &deeper_opts,
    )
    .unwrap();
    assert!(
        rels(&deeper).contains(&expected_deep.as_str()),
        "progressive deepen should reveal {expected_deep}; got {:?}",
        rels(&deeper)
    );
}

#[test]
fn deepen_clamps_to_documented_ceilings() {
    let mut opts = FolderListOptions {
        max_depth: FOLDER_LIST_DEPTH_CEILING,
        max_entries: FOLDER_LIST_ENTRIES_CEILING,
        browse_filter: None,
    };
    assert!(!opts.can_deepen());
    opts = opts.deepen();
    assert_eq!(opts.max_depth, FOLDER_LIST_DEPTH_CEILING);
    assert_eq!(opts.max_entries, FOLDER_LIST_ENTRIES_CEILING);

    let over = FolderListOptions {
        max_depth: FOLDER_LIST_DEPTH_CEILING + 50,
        max_entries: FOLDER_LIST_ENTRIES_CEILING + 50,
        browse_filter: None,
    }
    .clamp_to_ceilings();
    assert_eq!(over.max_depth, FOLDER_LIST_DEPTH_CEILING);
    assert_eq!(over.max_entries, FOLDER_LIST_ENTRIES_CEILING);
}

#[test]
fn browse_filter_narrows_listing_before_selection() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("pkg");
    write_tree(
        &root,
        &[
            "src/main.rs",
            "src/lib.rs",
            "src/util/mod.rs",
            "README.md",
            "docs/guide.md",
        ],
    );

    let opts = FolderListOptions::default().with_browse_filter("main");
    let listing = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &opts,
    )
    .unwrap();
    let paths = rels(&listing);
    assert_eq!(paths, vec!["src/main.rs"]);
    assert_eq!(listing.list_options.browse_filter.as_deref(), Some("main"));

    // Empty selection still cannot attach — listing ≠ attach.
    assert!(folder_attachment_from_selection(&listing, &[]).is_err());
    let draft = folder_attachment_from_selection(&listing, &["src/main.rs".into()]).unwrap();
    assert!(draft.context_block.contains("src/main.rs"));
}

#[test]
fn browse_filter_is_case_insensitive_substring_on_relative_path() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("case");
    write_tree(&root, &["Foo/Bar.RS", "other.txt", "foo/baz.md"]);

    let listing = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions::default().with_browse_filter("FOO"),
    )
    .unwrap();
    let paths = rels(&listing);
    assert!(paths.contains(&"Foo/Bar.RS"));
    assert!(paths.contains(&"foo/baz.md"));
    assert!(!paths.contains(&"other.txt"));
}

#[test]
fn ignore_deny_allow_still_apply_with_options_and_filter() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("priv");
    write_tree(
        &root,
        &[
            "keep.rs",
            "secret.env",
            "nested/keep.rs",
            "nested/secret.env",
        ],
    );
    std::fs::write(root.join(".gitignore"), "*.env\n").unwrap();

    let listing = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions::default().with_browse_filter("secret"),
    )
    .unwrap();
    assert!(
        rels(&listing).is_empty(),
        "gitignore must still omit *.env even when filter matches; got {:?}",
        rels(&listing)
    );

    let never = temp.path().join("never-me");
    std::fs::create_dir_all(&never).unwrap();
    std::fs::write(never.join("x.txt"), "x").unwrap();
    let policy = FolderListPolicy {
        never_list: vec![never.clone()],
        ..FolderListPolicy::default()
    };
    let blocked = list_folder_entries_with_options(
        &never,
        None,
        temp.path(),
        &policy,
        &FolderListOptions::default(),
    );
    assert!(blocked.is_err());
}

#[test]
fn truncation_is_honest_when_entry_cap_hit_under_filter() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("many");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..80 {
        std::fs::write(root.join(format!("hit-{i:03}.txt")), "h").unwrap();
        std::fs::write(root.join(format!("miss-{i:03}.txt")), "m").unwrap();
    }

    let opts = FolderListOptions {
        max_depth: FOLDER_LIST_MAX_DEPTH,
        max_entries: 25,
        browse_filter: Some("hit-".into()),
    };
    let listing = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &opts,
    )
    .unwrap();
    assert!(listing.truncated);
    assert_eq!(listing.entries.len(), 25);
    assert!(listing
        .entries
        .iter()
        .all(|e| e.relative_path.contains("hit-")));
}

#[test]
fn list_folder_entries_with_policy_uses_default_options() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("compat");
    write_tree(&root, &["a.txt"]);
    let a = list_folder_entries_with_policy(&root, None, temp.path(), &FolderListPolicy::default())
        .unwrap();
    let b = list_folder_entries_with_options(
        &root,
        None,
        temp.path(),
        &FolderListPolicy::default(),
        &FolderListOptions::default(),
    )
    .unwrap();
    assert_eq!(a.entries, b.entries);
    assert_eq!(a.truncated, b.truncated);
    assert_eq!(a.list_options, b.list_options);
}

#[test]
fn listing_never_auto_attaches_whole_folder() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("no-auto");
    write_tree(&root, &["a.txt", "b.txt"]);
    let listing = list_folder_entries(&root, None, temp.path()).unwrap();
    assert!(folder_attachment_from_selection(&listing, &[]).is_err());
}
