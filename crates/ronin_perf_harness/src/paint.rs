//! Chat Paint Path measurement and driver traits.

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::error::HarnessError;
use crate::scenario::ScenarioId;
use crate::timing::{PaintTiming, SpanTiming};

/// Drives (or simulates) the Chat Paint Path for a scenario.
pub trait ChatPaintDriver {
    /// Runs the paint path and returns Paint Timing.
    fn run_chat_paint_path(
        &mut self,
        scenario: ScenarioId<'_>,
    ) -> Result<PaintTiming, HarnessError>;
}

/// Minimal OS-level Drive Smoke (focus + one operable interaction).
pub trait DriveSmoke {
    /// Proves the painted window is operable; fail closed when required.
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError>;
}

/// Parsed markdown block (mirrors Ronin chat paint inputs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownBlock {
    /// Paragraph text.
    Paragraph(String),
    /// Fenced code.
    CodeBlock {
        /// Language tag.
        language: Option<String>,
        /// Body.
        content: String,
    },
    /// List items joined.
    List(Vec<String>),
}

/// Parses markdown into blocks used by Chat Paint Path measurement.
pub fn parse_markdown_blocks(text: &str) -> Vec<MarkdownBlock> {
    let parser = Parser::new(text);
    let mut blocks = Vec::new();
    let mut current_para = String::new();
    let mut in_code = false;
    let mut lang = None;
    let mut code = String::new();
    let mut in_item = false;
    let mut item_text = String::new();
    let mut list_items = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {
                if !in_item {
                    current_para.clear();
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code = true;
                code.clear();
                lang = match kind {
                    CodeBlockKind::Fenced(l) if !l.is_empty() => Some(l.into_string()),
                    _ => None,
                };
            }
            Event::Start(Tag::Item) => {
                in_item = true;
                item_text.clear();
            }
            Event::Text(t) => {
                if in_code {
                    code.push_str(&t);
                } else if in_item {
                    item_text.push_str(&t);
                } else {
                    current_para.push_str(&t);
                }
            }
            Event::Code(c) => {
                if in_item {
                    item_text.push_str(&c);
                } else {
                    current_para.push_str(&c);
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if !in_item && !current_para.is_empty() {
                    blocks.push(MarkdownBlock::Paragraph(std::mem::take(&mut current_para)));
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                blocks.push(MarkdownBlock::CodeBlock {
                    language: lang.take(),
                    content: std::mem::take(&mut code),
                });
                in_code = false;
            }
            Event::End(TagEnd::Item) => {
                list_items.push(std::mem::take(&mut item_text));
                in_item = false;
            }
            Event::End(TagEnd::List(_)) => {
                blocks.push(MarkdownBlock::List(std::mem::take(&mut list_items)));
            }
            _ => {}
        }
    }
    blocks
}

/// Measures Chat Paint Path cost for a list of message bodies (parse + highlight/render).
pub fn measure_chat_paint(messages: &[String]) -> PaintTiming {
    let wall_start = Instant::now();

    let parse_start = Instant::now();
    let mut all_blocks = Vec::new();
    for msg in messages {
        all_blocks.push(parse_markdown_blocks(msg));
    }
    let parse = parse_start.elapsed();

    let render_start = Instant::now();
    let (ps, theme) = highlight_assets();
    for blocks in &all_blocks {
        for block in blocks {
            if let MarkdownBlock::CodeBlock { language, content } = block {
                if let Some(lang) = language.as_deref().map(str::trim).filter(|l| !l.is_empty()) {
                    if let Some(syntax) = ps.find_syntax_by_token(lang) {
                        let mut highlighter = HighlightLines::new(syntax, theme);
                        for line in LinesWithEndings::from(content) {
                            let _ = highlighter.highlight_line(line, ps);
                        }
                    }
                }
            }
        }
    }
    let render = render_start.elapsed();
    let wall = wall_start.elapsed();

    PaintTiming {
        parse,
        render,
        wall,
        spans: vec![
            SpanTiming {
                name: "markdown.parse".into(),
                duration: parse,
            },
            SpanTiming {
                name: "chat.render".into(),
                duration: render,
            },
        ],
    }
}

fn highlight_assets() -> (&'static SyntaxSet, &'static Theme) {
    static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
    static THEMES: OnceLock<ThemeSet> = OnceLock::new();
    let ps = SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = THEMES.get_or_init(ThemeSet::load_defaults);
    let theme = ts
        .themes
        .get("base16-ocean.dark")
        .expect("syntect default theme base16-ocean.dark");
    (ps, theme)
}

/// Drive Smoke that passes when a display is available (minimal operable-environment check).
///
/// v1 checks that a graphical session exists. Focus + key/click against a live
/// Harness Build window is the next Drive Smoke slice once control-plane wiring lands.
pub struct DisplayDriveSmoke;

impl DriveSmoke for DisplayDriveSmoke {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        let has_display =
            std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some();
        if has_display {
            Ok(())
        } else {
            Err(HarnessError::DriveSmokeFailed(
                "no WAYLAND_DISPLAY/DISPLAY; window not operable in this environment".into(),
            ))
        }
    }
}

/// Always-ok smoke for unit tests of judgment logic.
pub struct AlwaysOkSmoke;

impl DriveSmoke for AlwaysOkSmoke {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        Ok(())
    }
}

/// Rounds timing up to whole milliseconds for stable ms-based reports.
///
/// Sub-millisecond work would otherwise serialize/`as_millis()` as `0`, which
/// makes hotspots look empty on tiny goldens like `plain_short`.
pub fn ceil_to_millis(d: Duration) -> Duration {
    if d.is_zero() {
        return Duration::from_millis(1);
    }
    let nanos = d.as_nanos();
    let ceil_ms = nanos.div_ceil(1_000_000);
    Duration::from_millis(ceil_ms as u64)
}

#[cfg(test)]
mod ceil_tests {
    use super::*;

    #[test]
    fn sub_millisecond_ceils_to_one_ms() {
        assert_eq!(
            ceil_to_millis(Duration::from_micros(400)),
            Duration::from_millis(1)
        );
    }

    #[test]
    fn zero_ceils_to_one_ms_for_oracle_stability() {
        assert_eq!(ceil_to_millis(Duration::ZERO), Duration::from_millis(1));
    }

    #[test]
    fn whole_millis_unchanged() {
        assert_eq!(
            ceil_to_millis(Duration::from_millis(8)),
            Duration::from_millis(8)
        );
    }
}
