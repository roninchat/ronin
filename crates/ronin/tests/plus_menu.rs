//! Composer + menu catalog and memory/artifact flags.

use ronin::plus_menu::plus_menu_items;

fn inserts(memories: bool, artifacts: bool) -> Vec<&'static str> {
    plus_menu_items(memories, artifacts)
        .into_iter()
        .map(|item| item.insert)
        .collect()
}

#[test]
fn plus_menu_should_always_include_file_folder_clipboard_and_screenshots() {
    let present = inserts(false, false);
    assert!(present.contains(&"@file:"));
    assert!(present.contains(&"@folder:"));
    assert!(present.contains(&"@clipboard"));
    assert!(present.contains(&"@screenshot"));
    assert!(present.contains(&"@screenshot:window"));
    assert!(!present.contains(&"@memory:"));
    assert!(!present.contains(&"@artifact:"));
}

#[test]
fn plus_menu_should_include_memory_and_artifact_only_when_enabled() {
    assert!(inserts(true, false).contains(&"@memory:"));
    assert!(!inserts(true, false).contains(&"@artifact:"));
    assert!(inserts(false, true).contains(&"@artifact:"));
    assert!(!inserts(false, true).contains(&"@memory:"));
    let both = inserts(true, true);
    assert!(both.contains(&"@memory:"));
    assert!(both.contains(&"@artifact:"));
}
