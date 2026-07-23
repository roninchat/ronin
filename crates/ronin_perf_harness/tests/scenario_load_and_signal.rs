//! Golden load + Improvement Signal schema corpus.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{
    load_scenario_messages, BudgetRules, ChatPaintDriver, DriveSmoke, HarnessError, IsolationPaths,
    PaintTiming, PerfHarness, PerfHarnessConfig, ScenarioId, SpanTiming,
};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

struct Fixed(PaintTiming);
impl ChatPaintDriver for Fixed {
    fn run_chat_paint_path(&mut self, _: ScenarioId<'_>) -> Result<PaintTiming, HarnessError> {
        Ok(self.0.clone())
    }
}
impl DriveSmoke for Fixed {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        Ok(())
    }
}

#[test]
fn load_all_goldens_many_times() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
    for _ in 0..100 {
        for id in ["plain_short", "heavy_fences", "long_history"] {
            let msgs = load_scenario_messages(&dir, ScenarioId(id)).unwrap();
            assert!(!msgs.is_empty(), "{id}");
        }
    }
}

#[test]
fn improvement_signal_contains_profile_and_paths() {
    for i in 0..80u64 {
        let temp = TempDir::new().unwrap();
        let baselines = temp.path().join("b");
        std::fs::create_dir_all(&baselines).unwrap();
        let t = PaintTiming {
            parse: Duration::from_millis(10),
            render: Duration::from_millis(10),
            wall: Duration::from_millis(10),
            spans: vec![SpanTiming {
                name: "markdown.parse".into(),
                duration: Duration::from_millis(10),
            }],
        };
        std::fs::write(
            baselines.join("plain_short.json"),
            serde_json::to_vec(&t).unwrap(),
        )
        .unwrap();
        let iso = IsolationPaths {
            config_dir: temp.path().join("c"),
            data_dir: temp.path().join("d"),
        };
        let harness = PerfHarness::new(PerfHarnessConfig {
            workspace_root: temp.path().to_path_buf(),
            baselines_dir: baselines,
            reports_dir: temp.path().join("r"),
            scenarios_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios"),
            budget: BudgetRules {
                max_regression_ratio: 1.0,
                parse_ceiling: Duration::from_secs(5),
                render_ceiling: Duration::from_secs(5),
                wall_ceiling: Duration::from_secs(5),
            },
            require_release: false,
            is_release_build: true,
            require_drive_smoke: false,
            isolation_paths: Some(iso),
        });
        let mut d = Fixed(t);
        let j = harness.run(ScenarioId("plain_short"), &mut d).unwrap();
        assert!(j.passed, "{i}");
        let body =
            std::fs::read_to_string(temp.path().join("r/plain_short.judgment.json")).unwrap();
        assert!(body.contains("improvement_signal"));
        assert!(body.contains("release"));
        assert!(body.contains("isolation_paths"));
        assert!(body.contains("config_dir"));
    }
}
