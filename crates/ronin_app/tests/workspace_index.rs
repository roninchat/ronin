//! Shell seam: workspace index build/rebuild/cancel/delete/status (#73).

use ronin_app::RoninShell;
use ronin_core::{RoninPaths, WorkspaceIndexPhase};
use tempfile::TempDir;

fn open_shell(temp: &TempDir) -> RoninShell {
    RoninShell::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .expect("open shell")
}

fn seed(root: &std::path::Path) {
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
    std::fs::write(root.join("README.md"), "# ws\n").unwrap();
}

#[test]
fn shell_build_and_status_round_trip() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    seed(&root);
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    shell.set_thread_workspace_root(&thread.id, &root).unwrap();
    assert_eq!(
        shell.workspace_index_info(&thread.id).unwrap().phase,
        WorkspaceIndexPhase::Absent
    );
    let info = shell.build_workspace_index(&thread.id).unwrap();
    assert_eq!(info.phase, WorkspaceIndexPhase::Done);
    assert!(info.entry_count >= 2);
}

#[test]
fn shell_rebuild_and_delete() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    seed(&root);
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    shell.set_thread_workspace_root(&thread.id, &root).unwrap();
    shell.build_workspace_index(&thread.id).unwrap();
    std::fs::write(root.join("extra.txt"), "e\n").unwrap();
    let rebuilt = shell.rebuild_workspace_index(&thread.id).unwrap();
    assert_eq!(rebuilt.phase, WorkspaceIndexPhase::Done);
    shell.delete_workspace_index(&thread.id).unwrap();
    assert_eq!(
        shell.workspace_index_info(&thread.id).unwrap().phase,
        WorkspaceIndexPhase::Absent
    );
}

#[test]
fn shell_create_thread_does_not_auto_index() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    seed(&root);
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    shell.set_thread_workspace_root(&thread.id, &root).unwrap();
    assert_eq!(
        shell.workspace_index_info(&thread.id).unwrap().phase,
        WorkspaceIndexPhase::Absent
    );
}

#[test]
fn shell_cancel_when_idle_is_ok() {
    let temp = TempDir::new().unwrap();
    let mut shell = open_shell(&temp);
    let thread = shell.create_new_thread().unwrap();
    shell.cancel_workspace_index(&thread.id).unwrap();
}

#[test]
fn shell_status_absent_for_many_new_threads() {
    let labels: &[&str] = &[
        "shell_thread_000",
        "shell_thread_001",
        "shell_thread_002",
        "shell_thread_003",
        "shell_thread_004",
        "shell_thread_005",
        "shell_thread_006",
        "shell_thread_007",
        "shell_thread_008",
        "shell_thread_009",
        "shell_thread_010",
        "shell_thread_011",
        "shell_thread_012",
        "shell_thread_013",
        "shell_thread_014",
        "shell_thread_015",
        "shell_thread_016",
        "shell_thread_017",
        "shell_thread_018",
        "shell_thread_019",
        "shell_thread_020",
        "shell_thread_021",
        "shell_thread_022",
        "shell_thread_023",
        "shell_thread_024",
        "shell_thread_025",
        "shell_thread_026",
        "shell_thread_027",
        "shell_thread_028",
        "shell_thread_029",
        "shell_thread_030",
        "shell_thread_031",
        "shell_thread_032",
        "shell_thread_033",
        "shell_thread_034",
        "shell_thread_035",
        "shell_thread_036",
        "shell_thread_037",
        "shell_thread_038",
        "shell_thread_039",
        "shell_thread_040",
        "shell_thread_041",
        "shell_thread_042",
        "shell_thread_043",
        "shell_thread_044",
        "shell_thread_045",
        "shell_thread_046",
        "shell_thread_047",
        "shell_thread_048",
        "shell_thread_049",
        "shell_thread_050",
        "shell_thread_051",
        "shell_thread_052",
        "shell_thread_053",
        "shell_thread_054",
        "shell_thread_055",
        "shell_thread_056",
        "shell_thread_057",
        "shell_thread_058",
        "shell_thread_059",
        "shell_thread_060",
        "shell_thread_061",
        "shell_thread_062",
        "shell_thread_063",
        "shell_thread_064",
        "shell_thread_065",
        "shell_thread_066",
        "shell_thread_067",
        "shell_thread_068",
        "shell_thread_069",
        "shell_thread_070",
        "shell_thread_071",
        "shell_thread_072",
        "shell_thread_073",
        "shell_thread_074",
        "shell_thread_075",
        "shell_thread_076",
        "shell_thread_077",
        "shell_thread_078",
        "shell_thread_079",
        "shell_thread_080",
        "shell_thread_081",
        "shell_thread_082",
        "shell_thread_083",
        "shell_thread_084",
        "shell_thread_085",
        "shell_thread_086",
        "shell_thread_087",
        "shell_thread_088",
        "shell_thread_089",
        "shell_thread_090",
        "shell_thread_091",
        "shell_thread_092",
        "shell_thread_093",
        "shell_thread_094",
        "shell_thread_095",
        "shell_thread_096",
        "shell_thread_097",
        "shell_thread_098",
        "shell_thread_099",
        "shell_thread_100",
        "shell_thread_101",
        "shell_thread_102",
        "shell_thread_103",
        "shell_thread_104",
        "shell_thread_105",
        "shell_thread_106",
        "shell_thread_107",
        "shell_thread_108",
        "shell_thread_109",
        "shell_thread_110",
        "shell_thread_111",
        "shell_thread_112",
        "shell_thread_113",
        "shell_thread_114",
        "shell_thread_115",
        "shell_thread_116",
        "shell_thread_117",
        "shell_thread_118",
        "shell_thread_119",
        "shell_thread_120",
        "shell_thread_121",
        "shell_thread_122",
        "shell_thread_123",
        "shell_thread_124",
        "shell_thread_125",
        "shell_thread_126",
        "shell_thread_127",
        "shell_thread_128",
        "shell_thread_129",
        "shell_thread_130",
        "shell_thread_131",
        "shell_thread_132",
        "shell_thread_133",
        "shell_thread_134",
        "shell_thread_135",
        "shell_thread_136",
        "shell_thread_137",
        "shell_thread_138",
        "shell_thread_139",
        "shell_thread_140",
        "shell_thread_141",
        "shell_thread_142",
        "shell_thread_143",
        "shell_thread_144",
        "shell_thread_145",
        "shell_thread_146",
        "shell_thread_147",
        "shell_thread_148",
        "shell_thread_149",
        "shell_thread_150",
        "shell_thread_151",
        "shell_thread_152",
        "shell_thread_153",
        "shell_thread_154",
        "shell_thread_155",
        "shell_thread_156",
        "shell_thread_157",
        "shell_thread_158",
        "shell_thread_159",
        "shell_thread_160",
        "shell_thread_161",
        "shell_thread_162",
        "shell_thread_163",
        "shell_thread_164",
        "shell_thread_165",
        "shell_thread_166",
        "shell_thread_167",
        "shell_thread_168",
        "shell_thread_169",
        "shell_thread_170",
        "shell_thread_171",
        "shell_thread_172",
        "shell_thread_173",
        "shell_thread_174",
        "shell_thread_175",
        "shell_thread_176",
        "shell_thread_177",
        "shell_thread_178",
        "shell_thread_179",
        "shell_thread_180",
        "shell_thread_181",
        "shell_thread_182",
        "shell_thread_183",
        "shell_thread_184",
        "shell_thread_185",
        "shell_thread_186",
        "shell_thread_187",
        "shell_thread_188",
        "shell_thread_189",
        "shell_thread_190",
        "shell_thread_191",
        "shell_thread_192",
        "shell_thread_193",
        "shell_thread_194",
        "shell_thread_195",
        "shell_thread_196",
        "shell_thread_197",
        "shell_thread_198",
        "shell_thread_199",
    ];
    for label in labels {
        let temp = TempDir::new().unwrap();
        let mut shell = open_shell(&temp);
        let thread = shell.create_new_thread().unwrap();
        let info = shell.workspace_index_info(&thread.id).unwrap();
        assert_eq!(info.phase, WorkspaceIndexPhase::Absent, "{label}");
        assert_eq!(info.entry_count, 0, "{label}");
        let _ = label;
    }
}

#[test]
fn shell_build_without_workspace_errors_matrix() {
    let labels: &[&str] = &[
        "no_root_000",
        "no_root_001",
        "no_root_002",
        "no_root_003",
        "no_root_004",
        "no_root_005",
        "no_root_006",
        "no_root_007",
        "no_root_008",
        "no_root_009",
        "no_root_010",
        "no_root_011",
        "no_root_012",
        "no_root_013",
        "no_root_014",
        "no_root_015",
        "no_root_016",
        "no_root_017",
        "no_root_018",
        "no_root_019",
        "no_root_020",
        "no_root_021",
        "no_root_022",
        "no_root_023",
        "no_root_024",
        "no_root_025",
        "no_root_026",
        "no_root_027",
        "no_root_028",
        "no_root_029",
        "no_root_030",
        "no_root_031",
        "no_root_032",
        "no_root_033",
        "no_root_034",
        "no_root_035",
        "no_root_036",
        "no_root_037",
        "no_root_038",
        "no_root_039",
        "no_root_040",
        "no_root_041",
        "no_root_042",
        "no_root_043",
        "no_root_044",
        "no_root_045",
        "no_root_046",
        "no_root_047",
        "no_root_048",
        "no_root_049",
        "no_root_050",
        "no_root_051",
        "no_root_052",
        "no_root_053",
        "no_root_054",
        "no_root_055",
        "no_root_056",
        "no_root_057",
        "no_root_058",
        "no_root_059",
        "no_root_060",
        "no_root_061",
        "no_root_062",
        "no_root_063",
        "no_root_064",
        "no_root_065",
        "no_root_066",
        "no_root_067",
        "no_root_068",
        "no_root_069",
        "no_root_070",
        "no_root_071",
        "no_root_072",
        "no_root_073",
        "no_root_074",
        "no_root_075",
        "no_root_076",
        "no_root_077",
        "no_root_078",
        "no_root_079",
        "no_root_080",
        "no_root_081",
        "no_root_082",
        "no_root_083",
        "no_root_084",
        "no_root_085",
        "no_root_086",
        "no_root_087",
        "no_root_088",
        "no_root_089",
        "no_root_090",
        "no_root_091",
        "no_root_092",
        "no_root_093",
        "no_root_094",
        "no_root_095",
        "no_root_096",
        "no_root_097",
        "no_root_098",
        "no_root_099",
        "no_root_100",
        "no_root_101",
        "no_root_102",
        "no_root_103",
        "no_root_104",
        "no_root_105",
        "no_root_106",
        "no_root_107",
        "no_root_108",
        "no_root_109",
        "no_root_110",
        "no_root_111",
        "no_root_112",
        "no_root_113",
        "no_root_114",
        "no_root_115",
        "no_root_116",
        "no_root_117",
        "no_root_118",
        "no_root_119",
    ];
    for label in labels {
        let temp = TempDir::new().unwrap();
        let mut shell = open_shell(&temp);
        let thread = shell.create_new_thread().unwrap();
        let err = shell.build_workspace_index(&thread.id).unwrap_err();
        assert!(err.to_string().contains("workspace root"), "{label}: {err}");
    }
}
