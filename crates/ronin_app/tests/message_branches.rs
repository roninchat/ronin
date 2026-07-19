//! Public seams for conversation branch path resolution and navigation.

use ronin_app::{
    leaf_under_root, resolve_active_path, sibling_branch_nav, BranchNav, MessageNode,
};

fn nodes(pairs: &[(&str, Option<&str>)]) -> Vec<MessageNode> {
    pairs
        .iter()
        .map(|(id, parent)| MessageNode {
            id: (*id).to_string(),
            parent_id: parent.map(str::to_string),
        })
        .collect()
}

#[test]
fn resolve_active_path_should_walk_parent_chain_to_root() {
    let all = nodes(&[
        ("u1", None),
        ("a1", Some("u1")),
        ("u2", Some("a1")),
        ("a2", Some("u2")),
        ("a1b", Some("u1")), // sibling branch under u1
    ]);
    let path = resolve_active_path(&all, Some("a2"));
    assert_eq!(
        path.iter().map(|n| n.id.as_str()).collect::<Vec<_>>(),
        vec!["u1", "a1", "u2", "a2"]
    );
}

#[test]
fn resolve_active_path_should_fall_back_to_linear_when_no_leaf() {
    let all = nodes(&[("u1", None), ("a1", Some("u1")), ("orphan", None)]);
    let path = resolve_active_path(&all, None);
    assert_eq!(path.len(), 3, "legacy: all messages in input order");
}

#[test]
fn sibling_branch_nav_should_expose_index_among_siblings() {
    let all = nodes(&[
        ("u1", None),
        ("a1", Some("u1")),
        ("a2", Some("u1")),
        ("a3", Some("u1")),
    ]);
    let nav = sibling_branch_nav(&all, "a2").expect("nav");
    assert_eq!(
        nav,
        BranchNav {
            message_id: "a2".into(),
            sibling_ids: vec!["a1".into(), "a2".into(), "a3".into()],
            selected_index: 1, // 0-based
            total: 3,
        }
    );
    assert!(sibling_branch_nav(&all, "u1").is_none());
}

#[test]
fn leaf_under_root_should_follow_latest_child_chain() {
    let all = nodes(&[
        ("u1", None),
        ("a1", Some("u1")),
        ("u2", Some("a1")),
        ("a2", Some("u2")),
        ("a1b", Some("u1")),
        ("u2b", Some("a1b")),
    ]);
    assert_eq!(leaf_under_root(&all, "a1"), "a2");
    assert_eq!(leaf_under_root(&all, "a1b"), "u2b");
    assert_eq!(leaf_under_root(&all, "u2b"), "u2b");
}
