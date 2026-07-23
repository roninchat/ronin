//! Improvement Signal on disk.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::HarnessError;
use crate::judgment::PerfJudgment;
use crate::timing::PaintTiming;

#[derive(Serialize)]
struct ImprovementSignal<'a> {
    scenario: &'a str,
    passed: bool,
    failures: &'a [String],
    hotspots: &'a [crate::judgment::RankedHotspot],
    timing: &'a PaintTiming,
    kind: &'static str,
    /// `release` or `debug`.
    build_profile: &'a str,
    /// Isolated config/data roots used for the run, when known.
    isolation_paths: Option<&'a IsolationPaths>,
}

/// Paths recorded on the Improvement Signal for agent context.
#[derive(Debug, Clone, Serialize)]
pub struct IsolationPaths {
    /// Isolated config directory.
    pub config_dir: PathBuf,
    /// Isolated data directory.
    pub data_dir: PathBuf,
}

/// Metadata bundled into the Improvement Signal.
#[derive(Debug, Clone)]
pub struct SignalMeta<'a> {
    /// Build profile label (`release` / `debug`).
    pub build_profile: &'a str,
    /// Optional isolation paths.
    pub isolation_paths: Option<&'a IsolationPaths>,
}

/// Writes the machine-readable Improvement Signal for agents / loop.
pub fn write_improvement_signal(
    reports_dir: &Path,
    scenario: &str,
    judgment: &PerfJudgment,
    timing: &PaintTiming,
    meta: SignalMeta<'_>,
) -> Result<(), HarnessError> {
    std::fs::create_dir_all(reports_dir)?;
    let path = reports_dir.join(format!("{scenario}.judgment.json"));
    let signal = ImprovementSignal {
        scenario,
        passed: judgment.passed,
        failures: &judgment.failures,
        hotspots: &judgment.hotspots,
        timing,
        kind: "improvement_signal",
        build_profile: meta.build_profile,
        isolation_paths: meta.isolation_paths,
    };
    std::fs::write(path, serde_json::to_vec_pretty(&signal)?)?;
    Ok(())
}
