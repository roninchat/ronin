//! Conversation branch path resolution and sibling navigation.
//!
//! Pure seams over message parent links — independent of SQLite/GPUI.

/// Minimal message node used for path / sibling algorithms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageNode {
    /// Message id.
    pub id: String,
    /// Parent message id (`None` for roots).
    pub parent_id: Option<String>,
}

/// Navigation snapshot for a fork with multiple siblings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchNav {
    /// Currently selected sibling message id.
    pub message_id: String,
    /// All sibling ids sharing the same parent, oldest first.
    pub sibling_ids: Vec<String>,
    /// Zero-based index of [`Self::message_id`] within [`Self::sibling_ids`].
    pub selected_index: usize,
    /// Number of siblings.
    pub total: usize,
}

/// Resolves the active conversation path ending at `active_leaf_id`.
///
/// When `active_leaf_id` is missing or unknown, returns `all` unchanged
/// (legacy linear threads).
pub fn resolve_active_path(all: &[MessageNode], active_leaf_id: Option<&str>) -> Vec<MessageNode> {
    let Some(leaf) = active_leaf_id else {
        return all.to_vec();
    };
    if !all.iter().any(|m| m.id == leaf) {
        return all.to_vec();
    }

    let by_id: std::collections::HashMap<&str, &MessageNode> =
        all.iter().map(|m| (m.id.as_str(), m)).collect();

    let mut path = Vec::new();
    let mut current = Some(leaf);
    let mut guard = 0usize;
    while let Some(id) = current {
        guard += 1;
        if guard > all.len() + 2 {
            break; // cycle protection
        }
        let Some(node) = by_id.get(id).copied() else {
            break;
        };
        path.push(node.clone());
        current = node.parent_id.as_deref();
    }
    path.reverse();
    path
}

/// Returns branch navigation when `message_id` has one or more siblings.
pub fn sibling_branch_nav(all: &[MessageNode], message_id: &str) -> Option<BranchNav> {
    let target = all.iter().find(|m| m.id == message_id)?;
    let parent = target.parent_id.as_deref();
    let siblings: Vec<&MessageNode> = all
        .iter()
        .filter(|m| m.parent_id.as_deref() == parent)
        .collect();
    if siblings.len() < 2 {
        return None;
    }
    let selected_index = siblings.iter().position(|m| m.id == message_id)?;
    Some(BranchNav {
        message_id: message_id.to_string(),
        sibling_ids: siblings.iter().map(|m| m.id.clone()).collect(),
        selected_index,
        total: siblings.len(),
    })
}

/// Walks to the deepest leaf under `root_id`, preferring the latest child at each step.
pub fn leaf_under_root(all: &[MessageNode], root_id: &str) -> String {
    let mut current = root_id.to_string();
    loop {
        let mut children: Vec<&MessageNode> = all
            .iter()
            .filter(|m| m.parent_id.as_deref() == Some(current.as_str()))
            .collect();
        if children.is_empty() {
            return current;
        }
        // Latest child wins (nodes are expected oldest→newest in `all`).
        children.sort_by_key(|m| all.iter().position(|x| x.id == m.id).unwrap_or(usize::MAX));
        current = children.last().unwrap().id.clone();
    }
}
