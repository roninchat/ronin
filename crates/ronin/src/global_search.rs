//! Global search across threads, artifacts, and memories.
//!
//! Public filter/query seams — testable without GPUI or SQLite.

/// Which content corpus a search hit belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchContentKind {
    /// Thread title and/or message body.
    Thread,
    /// Artifact title/content.
    Artifact,
    /// Memory title/content.
    Memory,
}

impl SearchContentKind {
    /// User-visible group label for result sections.
    pub fn label(self) -> &'static str {
        match self {
            Self::Thread => "Threads",
            Self::Artifact => "Artifacts",
            Self::Memory => "Memories",
        }
    }
}

/// Optional constraints applied after text matching.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchFilters {
    /// When non-empty, only these kinds are included.
    pub kinds: Vec<SearchContentKind>,
    /// Exact provider match when set (threads only typically).
    pub provider: Option<String>,
    /// Exact model match when set.
    pub model: Option<String>,
    /// Inclusive lower bound on `created_at` (Unix ms).
    pub created_after_ms: Option<i64>,
    /// Inclusive upper bound on `created_at` (Unix ms).
    pub created_before_ms: Option<i64>,
}

/// One searchable unit (thread message, artifact, or memory).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchDocument {
    /// Corpus kind.
    pub kind: SearchContentKind,
    /// Stable id for the hit (message id, artifact id, or memory id).
    pub id: String,
    /// Primary title shown in results.
    pub title: String,
    /// Full searchable body text.
    pub body: String,
    /// Owning thread when applicable.
    pub thread_id: Option<String>,
    /// Message id for scroll-to-match when kind is Thread.
    pub message_id: Option<String>,
    /// Thread provider when known.
    pub provider: Option<String>,
    /// Thread model when known.
    pub model: Option<String>,
    /// Creation time (Unix ms) used by date filters.
    pub created_at: i64,
}

/// A ranked search match with a short snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    /// Underlying document.
    pub document: SearchDocument,
    /// Short excerpt highlighting the match context.
    pub snippet: String,
    /// Higher is better (title matches outrank body-only).
    pub score: u32,
}

/// Date-range presets for the search filter UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchDatePreset {
    /// No date constraint.
    #[default]
    Any,
    /// Created within the last 7 days.
    Last7Days,
    /// Created within the last 30 days.
    Last30Days,
    /// Created within the last 365 days.
    LastYear,
}

impl SearchDatePreset {
    /// Applies this preset onto `filters` using `now_ms` as the reference clock.
    pub fn apply(self, filters: &mut SearchFilters, now_ms: i64) {
        let day_ms = 86_400_000_i64;
        filters.created_before_ms = None;
        filters.created_after_ms = match self {
            Self::Any => None,
            Self::Last7Days => Some(now_ms - 7 * day_ms),
            Self::Last30Days => Some(now_ms - 30 * day_ms),
            Self::LastYear => Some(now_ms - 365 * day_ms),
        };
    }
}

/// UI state for the global search overlay.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SearchPanelState {
    open: bool,
    query: String,
    filters: SearchFilters,
    selected: usize,
    date_preset: SearchDatePreset,
}

impl SearchPanelState {
    /// Whether the search overlay is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Opens the search overlay.
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Closes the search overlay.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Toggles the search overlay.
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Current query string.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Replaces the query (caller re-runs [`search`]).
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected = 0;
    }

    /// Active filters.
    pub fn filters(&self) -> &SearchFilters {
        &self.filters
    }

    /// Mutable filters.
    pub fn filters_mut(&mut self) -> &mut SearchFilters {
        &mut self.filters
    }

    /// Active date-range preset (for UI chip highlighting).
    pub fn date_preset(&self) -> SearchDatePreset {
        self.date_preset
    }

    /// Sets a date-range preset and updates filter bounds from `now_ms`.
    pub fn set_date_preset(&mut self, preset: SearchDatePreset, now_ms: i64) {
        self.date_preset = preset;
        preset.apply(&mut self.filters, now_ms);
        self.selected = 0;
    }

    /// Selected result index within the current hit list.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Moves selection within `hit_count` (wrapping).
    pub fn move_selection(&mut self, delta: isize, hit_count: usize) {
        if hit_count == 0 {
            self.selected = 0;
            return;
        }
        let len = hit_count as isize;
        let next = (self.selected as isize + delta).rem_euclid(len);
        self.selected = next as usize;
    }
}

/// Returns whether `doc` passes `filters` (ignoring the text query).
pub fn matches_filters(doc: &SearchDocument, filters: &SearchFilters) -> bool {
    if !filters.kinds.is_empty() && !filters.kinds.contains(&doc.kind) {
        return false;
    }
    if let Some(provider) = filters.provider.as_deref() {
        match doc.provider.as_deref() {
            Some(p) if p.eq_ignore_ascii_case(provider) => {}
            _ => return false,
        }
    }
    if let Some(model) = filters.model.as_deref() {
        match doc.model.as_deref() {
            Some(m) if m.eq_ignore_ascii_case(model) => {}
            _ => return false,
        }
    }
    if let Some(after) = filters.created_after_ms {
        if doc.created_at < after {
            return false;
        }
    }
    if let Some(before) = filters.created_before_ms {
        if doc.created_at > before {
            return false;
        }
    }
    true
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(&needle.to_ascii_lowercase())
}

fn snippet_around(text: &str, needle: &str) -> String {
    let lower = text.to_ascii_lowercase();
    let n = needle.to_ascii_lowercase();
    let Some(pos) = lower.find(&n) else {
        let mut chars = text.chars();
        let s: String = chars.by_ref().take(80).collect();
        if chars.next().is_some() {
            return format!("{s}…");
        }
        return s;
    };
    let start = pos.saturating_sub(24);
    let end = (pos + needle.len() + 24).min(text.len());
    // Align to char boundaries
    let start = text
        .char_indices()
        .find(|(i, _)| *i >= start)
        .map(|(i, _)| i)
        .unwrap_or(0);
    let end = text
        .char_indices()
        .find(|(i, _)| *i >= end)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    let mut out = String::new();
    if start > 0 {
        out.push('…');
    }
    out.push_str(&text[start..end]);
    if end < text.len() {
        out.push('…');
    }
    out
}

/// Runs a case-insensitive substring search with ranking and filters.
///
/// Empty / whitespace-only queries return no hits (search is opt-in).
pub fn search(query: &str, docs: &[SearchDocument], filters: &SearchFilters) -> Vec<SearchHit> {
    let needle = query.trim();
    if needle.is_empty() {
        return Vec::new();
    }

    let mut hits = Vec::new();
    for doc in docs {
        if !matches_filters(doc, filters) {
            continue;
        }
        let in_title = contains_ci(&doc.title, needle);
        let in_body = contains_ci(&doc.body, needle);
        if !in_title && !in_body {
            continue;
        }
        let score = if in_title { 100 } else { 0 } + if in_body { 10 } else { 0 };
        let snippet = if in_title && !in_body {
            doc.title.clone()
        } else if in_body {
            snippet_around(&doc.body, needle)
        } else {
            doc.title.clone()
        };
        hits.push(SearchHit {
            document: doc.clone(),
            snippet,
            score,
        });
    }
    hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.document.created_at.cmp(&b.document.created_at))
            .then_with(|| a.document.id.cmp(&b.document.id))
    });
    hits
}

/// Groups hits in display order: Threads → Artifacts → Memories.
pub fn group_hits_by_kind(hits: &[SearchHit]) -> Vec<(SearchContentKind, Vec<&SearchHit>)> {
    let order = [
        SearchContentKind::Thread,
        SearchContentKind::Artifact,
        SearchContentKind::Memory,
    ];
    let mut out = Vec::new();
    for kind in order {
        let group: Vec<&SearchHit> = hits.iter().filter(|h| h.document.kind == kind).collect();
        if !group.is_empty() {
            out.push((kind, group));
        }
    }
    out
}

/// Builds searchable documents for one thread's title (no messages).
pub fn thread_title_document(
    thread_id: &str,
    title: &str,
    provider: Option<&str>,
    model: Option<&str>,
    created_at: i64,
) -> SearchDocument {
    SearchDocument {
        kind: SearchContentKind::Thread,
        id: format!("thread:{thread_id}"),
        title: title.to_string(),
        body: String::new(),
        thread_id: Some(thread_id.to_string()),
        message_id: None,
        provider: provider.map(str::to_string),
        model: model.map(str::to_string),
        created_at,
    }
}

/// Builds a searchable document for one message within a thread.
pub fn thread_message_document(
    thread_id: &str,
    message_id: &str,
    thread_title: &str,
    content: &str,
    provider: Option<&str>,
    model: Option<&str>,
    created_at: i64,
) -> SearchDocument {
    SearchDocument {
        kind: SearchContentKind::Thread,
        id: message_id.to_string(),
        title: thread_title.to_string(),
        body: content.to_string(),
        thread_id: Some(thread_id.to_string()),
        message_id: Some(message_id.to_string()),
        provider: provider.map(str::to_string),
        model: model.map(str::to_string),
        created_at,
    }
}

/// Builds a searchable artifact document.
pub fn artifact_document(
    id: &str,
    title: &str,
    content: &str,
    thread_id: &str,
    created_at: i64,
) -> SearchDocument {
    SearchDocument {
        kind: SearchContentKind::Artifact,
        id: id.to_string(),
        title: title.to_string(),
        body: content.to_string(),
        thread_id: Some(thread_id.to_string()),
        message_id: None,
        provider: None,
        model: None,
        created_at,
    }
}

/// Builds a searchable memory document.
pub fn memory_document(id: &str, title: &str, content: &str, created_at: i64) -> SearchDocument {
    SearchDocument {
        kind: SearchContentKind::Memory,
        id: id.to_string(),
        title: title.to_string(),
        body: content.to_string(),
        thread_id: None,
        message_id: None,
        provider: None,
        model: None,
        created_at,
    }
}
