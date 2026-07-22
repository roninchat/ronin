//! Lexical FTS search on workspace index store (#74).

use ronin_db::{LexicalIndexDocument, WorkspaceLexicalStore};
use tempfile::TempDir;

fn seed_store(path: &std::path::Path) -> WorkspaceLexicalStore {
    let store = WorkspaceLexicalStore::open(path).unwrap();
    store
        .replace_documents(&[
            LexicalIndexDocument {
                relative_path: "src/main.rs".into(),
                body: "fn main() {\n    println!(\"alpha beacon\");\n}\n".into(),
                byte_len: 44,
            },
            LexicalIndexDocument {
                relative_path: "src/lib.rs".into(),
                body: "pub fn helper() { /* unused */ }\n".into(),
                byte_len: 33,
            },
            LexicalIndexDocument {
                relative_path: "README.md".into(),
                body: "# Project\n\nFind the alpha beacon here.\n".into(),
                byte_len: 39,
            },
        ])
        .unwrap();
    store
}

#[test]
fn search_returns_path_and_snippet_for_matching_docs() {
    let temp = TempDir::new().unwrap();
    let store = seed_store(&temp.path().join("idx.db"));
    let hits = store.search("alpha", 10).unwrap();
    assert!(hits.len() >= 2);
    for hit in &hits {
        assert!(!hit.relative_path.is_empty());
        assert!(!hit.snippet.is_empty());
        assert!(
            hit.snippet.to_ascii_lowercase().contains("alpha")
                || hit.relative_path.contains("main")
                || hit.relative_path.contains("README")
        );
    }
    let paths: Vec<_> = hits.iter().map(|h| h.relative_path.as_str()).collect();
    assert!(paths.contains(&"src/main.rs") || paths.contains(&"README.md"));
}

#[test]
fn search_empty_query_returns_no_hits() {
    let temp = TempDir::new().unwrap();
    let store = seed_store(&temp.path().join("idx.db"));
    assert!(store.search("", 10).unwrap().is_empty());
    assert!(store.search("   ", 10).unwrap().is_empty());
}

#[test]
fn search_respects_limit() {
    let temp = TempDir::new().unwrap();
    let store = seed_store(&temp.path().join("idx.db"));
    let hits = store.search("fn", 1).unwrap();
    assert!(hits.len() <= 1);
}

#[test]
fn search_no_match_returns_empty() {
    let temp = TempDir::new().unwrap();
    let store = seed_store(&temp.path().join("idx.db"));
    assert!(store.search("zzzznotfound999", 10).unwrap().is_empty());
}

#[test]
fn document_body_returns_stored_text() {
    let temp = TempDir::new().unwrap();
    let store = seed_store(&temp.path().join("idx.db"));
    let body = store.document_body("src/main.rs").unwrap().unwrap();
    assert!(body.contains("alpha beacon"));
    assert!(store.document_body("missing.rs").unwrap().is_none());
}
