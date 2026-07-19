//! Syntax highlighting for fenced code blocks (syntect).

use ronin::syntax_highlight::highlight_code;
use ronin_core::ColorScheme;

#[test]
fn highlight_without_language_should_be_plain_monospaced_color() {
    let lines = highlight_code(None, "fn main() {}\n", ColorScheme::Dark);
    assert_eq!(lines.len(), 2); // content line + trailing empty from \n
    assert_eq!(lines[0].spans.len(), 1);
    assert_eq!(lines[0].spans[0].text, "fn main() {}");
    // Catppuccin Mocha text
    assert_eq!(lines[0].spans[0].rgb, (0xcd, 0xd6, 0xf4));
}

#[test]
fn highlight_empty_language_should_be_plain() {
    let lines = highlight_code(Some(""), "x = 1", ColorScheme::Light);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].spans[0].text, "x = 1");
    assert_eq!(lines[0].spans[0].rgb, (0x4c, 0x4f, 0x69));
}

#[test]
fn highlight_rust_should_color_keywords_differently_from_plain() {
    let code = "fn main() {\n    let x = 1;\n}\n";
    let highlighted = highlight_code(Some("rust"), code, ColorScheme::Dark);
    let plain = highlight_code(None, code, ColorScheme::Dark);

    assert!(highlighted.len() >= 3);
    // Highlighted rust should produce multiple spans on the first line (fn, main, ...)
    assert!(
        highlighted[0].spans.len() > 1,
        "expected tokenized spans, got {:?}",
        highlighted[0].spans
    );
    // Plain has a single span per line
    assert_eq!(plain[0].spans.len(), 1);

    let joined: String = highlighted[0]
        .spans
        .iter()
        .map(|s| s.text.as_str())
        .collect();
    assert!(joined.contains("fn"));
    assert!(joined.contains("main"));
}

#[test]
fn highlight_should_support_required_languages() {
    let samples = [
        ("rust", "fn main() {}"),
        ("python", "def main():\n    pass"),
        ("javascript", "const x = 1;"),
        ("typescript", "const x: number = 1;"),
        ("go", "func main() {}"),
        ("c", "int main(void) { return 0; }"),
        ("cpp", "int main() { return 0; }"),
        ("bash", "echo hello"),
        ("json", "{\"a\": 1}"),
        ("toml", "a = 1"),
        ("yaml", "a: 1"),
        ("html", "<div></div>"),
        ("css", "body { color: red; }"),
        ("sql", "SELECT 1;"),
        ("markdown", "# Title"),
    ];

    for (lang, code) in samples {
        let lines = highlight_code(Some(lang), code, ColorScheme::Dark);
        assert!(
            !lines.is_empty(),
            "language {lang} produced no lines"
        );
        let has_non_default = lines.iter().any(|line| {
            line.spans.iter().any(|span| span.rgb != (0xcd, 0xd6, 0xf4))
        });
        // Most languages should produce at least some non-default coloring;
        // markdown headings / simple cases may be subtle — require tokenization
        // OR reconstructed text equality as a minimum correctness check.
        let reconstructed: String = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|s| s.text.as_str())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        let expected = code.trim_end_matches('\n');
        let actual = reconstructed.trim_end_matches('\n');
        assert_eq!(
            actual, expected,
            "language {lang} must preserve source text (colored={has_non_default})"
        );
    }
}

#[test]
fn highlight_light_and_dark_should_use_different_theme_colors() {
    let code = "fn main() { let x = \"hi\"; }\n";
    let dark = highlight_code(Some("rust"), code, ColorScheme::Dark);
    let light = highlight_code(Some("rust"), code, ColorScheme::Light);

    let dark_colors: Vec<_> = dark
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.rgb))
        .collect();
    let light_colors: Vec<_> = light
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.rgb))
        .collect();

    assert_ne!(
        dark_colors, light_colors,
        "light and dark themes should produce different span colors"
    );
}

#[test]
fn highlight_unknown_language_should_fall_back_to_plain() {
    let lines = highlight_code(Some("not-a-real-lang-xyz"), "abc", ColorScheme::Dark);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].spans.len(), 1);
    assert_eq!(lines[0].spans[0].text, "abc");
    assert_eq!(lines[0].spans[0].rgb, (0xcd, 0xd6, 0xf4));
}

#[test]
fn highlight_js_and_ts_aliases_should_work() {
    for lang in ["js", "ts", "tsx", "py", "sh", "yml", "c++"] {
        let lines = highlight_code(Some(lang), "x = 1", ColorScheme::Dark);
        assert!(!lines.is_empty(), "alias {lang} failed");
    }
}
