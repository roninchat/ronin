//! HarnessError / DriveSmoke / JudgmentProfile matrix.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{
    AlwaysOkSmoke, BudgetRules, ChatPaintDriver, DisplayDriveSmoke, DriveSmoke, HarnessError,
    PaintTiming, PerfHarness, PerfHarnessConfig, ScenarioId, SpanTiming,
};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

struct Fixed {
    t: PaintTiming,
    smoke: bool,
}
impl ChatPaintDriver for Fixed {
    fn run_chat_paint_path(&mut self, _: ScenarioId<'_>) -> Result<PaintTiming, HarnessError> {
        Ok(self.t.clone())
    }
}
impl DriveSmoke for Fixed {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        if self.smoke {
            Ok(())
        } else {
            Err(HarnessError::DriveSmokeFailed("x".into()))
        }
    }
}

fn t(ms: u64) -> PaintTiming {
    PaintTiming {
        parse: Duration::from_millis(ms),
        render: Duration::from_millis(ms),
        wall: Duration::from_millis(ms),
        spans: vec![SpanTiming {
            name: "markdown.parse".into(),
            duration: Duration::from_millis(ms),
        }],
    }
}

#[test]
fn judgment_profile_refuses_debug_matrix() {
    for i in 0..100 {
        let temp = TempDir::new().unwrap();
        let harness = PerfHarness::new(PerfHarnessConfig {
            workspace_root: temp.path().to_path_buf(),
            baselines_dir: temp.path().join("b"),
            reports_dir: temp.path().join("r"),
            scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
            budget: BudgetRules {
                max_regression_ratio: 0.1,
                parse_ceiling: Duration::from_secs(1),
                render_ceiling: Duration::from_secs(1),
                wall_ceiling: Duration::from_secs(1),
            },
            require_release: true,
            is_release_build: false,
            require_drive_smoke: false,
            isolation_paths: None,
        });
        let mut d = Fixed {
            t: t(1 + (i % 5)),
            smoke: true,
        };
        let err = harness.run(ScenarioId("plain_short"), &mut d).unwrap_err();
        assert!(matches!(err, HarnessError::JudgmentProfile(_)), "{i}");
    }
}

#[test]
fn drive_smoke_required_fails_matrix() {
    for i in 0..80 {
        let temp = TempDir::new().unwrap();
        let baselines = temp.path().join("b");
        std::fs::create_dir_all(&baselines).unwrap();
        std::fs::write(
            baselines.join("plain_short.json"),
            serde_json::to_vec(&t(10)).unwrap(),
        )
        .unwrap();
        let harness = PerfHarness::new(PerfHarnessConfig {
            workspace_root: temp.path().to_path_buf(),
            baselines_dir: baselines,
            reports_dir: temp.path().join("r"),
            scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
            budget: BudgetRules {
                max_regression_ratio: 1.0,
                parse_ceiling: Duration::from_secs(10),
                render_ceiling: Duration::from_secs(10),
                wall_ceiling: Duration::from_secs(10),
            },
            require_release: false,
            is_release_build: true,
            require_drive_smoke: true,
            isolation_paths: None,
        });
        let mut d = Fixed {
            t: t(10),
            smoke: false,
        };
        let err = harness.run(ScenarioId("plain_short"), &mut d).unwrap_err();
        assert!(matches!(err, HarnessError::DriveSmokeFailed(_)), "{i}");
    }
}

#[test]
fn always_ok_smoke_succeeds_repeatedly() {
    for _ in 0..200 {
        AlwaysOkSmoke.run_drive_smoke().unwrap();
    }
}

#[test]
fn display_smoke_is_deterministic_for_env() {
    let has =
        std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some();
    for _ in 0..50 {
        let r = DisplayDriveSmoke.run_drive_smoke();
        if has {
            r.unwrap();
        } else {
            assert!(r.is_err());
        }
    }
}

#[test]
fn unknown_scenario_rejected_by_harness() {
    for id in (0..100).map(|i| format!("nope_{i}")) {
        let temp = TempDir::new().unwrap();
        let harness = PerfHarness::new(PerfHarnessConfig {
            workspace_root: temp.path().to_path_buf(),
            baselines_dir: temp.path().join("b"),
            reports_dir: temp.path().join("r"),
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
        let mut d = Fixed {
            t: t(1),
            smoke: true,
        };
        let err = harness.run(ScenarioId(&id), &mut d).unwrap_err();
        assert!(matches!(err, HarnessError::Scenario(_)), "{id}");
    }
}
