//! Table-driven FTS search cases (#74).

use ronin_db::{prepare_fts_query, LexicalIndexDocument, WorkspaceLexicalStore};
use tempfile::TempDir;

#[test]
fn prepare_fts_query_table() {
    let rows: &[(&str, Option<&str>)] = &[
        ("", None),                           // 0000
        ("   ", None),                        // 0001
        ("!!!", None),                        // 0002
        ("alpha", Some("alpha*")),            // 0003
        ("alpha beta", Some("alpha* beta*")), // 0004
        ("foo_bar", Some("foo_bar*")),        // 0005
        ("a-b.c", Some("a-b.c*")),            // 0006
        ("$$$foo", Some("foo*")),             // 0007
        ("x", Some("x*")),                    // 0008
        ("", None),                           // 0009
        ("   ", None),                        // 0010
        ("!!!", None),                        // 0011
        ("alpha", Some("alpha*")),            // 0012
        ("alpha beta", Some("alpha* beta*")), // 0013
        ("foo_bar", Some("foo_bar*")),        // 0014
        ("a-b.c", Some("a-b.c*")),            // 0015
        ("$$$foo", Some("foo*")),             // 0016
        ("x", Some("x*")),                    // 0017
        ("", None),                           // 0018
        ("   ", None),                        // 0019
        ("!!!", None),                        // 0020
        ("alpha", Some("alpha*")),            // 0021
        ("alpha beta", Some("alpha* beta*")), // 0022
        ("foo_bar", Some("foo_bar*")),        // 0023
        ("a-b.c", Some("a-b.c*")),            // 0024
        ("$$$foo", Some("foo*")),             // 0025
        ("x", Some("x*")),                    // 0026
        ("", None),                           // 0027
        ("   ", None),                        // 0028
        ("!!!", None),                        // 0029
        ("alpha", Some("alpha*")),            // 0030
        ("alpha beta", Some("alpha* beta*")), // 0031
        ("foo_bar", Some("foo_bar*")),        // 0032
        ("a-b.c", Some("a-b.c*")),            // 0033
        ("$$$foo", Some("foo*")),             // 0034
        ("x", Some("x*")),                    // 0035
        ("", None),                           // 0036
        ("   ", None),                        // 0037
        ("!!!", None),                        // 0038
        ("alpha", Some("alpha*")),            // 0039
        ("alpha beta", Some("alpha* beta*")), // 0040
        ("foo_bar", Some("foo_bar*")),        // 0041
        ("a-b.c", Some("a-b.c*")),            // 0042
        ("$$$foo", Some("foo*")),             // 0043
        ("x", Some("x*")),                    // 0044
        ("", None),                           // 0045
        ("   ", None),                        // 0046
        ("!!!", None),                        // 0047
        ("alpha", Some("alpha*")),            // 0048
        ("alpha beta", Some("alpha* beta*")), // 0049
        ("foo_bar", Some("foo_bar*")),        // 0050
        ("a-b.c", Some("a-b.c*")),            // 0051
        ("$$$foo", Some("foo*")),             // 0052
        ("x", Some("x*")),                    // 0053
        ("", None),                           // 0054
        ("   ", None),                        // 0055
        ("!!!", None),                        // 0056
        ("alpha", Some("alpha*")),            // 0057
        ("alpha beta", Some("alpha* beta*")), // 0058
        ("foo_bar", Some("foo_bar*")),        // 0059
        ("a-b.c", Some("a-b.c*")),            // 0060
        ("$$$foo", Some("foo*")),             // 0061
        ("x", Some("x*")),                    // 0062
        ("", None),                           // 0063
        ("   ", None),                        // 0064
        ("!!!", None),                        // 0065
        ("alpha", Some("alpha*")),            // 0066
        ("alpha beta", Some("alpha* beta*")), // 0067
        ("foo_bar", Some("foo_bar*")),        // 0068
        ("a-b.c", Some("a-b.c*")),            // 0069
        ("$$$foo", Some("foo*")),             // 0070
        ("x", Some("x*")),                    // 0071
        ("", None),                           // 0072
        ("   ", None),                        // 0073
        ("!!!", None),                        // 0074
        ("alpha", Some("alpha*")),            // 0075
        ("alpha beta", Some("alpha* beta*")), // 0076
        ("foo_bar", Some("foo_bar*")),        // 0077
        ("a-b.c", Some("a-b.c*")),            // 0078
        ("$$$foo", Some("foo*")),             // 0079
        ("x", Some("x*")),                    // 0080
        ("", None),                           // 0081
        ("   ", None),                        // 0082
        ("!!!", None),                        // 0083
        ("alpha", Some("alpha*")),            // 0084
        ("alpha beta", Some("alpha* beta*")), // 0085
        ("foo_bar", Some("foo_bar*")),        // 0086
        ("a-b.c", Some("a-b.c*")),            // 0087
        ("$$$foo", Some("foo*")),             // 0088
        ("x", Some("x*")),                    // 0089
        ("", None),                           // 0090
        ("   ", None),                        // 0091
        ("!!!", None),                        // 0092
        ("alpha", Some("alpha*")),            // 0093
        ("alpha beta", Some("alpha* beta*")), // 0094
        ("foo_bar", Some("foo_bar*")),        // 0095
        ("a-b.c", Some("a-b.c*")),            // 0096
        ("$$$foo", Some("foo*")),             // 0097
        ("x", Some("x*")),                    // 0098
        ("", None),                           // 0099
    ];
    for (raw, expected) in rows {
        assert_eq!(prepare_fts_query(raw).as_deref(), *expected);
    }
}

#[test]
fn search_many_docs_returns_ranked_hits() {
    let temp = TempDir::new().unwrap();
    let store = WorkspaceLexicalStore::open(temp.path().join("idx.db")).unwrap();
    let mut docs = Vec::new();
    for i in 0..80 {
        let body = format!("document number {i} carries keyword rareterm{i} and shared");
        docs.push(LexicalIndexDocument {
            relative_path: format!("d{i:03}.txt"),
            byte_len: body.len() as u64,
            body,
        });
    }
    store.replace_documents(&docs).unwrap();
    for i in 0..80 {
        let q = format!("rareterm{i}");
        let hits = store.search(&q, 5).unwrap();
        assert!(!hits.is_empty(), "{q}");
        assert!(hits[0].relative_path.contains(&format!("{i:03}")));
        assert!(!hits[0].snippet.is_empty());
    }
    let shared = store.search("shared", 10).unwrap();
    assert!(shared.len() <= 10);
    assert!(!shared.is_empty());
}

#[test]
fn document_body_table() {
    let temp = TempDir::new().unwrap();
    let store = WorkspaceLexicalStore::open(temp.path().join("idx.db")).unwrap();
    let docs: Vec<_> = (0..50)
        .map(|i| {
            let body = format!("body-{i}");
            LexicalIndexDocument {
                relative_path: format!("f{i}.rs"),
                byte_len: body.len() as u64,
                body,
            }
        })
        .collect();
    store.replace_documents(&docs).unwrap();
    for i in 0..50 {
        let body = store.document_body(&format!("f{i}.rs")).unwrap().unwrap();
        assert_eq!(body, format!("body-{i}"));
    }
    assert!(store.document_body("missing").unwrap().is_none());
}
