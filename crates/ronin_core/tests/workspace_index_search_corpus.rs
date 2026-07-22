//! Corpus coverage for lexical search attach gate (#74).

use ronin_core::{
    drafts_for_workspace_index_include, may_inject_into_chat_request,
    workspace_index_origin_may_inject, ContextOrigin, RoninPaths, RoninSession,
    WorkspaceIndexHitSelection, WorkspaceIndexIncludeGate, WorkspaceIndexPhase,
};
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open")
}

fn build_corpus(temp: &TempDir, n_files: usize) -> (RoninSession, String) {
    let root = temp.path().join("corpus");
    std::fs::create_dir_all(root.join("src")).unwrap();
    for i in 0..n_files {
        std::fs::write(
            root.join("src").join(format!("f{i:03}.rs")),
            format!("pub fn marker_{i:03}_unique() {{}}\n"),
        )
        .unwrap();
    }
    std::fs::write(root.join("README.md"), "# corpus shared term\n").unwrap();
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
fn search_finds_unique_markers_across_corpus() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 40);
    for i in 0..40 {
        let q = format!("marker_{i:03}_unique");
        let hits = session.search_workspace_index(&tid, &q).unwrap();
        assert!(!hits.is_empty(), "expected hit for {q}");
        assert!(
            hits.iter()
                .any(|h| h.relative_path.contains(&format!("f{i:03}"))),
            "path missing for {q}"
        );
        for hit in &hits {
            assert!(!may_inject_into_chat_request(hit.context_origin()));
        }
    }
}

#[test]
fn attach_each_unique_marker_path() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 30);
    for i in 0..30 {
        let path = format!("src/f{i:03}.rs");
        let drafts = session
            .attach_workspace_index_hits(&tid, std::slice::from_ref(&path))
            .unwrap();
        assert_eq!(drafts.len(), 1);
        assert!(drafts[0].context_block.contains(&path));
        assert!(drafts[0]
            .context_block
            .contains(&format!("marker_{i:03}_unique")));
    }
}

#[test]
fn search_then_include_gate_stays_off_until_enabled() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 8);
    let hits = session.search_workspace_index(&tid, "shared").unwrap();
    assert!(!hits.is_empty());
    let paths: Vec<_> = hits.iter().map(|h| h.relative_path.clone()).collect();
    let drafts = session.attach_workspace_index_hits(&tid, &paths).unwrap();
    let mut gate = WorkspaceIndexIncludeGate::new();
    assert!(drafts_for_workspace_index_include(&gate, &drafts).is_empty());
    gate.set_enabled(true);
    let released = drafts_for_workspace_index_include(&gate, &drafts);
    assert_eq!(released.len(), drafts.len());
}

#[test]
fn origin_inject_matrix_for_search_pipeline() {
    let rows: &[(ContextOrigin, bool)] = &[
        (ContextOrigin::IndexSearchHit, false),         // 0000
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0001
        (ContextOrigin::ExplicitAttachment, true),      // 0002
        (ContextOrigin::VisiblePerSendInclude, true),   // 0003
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0004
        (ContextOrigin::ComposerText, true),            // 0005
        (ContextOrigin::EnabledProfileMemory, true),    // 0006
        (ContextOrigin::ClipboardWatchProposal, false), // 0007
        (ContextOrigin::NotificationPayload, false),    // 0008
        (ContextOrigin::AmbientDesktopEvent, false),    // 0009
        (ContextOrigin::IndexSearchHit, false),         // 0010
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0011
        (ContextOrigin::ExplicitAttachment, true),      // 0012
        (ContextOrigin::VisiblePerSendInclude, true),   // 0013
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0014
        (ContextOrigin::ComposerText, true),            // 0015
        (ContextOrigin::EnabledProfileMemory, true),    // 0016
        (ContextOrigin::ClipboardWatchProposal, false), // 0017
        (ContextOrigin::NotificationPayload, false),    // 0018
        (ContextOrigin::AmbientDesktopEvent, false),    // 0019
        (ContextOrigin::IndexSearchHit, false),         // 0020
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0021
        (ContextOrigin::ExplicitAttachment, true),      // 0022
        (ContextOrigin::VisiblePerSendInclude, true),   // 0023
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0024
        (ContextOrigin::ComposerText, true),            // 0025
        (ContextOrigin::EnabledProfileMemory, true),    // 0026
        (ContextOrigin::ClipboardWatchProposal, false), // 0027
        (ContextOrigin::NotificationPayload, false),    // 0028
        (ContextOrigin::AmbientDesktopEvent, false),    // 0029
        (ContextOrigin::IndexSearchHit, false),         // 0030
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0031
        (ContextOrigin::ExplicitAttachment, true),      // 0032
        (ContextOrigin::VisiblePerSendInclude, true),   // 0033
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0034
        (ContextOrigin::ComposerText, true),            // 0035
        (ContextOrigin::EnabledProfileMemory, true),    // 0036
        (ContextOrigin::ClipboardWatchProposal, false), // 0037
        (ContextOrigin::NotificationPayload, false),    // 0038
        (ContextOrigin::AmbientDesktopEvent, false),    // 0039
        (ContextOrigin::IndexSearchHit, false),         // 0040
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0041
        (ContextOrigin::ExplicitAttachment, true),      // 0042
        (ContextOrigin::VisiblePerSendInclude, true),   // 0043
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0044
        (ContextOrigin::ComposerText, true),            // 0045
        (ContextOrigin::EnabledProfileMemory, true),    // 0046
        (ContextOrigin::ClipboardWatchProposal, false), // 0047
        (ContextOrigin::NotificationPayload, false),    // 0048
        (ContextOrigin::AmbientDesktopEvent, false),    // 0049
        (ContextOrigin::IndexSearchHit, false),         // 0050
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0051
        (ContextOrigin::ExplicitAttachment, true),      // 0052
        (ContextOrigin::VisiblePerSendInclude, true),   // 0053
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0054
        (ContextOrigin::ComposerText, true),            // 0055
        (ContextOrigin::EnabledProfileMemory, true),    // 0056
        (ContextOrigin::ClipboardWatchProposal, false), // 0057
        (ContextOrigin::NotificationPayload, false),    // 0058
        (ContextOrigin::AmbientDesktopEvent, false),    // 0059
        (ContextOrigin::IndexSearchHit, false),         // 0060
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0061
        (ContextOrigin::ExplicitAttachment, true),      // 0062
        (ContextOrigin::VisiblePerSendInclude, true),   // 0063
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0064
        (ContextOrigin::ComposerText, true),            // 0065
        (ContextOrigin::EnabledProfileMemory, true),    // 0066
        (ContextOrigin::ClipboardWatchProposal, false), // 0067
        (ContextOrigin::NotificationPayload, false),    // 0068
        (ContextOrigin::AmbientDesktopEvent, false),    // 0069
        (ContextOrigin::IndexSearchHit, false),         // 0070
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0071
        (ContextOrigin::ExplicitAttachment, true),      // 0072
        (ContextOrigin::VisiblePerSendInclude, true),   // 0073
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0074
        (ContextOrigin::ComposerText, true),            // 0075
        (ContextOrigin::EnabledProfileMemory, true),    // 0076
        (ContextOrigin::ClipboardWatchProposal, false), // 0077
        (ContextOrigin::NotificationPayload, false),    // 0078
        (ContextOrigin::AmbientDesktopEvent, false),    // 0079
        (ContextOrigin::IndexSearchHit, false),         // 0080
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0081
        (ContextOrigin::ExplicitAttachment, true),      // 0082
        (ContextOrigin::VisiblePerSendInclude, true),   // 0083
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0084
        (ContextOrigin::ComposerText, true),            // 0085
        (ContextOrigin::EnabledProfileMemory, true),    // 0086
        (ContextOrigin::ClipboardWatchProposal, false), // 0087
        (ContextOrigin::NotificationPayload, false),    // 0088
        (ContextOrigin::AmbientDesktopEvent, false),    // 0089
        (ContextOrigin::IndexSearchHit, false),         // 0090
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0091
        (ContextOrigin::ExplicitAttachment, true),      // 0092
        (ContextOrigin::VisiblePerSendInclude, true),   // 0093
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0094
        (ContextOrigin::ComposerText, true),            // 0095
        (ContextOrigin::EnabledProfileMemory, true),    // 0096
        (ContextOrigin::ClipboardWatchProposal, false), // 0097
        (ContextOrigin::NotificationPayload, false),    // 0098
        (ContextOrigin::AmbientDesktopEvent, false),    // 0099
        (ContextOrigin::IndexSearchHit, false),         // 0100
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0101
        (ContextOrigin::ExplicitAttachment, true),      // 0102
        (ContextOrigin::VisiblePerSendInclude, true),   // 0103
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0104
        (ContextOrigin::ComposerText, true),            // 0105
        (ContextOrigin::EnabledProfileMemory, true),    // 0106
        (ContextOrigin::ClipboardWatchProposal, false), // 0107
        (ContextOrigin::NotificationPayload, false),    // 0108
        (ContextOrigin::AmbientDesktopEvent, false),    // 0109
        (ContextOrigin::IndexSearchHit, false),         // 0110
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0111
        (ContextOrigin::ExplicitAttachment, true),      // 0112
        (ContextOrigin::VisiblePerSendInclude, true),   // 0113
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0114
        (ContextOrigin::ComposerText, true),            // 0115
        (ContextOrigin::EnabledProfileMemory, true),    // 0116
        (ContextOrigin::ClipboardWatchProposal, false), // 0117
        (ContextOrigin::NotificationPayload, false),    // 0118
        (ContextOrigin::AmbientDesktopEvent, false),    // 0119
        (ContextOrigin::IndexSearchHit, false),         // 0120
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0121
        (ContextOrigin::ExplicitAttachment, true),      // 0122
        (ContextOrigin::VisiblePerSendInclude, true),   // 0123
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0124
        (ContextOrigin::ComposerText, true),            // 0125
        (ContextOrigin::EnabledProfileMemory, true),    // 0126
        (ContextOrigin::ClipboardWatchProposal, false), // 0127
        (ContextOrigin::NotificationPayload, false),    // 0128
        (ContextOrigin::AmbientDesktopEvent, false),    // 0129
        (ContextOrigin::IndexSearchHit, false),         // 0130
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0131
        (ContextOrigin::ExplicitAttachment, true),      // 0132
        (ContextOrigin::VisiblePerSendInclude, true),   // 0133
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0134
        (ContextOrigin::ComposerText, true),            // 0135
        (ContextOrigin::EnabledProfileMemory, true),    // 0136
        (ContextOrigin::ClipboardWatchProposal, false), // 0137
        (ContextOrigin::NotificationPayload, false),    // 0138
        (ContextOrigin::AmbientDesktopEvent, false),    // 0139
        (ContextOrigin::IndexSearchHit, false),         // 0140
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0141
        (ContextOrigin::ExplicitAttachment, true),      // 0142
        (ContextOrigin::VisiblePerSendInclude, true),   // 0143
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0144
        (ContextOrigin::ComposerText, true),            // 0145
        (ContextOrigin::EnabledProfileMemory, true),    // 0146
        (ContextOrigin::ClipboardWatchProposal, false), // 0147
        (ContextOrigin::NotificationPayload, false),    // 0148
        (ContextOrigin::AmbientDesktopEvent, false),    // 0149
        (ContextOrigin::IndexSearchHit, false),         // 0150
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0151
        (ContextOrigin::ExplicitAttachment, true),      // 0152
        (ContextOrigin::VisiblePerSendInclude, true),   // 0153
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0154
        (ContextOrigin::ComposerText, true),            // 0155
        (ContextOrigin::EnabledProfileMemory, true),    // 0156
        (ContextOrigin::ClipboardWatchProposal, false), // 0157
        (ContextOrigin::NotificationPayload, false),    // 0158
        (ContextOrigin::AmbientDesktopEvent, false),    // 0159
        (ContextOrigin::IndexSearchHit, false),         // 0160
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0161
        (ContextOrigin::ExplicitAttachment, true),      // 0162
        (ContextOrigin::VisiblePerSendInclude, true),   // 0163
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0164
        (ContextOrigin::ComposerText, true),            // 0165
        (ContextOrigin::EnabledProfileMemory, true),    // 0166
        (ContextOrigin::ClipboardWatchProposal, false), // 0167
        (ContextOrigin::NotificationPayload, false),    // 0168
        (ContextOrigin::AmbientDesktopEvent, false),    // 0169
        (ContextOrigin::IndexSearchHit, false),         // 0170
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0171
        (ContextOrigin::ExplicitAttachment, true),      // 0172
        (ContextOrigin::VisiblePerSendInclude, true),   // 0173
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0174
        (ContextOrigin::ComposerText, true),            // 0175
        (ContextOrigin::EnabledProfileMemory, true),    // 0176
        (ContextOrigin::ClipboardWatchProposal, false), // 0177
        (ContextOrigin::NotificationPayload, false),    // 0178
        (ContextOrigin::AmbientDesktopEvent, false),    // 0179
        (ContextOrigin::IndexSearchHit, false),         // 0180
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0181
        (ContextOrigin::ExplicitAttachment, true),      // 0182
        (ContextOrigin::VisiblePerSendInclude, true),   // 0183
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0184
        (ContextOrigin::ComposerText, true),            // 0185
        (ContextOrigin::EnabledProfileMemory, true),    // 0186
        (ContextOrigin::ClipboardWatchProposal, false), // 0187
        (ContextOrigin::NotificationPayload, false),    // 0188
        (ContextOrigin::AmbientDesktopEvent, false),    // 0189
        (ContextOrigin::IndexSearchHit, false),         // 0190
        (ContextOrigin::WorkspaceIndexCorpus, false),   // 0191
        (ContextOrigin::ExplicitAttachment, true),      // 0192
        (ContextOrigin::VisiblePerSendInclude, true),   // 0193
        (ContextOrigin::ConfirmToAttachAccepted, true), // 0194
        (ContextOrigin::ComposerText, true),            // 0195
        (ContextOrigin::EnabledProfileMemory, true),    // 0196
        (ContextOrigin::ClipboardWatchProposal, false), // 0197
        (ContextOrigin::NotificationPayload, false),    // 0198
        (ContextOrigin::AmbientDesktopEvent, false),    // 0199
    ];
    for (origin, expect) in rows {
        assert_eq!(workspace_index_origin_may_inject(*origin), *expect);
        assert_eq!(may_inject_into_chat_request(*origin), *expect);
    }
}

#[test]
fn selection_attach_round_trip_labels() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 12);
    let labels: &[&str] = &[
        "src/f000.rs",
        "src/f001.rs",
        "src/f002.rs",
        "src/f003.rs",
        "src/f004.rs",
        "src/f005.rs",
        "src/f006.rs",
        "src/f007.rs",
        "src/f008.rs",
        "src/f009.rs",
        "src/f010.rs",
        "src/f011.rs",
    ];
    let mut sel = WorkspaceIndexHitSelection::new();
    for path in labels {
        sel.select(*path);
    }
    assert_eq!(sel.paths().len(), 12);
    let drafts = session
        .attach_workspace_index_hits(&tid, sel.paths())
        .unwrap();
    assert_eq!(drafts.len(), 12);
    for (path, draft) in labels.iter().zip(drafts.iter()) {
        assert!(draft.context_block.contains(path));
    }
}

#[test]
fn attach_skips_unknown_paths() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 3);
    let paths: Vec<String> = vec![
        "src/f000.rs".into(),
        "no/such/file.rs".into(),
        "src/f001.rs".into(),
    ];
    let drafts = session.attach_workspace_index_hits(&tid, &paths).unwrap();
    assert_eq!(drafts.len(), 2);
}

#[test]
fn limited_search_never_exceeds_cap() {
    let temp = TempDir::new().unwrap();
    let (session, tid) = build_corpus(&temp, 25);
    for limit in [1usize, 2, 3, 5, 8, 13] {
        let hits = session
            .search_workspace_index_limited(&tid, "marker", limit)
            .unwrap();
        assert!(hits.len() <= limit);
        for hit in hits {
            assert!(!hit.snippet.is_empty() || !hit.relative_path.is_empty());
            assert!(!may_inject_into_chat_request(hit.context_origin()));
        }
    }
}
