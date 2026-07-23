//! Improvement Signal on-disk contract (agents / loop consume this JSON).
#![allow(clippy::too_many_lines)]

use std::path::PathBuf;
use std::time::Duration;

use ronin_perf_harness::{
    write_improvement_signal, BudgetRules, ChatPaintDriver, DriveSmoke, HarnessError,
    IsolationPaths, PaintTiming, PerfHarness, PerfHarnessConfig, ScenarioId, SignalMeta,
    SpanTiming,
};
use tempfile::TempDir;

struct StubDriver {
    timing: PaintTiming,
}

impl ChatPaintDriver for StubDriver {
    fn run_chat_paint_path(
        &mut self,
        _scenario: ScenarioId<'_>,
    ) -> Result<PaintTiming, HarnessError> {
        Ok(self.timing.clone())
    }
}

impl DriveSmoke for StubDriver {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        Ok(())
    }
}

fn baseline_json(parse: u64, render: u64, wall: u64) -> PaintTiming {
    PaintTiming {
        parse: Duration::from_millis(parse),
        render: Duration::from_millis(render),
        wall: Duration::from_millis(wall),
        spans: vec![
            SpanTiming {
                name: "markdown.parse".into(),
                duration: Duration::from_millis(parse),
            },
            SpanTiming {
                name: "chat.render".into(),
                duration: Duration::from_millis(render),
            },
        ],
    }
}

fn harness_dirs(temp: &TempDir) -> (PathBuf, PathBuf) {
    let baselines = temp.path().join("baselines");
    std::fs::create_dir_all(&baselines).expect("baselines");
    let reports = temp.path().join("reports");
    (baselines, reports)
}

fn harness_config(
    temp: &TempDir,
    baselines: PathBuf,
    reports: PathBuf,
    isolation: Option<IsolationPaths>,
) -> PerfHarnessConfig {
    PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baselines,
        reports_dir: reports,
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: BudgetRules {
            max_regression_ratio: 0.5,
            parse_ceiling: Duration::from_secs(30),
            render_ceiling: Duration::from_secs(30),
            wall_ceiling: Duration::from_secs(60),
        },
        require_release: false,
        is_release_build: true,
        require_drive_smoke: false,
        isolation_paths: isolation,
    }
}

#[derive(Clone, Copy)]
struct SignalCase {
    name: &'static str,
    parse: u64,
    render: u64,
    wall: u64,
    profile: &'static str,
    expect_pass: bool,
}

const SIGNAL_CASES: &[SignalCase] = &[
    SignalCase {
        name: "plain_short",
        parse: 40,
        render: 50,
        wall: 60,
        profile: "release",
        expect_pass: true,
    },
    SignalCase {
        name: "heavy_fences",
        parse: 80,
        render: 120,
        wall: 200,
        profile: "release",
        expect_pass: true,
    },
    SignalCase {
        name: "long_history",
        parse: 100,
        render: 150,
        wall: 250,
        profile: "release",
        expect_pass: true,
    },
];

#[test]
fn improvement_signal_json_contract_matrix() {
    for c in SIGNAL_CASES {
        let temp = TempDir::new().expect("temp");
        let (baselines, reports) = harness_dirs(&temp);
        let base = baseline_json(c.parse, c.render, c.wall);
        std::fs::write(
            baselines.join(format!("{}.json", c.name)),
            serde_json::to_vec_pretty(&base).expect("ser"),
        )
        .expect("write baseline");

        let isolation = IsolationPaths {
            config_dir: temp.path().join("cfg"),
            data_dir: temp.path().join("data"),
        };
        let harness = PerfHarness::new(harness_config(
            &temp,
            baselines,
            reports.clone(),
            Some(isolation.clone()),
        ));

        let mut driver = StubDriver {
            timing: base.clone(),
        };
        let judgment = harness.run(ScenarioId(c.name), &mut driver).expect("run");

        assert_eq!(judgment.passed, c.expect_pass, "{}", c.name);

        let path = reports.join(format!("{}.judgment.json", c.name));
        assert!(path.is_file(), "{}", c.name);
        let body: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("json");

        assert_eq!(body["kind"], "improvement_signal");
        assert_eq!(body["scenario"], c.name);
        assert_eq!(body["build_profile"], c.profile);
        assert_eq!(body["passed"], c.expect_pass);
        assert!(body["failures"].is_array());
        assert!(body["hotspots"].is_array());
        assert!(body["timing"]["parse"].is_number());
        assert!(body["timing"]["render"].is_number());
        assert!(body["timing"]["wall"].is_number());
        assert!(body["isolation_paths"]["config_dir"].is_string());
        assert!(body["isolation_paths"]["data_dir"].is_string());
    }
}

#[test]
fn write_improvement_signal_direct_round_trip() {
    let temp = TempDir::new().expect("temp");
    let reports = temp.path().join("reports");
    let timing = baseline_json(3, 4, 5);
    let judgment = ronin_perf_harness::judge_timings(
        "plain_short",
        &timing,
        &timing,
        &BudgetRules {
            max_regression_ratio: 0.1,
            parse_ceiling: Duration::from_secs(1),
            render_ceiling: Duration::from_secs(1),
            wall_ceiling: Duration::from_secs(2),
        },
    );
    write_improvement_signal(
        &reports,
        "plain_short",
        &judgment,
        &timing,
        SignalMeta {
            build_profile: "release",
            isolation_paths: None,
        },
    )
    .expect("write");

    let raw = std::fs::read_to_string(reports.join("plain_short.judgment.json")).expect("read");
    assert!(raw.contains("\"kind\": \"improvement_signal\""));
    assert!(raw.contains("\"passed\": true") || raw.contains("\"passed\":true"));
}

#[test]
fn accept_baseline_promotes_proposal_into_oracle() {
    let temp = TempDir::new().expect("temp");
    let (baselines, reports) = harness_dirs(&temp);
    std::fs::write(
        baselines.join("plain_short.json"),
        br#"{"parse":1,"render":1,"wall":1,"spans":[]}"#,
    )
    .unwrap();

    let harness = PerfHarness::new(harness_config(&temp, baselines.clone(), reports, None));
    let timing = baseline_json(42, 43, 44);
    harness
        .propose_baseline(ScenarioId("plain_short"), &timing)
        .expect("propose");

    let dest = harness
        .accept_baseline_proposal(ScenarioId("plain_short"))
        .expect("accept");
    assert_eq!(dest, baselines.join("plain_short.json"));

    let promoted: PaintTiming =
        serde_json::from_slice(&std::fs::read(&dest).expect("read oracle")).expect("parse");
    assert_eq!(promoted.parse, Duration::from_millis(42));
    assert_eq!(promoted.render, Duration::from_millis(43));
    assert_eq!(promoted.wall, Duration::from_millis(44));
}

#[test]
fn accept_baseline_errors_when_proposal_missing() {
    let temp = TempDir::new().expect("temp");
    let (baselines, reports) = harness_dirs(&temp);
    let harness = PerfHarness::new(harness_config(&temp, baselines, reports, None));
    let err = harness
        .accept_baseline_proposal(ScenarioId("plain_short"))
        .expect_err("no proposal");
    assert!(matches!(err, HarnessError::Baseline(_)));
}
