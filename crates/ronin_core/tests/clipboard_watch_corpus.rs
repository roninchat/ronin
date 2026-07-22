//! Dense corpus for clipboard watch silent-context + lifecycle (#77).

use ronin_core::{
    clipboard_attachment, clipboard_watch_proposal_may_inject_into_chat_request,
    clipboard_watch_proposal_origin, confirmed_clipboard_attach_may_inject_into_chat_request,
    confirmed_clipboard_attach_origin, may_inject_into_chat_request, parse_context_tools,
    proposal_preview, ClipboardObserveOutcome, ClipboardWatchController, ContextOrigin,
    ScriptedClipboardSource, AMBIENT_REDACTED,
};

#[test]
fn corpus_proposals_never_inject_across_payloads() {
    let payloads: &[&str] = &[
        "corpus clipboard payload 000 with token=secret000 and api_key=sk-000",
        "corpus clipboard payload 001 with token=secret001 and api_key=sk-001",
        "corpus clipboard payload 002 with token=secret002 and api_key=sk-002",
        "corpus clipboard payload 003 with token=secret003 and api_key=sk-003",
        "corpus clipboard payload 004 with token=secret004 and api_key=sk-004",
        "corpus clipboard payload 005 with token=secret005 and api_key=sk-005",
        "corpus clipboard payload 006 with token=secret006 and api_key=sk-006",
        "corpus clipboard payload 007 with token=secret007 and api_key=sk-007",
        "corpus clipboard payload 008 with token=secret008 and api_key=sk-008",
        "corpus clipboard payload 009 with token=secret009 and api_key=sk-009",
        "corpus clipboard payload 010 with token=secret010 and api_key=sk-010",
        "corpus clipboard payload 011 with token=secret011 and api_key=sk-011",
        "corpus clipboard payload 012 with token=secret012 and api_key=sk-012",
        "corpus clipboard payload 013 with token=secret013 and api_key=sk-013",
        "corpus clipboard payload 014 with token=secret014 and api_key=sk-014",
        "corpus clipboard payload 015 with token=secret015 and api_key=sk-015",
        "corpus clipboard payload 016 with token=secret016 and api_key=sk-016",
        "corpus clipboard payload 017 with token=secret017 and api_key=sk-017",
        "corpus clipboard payload 018 with token=secret018 and api_key=sk-018",
        "corpus clipboard payload 019 with token=secret019 and api_key=sk-019",
        "corpus clipboard payload 020 with token=secret020 and api_key=sk-020",
        "corpus clipboard payload 021 with token=secret021 and api_key=sk-021",
        "corpus clipboard payload 022 with token=secret022 and api_key=sk-022",
        "corpus clipboard payload 023 with token=secret023 and api_key=sk-023",
        "corpus clipboard payload 024 with token=secret024 and api_key=sk-024",
        "corpus clipboard payload 025 with token=secret025 and api_key=sk-025",
        "corpus clipboard payload 026 with token=secret026 and api_key=sk-026",
        "corpus clipboard payload 027 with token=secret027 and api_key=sk-027",
        "corpus clipboard payload 028 with token=secret028 and api_key=sk-028",
        "corpus clipboard payload 029 with token=secret029 and api_key=sk-029",
        "corpus clipboard payload 030 with token=secret030 and api_key=sk-030",
        "corpus clipboard payload 031 with token=secret031 and api_key=sk-031",
        "corpus clipboard payload 032 with token=secret032 and api_key=sk-032",
        "corpus clipboard payload 033 with token=secret033 and api_key=sk-033",
        "corpus clipboard payload 034 with token=secret034 and api_key=sk-034",
        "corpus clipboard payload 035 with token=secret035 and api_key=sk-035",
        "corpus clipboard payload 036 with token=secret036 and api_key=sk-036",
        "corpus clipboard payload 037 with token=secret037 and api_key=sk-037",
        "corpus clipboard payload 038 with token=secret038 and api_key=sk-038",
        "corpus clipboard payload 039 with token=secret039 and api_key=sk-039",
        "corpus clipboard payload 040 with token=secret040 and api_key=sk-040",
        "corpus clipboard payload 041 with token=secret041 and api_key=sk-041",
        "corpus clipboard payload 042 with token=secret042 and api_key=sk-042",
        "corpus clipboard payload 043 with token=secret043 and api_key=sk-043",
        "corpus clipboard payload 044 with token=secret044 and api_key=sk-044",
        "corpus clipboard payload 045 with token=secret045 and api_key=sk-045",
        "corpus clipboard payload 046 with token=secret046 and api_key=sk-046",
        "corpus clipboard payload 047 with token=secret047 and api_key=sk-047",
        "corpus clipboard payload 048 with token=secret048 and api_key=sk-048",
        "corpus clipboard payload 049 with token=secret049 and api_key=sk-049",
        "corpus clipboard payload 050 with token=secret050 and api_key=sk-050",
        "corpus clipboard payload 051 with token=secret051 and api_key=sk-051",
        "corpus clipboard payload 052 with token=secret052 and api_key=sk-052",
        "corpus clipboard payload 053 with token=secret053 and api_key=sk-053",
        "corpus clipboard payload 054 with token=secret054 and api_key=sk-054",
        "corpus clipboard payload 055 with token=secret055 and api_key=sk-055",
        "corpus clipboard payload 056 with token=secret056 and api_key=sk-056",
        "corpus clipboard payload 057 with token=secret057 and api_key=sk-057",
        "corpus clipboard payload 058 with token=secret058 and api_key=sk-058",
        "corpus clipboard payload 059 with token=secret059 and api_key=sk-059",
        "corpus clipboard payload 060 with token=secret060 and api_key=sk-060",
        "corpus clipboard payload 061 with token=secret061 and api_key=sk-061",
        "corpus clipboard payload 062 with token=secret062 and api_key=sk-062",
        "corpus clipboard payload 063 with token=secret063 and api_key=sk-063",
        "corpus clipboard payload 064 with token=secret064 and api_key=sk-064",
        "corpus clipboard payload 065 with token=secret065 and api_key=sk-065",
        "corpus clipboard payload 066 with token=secret066 and api_key=sk-066",
        "corpus clipboard payload 067 with token=secret067 and api_key=sk-067",
        "corpus clipboard payload 068 with token=secret068 and api_key=sk-068",
        "corpus clipboard payload 069 with token=secret069 and api_key=sk-069",
        "corpus clipboard payload 070 with token=secret070 and api_key=sk-070",
        "corpus clipboard payload 071 with token=secret071 and api_key=sk-071",
        "corpus clipboard payload 072 with token=secret072 and api_key=sk-072",
        "corpus clipboard payload 073 with token=secret073 and api_key=sk-073",
        "corpus clipboard payload 074 with token=secret074 and api_key=sk-074",
        "corpus clipboard payload 075 with token=secret075 and api_key=sk-075",
        "corpus clipboard payload 076 with token=secret076 and api_key=sk-076",
        "corpus clipboard payload 077 with token=secret077 and api_key=sk-077",
        "corpus clipboard payload 078 with token=secret078 and api_key=sk-078",
        "corpus clipboard payload 079 with token=secret079 and api_key=sk-079",
        "corpus clipboard payload 080 with token=secret080 and api_key=sk-080",
        "corpus clipboard payload 081 with token=secret081 and api_key=sk-081",
        "corpus clipboard payload 082 with token=secret082 and api_key=sk-082",
        "corpus clipboard payload 083 with token=secret083 and api_key=sk-083",
        "corpus clipboard payload 084 with token=secret084 and api_key=sk-084",
        "corpus clipboard payload 085 with token=secret085 and api_key=sk-085",
        "corpus clipboard payload 086 with token=secret086 and api_key=sk-086",
        "corpus clipboard payload 087 with token=secret087 and api_key=sk-087",
        "corpus clipboard payload 088 with token=secret088 and api_key=sk-088",
        "corpus clipboard payload 089 with token=secret089 and api_key=sk-089",
        "corpus clipboard payload 090 with token=secret090 and api_key=sk-090",
        "corpus clipboard payload 091 with token=secret091 and api_key=sk-091",
        "corpus clipboard payload 092 with token=secret092 and api_key=sk-092",
        "corpus clipboard payload 093 with token=secret093 and api_key=sk-093",
        "corpus clipboard payload 094 with token=secret094 and api_key=sk-094",
        "corpus clipboard payload 095 with token=secret095 and api_key=sk-095",
        "corpus clipboard payload 096 with token=secret096 and api_key=sk-096",
        "corpus clipboard payload 097 with token=secret097 and api_key=sk-097",
        "corpus clipboard payload 098 with token=secret098 and api_key=sk-098",
        "corpus clipboard payload 099 with token=secret099 and api_key=sk-099",
        "corpus clipboard payload 100 with token=secret100 and api_key=sk-100",
        "corpus clipboard payload 101 with token=secret101 and api_key=sk-101",
        "corpus clipboard payload 102 with token=secret102 and api_key=sk-102",
        "corpus clipboard payload 103 with token=secret103 and api_key=sk-103",
        "corpus clipboard payload 104 with token=secret104 and api_key=sk-104",
        "corpus clipboard payload 105 with token=secret105 and api_key=sk-105",
        "corpus clipboard payload 106 with token=secret106 and api_key=sk-106",
        "corpus clipboard payload 107 with token=secret107 and api_key=sk-107",
        "corpus clipboard payload 108 with token=secret108 and api_key=sk-108",
        "corpus clipboard payload 109 with token=secret109 and api_key=sk-109",
        "corpus clipboard payload 110 with token=secret110 and api_key=sk-110",
        "corpus clipboard payload 111 with token=secret111 and api_key=sk-111",
        "corpus clipboard payload 112 with token=secret112 and api_key=sk-112",
        "corpus clipboard payload 113 with token=secret113 and api_key=sk-113",
        "corpus clipboard payload 114 with token=secret114 and api_key=sk-114",
        "corpus clipboard payload 115 with token=secret115 and api_key=sk-115",
        "corpus clipboard payload 116 with token=secret116 and api_key=sk-116",
        "corpus clipboard payload 117 with token=secret117 and api_key=sk-117",
        "corpus clipboard payload 118 with token=secret118 and api_key=sk-118",
        "corpus clipboard payload 119 with token=secret119 and api_key=sk-119",
    ];
    for (i, text) in payloads.iter().enumerate() {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("baseline"));
        assert_eq!(watch.observe_text(text), ClipboardObserveOutcome::Proposed);
        let proposal = watch.pending_proposal().expect("proposal");
        assert_eq!(proposal.text, *text);
        assert!(!proposal.preview.contains(&format!("secret{i:03}")));
        assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
        assert!(!may_inject_into_chat_request(
            clipboard_watch_proposal_origin()
        ));
        assert!(!may_inject_into_chat_request(
            ContextOrigin::ClipboardWatchProposal
        ));
        assert_eq!(
            clipboard_watch_proposal_origin(),
            ContextOrigin::ClipboardWatchProposal
        );
    }
}

#[test]
fn corpus_confirm_path_uses_confirm_origin() {
    for i in 0..100usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("b"));
        let text = format!("confirm-corpus-{i}");
        watch.observe_text(&text);
        let draft = watch.confirm_pending().expect("draft");
        assert_eq!(draft.content.as_deref(), Some(text.as_str()));
        assert!(confirmed_clipboard_attach_may_inject_into_chat_request());
        assert!(may_inject_into_chat_request(
            confirmed_clipboard_attach_origin()
        ));
        assert_eq!(
            confirmed_clipboard_attach_origin(),
            ContextOrigin::ConfirmToAttachAccepted
        );
        let explicit = clipboard_attachment(&text);
        assert_eq!(explicit.context_block, draft.context_block);
    }
}

#[test]
fn corpus_ondemand_clipboard_refs_independent_of_watch() {
    for i in 0..80usize {
        let mut watch = ClipboardWatchController::new();
        assert!(!watch.is_enabled());
        let msg = format!("please use @clipboard item {i}");
        let parsed = parse_context_tools(&msg);
        assert!(
            parsed
                .refs
                .iter()
                .any(|r| matches!(r, ronin_core::ContextToolRef::Clipboard)),
            "missing clipboard ref at {i}"
        );
        assert_eq!(
            watch.observe_text(&format!("ambient-{i}")),
            ClipboardObserveOutcome::IgnoredDisabled
        );
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn corpus_scripted_poll_sequences() {
    for i in 0..50usize {
        let source = ScriptedClipboardSource::new();
        let base = format!("poll-base-{i}");
        let next = format!("poll-next-{i}");
        source.push_texts([base.clone(), base.clone(), next.clone()]);
        let mut watch = ClipboardWatchController::new();
        watch.enable(None);
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Unchanged
        );
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Unchanged
        );
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Proposed
        );
        assert_eq!(watch.pending_proposal().map(|p| p.text.clone()), Some(next));
        watch.disable();
        assert!(watch.pending_proposal().is_none());
    }
}

#[test]
fn corpus_preview_length_and_scrub() {
    for n in [10usize, 50, 100, 240, 241, 300, 500, 1000] {
        for i in 0..8usize {
            let mut body = format!("preview-{i}-password=hunter{i}-");
            body.push_str(&"z".repeat(n));
            let preview = proposal_preview(&body);
            assert!(!preview.contains(&format!("hunter{i}")));
            assert!(preview.contains(AMBIENT_REDACTED));
            assert!(preview.chars().count() <= 241);
        }
    }
}
