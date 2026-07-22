//! Public seams for global search across threads, artifacts, and memories.

use ronin::global_search::{
    group_hits_by_kind, matches_filters, search, SearchContentKind, SearchDatePreset,
    SearchDocument, SearchFilters, SearchHit, SearchPanelState,
};

fn doc(
    kind: SearchContentKind,
    id: &str,
    title: &str,
    body: &str,
    created_at: i64,
) -> SearchDocument {
    SearchDocument {
        kind,
        id: id.into(),
        title: title.into(),
        body: body.into(),
        thread_id: None,
        message_id: None,
        provider: None,
        model: None,
        created_at,
    }
}

#[test]
fn search_should_match_thread_titles_and_message_bodies() {
    let docs = vec![
        SearchDocument {
            kind: SearchContentKind::Thread,
            id: "t1".into(),
            title: "Rust tips".into(),
            body: "hello world".into(),
            thread_id: Some("t1".into()),
            message_id: Some("m1".into()),
            provider: Some("ollama".into()),
            model: Some("llama3.2".into()),
            created_at: 100,
        },
        SearchDocument {
            kind: SearchContentKind::Thread,
            id: "t2".into(),
            title: "Other".into(),
            body: "unrelated".into(),
            thread_id: Some("t2".into()),
            message_id: Some("m2".into()),
            provider: Some("openai".into()),
            model: Some("gpt-4o".into()),
            created_at: 200,
        },
    ];
    let hits = search("rust", &docs, &SearchFilters::default());
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.id, "t1");
    assert!(hits[0].snippet.to_lowercase().contains("rust"));

    let hits = search("HELLO", &docs, &SearchFilters::default());
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.message_id.as_deref(), Some("m1"));
}

#[test]
fn search_should_match_artifacts_and_memories() {
    let docs = vec![
        doc(
            SearchContentKind::Artifact,
            "a1",
            "API sketch",
            "POST /v1/chat",
            1,
        ),
        doc(
            SearchContentKind::Memory,
            "mem1",
            "Prefs",
            "User prefers tea over coffee",
            2,
        ),
    ];
    let art = search("api", &docs, &SearchFilters::default());
    assert_eq!(art.len(), 1);
    assert_eq!(art[0].document.kind, SearchContentKind::Artifact);

    let mem = search("coffee", &docs, &SearchFilters::default());
    assert_eq!(mem.len(), 1);
    assert_eq!(mem[0].document.kind, SearchContentKind::Memory);
}

#[test]
fn search_should_group_hits_by_type_with_labels() {
    let docs = vec![
        doc(SearchContentKind::Thread, "t", "T", "x", 1),
        doc(SearchContentKind::Artifact, "a", "A", "x", 2),
        doc(SearchContentKind::Memory, "m", "M", "x", 3),
    ];
    let hits = search("x", &docs, &SearchFilters::default());
    let grouped = group_hits_by_kind(&hits);
    assert_eq!(grouped.len(), 3);
    assert_eq!(grouped[0].0.label(), "Threads");
    assert_eq!(grouped[1].0.label(), "Artifacts");
    assert_eq!(grouped[2].0.label(), "Memories");
}

#[test]
fn filters_should_narrow_by_provider_model_date_and_kind() {
    let docs = vec![
        SearchDocument {
            kind: SearchContentKind::Thread,
            id: "t1".into(),
            title: "A".into(),
            body: "needle".into(),
            thread_id: Some("t1".into()),
            message_id: None,
            provider: Some("ollama".into()),
            model: Some("llama3.2".into()),
            created_at: 1_000,
        },
        SearchDocument {
            kind: SearchContentKind::Thread,
            id: "t2".into(),
            title: "B".into(),
            body: "needle".into(),
            thread_id: Some("t2".into()),
            message_id: None,
            provider: Some("openai".into()),
            model: Some("gpt-4o".into()),
            created_at: 5_000,
        },
        SearchDocument {
            kind: SearchContentKind::Memory,
            id: "m1".into(),
            title: "C".into(),
            body: "needle".into(),
            thread_id: None,
            message_id: None,
            provider: None,
            model: None,
            created_at: 3_000,
        },
    ];

    let filters = SearchFilters {
        provider: Some("ollama".into()),
        ..Default::default()
    };
    let hits = search("needle", &docs, &filters);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.id, "t1");

    let filters = SearchFilters {
        model: Some("gpt-4o".into()),
        ..Default::default()
    };
    let hits = search("needle", &docs, &filters);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.id, "t2");

    let filters = SearchFilters {
        created_after_ms: Some(2_000),
        created_before_ms: Some(4_000),
        ..Default::default()
    };
    let hits = search("needle", &docs, &filters);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.id, "m1");

    let filters = SearchFilters {
        kinds: vec![SearchContentKind::Memory],
        ..Default::default()
    };
    let hits = search("needle", &docs, &filters);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].document.kind, SearchContentKind::Memory);
}

#[test]
fn empty_query_should_yield_no_hits() {
    let docs = vec![doc(SearchContentKind::Thread, "t", "Hi", "there", 1)];
    assert!(search("", &docs, &SearchFilters::default()).is_empty());
    assert!(search("   ", &docs, &SearchFilters::default()).is_empty());
}

#[test]
fn matches_filters_should_respect_defaults() {
    let d = doc(SearchContentKind::Artifact, "a", "t", "b", 50);
    assert!(matches_filters(&d, &SearchFilters::default()));
}

#[test]
fn search_panel_state_should_toggle_and_track_query() {
    let mut panel = SearchPanelState::default();
    assert!(!panel.is_open());
    panel.open();
    assert!(panel.is_open());
    panel.set_query("rust");
    assert_eq!(panel.query(), "rust");
    panel.close();
    assert!(!panel.is_open());
}

#[test]
fn title_matches_should_rank_above_body_only_matches() {
    let docs = vec![
        doc(
            SearchContentKind::Thread,
            "body",
            "Other",
            "zzz rust zzz",
            1,
        ),
        doc(SearchContentKind::Thread, "title", "Rust guide", "hello", 2),
    ];
    let hits = search("rust", &docs, &SearchFilters::default());
    assert_eq!(hits[0].document.id, "title");
    assert!(hits[0].score > hits[1].score);
}

#[test]
fn search_hit_should_expose_navigation_targets() {
    let hit = SearchHit {
        document: SearchDocument {
            kind: SearchContentKind::Thread,
            id: "hit".into(),
            title: "T".into(),
            body: "msg".into(),
            thread_id: Some("thr".into()),
            message_id: Some("msg-1".into()),
            provider: None,
            model: None,
            created_at: 0,
        },
        snippet: "msg".into(),
        score: 1,
    };
    assert_eq!(hit.document.thread_id.as_deref(), Some("thr"));
    assert_eq!(hit.document.message_id.as_deref(), Some("msg-1"));
    assert_eq!(hit.document.kind, SearchContentKind::Thread);
}

#[test]
fn date_preset_should_set_created_after_bounds() {
    let mut panel = SearchPanelState::default();
    let now = 1_000_000_000_i64;
    panel.set_date_preset(SearchDatePreset::Last7Days, now);
    assert_eq!(panel.date_preset(), SearchDatePreset::Last7Days);
    assert_eq!(panel.filters().created_after_ms, Some(now - 7 * 86_400_000));
    panel.set_date_preset(SearchDatePreset::Any, now);
    assert!(panel.filters().created_after_ms.is_none());
}
