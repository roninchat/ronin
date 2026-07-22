//! Folder attachment: interactive file selection over a bounded listing.

use std::collections::BTreeSet;

use ronin_core::{
    folder_attachment_from_selection, ContextAttachmentDraft, ContextToolError, FolderEntry,
    FolderListOptions, FolderListing,
};

/// Interactive folder-attach state (listing + selection + browse filter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderAttachState {
    listing: FolderListing,
    selected: BTreeSet<String>,
    /// UI browse filter — narrows [`Self::visible_entries`] before selection.
    browse_filter: String,
}

impl FolderAttachState {
    /// Creates state with every listed file selected by default.
    pub fn from_listing(listing: FolderListing) -> Self {
        let selected = listing
            .entries
            .iter()
            .map(|e| e.relative_path.clone())
            .collect();
        Self {
            listing,
            selected,
            browse_filter: String::new(),
        }
    }

    /// Folder display name.
    pub fn name(&self) -> &str {
        &self.listing.name
    }

    /// Underlying listing (bounded).
    pub fn listing(&self) -> &FolderListing {
        &self.listing
    }

    /// Current UI browse filter text.
    pub fn browse_filter(&self) -> &str {
        &self.browse_filter
    }

    /// Sets the browse filter used by [`Self::visible_entries`].
    ///
    /// Does not change selection; empty/whitespace clears the filter.
    pub fn set_browse_filter(&mut self, filter: impl Into<String>) {
        self.browse_filter = filter.into();
    }

    /// Clears the browse filter so all listed entries are visible again.
    pub fn clear_browse_filter(&mut self) {
        self.browse_filter.clear();
    }

    /// Entries visible under the current browse filter (case-insensitive path substring).
    pub fn visible_entries(&self) -> Vec<&FolderEntry> {
        let filter = self.browse_filter.trim();
        if filter.is_empty() {
            return self.listing.entries.iter().collect();
        }
        let needle = filter.to_ascii_lowercase();
        self.listing
            .entries
            .iter()
            .filter(|e| e.relative_path.to_ascii_lowercase().contains(&needle))
            .collect()
    }

    /// Options for a progressive deepen re-list of this folder.
    #[must_use]
    pub fn deepen_options(&self) -> FolderListOptions {
        self.listing.list_options.deepen()
    }

    /// Whether progressive deepen can raise listing caps further.
    #[must_use]
    pub fn can_reveal_more(&self) -> bool {
        self.listing.truncated && self.listing.list_options.can_deepen()
    }

    /// Replaces the listing after a progressive deepen / re-filter walk.
    ///
    /// Keeps selections that still exist in the new listing; newly listed files
    /// are not auto-selected (explicit selection remains mandatory for new paths).
    pub fn replace_listing(&mut self, listing: FolderListing) {
        let paths: BTreeSet<&str> = listing
            .entries
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        self.selected.retain(|p| paths.contains(p.as_str()));
        self.listing = listing;
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

    /// Selects all currently visible files (respects browse filter).
    pub fn select_all_visible(&mut self) {
        let paths: Vec<String> = self
            .visible_entries()
            .iter()
            .map(|e| e.relative_path.clone())
            .collect();
        for path in paths {
            self.selected.insert(path);
        }
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
    "Listing truncated — reveal more or narrow with the browse filter."
}

/// Label for the progressive deepen control.
pub fn folder_reveal_more_label() -> &'static str {
    "Reveal more"
}

/// Placeholder for the folder browse filter field.
pub fn folder_browse_filter_placeholder() -> &'static str {
    "Filter files…"
}
