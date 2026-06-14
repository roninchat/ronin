/// Represents a parsed Markdown block.
#[derive(Debug, PartialEq, Clone)]
pub enum MarkdownBlock {
    /// A paragraph of text.
    Paragraph(Vec<Inline>),
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
}

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

/// Parses a Markdown string into a sequence of blocks.
pub fn parse_markdown(text: &str) -> Vec<MarkdownBlock> {
    let parser = Parser::new(text);
    let mut blocks = Vec::new();

    let mut current_inlines = Vec::new();
    let mut in_code_block = false;
    let mut current_language = None;
    let mut current_code = String::new();

    let mut in_list_item = false;
    let mut current_list_items = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::List(_)) => {
                current_list_items.clear();
            }
            Event::Start(Tag::Item) => {
                in_list_item = true;
                current_inlines.clear();
            }
            Event::Start(Tag::Paragraph) => {
                if !in_list_item {
                    current_inlines.clear();
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                current_code.clear();
                current_language = match kind {
                    CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.into_string()),
                    _ => None,
                };
            }
            Event::Text(text) => {
                if in_code_block {
                    current_code.push_str(&text);
                } else {
                    current_inlines.push(Inline::Text(text.into_string()));
                }
            }
            Event::Code(code) => {
                current_inlines.push(Inline::Code(code.into_string()));
            }
            Event::End(TagEnd::Paragraph) => {
                if !in_list_item {
                    blocks.push(MarkdownBlock::Paragraph(std::mem::take(
                        &mut current_inlines,
                    )));
                }
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
                    inlines: std::mem::take(&mut current_inlines),
                });
                in_list_item = false;
            }
            Event::End(TagEnd::List(_)) => {
                blocks.push(MarkdownBlock::List(std::mem::take(&mut current_list_items)));
            }
            _ => {}
        }
    }

    blocks
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
}
