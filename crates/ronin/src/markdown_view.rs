//! Renders parsed Markdown inlines into GPUI elements.
//!
//! Text is laid out as wrapping flex rows of word segments. Words longer than
//! [`MAX_WORD_CHARS`] are split into chunks so a single unbroken token (long
//! URLs, hashes, identifiers) can wrap instead of forcing the row wider than
//! the window.

use gpui::prelude::*;
use gpui::{div, rgb, Div};

use crate::markdown::Inline;
use crate::syntax_highlight::{highlight_code, HighlightedLine};
use crate::theme::M0Theme;

/// Maximum characters a single word segment may occupy before it is split.
pub const MAX_WORD_CHARS: usize = 40;

/// Splits a word into chunks of at most `max_chars` characters, respecting
/// UTF-8 boundaries. Returns the word unchanged when it is short enough.
pub fn split_long_word(word: &str, max_chars: usize) -> Vec<&str> {
    if word.chars().count() <= max_chars {
        return vec![word];
    }
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut chars_in_chunk = 0;
    for (idx, _) in word.char_indices() {
        if chars_in_chunk == max_chars {
            chunks.push(&word[start..idx]);
            start = idx;
            chars_in_chunk = 0;
        }
        chars_in_chunk += 1;
    }
    if start < word.len() {
        chunks.push(&word[start..]);
    }
    chunks
}

/// Renders a sequence of Markdown inlines as a wrapping flex row.
///
/// Plain text is split into word segments; inline code gets a monospace
/// highlighted pill. Long words are chunked so they wrap within the container.
pub fn render_inline_flow(inlines: &[Inline], theme: &M0Theme) -> Div {
    let mut flow = div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap_1();
    for inline in inlines {
        match inline {
            Inline::Text(text) => {
                for word in text.split(' ') {
                    if word.is_empty() {
                        continue;
                    }
                    for chunk in split_long_word(word, MAX_WORD_CHARS) {
                        flow = flow.child(div().child(chunk.to_string()));
                    }
                }
            }
            Inline::Code(code) => {
                for chunk in split_long_word(code, MAX_WORD_CHARS) {
                    flow = flow.child(
                        div()
                            .bg(theme.surface_muted)
                            .rounded_sm()
                            .px_1()
                            .font_family("Courier New")
                            .text_color(theme.accent)
                            .child(chunk.to_string()),
                    );
                }
            }
        }
    }
    flow
}

/// Renders fenced code body lines with theme-aware syntax highlighting.
///
/// Uses [`highlight_code`] so streaming re-renders stay safe: unknown languages
/// and missing language tags fall back to plain monospaced text.
pub fn render_highlighted_code_lines(
    language: Option<&str>,
    content: &str,
    theme: &M0Theme,
) -> Div {
    let lines = highlight_code(language, content, theme.color_scheme);
    render_highlighted_lines(&lines)
}

fn render_highlighted_lines(lines: &[HighlightedLine]) -> Div {
    let mut code_lines = div().w_full().font_family("Courier New").flex().flex_col();
    for line in lines {
        let mut row = div().flex().flex_row().flex_wrap();
        if line.spans.is_empty() || (line.spans.len() == 1 && line.spans[0].text.is_empty()) {
            row = row.child(div().child(" "));
        } else {
            for span in &line.spans {
                let (r, g, b) = span.rgb;
                let color = rgb(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
                row = row.child(div().text_color(color).child(span.text.clone()));
            }
        }
        code_lines = code_lines.child(row);
    }
    code_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_long_word_should_return_word_unchanged_when_short() {
        assert_eq!(split_long_word("hello", 40), vec!["hello"]);
    }

    #[test]
    fn split_long_word_should_chunk_when_exceeding_max() {
        let word = "a".repeat(100);
        let chunks = split_long_word(&word, 40);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].len(), 40);
        assert_eq!(chunks[1].len(), 40);
        assert_eq!(chunks[2].len(), 20);
    }

    #[test]
    fn split_long_word_should_respect_utf8_boundaries() {
        let word = "é".repeat(50);
        let chunks = split_long_word(&word, 40);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].chars().count(), 40);
        assert_eq!(chunks[1].chars().count(), 10);
    }
}
