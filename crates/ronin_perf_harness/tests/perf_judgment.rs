//! Behavior tests at the PerfHarness seam (spec #87).

use std::path::PathBuf;
use std::time::Duration;

use ronin_perf_harness::{
    BudgetRules, ChatPaintDriver, DriveSmoke, HarnessError, PaintTiming, PerfHarness,
    PerfHarnessConfig, ScenarioId, SpanTiming,
};
use tempfile::TempDir;

struct FixedDriver {
    timing: PaintTiming,
    smoke_ok: bool,
}

impl ChatPaintDriver for FixedDriver {
    fn run_chat_paint_path(
        &mut self,
        _scenario: ScenarioId<'_>,
    ) -> Result<PaintTiming, HarnessError> {
        Ok(self.timing.clone())
    }
}

impl DriveSmoke for FixedDriver {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        if self.smoke_ok {
            Ok(())
        } else {
            Err(HarnessError::DriveSmokeFailed("window not operable".into()))
        }
    }
}

fn rules_pass() -> BudgetRules {
    BudgetRules {
        max_regression_ratio: 0.10,
        parse_ceiling: Duration::from_millis(500),
        render_ceiling: Duration::from_millis(500),
        wall_ceiling: Duration::from_secs(5),
    }
}

#[test]
fn harness_should_pass_when_timings_within_baseline_and_ceilings() {
    let temp = TempDir::new().expect("temp");
    let baseline_dir = temp.path().join("baselines");
    std::fs::create_dir_all(&baseline_dir).expect("baselines dir");
    let report_dir = temp.path().join("reports");

    let baseline = PaintTiming {
        parse: Duration::from_millis(40),
        render: Duration::from_millis(80),
        wall: Duration::from_millis(150),
        spans: vec![
            SpanTiming {
                name: "markdown.parse".into(),
                duration: Duration::from_millis(40),
            },
            SpanTiming {
                name: "chat.render".into(),
                duration: Duration::from_millis(80),
            },
        ],
    };
    std::fs::write(
        baseline_dir.join("plain_short.json"),
        serde_json::to_vec_pretty(&baseline).expect("ser"),
    )
    .expect("write baseline");

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baseline_dir,
        reports_dir: report_dir.clone(),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: rules_pass(),
        require_release: false,
        is_release_build: true,
        require_drive_smoke: true,
        isolation_paths: None,
    });

    let mut driver = FixedDriver {
        timing: PaintTiming {
            parse: Duration::from_millis(42),
            render: Duration::from_millis(85),
            wall: Duration::from_millis(160),
            spans: baseline.spans.clone(),
        },
        smoke_ok: true,
    };

    let judgment = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect("run");

    assert!(judgment.passed, "expected pass: {:?}", judgment);
    let report = report_dir.join("plain_short.judgment.json");
    assert!(report.is_file(), "Improvement Signal must be on disk");
    let body = std::fs::read_to_string(&report).expect("read report");
    assert!(body.contains("\"passed\": true") || body.contains("\"passed\":true"));
}

#[test]
fn harness_should_fail_on_baseline_regression() {
    let temp = TempDir::new().expect("temp");
    let baseline_dir = temp.path().join("baselines");
    std::fs::create_dir_all(&baseline_dir).expect("baselines dir");

    let baseline = PaintTiming {
        parse: Duration::from_millis(40),
        render: Duration::from_millis(80),
        wall: Duration::from_millis(150),
        spans: vec![],
    };
    std::fs::write(
        baseline_dir.join("plain_short.json"),
        serde_json::to_vec_pretty(&baseline).expect("ser"),
    )
    .expect("write baseline");

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baseline_dir,
        reports_dir: temp.path().join("reports"),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: rules_pass(),
        require_release: false,
        is_release_build: true,
        require_drive_smoke: true,
        isolation_paths: None,
    });

    let mut driver = FixedDriver {
        timing: PaintTiming {
            parse: Duration::from_millis(80), // 100% regression
            render: Duration::from_millis(80),
            wall: Duration::from_millis(200),
            spans: vec![SpanTiming {
                name: "markdown.parse".into(),
                duration: Duration::from_millis(80),
            }],
        },
        smoke_ok: true,
    };

    let judgment = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect("run");

    assert!(!judgment.passed);
    assert!(
        judgment
            .failures
            .iter()
            .any(|f| f.contains("regression") || f.contains("parse")),
        "failures: {:?}",
        judgment.failures
    );
}

#[test]
fn harness_should_fail_when_disaster_ceiling_breached() {
    let temp = TempDir::new().expect("temp");
    let baseline_dir = temp.path().join("baselines");
    std::fs::create_dir_all(&baseline_dir).expect("baselines dir");

    let baseline = PaintTiming {
        parse: Duration::from_millis(40),
        render: Duration::from_millis(80),
        wall: Duration::from_millis(150),
        spans: vec![],
    };
    std::fs::write(
        baseline_dir.join("plain_short.json"),
        serde_json::to_vec_pretty(&baseline).expect("ser"),
    )
    .expect("write baseline");

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baseline_dir,
        reports_dir: temp.path().join("reports"),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: BudgetRules {
            max_regression_ratio: 10.0, // ignore regression
            parse_ceiling: Duration::from_millis(50),
            render_ceiling: Duration::from_millis(500),
            wall_ceiling: Duration::from_secs(5),
        },
        require_release: false,
        is_release_build: true,
        require_drive_smoke: true,
        isolation_paths: None,
    });

    let mut driver = FixedDriver {
        timing: PaintTiming {
            parse: Duration::from_millis(200),
            render: Duration::from_millis(80),
            wall: Duration::from_millis(300),
            spans: vec![],
        },
        smoke_ok: true,
    };

    let judgment = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect("run");

    assert!(!judgment.passed);
    assert!(
        judgment.failures.iter().any(|f| f.contains("ceiling")),
        "failures: {:?}",
        judgment.failures
    );
}

#[test]
fn harness_should_fail_when_drive_smoke_fails() {
    let temp = TempDir::new().expect("temp");
    let baseline_dir = temp.path().join("baselines");
    std::fs::create_dir_all(&baseline_dir).expect("baselines dir");

    let baseline = PaintTiming {
        parse: Duration::from_millis(40),
        render: Duration::from_millis(80),
        wall: Duration::from_millis(150),
        spans: vec![],
    };
    std::fs::write(
        baseline_dir.join("plain_short.json"),
        serde_json::to_vec_pretty(&baseline).expect("ser"),
    )
    .expect("write baseline");

    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: baseline_dir,
        reports_dir: temp.path().join("reports"),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: rules_pass(),
        require_release: false,
        is_release_build: true,
        require_drive_smoke: true,
        isolation_paths: None,
    });

    let mut driver = FixedDriver {
        timing: baseline.clone(),
        smoke_ok: false,
    };

    let err = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect_err("smoke must fail closed");
    assert!(matches!(err, HarnessError::DriveSmokeFailed(_)));
}

#[test]
fn harness_should_refuse_official_run_on_debug_when_release_required() {
    let temp = TempDir::new().expect("temp");
    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: temp.path().to_path_buf(),
        baselines_dir: temp.path().join("baselines"),
        reports_dir: temp.path().join("reports"),
        scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
        budget: rules_pass(),
        require_release: true,
        is_release_build: false,
        require_drive_smoke: false,
        isolation_paths: None,
    });

    let mut driver = FixedDriver {
        timing: PaintTiming {
            parse: Duration::from_millis(1),
            render: Duration::from_millis(1),
            wall: Duration::from_millis(1),
            spans: vec![],
        },
        smoke_ok: true,
    };

    let err = harness
        .run(ScenarioId("plain_short"), &mut driver)
        .expect_err("debug must not be official");
    assert!(matches!(err, HarnessError::JudgmentProfile(_)));
}
