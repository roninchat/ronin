//! Composer `@` attachment picker and `/` action picker.
//!
//! Presentation seams for filterable inline menus — testable without GPUI.

use crate::completions::token_before_cursor;

/// Which picker menu is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerKind {
    /// `@` attachment source menu.
    AtAttachment,
    /// `/` slash-command menu.
    SlashAction,
}

/// Attachment source selected from the `@` picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtAttachmentKind {
    /// `@file:` path attachment.
    File,
    /// `@folder:` directory attachment with file selection.
    Folder,
    /// `@clipboard` paste attachment.
    Clipboard,
    /// `@screenshot` capture attachment.
    Screenshot,
    /// `@artifact:` saved artifact ref.
    Artifact,
    /// `@memory:` saved memory ref.
    Memory,
}

/// Slash command selected from the `/` picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashActionKind {
    /// `/new` — create a new thread.
    NewThread,
    /// `/clear` — clear composer text.
    ClearComposer,
    /// `/model` — switch the active model.
    SwitchModel,
    /// `/clipboard-watch` — toggle opt-in clipboard watch.
    ClipboardWatchToggle,
    /// `/clipboard-confirm` — confirm pending clipboard-watch attach proposal.
    ClipboardWatchConfirm,
    /// `/clipboard-dismiss` — dismiss pending clipboard-watch proposal.
    ClipboardWatchDismiss,
}

/// One row in an `@` or `/` picker menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PickerItem {
    /// Text inserted / matched (e.g. `"@file:"`, `"/new"`).
    pub insert: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    kind: PickerItemKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickerItemKind {
    At(AtAttachmentKind),
    Slash(SlashActionKind),
}

impl PickerItem {
    /// Returns the `@` attachment kind when this item is an attachment option.
    pub fn at_kind(self) -> Option<AtAttachmentKind> {
        match self.kind {
            PickerItemKind::At(k) => Some(k),
            PickerItemKind::Slash(_) => None,
        }
    }

    /// Returns the slash action kind when this item is a slash command.
    pub fn slash_kind(self) -> Option<SlashActionKind> {
        match self.kind {
            PickerItemKind::Slash(k) => Some(k),
            PickerItemKind::At(_) => None,
        }
    }
}

/// Active picker snapshot for the current composer token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivePicker {
    /// `@` vs `/` menu.
    pub kind: PickerKind,
    /// Byte offset of the trigger token.
    pub token_start: usize,
    /// Text after `@` or `/` used for filtering.
    pub query: String,
    /// Filtered options.
    pub items: Vec<PickerItem>,
}

/// Catalog of `@` attachment sources.
pub fn at_attachment_catalog() -> &'static [PickerItem] {
    &[
        PickerItem {
            insert: "@file:",
            label: "Attach file",
            kind: PickerItemKind::At(AtAttachmentKind::File),
        },
        PickerItem {
            insert: "@folder:",
            label: "Attach folder",
            kind: PickerItemKind::At(AtAttachmentKind::Folder),
        },
        PickerItem {
            insert: "@clipboard",
            label: "Attach clipboard",
            kind: PickerItemKind::At(AtAttachmentKind::Clipboard),
        },
        PickerItem {
            insert: "@screenshot",
            label: "Capture screenshot",
            kind: PickerItemKind::At(AtAttachmentKind::Screenshot),
        },
        PickerItem {
            insert: "@artifact:",
            label: "Attach artifact",
            kind: PickerItemKind::At(AtAttachmentKind::Artifact),
        },
        PickerItem {
            insert: "@memory:",
            label: "Attach memory",
            kind: PickerItemKind::At(AtAttachmentKind::Memory),
        },
    ]
}

/// Catalog of `/` slash actions.
pub fn slash_action_catalog() -> &'static [PickerItem] {
    &[
        PickerItem {
            insert: "/new",
            label: "New thread",
            kind: PickerItemKind::Slash(SlashActionKind::NewThread),
        },
        PickerItem {
            insert: "/clear",
            label: "Clear composer",
            kind: PickerItemKind::Slash(SlashActionKind::ClearComposer),
        },
        PickerItem {
            insert: "/model",
            label: "Switch model",
            kind: PickerItemKind::Slash(SlashActionKind::SwitchModel),
        },
        PickerItem {
            insert: "/clipboard-watch",
            label: "Toggle clipboard watch",
            kind: PickerItemKind::Slash(SlashActionKind::ClipboardWatchToggle),
        },
        PickerItem {
            insert: "/clipboard-confirm",
            label: "Confirm clipboard attach",
            kind: PickerItemKind::Slash(SlashActionKind::ClipboardWatchConfirm),
        },
        PickerItem {
            insert: "/clipboard-dismiss",
            label: "Dismiss clipboard proposal",
            kind: PickerItemKind::Slash(SlashActionKind::ClipboardWatchDismiss),
        },
    ]
}

/// Filters catalog rows whose insert (without leading trigger) or label match `query`.
pub fn filter_picker_items(catalog: &[PickerItem], query: &str) -> Vec<PickerItem> {
    let q = query.to_ascii_lowercase();
    catalog
        .iter()
        .copied()
        .filter(|item| {
            if q.is_empty() {
                return true;
            }
            let insert_tail = item
                .insert
                .trim_start_matches(['@', '/'])
                .to_ascii_lowercase();
            insert_tail.starts_with(&q)
                || item.label.to_ascii_lowercase().contains(&q)
                || item.insert.to_ascii_lowercase().contains(&q)
        })
        .collect()
}

/// Detects an active `@` or `/` picker at `cursor`, or `None` when inactive.
///
/// Mid-word triggers (e.g. `user@host`, `a/b`) do not open a picker because the
/// whitespace-delimited token does not start with `@` / `/`.
pub fn detect_active_picker(text: &str, cursor: usize) -> Option<ActivePicker> {
    let (token_start, token) = token_before_cursor(text, cursor);
    if token.starts_with('@') {
        return detect_at_picker(token_start, token);
    }
    if token.starts_with('/') {
        return detect_slash_picker(token_start, token);
    }
    None
}

fn detect_at_picker(token_start: usize, token: &str) -> Option<ActivePicker> {
    // Sub-flows own the UI once a colon payload command is underway.
    if token.starts_with("@file:")
        || token.starts_with("@folder:")
        || token.starts_with("@memory:")
        || token.starts_with("@artifact:")
    {
        return None;
    }
    let query = token[1..].to_string();
    let items = filter_picker_items(at_attachment_catalog(), &query);
    Some(ActivePicker {
        kind: PickerKind::AtAttachment,
        token_start,
        query,
        items,
    })
}

fn detect_slash_picker(token_start: usize, token: &str) -> Option<ActivePicker> {
    // Require a true slash-command token: `/` or `/partial` without extra path segments
    // after a second slash (avoids treating `http://…` — which does not start with `/`).
    if token.matches('/').count() > 1 {
        return None;
    }
    let query = token[1..].to_string();
    let items = filter_picker_items(slash_action_catalog(), &query);
    Some(ActivePicker {
        kind: PickerKind::SlashAction,
        token_start,
        query,
        items,
    })
}

/// Moves the highlighted picker row by `delta`, wrapping within `len`.
pub fn move_picker_selection(selected: usize, len: usize, delta: i32) -> usize {
    if len == 0 {
        return 0;
    }
    let len_i = len as i32;
    let next = selected as i32 + delta;
    let wrapped = ((next % len_i) + len_i) % len_i;
    wrapped as usize
}
