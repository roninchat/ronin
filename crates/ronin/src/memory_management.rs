//! Memory management presentation: enable/disable, profile group, context indicator.
//!
//! Public seams — testable without GPUI or SQLite.

/// User-visible label for the profile memory group.
pub const PROFILE_GROUP_LABEL: &str = "Profile";

/// User-visible label for regular (non-profile) memories.
pub const REGULAR_GROUP_LABEL: &str = "Memories";

/// Maximum characters shown in a memory preview snippet.
pub const MEMORY_SNIPPET_CHARS: usize = 100;

/// Organizational group for a memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryGroup {
    /// Always-on user profile context (preferences, role, identity).
    Profile,
    /// Regular memory; not auto-injected into provider requests.
    Regular,
}

impl MemoryGroup {
    /// Section header for grouped lists.
    pub fn label(self) -> &'static str {
        match self {
            Self::Profile => PROFILE_GROUP_LABEL,
            Self::Regular => REGULAR_GROUP_LABEL,
        }
    }
}

/// One memory row for management UI and context selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryListItem {
    /// Stable memory id.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Full content body.
    pub content: String,
    /// Whether the memory may be included in provider context.
    pub enabled: bool,
    /// Profile vs regular grouping.
    pub group: MemoryGroup,
    /// Creation time (Unix ms).
    pub created_at: i64,
}

impl MemoryListItem {
    /// Builds a list item from persisted memory fields.
    pub fn from_fields(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
        enabled: bool,
        is_profile: bool,
        created_at: i64,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            enabled,
            group: if is_profile {
                MemoryGroup::Profile
            } else {
                MemoryGroup::Regular
            },
            created_at,
        }
    }
}

/// Preview card for the memory management list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPreviewCard {
    /// Memory id.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Truncated content snippet.
    pub snippet: String,
    /// Whether enabled for context.
    pub enabled: bool,
    /// `"Enabled"` or `"Disabled"`.
    pub status_label: &'static str,
    /// Organizational group.
    pub group: MemoryGroup,
    /// Group badge label.
    pub group_label: &'static str,
    /// Convenience mirror of `group == Profile`.
    pub is_profile: bool,
    /// Human-readable created date.
    pub created_label: String,
}

/// Composer/chat indicator when profile memories will be sent with the next request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryContextIndicator {
    /// Number of enabled profile memories.
    pub active_count: usize,
    /// Compact label (e.g. `2 memories active`).
    pub summary_label: String,
    /// Titles of active memories joined for detail/tooltip.
    pub detail_label: String,
}

/// UI open/closed state for the memory management page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MemoryManagementState {
    open: bool,
}

impl MemoryManagementState {
    /// Whether the management page is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Opens the management page.
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Closes the management page.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Toggles visibility.
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

/// Builds a preview card from a list item.
pub fn memory_preview_card(item: &MemoryListItem) -> MemoryPreviewCard {
    MemoryPreviewCard {
        id: item.id.clone(),
        title: item.title.clone(),
        snippet: content_snippet(&item.content, MEMORY_SNIPPET_CHARS),
        enabled: item.enabled,
        status_label: if item.enabled { "Enabled" } else { "Disabled" },
        group: item.group,
        group_label: item.group.label(),
        is_profile: item.group == MemoryGroup::Profile,
        created_label: format_created_date(item.created_at),
    }
}

/// Truncates content for preview cards.
pub fn content_snippet(content: &str, max_chars: usize) -> String {
    let mut chars = content.chars();
    let snippet: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{snippet}…")
    } else {
        snippet
    }
}

/// Formats a Unix-ms timestamp as a short UTC date (`YYYY-MM-DD`).
pub fn format_created_date(created_at_ms: i64) -> String {
    let secs = created_at_ms.div_euclid(1000);
    // Manual civil date from Unix seconds (UTC) — avoids pulling chrono into the UI crate.
    let (year, month, day) = unix_secs_to_ymd(secs);
    format!("{year:04}-{month:02}-{day:02}")
}

fn unix_secs_to_ymd(secs: i64) -> (i32, u32, u32) {
    // Algorithm from civil_from_days (Howard Hinnant), days since 1970-01-01.
    let z = secs.div_euclid(86_400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

/// Groups cards with Profile section first, then Regular.
pub fn group_memory_cards(
    items: &[MemoryListItem],
) -> Vec<(MemoryGroup, Vec<MemoryPreviewCard>)> {
    let order = [MemoryGroup::Profile, MemoryGroup::Regular];
    let mut out = Vec::new();
    for group in order {
        let cards: Vec<MemoryPreviewCard> = items
            .iter()
            .filter(|i| i.group == group)
            .map(memory_preview_card)
            .collect();
        if !cards.is_empty() {
            out.push((group, cards));
        }
    }
    out
}

/// Enabled profile memories that should be auto-injected into provider context.
pub fn active_memories_for_context(items: &[MemoryListItem]) -> Vec<&MemoryListItem> {
    items
        .iter()
        .filter(|m| m.enabled && m.group == MemoryGroup::Profile)
        .collect()
}

/// Builds the system-context block for active profile memories.
pub fn memory_context_block(items: &[MemoryListItem]) -> Option<String> {
    let active = active_memories_for_context(items);
    if active.is_empty() {
        return None;
    }
    let block = active
        .iter()
        .map(|m| format!("[Profile memory: {}]\n{}", m.title, m.content))
        .collect::<Vec<_>>()
        .join("\n\n");
    Some(block)
}

/// Builds a composer indicator when profile memories will be included.
pub fn memory_context_indicator(items: &[MemoryListItem]) -> Option<MemoryContextIndicator> {
    let active = active_memories_for_context(items);
    if active.is_empty() {
        return None;
    }
    let active_count = active.len();
    let summary_label = if active_count == 1 {
        "1 memory active".to_string()
    } else {
        format!("{active_count} memories active")
    };
    let detail_label = active
        .iter()
        .map(|m| m.title.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    Some(MemoryContextIndicator {
        active_count,
        summary_label,
        detail_label,
    })
}

/// Merges optional attachment and memory context blocks for a provider request.
pub fn merge_context_blocks(
    memory_block: Option<&str>,
    attachment_block: Option<&str>,
) -> Option<String> {
    match (memory_block, attachment_block) {
        (Some(m), Some(a)) if !m.is_empty() && !a.is_empty() => Some(format!("{m}\n\n{a}")),
        (Some(m), _) if !m.is_empty() => Some(m.to_string()),
        (_, Some(a)) if !a.is_empty() => Some(a.to_string()),
        _ => None,
    }
}
