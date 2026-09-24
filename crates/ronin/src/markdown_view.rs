//! Renders parsed Markdown inlines into GPUI elements.
//!
//! Body copy is one wrapping [`StyledText`] with [`TextRun`] spans. Words longer
//! than [`MAX_WORD_CHARS`] get zero-width wrap breaks so a single unbroken token
//! (long URLs, hashes, identifiers) can wrap instead of forcing the row wider
//! than the window.

use gpui::prelude::*;
use gpui::{div, font, px, rgb, Div, FontStyle, FontWeight, StyledText, TextRun, UnderlineStyle};

use crate::markdown::{Inline, ListItem, MarkdownBlock};
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

/// Renders a sequence of Markdown inlines as wrapping styled text.
///
/// Strong, emphasis, code, and links become [`TextRun`] spans over one string.
/// Extremely long tokens still receive wrap opportunities via [`split_long_word`].
pub fn render_inline_flow(inlines: &[Inline], theme: &M0Theme) -> Div {
    render_inline_flow_with_weight(inlines, theme, FontWeight::NORMAL)
}

/// Renders an ATX heading. Level 1 is extra-large, 2 is 16px, 3 (and fallback) is 14px.
pub fn render_heading(level: u8, inlines: &[Inline], theme: &M0Theme) -> Div {
    let mut heading = div()
        .w_full()
        .min_w_0()
        .font_weight(FontWeight(600.))
        .text_color(theme.text_primary)
        .child(render_inline_flow_with_weight(
            inlines,
            theme,
            FontWeight(600.),
        ));
    heading = match level {
        1 => heading.text_xl(),
        2 => heading.text_size(px(16.)),
        _ => heading.text_size(px(14.)),
    };
    heading
}

/// Renders a parsed block without message-chrome (copy buttons, snippet save).
///
/// Headings, paragraphs, and lists use the styled-text path. Fenced code uses
/// [`render_highlighted_code_lines`].
pub fn render_block(block: &MarkdownBlock, theme: &M0Theme) -> Div {
    match block {
        MarkdownBlock::Paragraph(inlines) => render_inline_flow(inlines, theme),
        MarkdownBlock::Heading { level, inlines } => render_heading(*level, inlines, theme),
        MarkdownBlock::CodeBlock { language, content } => {
            render_highlighted_code_lines(language.as_deref(), content, theme)
        }
        MarkdownBlock::List(items) => render_list(items, theme),
    }
}

fn render_list(items: &[ListItem], theme: &M0Theme) -> Div {
    let mut list_div = div().w_full().min_w_0().flex().flex_col().gap_1().pl_4();
    for item in items {
        list_div = list_div.child(
            div()
                .w_full()
                .min_w_0()
                .flex()
                .flex_row()
                .gap_2()
                .child(div().flex_shrink_0().child("•"))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .child(render_inline_flow(&item.inlines, theme)),
                ),
        );
    }
    list_div
}

fn render_inline_flow_with_weight(
    inlines: &[Inline],
    theme: &M0Theme,
    base_weight: FontWeight,
) -> Div {
    let (text, runs) = styled_text_parts(inlines, theme, base_weight);
    let styled = styled_text_from_parts(text, runs);
    div().w_full().min_w_0().whitespace_normal().child(styled)
}

fn styled_text_from_parts(text: String, runs: Vec<TextRun>) -> StyledText {
    let covered: usize = runs.iter().map(|run| run.len).sum();
    if !runs.is_empty() && covered == text.len() {
        StyledText::new(text).with_runs(runs)
    } else {
        StyledText::new(text)
    }
}

#[derive(Clone, Copy)]
struct SpanFlags {
    strong: bool,
    emphasis: bool,
    code: bool,
    link: bool,
}

impl SpanFlags {
    fn plain() -> Self {
        Self {
            strong: false,
            emphasis: false,
            code: false,
            link: false,
        }
    }
}

struct RunBuilder<'a> {
    text: String,
    runs: Vec<TextRun>,
    theme: &'a M0Theme,
    base_weight: FontWeight,
}

fn styled_text_parts(
    inlines: &[Inline],
    theme: &M0Theme,
    base_weight: FontWeight,
) -> (String, Vec<TextRun>) {
    let mut builder = RunBuilder {
        text: String::new(),
        runs: Vec::new(),
        theme,
        base_weight,
    };
    builder.walk(inlines, SpanFlags::plain());
    (builder.text, builder.runs)
}

impl RunBuilder<'_> {
    fn walk(&mut self, inlines: &[Inline], flags: SpanFlags) {
        for inline in inlines {
            match inline {
                Inline::Text(text) => self.push_text(text, flags),
                Inline::Code(text) => {
                    let mut nested = flags;
                    nested.code = true;
                    self.push_text(text, nested);
                }
                Inline::Strong(inner) => {
                    let mut nested = flags;
                    nested.strong = true;
                    self.walk(inner, nested);
                }
                Inline::Emphasis(inner) => {
                    let mut nested = flags;
                    nested.emphasis = true;
                    self.walk(inner, nested);
                }
                Inline::Link { text, .. } => {
                    let mut nested = flags;
                    nested.link = true;
                    self.walk(text, nested);
                }
            }
        }
    }

    fn push_text(&mut self, raw: &str, flags: SpanFlags) {
        let piece = with_wrap_breaks(raw);
        if piece.is_empty() {
            return;
        }
        let font = font_for(flags, self.base_weight);
        let color = color_for(flags, self.theme);
        let underline = underline_for(flags, self.theme);
        let len = piece.len();
        if let Some(last) = self.runs.last_mut() {
            if last.font == font
                && last.color == color
                && last.underline == underline
                && last.background_color.is_none()
                && last.strikethrough.is_none()
            {
                last.len += len;
                self.text.push_str(&piece);
                return;
            }
        }
        self.runs.push(TextRun {
            len,
            font,
            color,
            background_color: None,
            underline,
            strikethrough: None,
        });
        self.text.push_str(&piece);
    }
}

fn font_for(flags: SpanFlags, base_weight: FontWeight) -> gpui::Font {
    let family = if flags.code { "Courier New" } else { "Inter" };
    let mut face = font(family);
    face.weight = if flags.strong {
        FontWeight(700.)
    } else {
        base_weight
    };
    if flags.emphasis {
        face.style = FontStyle::Italic;
    }
    face
}

fn color_for(flags: SpanFlags, theme: &M0Theme) -> gpui::Hsla {
    if flags.code || flags.link {
        theme.accent
    } else {
        theme.text_primary
    }
}

fn underline_for(flags: SpanFlags, theme: &M0Theme) -> Option<UnderlineStyle> {
    if !flags.link {
        return None;
    }
    Some(UnderlineStyle {
        thickness: px(1.),
        color: Some(theme.accent),
        wavy: false,
    })
}

fn with_wrap_breaks(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut word_start = 0;
    for (idx, ch) in text.char_indices() {
        if ch.is_whitespace() {
            append_word(&mut output, &text[word_start..idx]);
            output.push(ch);
            word_start = idx + ch.len_utf8();
        }
    }
    append_word(&mut output, &text[word_start..]);
    output
}

fn append_word(output: &mut String, word: &str) {
    for (i, chunk) in split_long_word(word, MAX_WORD_CHARS)
        .into_iter()
        .enumerate()
    {
        if i > 0 {
            output.push('\u{200B}');
        }
        output.push_str(chunk);
    }
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
    use crate::theme::M0Theme;

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

    #[test]
    fn styled_text_parts_should_cover_utf8_bytes_when_marks_present() {
        let theme = M0Theme::dark();
        let inlines = vec![
            Inline::Text("Hello ".to_string()),
            Inline::Strong(vec![Inline::Text("world".to_string())]),
            Inline::Text(" ".to_string()),
            Inline::Code("x".to_string()),
            Inline::Text(" ".to_string()),
            Inline::Emphasis(vec![Inline::Text("y".to_string())]),
            Inline::Text(" ".to_string()),
            Inline::Link {
                text: vec![Inline::Text("z".to_string())],
                href: "https://example.com".to_string(),
            },
        ];
        let (text, runs) = styled_text_parts(&inlines, &theme, FontWeight::NORMAL);
        assert_eq!(text, "Hello world x y z");
        assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), text.len());
        assert!(runs.iter().any(|run| run.underline.is_some()));
        assert!(runs.iter().any(|run| run.font.style == FontStyle::Italic));
        assert!(runs
            .iter()
            .any(|run| run.font.family.as_ref() == "Courier New"));
    }

    #[test]
    fn with_wrap_breaks_should_insert_zwsp_when_word_exceeds_max() {
        let word = "a".repeat(50);
        let broken = with_wrap_breaks(&word);
        assert!(broken.contains('\u{200B}'));
        assert_eq!(broken.chars().filter(|ch| *ch != '\u{200B}').count(), 50);
    }
}
