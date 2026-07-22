//! Public seams for memory management: enable/disable, profile group, context indicator.

use ronin::memory_management::{
    active_memories_for_context, format_created_date, group_memory_cards, memory_context_block,
    memory_context_indicator, memory_preview_card, MemoryGroup, MemoryListItem,
    MemoryManagementState, PROFILE_GROUP_LABEL,
};

fn item(
    id: &str,
    title: &str,
    content: &str,
    enabled: bool,
    group: MemoryGroup,
    created_at: i64,
) -> MemoryListItem {
    MemoryListItem {
        id: id.into(),
        title: title.into(),
        content: content.into(),
        enabled,
        group,
        created_at,
    }
}

#[test]
fn preview_card_should_include_snippet_date_and_enabled_status() {
    let mem = item(
        "m1",
        "Prefs",
        "User prefers concise answers and Rust examples that are quite long to truncate",
        true,
        MemoryGroup::Regular,
        1_700_000_000_000,
    );
    let card = memory_preview_card(&mem);
    assert_eq!(card.id, "m1");
    assert_eq!(card.title, "Prefs");
    assert!(card.snippet.len() <= 101);
    assert!(card.enabled);
    assert_eq!(card.group, MemoryGroup::Regular);
    assert!(!card.created_label.is_empty());
    assert_eq!(card.status_label, "Enabled");
}

#[test]
fn disabled_card_should_show_disabled_status() {
    let mem = item("m2", "X", "y", false, MemoryGroup::Regular, 0);
    let card = memory_preview_card(&mem);
    assert!(!card.enabled);
    assert_eq!(card.status_label, "Disabled");
}

#[test]
fn profile_memories_should_be_visually_distinguished() {
    let mem = item(
        "p1",
        "Role",
        "Staff engineer",
        true,
        MemoryGroup::Profile,
        1,
    );
    let card = memory_preview_card(&mem);
    assert_eq!(card.group, MemoryGroup::Profile);
    assert_eq!(card.group_label, PROFILE_GROUP_LABEL);
    assert!(card.is_profile);
}

#[test]
fn group_memory_cards_should_list_profile_before_regular() {
    let items = vec![
        item("r", "Regular", "a", true, MemoryGroup::Regular, 2),
        item("p", "Profile", "b", true, MemoryGroup::Profile, 1),
    ];
    let grouped = group_memory_cards(&items);
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped[0].0, MemoryGroup::Profile);
    assert_eq!(grouped[0].1[0].id, "p");
    assert_eq!(grouped[1].0, MemoryGroup::Regular);
    assert_eq!(grouped[1].1[0].id, "r");
}

#[test]
fn active_memories_for_context_should_include_only_enabled_profile() {
    let items = vec![
        item("p-on", "A", "alpha", true, MemoryGroup::Profile, 1),
        item("p-off", "B", "beta", false, MemoryGroup::Profile, 2),
        item("r-on", "C", "gamma", true, MemoryGroup::Regular, 3),
    ];
    let active = active_memories_for_context(&items);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, "p-on");
}

#[test]
fn memory_context_block_should_format_active_profile_memories() {
    let items = vec![
        item("p1", "Name", "Ada", true, MemoryGroup::Profile, 1),
        item("r1", "Note", "skip me", true, MemoryGroup::Regular, 2),
    ];
    let block = memory_context_block(&items).expect("block");
    assert!(block.contains("[Profile memory: Name]"));
    assert!(block.contains("Ada"));
    assert!(!block.contains("skip me"));
}

#[test]
fn memory_context_block_should_be_none_when_no_active_profile() {
    let items = vec![item(
        "r",
        "Note",
        "only regular",
        true,
        MemoryGroup::Regular,
        1,
    )];
    assert!(memory_context_block(&items).is_none());
}

#[test]
fn indicator_should_report_active_count_and_titles() {
    let items = vec![
        item("p1", "Name", "Ada", true, MemoryGroup::Profile, 1),
        item("p2", "Role", "Eng", true, MemoryGroup::Profile, 2),
        item("p3", "Off", "x", false, MemoryGroup::Profile, 3),
    ];
    let indicator = memory_context_indicator(&items);
    assert!(indicator.is_some());
    let ind = indicator.unwrap();
    assert_eq!(ind.active_count, 2);
    assert!(ind.summary_label.contains("2"));
    assert!(ind.summary_label.to_lowercase().contains("memor"));
    assert!(ind.detail_label.contains("Name"));
    assert!(ind.detail_label.contains("Role"));
    assert!(!ind.detail_label.contains("Off"));
}

#[test]
fn indicator_should_be_none_when_no_active_memories() {
    let items = vec![item("p", "Off", "x", false, MemoryGroup::Profile, 1)];
    assert!(memory_context_indicator(&items).is_none());
}

#[test]
fn management_state_should_toggle_and_track_panel() {
    let mut state = MemoryManagementState::default();
    assert!(!state.is_open());
    state.open();
    assert!(state.is_open());
    state.close();
    assert!(!state.is_open());
    state.toggle();
    assert!(state.is_open());
}

#[test]
fn format_created_date_should_be_stable_for_epoch() {
    let label = format_created_date(0);
    assert!(!label.is_empty());
}
