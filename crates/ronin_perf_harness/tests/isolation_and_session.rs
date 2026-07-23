//! Isolation + session seeding behavior at the PerfHarness-related seams.

use std::path::PathBuf;
use std::time::Duration;

use ronin_perf_harness::{
    isolated_ronin_paths, AlwaysOkSmoke, BudgetRules, PerfHarness, PerfHarnessConfig, ScenarioId,
    SessionPaintDriver,
};
use tempfile::TempDir;

#[test]
fn isolated_paths_must_not_use_user_xdg_ronin() {
    let temp = TempDir::new().expect("temp");
    let paths = isolated_ronin_paths(temp.path()).expect("isolate");
    let home = std::env::var("HOME").unwrap_or_default();
    if !home.is_empty() {
        assert!(!paths
            .data_dir
            .starts_with(PathBuf::from(&home).join(".local/share/ronin")));
        assert!(!paths
            .config_dir
            .starts_with(PathBuf::from(&home).join(".config/ronin")));
    }
    assert!(paths.data_dir.starts_with(temp.path()));
    assert!(paths.config_dir.starts_with(temp.path()));
}

#[test]
fn session_driver_should_seed_golden_and_produce_paint_timing() {
    let temp = TempDir::new().expect("temp");
    let paths = isolated_ronin_paths(temp.path()).expect("isolate");
    let scenarios = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
    let mut driver = SessionPaintDriver::new(paths, scenarios, Box::new(AlwaysOkSmoke));
    let timing = {
        use ronin_perf_harness::ChatPaintDriver;
        driver
            .run_chat_paint_path(ScenarioId("plain_short"))
            .expect("paint")
    };
    assert!(timing.parse > Duration::ZERO || timing.parse == Duration::from_millis(1));
    assert!(!timing.spans.is_empty());
}

#[test]
fn harness_end_to_end_plain_short_with_session_driver() {
    let temp = TempDir::new().expect("temp");
    let baseline_dir = temp.path().join("baselines");
    std::fs::create_dir_all(&baseline_dir).unwrap();
    // Generous baseline so judgment focuses on wiring, not machine noise.
    let baseline = r#"{
      "parse": 5000,
      "render": 5000,
      "wall": 10000,
      "spans": [
        { "name": "markdown.parse", "duration": 5000 },
        { "name": "chat.render", "duration": 5000 }
      ]
    }"#;
    std::fs::write(baseline_dir.join("plain_short.json"), baseline).unwrap();

    let paths = isolated_ronin_paths(&temp.path().join("iso")).unwrap();
    let scenarios = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
    let mut driver = SessionPaintDriver::new(paths, scenarios.clone(), Box::new(AlwaysOkSmoke));

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baseline_dir,
        reports_dir: temp.path().join("reports"),
        scenarios_dir: scenarios,
        budget: BudgetRules {
            max_regression_ratio: 1.0,
            parse_ceiling: Duration::from_secs(30),
            render_ceiling: Duration::from_secs(30),
            wall_ceiling: Duration::from_secs(60),
        },
        require_release: false,
        is_release_build: true,
        require_drive_smoke: true,
        isolation_paths: None,
    });

    let judgment = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect("run");
    assert!(judgment.passed, "{:?}", judgment.failures);
    assert!(temp
        .path()
        .join("reports/plain_short.judgment.json")
        .is_file());
}

#[test]
fn propose_baseline_does_not_overwrite_oracle() {
    let temp = TempDir::new().expect("temp");
    let baselines = temp.path().join("baselines");
    std::fs::create_dir_all(&baselines).unwrap();
    let oracle = baselines.join("plain_short.json");
    std::fs::write(&oracle, br#"{"parse":1,"render":1,"wall":1,"spans":[]}"#).unwrap();

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baselines.clone(),
        reports_dir: temp.path().join("reports"),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: BudgetRules {
            max_regression_ratio: 0.1,
            parse_ceiling: Duration::from_secs(1),
            render_ceiling: Duration::from_secs(1),
            wall_ceiling: Duration::from_secs(1),
        },
        require_release: false,
        is_release_build: true,
        require_drive_smoke: false,
        isolation_paths: None,
    });

    let timing = ronin_perf_harness::PaintTiming {
        parse: Duration::from_millis(9),
        render: Duration::from_millis(9),
        wall: Duration::from_millis(9),
        spans: vec![],
    };
    let proposal = harness
        .propose_baseline(ScenarioId("plain_short"), &timing)
        .expect("propose");
    assert!(proposal.is_file());
    let oracle_after = std::fs::read_to_string(&oracle).unwrap();
    assert!(oracle_after.contains("\"parse\": 1") || oracle_after.contains("\"parse\":1"));
}
