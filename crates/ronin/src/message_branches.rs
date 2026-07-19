//! Message edit drafts and branch navigation presentation labels.

/// Formats a 0-based branch index for the sidebar/message chrome (`1 / N`).
pub fn branch_nav_label(selected_index: usize, total: usize) -> String {
    format!("{} / {}", selected_index.saturating_add(1), total.max(1))
}

/// In-progress edit of a previously sent user message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageEditDraft {
    /// Message being edited.
    pub message_id: String,
    /// Draft replacement content.
    pub draft: String,
}

/// UI state for post-send message editing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MessageEditState {
    editing: Option<MessageEditDraft>,
}

impl MessageEditState {
    /// Starts editing `message_id` with the current content as the draft.
    pub fn begin_edit(&mut self, message_id: impl Into<String>, current: impl Into<String>) {
        self.editing = Some(MessageEditDraft {
            message_id: message_id.into(),
            draft: current.into(),
        });
    }

    /// Active edit draft, if any.
    pub fn editing(&self) -> Option<&MessageEditDraft> {
        self.editing.as_ref()
    }

    /// Updates the draft text.
    pub fn update_draft(&mut self, draft: impl Into<String>) {
        if let Some(editing) = self.editing.as_mut() {
            editing.draft = draft.into();
        }
    }

    /// Cancels without committing.
    pub fn cancel(&mut self) {
        self.editing = None;
    }
}

/// Commits a non-empty trimmed draft and clears editing state.
pub fn edit_draft_commit(state: &mut MessageEditState) -> Option<MessageEditDraft> {
    let Some(editing) = state.editing.as_ref() else {
        return None;
    };
    let trimmed = editing.draft.trim().to_string();
    if trimmed.is_empty() {
        return None;
    }
    let committed = MessageEditDraft {
        message_id: editing.message_id.clone(),
        draft: trimmed,
    };
    state.editing = None;
    Some(committed)
}
