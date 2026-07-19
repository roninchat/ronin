//! Syntax highlighting for fenced Markdown code blocks.

use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use ronin_core::ColorScheme;

/// A colored span of source text within a highlighted line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightedSpan {
    /// Source text for this span.
    pub text: String,
    /// Foreground RGB color.
    pub rgb: (u8, u8, u8),
}

/// One line of highlighted source, as ordered spans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightedLine {
    /// Colored spans that reconstruct the line (without trailing newline).
    pub spans: Vec<HighlightedSpan>,
}

/// Highlights `content` for an optional fenced-language identifier.
///
/// When `language` is `None` or empty, returns plain monospaced lines using the
/// scheme's default foreground (no syntect tokenization).
///
/// Highlighting is line-oriented and safe to call repeatedly during streaming:
/// incomplete or unknown languages fall back to plain text rather than failing.
pub fn highlight_code(
    language: Option<&str>,
    content: &str,
    scheme: ColorScheme,
) -> Vec<HighlightedLine> {
    let default_fg = default_foreground(scheme);
    let Some(lang) = language.map(str::trim).filter(|l| !l.is_empty()) else {
        return plain_lines(content, default_fg);
    };

    match highlight_with_syntect(lang, content, scheme) {
        Some(lines) => lines,
        None => plain_lines(content, default_fg),
    }
}

fn default_foreground(scheme: ColorScheme) -> (u8, u8, u8) {
    match scheme {
        ColorScheme::Dark => (0xcd, 0xd6, 0xf4),
        ColorScheme::Light => (0x4c, 0x4f, 0x69),
    }
}

fn plain_lines(content: &str, rgb: (u8, u8, u8)) -> Vec<HighlightedLine> {
    // Preserve empty trailing line behavior of split — match UI split('\n')
    content
        .split('\n')
        .map(|line| HighlightedLine {
            spans: vec![HighlightedSpan {
                text: line.to_string(),
                rgb,
            }],
        })
        .collect()
}

fn highlight_with_syntect(
    language: &str,
    content: &str,
    scheme: ColorScheme,
) -> Option<Vec<HighlightedLine>> {
    let ps = syntax_set();
    let syntax = resolve_syntax(ps, language)?;
    let theme = theme_for_scheme(scheme);
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut lines = Vec::new();
    for line in LinesWithEndings::from(content) {
        let ranges = highlighter.highlight_line(line, ps).ok()?;
        let mut spans = Vec::new();
        for (style, text) in ranges {
            let text = text.trim_end_matches('\n').trim_end_matches('\r');
            if text.is_empty() && spans.is_empty() {
                // Keep empty lines as a single empty span.
                continue;
            }
            if text.is_empty() {
                continue;
            }
            spans.push(HighlightedSpan {
                text: text.to_string(),
                rgb: (style.foreground.r, style.foreground.g, style.foreground.b),
            });
        }
        if spans.is_empty() {
            spans.push(HighlightedSpan {
                text: String::new(),
                rgb: default_foreground(scheme),
            });
        }
        lines.push(HighlightedLine { spans });
    }

    // LinesWithEndings drops a final empty line when content ends with \n.
    // Match plain_lines / UI split('\n') which yields a trailing empty segment.
    if content.ends_with('\n') {
        lines.push(HighlightedLine {
            spans: vec![HighlightedSpan {
                text: String::new(),
                rgb: default_foreground(scheme),
            }],
        });
    }

    Some(lines)
}

fn resolve_syntax<'a>(
    ps: &'a SyntaxSet,
    language: &str,
) -> Option<&'a syntect::parsing::SyntaxReference> {
    let key = language.to_ascii_lowercase();
    let mapped = match key.as_str() {
        "rs" => "Rust",
        "py" | "python" => "Python",
        "js" | "javascript" | "jsx" => "JavaScript",
        "ts" | "typescript" | "tsx" => "TypeScript",
        "go" | "golang" => "Go",
        "c" => "C",
        "cpp" | "c++" | "cc" | "cxx" => "C++",
        "sh" | "bash" | "shell" | "zsh" => "Bash",
        "json" => "JSON",
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "sql" => "SQL",
        "md" | "markdown" => "Markdown",
        other => other,
    };

    ps.find_syntax_by_name(mapped)
        .or_else(|| ps.find_syntax_by_extension(mapped))
        .or_else(|| ps.find_syntax_by_token(&key))
        .or_else(|| ps.find_syntax_by_extension(&key))
        .filter(|syntax| syntax.name != "Plain Text")
}

fn theme_for_scheme(scheme: ColorScheme) -> &'static Theme {
    let themes = theme_set();
    match scheme {
        ColorScheme::Dark => &themes.themes["base16-ocean.dark"],
        ColorScheme::Light => &themes.themes["base16-ocean.light"],
    }
}

fn syntax_set() -> &'static SyntaxSet {
    use std::sync::OnceLock;
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme_set() -> &'static ThemeSet {
    use std::sync::OnceLock;
    static SET: OnceLock<ThemeSet> = OnceLock::new();
    SET.get_or_init(ThemeSet::load_defaults)
}
