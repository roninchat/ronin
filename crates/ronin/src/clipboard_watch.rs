//! Host clipboard text source for the opt-in clipboard watcher (#77).

use ronin_core::{ClipboardTextSource, ClipboardWatchError};

/// Reads clipboard text via `arboard` for the watch poll loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArboardClipboardSource;

impl ArboardClipboardSource {
    /// Creates a new arboard-backed clipboard reader.
    pub fn new() -> Self {
        Self
    }
}

impl ClipboardTextSource for ArboardClipboardSource {
    fn read_text(&self) -> Result<String, ClipboardWatchError> {
        arboard::Clipboard::new()
            .and_then(|mut c| c.get_text())
            .map_err(|e| ClipboardWatchError::ReadFailed(e.to_string()))
    }
}

/// Safe read that treats clipboard errors as empty (watch poll must not crash UI).
pub fn read_clipboard_text_lossy(source: &dyn ClipboardTextSource) -> String {
    source.read_text().unwrap_or_default()
}
