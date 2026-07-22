//! Extra table-driven clipboard watch observe/confirm patterns (#77).

use ronin_core::{
    clipboard_watch_proposal_may_inject_into_chat_request,
    confirmed_clipboard_attach_may_inject_into_chat_request, may_inject_into_chat_request,
    proposal_preview, ClipboardObserveOutcome, ClipboardWatchController, ContextOrigin,
    ScriptedClipboardSource, AMBIENT_REDACTED,
};

#[test]
fn pattern_observe_change_grid() {
    let pairs: &[(&str, &str)] = &[
        ("base-pat-000", "next-pat-000-token=pat000"),
        ("base-pat-001", "next-pat-001-token=pat001"),
        ("base-pat-002", "next-pat-002-token=pat002"),
        ("base-pat-003", "next-pat-003-token=pat003"),
        ("base-pat-004", "next-pat-004-token=pat004"),
        ("base-pat-005", "next-pat-005-token=pat005"),
        ("base-pat-006", "next-pat-006-token=pat006"),
        ("base-pat-007", "next-pat-007-token=pat007"),
        ("base-pat-008", "next-pat-008-token=pat008"),
        ("base-pat-009", "next-pat-009-token=pat009"),
        ("base-pat-010", "next-pat-010-token=pat010"),
        ("base-pat-011", "next-pat-011-token=pat011"),
        ("base-pat-012", "next-pat-012-token=pat012"),
        ("base-pat-013", "next-pat-013-token=pat013"),
        ("base-pat-014", "next-pat-014-token=pat014"),
        ("base-pat-015", "next-pat-015-token=pat015"),
        ("base-pat-016", "next-pat-016-token=pat016"),
        ("base-pat-017", "next-pat-017-token=pat017"),
        ("base-pat-018", "next-pat-018-token=pat018"),
        ("base-pat-019", "next-pat-019-token=pat019"),
        ("base-pat-020", "next-pat-020-token=pat020"),
        ("base-pat-021", "next-pat-021-token=pat021"),
        ("base-pat-022", "next-pat-022-token=pat022"),
        ("base-pat-023", "next-pat-023-token=pat023"),
        ("base-pat-024", "next-pat-024-token=pat024"),
        ("base-pat-025", "next-pat-025-token=pat025"),
        ("base-pat-026", "next-pat-026-token=pat026"),
        ("base-pat-027", "next-pat-027-token=pat027"),
        ("base-pat-028", "next-pat-028-token=pat028"),
        ("base-pat-029", "next-pat-029-token=pat029"),
        ("base-pat-030", "next-pat-030-token=pat030"),
        ("base-pat-031", "next-pat-031-token=pat031"),
        ("base-pat-032", "next-pat-032-token=pat032"),
        ("base-pat-033", "next-pat-033-token=pat033"),
        ("base-pat-034", "next-pat-034-token=pat034"),
        ("base-pat-035", "next-pat-035-token=pat035"),
        ("base-pat-036", "next-pat-036-token=pat036"),
        ("base-pat-037", "next-pat-037-token=pat037"),
        ("base-pat-038", "next-pat-038-token=pat038"),
        ("base-pat-039", "next-pat-039-token=pat039"),
        ("base-pat-040", "next-pat-040-token=pat040"),
        ("base-pat-041", "next-pat-041-token=pat041"),
        ("base-pat-042", "next-pat-042-token=pat042"),
        ("base-pat-043", "next-pat-043-token=pat043"),
        ("base-pat-044", "next-pat-044-token=pat044"),
        ("base-pat-045", "next-pat-045-token=pat045"),
        ("base-pat-046", "next-pat-046-token=pat046"),
        ("base-pat-047", "next-pat-047-token=pat047"),
        ("base-pat-048", "next-pat-048-token=pat048"),
        ("base-pat-049", "next-pat-049-token=pat049"),
        ("base-pat-050", "next-pat-050-token=pat050"),
        ("base-pat-051", "next-pat-051-token=pat051"),
        ("base-pat-052", "next-pat-052-token=pat052"),
        ("base-pat-053", "next-pat-053-token=pat053"),
        ("base-pat-054", "next-pat-054-token=pat054"),
        ("base-pat-055", "next-pat-055-token=pat055"),
        ("base-pat-056", "next-pat-056-token=pat056"),
        ("base-pat-057", "next-pat-057-token=pat057"),
        ("base-pat-058", "next-pat-058-token=pat058"),
        ("base-pat-059", "next-pat-059-token=pat059"),
        ("base-pat-060", "next-pat-060-token=pat060"),
        ("base-pat-061", "next-pat-061-token=pat061"),
        ("base-pat-062", "next-pat-062-token=pat062"),
        ("base-pat-063", "next-pat-063-token=pat063"),
        ("base-pat-064", "next-pat-064-token=pat064"),
        ("base-pat-065", "next-pat-065-token=pat065"),
        ("base-pat-066", "next-pat-066-token=pat066"),
        ("base-pat-067", "next-pat-067-token=pat067"),
        ("base-pat-068", "next-pat-068-token=pat068"),
        ("base-pat-069", "next-pat-069-token=pat069"),
        ("base-pat-070", "next-pat-070-token=pat070"),
        ("base-pat-071", "next-pat-071-token=pat071"),
        ("base-pat-072", "next-pat-072-token=pat072"),
        ("base-pat-073", "next-pat-073-token=pat073"),
        ("base-pat-074", "next-pat-074-token=pat074"),
        ("base-pat-075", "next-pat-075-token=pat075"),
        ("base-pat-076", "next-pat-076-token=pat076"),
        ("base-pat-077", "next-pat-077-token=pat077"),
        ("base-pat-078", "next-pat-078-token=pat078"),
        ("base-pat-079", "next-pat-079-token=pat079"),
        ("base-pat-080", "next-pat-080-token=pat080"),
        ("base-pat-081", "next-pat-081-token=pat081"),
        ("base-pat-082", "next-pat-082-token=pat082"),
        ("base-pat-083", "next-pat-083-token=pat083"),
        ("base-pat-084", "next-pat-084-token=pat084"),
        ("base-pat-085", "next-pat-085-token=pat085"),
        ("base-pat-086", "next-pat-086-token=pat086"),
        ("base-pat-087", "next-pat-087-token=pat087"),
        ("base-pat-088", "next-pat-088-token=pat088"),
        ("base-pat-089", "next-pat-089-token=pat089"),
        ("base-pat-090", "next-pat-090-token=pat090"),
        ("base-pat-091", "next-pat-091-token=pat091"),
        ("base-pat-092", "next-pat-092-token=pat092"),
        ("base-pat-093", "next-pat-093-token=pat093"),
        ("base-pat-094", "next-pat-094-token=pat094"),
        ("base-pat-095", "next-pat-095-token=pat095"),
        ("base-pat-096", "next-pat-096-token=pat096"),
        ("base-pat-097", "next-pat-097-token=pat097"),
        ("base-pat-098", "next-pat-098-token=pat098"),
        ("base-pat-099", "next-pat-099-token=pat099"),
        ("base-pat-100", "next-pat-100-token=pat100"),
        ("base-pat-101", "next-pat-101-token=pat101"),
        ("base-pat-102", "next-pat-102-token=pat102"),
        ("base-pat-103", "next-pat-103-token=pat103"),
        ("base-pat-104", "next-pat-104-token=pat104"),
        ("base-pat-105", "next-pat-105-token=pat105"),
        ("base-pat-106", "next-pat-106-token=pat106"),
        ("base-pat-107", "next-pat-107-token=pat107"),
        ("base-pat-108", "next-pat-108-token=pat108"),
        ("base-pat-109", "next-pat-109-token=pat109"),
        ("base-pat-110", "next-pat-110-token=pat110"),
        ("base-pat-111", "next-pat-111-token=pat111"),
        ("base-pat-112", "next-pat-112-token=pat112"),
        ("base-pat-113", "next-pat-113-token=pat113"),
        ("base-pat-114", "next-pat-114-token=pat114"),
        ("base-pat-115", "next-pat-115-token=pat115"),
        ("base-pat-116", "next-pat-116-token=pat116"),
        ("base-pat-117", "next-pat-117-token=pat117"),
        ("base-pat-118", "next-pat-118-token=pat118"),
        ("base-pat-119", "next-pat-119-token=pat119"),
        ("base-pat-120", "next-pat-120-token=pat120"),
        ("base-pat-121", "next-pat-121-token=pat121"),
        ("base-pat-122", "next-pat-122-token=pat122"),
        ("base-pat-123", "next-pat-123-token=pat123"),
        ("base-pat-124", "next-pat-124-token=pat124"),
        ("base-pat-125", "next-pat-125-token=pat125"),
        ("base-pat-126", "next-pat-126-token=pat126"),
        ("base-pat-127", "next-pat-127-token=pat127"),
        ("base-pat-128", "next-pat-128-token=pat128"),
        ("base-pat-129", "next-pat-129-token=pat129"),
        ("base-pat-130", "next-pat-130-token=pat130"),
        ("base-pat-131", "next-pat-131-token=pat131"),
        ("base-pat-132", "next-pat-132-token=pat132"),
        ("base-pat-133", "next-pat-133-token=pat133"),
        ("base-pat-134", "next-pat-134-token=pat134"),
        ("base-pat-135", "next-pat-135-token=pat135"),
        ("base-pat-136", "next-pat-136-token=pat136"),
        ("base-pat-137", "next-pat-137-token=pat137"),
        ("base-pat-138", "next-pat-138-token=pat138"),
        ("base-pat-139", "next-pat-139-token=pat139"),
        ("base-pat-140", "next-pat-140-token=pat140"),
        ("base-pat-141", "next-pat-141-token=pat141"),
        ("base-pat-142", "next-pat-142-token=pat142"),
        ("base-pat-143", "next-pat-143-token=pat143"),
        ("base-pat-144", "next-pat-144-token=pat144"),
        ("base-pat-145", "next-pat-145-token=pat145"),
        ("base-pat-146", "next-pat-146-token=pat146"),
        ("base-pat-147", "next-pat-147-token=pat147"),
        ("base-pat-148", "next-pat-148-token=pat148"),
        ("base-pat-149", "next-pat-149-token=pat149"),
        ("base-pat-150", "next-pat-150-token=pat150"),
        ("base-pat-151", "next-pat-151-token=pat151"),
        ("base-pat-152", "next-pat-152-token=pat152"),
        ("base-pat-153", "next-pat-153-token=pat153"),
        ("base-pat-154", "next-pat-154-token=pat154"),
        ("base-pat-155", "next-pat-155-token=pat155"),
        ("base-pat-156", "next-pat-156-token=pat156"),
        ("base-pat-157", "next-pat-157-token=pat157"),
        ("base-pat-158", "next-pat-158-token=pat158"),
        ("base-pat-159", "next-pat-159-token=pat159"),
        ("base-pat-160", "next-pat-160-token=pat160"),
        ("base-pat-161", "next-pat-161-token=pat161"),
        ("base-pat-162", "next-pat-162-token=pat162"),
        ("base-pat-163", "next-pat-163-token=pat163"),
        ("base-pat-164", "next-pat-164-token=pat164"),
        ("base-pat-165", "next-pat-165-token=pat165"),
        ("base-pat-166", "next-pat-166-token=pat166"),
        ("base-pat-167", "next-pat-167-token=pat167"),
        ("base-pat-168", "next-pat-168-token=pat168"),
        ("base-pat-169", "next-pat-169-token=pat169"),
        ("base-pat-170", "next-pat-170-token=pat170"),
        ("base-pat-171", "next-pat-171-token=pat171"),
        ("base-pat-172", "next-pat-172-token=pat172"),
        ("base-pat-173", "next-pat-173-token=pat173"),
        ("base-pat-174", "next-pat-174-token=pat174"),
        ("base-pat-175", "next-pat-175-token=pat175"),
        ("base-pat-176", "next-pat-176-token=pat176"),
        ("base-pat-177", "next-pat-177-token=pat177"),
        ("base-pat-178", "next-pat-178-token=pat178"),
        ("base-pat-179", "next-pat-179-token=pat179"),
        ("base-pat-180", "next-pat-180-token=pat180"),
        ("base-pat-181", "next-pat-181-token=pat181"),
        ("base-pat-182", "next-pat-182-token=pat182"),
        ("base-pat-183", "next-pat-183-token=pat183"),
        ("base-pat-184", "next-pat-184-token=pat184"),
        ("base-pat-185", "next-pat-185-token=pat185"),
        ("base-pat-186", "next-pat-186-token=pat186"),
        ("base-pat-187", "next-pat-187-token=pat187"),
        ("base-pat-188", "next-pat-188-token=pat188"),
        ("base-pat-189", "next-pat-189-token=pat189"),
        ("base-pat-190", "next-pat-190-token=pat190"),
        ("base-pat-191", "next-pat-191-token=pat191"),
        ("base-pat-192", "next-pat-192-token=pat192"),
        ("base-pat-193", "next-pat-193-token=pat193"),
        ("base-pat-194", "next-pat-194-token=pat194"),
        ("base-pat-195", "next-pat-195-token=pat195"),
        ("base-pat-196", "next-pat-196-token=pat196"),
        ("base-pat-197", "next-pat-197-token=pat197"),
        ("base-pat-198", "next-pat-198-token=pat198"),
        ("base-pat-199", "next-pat-199-token=pat199"),
    ];
    for (base, nxt) in pairs {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some(base));
        assert_eq!(watch.observe_text(base), ClipboardObserveOutcome::Unchanged);
        assert_eq!(watch.observe_text(nxt), ClipboardObserveOutcome::Proposed);
        let p = watch.pending_proposal().unwrap();
        assert_eq!(p.text, *nxt);
        assert!(p.preview.contains(AMBIENT_REDACTED));
        assert!(!clipboard_watch_proposal_may_inject_into_chat_request());
        assert!(!may_inject_into_chat_request(
            ContextOrigin::ClipboardWatchProposal
        ));
        let draft = watch.confirm_pending().unwrap();
        assert_eq!(draft.content.as_deref(), Some(*nxt));
        assert!(confirmed_clipboard_attach_may_inject_into_chat_request());
    }
}

#[test]
fn pattern_dismiss_then_new_proposal() {
    for i in 0..120usize {
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("d0"));
        let first = format!("first-{i}");
        let second = format!("second-{i}");
        assert_eq!(
            watch.observe_text(&first),
            ClipboardObserveOutcome::Proposed
        );
        watch.dismiss_pending();
        assert!(watch.pending_proposal().is_none());
        assert_eq!(
            watch.observe_text(&second),
            ClipboardObserveOutcome::Proposed
        );
        assert_eq!(watch.pending_proposal().unwrap().text, second);
        watch.disable();
        assert_eq!(
            watch.observe_text("x"),
            ClipboardObserveOutcome::IgnoredDisabled
        );
    }
}

#[test]
fn pattern_scripted_source_fifo() {
    for i in 0..80usize {
        let source = ScriptedClipboardSource::new();
        let a = format!("a{i}");
        let b = format!("b{i}");
        let c = format!("c{i}");
        source.push_texts([a.clone(), b.clone(), c.clone()]);
        let mut watch = ClipboardWatchController::new();
        watch.enable(None);
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Unchanged
        );
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Proposed
        );
        assert_eq!(watch.pending_proposal().unwrap().text, b);
        assert_eq!(
            watch.poll_source(&source).unwrap(),
            ClipboardObserveOutcome::Proposed
        );
        assert_eq!(watch.pending_proposal().unwrap().text, c);
    }
}

#[test]
fn pattern_preview_only_for_display_not_inject() {
    let secrets: &[&str] = &[
        "Bearer sk-live-PATTERN000 password=pw000",
        "Bearer sk-live-PATTERN001 password=pw001",
        "Bearer sk-live-PATTERN002 password=pw002",
        "Bearer sk-live-PATTERN003 password=pw003",
        "Bearer sk-live-PATTERN004 password=pw004",
        "Bearer sk-live-PATTERN005 password=pw005",
        "Bearer sk-live-PATTERN006 password=pw006",
        "Bearer sk-live-PATTERN007 password=pw007",
        "Bearer sk-live-PATTERN008 password=pw008",
        "Bearer sk-live-PATTERN009 password=pw009",
        "Bearer sk-live-PATTERN010 password=pw010",
        "Bearer sk-live-PATTERN011 password=pw011",
        "Bearer sk-live-PATTERN012 password=pw012",
        "Bearer sk-live-PATTERN013 password=pw013",
        "Bearer sk-live-PATTERN014 password=pw014",
        "Bearer sk-live-PATTERN015 password=pw015",
        "Bearer sk-live-PATTERN016 password=pw016",
        "Bearer sk-live-PATTERN017 password=pw017",
        "Bearer sk-live-PATTERN018 password=pw018",
        "Bearer sk-live-PATTERN019 password=pw019",
        "Bearer sk-live-PATTERN020 password=pw020",
        "Bearer sk-live-PATTERN021 password=pw021",
        "Bearer sk-live-PATTERN022 password=pw022",
        "Bearer sk-live-PATTERN023 password=pw023",
        "Bearer sk-live-PATTERN024 password=pw024",
        "Bearer sk-live-PATTERN025 password=pw025",
        "Bearer sk-live-PATTERN026 password=pw026",
        "Bearer sk-live-PATTERN027 password=pw027",
        "Bearer sk-live-PATTERN028 password=pw028",
        "Bearer sk-live-PATTERN029 password=pw029",
        "Bearer sk-live-PATTERN030 password=pw030",
        "Bearer sk-live-PATTERN031 password=pw031",
        "Bearer sk-live-PATTERN032 password=pw032",
        "Bearer sk-live-PATTERN033 password=pw033",
        "Bearer sk-live-PATTERN034 password=pw034",
        "Bearer sk-live-PATTERN035 password=pw035",
        "Bearer sk-live-PATTERN036 password=pw036",
        "Bearer sk-live-PATTERN037 password=pw037",
        "Bearer sk-live-PATTERN038 password=pw038",
        "Bearer sk-live-PATTERN039 password=pw039",
        "Bearer sk-live-PATTERN040 password=pw040",
        "Bearer sk-live-PATTERN041 password=pw041",
        "Bearer sk-live-PATTERN042 password=pw042",
        "Bearer sk-live-PATTERN043 password=pw043",
        "Bearer sk-live-PATTERN044 password=pw044",
        "Bearer sk-live-PATTERN045 password=pw045",
        "Bearer sk-live-PATTERN046 password=pw046",
        "Bearer sk-live-PATTERN047 password=pw047",
        "Bearer sk-live-PATTERN048 password=pw048",
        "Bearer sk-live-PATTERN049 password=pw049",
        "Bearer sk-live-PATTERN050 password=pw050",
        "Bearer sk-live-PATTERN051 password=pw051",
        "Bearer sk-live-PATTERN052 password=pw052",
        "Bearer sk-live-PATTERN053 password=pw053",
        "Bearer sk-live-PATTERN054 password=pw054",
        "Bearer sk-live-PATTERN055 password=pw055",
        "Bearer sk-live-PATTERN056 password=pw056",
        "Bearer sk-live-PATTERN057 password=pw057",
        "Bearer sk-live-PATTERN058 password=pw058",
        "Bearer sk-live-PATTERN059 password=pw059",
        "Bearer sk-live-PATTERN060 password=pw060",
        "Bearer sk-live-PATTERN061 password=pw061",
        "Bearer sk-live-PATTERN062 password=pw062",
        "Bearer sk-live-PATTERN063 password=pw063",
        "Bearer sk-live-PATTERN064 password=pw064",
        "Bearer sk-live-PATTERN065 password=pw065",
        "Bearer sk-live-PATTERN066 password=pw066",
        "Bearer sk-live-PATTERN067 password=pw067",
        "Bearer sk-live-PATTERN068 password=pw068",
        "Bearer sk-live-PATTERN069 password=pw069",
        "Bearer sk-live-PATTERN070 password=pw070",
        "Bearer sk-live-PATTERN071 password=pw071",
        "Bearer sk-live-PATTERN072 password=pw072",
        "Bearer sk-live-PATTERN073 password=pw073",
        "Bearer sk-live-PATTERN074 password=pw074",
        "Bearer sk-live-PATTERN075 password=pw075",
        "Bearer sk-live-PATTERN076 password=pw076",
        "Bearer sk-live-PATTERN077 password=pw077",
        "Bearer sk-live-PATTERN078 password=pw078",
        "Bearer sk-live-PATTERN079 password=pw079",
        "Bearer sk-live-PATTERN080 password=pw080",
        "Bearer sk-live-PATTERN081 password=pw081",
        "Bearer sk-live-PATTERN082 password=pw082",
        "Bearer sk-live-PATTERN083 password=pw083",
        "Bearer sk-live-PATTERN084 password=pw084",
        "Bearer sk-live-PATTERN085 password=pw085",
        "Bearer sk-live-PATTERN086 password=pw086",
        "Bearer sk-live-PATTERN087 password=pw087",
        "Bearer sk-live-PATTERN088 password=pw088",
        "Bearer sk-live-PATTERN089 password=pw089",
        "Bearer sk-live-PATTERN090 password=pw090",
        "Bearer sk-live-PATTERN091 password=pw091",
        "Bearer sk-live-PATTERN092 password=pw092",
        "Bearer sk-live-PATTERN093 password=pw093",
        "Bearer sk-live-PATTERN094 password=pw094",
        "Bearer sk-live-PATTERN095 password=pw095",
        "Bearer sk-live-PATTERN096 password=pw096",
        "Bearer sk-live-PATTERN097 password=pw097",
        "Bearer sk-live-PATTERN098 password=pw098",
        "Bearer sk-live-PATTERN099 password=pw099",
    ];
    for secret in secrets {
        let preview = proposal_preview(secret);
        assert!(preview.contains(AMBIENT_REDACTED));
        assert!(!preview.contains("sk-live-PATTERN"));
        assert!(!preview.contains("password=pw") || preview.contains(AMBIENT_REDACTED));
        let mut watch = ClipboardWatchController::new();
        watch.enable(Some("z"));
        watch.observe_text(secret);
        assert!(!may_inject_into_chat_request(
            ContextOrigin::ClipboardWatchProposal
        ));
        // Confirm attaches full text (user chose) but origin is confirm-accepted.
        let draft = watch.confirm_pending().unwrap();
        assert_eq!(draft.content.as_deref(), Some(*secret));
        assert!(may_inject_into_chat_request(
            ContextOrigin::ConfirmToAttachAccepted
        ));
    }
}

#[test]
fn pattern_ids_unique_across_long_session() {
    let mut watch = ClipboardWatchController::new();
    watch.enable(Some("id0"));
    let mut seen = std::collections::HashSet::new();
    for i in 0..100usize {
        watch.observe_text(&format!("uniq-{i}"));
        let id = watch.pending_proposal().unwrap().id.clone();
        assert!(seen.insert(id), "id collision at {i}");
    }
    assert_eq!(seen.len(), 100);
}
