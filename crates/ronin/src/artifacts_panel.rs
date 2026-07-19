//! Presentation model for the dedicated Artifacts panel.

use ronin_core::Artifact;

/// Maximum characters shown in an artifact preview card snippet.
pub const ARTIFACT_SNIPPET_CHARS: usize = 100;

/// Kind badge label for document artifact preview cards.
pub const ARTIFACT_KIND_BADGE: &str = "Artifact";

/// Kind badge label for code-snippet artifact preview cards.
pub const SNIPPET_KIND_BADGE: &str = "Snippet";

/// Empty-state copy shown when the panel has no artifacts.
///
/// Kept in sync with [`crate::visual_polish::EmptyStateKind::NoArtifacts`].
pub const ARTIFACTS_EMPTY_STATE: &str =
    "No artifacts yet. Save an assistant message as an artifact to see it here.";

/// A preview card for one artifact in the panel list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPreviewCard {
    /// Artifact id.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Kind badge label (e.g. `"Artifact"` or `"Snippet"`).
    pub kind: String,
    /// Fence language for snippet artifacts, when present.
    pub language: Option<String>,
    /// Truncated content snippet.
    pub snippet: String,
    /// Source thread id.
    pub source_thread_id: String,
    /// Source thread title for display.
    pub source_thread_title: String,
}

/// Builds a preview card from an artifact and its source thread title.
pub fn artifact_preview_card(artifact: &Artifact, source_thread_title: &str) -> ArtifactPreviewCard {
    let kind = if artifact.is_snippet() {
        SNIPPET_KIND_BADGE
    } else {
        ARTIFACT_KIND_BADGE
    };
    ArtifactPreviewCard {
        id: artifact.id.0.clone(),
        title: artifact.title.clone(),
        kind: kind.to_string(),
        language: artifact.language.clone(),
        snippet: content_snippet(&artifact.content, ARTIFACT_SNIPPET_CHARS),
        source_thread_id: artifact.thread_id.clone(),
        source_thread_title: source_thread_title.to_string(),
    }
}

/// Badge text shown on an artifact card (language for snippets, generic otherwise).
pub fn artifact_kind_badge(artifact: &Artifact) -> String {
    if artifact.is_snippet() {
        artifact
            .language
            .as_deref()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .unwrap_or(SNIPPET_KIND_BADGE)
            .to_string()
    } else {
        ARTIFACT_KIND_BADGE.to_string()
    }
}

/// Label for saving a fenced code block as a snippet artifact.
pub fn save_code_block_as_snippet_label() -> &'static str {
    "Save snippet"
}

/// Default title when saving a code block fence as a snippet.
pub fn snippet_title_from_language(language: Option<&str>) -> String {
    match language.map(str::trim).filter(|l| !l.is_empty()) {
        Some(lang) => format!("{lang} snippet"),
        None => "code snippet".to_string(),
    }
}

/// Truncates content to `max_chars` Unicode characters for preview cards.
pub fn content_snippet(content: &str, max_chars: usize) -> String {
    let mut chars = content.chars();
    let snippet: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{snippet}…")
    } else {
        snippet
    }
}

/// Returns empty-state copy when there are no preview cards; otherwise `None`.
pub fn artifacts_empty_state(cards: &[ArtifactPreviewCard]) -> Option<&'static str> {
    if cards.is_empty() {
        Some(ARTIFACTS_EMPTY_STATE)
    } else {
        None
    }
}

/// Interaction state for the Artifacts panel (edit / delete confirmation).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactsPanelState {
    /// Artifact id pending delete confirmation, if any.
    confirm_delete_id: Option<String>,
    /// Artifact currently being edited (title + content draft).
    editing: Option<ArtifactEditDraft>,
}

/// In-progress rename/edit draft for an artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEditDraft {
    /// Artifact being edited.
    pub id: String,
    /// Draft title.
    pub title: String,
    /// Draft content.
    pub content: String,
}

impl ArtifactsPanelState {
    /// Starts a delete confirmation for `id`.
    pub fn request_delete(&mut self, id: impl Into<String>) {
        self.confirm_delete_id = Some(id.into());
    }

    /// Cancels a pending delete confirmation.
    pub fn cancel_delete(&mut self) {
        self.confirm_delete_id = None;
    }

    /// Returns the id awaiting delete confirmation, if any.
    pub fn pending_delete_id(&self) -> Option<&str> {
        self.confirm_delete_id.as_deref()
    }

    /// Confirms deletion: returns the id to delete and clears the prompt.
    /// Returns `None` if nothing was pending.
    pub fn confirm_delete(&mut self) -> Option<String> {
        self.confirm_delete_id.take()
    }

    /// Begins editing an artifact with the given title and content drafts.
    pub fn begin_edit(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) {
        self.editing = Some(ArtifactEditDraft {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        });
    }

    /// Returns the current edit draft, if any.
    pub fn editing(&self) -> Option<&ArtifactEditDraft> {
        self.editing.as_ref()
    }

    /// Updates the draft title while editing.
    pub fn set_edit_title(&mut self, title: impl Into<String>) {
        if let Some(draft) = self.editing.as_mut() {
            draft.title = title.into();
        }
    }

    /// Updates the draft content while editing.
    pub fn set_edit_content(&mut self, content: impl Into<String>) {
        if let Some(draft) = self.editing.as_mut() {
            draft.content = content.into();
        }
    }

    /// Cancels the in-progress edit without saving.
    pub fn cancel_edit(&mut self) {
        self.editing = None;
    }

    /// Completes the edit: returns the draft to persist and clears editing state.
    pub fn commit_edit(&mut self) -> Option<ArtifactEditDraft> {
        self.editing.take()
    }
}
