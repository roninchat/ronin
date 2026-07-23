//! Perf Harness — Chat Paint Path sensor+judge (tooling track; not a product surface).
//!
//! Primary seam: [`PerfHarness`]. See spec #87 and `CONTEXT.md`.

#![deny(missing_docs)]

mod budget;
mod error;
mod isolation;
mod judgment;
mod paint;
mod report;
mod scenario;
mod session_driver;
mod timing;

pub use budget::{judge_timings, BudgetRules};
pub use error::HarnessError;
pub use isolation::isolated_ronin_paths;
pub use judgment::{PerfJudgment, RankedHotspot};
pub use paint::{
    ceil_to_millis, measure_chat_paint, parse_markdown_blocks, AlwaysOkSmoke, ChatPaintDriver,
    DisplayDriveSmoke, DriveSmoke,
};
pub use report::{write_improvement_signal, IsolationPaths, SignalMeta};
pub use scenario::{generate_scale_messages, load_scenario_messages, ScenarioId, ScenarioKind};
pub use session_driver::SessionPaintDriver;
pub use timing::{PaintTiming, SpanTiming};

use std::path::PathBuf;

/// Configuration for a PerfHarness run.
#[derive(Debug, Clone)]
pub struct PerfHarnessConfig {
    /// Repo or harness workspace root (informational / relative resolution).
    pub workspace_root: PathBuf,
    /// Directory of baseline `PaintTiming` JSON files named `{scenario}.json`.
    pub baselines_dir: PathBuf,
    /// Directory where Improvement Signal JSON reports are written.
    pub reports_dir: PathBuf,
    /// Directory containing scenario goldens / manifests.
    pub scenarios_dir: PathBuf,
    /// Perf Budget rules.
    pub budget: BudgetRules,
    /// When true, refuse official runs unless `is_release_build`.
    pub require_release: bool,
    /// Whether this process is an optimized/release build.
    pub is_release_build: bool,
    /// When true, Drive Smoke must succeed before judgment.
    pub require_drive_smoke: bool,
    /// Optional isolation paths recorded on the Improvement Signal.
    pub isolation_paths: Option<IsolationPaths>,
}

/// Deep runner seam: scenario → Paint Timing → Perf Judgment → Improvement Signal.
pub struct PerfHarness {
    config: PerfHarnessConfig,
}

impl PerfHarness {
    /// Creates a harness from config.
    pub fn new(config: PerfHarnessConfig) -> Self {
        Self { config }
    }

    /// Runs one Perf Scenario through the driver and returns a Perf Judgment.
    pub fn run<D>(
        &self,
        scenario: ScenarioId<'_>,
        driver: &mut D,
    ) -> Result<PerfJudgment, HarnessError>
    where
        D: ChatPaintDriver + DriveSmoke,
    {
        if self.config.require_release && !self.config.is_release_build {
            return Err(HarnessError::JudgmentProfile(
                "official Perf Judgments require a release Harness Build (--release)".into(),
            ));
        }

        let _ = ScenarioKind::parse(scenario.0)?;

        if self.config.require_drive_smoke {
            driver.run_drive_smoke()?;
        }

        let timing = driver.run_chat_paint_path(scenario)?;
        let baseline = load_baseline(&self.config.baselines_dir, scenario)?;
        let judgment = judge_timings(scenario.0, &timing, &baseline, &self.config.budget);
        let profile = if self.config.is_release_build {
            "release"
        } else {
            "debug"
        };
        write_improvement_signal(
            &self.config.reports_dir,
            scenario.0,
            &judgment,
            &timing,
            SignalMeta {
                build_profile: profile,
                isolation_paths: self.config.isolation_paths.as_ref(),
            },
        )?;
        Ok(judgment)
    }

    /// Writes a proposed baseline bump (does not accept/promote the oracle).
    pub fn propose_baseline(
        &self,
        scenario: ScenarioId<'_>,
        timing: &PaintTiming,
    ) -> Result<PathBuf, HarnessError> {
        let dir = self.config.reports_dir.join("baseline_proposals");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.json", scenario.0));
        std::fs::write(&path, serde_json::to_vec_pretty(timing)?)?;
        Ok(path)
    }

    /// Explicitly accepts a baseline proposal into the oracle baselines dir (ADR-0003).
    pub fn accept_baseline_proposal(
        &self,
        scenario: ScenarioId<'_>,
    ) -> Result<PathBuf, HarnessError> {
        let proposal = self
            .config
            .reports_dir
            .join("baseline_proposals")
            .join(format!("{}.json", scenario.0));
        if !proposal.is_file() {
            return Err(HarnessError::Baseline(format!(
                "no proposal at {}",
                proposal.display()
            )));
        }
        std::fs::create_dir_all(&self.config.baselines_dir)?;
        let dest = self
            .config
            .baselines_dir
            .join(format!("{}.json", scenario.0));
        std::fs::copy(&proposal, &dest)?;
        Ok(dest)
    }
}

fn load_baseline(
    dir: &std::path::Path,
    scenario: ScenarioId<'_>,
) -> Result<PaintTiming, HarnessError> {
    let path = dir.join(format!("{}.json", scenario.0));
    let bytes = std::fs::read(&path)
        .map_err(|e| HarnessError::Baseline(format!("missing baseline {}: {e}", path.display())))?;
    Ok(serde_json::from_slice(&bytes)?)
}
