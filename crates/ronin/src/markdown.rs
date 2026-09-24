/// Represents a parsed Markdown block.
#[derive(Debug, PartialEq, Clone)]
pub enum MarkdownBlock {
    /// A paragraph of text.
    Paragraph(Vec<Inline>),
    /// An ATX heading. Levels 4–6 are stored as 3.
    Heading {
        /// Heading level from 1 to 3.
        level: u8,
        /// Inline content of the heading.
        inlines: Vec<Inline>,
    },
    /// A fenced code block.
    CodeBlock {
        /// Optional language label.
        language: Option<String>,
        /// The raw code content.
        content: String,
    },
    /// A bulleted list.
    List(Vec<ListItem>),
}

/// Represents a list item.
#[derive(Debug, PartialEq, Clone)]
pub struct ListItem {
    /// Inlines within the list item.
    pub inlines: Vec<Inline>,
}

/// Represents an inline Markdown element.
#[derive(Debug, PartialEq, Clone)]
pub enum Inline {
    /// Plain text.
    Text(String),
    /// Inline code segment.
    Code(String),
    /// Strong (bold) span. Nested inlines keep inner marks.
    Strong(Vec<Inline>),
    /// Emphasis (italic) span. Nested inlines keep inner marks.
    Emphasis(Vec<Inline>),
    /// A hyperlink. Nested inlines are the visible text.
    Link {
        /// Visible link text.
        text: Vec<Inline>,
        /// Destination URL.
        href: String,
    },
}

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

/// Parses a Markdown string into a sequence of blocks.
pub fn parse_markdown(text: &str) -> Vec<MarkdownBlock> {
    let parser = Parser::new(text);
    let mut blocks = Vec::new();

    let mut inline_stack: Vec<Vec<Inline>> = vec![Vec::new()];
    let mut link_hrefs: Vec<String> = Vec::new();
    let mut in_code_block = false;
    let mut current_language = None;
    let mut current_code = String::new();

    let mut in_list_item = false;
    let mut current_list_items = Vec::new();
    let mut heading_level: Option<u8> = None;

    for event in parser {
        match event {
            Event::Start(Tag::List(_)) => {
                current_list_items.clear();
            }
            Event::Start(Tag::Item) => {
                in_list_item = true;
                reset_inlines(&mut inline_stack);
            }
            Event::Start(Tag::Paragraph) => {
                if !in_list_item {
                    reset_inlines(&mut inline_stack);
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                reset_inlines(&mut inline_stack);
                heading_level = Some(clamp_heading_level(level));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                current_code.clear();
                current_language = match kind {
                    CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.into_string()),
                    _ => None,
                };
            }
            Event::Start(Tag::Strong) => {
                inline_stack.push(Vec::new());
            }
            Event::Start(Tag::Emphasis) => {
                inline_stack.push(Vec::new());
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                link_hrefs.push(dest_url.into_string());
                inline_stack.push(Vec::new());
            }
            Event::Text(text) => {
                if in_code_block {
                    current_code.push_str(&text);
                } else {
                    push_inline(&mut inline_stack, Inline::Text(text.into_string()));
                }
            }
            Event::Code(code) => {
                push_inline(&mut inline_stack, Inline::Code(code.into_string()));
            }
            Event::End(TagEnd::Paragraph) => {
                if !in_list_item {
                    blocks.push(MarkdownBlock::Paragraph(take_root_inlines(
                        &mut inline_stack,
                    )));
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                blocks.push(MarkdownBlock::Heading {
                    level: heading_level.take().unwrap_or(1),
                    inlines: take_root_inlines(&mut inline_stack),
                });
            }
            Event::End(TagEnd::CodeBlock) => {
                blocks.push(MarkdownBlock::CodeBlock {
                    language: current_language.take(),
                    content: std::mem::take(&mut current_code),
                });
                in_code_block = false;
            }
            Event::End(TagEnd::Item) => {
                current_list_items.push(ListItem {
                    inlines: take_root_inlines(&mut inline_stack),
                });
                in_list_item = false;
            }
            Event::End(TagEnd::List(_)) => {
                blocks.push(MarkdownBlock::List(std::mem::take(&mut current_list_items)));
            }
            Event::End(TagEnd::Strong) => {
                let nested = pop_nested_inlines(&mut inline_stack);
                push_inline(&mut inline_stack, Inline::Strong(nested));
            }
            Event::End(TagEnd::Emphasis) => {
                let nested = pop_nested_inlines(&mut inline_stack);
                push_inline(&mut inline_stack, Inline::Emphasis(nested));
            }
            Event::End(TagEnd::Link) => {
                let nested = pop_nested_inlines(&mut inline_stack);
                let href = link_hrefs.pop().unwrap_or_default();
                push_inline(&mut inline_stack, Inline::Link { text: nested, href });
            }
            _ => {}
        }
    }

    blocks
}

fn clamp_heading_level(level: HeadingLevel) -> u8 {
    (level as u8).min(3)
}

fn reset_inlines(stack: &mut Vec<Vec<Inline>>) {
    stack.clear();
    stack.push(Vec::new());
}

fn push_inline(stack: &mut Vec<Vec<Inline>>, inline: Inline) {
    if let Some(current) = stack.last_mut() {
        current.push(inline);
        return;
    }
    stack.push(vec![inline]);
}

fn pop_nested_inlines(stack: &mut Vec<Vec<Inline>>) -> Vec<Inline> {
    if stack.len() <= 1 {
        return Vec::new();
    }
    stack.pop().unwrap_or_default()
}

fn take_root_inlines(stack: &mut Vec<Vec<Inline>>) -> Vec<Inline> {
    while stack.len() > 1 {
        let nested = stack.pop().unwrap_or_default();
        if let Some(parent) = stack.last_mut() {
            parent.extend(nested);
        }
    }
    let inlines = stack.pop().unwrap_or_default();
    stack.push(Vec::new());
    inlines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_paragraph() {
        let text = "Hello world";
        let blocks = parse_markdown(text);
        assert_eq!(
            blocks,
            vec![MarkdownBlock::Paragraph(vec![Inline::Text(
                "Hello world".to_string()
            )])]
        );
    }

    #[test]
    fn parse_paragraph_with_inline_code() {
        let text = "Here is `some code` inline.";
        let blocks = parse_markdown(text);
        assert_eq!(
            blocks,
            vec![MarkdownBlock::Paragraph(vec![
                Inline::Text("Here is ".to_string()),
                Inline::Code("some code".to_string()),
                Inline::Text(" inline.".to_string()),
            ])]
        );
    }

    #[test]
    fn parse_fenced_code_block() {
        let text = "```rust\nfn main() {}\n```";
        let blocks = parse_markdown(text);
        assert_eq!(
            blocks,
            vec![MarkdownBlock::CodeBlock {
                language: Some("rust".to_string()),
                content: "fn main() {}\n".to_string(),
            }]
        );
    }

    #[test]
    fn parse_bullet_list() {
        let text = "- Item 1\n- Item 2";
        let blocks = parse_markdown(text);
        assert_eq!(
            blocks,
            vec![MarkdownBlock::List(vec![
                ListItem {
                    inlines: vec![Inline::Text("Item 1".to_string())],
                },
                ListItem {
                    inlines: vec![Inline::Text("Item 2".to_string())],
                },
            ])]
        );
    }

    #[test]
    fn parse_markdown_should_emit_heading_when_atx() {
        let cases = [
            ("# Title", 1, "Title"),
            ("## Sub", 2, "Sub"),
            ("### Small", 3, "Small"),
        ];
        for (src, level, title) in cases {
            assert_eq!(
                parse_markdown(src),
                vec![MarkdownBlock::Heading {
                    level,
                    inlines: vec![Inline::Text(title.to_string())],
                }],
                "{src}"
            );
        }
    }

    #[test]
    fn parse_markdown_should_clamp_heading_level_when_deeper_than_three() {
        let blocks = parse_markdown("#### Deep");
        assert_eq!(
            blocks,
            vec![MarkdownBlock::Heading {
                level: 3,
                inlines: vec![Inline::Text("Deep".to_string())],
            }]
        );
    }

    #[test]
    fn parse_markdown_should_emit_strong_and_emphasis_when_marked() {
        let blocks = parse_markdown("**bold** and *italic*");
        assert_eq!(
            blocks,
            vec![MarkdownBlock::Paragraph(vec![
                Inline::Strong(vec![Inline::Text("bold".to_string())]),
                Inline::Text(" and ".to_string()),
                Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
            ])]
        );
    }

    #[test]
    fn parse_markdown_should_emit_link_when_inline_link() {
        let blocks = parse_markdown("[links](url)");
        assert_eq!(
            blocks,
            vec![MarkdownBlock::Paragraph(vec![Inline::Link {
                text: vec![Inline::Text("links".to_string())],
                href: "url".to_string(),
            }])]
        );
    }
}
