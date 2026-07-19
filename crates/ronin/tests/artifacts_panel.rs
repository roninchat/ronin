//! Artifacts panel presentation: preview cards, empty state, edit/delete flow.

use ronin::artifacts_panel::{
    artifact_kind_badge, artifact_preview_card, artifacts_empty_state, content_snippet,
    save_code_block_as_snippet_label, snippet_title_from_language, ArtifactsPanelState,
    ARTIFACTS_EMPTY_STATE, ARTIFACT_KIND_BADGE, ARTIFACT_SNIPPET_CHARS, SNIPPET_KIND_BADGE,
};
use ronin::syntax_highlight::highlight_code;
use ronin_core::{Artifact, ArtifactId, ColorScheme};

fn sample_artifact(title: &str, content: &str, thread_id: &str) -> Artifact {
    Artifact {
        id: ArtifactId("art-1".into()),
        thread_id: thread_id.into(),
        message_id: "msg-1".into(),
        title: title.into(),
        content: content.into(),
        kind: "document".into(),
        language: None,
        created_at: 1,
    }
}

fn sample_snippet(title: &str, content: &str, language: &str) -> Artifact {
    Artifact {
        id: ArtifactId("snip-1".into()),
        thread_id: "thread-1".into(),
        message_id: "msg-1".into(),
        title: title.into(),
        content: content.into(),
        kind: "snippet".into(),
        language: Some(language.into()),
        created_at: 1,
    }
}

#[test]
fn preview_card_should_include_title_kind_snippet_and_source_thread() {
    let artifact = sample_artifact(
        "Refactor helpers",
        "fn helper() { /* long body */ }",
        "thread-42",
    );

    let card = artifact_preview_card(&artifact, "Cleanup chat");

    assert_eq!(card.id, "art-1");
    assert_eq!(card.title, "Refactor helpers");
    assert_eq!(card.kind, ARTIFACT_KIND_BADGE);
    assert_eq!(card.snippet, "fn helper() { /* long body */ }");
    assert_eq!(card.source_thread_id, "thread-42");
    assert_eq!(card.source_thread_title, "Cleanup chat");
}

#[test]
fn preview_card_snippet_should_truncate_long_content() {
    let long: String = "a".repeat(ARTIFACT_SNIPPET_CHARS + 20);
    let artifact = sample_artifact("Long", &long, "t1");
    let card = artifact_preview_card(&artifact, "Thread");

    assert_eq!(card.snippet.chars().count(), ARTIFACT_SNIPPET_CHARS + 1); // + ellipsis
    assert!(card.snippet.ends_with('…'));
    assert_eq!(
        content_snippet(&long, ARTIFACT_SNIPPET_CHARS),
        card.snippet
    );
}

#[test]
fn empty_state_should_be_clear_when_no_artifacts() {
    assert_eq!(
        artifacts_empty_state(&[]),
        Some(ARTIFACTS_EMPTY_STATE)
    );
    assert!(ARTIFACTS_EMPTY_STATE.contains("No artifacts"));
}

#[test]
fn empty_state_should_be_none_when_artifacts_exist() {
    let artifact = sample_artifact("A", "b", "t");
    let card = artifact_preview_card(&artifact, "T");
    assert_eq!(artifacts_empty_state(&[card]), None);
}

#[test]
fn delete_should_require_confirmation_before_returning_id() {
    let mut state = ArtifactsPanelState::default();
    assert_eq!(state.pending_delete_id(), None);

    state.request_delete("art-1");
    assert_eq!(state.pending_delete_id(), Some("art-1"));

    state.cancel_delete();
    assert_eq!(state.pending_delete_id(), None);
    assert_eq!(state.confirm_delete(), None);

    state.request_delete("art-2");
    assert_eq!(state.confirm_delete(), Some("art-2".into()));
    assert_eq!(state.pending_delete_id(), None);
}

#[test]
fn edit_flow_should_expose_draft_then_commit_for_rename_and_content() {
    let mut state = ArtifactsPanelState::default();
    state.begin_edit("art-1", "Old title", "old content");

    let draft = state.editing().expect("editing");
    assert_eq!(draft.id, "art-1");
    assert_eq!(draft.title, "Old title");
    assert_eq!(draft.content, "old content");

    state.set_edit_title("New title");
    state.set_edit_content("new content");

    let committed = state.commit_edit().expect("commit");
    assert_eq!(committed.title, "New title");
    assert_eq!(committed.content, "new content");
    assert!(state.editing().is_none());
}

#[test]
fn cancel_edit_should_discard_draft_without_commit() {
    let mut state = ArtifactsPanelState::default();
    state.begin_edit("art-1", "T", "C");
    state.cancel_edit();
    assert!(state.editing().is_none());
    assert!(state.commit_edit().is_none());
}

#[test]
fn snippet_preview_card_should_show_language_badge() {
    let artifact = sample_snippet("helpers", "fn helper() {}", "rust");
    let card = artifact_preview_card(&artifact, "Cleanup chat");

    assert_eq!(card.kind, SNIPPET_KIND_BADGE);
    assert_eq!(card.language.as_deref(), Some("rust"));
    assert_eq!(artifact_kind_badge(&artifact), "rust");
    assert_eq!(save_code_block_as_snippet_label(), "Save snippet");
    assert_eq!(snippet_title_from_language(Some("rust")), "rust snippet");
}

#[test]
fn document_preview_card_should_keep_generic_artifact_badge() {
    let artifact = sample_artifact("Note", "hello", "t1");
    let card = artifact_preview_card(&artifact, "Thread");
    assert_eq!(card.kind, ARTIFACT_KIND_BADGE);
    assert_eq!(card.language, None);
    assert_eq!(artifact_kind_badge(&artifact), ARTIFACT_KIND_BADGE);
}

#[test]
fn snippet_artifact_should_highlight_with_language_metadata() {
    let artifact = sample_snippet("main", "fn main() { let x = 1; }", "rust");
    assert!(artifact.is_snippet());
    let lines = highlight_code(artifact.language.as_deref(), &artifact.content, ColorScheme::Dark);
    assert!(!lines.is_empty());
    // Syntect should produce more than a single plain span for Rust keywords.
    let span_count: usize = lines.iter().map(|l| l.spans.len()).sum();
    assert!(
        span_count > 1,
        "expected syntax-colored spans for rust snippet, got {span_count}"
    );
}
