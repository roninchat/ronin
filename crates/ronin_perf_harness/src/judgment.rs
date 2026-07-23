//! Perf Judgment types.

use serde::{Deserialize, Serialize};

use crate::timing::PaintTiming;

/// A ranked hotspot pointer for agents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedHotspot {
    /// Span or family name.
    pub name: String,
    /// Duration milliseconds.
    pub duration_ms: u64,
    /// Short reason this is ranked high.
    pub reason: String,
}

/// Pass/fail result of a Perf Scenario against Perf Budgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerfJudgment {
    /// Scenario id.
    pub scenario: String,
    /// Whether the scenario passed.
    pub passed: bool,
    /// Human/agent readable failure reasons.
    pub failures: Vec<String>,
    /// Ranked hotspots (slowest / most regressing first).
    pub hotspots: Vec<RankedHotspot>,
    /// Timing observed this run.
    pub timing: PaintTiming,
}
