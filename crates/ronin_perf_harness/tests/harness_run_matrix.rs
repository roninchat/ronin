//! End-to-end PerfHarness run matrix through the public seam.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{
    BudgetRules, ChatPaintDriver, DriveSmoke, HarnessError, PaintTiming, PerfHarness,
    PerfHarnessConfig, ScenarioId, SpanTiming,
};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

struct FixedDriver {
    timing: PaintTiming,
    smoke_ok: bool,
}
impl ChatPaintDriver for FixedDriver {
    fn run_chat_paint_path(&mut self, _: ScenarioId<'_>) -> Result<PaintTiming, HarnessError> {
        Ok(self.timing.clone())
    }
}
impl DriveSmoke for FixedDriver {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        if self.smoke_ok {
            Ok(())
        } else {
            Err(HarnessError::DriveSmokeFailed("no".into()))
        }
    }
}

fn timing(p: u64, r: u64, w: u64) -> PaintTiming {
    PaintTiming {
        parse: Duration::from_millis(p),
        render: Duration::from_millis(r),
        wall: Duration::from_millis(w),
        spans: vec![
            SpanTiming {
                name: "markdown.parse".into(),
                duration: Duration::from_millis(p),
            },
            SpanTiming {
                name: "chat.render".into(),
                duration: Duration::from_millis(r),
            },
        ],
    }
}

fn write_baseline(dir: &std::path::Path, id: &str, t: &PaintTiming) {
    std::fs::write(
        dir.join(format!("{id}.json")),
        serde_json::to_vec_pretty(t).unwrap(),
    )
    .unwrap();
}

#[test]
fn harness_run_matrix_pass_and_fail_across_scenarios() {
    let scenarios = ["plain_short", "heavy_fences", "long_history"];
    for (si, scenario) in scenarios.iter().enumerate() {
        for bump in 0..40u64 {
            let temp = TempDir::new().unwrap();
            let baselines = temp.path().join("baselines");
            std::fs::create_dir_all(&baselines).unwrap();
            let base = timing(50 + si as u64, 80 + si as u64, 150 + si as u64);
            write_baseline(&baselines, scenario, &base);
            let obs = timing(50 + si as u64 + bump % 3, 80 + si as u64, 150 + si as u64);
            let harness = PerfHarness::new(PerfHarnessConfig {
                workspace_root: temp.path().to_path_buf(),
                baselines_dir: baselines,
                reports_dir: temp.path().join("reports"),
                scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
                budget: BudgetRules {
                    max_regression_ratio: 0.25,
                    parse_ceiling: Duration::from_secs(5),
                    render_ceiling: Duration::from_secs(5),
                    wall_ceiling: Duration::from_secs(10),
                },
                require_release: false,
                is_release_build: true,
                require_drive_smoke: true,
                isolation_paths: None,
            });
            let mut driver = FixedDriver {
                timing: obs,
                smoke_ok: true,
            };
            let j = harness.run(ScenarioId(scenario), &mut driver).unwrap();
            assert!(j.passed, "{scenario} bump={bump} {:?}", j.failures);
            assert!(temp
                .path()
                .join(format!("reports/{scenario}.judgment.json"))
                .is_file());
        }
    }
}

#[test]
fn harness_run_matrix_regression_fails() {
    for i in 0..80u64 {
        let temp = TempDir::new().unwrap();
        let baselines = temp.path().join("baselines");
        std::fs::create_dir_all(&baselines).unwrap();
        let base = timing(40, 80, 120);
        write_baseline(&baselines, "plain_short", &base);
        let obs = timing(40 + 40 + i % 20, 80, 120);
        let harness = PerfHarness::new(PerfHarnessConfig {
            workspace_root: temp.path().to_path_buf(),
            baselines_dir: baselines,
            reports_dir: temp.path().join("reports"),
            scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
            budget: BudgetRules {
                max_regression_ratio: 0.10,
                parse_ceiling: Duration::from_secs(30),
                render_ceiling: Duration::from_secs(30),
                wall_ceiling: Duration::from_secs(60),
            },
            require_release: false,
            is_release_build: true,
            require_drive_smoke: false,
            isolation_paths: None,
        });
        let mut driver = FixedDriver {
            timing: obs,
            smoke_ok: true,
        };
        let j = harness.run(ScenarioId("plain_short"), &mut driver).unwrap();
        assert!(!j.passed, "i={i}");
    }
}

#[test]
fn harness_propose_and_accept_baseline_matrix() {
    for i in 0..60u64 {
        let temp = TempDir::new().unwrap();
        let baselines = temp.path().join("baselines");
        std::fs::create_dir_all(&baselines).unwrap();
        let oracle = timing(1, 1, 1);
        write_baseline(&baselines, "plain_short", &oracle);
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
        let proposed = timing(10 + i, 11 + i, 12 + i);
        let path = harness
            .propose_baseline(ScenarioId("plain_short"), &proposed)
            .unwrap();
        assert!(path.is_file());
        let before = std::fs::read_to_string(baselines.join("plain_short.json")).unwrap();
        assert!(before.contains("\"parse\": 1") || before.contains("\"parse\":1"));
        let dest = harness
            .accept_baseline_proposal(ScenarioId("plain_short"))
            .unwrap();
        assert!(dest.is_file());
        let after = std::fs::read_to_string(dest).unwrap();
        let needle_a = format!("\"parse\": {}", 10 + i);
        let needle_b = format!("\"parse\":{}", 10 + i);
        assert!(
            after.contains(&needle_a) || after.contains(&needle_b),
            "{after}"
        );
    }
}
