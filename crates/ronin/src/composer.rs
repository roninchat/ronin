//! Composer editor state, input handling, and rendering for Ronin GPUI shell.

use gpui::prelude::*;
use gpui::{div, px, rgb, IntoElement, KeyDownEvent};

/// Cursor-and-selection editor for Ronin's composer input.
pub struct ComposerEditor {
    text: String,
    cursor: usize,
    selection: Option<(usize, usize)>,
    dragging: bool,
    drag_anchor: usize,
    char_width: f32,
    line_height: f32,
    /// Last known container pixel width for layout calcs.
    container_width: f32,
    /// Whether the text cursor is currently visible (for blinking).
    pub cursor_visible: bool,
}

impl Default for ComposerEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposerEditor {
    /// Creates an empty composer editor with default font metrics.
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            selection: None,
            dragging: false,
            drag_anchor: 0,
            char_width: 8.0,
            line_height: 18.0,
            container_width: 400.0,
            cursor_visible: true,
        }
    }

    /// Update font metrics from rem size.
    pub fn set_font_metrics_from_rem(&mut self, rem_px: f32) {
        // Monospace approx: char width ≈ 0.6 * rem, line height ≈ 1.5 * rem
        self.char_width = rem_px * 0.6;
        self.line_height = rem_px * 1.5;
    }

    /// Set container pixel width for layout computations.
    pub fn set_container_width(&mut self, width: f32) {
        self.container_width = width.max(1.0);
    }

    // ── text access ──

    /// Returns the current text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Replaces the text, clamping the cursor if necessary.
    pub fn set_text(&mut self, text: String) {
        self.cursor = self.cursor.min(text.len());
        self.text = text;
    }

    /// Takes the text, resetting cursor and selection.
    pub fn take_text(&mut self) -> String {
        self.cursor = 0;
        self.selection = None;
        std::mem::take(&mut self.text)
    }

    /// Returns the current byte cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor.min(self.text.len())
    }

    /// Returns the current selection range if any.
    pub fn selection(&self) -> Option<(usize, usize)> {
        self.selection
    }

    // ── visual lines ──

    /// Returns visual lines (byte ranges) splitting on explicit `\n` and soft-wrapping.
    pub fn visual_lines(&self) -> Vec<(usize, usize)> {
        if self.text.is_empty() {
            return vec![(0, 0)];
        }
        let max_chars = (self.container_width / self.char_width).max(1.0) as usize;
        let mut lines = Vec::new();
        let mut byte_pos = 0;
        for logical in self.text.split('\n') {
            if logical.is_empty() {
                lines.push((byte_pos, byte_pos));
            } else {
                let mut start = byte_pos;
                let end = byte_pos + logical.len();
                while start < end {
                    let mut line_end = start;
                    for (count, (_, c)) in self.text[start..end].char_indices().enumerate() {
                        if count >= max_chars {
                            break;
                        }
                        line_end += c.len_utf8();
                    }
                    // On last chunk, go to end
                    if line_end == start || line_end >= end {
                        line_end = end;
                    }
                    lines.push((start, line_end));
                    start = line_end;
                }
            }
            byte_pos += logical.len() + 1; // +1 for the \n
        }
        if lines.is_empty() {
            lines.push((0, 0));
        }
        lines
    }

    /// Returns the index of the visual line containing the given byte position.
    pub fn visual_line_index(&self, byte_pos: usize) -> usize {
        let lines = self.visual_lines();
        for (i, &(s, e)) in lines.iter().enumerate() {
            if byte_pos >= s && byte_pos <= e {
                return i;
            }
        }
        lines.len().saturating_sub(1)
    }

    /// Byte offset within the current visual line.
    fn column_in_line(&self, byte_pos: usize) -> usize {
        let lines = self.visual_lines();
        let li = self.visual_line_index(byte_pos);
        if li >= lines.len() {
            return 0;
        }
        let (s, _) = lines[li];
        byte_pos.saturating_sub(s)
    }

    /// Converts (line_index, column_bytes) to a byte offset in the full text.
    fn byte_from_line_col(&self, line_index: usize, col_bytes: usize) -> usize {
        let lines = self.visual_lines();
        if lines.is_empty() {
            return 0;
        }
        let (s, e) = lines[line_index.min(lines.len() - 1)];
        (s + col_bytes).min(e)
    }

    // ── cursor helpers ──

    fn clamp_cursor(&mut self) {
        self.cursor = self.cursor.min(self.text.len());
        while !self.text.is_char_boundary(self.cursor) {
            self.cursor -= 1;
        }
    }

    fn clear_selection(&mut self) {
        self.selection = None;
    }

    fn set_selection_range(&mut self, start: usize, end: usize) {
        let s = start.min(self.text.len());
        let e = end.min(self.text.len());
        if s == e {
            self.selection = None;
        } else {
            self.selection = Some((s.min(e), s.max(e)));
        }
    }

    fn selection_anchor(&self) -> usize {
        self.selection.map(|(s, _)| s).unwrap_or(self.cursor)
    }

    fn selection_cursor(&self) -> usize {
        self.selection.map(|(_, e)| e).unwrap_or(self.cursor)
    }

    /// Returns the currently selected text.
    pub fn selected_text(&self) -> Option<&str> {
        self.selection.map(|(s, e)| &self.text[s..e])
    }

    // ── word movement ──

    fn prev_word_byte(&self, from: usize) -> usize {
        let bytes = self.text.as_bytes();
        let mut i = from.min(bytes.len());
        while i > 0 && bytes[i - 1] == b' ' {
            i -= 1;
        }
        while i > 0 && bytes[i - 1] != b' ' {
            i -= 1;
        }
        i
    }

    fn next_word_byte(&self, from: usize) -> usize {
        let bytes = self.text.as_bytes();
        let mut i = from;
        while i < bytes.len() && bytes[i] != b' ' {
            i += 1;
        }
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        i
    }

    // ── editing ──

    fn delete_selection(&mut self) {
        if let Some((start, end)) = self.selection.take() {
            self.text.drain(start..end);
            self.cursor = start;
        }
    }

    /// Inserts a single character at the current cursor position.
    pub fn insert_char(&mut self, ch: char) {
        self.delete_selection();
        self.clamp_cursor();
        let mut buf = [0u8; 4];
        self.text.insert_str(self.cursor, ch.encode_utf8(&mut buf));
        self.cursor += ch.len_utf8();
    }

    /// Inserts a string at the current cursor position.
    pub fn insert_str(&mut self, s: &str) {
        self.delete_selection();
        self.clamp_cursor();
        self.text.insert_str(self.cursor, s);
        self.cursor += s.len();
    }

    /// Deletes one character before the cursor (or the selection).
    pub fn delete_before_cursor(&mut self) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }
        self.clamp_cursor();
        if self.cursor == 0 {
            return;
        }
        let prev = self.text[..self.cursor]
            .char_indices()
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.text.drain(prev..self.cursor);
        self.cursor = prev;
    }

    /// Deletes one character after the cursor (or the selection).
    pub fn delete_at_cursor(&mut self) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }
        self.clamp_cursor();
        if self.cursor >= self.text.len() {
            return;
        }
        let next = self.text[self.cursor..]
            .char_indices()
            .nth(1)
            .map(|(i, _)| self.cursor + i)
            .unwrap_or(self.text.len());
        self.text.drain(self.cursor..next);
    }

    /// Deletes the word preceding the cursor (or the selection).
    pub fn delete_prev_word(&mut self) {
        if self.selection.is_some() {
            self.delete_selection();
            return;
        }
        let target = self.prev_word_byte(self.cursor);
        self.text.drain(target..self.cursor);
        self.cursor = target;
    }

    // ── movement ──

    /// Moves the cursor one character left.
    pub fn move_left(&mut self) {
        self.clamp_cursor();
        if self.cursor == 0 {
            return;
        }
        self.cursor = self.text[..self.cursor]
            .char_indices()
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);
    }

    /// Moves the cursor one character right.
    pub fn move_right(&mut self) {
        self.clamp_cursor();
        if self.cursor >= self.text.len() {
            return;
        }
        self.cursor = self.text[self.cursor..]
            .char_indices()
            .nth(1)
            .map(|(i, _)| self.cursor + i)
            .unwrap_or(self.text.len());
    }

    /// Moves the cursor one word left.
    pub fn move_word_left(&mut self) {
        self.clamp_cursor();
        self.cursor = self.prev_word_byte(self.cursor);
    }

    /// Moves the cursor one word right.
    pub fn move_word_right(&mut self) {
        self.clamp_cursor();
        self.cursor = self.next_word_byte(self.cursor);
    }

    /// Moves the cursor up one visual line, preserving column.
    pub fn move_up(&mut self) {
        let li = self.visual_line_index(self.cursor);
        if li == 0 {
            self.cursor = 0;
            return;
        }
        let col = self.column_in_line(self.cursor);
        self.cursor = self.byte_from_line_col(li.saturating_sub(1), col);
    }

    /// Moves the cursor down one visual line, preserving column.
    pub fn move_down(&mut self) {
        let lines = self.visual_lines();
        let li = self.visual_line_index(self.cursor);
        if li >= lines.len().saturating_sub(1) {
            self.cursor = self.text.len();
            return;
        }
        let col = self.column_in_line(self.cursor);
        self.cursor = self.byte_from_line_col(li + 1, col);
    }

    // ── selection extensions ──

    /// Extends the selection one character left.
    pub fn extend_left(&mut self) {
        let anchor = self.selection_anchor();
        let cur = self.selection_cursor();
        if cur == 0 {
            return;
        }
        let new = self.text[..cur]
            .char_indices()
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.cursor = new;
        self.set_selection_range(anchor, new);
    }

    /// Extends the selection one character right.
    pub fn extend_right(&mut self) {
        let anchor = self.selection_anchor();
        let cur = self.selection_cursor();
        if cur >= self.text.len() {
            return;
        }
        let new = self.text[cur..]
            .char_indices()
            .nth(1)
            .map(|(i, _)| cur + i)
            .unwrap_or(self.text.len());
        self.cursor = new;
        self.set_selection_range(anchor, new);
    }

    /// Extends the selection one word left.
    pub fn extend_word_left(&mut self) {
        let anchor = self.selection_anchor();
        let new = self.prev_word_byte(anchor);
        self.set_selection_range(new, self.cursor);
    }

    /// Extends the selection one word right.
    pub fn extend_word_right(&mut self) {
        let anchor = self.selection_anchor();
        if self.cursor < self.text.len() {
            let new = self.next_word_byte(self.cursor);
            self.cursor = new;
            self.set_selection_range(anchor, new);
        }
    }

    /// Extends the selection up one visual line.
    pub fn extend_up(&mut self) {
        let anchor = self.selection_anchor();
        let cur = self.selection_cursor();
        let cur_li = self.visual_line_index(cur);
        let col = self.column_in_line(cur);
        let new = if cur_li == 0 {
            0
        } else {
            self.byte_from_line_col(cur_li.saturating_sub(1), col)
        };
        self.cursor = new;
        self.set_selection_range(anchor, new);
    }

    /// Extends the selection down one visual line.
    pub fn extend_down(&mut self) {
        let anchor = self.selection_anchor();
        let cur = self.selection_cursor();
        let lines = self.visual_lines();
        let cur_li = self.visual_line_index(cur);
        let col = self.column_in_line(cur);
        let new = if cur_li >= lines.len().saturating_sub(1) {
            self.text.len()
        } else {
            self.byte_from_line_col(cur_li + 1, col)
        };
        self.cursor = new;
        self.set_selection_range(anchor, new);
    }

    /// Selects all text.
    pub fn select_all(&mut self) {
        let len = self.text.len();
        if len > 0 {
            self.selection = Some((0, len));
            self.cursor = len;
        }
    }

    /// Replaces a range of text (used for command completions).
    pub fn replace_range(&mut self, start: usize, end: usize, replacement: &str) {
        self.text.replace_range(start..end, replacement);
        self.cursor = start + replacement.len();
        self.selection = None;
    }

    // ── mouse input ──

    /// Converts pixel position to byte offset.
    pub fn cursor_from_point(&self, x: f32, y: f32) -> usize {
        let line_index = (y / self.line_height.max(1.0)).floor().max(0.0) as usize;
        let char_index = (x / self.char_width.max(1.0)).floor().max(0.0) as usize;

        let lines = self.visual_lines();
        if line_index >= lines.len() {
            return self.text.len();
        }
        let (s, e) = lines[line_index];
        let col_bytes = {
            let mut byte_count = 0;
            for (char_count, (_, c)) in self.text[s..e].char_indices().enumerate() {
                if char_count >= char_index {
                    break;
                }
                byte_count += c.len_utf8();
            }
            byte_count
        };
        (s + col_bytes).min(e)
    }

    /// Handles mouse click when pixel positioning is unavailable.
    /// Moves cursor to end of text.
    pub fn click_at_end(&mut self) {
        self.drag_anchor = self.cursor;
        self.dragging = true;
    }

    /// Handles mouse drag when pixel positioning is unavailable.
    /// Extends selection from drag anchor to end of text.
    /// Returns true if selection changed.
    pub fn drag_to_end(&mut self) -> bool {
        if !self.dragging {
            return false;
        }
        let end = self.text.len();
        if self.drag_anchor == end {
            return false;
        }
        self.cursor = end;
        self.set_selection_range(self.drag_anchor, end);
        true
    }

    /// Ends a mouse drag.
    pub fn end_drag(&mut self) {
        self.dragging = false;
    }

    // ── rendering ──

    /// Renders the composer text with cursor and selection into a GPUI column element.
    /// Each visual line is a separate row.
    pub fn render_text(
        &self,
        placeholder: &str,
        text_color: gpui::Hsla,
        placeholder_color: gpui::Hsla,
        accent: gpui::Hsla,
    ) -> impl IntoElement {
        if self.text.is_empty() {
            return div()
                .text_color(placeholder_color)
                .child(placeholder.to_string());
        }

        let lines = self.visual_lines();
        let selection = self.selection;
        let cursor = self.cursor.min(self.text.len());

        let mut col = div().flex().flex_col();

        for (li, &(s, e)) in lines.iter().enumerate() {
            let line_text = &self.text[s..e];
            let is_last = li == lines.len().saturating_sub(1);

            // Determine selection within this line
            let (sel_start, sel_end) = if let Some((sel_s, sel_e)) = selection {
                if sel_s < e && sel_e > s {
                    (sel_s.max(s), sel_e.min(e))
                } else {
                    (s, s) // no selection in this line
                }
            } else {
                (s, s) // no selection at all
            };
            // Byte offsets relative to line start
            let local_sel_s = sel_start.saturating_sub(s);
            let local_sel_e = sel_end.saturating_sub(s);

            // Determine cursor within this line
            let cursor_in_line = cursor >= s && (cursor < e || (cursor == e && is_last));
            let local_cursor = if cursor_in_line {
                cursor.saturating_sub(s)
            } else {
                line_text.len()
            };

            let has_sel = sel_start < sel_end;
            let (before, mid, after) = if has_sel {
                (
                    &line_text[..local_sel_s],
                    &line_text[local_sel_s..local_sel_e],
                    &line_text[local_sel_e..],
                )
            } else {
                (&line_text[..local_cursor], "", &line_text[local_cursor..])
            };

            let mut row = div().flex().flex_row().items_center();

            if !before.is_empty() {
                row = row.child(div().text_color(text_color).child(before.to_string()));
            }

            if has_sel {
                row = row.child(
                    div()
                        .bg(rgb(0x585b70))
                        .text_color(text_color)
                        .child(mid.to_string()),
                );
            } else if cursor_in_line && self.cursor_visible {
                row = row.child(div().w(px(1.5)).h_full().bg(accent).flex_shrink_0());
            }

            if !after.is_empty() {
                row = row.child(div().text_color(text_color).child(after.to_string()));
            }

            col = col.child(row.min_h(px(self.line_height)));
        }

        col
    }

    // ── keyboard dispatch ──

    /// Processes a key-down event. Returns `true` if consumed.
    pub fn on_key_down(&mut self, event: &KeyDownEvent) -> bool {
        let ks = &event.keystroke;
        let ctrl = ks.modifiers.control;
        let shift = ks.modifiers.shift;
        let alt_or_plat = ks.modifiers.alt || ks.modifiers.platform;

        if alt_or_plat {
            return false;
        }

        match ks.key.as_str() {
            "backspace" => {
                if ctrl {
                    self.delete_prev_word();
                } else {
                    self.delete_before_cursor();
                }
                true
            }
            "delete" => {
                self.delete_at_cursor();
                true
            }
            "left" => {
                if ctrl && shift {
                    self.extend_word_left();
                } else if shift {
                    self.extend_left();
                } else if ctrl {
                    self.clear_selection();
                    self.move_word_left();
                } else {
                    self.clear_selection();
                    self.move_left();
                }
                true
            }
            "right" => {
                if ctrl && shift {
                    self.extend_word_right();
                } else if shift {
                    self.extend_right();
                } else if ctrl {
                    self.clear_selection();
                    self.move_word_right();
                } else {
                    self.clear_selection();
                    self.move_right();
                }
                true
            }
            "up" => {
                if shift {
                    self.extend_up();
                } else {
                    self.clear_selection();
                    self.move_up();
                }
                true
            }
            "down" => {
                if shift {
                    self.extend_down();
                } else {
                    self.clear_selection();
                    self.move_down();
                }
                true
            }
            "home" => {
                if shift {
                    // Extend to line start
                    let lines = self.visual_lines();
                    let li = self.visual_line_index(self.selection_cursor());
                    let line_start = lines.get(li).map(|&(s, _)| s).unwrap_or(0);
                    let anchor = self.selection_anchor();
                    self.set_selection_range(anchor, line_start);
                    self.cursor = line_start;
                } else {
                    let lines = self.visual_lines();
                    let li = self.visual_line_index(self.cursor);
                    let line_start = lines.get(li).map(|&(s, _)| s).unwrap_or(0);
                    self.clear_selection();
                    self.cursor = line_start;
                }
                true
            }
            "end" => {
                if shift {
                    let lines = self.visual_lines();
                    let li = self.visual_line_index(self.selection_cursor());
                    let line_end = lines.get(li).map(|&(_, e)| e).unwrap_or(self.text.len());
                    let anchor = self.selection_anchor();
                    self.set_selection_range(anchor, line_end);
                    self.cursor = line_end;
                } else {
                    let lines = self.visual_lines();
                    let li = self.visual_line_index(self.cursor);
                    let line_end = lines.get(li).map(|&(_, e)| e).unwrap_or(self.text.len());
                    self.clear_selection();
                    self.cursor = line_end;
                }
                true
            }
            "a" if ctrl => {
                self.select_all();
                true
            }
            _ => false,
        }
    }
}
