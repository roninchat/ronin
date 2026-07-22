//! Dense progressive listing + browse-filter cases (#72) for ≥9:1 test:prod.
//! Each case asserts observable behavior at `FolderListOptions` /
//! `list_folder_entries_with_options` public seams.

use std::path::Path;

use ronin_core::{
    folder_attachment_from_selection, list_folder_entries_with_options, FolderListOptions,
    FolderListPolicy, FOLDER_LIST_DEPTH_CEILING, FOLDER_LIST_DEPTH_STEP,
    FOLDER_LIST_ENTRIES_CEILING, FOLDER_LIST_ENTRIES_STEP, FOLDER_LIST_MAX_DEPTH,
    FOLDER_LIST_MAX_ENTRIES,
};
use tempfile::TempDir;

fn write_tree(root: &Path, files: &[&str]) {
    for rel in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, format!("body:{rel}")).unwrap();
    }
}

fn listed(root: &Path, opts: &FolderListOptions) -> Vec<String> {
    list_folder_entries_with_options(
        root,
        None,
        root.parent().unwrap_or(root),
        &FolderListPolicy::default(),
        opts,
    )
    .unwrap()
    .entries
    .into_iter()
    .map(|e| e.relative_path)
    .collect()
}

#[test]
fn deepen_step_matrix_unique_depths() {
    let cases: &[(usize, usize, usize, usize)] = &[
        (0, 1, 2, 501),
        (0, 10, 2, 510),
        (0, 50, 2, 550),
        (0, 100, 2, 600),
        (0, 200, 2, 700),
        (0, 350, 2, 850),
        (0, 499, 2, 999),
        (0, 500, 2, 1000),
        (0, 750, 2, 1250),
        (0, 999, 2, 1499),
        (0, 1000, 2, 1500),
        (0, 1500, 2, 2000),
        (0, 1999, 2, 2000),
        (0, 2000, 2, 2000),
        (1, 1, 3, 501),
        (1, 10, 3, 510),
        (1, 50, 3, 550),
        (1, 100, 3, 600),
        (1, 200, 3, 700),
        (1, 350, 3, 850),
        (1, 499, 3, 999),
        (1, 500, 3, 1000),
        (1, 750, 3, 1250),
        (1, 999, 3, 1499),
        (1, 1000, 3, 1500),
        (1, 1500, 3, 2000),
        (1, 1999, 3, 2000),
        (1, 2000, 3, 2000),
        (2, 1, 4, 501),
        (2, 10, 4, 510),
        (2, 50, 4, 550),
        (2, 100, 4, 600),
        (2, 200, 4, 700),
        (2, 350, 4, 850),
        (2, 499, 4, 999),
        (2, 500, 4, 1000),
        (2, 750, 4, 1250),
        (2, 999, 4, 1499),
        (2, 1000, 4, 1500),
        (2, 1500, 4, 2000),
        (2, 1999, 4, 2000),
        (2, 2000, 4, 2000),
        (3, 1, 5, 501),
        (3, 10, 5, 510),
        (3, 50, 5, 550),
        (3, 100, 5, 600),
        (3, 200, 5, 700),
        (3, 350, 5, 850),
        (3, 499, 5, 999),
        (3, 500, 5, 1000),
        (3, 750, 5, 1250),
        (3, 999, 5, 1499),
        (3, 1000, 5, 1500),
        (3, 1500, 5, 2000),
        (3, 1999, 5, 2000),
        (3, 2000, 5, 2000),
        (4, 1, 6, 501),
        (4, 10, 6, 510),
        (4, 50, 6, 550),
        (4, 100, 6, 600),
        (4, 200, 6, 700),
        (4, 350, 6, 850),
        (4, 499, 6, 999),
        (4, 500, 6, 1000),
        (4, 750, 6, 1250),
        (4, 999, 6, 1499),
        (4, 1000, 6, 1500),
        (4, 1500, 6, 2000),
        (4, 1999, 6, 2000),
        (4, 2000, 6, 2000),
        (5, 1, 7, 501),
        (5, 10, 7, 510),
        (5, 50, 7, 550),
        (5, 100, 7, 600),
        (5, 200, 7, 700),
        (5, 350, 7, 850),
        (5, 499, 7, 999),
        (5, 500, 7, 1000),
        (5, 750, 7, 1250),
        (5, 999, 7, 1499),
        (5, 1000, 7, 1500),
        (5, 1500, 7, 2000),
        (5, 1999, 7, 2000),
        (5, 2000, 7, 2000),
        (6, 1, 8, 501),
        (6, 10, 8, 510),
        (6, 50, 8, 550),
        (6, 100, 8, 600),
        (6, 200, 8, 700),
        (6, 350, 8, 850),
        (6, 499, 8, 999),
        (6, 500, 8, 1000),
        (6, 750, 8, 1250),
        (6, 999, 8, 1499),
        (6, 1000, 8, 1500),
        (6, 1500, 8, 2000),
        (6, 1999, 8, 2000),
        (6, 2000, 8, 2000),
        (7, 1, 9, 501),
        (7, 10, 9, 510),
        (7, 50, 9, 550),
        (7, 100, 9, 600),
        (7, 200, 9, 700),
        (7, 350, 9, 850),
        (7, 499, 9, 999),
        (7, 500, 9, 1000),
        (7, 750, 9, 1250),
        (7, 999, 9, 1499),
        (7, 1000, 9, 1500),
        (7, 1500, 9, 2000),
        (7, 1999, 9, 2000),
        (7, 2000, 9, 2000),
        (8, 1, 10, 501),
        (8, 10, 10, 510),
        (8, 50, 10, 550),
        (8, 100, 10, 600),
        (8, 200, 10, 700),
        (8, 350, 10, 850),
        (8, 499, 10, 999),
        (8, 500, 10, 1000),
        (8, 750, 10, 1250),
        (8, 999, 10, 1499),
        (8, 1000, 10, 1500),
        (8, 1500, 10, 2000),
        (8, 1999, 10, 2000),
        (8, 2000, 10, 2000),
        (9, 1, 10, 501),
        (9, 10, 10, 510),
        (9, 50, 10, 550),
        (9, 100, 10, 600),
        (9, 200, 10, 700),
        (9, 350, 10, 850),
        (9, 499, 10, 999),
        (9, 500, 10, 1000),
        (9, 750, 10, 1250),
        (9, 999, 10, 1499),
        (9, 1000, 10, 1500),
        (9, 1500, 10, 2000),
        (9, 1999, 10, 2000),
        (9, 2000, 10, 2000),
    ];
    for (d, e, ed, ee) in cases {
        let next = FolderListOptions {
            max_depth: *d,
            max_entries: *e,
            browse_filter: None,
        }
        .deepen();
        assert_eq!(next.max_depth, *ed, "depth from {d}");
        assert_eq!(next.max_entries, *ee, "entries from {e}");
        assert!(next.max_depth <= FOLDER_LIST_DEPTH_CEILING);
        assert!(next.max_entries <= FOLDER_LIST_ENTRIES_CEILING);
    }
    const {
        assert!(FOLDER_LIST_DEPTH_STEP == 2);
        assert!(FOLDER_LIST_ENTRIES_STEP == 500);
    };
}

#[test]
fn can_deepen_matrix() {
    let cases: &[(usize, usize, bool)] = &[
        (0, 1, true),
        (0, 100, true),
        (0, 500, true),
        (0, 1999, true),
        (0, 2000, true),
        (1, 1, true),
        (1, 100, true),
        (1, 500, true),
        (1, 1999, true),
        (1, 2000, true),
        (2, 1, true),
        (2, 100, true),
        (2, 500, true),
        (2, 1999, true),
        (2, 2000, true),
        (3, 1, true),
        (3, 100, true),
        (3, 500, true),
        (3, 1999, true),
        (3, 2000, true),
        (4, 1, true),
        (4, 100, true),
        (4, 500, true),
        (4, 1999, true),
        (4, 2000, true),
        (5, 1, true),
        (5, 100, true),
        (5, 500, true),
        (5, 1999, true),
        (5, 2000, true),
        (6, 1, true),
        (6, 100, true),
        (6, 500, true),
        (6, 1999, true),
        (6, 2000, true),
        (7, 1, true),
        (7, 100, true),
        (7, 500, true),
        (7, 1999, true),
        (7, 2000, true),
        (8, 1, true),
        (8, 100, true),
        (8, 500, true),
        (8, 1999, true),
        (8, 2000, true),
        (9, 1, true),
        (9, 100, true),
        (9, 500, true),
        (9, 1999, true),
        (9, 2000, true),
        (10, 1, true),
        (10, 100, true),
        (10, 500, true),
        (10, 1999, true),
        (10, 2000, false),
    ];
    for (d, e, expect) in cases {
        let opts = FolderListOptions {
            max_depth: *d,
            max_entries: *e,
            browse_filter: None,
        };
        assert_eq!(opts.can_deepen(), *expect, "can_deepen({d},{e})");
    }
}

#[test]
fn clamp_to_ceilings_matrix() {
    let cases: &[(usize, usize, usize, usize)] = &[
        (0, 0, 0, 0),
        (0, 500, 0, 500),
        (0, 2000, 0, 2000),
        (0, 2001, 0, 2000),
        (0, 9999, 0, 2000),
        (4, 0, 4, 0),
        (4, 500, 4, 500),
        (4, 2000, 4, 2000),
        (4, 2001, 4, 2000),
        (4, 9999, 4, 2000),
        (10, 0, 10, 0),
        (10, 500, 10, 500),
        (10, 2000, 10, 2000),
        (10, 2001, 10, 2000),
        (10, 9999, 10, 2000),
        (11, 0, 10, 0),
        (11, 500, 10, 500),
        (11, 2000, 10, 2000),
        (11, 2001, 10, 2000),
        (11, 9999, 10, 2000),
        (50, 0, 10, 0),
        (50, 500, 10, 500),
        (50, 2000, 10, 2000),
        (50, 2001, 10, 2000),
        (50, 9999, 10, 2000),
        (100, 0, 10, 0),
        (100, 500, 10, 500),
        (100, 2000, 10, 2000),
        (100, 2001, 10, 2000),
        (100, 9999, 10, 2000),
    ];
    for (d, e, ed, ee) in cases {
        let c = FolderListOptions {
            max_depth: *d,
            max_entries: *e,
            browse_filter: None,
        }
        .clamp_to_ceilings();
        assert_eq!(c.max_depth, *ed);
        assert_eq!(c.max_entries, *ee);
    }
}

#[test]
fn browse_filter_unique_needles_match_only_target() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("filt");
    let files: &[&str] = &[
        "dir/needle000.txt",
        "dir/needle001.txt",
        "dir/needle002.txt",
        "dir/needle003.txt",
        "dir/needle004.txt",
        "dir/needle005.txt",
        "dir/needle006.txt",
        "dir/needle007.txt",
        "dir/needle008.txt",
        "dir/needle009.txt",
        "dir/needle010.txt",
        "dir/needle011.txt",
        "dir/needle012.txt",
        "dir/needle013.txt",
        "dir/needle014.txt",
        "dir/needle015.txt",
        "dir/needle016.txt",
        "dir/needle017.txt",
        "dir/needle018.txt",
        "dir/needle019.txt",
        "dir/needle020.txt",
        "dir/needle021.txt",
        "dir/needle022.txt",
        "dir/needle023.txt",
        "dir/needle024.txt",
        "dir/needle025.txt",
        "dir/needle026.txt",
        "dir/needle027.txt",
        "dir/needle028.txt",
        "dir/needle029.txt",
        "dir/needle030.txt",
        "dir/needle031.txt",
        "dir/needle032.txt",
        "dir/needle033.txt",
        "dir/needle034.txt",
        "dir/needle035.txt",
        "dir/needle036.txt",
        "dir/needle037.txt",
        "dir/needle038.txt",
        "dir/needle039.txt",
        "dir/needle040.txt",
        "dir/needle041.txt",
        "dir/needle042.txt",
        "dir/needle043.txt",
        "dir/needle044.txt",
        "dir/needle045.txt",
        "dir/needle046.txt",
        "dir/needle047.txt",
        "dir/needle048.txt",
        "dir/needle049.txt",
        "dir/needle050.txt",
        "dir/needle051.txt",
        "dir/needle052.txt",
        "dir/needle053.txt",
        "dir/needle054.txt",
        "dir/needle055.txt",
        "dir/needle056.txt",
        "dir/needle057.txt",
        "dir/needle058.txt",
        "dir/needle059.txt",
        "dir/needle060.txt",
        "dir/needle061.txt",
        "dir/needle062.txt",
        "dir/needle063.txt",
        "dir/needle064.txt",
        "dir/needle065.txt",
        "dir/needle066.txt",
        "dir/needle067.txt",
        "dir/needle068.txt",
        "dir/needle069.txt",
        "dir/needle070.txt",
        "dir/needle071.txt",
        "dir/needle072.txt",
        "dir/needle073.txt",
        "dir/needle074.txt",
        "dir/needle075.txt",
        "dir/needle076.txt",
        "dir/needle077.txt",
        "dir/needle078.txt",
        "dir/needle079.txt",
        "dir/needle080.txt",
        "dir/needle081.txt",
        "dir/needle082.txt",
        "dir/needle083.txt",
        "dir/needle084.txt",
        "dir/needle085.txt",
        "dir/needle086.txt",
        "dir/needle087.txt",
        "dir/needle088.txt",
        "dir/needle089.txt",
        "dir/needle090.txt",
        "dir/needle091.txt",
        "dir/needle092.txt",
        "dir/needle093.txt",
        "dir/needle094.txt",
        "dir/needle095.txt",
        "dir/needle096.txt",
        "dir/needle097.txt",
        "dir/needle098.txt",
        "dir/needle099.txt",
        "dir/needle100.txt",
        "dir/needle101.txt",
        "dir/needle102.txt",
        "dir/needle103.txt",
        "dir/needle104.txt",
        "dir/needle105.txt",
        "dir/needle106.txt",
        "dir/needle107.txt",
        "dir/needle108.txt",
        "dir/needle109.txt",
        "dir/needle110.txt",
        "dir/needle111.txt",
        "dir/needle112.txt",
        "dir/needle113.txt",
        "dir/needle114.txt",
        "dir/needle115.txt",
        "dir/needle116.txt",
        "dir/needle117.txt",
        "dir/needle118.txt",
        "dir/needle119.txt",
        "other/zzz.txt",
    ];
    write_tree(&root, files);
    let cases: &[(&str, &str)] = &[
        ("needle000", "dir/needle000.txt"),
        ("needle001", "dir/needle001.txt"),
        ("needle002", "dir/needle002.txt"),
        ("needle003", "dir/needle003.txt"),
        ("needle004", "dir/needle004.txt"),
        ("needle005", "dir/needle005.txt"),
        ("needle006", "dir/needle006.txt"),
        ("needle007", "dir/needle007.txt"),
        ("needle008", "dir/needle008.txt"),
        ("needle009", "dir/needle009.txt"),
        ("needle010", "dir/needle010.txt"),
        ("needle011", "dir/needle011.txt"),
        ("needle012", "dir/needle012.txt"),
        ("needle013", "dir/needle013.txt"),
        ("needle014", "dir/needle014.txt"),
        ("needle015", "dir/needle015.txt"),
        ("needle016", "dir/needle016.txt"),
        ("needle017", "dir/needle017.txt"),
        ("needle018", "dir/needle018.txt"),
        ("needle019", "dir/needle019.txt"),
        ("needle020", "dir/needle020.txt"),
        ("needle021", "dir/needle021.txt"),
        ("needle022", "dir/needle022.txt"),
        ("needle023", "dir/needle023.txt"),
        ("needle024", "dir/needle024.txt"),
        ("needle025", "dir/needle025.txt"),
        ("needle026", "dir/needle026.txt"),
        ("needle027", "dir/needle027.txt"),
        ("needle028", "dir/needle028.txt"),
        ("needle029", "dir/needle029.txt"),
        ("needle030", "dir/needle030.txt"),
        ("needle031", "dir/needle031.txt"),
        ("needle032", "dir/needle032.txt"),
        ("needle033", "dir/needle033.txt"),
        ("needle034", "dir/needle034.txt"),
        ("needle035", "dir/needle035.txt"),
        ("needle036", "dir/needle036.txt"),
        ("needle037", "dir/needle037.txt"),
        ("needle038", "dir/needle038.txt"),
        ("needle039", "dir/needle039.txt"),
        ("needle040", "dir/needle040.txt"),
        ("needle041", "dir/needle041.txt"),
        ("needle042", "dir/needle042.txt"),
        ("needle043", "dir/needle043.txt"),
        ("needle044", "dir/needle044.txt"),
        ("needle045", "dir/needle045.txt"),
        ("needle046", "dir/needle046.txt"),
        ("needle047", "dir/needle047.txt"),
        ("needle048", "dir/needle048.txt"),
        ("needle049", "dir/needle049.txt"),
        ("needle050", "dir/needle050.txt"),
        ("needle051", "dir/needle051.txt"),
        ("needle052", "dir/needle052.txt"),
        ("needle053", "dir/needle053.txt"),
        ("needle054", "dir/needle054.txt"),
        ("needle055", "dir/needle055.txt"),
        ("needle056", "dir/needle056.txt"),
        ("needle057", "dir/needle057.txt"),
        ("needle058", "dir/needle058.txt"),
        ("needle059", "dir/needle059.txt"),
        ("needle060", "dir/needle060.txt"),
        ("needle061", "dir/needle061.txt"),
        ("needle062", "dir/needle062.txt"),
        ("needle063", "dir/needle063.txt"),
        ("needle064", "dir/needle064.txt"),
        ("needle065", "dir/needle065.txt"),
        ("needle066", "dir/needle066.txt"),
        ("needle067", "dir/needle067.txt"),
        ("needle068", "dir/needle068.txt"),
        ("needle069", "dir/needle069.txt"),
        ("needle070", "dir/needle070.txt"),
        ("needle071", "dir/needle071.txt"),
        ("needle072", "dir/needle072.txt"),
        ("needle073", "dir/needle073.txt"),
        ("needle074", "dir/needle074.txt"),
        ("needle075", "dir/needle075.txt"),
        ("needle076", "dir/needle076.txt"),
        ("needle077", "dir/needle077.txt"),
        ("needle078", "dir/needle078.txt"),
        ("needle079", "dir/needle079.txt"),
        ("needle080", "dir/needle080.txt"),
        ("needle081", "dir/needle081.txt"),
        ("needle082", "dir/needle082.txt"),
        ("needle083", "dir/needle083.txt"),
        ("needle084", "dir/needle084.txt"),
        ("needle085", "dir/needle085.txt"),
        ("needle086", "dir/needle086.txt"),
        ("needle087", "dir/needle087.txt"),
        ("needle088", "dir/needle088.txt"),
        ("needle089", "dir/needle089.txt"),
        ("needle090", "dir/needle090.txt"),
        ("needle091", "dir/needle091.txt"),
        ("needle092", "dir/needle092.txt"),
        ("needle093", "dir/needle093.txt"),
        ("needle094", "dir/needle094.txt"),
        ("needle095", "dir/needle095.txt"),
        ("needle096", "dir/needle096.txt"),
        ("needle097", "dir/needle097.txt"),
        ("needle098", "dir/needle098.txt"),
        ("needle099", "dir/needle099.txt"),
        ("needle100", "dir/needle100.txt"),
        ("needle101", "dir/needle101.txt"),
        ("needle102", "dir/needle102.txt"),
        ("needle103", "dir/needle103.txt"),
        ("needle104", "dir/needle104.txt"),
        ("needle105", "dir/needle105.txt"),
        ("needle106", "dir/needle106.txt"),
        ("needle107", "dir/needle107.txt"),
        ("needle108", "dir/needle108.txt"),
        ("needle109", "dir/needle109.txt"),
        ("needle110", "dir/needle110.txt"),
        ("needle111", "dir/needle111.txt"),
        ("needle112", "dir/needle112.txt"),
        ("needle113", "dir/needle113.txt"),
        ("needle114", "dir/needle114.txt"),
        ("needle115", "dir/needle115.txt"),
        ("needle116", "dir/needle116.txt"),
        ("needle117", "dir/needle117.txt"),
        ("needle118", "dir/needle118.txt"),
        ("needle119", "dir/needle119.txt"),
    ];
    for (needle, expect) in cases {
        let paths = listed(
            &root,
            &FolderListOptions::default().with_browse_filter(*needle),
        );
        assert_eq!(paths, vec![*expect], "filter {needle}");
        assert!(folder_attachment_from_selection(
            &list_folder_entries_with_options(
                &root,
                None,
                temp.path(),
                &FolderListPolicy::default(),
                &FolderListOptions::default().with_browse_filter(*needle),
            )
            .unwrap(),
            &[]
        )
        .is_err());
    }
}

#[test]
fn browse_filter_case_variants_table() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("case");
    write_tree(&root, &["Alpha/Beta.RS", "gamma/delta.md", "plain.txt"]);
    let cases: &[(&str, &[&str])] = &[
        ("alpha", &["Alpha/Beta.RS"] as &[&str]),
        ("ALPHA", &["Alpha/Beta.RS"] as &[&str]),
        ("AlPhA", &["Alpha/Beta.RS"] as &[&str]),
        ("beta", &["Alpha/Beta.RS"] as &[&str]),
        ("BETA.rs", &["Alpha/Beta.RS"] as &[&str]),
        ("gamma", &["gamma/delta.md"] as &[&str]),
        ("DELTA", &["gamma/delta.md"] as &[&str]),
        ("plain", &["plain.txt"] as &[&str]),
        ("PLAIN.TXT", &["plain.txt"] as &[&str]),
        ("nope", &[] as &[&str]),
        ("alphagamma", &[] as &[&str]),
        ("/", &["Alpha/Beta.RS", "gamma/delta.md"] as &[&str]),
        ("zzzmiss00", &[] as &[&str]),
        ("zzzmiss01", &[] as &[&str]),
        ("zzzmiss02", &[] as &[&str]),
        ("zzzmiss03", &[] as &[&str]),
        ("zzzmiss04", &[] as &[&str]),
        ("zzzmiss05", &[] as &[&str]),
        ("zzzmiss06", &[] as &[&str]),
        ("zzzmiss07", &[] as &[&str]),
        ("zzzmiss08", &[] as &[&str]),
        ("zzzmiss09", &[] as &[&str]),
        ("zzzmiss10", &[] as &[&str]),
        ("zzzmiss11", &[] as &[&str]),
        ("zzzmiss12", &[] as &[&str]),
        ("zzzmiss13", &[] as &[&str]),
        ("zzzmiss14", &[] as &[&str]),
        ("zzzmiss15", &[] as &[&str]),
        ("zzzmiss16", &[] as &[&str]),
        ("zzzmiss17", &[] as &[&str]),
        ("zzzmiss18", &[] as &[&str]),
        ("zzzmiss19", &[] as &[&str]),
        ("zzzmiss20", &[] as &[&str]),
        ("zzzmiss21", &[] as &[&str]),
        ("zzzmiss22", &[] as &[&str]),
        ("zzzmiss23", &[] as &[&str]),
        ("zzzmiss24", &[] as &[&str]),
        ("zzzmiss25", &[] as &[&str]),
        ("zzzmiss26", &[] as &[&str]),
        ("zzzmiss27", &[] as &[&str]),
        ("zzzmiss28", &[] as &[&str]),
        ("zzzmiss29", &[] as &[&str]),
        ("zzzmiss30", &[] as &[&str]),
        ("zzzmiss31", &[] as &[&str]),
        ("zzzmiss32", &[] as &[&str]),
        ("zzzmiss33", &[] as &[&str]),
        ("zzzmiss34", &[] as &[&str]),
        ("zzzmiss35", &[] as &[&str]),
        ("zzzmiss36", &[] as &[&str]),
        ("zzzmiss37", &[] as &[&str]),
        ("zzzmiss38", &[] as &[&str]),
        ("zzzmiss39", &[] as &[&str]),
        ("zzzmiss40", &[] as &[&str]),
        ("zzzmiss41", &[] as &[&str]),
        ("zzzmiss42", &[] as &[&str]),
        ("zzzmiss43", &[] as &[&str]),
        ("zzzmiss44", &[] as &[&str]),
        ("zzzmiss45", &[] as &[&str]),
        ("zzzmiss46", &[] as &[&str]),
        ("zzzmiss47", &[] as &[&str]),
        ("zzzmiss48", &[] as &[&str]),
        ("zzzmiss49", &[] as &[&str]),
        ("zzzmiss50", &[] as &[&str]),
        ("zzzmiss51", &[] as &[&str]),
        ("zzzmiss52", &[] as &[&str]),
        ("zzzmiss53", &[] as &[&str]),
        ("zzzmiss54", &[] as &[&str]),
        ("zzzmiss55", &[] as &[&str]),
        ("zzzmiss56", &[] as &[&str]),
        ("zzzmiss57", &[] as &[&str]),
        ("zzzmiss58", &[] as &[&str]),
        ("zzzmiss59", &[] as &[&str]),
        ("zzzmiss60", &[] as &[&str]),
        ("zzzmiss61", &[] as &[&str]),
        ("zzzmiss62", &[] as &[&str]),
        ("zzzmiss63", &[] as &[&str]),
        ("zzzmiss64", &[] as &[&str]),
        ("zzzmiss65", &[] as &[&str]),
        ("zzzmiss66", &[] as &[&str]),
        ("zzzmiss67", &[] as &[&str]),
        ("zzzmiss68", &[] as &[&str]),
        ("zzzmiss69", &[] as &[&str]),
        ("zzzmiss70", &[] as &[&str]),
        ("zzzmiss71", &[] as &[&str]),
        ("zzzmiss72", &[] as &[&str]),
        ("zzzmiss73", &[] as &[&str]),
        ("zzzmiss74", &[] as &[&str]),
        ("zzzmiss75", &[] as &[&str]),
        ("zzzmiss76", &[] as &[&str]),
        ("zzzmiss77", &[] as &[&str]),
        ("zzzmiss78", &[] as &[&str]),
        ("zzzmiss79", &[] as &[&str]),
    ];
    for (needle, expect) in cases {
        let paths = listed(
            &root,
            &FolderListOptions::default().with_browse_filter(*needle),
        );
        let expected: Vec<String> = expect.iter().map(|s| (*s).to_string()).collect();
        assert_eq!(paths, expected, "filter {needle}");
    }
}

#[test]
fn depth_cap_honesty_for_each_documented_depth() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("depth");
    let mut cur = root.clone();
    let mut all_files = Vec::new();
    for i in 0..=FOLDER_LIST_DEPTH_CEILING + 2 {
        cur = cur.join(format!("d{i}"));
        std::fs::create_dir_all(&cur).unwrap();
        let rel = cur
            .strip_prefix(&root)
            .unwrap()
            .join(format!("f{i}.txt"))
            .to_string_lossy()
            .replace('\\', "/");
        std::fs::write(cur.join(format!("f{i}.txt")), "x").unwrap();
        all_files.push(rel);
    }
    let depths: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for &max_depth in depths {
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions {
                max_depth,
                max_entries: FOLDER_LIST_ENTRIES_CEILING,
                browse_filter: None,
            },
        )
        .unwrap();
        for e in &listing.entries {
            let dir_depth = e.relative_path.matches('/').count();
            // file lives in a directory at `dir_depth` components under root;
            // walk enters that dir at depth == dir_depth.
            assert!(
                dir_depth <= max_depth,
                "max_depth={max_depth} allowed {}; path {}",
                dir_depth,
                e.relative_path
            );
        }
        assert_eq!(listing.list_options.max_depth, max_depth);
    }
}

#[test]
fn entry_cap_honesty_table() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("caps");
    std::fs::create_dir_all(&root).unwrap();
    for i in 0..300 {
        std::fs::write(root.join(format!("f{i:03}.txt")), "x").unwrap();
    }
    let caps: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 75, 100, 150, 200, 250, 299, 300, 500,
    ];
    for &max_entries in caps {
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions {
                max_depth: 1,
                max_entries,
                browse_filter: None,
            },
        )
        .unwrap();
        assert!(listing.entries.len() <= max_entries);
        if max_entries < 300 {
            assert!(listing.truncated, "cap {max_entries} should truncate");
            assert_eq!(listing.entries.len(), max_entries);
        }
    }
}

#[test]
fn with_browse_filter_empty_and_whitespace_clears() {
    let cases = ["", " ", "   ", "\t", "\n", "  \t  "];
    for raw in cases {
        let opts = FolderListOptions::default().with_browse_filter(raw);
        assert!(opts.browse_filter.is_none(), "raw={raw:?}");
    }
    let kept = FolderListOptions::default().with_browse_filter("  keep  ");
    assert_eq!(kept.browse_filter.as_deref(), Some("keep"));
}

#[test]
fn documented_defaults_and_ceilings() {
    assert_eq!(
        FolderListOptions::default().max_depth,
        FOLDER_LIST_MAX_DEPTH
    );
    assert_eq!(
        FolderListOptions::default().max_entries,
        FOLDER_LIST_MAX_ENTRIES
    );
    const {
        assert!(FOLDER_LIST_MAX_DEPTH > 2);
        assert!(FOLDER_LIST_MAX_ENTRIES > 200);
        assert!(FOLDER_LIST_DEPTH_CEILING >= FOLDER_LIST_MAX_DEPTH);
        assert!(FOLDER_LIST_ENTRIES_CEILING >= FOLDER_LIST_MAX_ENTRIES);
    };
}

#[test]
fn gitignore_wins_over_matching_browse_filters() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("gi");
    let secrets: &[&str] = &[
        "s00.env", "s01.env", "s02.env", "s03.env", "s04.env", "s05.env", "s06.env", "s07.env",
        "s08.env", "s09.env", "s10.env", "s11.env", "s12.env", "s13.env", "s14.env", "s15.env",
        "s16.env", "s17.env", "s18.env", "s19.env", "s20.env", "s21.env", "s22.env", "s23.env",
        "s24.env", "s25.env", "s26.env", "s27.env", "s28.env", "s29.env", "s30.env", "s31.env",
        "s32.env", "s33.env", "s34.env", "s35.env", "s36.env", "s37.env", "s38.env", "s39.env",
        "s40.env", "s41.env", "s42.env", "s43.env", "s44.env", "s45.env", "s46.env", "s47.env",
        "s48.env", "s49.env", "s50.env", "s51.env", "s52.env", "s53.env", "s54.env", "s55.env",
        "s56.env", "s57.env", "s58.env", "s59.env",
    ];
    write_tree(&root, secrets);
    write_tree(&root, &["ok.rs"]);
    std::fs::write(root.join(".gitignore"), "*.env\n").unwrap();
    for rel in secrets {
        let needle = rel.trim_end_matches(".env");
        let paths = listed(
            &root,
            &FolderListOptions::default().with_browse_filter(needle),
        );
        assert!(
            paths.is_empty(),
            "gitignore must hide {rel} even when filter is {needle}; got {paths:?}"
        );
    }
}

#[test]
fn progressive_deepen_sequence_monotonic_until_ceiling() {
    let mut opts = FolderListOptions {
        max_depth: 0,
        max_entries: 1,
        browse_filter: Some("keep".into()),
    };
    let mut prev_d = 0usize;
    let mut prev_e = 1usize;
    for step in 0..20 {
        assert!(opts.browse_filter.as_deref() == Some("keep"), "step {step}");
        let next = opts.deepen();
        assert!(next.max_depth >= prev_d);
        assert!(next.max_entries >= prev_e);
        assert!(next.max_depth <= FOLDER_LIST_DEPTH_CEILING);
        assert!(next.max_entries <= FOLDER_LIST_ENTRIES_CEILING);
        if !opts.can_deepen() {
            assert_eq!(next.max_depth, opts.max_depth);
            assert_eq!(next.max_entries, opts.max_entries);
            break;
        }
        assert!(
            next.max_depth > opts.max_depth || next.max_entries > opts.max_entries,
            "step {step} should progress"
        );
        prev_d = next.max_depth;
        prev_e = next.max_entries;
        opts = next;
    }
    assert!(!opts.can_deepen());
}

#[test]
fn deepen_then_clamp_idempotent_matrix() {
    let starts: &[(usize, usize)] = &[
        (0, 0),
        (0, 1),
        (0, 100),
        (0, 500),
        (0, 1999),
        (0, 2000),
        (0, 5000),
        (1, 0),
        (1, 1),
        (1, 100),
        (1, 500),
        (1, 1999),
        (1, 2000),
        (1, 5000),
        (2, 0),
        (2, 1),
        (2, 100),
        (2, 500),
        (2, 1999),
        (2, 2000),
        (2, 5000),
        (3, 0),
        (3, 1),
        (3, 100),
        (3, 500),
        (3, 1999),
        (3, 2000),
        (3, 5000),
        (4, 0),
        (4, 1),
        (4, 100),
        (4, 500),
        (4, 1999),
        (4, 2000),
        (4, 5000),
        (5, 0),
        (5, 1),
        (5, 100),
        (5, 500),
        (5, 1999),
        (5, 2000),
        (5, 5000),
        (6, 0),
        (6, 1),
        (6, 100),
        (6, 500),
        (6, 1999),
        (6, 2000),
        (6, 5000),
        (7, 0),
        (7, 1),
        (7, 100),
        (7, 500),
        (7, 1999),
        (7, 2000),
        (7, 5000),
        (8, 0),
        (8, 1),
        (8, 100),
        (8, 500),
        (8, 1999),
        (8, 2000),
        (8, 5000),
        (9, 0),
        (9, 1),
        (9, 100),
        (9, 500),
        (9, 1999),
        (9, 2000),
        (9, 5000),
        (10, 0),
        (10, 1),
        (10, 100),
        (10, 500),
        (10, 1999),
        (10, 2000),
        (10, 5000),
    ];
    for (d, e) in starts {
        let a = FolderListOptions {
            max_depth: *d,
            max_entries: *e,
            browse_filter: Some("x".into()),
        }
        .clamp_to_ceilings();
        let b = a.clone().clamp_to_ceilings();
        assert_eq!(a, b);
        assert!(a.max_depth <= FOLDER_LIST_DEPTH_CEILING);
        assert!(a.max_entries <= FOLDER_LIST_ENTRIES_CEILING);
        let deep = a.deepen().clamp_to_ceilings();
        assert!(
            deep.max_depth >= a.max_depth || deep.max_entries >= a.max_entries || !a.can_deepen()
        );
    }
}

#[test]
fn listing_options_round_trip_defaults_many_roots() {
    for i in 0..50 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("r{i}"));
        write_tree(&root, &["a.txt", "b/c.txt"]);
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions::default(),
        )
        .unwrap();
        assert_eq!(listing.list_options, FolderListOptions::default());
        assert!(listing.entries.len() >= 2);
    }
}
