//! Folder attachment: interactive file selection over a bounded listing.

use std::collections::BTreeSet;

use ronin_core::{
    folder_attachment_from_selection, ContextAttachmentDraft, ContextToolError, FolderListing,
};

/// Interactive folder-attach state (listing + selection).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderAttachState {
    listing: FolderListing,
    selected: BTreeSet<String>,
}

impl FolderAttachState {
    /// Creates state with every listed file selected by default.
    pub fn from_listing(listing: FolderListing) -> Self {
        let selected = listing
            .entries
            .iter()
            .map(|e| e.relative_path.clone())
            .collect();
        Self { listing, selected }
    }

    /// Folder display name.
    pub fn name(&self) -> &str {
        &self.listing.name
    }

    /// Underlying listing (bounded).
    pub fn listing(&self) -> &FolderListing {
        &self.listing
    }

    /// Whether `relative_path` is selected.
    pub fn is_selected(&self, relative_path: &str) -> bool {
        self.selected.contains(relative_path)
    }

    /// Number of selected files.
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// Toggles selection for one listed file.
    pub fn toggle_file(&mut self, relative_path: &str) {
        if !self
            .listing
            .entries
            .iter()
            .any(|e| e.relative_path == relative_path)
        {
            return;
        }
        if !self.selected.remove(relative_path) {
            self.selected.insert(relative_path.to_string());
        }
    }

    /// Clears all selections.
    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }

    /// Selects all listed files.
    pub fn select_all(&mut self) {
        self.selected = self
            .listing
            .entries
            .iter()
            .map(|e| e.relative_path.clone())
            .collect();
    }

    /// Builds a folder attachment draft from the current selection.
    pub fn to_context_draft(&self) -> Result<ContextAttachmentDraft, ContextToolError> {
        let selected: Vec<String> = self.selected.iter().cloned().collect();
        folder_attachment_from_selection(&self.listing, &selected)
    }
}

/// Creates folder-attach state from a listing (all files selected).
pub fn folder_attach_from_listing(listing: FolderListing) -> FolderAttachState {
    FolderAttachState::from_listing(listing)
}

/// Label for the `@folder` picker row.
pub fn folder_attach_label() -> &'static str {
    "Attach folder"
}

/// Truncation hint when a listing was bounded.
pub fn folder_truncated_hint() -> &'static str {
    "Listing truncated — only the first files within depth limits are shown."
}
