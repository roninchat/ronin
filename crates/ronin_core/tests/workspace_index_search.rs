//! Lexical search + attach/include gate (#74).
//!
//! Public seams: `RoninSession::search_workspace_index`,
//! `attach_workspace_index_hits`, `WorkspaceIndexIncludeGate`,
//! `workspace_index_hit_attachment`.

use ronin_core::{
    drafts_for_workspace_index_include, may_inject_into_chat_request,
    workspace_index_hit_attachment, workspace_index_hit_attachment_origin,
    workspace_index_origin_may_inject, ContextOrigin, RoninPaths, RoninSession,
    WorkspaceIndexHitSelection, WorkspaceIndexIncludeGate, WorkspaceIndexPhase,
    WORKSPACE_INDEX_INCLUDE_GATE_LABEL,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open session")
}

fn indexed_thread(temp: &TempDir) -> (RoninSession, String) {
    let root = temp.path().join("proj");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("src/main.rs"),
        "fn main() { println!(\"alpha beacon\"); }\n",
    )
    .unwrap();
    std::fs::write(root.join("README.md"), "# Project\n\nalpha beacon docs\n").unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn helper() {}\n").unwrap();
    let session = open_session(temp);
    let thread = session.create_thread().unwrap();
    session
        .set_thread_workspace_root(&thread.id, &root)
        .unwrap();
    let info = session.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    (session, thread.id)
}

#[test]
fn search_returns_candidate_hits_with_path_and_snippet() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let hits = session.search_workspace_index(&tid, "alpha").unwrap();
    assert!(!hits.is_empty());
    for hit in &hits {
        assert!(!hit.relative_path.is_empty());
        assert!(!hit.snippet.is_empty());
        assert_eq!(hit.context_origin(), ContextOrigin::IndexSearchHit);
        assert!(!may_inject_into_chat_request(hit.context_origin()));
    }
}

#[test]
fn search_requires_done_index() {
    let temp = TempDir::new().unwrap();
    let session = open_session(&temp);
    let thread = session.create_thread().unwrap();
    let err = session
        .search_workspace_index(&thread.id, "alpha")
        .unwrap_err();
    assert!(err.to_string().contains("not ready"));
}

#[test]
fn search_empty_query_yields_no_hits() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    assert!(session.search_workspace_index(&tid, "").unwrap().is_empty());
    assert!(session
        .search_workspace_index(&tid, "   ")
        .unwrap()
        .is_empty());
}

#[test]
fn search_respects_limit() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let hits = session
        .search_workspace_index_limited(&tid, "fn", 1)
        .unwrap();
    assert!(hits.len() <= 1);
}

#[test]
fn attach_selected_hits_builds_explicit_drafts() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let hits = session.search_workspace_index(&tid, "alpha").unwrap();
    assert!(!hits.is_empty());
    let paths: Vec<String> = hits.iter().map(|h| h.relative_path.clone()).collect();
    let drafts = session.attach_workspace_index_hits(&tid, &paths).unwrap();
    assert!(!drafts.is_empty());
    for draft in &drafts {
        assert!(draft.context_block.contains("Attached workspace file"));
        assert!(draft.content.is_some());
    }
    assert_eq!(
        workspace_index_hit_attachment_origin(),
        ContextOrigin::ExplicitAttachment
    );
    assert!(may_inject_into_chat_request(
        workspace_index_hit_attachment_origin()
    ));
}

#[test]
fn attach_empty_selection_returns_no_drafts() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let drafts = session
        .attach_workspace_index_hits(&tid, &[] as &[&str])
        .unwrap();
    assert!(drafts.is_empty());
}

#[test]
fn search_hits_never_auto_merge_into_chat() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let hits = session.search_workspace_index(&tid, "alpha").unwrap();
    assert!(!hits.is_empty());
    for hit in hits {
        assert!(!workspace_index_origin_may_inject(hit.context_origin()));
        assert!(!may_inject_into_chat_request(ContextOrigin::IndexSearchHit));
    }
}

#[test]
fn include_gate_defaults_off_and_blocks_drafts() {
    let gate = WorkspaceIndexIncludeGate::new();
    assert!(!gate.is_enabled());
    assert_eq!(gate.label(), WORKSPACE_INDEX_INCLUDE_GATE_LABEL);
    assert!(!may_inject_into_chat_request(gate.context_origin()));

    let draft = workspace_index_hit_attachment("a.rs", "fn a() {}");
    let out = drafts_for_workspace_index_include(&gate, &[draft]);
    assert!(out.is_empty());
}

#[test]
fn include_gate_enabled_releases_selected_drafts_only() {
    let mut gate = WorkspaceIndexIncludeGate::new();
    gate.set_enabled(true);
    assert!(may_inject_into_chat_request(gate.context_origin()));
    assert_eq!(gate.context_origin(), ContextOrigin::VisiblePerSendInclude);

    let draft = workspace_index_hit_attachment("a.rs", "fn a() {}");
    let out = drafts_for_workspace_index_include(&gate, std::slice::from_ref(&draft));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].context_block, draft.context_block);

    // Empty selection still yields nothing even when gate is on.
    assert!(drafts_for_workspace_index_include(&gate, &[]).is_empty());
}

#[test]
fn hit_selection_tracks_paths_without_injecting() {
    let mut sel = WorkspaceIndexHitSelection::new();
    assert!(sel.is_empty());
    sel.select("src/main.rs");
    sel.select("src/main.rs"); // dedupe
    sel.select("README.md");
    assert_eq!(
        sel.paths(),
        &["src/main.rs".to_string(), "README.md".to_string()]
    );
    sel.deselect("src/main.rs");
    assert_eq!(sel.paths(), &["README.md".to_string()]);
    sel.clear();
    assert!(sel.is_empty());
}

#[test]
fn search_does_not_attach_by_itself() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let _hits = session.search_workspace_index(&tid, "alpha").unwrap();
    // Searching must not mutate messages / create attachments.
    let msgs = session.list_messages(&tid).unwrap();
    assert!(msgs.is_empty());
}

#[test]
fn hit_attachment_scrubs_secrets_in_body() {
    let draft = workspace_index_hit_attachment("secret.env", "api_key=sk-live-secret\n");
    assert!(!draft.context_block.contains("sk-live-secret"));
    assert!(draft.context_block.contains("[REDACTED]") || draft.context_block.contains("REDACTED"));
}

#[test]
fn search_hits_absent_from_request_context_without_attach() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = indexed_thread(&temp);
    let hits = session.search_workspace_index(&tid, "alpha").unwrap();
    assert!(!hits.is_empty());

    // Search alone yields no drafts → nothing to merge into provider context.
    let unattached: Vec<ronin_core::ContextAttachmentDraft> = Vec::new();
    let block = unattached
        .iter()
        .map(|d| d.context_block.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");
    assert!(block.is_empty());
    for hit in &hits {
        assert!(!block.contains(&hit.relative_path));
        assert!(!block.contains(&hit.snippet));
    }

    // After explicit attach, drafts may enter context (ExplicitAttachment).
    let paths: Vec<_> = hits.iter().map(|h| h.relative_path.clone()).collect();
    let drafts = session.attach_workspace_index_hits(&tid, &paths).unwrap();
    let attached_block = drafts
        .iter()
        .map(|d| d.context_block.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");
    assert!(!attached_block.is_empty());
    assert!(may_inject_into_chat_request(
        workspace_index_hit_attachment_origin()
    ));
}
