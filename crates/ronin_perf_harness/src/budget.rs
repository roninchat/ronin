//! Perf Budget comparison (baseline regression + disaster ceilings).

use std::time::Duration;

use crate::judgment::{PerfJudgment, RankedHotspot};
use crate::timing::PaintTiming;

/// Pass/fail rules for Chat Paint Path.
#[derive(Debug, Clone)]
pub struct BudgetRules {
    /// Max allowed relative regression vs baseline (e.g. 0.10 = 10%).
    pub max_regression_ratio: f64,
    /// Absolute parse disaster ceiling.
    pub parse_ceiling: Duration,
    /// Absolute render disaster ceiling.
    pub render_ceiling: Duration,
    /// Absolute wall-clock disaster ceiling.
    pub wall_ceiling: Duration,
}

/// Compares observed timings to baseline + ceilings.
pub fn judge_timings(
    scenario: &str,
    observed: &PaintTiming,
    baseline: &PaintTiming,
    rules: &BudgetRules,
) -> PerfJudgment {
    let mut failures = Vec::new();

    check_ceiling("parse", observed.parse, rules.parse_ceiling, &mut failures);
    check_ceiling(
        "render",
        observed.render,
        rules.render_ceiling,
        &mut failures,
    );
    check_ceiling("wall", observed.wall, rules.wall_ceiling, &mut failures);

    check_regression(
        "parse",
        observed.parse,
        baseline.parse,
        rules.max_regression_ratio,
        &mut failures,
    );
    check_regression(
        "render",
        observed.render,
        baseline.render,
        rules.max_regression_ratio,
        &mut failures,
    );
    check_regression(
        "wall",
        observed.wall,
        baseline.wall,
        rules.max_regression_ratio,
        &mut failures,
    );

    let hotspots = rank_hotspots(observed, baseline);

    PerfJudgment {
        scenario: scenario.to_string(),
        passed: failures.is_empty(),
        failures,
        hotspots,
        timing: observed.clone(),
    }
}

fn check_ceiling(name: &str, observed: Duration, ceiling: Duration, failures: &mut Vec<String>) {
    if observed > ceiling {
        failures.push(format!(
            "{name} ceiling breached: {}ms > {}ms",
            observed.as_millis(),
            ceiling.as_millis()
        ));
    }
}

fn check_regression(
    name: &str,
    observed: Duration,
    baseline: Duration,
    max_ratio: f64,
    failures: &mut Vec<String>,
) {
    if baseline.is_zero() {
        return;
    }
    let ratio = observed.as_secs_f64() / baseline.as_secs_f64() - 1.0;
    if ratio > max_ratio {
        failures.push(format!(
            "{name} regression: {:.1}% over baseline ({}ms → {}ms, max {:.1}%)",
            ratio * 100.0,
            baseline.as_millis(),
            observed.as_millis(),
            max_ratio * 100.0
        ));
    }
}

fn rank_hotspots(observed: &PaintTiming, baseline: &PaintTiming) -> Vec<RankedHotspot> {
    let mut hotspots = Vec::new();
    for span in &observed.spans {
        let base = baseline
            .spans
            .iter()
            .find(|s| s.name == span.name)
            .map(|s| s.duration);
        let reason = match base {
            Some(b) if b > Duration::ZERO && span.duration > b => {
                format!(
                    "slower than baseline ({}ms → {}ms)",
                    b.as_millis(),
                    span.duration.as_millis()
                )
            }
            _ => "largest attributed cost".into(),
        };
        hotspots.push(RankedHotspot {
            name: span.name.clone(),
            duration_ms: span.duration.as_millis() as u64,
            reason,
        });
    }
    if hotspots.is_empty() {
        hotspots.push(RankedHotspot {
            name: "markdown.parse".into(),
            duration_ms: observed.parse.as_millis() as u64,
            reason: "parse aggregate".into(),
        });
        hotspots.push(RankedHotspot {
            name: "chat.render".into(),
            duration_ms: observed.render.as_millis() as u64,
            reason: "render aggregate".into(),
        });
    }
    hotspots.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms));
    hotspots
}
