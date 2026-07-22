//! Table-driven progressive folder options cases (#72).

use std::path::Path;

use ronin_core::{
    list_folder_entries_with_options, FolderListOptions, FolderListPolicy,
    FOLDER_LIST_DEPTH_CEILING, FOLDER_LIST_ENTRIES_CEILING,
};
use tempfile::TempDir;

fn write_files(root: &Path, n: usize, prefix: &str) {
    std::fs::create_dir_all(root).unwrap();
    for i in 0..n {
        std::fs::write(root.join(format!("{prefix}{i:04}.txt")), "x").unwrap();
    }
}

#[test]
fn options_deepen_preserves_filter_across_many_filters() {
    let filters: &[&str] = &[
        "filt_000", "filt_001", "filt_002", "filt_003", "filt_004", "filt_005", "filt_006",
        "filt_007", "filt_008", "filt_009", "filt_010", "filt_011", "filt_012", "filt_013",
        "filt_014", "filt_015", "filt_016", "filt_017", "filt_018", "filt_019", "filt_020",
        "filt_021", "filt_022", "filt_023", "filt_024", "filt_025", "filt_026", "filt_027",
        "filt_028", "filt_029", "filt_030", "filt_031", "filt_032", "filt_033", "filt_034",
        "filt_035", "filt_036", "filt_037", "filt_038", "filt_039", "filt_040", "filt_041",
        "filt_042", "filt_043", "filt_044", "filt_045", "filt_046", "filt_047", "filt_048",
        "filt_049", "filt_050", "filt_051", "filt_052", "filt_053", "filt_054", "filt_055",
        "filt_056", "filt_057", "filt_058", "filt_059", "filt_060", "filt_061", "filt_062",
        "filt_063", "filt_064", "filt_065", "filt_066", "filt_067", "filt_068", "filt_069",
        "filt_070", "filt_071", "filt_072", "filt_073", "filt_074", "filt_075", "filt_076",
        "filt_077", "filt_078", "filt_079", "filt_080", "filt_081", "filt_082", "filt_083",
        "filt_084", "filt_085", "filt_086", "filt_087", "filt_088", "filt_089", "filt_090",
        "filt_091", "filt_092", "filt_093", "filt_094", "filt_095", "filt_096", "filt_097",
        "filt_098", "filt_099", "filt_100", "filt_101", "filt_102", "filt_103", "filt_104",
        "filt_105", "filt_106", "filt_107", "filt_108", "filt_109", "filt_110", "filt_111",
        "filt_112", "filt_113", "filt_114", "filt_115", "filt_116", "filt_117", "filt_118",
        "filt_119", "filt_120", "filt_121", "filt_122", "filt_123", "filt_124", "filt_125",
        "filt_126", "filt_127", "filt_128", "filt_129", "filt_130", "filt_131", "filt_132",
        "filt_133", "filt_134", "filt_135", "filt_136", "filt_137", "filt_138", "filt_139",
        "filt_140", "filt_141", "filt_142", "filt_143", "filt_144", "filt_145", "filt_146",
        "filt_147", "filt_148", "filt_149",
    ];
    for f in filters {
        let opts = FolderListOptions {
            max_depth: 3,
            max_entries: 100,
            browse_filter: Some((*f).into()),
        };
        let next = opts.deepen();
        assert_eq!(next.browse_filter.as_deref(), Some(*f));
        assert_eq!(next.max_depth, 5);
        assert_eq!(next.max_entries, 600);
    }
}

#[test]
fn listing_respects_paired_depth_and_entry_caps() {
    let pairs: &[(usize, usize)] = &[
        (0, 1),
        (0, 5),
        (0, 10),
        (0, 25),
        (0, 50),
        (0, 100),
        (0, 250),
        (0, 500),
        (0, 1000),
        (0, 2000),
        (1, 1),
        (1, 5),
        (1, 10),
        (1, 25),
        (1, 50),
        (1, 100),
        (1, 250),
        (1, 500),
        (1, 1000),
        (1, 2000),
        (2, 1),
        (2, 5),
        (2, 10),
        (2, 25),
        (2, 50),
        (2, 100),
        (2, 250),
        (2, 500),
        (2, 1000),
        (2, 2000),
        (3, 1),
        (3, 5),
        (3, 10),
        (3, 25),
        (3, 50),
        (3, 100),
        (3, 250),
        (3, 500),
        (3, 1000),
        (3, 2000),
        (4, 1),
        (4, 5),
        (4, 10),
        (4, 25),
        (4, 50),
        (4, 100),
        (4, 250),
        (4, 500),
        (4, 1000),
        (4, 2000),
        (5, 1),
        (5, 5),
        (5, 10),
        (5, 25),
        (5, 50),
        (5, 100),
        (5, 250),
        (5, 500),
        (5, 1000),
        (5, 2000),
        (6, 1),
        (6, 5),
        (6, 10),
        (6, 25),
        (6, 50),
        (6, 100),
        (6, 250),
        (6, 500),
        (6, 1000),
        (6, 2000),
        (7, 1),
        (7, 5),
        (7, 10),
        (7, 25),
        (7, 50),
        (7, 100),
        (7, 250),
        (7, 500),
        (7, 1000),
        (7, 2000),
        (8, 1),
        (8, 5),
        (8, 10),
        (8, 25),
        (8, 50),
        (8, 100),
        (8, 250),
        (8, 500),
        (8, 1000),
        (8, 2000),
        (9, 1),
        (9, 5),
        (9, 10),
        (9, 25),
        (9, 50),
        (9, 100),
        (9, 250),
        (9, 500),
        (9, 1000),
        (9, 2000),
        (10, 1),
        (10, 5),
        (10, 10),
        (10, 25),
        (10, 50),
        (10, 100),
        (10, 250),
        (10, 500),
        (10, 1000),
        (10, 2000),
    ];
    for (max_depth, max_entries) in pairs {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        // build depth chain
        let mut cur = root.clone();
        for i in 0..=FOLDER_LIST_DEPTH_CEILING {
            cur = cur.join(format!("d{i}"));
            std::fs::create_dir_all(&cur).unwrap();
            std::fs::write(cur.join(format!("f{i}.txt")), "x").unwrap();
        }
        write_files(&root, (*max_entries).saturating_add(30), "top");
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions {
                max_depth: *max_depth,
                max_entries: *max_entries,
                browse_filter: None,
            },
        )
        .unwrap();
        assert!(listing.entries.len() <= *max_entries);
        assert_eq!(
            listing.list_options.max_depth,
            (*max_depth).min(FOLDER_LIST_DEPTH_CEILING)
        );
        assert_eq!(
            listing.list_options.max_entries,
            (*max_entries).min(FOLDER_LIST_ENTRIES_CEILING)
        );
        for e in &listing.entries {
            assert!(e.relative_path.matches('/').count() <= *max_depth);
        }
    }
}

#[test]
fn browse_filter_prefix_table_unique() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("pref");
    std::fs::create_dir_all(root.join("src")).unwrap();
    let prefixes: &[&str] = &[
        "pref000", "pref001", "pref002", "pref003", "pref004", "pref005", "pref006", "pref007",
        "pref008", "pref009", "pref010", "pref011", "pref012", "pref013", "pref014", "pref015",
        "pref016", "pref017", "pref018", "pref019", "pref020", "pref021", "pref022", "pref023",
        "pref024", "pref025", "pref026", "pref027", "pref028", "pref029", "pref030", "pref031",
        "pref032", "pref033", "pref034", "pref035", "pref036", "pref037", "pref038", "pref039",
        "pref040", "pref041", "pref042", "pref043", "pref044", "pref045", "pref046", "pref047",
        "pref048", "pref049", "pref050", "pref051", "pref052", "pref053", "pref054", "pref055",
        "pref056", "pref057", "pref058", "pref059", "pref060", "pref061", "pref062", "pref063",
        "pref064", "pref065", "pref066", "pref067", "pref068", "pref069", "pref070", "pref071",
        "pref072", "pref073", "pref074", "pref075", "pref076", "pref077", "pref078", "pref079",
        "pref080", "pref081", "pref082", "pref083", "pref084", "pref085", "pref086", "pref087",
        "pref088", "pref089", "pref090", "pref091", "pref092", "pref093", "pref094", "pref095",
        "pref096", "pref097", "pref098", "pref099", "pref100", "pref101", "pref102", "pref103",
        "pref104", "pref105", "pref106", "pref107", "pref108", "pref109", "pref110", "pref111",
        "pref112", "pref113", "pref114", "pref115", "pref116", "pref117", "pref118", "pref119",
    ];
    for p in prefixes {
        std::fs::write(root.join("src").join(format!("{p}.rs")), "x").unwrap();
        std::fs::write(root.join(format!("other_{p}.txt")), "y").unwrap();
    }
    for p in prefixes {
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions::default().with_browse_filter(*p),
        )
        .unwrap();
        let paths: Vec<_> = listing
            .entries
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        assert!(
            paths.contains(&format!("src/{p}.rs").as_str()),
            "{p} missing in {paths:?}"
        );
        assert!(
            paths.contains(&format!("other_{p}.txt").as_str()),
            "{p} other missing in {paths:?}"
        );
        assert_eq!(paths.len(), 2, "{p} -> {paths:?}");
    }
}

#[test]
fn empty_filter_lists_same_as_none_for_many_trees() {
    for n in 1..=40 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("e");
        write_files(&root, n, "f");
        let a = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions::default(),
        )
        .unwrap();
        let b = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions::default().with_browse_filter(""),
        )
        .unwrap();
        assert_eq!(a.entries, b.entries, "n={n}");
        assert_eq!(a.truncated, b.truncated, "n={n}");
    }
}
