//! Hotspot ranking and JSON round-trip corpus.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{judge_timings, BudgetRules, PaintTiming, SpanTiming};
use std::time::Duration;

#[test]
fn hotspot_rank_orders_by_duration_desc_matrix() {
    for a in 1..=40u64 {
        for b in 1..=40u64 {
            if a == b {
                continue;
            }
            let observed = PaintTiming {
                parse: Duration::from_millis(a),
                render: Duration::from_millis(b),
                wall: Duration::from_millis(a + b),
                spans: vec![
                    SpanTiming {
                        name: "markdown.parse".into(),
                        duration: Duration::from_millis(a),
                    },
                    SpanTiming {
                        name: "chat.render".into(),
                        duration: Duration::from_millis(b),
                    },
                ],
            };
            let baseline = observed.clone();
            let j = judge_timings(
                "hot",
                &observed,
                &baseline,
                &BudgetRules {
                    max_regression_ratio: 1.0,
                    parse_ceiling: Duration::from_secs(10),
                    render_ceiling: Duration::from_secs(10),
                    wall_ceiling: Duration::from_secs(10),
                },
            );
            assert!(j.passed);
            assert!(
                j.hotspots[0].duration_ms >= j.hotspots[1].duration_ms,
                "a={a} b={b}"
            );
        }
    }
}

#[test]
fn paint_timing_json_round_trip_matrix() {
    for p in 0..50u64 {
        for r in 0..30u64 {
            let t = PaintTiming {
                parse: Duration::from_millis(p),
                render: Duration::from_millis(r),
                wall: Duration::from_millis(p + r),
                spans: vec![SpanTiming {
                    name: format!("s{p}"),
                    duration: Duration::from_millis(p),
                }],
            };
            let bytes = serde_json::to_vec(&t).unwrap();
            let back: PaintTiming = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(back.parse, t.parse);
            assert_eq!(back.render, t.render);
            assert_eq!(back.wall, t.wall);
            assert_eq!(back.spans.len(), 1);
        }
    }
}
