//! SessionPaintDriver dense golden seeding.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{
    generate_scale_messages, isolated_ronin_paths, AlwaysOkSmoke, ChatPaintDriver, ScenarioId,
    SessionPaintDriver,
};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn session_paint_dense_all_goldens_repeated() {
    let scenarios = ["plain_short", "heavy_fences", "long_history"];
    for round in 0..25 {
        for scenario in scenarios {
            let temp = TempDir::new().unwrap();
            let paths = isolated_ronin_paths(temp.path()).unwrap();
            let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
            let mut driver = SessionPaintDriver::new(paths, scenarios_dir, Box::new(AlwaysOkSmoke));
            let timing = driver.run_chat_paint_path(ScenarioId(scenario)).unwrap();
            assert!(
                timing.parse >= Duration::from_millis(1),
                "{scenario} r={round}"
            );
            assert!(
                timing.render >= Duration::from_millis(1),
                "{scenario} r={round}"
            );
            assert_eq!(timing.spans.len(), 2);
        }
    }
}

#[test]
fn session_paint_dense_generator_overrides() {
    for n in [1usize, 2, 5, 10, 15, 20, 30, 40, 50] {
        for with_fences in [false, true] {
            let temp = TempDir::new().unwrap();
            let paths = isolated_ronin_paths(temp.path()).unwrap();
            let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
            let msgs = generate_scale_messages(n, with_fences);
            let mut driver = SessionPaintDriver::new(paths, scenarios_dir, Box::new(AlwaysOkSmoke))
                .with_messages(msgs);
            let timing = driver
                .run_chat_paint_path(ScenarioId("plain_short"))
                .unwrap();
            assert!(
                timing.wall >= Duration::from_millis(1),
                "n={n} fences={with_fences}"
            );
        }
    }
}
