//! Regression/ceiling cases with independent expected outcomes (no tautologies).
#![allow(clippy::too_many_lines)]

use std::time::Duration;

use ronin_perf_harness::{judge_timings, BudgetRules, PaintTiming, SpanTiming};

fn t(p: u64, r: u64, w: u64) -> PaintTiming {
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

fn rules(max_reg: f64) -> BudgetRules {
    BudgetRules {
        max_regression_ratio: max_reg,
        parse_ceiling: Duration::from_secs(60),
        render_ceiling: Duration::from_secs(60),
        wall_ceiling: Duration::from_secs(60),
    }
}

#[test]
fn known_pass_and_fail_literals() {
    // Exactly at +10% with max 10% → pass (not strictly greater).
    let j = judge_timings("eq", &t(110, 100, 100), &t(100, 100, 100), &rules(0.10));
    assert!(j.passed, "{:?}", j.failures);

    // +11% parse → fail.
    let j = judge_timings("fail", &t(111, 100, 100), &t(100, 100, 100), &rules(0.10));
    assert!(!j.passed);
    assert!(j.failures.iter().any(|f| f.contains("parse regression")));

    // Double render → fail.
    let j = judge_timings("r2", &t(100, 200, 100), &t(100, 100, 100), &rules(0.10));
    assert!(!j.passed);

    // Wall double → fail.
    let j = judge_timings("w2", &t(100, 100, 200), &t(100, 100, 100), &rules(0.10));
    assert!(!j.passed);

    // Zero baseline never regresses.
    let j = judge_timings("z", &t(500, 500, 500), &t(0, 0, 0), &rules(0.01));
    assert!(j.passed, "{:?}", j.failures);
}

#[test]
fn ceiling_literals() {
    let tight = BudgetRules {
        max_regression_ratio: 10.0,
        parse_ceiling: Duration::from_millis(50),
        render_ceiling: Duration::from_millis(50),
        wall_ceiling: Duration::from_millis(50),
    };
    let j = judge_timings("c", &t(51, 10, 10), &t(10, 10, 10), &tight);
    assert!(!j.passed);
    assert!(j.failures.iter().any(|f| f.contains("parse ceiling")));
}

#[test]
fn dense_near_baseline_passes() {
    for base in [20u64, 40, 80, 100, 160, 200, 250, 300] {
        for add in 0..=5 {
            let j = judge_timings(
                "near",
                &t(base + add, base, base),
                &t(base, base, base),
                &rules(0.25),
            );
            assert!(j.passed, "base={base} add={add} {:?}", j.failures);
        }
    }
}

#[test]
fn dense_clear_regressions_fail() {
    for base in [20u64, 40, 80, 100, 160, 200] {
        let obs = base * 2;
        let j = judge_timings(
            "reg",
            &t(obs, base, base),
            &t(base, base, base),
            &rules(0.10),
        );
        assert!(!j.passed, "base={base}");
    }
}
