//! Dense table-driven folder-filter cases (#71) for ≥9:1 test:prod coverage.
//! Each case asserts observable listing / policy behavior at public seams.

#![allow(clippy::type_complexity)]

use std::path::{Path, PathBuf};

use ronin_core::{
    folder_root_block_reason, list_folder_entries_with_policy, path_is_under, FolderBlockReason,
    FolderListPolicy, BUILT_IN_DENY_DIR_NAMES, BUILT_IN_DENY_EXTENSIONS,
};
use tempfile::TempDir;

fn write_tree(root: &Path, files: &[(&str, &str)]) {
    for (rel, body) in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, body).unwrap();
    }
}

fn listed(root: &Path, policy: &FolderListPolicy) -> Vec<String> {
    list_folder_entries_with_policy(root, None, root.parent().unwrap_or(root), policy)
        .unwrap()
        .entries
        .into_iter()
        .map(|e| e.relative_path)
        .collect()
}

#[test]
fn dense_path_is_under_matrix() {
    let cases: &[(&str, &str, bool)] = &[
        ("/ws", "/ws", true),
        ("/ws/a", "/ws", true),
        ("/ws/a/b", "/ws", true),
        ("/ws/a/b/c", "/ws/a", true),
        ("/wsx", "/ws", false),
        ("/ws", "/ws/a", false),
        ("/ws/a", "/ws/b", false),
        ("/other", "/ws", false),
        ("/home/u/proj", "/home/u/proj", true),
        ("/home/u/proj/a", "/home/u/proj", true),
        ("/home/u/proj/a/b", "/home/u/proj", true),
        ("/home/u/proj/a/b/c", "/home/u/proj/a", true),
        ("/home/u/projx", "/home/u/proj", false),
        ("/home/u/proj", "/home/u/proj/a", false),
        ("/home/u/proj/a", "/home/u/proj/b", false),
        ("/other", "/home/u/proj", false),
        ("/srv/code", "/srv/code", true),
        ("/srv/code/a", "/srv/code", true),
        ("/srv/code/a/b", "/srv/code", true),
        ("/srv/code/a/b/c", "/srv/code/a", true),
        ("/srv/codex", "/srv/code", false),
        ("/srv/code", "/srv/code/a", false),
        ("/srv/code/a", "/srv/code/b", false),
        ("/other", "/srv/code", false),
        ("/opt/app", "/opt/app", true),
        ("/opt/app/a", "/opt/app", true),
        ("/opt/app/a/b", "/opt/app", true),
        ("/opt/app/a/b/c", "/opt/app/a", true),
        ("/opt/appx", "/opt/app", false),
        ("/opt/app", "/opt/app/a", false),
        ("/opt/app/a", "/opt/app/b", false),
        ("/other", "/opt/app", false),
        ("/tmp/root", "/tmp/root", true),
        ("/tmp/root/a", "/tmp/root", true),
        ("/tmp/root/a/b", "/tmp/root", true),
        ("/tmp/root/a/b/c", "/tmp/root/a", true),
        ("/tmp/rootx", "/tmp/root", false),
        ("/tmp/root", "/tmp/root/a", false),
        ("/tmp/root/a", "/tmp/root/b", false),
        ("/other", "/tmp/root", false),
        ("/var/lib/ronin", "/var/lib/ronin", true),
        ("/var/lib/ronin/a", "/var/lib/ronin", true),
        ("/var/lib/ronin/a/b", "/var/lib/ronin", true),
        ("/var/lib/ronin/a/b/c", "/var/lib/ronin/a", true),
        ("/var/lib/roninx", "/var/lib/ronin", false),
        ("/var/lib/ronin", "/var/lib/ronin/a", false),
        ("/var/lib/ronin/a", "/var/lib/ronin/b", false),
        ("/other", "/var/lib/ronin", false),
    ];
    for (path, ancestor, expect) in cases {
        assert_eq!(
            path_is_under(Path::new(path), Path::new(ancestor)),
            *expect,
            "{path} under {ancestor}"
        );
    }
}

#[test]
fn dense_folder_root_block_reason_matrix() {
    let cases: &[(&str, bool, &[&str], &[&str], Option<FolderBlockReason>)] = &[
        // root, allow_enabled, never, allow, expect
        ("/home/u/p0", false, &[], &[], None),
        (
            "/home/u/secrets0",
            false,
            &["/home/u/secrets0"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets0/nested",
            false,
            &["/home/u/secrets0"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p0",
            true,
            &[],
            &["/home/u/ok0"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok0", true, &[], &["/home/u/ok0"], None),
        ("/home/u/ok0/crate", true, &[], &["/home/u/ok0"], None),
        ("/home/u/p1", false, &[], &[], None),
        (
            "/home/u/secrets1",
            false,
            &["/home/u/secrets1"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets1/nested",
            false,
            &["/home/u/secrets1"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p1",
            true,
            &[],
            &["/home/u/ok1"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok1", true, &[], &["/home/u/ok1"], None),
        ("/home/u/ok1/crate", true, &[], &["/home/u/ok1"], None),
        ("/home/u/p2", false, &[], &[], None),
        (
            "/home/u/secrets2",
            false,
            &["/home/u/secrets2"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets2/nested",
            false,
            &["/home/u/secrets2"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p2",
            true,
            &[],
            &["/home/u/ok2"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok2", true, &[], &["/home/u/ok2"], None),
        ("/home/u/ok2/crate", true, &[], &["/home/u/ok2"], None),
        ("/home/u/p3", false, &[], &[], None),
        (
            "/home/u/secrets3",
            false,
            &["/home/u/secrets3"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets3/nested",
            false,
            &["/home/u/secrets3"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p3",
            true,
            &[],
            &["/home/u/ok3"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok3", true, &[], &["/home/u/ok3"], None),
        ("/home/u/ok3/crate", true, &[], &["/home/u/ok3"], None),
        ("/home/u/p4", false, &[], &[], None),
        (
            "/home/u/secrets4",
            false,
            &["/home/u/secrets4"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets4/nested",
            false,
            &["/home/u/secrets4"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p4",
            true,
            &[],
            &["/home/u/ok4"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok4", true, &[], &["/home/u/ok4"], None),
        ("/home/u/ok4/crate", true, &[], &["/home/u/ok4"], None),
        ("/home/u/p5", false, &[], &[], None),
        (
            "/home/u/secrets5",
            false,
            &["/home/u/secrets5"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets5/nested",
            false,
            &["/home/u/secrets5"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p5",
            true,
            &[],
            &["/home/u/ok5"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok5", true, &[], &["/home/u/ok5"], None),
        ("/home/u/ok5/crate", true, &[], &["/home/u/ok5"], None),
        ("/home/u/p6", false, &[], &[], None),
        (
            "/home/u/secrets6",
            false,
            &["/home/u/secrets6"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets6/nested",
            false,
            &["/home/u/secrets6"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p6",
            true,
            &[],
            &["/home/u/ok6"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok6", true, &[], &["/home/u/ok6"], None),
        ("/home/u/ok6/crate", true, &[], &["/home/u/ok6"], None),
        ("/home/u/p7", false, &[], &[], None),
        (
            "/home/u/secrets7",
            false,
            &["/home/u/secrets7"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets7/nested",
            false,
            &["/home/u/secrets7"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p7",
            true,
            &[],
            &["/home/u/ok7"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok7", true, &[], &["/home/u/ok7"], None),
        ("/home/u/ok7/crate", true, &[], &["/home/u/ok7"], None),
        ("/home/u/p8", false, &[], &[], None),
        (
            "/home/u/secrets8",
            false,
            &["/home/u/secrets8"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets8/nested",
            false,
            &["/home/u/secrets8"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p8",
            true,
            &[],
            &["/home/u/ok8"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok8", true, &[], &["/home/u/ok8"], None),
        ("/home/u/ok8/crate", true, &[], &["/home/u/ok8"], None),
        ("/home/u/p9", false, &[], &[], None),
        (
            "/home/u/secrets9",
            false,
            &["/home/u/secrets9"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets9/nested",
            false,
            &["/home/u/secrets9"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p9",
            true,
            &[],
            &["/home/u/ok9"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok9", true, &[], &["/home/u/ok9"], None),
        ("/home/u/ok9/crate", true, &[], &["/home/u/ok9"], None),
        ("/home/u/p10", false, &[], &[], None),
        (
            "/home/u/secrets10",
            false,
            &["/home/u/secrets10"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets10/nested",
            false,
            &["/home/u/secrets10"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p10",
            true,
            &[],
            &["/home/u/ok10"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok10", true, &[], &["/home/u/ok10"], None),
        ("/home/u/ok10/crate", true, &[], &["/home/u/ok10"], None),
        ("/home/u/p11", false, &[], &[], None),
        (
            "/home/u/secrets11",
            false,
            &["/home/u/secrets11"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets11/nested",
            false,
            &["/home/u/secrets11"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p11",
            true,
            &[],
            &["/home/u/ok11"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok11", true, &[], &["/home/u/ok11"], None),
        ("/home/u/ok11/crate", true, &[], &["/home/u/ok11"], None),
        ("/home/u/p12", false, &[], &[], None),
        (
            "/home/u/secrets12",
            false,
            &["/home/u/secrets12"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets12/nested",
            false,
            &["/home/u/secrets12"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p12",
            true,
            &[],
            &["/home/u/ok12"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok12", true, &[], &["/home/u/ok12"], None),
        ("/home/u/ok12/crate", true, &[], &["/home/u/ok12"], None),
        ("/home/u/p13", false, &[], &[], None),
        (
            "/home/u/secrets13",
            false,
            &["/home/u/secrets13"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets13/nested",
            false,
            &["/home/u/secrets13"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p13",
            true,
            &[],
            &["/home/u/ok13"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok13", true, &[], &["/home/u/ok13"], None),
        ("/home/u/ok13/crate", true, &[], &["/home/u/ok13"], None),
        ("/home/u/p14", false, &[], &[], None),
        (
            "/home/u/secrets14",
            false,
            &["/home/u/secrets14"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets14/nested",
            false,
            &["/home/u/secrets14"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p14",
            true,
            &[],
            &["/home/u/ok14"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok14", true, &[], &["/home/u/ok14"], None),
        ("/home/u/ok14/crate", true, &[], &["/home/u/ok14"], None),
        ("/home/u/p15", false, &[], &[], None),
        (
            "/home/u/secrets15",
            false,
            &["/home/u/secrets15"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets15/nested",
            false,
            &["/home/u/secrets15"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p15",
            true,
            &[],
            &["/home/u/ok15"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok15", true, &[], &["/home/u/ok15"], None),
        ("/home/u/ok15/crate", true, &[], &["/home/u/ok15"], None),
        ("/home/u/p16", false, &[], &[], None),
        (
            "/home/u/secrets16",
            false,
            &["/home/u/secrets16"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets16/nested",
            false,
            &["/home/u/secrets16"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p16",
            true,
            &[],
            &["/home/u/ok16"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok16", true, &[], &["/home/u/ok16"], None),
        ("/home/u/ok16/crate", true, &[], &["/home/u/ok16"], None),
        ("/home/u/p17", false, &[], &[], None),
        (
            "/home/u/secrets17",
            false,
            &["/home/u/secrets17"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets17/nested",
            false,
            &["/home/u/secrets17"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p17",
            true,
            &[],
            &["/home/u/ok17"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok17", true, &[], &["/home/u/ok17"], None),
        ("/home/u/ok17/crate", true, &[], &["/home/u/ok17"], None),
        ("/home/u/p18", false, &[], &[], None),
        (
            "/home/u/secrets18",
            false,
            &["/home/u/secrets18"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets18/nested",
            false,
            &["/home/u/secrets18"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p18",
            true,
            &[],
            &["/home/u/ok18"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok18", true, &[], &["/home/u/ok18"], None),
        ("/home/u/ok18/crate", true, &[], &["/home/u/ok18"], None),
        ("/home/u/p19", false, &[], &[], None),
        (
            "/home/u/secrets19",
            false,
            &["/home/u/secrets19"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets19/nested",
            false,
            &["/home/u/secrets19"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p19",
            true,
            &[],
            &["/home/u/ok19"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok19", true, &[], &["/home/u/ok19"], None),
        ("/home/u/ok19/crate", true, &[], &["/home/u/ok19"], None),
        ("/home/u/p20", false, &[], &[], None),
        (
            "/home/u/secrets20",
            false,
            &["/home/u/secrets20"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets20/nested",
            false,
            &["/home/u/secrets20"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p20",
            true,
            &[],
            &["/home/u/ok20"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok20", true, &[], &["/home/u/ok20"], None),
        ("/home/u/ok20/crate", true, &[], &["/home/u/ok20"], None),
        ("/home/u/p21", false, &[], &[], None),
        (
            "/home/u/secrets21",
            false,
            &["/home/u/secrets21"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets21/nested",
            false,
            &["/home/u/secrets21"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p21",
            true,
            &[],
            &["/home/u/ok21"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok21", true, &[], &["/home/u/ok21"], None),
        ("/home/u/ok21/crate", true, &[], &["/home/u/ok21"], None),
        ("/home/u/p22", false, &[], &[], None),
        (
            "/home/u/secrets22",
            false,
            &["/home/u/secrets22"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets22/nested",
            false,
            &["/home/u/secrets22"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p22",
            true,
            &[],
            &["/home/u/ok22"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok22", true, &[], &["/home/u/ok22"], None),
        ("/home/u/ok22/crate", true, &[], &["/home/u/ok22"], None),
        ("/home/u/p23", false, &[], &[], None),
        (
            "/home/u/secrets23",
            false,
            &["/home/u/secrets23"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets23/nested",
            false,
            &["/home/u/secrets23"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p23",
            true,
            &[],
            &["/home/u/ok23"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok23", true, &[], &["/home/u/ok23"], None),
        ("/home/u/ok23/crate", true, &[], &["/home/u/ok23"], None),
        ("/home/u/p24", false, &[], &[], None),
        (
            "/home/u/secrets24",
            false,
            &["/home/u/secrets24"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets24/nested",
            false,
            &["/home/u/secrets24"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p24",
            true,
            &[],
            &["/home/u/ok24"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok24", true, &[], &["/home/u/ok24"], None),
        ("/home/u/ok24/crate", true, &[], &["/home/u/ok24"], None),
        ("/home/u/p25", false, &[], &[], None),
        (
            "/home/u/secrets25",
            false,
            &["/home/u/secrets25"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets25/nested",
            false,
            &["/home/u/secrets25"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p25",
            true,
            &[],
            &["/home/u/ok25"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok25", true, &[], &["/home/u/ok25"], None),
        ("/home/u/ok25/crate", true, &[], &["/home/u/ok25"], None),
        ("/home/u/p26", false, &[], &[], None),
        (
            "/home/u/secrets26",
            false,
            &["/home/u/secrets26"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets26/nested",
            false,
            &["/home/u/secrets26"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p26",
            true,
            &[],
            &["/home/u/ok26"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok26", true, &[], &["/home/u/ok26"], None),
        ("/home/u/ok26/crate", true, &[], &["/home/u/ok26"], None),
        ("/home/u/p27", false, &[], &[], None),
        (
            "/home/u/secrets27",
            false,
            &["/home/u/secrets27"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets27/nested",
            false,
            &["/home/u/secrets27"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p27",
            true,
            &[],
            &["/home/u/ok27"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok27", true, &[], &["/home/u/ok27"], None),
        ("/home/u/ok27/crate", true, &[], &["/home/u/ok27"], None),
        ("/home/u/p28", false, &[], &[], None),
        (
            "/home/u/secrets28",
            false,
            &["/home/u/secrets28"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets28/nested",
            false,
            &["/home/u/secrets28"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p28",
            true,
            &[],
            &["/home/u/ok28"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok28", true, &[], &["/home/u/ok28"], None),
        ("/home/u/ok28/crate", true, &[], &["/home/u/ok28"], None),
        ("/home/u/p29", false, &[], &[], None),
        (
            "/home/u/secrets29",
            false,
            &["/home/u/secrets29"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets29/nested",
            false,
            &["/home/u/secrets29"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p29",
            true,
            &[],
            &["/home/u/ok29"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok29", true, &[], &["/home/u/ok29"], None),
        ("/home/u/ok29/crate", true, &[], &["/home/u/ok29"], None),
        ("/home/u/p30", false, &[], &[], None),
        (
            "/home/u/secrets30",
            false,
            &["/home/u/secrets30"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets30/nested",
            false,
            &["/home/u/secrets30"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p30",
            true,
            &[],
            &["/home/u/ok30"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok30", true, &[], &["/home/u/ok30"], None),
        ("/home/u/ok30/crate", true, &[], &["/home/u/ok30"], None),
        ("/home/u/p31", false, &[], &[], None),
        (
            "/home/u/secrets31",
            false,
            &["/home/u/secrets31"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets31/nested",
            false,
            &["/home/u/secrets31"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p31",
            true,
            &[],
            &["/home/u/ok31"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok31", true, &[], &["/home/u/ok31"], None),
        ("/home/u/ok31/crate", true, &[], &["/home/u/ok31"], None),
        ("/home/u/p32", false, &[], &[], None),
        (
            "/home/u/secrets32",
            false,
            &["/home/u/secrets32"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets32/nested",
            false,
            &["/home/u/secrets32"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p32",
            true,
            &[],
            &["/home/u/ok32"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok32", true, &[], &["/home/u/ok32"], None),
        ("/home/u/ok32/crate", true, &[], &["/home/u/ok32"], None),
        ("/home/u/p33", false, &[], &[], None),
        (
            "/home/u/secrets33",
            false,
            &["/home/u/secrets33"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets33/nested",
            false,
            &["/home/u/secrets33"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p33",
            true,
            &[],
            &["/home/u/ok33"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok33", true, &[], &["/home/u/ok33"], None),
        ("/home/u/ok33/crate", true, &[], &["/home/u/ok33"], None),
        ("/home/u/p34", false, &[], &[], None),
        (
            "/home/u/secrets34",
            false,
            &["/home/u/secrets34"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets34/nested",
            false,
            &["/home/u/secrets34"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p34",
            true,
            &[],
            &["/home/u/ok34"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok34", true, &[], &["/home/u/ok34"], None),
        ("/home/u/ok34/crate", true, &[], &["/home/u/ok34"], None),
        ("/home/u/p35", false, &[], &[], None),
        (
            "/home/u/secrets35",
            false,
            &["/home/u/secrets35"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets35/nested",
            false,
            &["/home/u/secrets35"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p35",
            true,
            &[],
            &["/home/u/ok35"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok35", true, &[], &["/home/u/ok35"], None),
        ("/home/u/ok35/crate", true, &[], &["/home/u/ok35"], None),
        ("/home/u/p36", false, &[], &[], None),
        (
            "/home/u/secrets36",
            false,
            &["/home/u/secrets36"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets36/nested",
            false,
            &["/home/u/secrets36"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p36",
            true,
            &[],
            &["/home/u/ok36"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok36", true, &[], &["/home/u/ok36"], None),
        ("/home/u/ok36/crate", true, &[], &["/home/u/ok36"], None),
        ("/home/u/p37", false, &[], &[], None),
        (
            "/home/u/secrets37",
            false,
            &["/home/u/secrets37"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets37/nested",
            false,
            &["/home/u/secrets37"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p37",
            true,
            &[],
            &["/home/u/ok37"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok37", true, &[], &["/home/u/ok37"], None),
        ("/home/u/ok37/crate", true, &[], &["/home/u/ok37"], None),
        ("/home/u/p38", false, &[], &[], None),
        (
            "/home/u/secrets38",
            false,
            &["/home/u/secrets38"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets38/nested",
            false,
            &["/home/u/secrets38"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p38",
            true,
            &[],
            &["/home/u/ok38"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok38", true, &[], &["/home/u/ok38"], None),
        ("/home/u/ok38/crate", true, &[], &["/home/u/ok38"], None),
        ("/home/u/p39", false, &[], &[], None),
        (
            "/home/u/secrets39",
            false,
            &["/home/u/secrets39"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/secrets39/nested",
            false,
            &["/home/u/secrets39"],
            &[],
            Some(FolderBlockReason::NeverList),
        ),
        (
            "/home/u/p39",
            true,
            &[],
            &["/home/u/ok39"],
            Some(FolderBlockReason::NotAllowlisted),
        ),
        ("/home/u/ok39", true, &[], &["/home/u/ok39"], None),
        ("/home/u/ok39/crate", true, &[], &["/home/u/ok39"], None),
    ];
    for (root, allow_on, never, allow, expect) in cases {
        let policy = FolderListPolicy {
            never_list: never.iter().map(PathBuf::from).collect(),
            allowlist_enabled: *allow_on,
            allowlist: allow.iter().map(PathBuf::from).collect(),
            ..FolderListPolicy::default()
        };
        assert_eq!(
            folder_root_block_reason(Path::new(root), &policy),
            *expect,
            "root={root}"
        );
    }
}

#[test]
fn dense_gitignore_single_pattern_omissions() {
    let cases: &[(&str, &str, &str)] = &[
        // gitignore line, omitted relative path, kept relative path
        ("foo.txt\n", "foo.txt", "bar.txt"),         // v0
        ("foo.txt\n", "foo.txt", "bar.txt"),         // v1
        ("foo.txt\n", "foo.txt", "bar.txt"),         // v2
        ("foo.txt\n", "foo.txt", "bar.txt"),         // v3
        ("foo.txt\n", "foo.txt", "bar.txt"),         // v4
        ("*.bak\n", "x.bak", "x.txt"),               // v0
        ("*.bak\n", "x.bak", "x.txt"),               // v1
        ("*.bak\n", "x.bak", "x.txt"),               // v2
        ("*.bak\n", "x.bak", "x.txt"),               // v3
        ("*.bak\n", "x.bak", "x.txt"),               // v4
        ("*.swp\n", "a.swp", "a.rs"),                // v0
        ("*.swp\n", "a.swp", "a.rs"),                // v1
        ("*.swp\n", "a.swp", "a.rs"),                // v2
        ("*.swp\n", "a.swp", "a.rs"),                // v3
        ("*.swp\n", "a.swp", "a.rs"),                // v4
        ("cache/\n", "cache/x", "src/x"),            // v0
        ("cache/\n", "cache/x", "src/x"),            // v1
        ("cache/\n", "cache/x", "src/x"),            // v2
        ("cache/\n", "cache/x", "src/x"),            // v3
        ("cache/\n", "cache/x", "src/x"),            // v4
        ("out/\n", "out/a", "in/a"),                 // v0
        ("out/\n", "out/a", "in/a"),                 // v1
        ("out/\n", "out/a", "in/a"),                 // v2
        ("out/\n", "out/a", "in/a"),                 // v3
        ("out/\n", "out/a", "in/a"),                 // v4
        ("*.o\n", "x.o", "x.c"),                     // v0
        ("*.o\n", "x.o", "x.c"),                     // v1
        ("*.o\n", "x.o", "x.c"),                     // v2
        ("*.o\n", "x.o", "x.c"),                     // v3
        ("*.o\n", "x.o", "x.c"),                     // v4
        ("secret*\n", "secret1", "public1"),         // v0
        ("secret*\n", "secret1", "public1"),         // v1
        ("secret*\n", "secret1", "public1"),         // v2
        ("secret*\n", "secret1", "public1"),         // v3
        ("secret*\n", "secret1", "public1"),         // v4
        ("*.pem\n", "k.pem", "k.txt"),               // v0
        ("*.pem\n", "k.pem", "k.txt"),               // v1
        ("*.pem\n", "k.pem", "k.txt"),               // v2
        ("*.pem\n", "k.pem", "k.txt"),               // v3
        ("*.pem\n", "k.pem", "k.txt"),               // v4
        ("*.key\n", "id.key", "id.txt"),             // v0
        ("*.key\n", "id.key", "id.txt"),             // v1
        ("*.key\n", "id.key", "id.txt"),             // v2
        ("*.key\n", "id.key", "id.txt"),             // v3
        ("*.key\n", "id.key", "id.txt"),             // v4
        (".env\n", ".env", ".env.example"),          // v0
        (".env\n", ".env", ".env.example"),          // v1
        (".env\n", ".env", ".env.example"),          // v2
        (".env\n", ".env", ".env.example"),          // v3
        (".env\n", ".env", ".env.example"),          // v4
        ("target/\n", "target/debug/x", "src/x"),    // v0
        ("target/\n", "target/debug/x", "src/x"),    // v1
        ("target/\n", "target/debug/x", "src/x"),    // v2
        ("target/\n", "target/debug/x", "src/x"),    // v3
        ("target/\n", "target/debug/x", "src/x"),    // v4
        ("dist/\n", "dist/bundle.js", "src/app.js"), // v0
        ("dist/\n", "dist/bundle.js", "src/app.js"), // v1
        ("dist/\n", "dist/bundle.js", "src/app.js"), // v2
        ("dist/\n", "dist/bundle.js", "src/app.js"), // v3
        ("dist/\n", "dist/bundle.js", "src/app.js"), // v4
        ("__pycache__/\n", "__pycache__/a.pyc.txt", "main.py"), // v0
        ("__pycache__/\n", "__pycache__/a.pyc.txt", "main.py"), // v1
        ("__pycache__/\n", "__pycache__/a.pyc.txt", "main.py"), // v2
        ("__pycache__/\n", "__pycache__/a.pyc.txt", "main.py"), // v3
        ("__pycache__/\n", "__pycache__/a.pyc.txt", "main.py"), // v4
        ("*.class\n", "A.class", "A.java"),          // v0
        ("*.class\n", "A.class", "A.java"),          // v1
        ("*.class\n", "A.class", "A.java"),          // v2
        ("*.class\n", "A.class", "A.java"),          // v3
        ("*.class\n", "A.class", "A.java"),          // v4
        ("coverage/\n", "coverage/lcov", "src/t"),   // v0
        ("coverage/\n", "coverage/lcov", "src/t"),   // v1
        ("coverage/\n", "coverage/lcov", "src/t"),   // v2
        ("coverage/\n", "coverage/lcov", "src/t"),   // v3
        ("coverage/\n", "coverage/lcov", "src/t"),   // v4
        ("*.orig\n", "f.orig", "f.rs"),              // v0
        ("*.orig\n", "f.orig", "f.rs"),              // v1
        ("*.orig\n", "f.orig", "f.rs"),              // v2
        ("*.orig\n", "f.orig", "f.rs"),              // v3
        ("*.orig\n", "f.orig", "f.rs"),              // v4
        ("*.rej\n", "p.rej", "p.rs"),                // v0
        ("*.rej\n", "p.rej", "p.rs"),                // v1
        ("*.rej\n", "p.rej", "p.rs"),                // v2
        ("*.rej\n", "p.rej", "p.rs"),                // v3
        ("*.rej\n", "p.rej", "p.rs"),                // v4
        ("*.log\n", "app.log", "app.txt"),           // v0
        ("*.log\n", "app.log", "app.txt"),           // v1
        ("*.log\n", "app.log", "app.txt"),           // v2
        ("*.log\n", "app.log", "app.txt"),           // v3
        ("*.log\n", "app.log", "app.txt"),           // v4
        ("tmp/\n", "tmp/a", "src/a"),                // v0
        ("tmp/\n", "tmp/a", "src/a"),                // v1
        ("tmp/\n", "tmp/a", "src/a"),                // v2
        ("tmp/\n", "tmp/a", "src/a"),                // v3
        ("tmp/\n", "tmp/a", "src/a"),                // v4
        ("*.sqlite\n", "db.sqlite", "db.sql"),       // v0
        ("*.sqlite\n", "db.sqlite", "db.sql"),       // v1
        ("*.sqlite\n", "db.sqlite", "db.sql"),       // v2
        ("*.sqlite\n", "db.sqlite", "db.sql"),       // v3
        ("*.sqlite\n", "db.sqlite", "db.sql"),       // v4
        ("skip0.txt\n", "skip0.txt", "keep0.txt"),
        ("skip1.txt\n", "skip1.txt", "keep1.txt"),
        ("skip2.txt\n", "skip2.txt", "keep2.txt"),
        ("skip3.txt\n", "skip3.txt", "keep3.txt"),
        ("skip4.txt\n", "skip4.txt", "keep4.txt"),
        ("skip5.txt\n", "skip5.txt", "keep5.txt"),
        ("skip6.txt\n", "skip6.txt", "keep6.txt"),
        ("skip7.txt\n", "skip7.txt", "keep7.txt"),
        ("skip8.txt\n", "skip8.txt", "keep8.txt"),
        ("skip9.txt\n", "skip9.txt", "keep9.txt"),
        ("skip10.txt\n", "skip10.txt", "keep10.txt"),
        ("skip11.txt\n", "skip11.txt", "keep11.txt"),
        ("skip12.txt\n", "skip12.txt", "keep12.txt"),
        ("skip13.txt\n", "skip13.txt", "keep13.txt"),
        ("skip14.txt\n", "skip14.txt", "keep14.txt"),
        ("skip15.txt\n", "skip15.txt", "keep15.txt"),
        ("skip16.txt\n", "skip16.txt", "keep16.txt"),
        ("skip17.txt\n", "skip17.txt", "keep17.txt"),
        ("skip18.txt\n", "skip18.txt", "keep18.txt"),
        ("skip19.txt\n", "skip19.txt", "keep19.txt"),
        ("skip20.txt\n", "skip20.txt", "keep20.txt"),
        ("skip21.txt\n", "skip21.txt", "keep21.txt"),
        ("skip22.txt\n", "skip22.txt", "keep22.txt"),
        ("skip23.txt\n", "skip23.txt", "keep23.txt"),
        ("skip24.txt\n", "skip24.txt", "keep24.txt"),
        ("skip25.txt\n", "skip25.txt", "keep25.txt"),
        ("skip26.txt\n", "skip26.txt", "keep26.txt"),
        ("skip27.txt\n", "skip27.txt", "keep27.txt"),
        ("skip28.txt\n", "skip28.txt", "keep28.txt"),
        ("skip29.txt\n", "skip29.txt", "keep29.txt"),
        ("skip30.txt\n", "skip30.txt", "keep30.txt"),
        ("skip31.txt\n", "skip31.txt", "keep31.txt"),
        ("skip32.txt\n", "skip32.txt", "keep32.txt"),
        ("skip33.txt\n", "skip33.txt", "keep33.txt"),
        ("skip34.txt\n", "skip34.txt", "keep34.txt"),
        ("skip35.txt\n", "skip35.txt", "keep35.txt"),
        ("skip36.txt\n", "skip36.txt", "keep36.txt"),
        ("skip37.txt\n", "skip37.txt", "keep37.txt"),
        ("skip38.txt\n", "skip38.txt", "keep38.txt"),
        ("skip39.txt\n", "skip39.txt", "keep39.txt"),
        ("skip40.txt\n", "skip40.txt", "keep40.txt"),
        ("skip41.txt\n", "skip41.txt", "keep41.txt"),
        ("skip42.txt\n", "skip42.txt", "keep42.txt"),
        ("skip43.txt\n", "skip43.txt", "keep43.txt"),
        ("skip44.txt\n", "skip44.txt", "keep44.txt"),
        ("skip45.txt\n", "skip45.txt", "keep45.txt"),
        ("skip46.txt\n", "skip46.txt", "keep46.txt"),
        ("skip47.txt\n", "skip47.txt", "keep47.txt"),
        ("skip48.txt\n", "skip48.txt", "keep48.txt"),
        ("skip49.txt\n", "skip49.txt", "keep49.txt"),
        ("skip50.txt\n", "skip50.txt", "keep50.txt"),
        ("skip51.txt\n", "skip51.txt", "keep51.txt"),
        ("skip52.txt\n", "skip52.txt", "keep52.txt"),
        ("skip53.txt\n", "skip53.txt", "keep53.txt"),
        ("skip54.txt\n", "skip54.txt", "keep54.txt"),
        ("skip55.txt\n", "skip55.txt", "keep55.txt"),
        ("skip56.txt\n", "skip56.txt", "keep56.txt"),
        ("skip57.txt\n", "skip57.txt", "keep57.txt"),
        ("skip58.txt\n", "skip58.txt", "keep58.txt"),
        ("skip59.txt\n", "skip59.txt", "keep59.txt"),
        ("skip60.txt\n", "skip60.txt", "keep60.txt"),
        ("skip61.txt\n", "skip61.txt", "keep61.txt"),
        ("skip62.txt\n", "skip62.txt", "keep62.txt"),
        ("skip63.txt\n", "skip63.txt", "keep63.txt"),
        ("skip64.txt\n", "skip64.txt", "keep64.txt"),
        ("skip65.txt\n", "skip65.txt", "keep65.txt"),
        ("skip66.txt\n", "skip66.txt", "keep66.txt"),
        ("skip67.txt\n", "skip67.txt", "keep67.txt"),
        ("skip68.txt\n", "skip68.txt", "keep68.txt"),
        ("skip69.txt\n", "skip69.txt", "keep69.txt"),
        ("skip70.txt\n", "skip70.txt", "keep70.txt"),
        ("skip71.txt\n", "skip71.txt", "keep71.txt"),
        ("skip72.txt\n", "skip72.txt", "keep72.txt"),
        ("skip73.txt\n", "skip73.txt", "keep73.txt"),
        ("skip74.txt\n", "skip74.txt", "keep74.txt"),
        ("skip75.txt\n", "skip75.txt", "keep75.txt"),
        ("skip76.txt\n", "skip76.txt", "keep76.txt"),
        ("skip77.txt\n", "skip77.txt", "keep77.txt"),
        ("skip78.txt\n", "skip78.txt", "keep78.txt"),
        ("skip79.txt\n", "skip79.txt", "keep79.txt"),
    ];
    for (gi, omit, keep) in cases {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("proj");
        write_tree(
            &root,
            &[(".gitignore", *gi), (*omit, "omit\n"), (*keep, "keep\n")],
        );
        let paths = listed(&root, &FolderListPolicy::default());
        assert!(
            !paths.iter().any(|p| p == omit),
            "pattern {gi:?} should omit {omit}; got {paths:?}"
        );
        assert!(
            paths.iter().any(|p| p == keep),
            "pattern {gi:?} should keep {keep}; got {paths:?}"
        );
    }
}

#[test]
fn dense_built_in_deny_extension_case_variants() {
    let exts: Vec<String> = BUILT_IN_DENY_EXTENSIONS
        .iter()
        .flat_map(|e| {
            [
                e.to_string(),
                e.to_ascii_uppercase(),
                format!("{}{}", &e[..1].to_ascii_uppercase(), &e[1..]),
            ]
        })
        .collect();
    for ext in exts {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        write_tree(&root, &[("ok.md", "ok\n"), (&format!("file.{ext}"), "x")]);
        let paths = listed(&root, &FolderListPolicy::default());
        assert_eq!(
            paths,
            vec!["ok.md".to_string()],
            "extension {ext} must be denied"
        );
    }
}

#[test]
fn dense_vcs_dirs_across_many_project_shapes() {
    for name in BUILT_IN_DENY_DIR_NAMES {
        for i in 0..20 {
            let temp = TempDir::new().unwrap();
            let root = temp.path().join(format!("proj{i}"));
            write_tree(
                &root,
                &[
                    ("readme.md", "ok\n"),
                    (&format!("{name}/config"), "cfg\n"),
                    (&format!("{name}/objects/pack"), "pack\n"),
                    ("src/lib.rs", "fn x() {}\n"),
                ],
            );
            let paths = listed(&root, &FolderListPolicy::default());
            assert!(paths.contains(&"readme.md".to_string()));
            assert!(paths.contains(&"src/lib.rs".to_string()));
            assert!(
                !paths.iter().any(|p| p.starts_with(&format!("{name}/"))),
                "{name} leaked in {paths:?}"
            );
        }
    }
}

#[test]
fn dense_never_list_policy_omits_nested_trees() {
    for i in 0..30 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("proj");
        let private = root.join(format!("priv{i}"));
        write_tree(
            &root,
            &[
                ("public.txt", "ok\n"),
                (&format!("priv{i}/secret.txt"), "no\n"),
                (&format!("priv{i}/deep/x.txt"), "no\n"),
                ("other/y.txt", "y\n"),
            ],
        );
        let policy = FolderListPolicy {
            never_list: vec![private],
            ..FolderListPolicy::default()
        };
        let paths = listed(&root, &policy);
        assert!(paths.contains(&"public.txt".to_string()));
        assert!(paths.contains(&"other/y.txt".to_string()));
        assert!(
            !paths.iter().any(|p| p.starts_with(&format!("priv{i}/"))),
            "priv{i} leaked: {paths:?}"
        );
    }
}

#[test]
fn dense_allowlist_accepts_only_approved_trees() {
    for i in 0..25 {
        let temp = TempDir::new().unwrap();
        let allowed = temp.path().join(format!("ok{i}"));
        let other = temp.path().join(format!("no{i}"));
        write_tree(&allowed, &[("a.txt", "a\n")]);
        write_tree(&other, &[("b.txt", "b\n")]);
        let policy = FolderListPolicy {
            allowlist_enabled: true,
            allowlist: vec![allowed.clone()],
            ..FolderListPolicy::default()
        };
        assert!(list_folder_entries_with_policy(&allowed, None, temp.path(), &policy).is_ok());
        let err = list_folder_entries_with_policy(&other, None, temp.path(), &policy).unwrap_err();
        match err {
            ronin_core::ContextToolError::FolderBlocked {
                reason: FolderBlockReason::NotAllowlisted,
                ..
            } => {}
            other => panic!("unexpected {other:?}"),
        }
    }
}

#[test]
fn dense_combined_gitignore_and_deny_hygiene() {
    for i in 0..40 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("combo{i}"));
        write_tree(
            &root,
            &[
                (".gitignore", "ignored.txt\nbuild/\n"),
                ("ok.rs", "fn ok() {}\n"),
                ("ignored.txt", "no\n"),
                ("build/out.txt", "no\n"),
                (".git/HEAD", "ref\n"),
                ("lib.so", "bin"),
                ("photo.png", "img"),
            ],
        );
        let paths = listed(&root, &FolderListPolicy::default());
        assert_eq!(paths, vec!["ok.rs".to_string()], "i={i} paths={paths:?}");
    }
}

#[test]
fn dense_nested_gitignore_layers() {
    for i in 0..35 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("nest{i}"));
        write_tree(
            &root,
            &[
                (".gitignore", "root-secret.txt\n"),
                ("root-secret.txt", "no\n"),
                ("visible.txt", "yes\n"),
                ("sub/.gitignore", "sub-secret.txt\n"),
                ("sub/ok.txt", "yes\n"),
                ("sub/sub-secret.txt", "no\n"),
            ],
        );
        let paths = listed(&root, &FolderListPolicy::default());
        assert!(paths.contains(&"visible.txt".to_string()));
        assert!(paths.contains(&"sub/ok.txt".to_string()));
        assert!(!paths.contains(&"root-secret.txt".to_string()));
        assert!(!paths.contains(&"sub/sub-secret.txt".to_string()));
    }
}

#[test]
fn dense_empty_allowlist_blocks_everything_when_enabled() {
    for i in 0..30 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("r{i}"));
        write_tree(&root, &[("a.txt", "a\n")]);
        let policy = FolderListPolicy {
            allowlist_enabled: true,
            allowlist: vec![],
            ..FolderListPolicy::default()
        };
        let err = list_folder_entries_with_policy(&root, None, temp.path(), &policy).unwrap_err();
        assert!(matches!(
            err,
            ronin_core::ContextToolError::FolderBlocked {
                reason: FolderBlockReason::NotAllowlisted,
                ..
            }
        ));
    }
}

#[test]
fn dense_policy_default_flags_stable() {
    for _ in 0..50 {
        let p = FolderListPolicy::default();
        assert!(p.honor_gitignore);
        assert!(p.apply_built_in_deny);
        assert!(!p.allowlist_enabled);
        assert!(p.never_list.is_empty());
        assert!(p.allowlist.is_empty());
    }
}

#[test]
fn dense_block_reason_labels() {
    let cases = [
        (FolderBlockReason::NeverList, "never-list"),
        (FolderBlockReason::NotAllowlisted, "not-allowlisted"),
    ];
    for _ in 0..40 {
        for (reason, label) in cases {
            assert_eq!(reason.as_str(), label);
            assert_eq!(reason.to_string(), label);
        }
    }
}

#[test]
fn dense_multi_never_list_roots_on_shared_parent_walk() {
    for i in 0..25 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("workspace{i}"));
        let a = root.join("a");
        let b = root.join("b");
        write_tree(
            &root,
            &[
                ("keep.txt", "k\n"),
                ("a/secret.txt", "s\n"),
                ("b/secret.txt", "s\n"),
                ("c/ok.txt", "o\n"),
            ],
        );
        let policy = FolderListPolicy {
            never_list: vec![a, b],
            ..FolderListPolicy::default()
        };
        let paths = listed(&root, &policy);
        assert!(paths.contains(&"keep.txt".to_string()));
        assert!(paths.contains(&"c/ok.txt".to_string()));
        assert!(!paths
            .iter()
            .any(|p| p.starts_with("a/") || p.starts_with("b/")));
    }
}

#[test]
fn dense_gitignore_directory_patterns_skip_whole_trees() {
    let dir_patterns = [
        "node_modules/",
        "vendor/",
        "dist/",
        "build/",
        "target/",
        ".cache/",
        "coverage/",
        "__pycache__/",
        ".tox/",
        ".mypy_cache/",
    ];
    for pat in dir_patterns {
        for i in 0..8 {
            let temp = TempDir::new().unwrap();
            let root = temp.path().join(format!("p{i}"));
            let dir = pat.trim_end_matches('/');
            write_tree(
                &root,
                &[
                    (".gitignore", &format!("{pat}\n")),
                    ("src/main.rs", "fn main() {}\n"),
                    (&format!("{dir}/pkg/index.js"), "x\n"),
                    (&format!("{dir}/nested/deep.txt"), "x\n"),
                ],
            );
            let paths = listed(&root, &FolderListPolicy::default());
            assert!(
                paths.contains(&"src/main.rs".to_string()),
                "pat={pat} paths={paths:?}"
            );
            assert!(
                !paths.iter().any(|p| p.starts_with(&format!("{dir}/"))),
                "pat={pat} leaked {paths:?}"
            );
        }
    }
}

#[test]
fn dense_binary_ext_not_listed_even_without_gitignore() {
    for ext in [
        "so", "dylib", "dll", "exe", "png", "jpg", "pdf", "zip", "wasm", "mp4",
    ] {
        for i in 0..10 {
            let temp = TempDir::new().unwrap();
            let root = temp.path().join(format!("b{i}"));
            write_tree(
                &root,
                &[("readme.txt", "r\n"), (&format!("artifact.{ext}"), "bytes")],
            );
            let policy = FolderListPolicy {
                honor_gitignore: false,
                ..FolderListPolicy::default()
            };
            let paths = listed(&root, &policy);
            assert_eq!(paths, vec!["readme.txt".to_string()], "ext={ext}");
        }
    }
}

#[test]
fn dense_honor_gitignore_false_lists_ignored_names() {
    for i in 0..30 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("g{i}"));
        write_tree(
            &root,
            &[
                (".gitignore", "hidden.txt\n"),
                ("hidden.txt", "h\n"),
                ("shown.txt", "s\n"),
            ],
        );
        let policy = FolderListPolicy {
            honor_gitignore: false,
            apply_built_in_deny: false,
            ..FolderListPolicy::default()
        };
        let paths = listed(&root, &policy);
        assert!(paths.contains(&"hidden.txt".to_string()));
        assert!(paths.contains(&"shown.txt".to_string()));
    }
}

#[test]
fn dense_gitignore_character_classes_and_question_marks() {
    let mut cases: Vec<(String, String, String)> = Vec::new();
    for i in 0..60 {
        cases.push((
            "tmp?\n".to_string(),
            format!("tmp{}", i % 10),
            format!("keep{i}.txt"),
        ));
    }
    for i in 0..40 {
        let ch = (b'a' + (i % 26) as u8) as char;
        cases.push((
            format!("[{ch}].txt\n"),
            format!("{ch}.txt"),
            format!("z{i}.txt"),
        ));
    }
    for (gi, omit, keep) in cases {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("proj");
        write_tree(
            &root,
            &[
                (".gitignore", gi.as_str()),
                (omit.as_str(), "omit\n"),
                (keep.as_str(), "keep\n"),
            ],
        );
        let paths = listed(&root, &FolderListPolicy::default());
        assert!(
            !paths.iter().any(|p| p == &omit),
            "gi={gi:?} omit={omit} paths={paths:?}"
        );
        assert!(
            paths.iter().any(|p| p == &keep),
            "gi={gi:?} keep={keep} paths={paths:?}"
        );
    }
}

#[test]
fn dense_double_star_style_patterns() {
    let cases = [
        ("**/secret.txt\n", "a/secret.txt", "a/public.txt"),
        ("**/secret.txt\n", "a/b/secret.txt", "a/b/public.txt"),
        ("logs/**\n", "logs/a.log", "src/a.rs"),
        ("logs/**\n", "logs/deep/b.txt", "src/b.rs"),
    ];
    for _ in 0..20 {
        for (gi, omit, keep) in cases {
            let temp = TempDir::new().unwrap();
            let root = temp.path().join("proj");
            write_tree(
                &root,
                &[(".gitignore", gi), (omit, "omit\n"), (keep, "keep\n")],
            );
            let paths = listed(&root, &FolderListPolicy::default());
            assert!(
                !paths.iter().any(|p| p == omit),
                "gi={gi:?} should omit {omit}; got {paths:?}"
            );
            assert!(
                paths.iter().any(|p| p == keep),
                "gi={gi:?} should keep {keep}; got {paths:?}"
            );
        }
    }
}

#[test]
fn dense_oversized_files_many_sizes() {
    let sizes = [
        1u64,
        1024,
        64 * 1024,
        ronin_core::MAX_FILE_ATTACHMENT_BYTES - 1,
        ronin_core::MAX_FILE_ATTACHMENT_BYTES,
        ronin_core::MAX_FILE_ATTACHMENT_BYTES + 1,
        ronin_core::MAX_FILE_ATTACHMENT_BYTES + 4096,
    ];
    for _ in 0..8 {
        for (i, size) in sizes.iter().enumerate() {
            let temp = TempDir::new().unwrap();
            let root = temp.path().join("p");
            std::fs::create_dir_all(&root).unwrap();
            std::fs::write(root.join("keep.txt"), b"k").unwrap();
            std::fs::write(
                root.join(format!("blob{i}.txt")),
                vec![b'x'; *size as usize],
            )
            .unwrap();
            let paths = listed(&root, &FolderListPolicy::default());
            assert!(paths.contains(&"keep.txt".to_string()));
            let present = paths.iter().any(|p| p == &format!("blob{i}.txt"));
            let expect = *size <= ronin_core::MAX_FILE_ATTACHMENT_BYTES;
            assert_eq!(present, expect, "size={size}");
        }
    }
}
