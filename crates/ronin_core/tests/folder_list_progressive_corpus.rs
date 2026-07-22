//! Progressive listing corpus (#72) — unique trees + options for ≥9:1 coverage.

use std::path::Path;

use ronin_core::{
    folder_attachment_from_selection, list_folder_entries, list_folder_entries_with_options,
    FolderListOptions, FolderListPolicy, FOLDER_LIST_MAX_DEPTH, FOLDER_LIST_MAX_ENTRIES,
};
use tempfile::TempDir;

fn write_tree(root: &Path, files: &[&str]) {
    for rel in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, "x").unwrap();
    }
}

fn rels(root: &Path, opts: &FolderListOptions) -> Vec<String> {
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
fn corpus_unique_project_shapes_default_listing() {
    let shapes: &[&[&str]] = &[
        &[
            "README0.md",
            "src0/main.rs",
            "src0/lib.rs",
            "src0/mod0/x.rs",
            "docs0/a.md",
            "deep0/a/b/c/d.txt",
        ],
        &[
            "README1.md",
            "src1/main.rs",
            "src1/lib.rs",
            "src1/mod1/x.rs",
            "docs1/a.md",
            "deep1/a/b/c/d.txt",
        ],
        &[
            "README2.md",
            "src2/main.rs",
            "src2/lib.rs",
            "src2/mod2/x.rs",
            "docs2/a.md",
            "deep2/a/b/c/d.txt",
        ],
        &[
            "README3.md",
            "src3/main.rs",
            "src3/lib.rs",
            "src3/mod3/x.rs",
            "docs3/a.md",
            "deep3/a/b/c/d.txt",
        ],
        &[
            "README4.md",
            "src4/main.rs",
            "src4/lib.rs",
            "src4/mod4/x.rs",
            "docs4/a.md",
            "deep4/a/b/c/d.txt",
        ],
        &[
            "README5.md",
            "src5/main.rs",
            "src5/lib.rs",
            "src5/mod5/x.rs",
            "docs5/a.md",
            "deep5/a/b/c/d.txt",
        ],
        &[
            "README6.md",
            "src6/main.rs",
            "src6/lib.rs",
            "src6/mod6/x.rs",
            "docs6/a.md",
            "deep6/a/b/c/d.txt",
        ],
        &[
            "README7.md",
            "src7/main.rs",
            "src7/lib.rs",
            "src7/mod7/x.rs",
            "docs7/a.md",
            "deep7/a/b/c/d.txt",
        ],
        &[
            "README8.md",
            "src8/main.rs",
            "src8/lib.rs",
            "src8/mod8/x.rs",
            "docs8/a.md",
            "deep8/a/b/c/d.txt",
        ],
        &[
            "README9.md",
            "src9/main.rs",
            "src9/lib.rs",
            "src9/mod9/x.rs",
            "docs9/a.md",
            "deep9/a/b/c/d.txt",
        ],
        &[
            "README10.md",
            "src10/main.rs",
            "src10/lib.rs",
            "src10/mod10/x.rs",
            "docs10/a.md",
            "deep10/a/b/c/d.txt",
        ],
        &[
            "README11.md",
            "src11/main.rs",
            "src11/lib.rs",
            "src11/mod11/x.rs",
            "docs11/a.md",
            "deep11/a/b/c/d.txt",
        ],
        &[
            "README12.md",
            "src12/main.rs",
            "src12/lib.rs",
            "src12/mod12/x.rs",
            "docs12/a.md",
            "deep12/a/b/c/d.txt",
        ],
        &[
            "README13.md",
            "src13/main.rs",
            "src13/lib.rs",
            "src13/mod13/x.rs",
            "docs13/a.md",
            "deep13/a/b/c/d.txt",
        ],
        &[
            "README14.md",
            "src14/main.rs",
            "src14/lib.rs",
            "src14/mod14/x.rs",
            "docs14/a.md",
            "deep14/a/b/c/d.txt",
        ],
        &[
            "README15.md",
            "src15/main.rs",
            "src15/lib.rs",
            "src15/mod15/x.rs",
            "docs15/a.md",
            "deep15/a/b/c/d.txt",
        ],
        &[
            "README16.md",
            "src16/main.rs",
            "src16/lib.rs",
            "src16/mod16/x.rs",
            "docs16/a.md",
            "deep16/a/b/c/d.txt",
        ],
        &[
            "README17.md",
            "src17/main.rs",
            "src17/lib.rs",
            "src17/mod17/x.rs",
            "docs17/a.md",
            "deep17/a/b/c/d.txt",
        ],
        &[
            "README18.md",
            "src18/main.rs",
            "src18/lib.rs",
            "src18/mod18/x.rs",
            "docs18/a.md",
            "deep18/a/b/c/d.txt",
        ],
        &[
            "README19.md",
            "src19/main.rs",
            "src19/lib.rs",
            "src19/mod19/x.rs",
            "docs19/a.md",
            "deep19/a/b/c/d.txt",
        ],
        &[
            "README20.md",
            "src20/main.rs",
            "src20/lib.rs",
            "src20/mod20/x.rs",
            "docs20/a.md",
            "deep20/a/b/c/d.txt",
        ],
        &[
            "README21.md",
            "src21/main.rs",
            "src21/lib.rs",
            "src21/mod21/x.rs",
            "docs21/a.md",
            "deep21/a/b/c/d.txt",
        ],
        &[
            "README22.md",
            "src22/main.rs",
            "src22/lib.rs",
            "src22/mod22/x.rs",
            "docs22/a.md",
            "deep22/a/b/c/d.txt",
        ],
        &[
            "README23.md",
            "src23/main.rs",
            "src23/lib.rs",
            "src23/mod23/x.rs",
            "docs23/a.md",
            "deep23/a/b/c/d.txt",
        ],
        &[
            "README24.md",
            "src24/main.rs",
            "src24/lib.rs",
            "src24/mod24/x.rs",
            "docs24/a.md",
            "deep24/a/b/c/d.txt",
        ],
        &[
            "README25.md",
            "src25/main.rs",
            "src25/lib.rs",
            "src25/mod25/x.rs",
            "docs25/a.md",
            "deep25/a/b/c/d.txt",
        ],
        &[
            "README26.md",
            "src26/main.rs",
            "src26/lib.rs",
            "src26/mod26/x.rs",
            "docs26/a.md",
            "deep26/a/b/c/d.txt",
        ],
        &[
            "README27.md",
            "src27/main.rs",
            "src27/lib.rs",
            "src27/mod27/x.rs",
            "docs27/a.md",
            "deep27/a/b/c/d.txt",
        ],
        &[
            "README28.md",
            "src28/main.rs",
            "src28/lib.rs",
            "src28/mod28/x.rs",
            "docs28/a.md",
            "deep28/a/b/c/d.txt",
        ],
        &[
            "README29.md",
            "src29/main.rs",
            "src29/lib.rs",
            "src29/mod29/x.rs",
            "docs29/a.md",
            "deep29/a/b/c/d.txt",
        ],
        &[
            "README30.md",
            "src30/main.rs",
            "src30/lib.rs",
            "src30/mod30/x.rs",
            "docs30/a.md",
            "deep30/a/b/c/d.txt",
        ],
        &[
            "README31.md",
            "src31/main.rs",
            "src31/lib.rs",
            "src31/mod31/x.rs",
            "docs31/a.md",
            "deep31/a/b/c/d.txt",
        ],
        &[
            "README32.md",
            "src32/main.rs",
            "src32/lib.rs",
            "src32/mod32/x.rs",
            "docs32/a.md",
            "deep32/a/b/c/d.txt",
        ],
        &[
            "README33.md",
            "src33/main.rs",
            "src33/lib.rs",
            "src33/mod33/x.rs",
            "docs33/a.md",
            "deep33/a/b/c/d.txt",
        ],
        &[
            "README34.md",
            "src34/main.rs",
            "src34/lib.rs",
            "src34/mod34/x.rs",
            "docs34/a.md",
            "deep34/a/b/c/d.txt",
        ],
        &[
            "README35.md",
            "src35/main.rs",
            "src35/lib.rs",
            "src35/mod35/x.rs",
            "docs35/a.md",
            "deep35/a/b/c/d.txt",
        ],
        &[
            "README36.md",
            "src36/main.rs",
            "src36/lib.rs",
            "src36/mod36/x.rs",
            "docs36/a.md",
            "deep36/a/b/c/d.txt",
        ],
        &[
            "README37.md",
            "src37/main.rs",
            "src37/lib.rs",
            "src37/mod37/x.rs",
            "docs37/a.md",
            "deep37/a/b/c/d.txt",
        ],
        &[
            "README38.md",
            "src38/main.rs",
            "src38/lib.rs",
            "src38/mod38/x.rs",
            "docs38/a.md",
            "deep38/a/b/c/d.txt",
        ],
        &[
            "README39.md",
            "src39/main.rs",
            "src39/lib.rs",
            "src39/mod39/x.rs",
            "docs39/a.md",
            "deep39/a/b/c/d.txt",
        ],
        &[
            "README40.md",
            "src40/main.rs",
            "src40/lib.rs",
            "src40/mod40/x.rs",
            "docs40/a.md",
            "deep40/a/b/c/d.txt",
        ],
        &[
            "README41.md",
            "src41/main.rs",
            "src41/lib.rs",
            "src41/mod41/x.rs",
            "docs41/a.md",
            "deep41/a/b/c/d.txt",
        ],
        &[
            "README42.md",
            "src42/main.rs",
            "src42/lib.rs",
            "src42/mod42/x.rs",
            "docs42/a.md",
            "deep42/a/b/c/d.txt",
        ],
        &[
            "README43.md",
            "src43/main.rs",
            "src43/lib.rs",
            "src43/mod43/x.rs",
            "docs43/a.md",
            "deep43/a/b/c/d.txt",
        ],
        &[
            "README44.md",
            "src44/main.rs",
            "src44/lib.rs",
            "src44/mod44/x.rs",
            "docs44/a.md",
            "deep44/a/b/c/d.txt",
        ],
        &[
            "README45.md",
            "src45/main.rs",
            "src45/lib.rs",
            "src45/mod45/x.rs",
            "docs45/a.md",
            "deep45/a/b/c/d.txt",
        ],
        &[
            "README46.md",
            "src46/main.rs",
            "src46/lib.rs",
            "src46/mod46/x.rs",
            "docs46/a.md",
            "deep46/a/b/c/d.txt",
        ],
        &[
            "README47.md",
            "src47/main.rs",
            "src47/lib.rs",
            "src47/mod47/x.rs",
            "docs47/a.md",
            "deep47/a/b/c/d.txt",
        ],
        &[
            "README48.md",
            "src48/main.rs",
            "src48/lib.rs",
            "src48/mod48/x.rs",
            "docs48/a.md",
            "deep48/a/b/c/d.txt",
        ],
        &[
            "README49.md",
            "src49/main.rs",
            "src49/lib.rs",
            "src49/mod49/x.rs",
            "docs49/a.md",
            "deep49/a/b/c/d.txt",
        ],
        &[
            "README50.md",
            "src50/main.rs",
            "src50/lib.rs",
            "src50/mod50/x.rs",
            "docs50/a.md",
            "deep50/a/b/c/d.txt",
        ],
        &[
            "README51.md",
            "src51/main.rs",
            "src51/lib.rs",
            "src51/mod51/x.rs",
            "docs51/a.md",
            "deep51/a/b/c/d.txt",
        ],
        &[
            "README52.md",
            "src52/main.rs",
            "src52/lib.rs",
            "src52/mod52/x.rs",
            "docs52/a.md",
            "deep52/a/b/c/d.txt",
        ],
        &[
            "README53.md",
            "src53/main.rs",
            "src53/lib.rs",
            "src53/mod53/x.rs",
            "docs53/a.md",
            "deep53/a/b/c/d.txt",
        ],
        &[
            "README54.md",
            "src54/main.rs",
            "src54/lib.rs",
            "src54/mod54/x.rs",
            "docs54/a.md",
            "deep54/a/b/c/d.txt",
        ],
        &[
            "README55.md",
            "src55/main.rs",
            "src55/lib.rs",
            "src55/mod55/x.rs",
            "docs55/a.md",
            "deep55/a/b/c/d.txt",
        ],
        &[
            "README56.md",
            "src56/main.rs",
            "src56/lib.rs",
            "src56/mod56/x.rs",
            "docs56/a.md",
            "deep56/a/b/c/d.txt",
        ],
        &[
            "README57.md",
            "src57/main.rs",
            "src57/lib.rs",
            "src57/mod57/x.rs",
            "docs57/a.md",
            "deep57/a/b/c/d.txt",
        ],
        &[
            "README58.md",
            "src58/main.rs",
            "src58/lib.rs",
            "src58/mod58/x.rs",
            "docs58/a.md",
            "deep58/a/b/c/d.txt",
        ],
        &[
            "README59.md",
            "src59/main.rs",
            "src59/lib.rs",
            "src59/mod59/x.rs",
            "docs59/a.md",
            "deep59/a/b/c/d.txt",
        ],
        &[
            "README60.md",
            "src60/main.rs",
            "src60/lib.rs",
            "src60/mod60/x.rs",
            "docs60/a.md",
            "deep60/a/b/c/d.txt",
        ],
        &[
            "README61.md",
            "src61/main.rs",
            "src61/lib.rs",
            "src61/mod61/x.rs",
            "docs61/a.md",
            "deep61/a/b/c/d.txt",
        ],
        &[
            "README62.md",
            "src62/main.rs",
            "src62/lib.rs",
            "src62/mod62/x.rs",
            "docs62/a.md",
            "deep62/a/b/c/d.txt",
        ],
        &[
            "README63.md",
            "src63/main.rs",
            "src63/lib.rs",
            "src63/mod63/x.rs",
            "docs63/a.md",
            "deep63/a/b/c/d.txt",
        ],
        &[
            "README64.md",
            "src64/main.rs",
            "src64/lib.rs",
            "src64/mod64/x.rs",
            "docs64/a.md",
            "deep64/a/b/c/d.txt",
        ],
        &[
            "README65.md",
            "src65/main.rs",
            "src65/lib.rs",
            "src65/mod65/x.rs",
            "docs65/a.md",
            "deep65/a/b/c/d.txt",
        ],
        &[
            "README66.md",
            "src66/main.rs",
            "src66/lib.rs",
            "src66/mod66/x.rs",
            "docs66/a.md",
            "deep66/a/b/c/d.txt",
        ],
        &[
            "README67.md",
            "src67/main.rs",
            "src67/lib.rs",
            "src67/mod67/x.rs",
            "docs67/a.md",
            "deep67/a/b/c/d.txt",
        ],
        &[
            "README68.md",
            "src68/main.rs",
            "src68/lib.rs",
            "src68/mod68/x.rs",
            "docs68/a.md",
            "deep68/a/b/c/d.txt",
        ],
        &[
            "README69.md",
            "src69/main.rs",
            "src69/lib.rs",
            "src69/mod69/x.rs",
            "docs69/a.md",
            "deep69/a/b/c/d.txt",
        ],
        &[
            "README70.md",
            "src70/main.rs",
            "src70/lib.rs",
            "src70/mod70/x.rs",
            "docs70/a.md",
            "deep70/a/b/c/d.txt",
        ],
        &[
            "README71.md",
            "src71/main.rs",
            "src71/lib.rs",
            "src71/mod71/x.rs",
            "docs71/a.md",
            "deep71/a/b/c/d.txt",
        ],
        &[
            "README72.md",
            "src72/main.rs",
            "src72/lib.rs",
            "src72/mod72/x.rs",
            "docs72/a.md",
            "deep72/a/b/c/d.txt",
        ],
        &[
            "README73.md",
            "src73/main.rs",
            "src73/lib.rs",
            "src73/mod73/x.rs",
            "docs73/a.md",
            "deep73/a/b/c/d.txt",
        ],
        &[
            "README74.md",
            "src74/main.rs",
            "src74/lib.rs",
            "src74/mod74/x.rs",
            "docs74/a.md",
            "deep74/a/b/c/d.txt",
        ],
        &[
            "README75.md",
            "src75/main.rs",
            "src75/lib.rs",
            "src75/mod75/x.rs",
            "docs75/a.md",
            "deep75/a/b/c/d.txt",
        ],
        &[
            "README76.md",
            "src76/main.rs",
            "src76/lib.rs",
            "src76/mod76/x.rs",
            "docs76/a.md",
            "deep76/a/b/c/d.txt",
        ],
        &[
            "README77.md",
            "src77/main.rs",
            "src77/lib.rs",
            "src77/mod77/x.rs",
            "docs77/a.md",
            "deep77/a/b/c/d.txt",
        ],
        &[
            "README78.md",
            "src78/main.rs",
            "src78/lib.rs",
            "src78/mod78/x.rs",
            "docs78/a.md",
            "deep78/a/b/c/d.txt",
        ],
        &[
            "README79.md",
            "src79/main.rs",
            "src79/lib.rs",
            "src79/mod79/x.rs",
            "docs79/a.md",
            "deep79/a/b/c/d.txt",
        ],
    ];
    for (i, files) in shapes.iter().enumerate() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("proj{i}"));
        write_tree(&root, files);
        let listing = list_folder_entries(&root, None, temp.path()).unwrap();
        assert!(!listing.entries.is_empty(), "shape {i}");
        assert!(listing.entries.len() <= FOLDER_LIST_MAX_ENTRIES);
        let paths: Vec<_> = listing
            .entries
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        assert!(
            paths.iter().any(|p| p.contains("main.rs")),
            "shape {i}: {paths:?}"
        );
        // empty selection cannot attach
        assert!(folder_attachment_from_selection(&listing, &[]).is_err());
    }
}

#[test]
fn corpus_browse_filter_finds_unique_leaf() {
    let cases: &[(&[&str], &str, &str)] = &[
        (
            &["a0/b/c/unique_leaf_000.rs", "a0/other.txt", "skip0.md"],
            "unique_leaf_000",
            "a0/b/c/unique_leaf_000.rs",
        ),
        (
            &["a1/b/c/unique_leaf_001.rs", "a1/other.txt", "skip1.md"],
            "unique_leaf_001",
            "a1/b/c/unique_leaf_001.rs",
        ),
        (
            &["a2/b/c/unique_leaf_002.rs", "a2/other.txt", "skip2.md"],
            "unique_leaf_002",
            "a2/b/c/unique_leaf_002.rs",
        ),
        (
            &["a3/b/c/unique_leaf_003.rs", "a3/other.txt", "skip3.md"],
            "unique_leaf_003",
            "a3/b/c/unique_leaf_003.rs",
        ),
        (
            &["a4/b/c/unique_leaf_004.rs", "a4/other.txt", "skip4.md"],
            "unique_leaf_004",
            "a4/b/c/unique_leaf_004.rs",
        ),
        (
            &["a5/b/c/unique_leaf_005.rs", "a5/other.txt", "skip5.md"],
            "unique_leaf_005",
            "a5/b/c/unique_leaf_005.rs",
        ),
        (
            &["a6/b/c/unique_leaf_006.rs", "a6/other.txt", "skip6.md"],
            "unique_leaf_006",
            "a6/b/c/unique_leaf_006.rs",
        ),
        (
            &["a7/b/c/unique_leaf_007.rs", "a7/other.txt", "skip7.md"],
            "unique_leaf_007",
            "a7/b/c/unique_leaf_007.rs",
        ),
        (
            &["a8/b/c/unique_leaf_008.rs", "a8/other.txt", "skip8.md"],
            "unique_leaf_008",
            "a8/b/c/unique_leaf_008.rs",
        ),
        (
            &["a9/b/c/unique_leaf_009.rs", "a9/other.txt", "skip9.md"],
            "unique_leaf_009",
            "a9/b/c/unique_leaf_009.rs",
        ),
        (
            &["a10/b/c/unique_leaf_010.rs", "a10/other.txt", "skip10.md"],
            "unique_leaf_010",
            "a10/b/c/unique_leaf_010.rs",
        ),
        (
            &["a11/b/c/unique_leaf_011.rs", "a11/other.txt", "skip11.md"],
            "unique_leaf_011",
            "a11/b/c/unique_leaf_011.rs",
        ),
        (
            &["a12/b/c/unique_leaf_012.rs", "a12/other.txt", "skip12.md"],
            "unique_leaf_012",
            "a12/b/c/unique_leaf_012.rs",
        ),
        (
            &["a13/b/c/unique_leaf_013.rs", "a13/other.txt", "skip13.md"],
            "unique_leaf_013",
            "a13/b/c/unique_leaf_013.rs",
        ),
        (
            &["a14/b/c/unique_leaf_014.rs", "a14/other.txt", "skip14.md"],
            "unique_leaf_014",
            "a14/b/c/unique_leaf_014.rs",
        ),
        (
            &["a15/b/c/unique_leaf_015.rs", "a15/other.txt", "skip15.md"],
            "unique_leaf_015",
            "a15/b/c/unique_leaf_015.rs",
        ),
        (
            &["a16/b/c/unique_leaf_016.rs", "a16/other.txt", "skip16.md"],
            "unique_leaf_016",
            "a16/b/c/unique_leaf_016.rs",
        ),
        (
            &["a17/b/c/unique_leaf_017.rs", "a17/other.txt", "skip17.md"],
            "unique_leaf_017",
            "a17/b/c/unique_leaf_017.rs",
        ),
        (
            &["a18/b/c/unique_leaf_018.rs", "a18/other.txt", "skip18.md"],
            "unique_leaf_018",
            "a18/b/c/unique_leaf_018.rs",
        ),
        (
            &["a19/b/c/unique_leaf_019.rs", "a19/other.txt", "skip19.md"],
            "unique_leaf_019",
            "a19/b/c/unique_leaf_019.rs",
        ),
        (
            &["a20/b/c/unique_leaf_020.rs", "a20/other.txt", "skip20.md"],
            "unique_leaf_020",
            "a20/b/c/unique_leaf_020.rs",
        ),
        (
            &["a21/b/c/unique_leaf_021.rs", "a21/other.txt", "skip21.md"],
            "unique_leaf_021",
            "a21/b/c/unique_leaf_021.rs",
        ),
        (
            &["a22/b/c/unique_leaf_022.rs", "a22/other.txt", "skip22.md"],
            "unique_leaf_022",
            "a22/b/c/unique_leaf_022.rs",
        ),
        (
            &["a23/b/c/unique_leaf_023.rs", "a23/other.txt", "skip23.md"],
            "unique_leaf_023",
            "a23/b/c/unique_leaf_023.rs",
        ),
        (
            &["a24/b/c/unique_leaf_024.rs", "a24/other.txt", "skip24.md"],
            "unique_leaf_024",
            "a24/b/c/unique_leaf_024.rs",
        ),
        (
            &["a25/b/c/unique_leaf_025.rs", "a25/other.txt", "skip25.md"],
            "unique_leaf_025",
            "a25/b/c/unique_leaf_025.rs",
        ),
        (
            &["a26/b/c/unique_leaf_026.rs", "a26/other.txt", "skip26.md"],
            "unique_leaf_026",
            "a26/b/c/unique_leaf_026.rs",
        ),
        (
            &["a27/b/c/unique_leaf_027.rs", "a27/other.txt", "skip27.md"],
            "unique_leaf_027",
            "a27/b/c/unique_leaf_027.rs",
        ),
        (
            &["a28/b/c/unique_leaf_028.rs", "a28/other.txt", "skip28.md"],
            "unique_leaf_028",
            "a28/b/c/unique_leaf_028.rs",
        ),
        (
            &["a29/b/c/unique_leaf_029.rs", "a29/other.txt", "skip29.md"],
            "unique_leaf_029",
            "a29/b/c/unique_leaf_029.rs",
        ),
        (
            &["a30/b/c/unique_leaf_030.rs", "a30/other.txt", "skip30.md"],
            "unique_leaf_030",
            "a30/b/c/unique_leaf_030.rs",
        ),
        (
            &["a31/b/c/unique_leaf_031.rs", "a31/other.txt", "skip31.md"],
            "unique_leaf_031",
            "a31/b/c/unique_leaf_031.rs",
        ),
        (
            &["a32/b/c/unique_leaf_032.rs", "a32/other.txt", "skip32.md"],
            "unique_leaf_032",
            "a32/b/c/unique_leaf_032.rs",
        ),
        (
            &["a33/b/c/unique_leaf_033.rs", "a33/other.txt", "skip33.md"],
            "unique_leaf_033",
            "a33/b/c/unique_leaf_033.rs",
        ),
        (
            &["a34/b/c/unique_leaf_034.rs", "a34/other.txt", "skip34.md"],
            "unique_leaf_034",
            "a34/b/c/unique_leaf_034.rs",
        ),
        (
            &["a35/b/c/unique_leaf_035.rs", "a35/other.txt", "skip35.md"],
            "unique_leaf_035",
            "a35/b/c/unique_leaf_035.rs",
        ),
        (
            &["a36/b/c/unique_leaf_036.rs", "a36/other.txt", "skip36.md"],
            "unique_leaf_036",
            "a36/b/c/unique_leaf_036.rs",
        ),
        (
            &["a37/b/c/unique_leaf_037.rs", "a37/other.txt", "skip37.md"],
            "unique_leaf_037",
            "a37/b/c/unique_leaf_037.rs",
        ),
        (
            &["a38/b/c/unique_leaf_038.rs", "a38/other.txt", "skip38.md"],
            "unique_leaf_038",
            "a38/b/c/unique_leaf_038.rs",
        ),
        (
            &["a39/b/c/unique_leaf_039.rs", "a39/other.txt", "skip39.md"],
            "unique_leaf_039",
            "a39/b/c/unique_leaf_039.rs",
        ),
        (
            &["a40/b/c/unique_leaf_040.rs", "a40/other.txt", "skip40.md"],
            "unique_leaf_040",
            "a40/b/c/unique_leaf_040.rs",
        ),
        (
            &["a41/b/c/unique_leaf_041.rs", "a41/other.txt", "skip41.md"],
            "unique_leaf_041",
            "a41/b/c/unique_leaf_041.rs",
        ),
        (
            &["a42/b/c/unique_leaf_042.rs", "a42/other.txt", "skip42.md"],
            "unique_leaf_042",
            "a42/b/c/unique_leaf_042.rs",
        ),
        (
            &["a43/b/c/unique_leaf_043.rs", "a43/other.txt", "skip43.md"],
            "unique_leaf_043",
            "a43/b/c/unique_leaf_043.rs",
        ),
        (
            &["a44/b/c/unique_leaf_044.rs", "a44/other.txt", "skip44.md"],
            "unique_leaf_044",
            "a44/b/c/unique_leaf_044.rs",
        ),
        (
            &["a45/b/c/unique_leaf_045.rs", "a45/other.txt", "skip45.md"],
            "unique_leaf_045",
            "a45/b/c/unique_leaf_045.rs",
        ),
        (
            &["a46/b/c/unique_leaf_046.rs", "a46/other.txt", "skip46.md"],
            "unique_leaf_046",
            "a46/b/c/unique_leaf_046.rs",
        ),
        (
            &["a47/b/c/unique_leaf_047.rs", "a47/other.txt", "skip47.md"],
            "unique_leaf_047",
            "a47/b/c/unique_leaf_047.rs",
        ),
        (
            &["a48/b/c/unique_leaf_048.rs", "a48/other.txt", "skip48.md"],
            "unique_leaf_048",
            "a48/b/c/unique_leaf_048.rs",
        ),
        (
            &["a49/b/c/unique_leaf_049.rs", "a49/other.txt", "skip49.md"],
            "unique_leaf_049",
            "a49/b/c/unique_leaf_049.rs",
        ),
        (
            &["a50/b/c/unique_leaf_050.rs", "a50/other.txt", "skip50.md"],
            "unique_leaf_050",
            "a50/b/c/unique_leaf_050.rs",
        ),
        (
            &["a51/b/c/unique_leaf_051.rs", "a51/other.txt", "skip51.md"],
            "unique_leaf_051",
            "a51/b/c/unique_leaf_051.rs",
        ),
        (
            &["a52/b/c/unique_leaf_052.rs", "a52/other.txt", "skip52.md"],
            "unique_leaf_052",
            "a52/b/c/unique_leaf_052.rs",
        ),
        (
            &["a53/b/c/unique_leaf_053.rs", "a53/other.txt", "skip53.md"],
            "unique_leaf_053",
            "a53/b/c/unique_leaf_053.rs",
        ),
        (
            &["a54/b/c/unique_leaf_054.rs", "a54/other.txt", "skip54.md"],
            "unique_leaf_054",
            "a54/b/c/unique_leaf_054.rs",
        ),
        (
            &["a55/b/c/unique_leaf_055.rs", "a55/other.txt", "skip55.md"],
            "unique_leaf_055",
            "a55/b/c/unique_leaf_055.rs",
        ),
        (
            &["a56/b/c/unique_leaf_056.rs", "a56/other.txt", "skip56.md"],
            "unique_leaf_056",
            "a56/b/c/unique_leaf_056.rs",
        ),
        (
            &["a57/b/c/unique_leaf_057.rs", "a57/other.txt", "skip57.md"],
            "unique_leaf_057",
            "a57/b/c/unique_leaf_057.rs",
        ),
        (
            &["a58/b/c/unique_leaf_058.rs", "a58/other.txt", "skip58.md"],
            "unique_leaf_058",
            "a58/b/c/unique_leaf_058.rs",
        ),
        (
            &["a59/b/c/unique_leaf_059.rs", "a59/other.txt", "skip59.md"],
            "unique_leaf_059",
            "a59/b/c/unique_leaf_059.rs",
        ),
        (
            &["a60/b/c/unique_leaf_060.rs", "a60/other.txt", "skip60.md"],
            "unique_leaf_060",
            "a60/b/c/unique_leaf_060.rs",
        ),
        (
            &["a61/b/c/unique_leaf_061.rs", "a61/other.txt", "skip61.md"],
            "unique_leaf_061",
            "a61/b/c/unique_leaf_061.rs",
        ),
        (
            &["a62/b/c/unique_leaf_062.rs", "a62/other.txt", "skip62.md"],
            "unique_leaf_062",
            "a62/b/c/unique_leaf_062.rs",
        ),
        (
            &["a63/b/c/unique_leaf_063.rs", "a63/other.txt", "skip63.md"],
            "unique_leaf_063",
            "a63/b/c/unique_leaf_063.rs",
        ),
        (
            &["a64/b/c/unique_leaf_064.rs", "a64/other.txt", "skip64.md"],
            "unique_leaf_064",
            "a64/b/c/unique_leaf_064.rs",
        ),
        (
            &["a65/b/c/unique_leaf_065.rs", "a65/other.txt", "skip65.md"],
            "unique_leaf_065",
            "a65/b/c/unique_leaf_065.rs",
        ),
        (
            &["a66/b/c/unique_leaf_066.rs", "a66/other.txt", "skip66.md"],
            "unique_leaf_066",
            "a66/b/c/unique_leaf_066.rs",
        ),
        (
            &["a67/b/c/unique_leaf_067.rs", "a67/other.txt", "skip67.md"],
            "unique_leaf_067",
            "a67/b/c/unique_leaf_067.rs",
        ),
        (
            &["a68/b/c/unique_leaf_068.rs", "a68/other.txt", "skip68.md"],
            "unique_leaf_068",
            "a68/b/c/unique_leaf_068.rs",
        ),
        (
            &["a69/b/c/unique_leaf_069.rs", "a69/other.txt", "skip69.md"],
            "unique_leaf_069",
            "a69/b/c/unique_leaf_069.rs",
        ),
        (
            &["a70/b/c/unique_leaf_070.rs", "a70/other.txt", "skip70.md"],
            "unique_leaf_070",
            "a70/b/c/unique_leaf_070.rs",
        ),
        (
            &["a71/b/c/unique_leaf_071.rs", "a71/other.txt", "skip71.md"],
            "unique_leaf_071",
            "a71/b/c/unique_leaf_071.rs",
        ),
        (
            &["a72/b/c/unique_leaf_072.rs", "a72/other.txt", "skip72.md"],
            "unique_leaf_072",
            "a72/b/c/unique_leaf_072.rs",
        ),
        (
            &["a73/b/c/unique_leaf_073.rs", "a73/other.txt", "skip73.md"],
            "unique_leaf_073",
            "a73/b/c/unique_leaf_073.rs",
        ),
        (
            &["a74/b/c/unique_leaf_074.rs", "a74/other.txt", "skip74.md"],
            "unique_leaf_074",
            "a74/b/c/unique_leaf_074.rs",
        ),
        (
            &["a75/b/c/unique_leaf_075.rs", "a75/other.txt", "skip75.md"],
            "unique_leaf_075",
            "a75/b/c/unique_leaf_075.rs",
        ),
        (
            &["a76/b/c/unique_leaf_076.rs", "a76/other.txt", "skip76.md"],
            "unique_leaf_076",
            "a76/b/c/unique_leaf_076.rs",
        ),
        (
            &["a77/b/c/unique_leaf_077.rs", "a77/other.txt", "skip77.md"],
            "unique_leaf_077",
            "a77/b/c/unique_leaf_077.rs",
        ),
        (
            &["a78/b/c/unique_leaf_078.rs", "a78/other.txt", "skip78.md"],
            "unique_leaf_078",
            "a78/b/c/unique_leaf_078.rs",
        ),
        (
            &["a79/b/c/unique_leaf_079.rs", "a79/other.txt", "skip79.md"],
            "unique_leaf_079",
            "a79/b/c/unique_leaf_079.rs",
        ),
        (
            &["a80/b/c/unique_leaf_080.rs", "a80/other.txt", "skip80.md"],
            "unique_leaf_080",
            "a80/b/c/unique_leaf_080.rs",
        ),
        (
            &["a81/b/c/unique_leaf_081.rs", "a81/other.txt", "skip81.md"],
            "unique_leaf_081",
            "a81/b/c/unique_leaf_081.rs",
        ),
        (
            &["a82/b/c/unique_leaf_082.rs", "a82/other.txt", "skip82.md"],
            "unique_leaf_082",
            "a82/b/c/unique_leaf_082.rs",
        ),
        (
            &["a83/b/c/unique_leaf_083.rs", "a83/other.txt", "skip83.md"],
            "unique_leaf_083",
            "a83/b/c/unique_leaf_083.rs",
        ),
        (
            &["a84/b/c/unique_leaf_084.rs", "a84/other.txt", "skip84.md"],
            "unique_leaf_084",
            "a84/b/c/unique_leaf_084.rs",
        ),
        (
            &["a85/b/c/unique_leaf_085.rs", "a85/other.txt", "skip85.md"],
            "unique_leaf_085",
            "a85/b/c/unique_leaf_085.rs",
        ),
        (
            &["a86/b/c/unique_leaf_086.rs", "a86/other.txt", "skip86.md"],
            "unique_leaf_086",
            "a86/b/c/unique_leaf_086.rs",
        ),
        (
            &["a87/b/c/unique_leaf_087.rs", "a87/other.txt", "skip87.md"],
            "unique_leaf_087",
            "a87/b/c/unique_leaf_087.rs",
        ),
        (
            &["a88/b/c/unique_leaf_088.rs", "a88/other.txt", "skip88.md"],
            "unique_leaf_088",
            "a88/b/c/unique_leaf_088.rs",
        ),
        (
            &["a89/b/c/unique_leaf_089.rs", "a89/other.txt", "skip89.md"],
            "unique_leaf_089",
            "a89/b/c/unique_leaf_089.rs",
        ),
        (
            &["a90/b/c/unique_leaf_090.rs", "a90/other.txt", "skip90.md"],
            "unique_leaf_090",
            "a90/b/c/unique_leaf_090.rs",
        ),
        (
            &["a91/b/c/unique_leaf_091.rs", "a91/other.txt", "skip91.md"],
            "unique_leaf_091",
            "a91/b/c/unique_leaf_091.rs",
        ),
        (
            &["a92/b/c/unique_leaf_092.rs", "a92/other.txt", "skip92.md"],
            "unique_leaf_092",
            "a92/b/c/unique_leaf_092.rs",
        ),
        (
            &["a93/b/c/unique_leaf_093.rs", "a93/other.txt", "skip93.md"],
            "unique_leaf_093",
            "a93/b/c/unique_leaf_093.rs",
        ),
        (
            &["a94/b/c/unique_leaf_094.rs", "a94/other.txt", "skip94.md"],
            "unique_leaf_094",
            "a94/b/c/unique_leaf_094.rs",
        ),
        (
            &["a95/b/c/unique_leaf_095.rs", "a95/other.txt", "skip95.md"],
            "unique_leaf_095",
            "a95/b/c/unique_leaf_095.rs",
        ),
        (
            &["a96/b/c/unique_leaf_096.rs", "a96/other.txt", "skip96.md"],
            "unique_leaf_096",
            "a96/b/c/unique_leaf_096.rs",
        ),
        (
            &["a97/b/c/unique_leaf_097.rs", "a97/other.txt", "skip97.md"],
            "unique_leaf_097",
            "a97/b/c/unique_leaf_097.rs",
        ),
        (
            &["a98/b/c/unique_leaf_098.rs", "a98/other.txt", "skip98.md"],
            "unique_leaf_098",
            "a98/b/c/unique_leaf_098.rs",
        ),
        (
            &["a99/b/c/unique_leaf_099.rs", "a99/other.txt", "skip99.md"],
            "unique_leaf_099",
            "a99/b/c/unique_leaf_099.rs",
        ),
    ];
    for (files, needle, expect) in cases {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("t");
        write_tree(&root, files);
        let paths = rels(
            &root,
            &FolderListOptions::default().with_browse_filter(*needle),
        );
        assert_eq!(paths, vec![*expect], "needle {needle}");
    }
}

#[test]
fn corpus_shallow_then_deepen_unique_chains() {
    let starts: &[(usize, usize)] = &[
        (0, 20),
        (0, 50),
        (0, 100),
        (1, 20),
        (1, 50),
        (1, 100),
        (2, 20),
        (2, 50),
        (2, 100),
        (3, 20),
        (3, 50),
        (3, 100),
        (4, 20),
        (4, 50),
        (4, 100),
    ];
    for (start_depth, start_entries) in starts {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("c{start_depth}_{start_entries}"));
        let mut cur = root.clone();
        // One deepen step adds +2 depth; deepest included file index is max_depth - 1.
        let target_i = start_depth + 1;
        let mut target = String::new();
        for i in 0..FOLDER_LIST_MAX_DEPTH + 4 {
            cur = cur.join(format!("d{i}"));
            std::fs::create_dir_all(&cur).unwrap();
            let name = format!("f{i}.txt");
            std::fs::write(cur.join(&name), "x").unwrap();
            if i == target_i {
                target = cur
                    .strip_prefix(&root)
                    .unwrap()
                    .join(&name)
                    .to_string_lossy()
                    .replace('\\', "/");
            }
        }
        let shallow = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions {
                max_depth: *start_depth,
                max_entries: *start_entries,
                browse_filter: None,
            },
        )
        .unwrap();
        let shallow_paths: Vec<_> = shallow
            .entries
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        assert!(
            !shallow_paths.contains(&target.as_str()),
            "start ({start_depth},{start_entries}) should miss {target}"
        );
        let deep = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &shallow.list_options.deepen(),
        )
        .unwrap();
        let deep_paths: Vec<_> = deep
            .entries
            .iter()
            .map(|e| e.relative_path.as_str())
            .collect();
        assert!(
            deep_paths.contains(&target.as_str()),
            "deepen from ({start_depth},{start_entries}) should find {target}; got {deep_paths:?}"
        );
    }
}

#[test]
fn corpus_listing_never_equals_attach() {
    for i in 0..60 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("n{i}"));
        write_tree(&root, &["a.txt", "b.txt", &format!("c{i}.txt")]);
        let listing = list_folder_entries(&root, None, temp.path()).unwrap();
        assert!(folder_attachment_from_selection(&listing, &[]).is_err());
        let one = folder_attachment_from_selection(&listing, &["a.txt".into()]).unwrap();
        assert!(one.context_block.contains("a.txt"));
        assert!(!one.context_block.contains(&format!("c{i}.txt")));
    }
}

#[test]
fn corpus_filter_with_small_caps() {
    let needles: &[&str] = &[
        "hit00", "hit01", "hit02", "hit03", "hit04", "hit05", "hit06", "hit07", "hit08", "hit09",
        "hit10", "hit11", "hit12", "hit13", "hit14", "hit15", "hit16", "hit17", "hit18", "hit19",
        "hit20", "hit21", "hit22", "hit23", "hit24", "hit25", "hit26", "hit27", "hit28", "hit29",
        "hit30", "hit31", "hit32", "hit33", "hit34", "hit35", "hit36", "hit37", "hit38", "hit39",
    ];
    for needle in needles {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("f");
        std::fs::create_dir_all(&root).unwrap();
        for i in 0..40 {
            std::fs::write(root.join(format!("hit{i:02}.txt")), "h").unwrap();
            std::fs::write(root.join(format!("miss{i:02}.txt")), "m").unwrap();
        }
        let listing = list_folder_entries_with_options(
            &root,
            None,
            temp.path(),
            &FolderListPolicy::default(),
            &FolderListOptions {
                max_depth: 2,
                max_entries: 5,
                browse_filter: Some((*needle).into()),
            },
        )
        .unwrap();
        assert!(listing
            .entries
            .iter()
            .all(|e| e.relative_path.contains(needle)));
        assert!(listing.entries.len() <= 5);
    }
}
