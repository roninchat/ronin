//! Inline thread rename presentation and title-generation status copy.
//!
//! Public seams for sidebar rename UX — testable without GPUI pixels.

/// Status hint shown while an extra model request generates a thread title.
pub const TITLE_GENERATING_HINT: &str = "Generating title (extra model request)…";

/// In-progress inline rename draft for a sidebar thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadRenameDraft {
    /// Thread being renamed.
    pub thread_id: String,
    /// Draft title text.
    pub draft: String,
}

/// Interaction state for sidebar thread rename.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadRenameState {
    editing: Option<ThreadRenameDraft>,
}

impl ThreadRenameState {
    /// Begins inline rename for `thread_id` with the current title as draft.
    pub fn begin_rename(&mut self, thread_id: impl Into<String>, current_title: impl Into<String>) {
        self.editing = Some(ThreadRenameDraft {
            thread_id: thread_id.into(),
            draft: current_title.into(),
        });
    }

    /// Returns the active rename draft, if any.
    pub fn editing(&self) -> Option<&ThreadRenameDraft> {
        self.editing.as_ref()
    }

    /// Updates the draft title while editing.
    pub fn update_draft(&mut self, draft: impl Into<String>) {
        if let Some(editing) = self.editing.as_mut() {
            editing.draft = draft.into();
        }
    }

    /// Cancels rename without persisting.
    pub fn cancel(&mut self) {
        self.editing = None;
    }

    /// Commits the draft when non-empty after trim; leaves editing active on failure.
    pub fn commit(&mut self) -> Option<ThreadRenameDraft> {
        let editing = self.editing.as_ref()?;
        let trimmed = editing.draft.trim().to_string();
        if trimmed.is_empty() {
            return None;
        }
        let committed = ThreadRenameDraft {
            thread_id: editing.thread_id.clone(),
            draft: trimmed,
        };
        self.editing = None;
        Some(committed)
    }
}

/// Label disclosing that title generation uses an extra model request.
pub fn title_generation_status_label(generating: bool) -> Option<&'static str> {
    if generating {
        Some(TITLE_GENERATING_HINT)
    } else {
        None
    }
}

/// Compact sidebar badge when a thread has an active chat generation.
pub const THREAD_GENERATING_BADGE: &str = "●";

/// Formats a sidebar thread title, marking active generations.
pub fn format_sidebar_thread_title(title: &str, generating: bool) -> String {
    if generating {
        format!("{THREAD_GENERATING_BADGE} {title}")
    } else {
        title.to_string()
    }
}
