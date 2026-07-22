//! CLI for the Perf Harness tooling track.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use ronin_perf_harness::{
    generate_scale_messages, isolated_ronin_paths, AlwaysOkSmoke, BudgetRules, DisplayDriveSmoke,
    DriveSmoke, PerfHarness, PerfHarnessConfig, ScenarioId, SessionPaintDriver,
};

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("ronin-perf-harness: {e}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".into());
    match cmd.as_str() {
        "help" | "-h" | "--help" => {
            print_help();
            Ok(true)
        }
        "run" => {
            let scenario = args
                .next()
                .ok_or("usage: ronin-perf-harness run <scenario_id>")?;
            let require_smoke = !args.any(|a| a == "--skip-smoke");
            run_scenario(&scenario, require_smoke)
        }
        "propose-baseline" => {
            let scenario = args
                .next()
                .ok_or("usage: ronin-perf-harness propose-baseline <scenario_id>")?;
            propose_baseline(&scenario)
        }
        "accept-baseline" => {
            let scenario = args
                .next()
                .ok_or("usage: ronin-perf-harness accept-baseline <scenario_id>")?;
            accept_baseline(&scenario)
        }
        "generate-sweep" => {
            let n: usize = args
                .next()
                .unwrap_or_else(|| "50".into())
                .parse()
                .map_err(|_| "generate-sweep expects message count")?;
            run_generated_sweep(n)
        }
        other => Err(format!("unknown command: {other}").into()),
    }
}

fn print_help() {
    println!(
        "\
ronin-perf-harness — Chat Paint Path sensor+judge (tooling; not product)

Commands:
  run <scenario_id> [--skip-smoke]
  propose-baseline <scenario_id>
  accept-baseline <scenario_id>
  generate-sweep [count]

Scenarios: plain_short | heavy_fences | long_history

Official judgments: build with --release (debug is exploratory only).
"
    );
}

fn default_dirs() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let scenarios = manifest.join("scenarios");
    let baselines = manifest.join("baselines");
    let reports = workspace.join("target/perf-harness/reports");
    (workspace, scenarios, baselines, reports)
}

fn budget_rules() -> BudgetRules {
    BudgetRules {
        max_regression_ratio: 0.25,
        parse_ceiling: Duration::from_secs(2),
        render_ceiling: Duration::from_secs(5),
        wall_ceiling: Duration::from_secs(15),
    }
}

fn run_scenario(scenario: &str, require_smoke: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let (workspace, scenarios, baselines, reports) = default_dirs();
    let isolation_root = workspace.join("target/perf-harness/isolated");
    if isolation_root.exists() {
        std::fs::remove_dir_all(&isolation_root)?;
    }
    let paths = isolated_ronin_paths(&isolation_root)?;
    let iso = ronin_perf_harness::IsolationPaths {
        config_dir: paths.config_dir.clone(),
        data_dir: paths.data_dir.clone(),
    };

    let smoke: Box<dyn DriveSmoke + Send> = if require_smoke {
        Box::new(DisplayDriveSmoke)
    } else {
        Box::new(AlwaysOkSmoke)
    };

    let mut driver = SessionPaintDriver::new(paths, scenarios.clone(), smoke);
    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: workspace,
        baselines_dir: baselines,
        reports_dir: reports.clone(),
        scenarios_dir: scenarios,
        budget: budget_rules(),
        require_release: true,
        is_release_build: !cfg!(debug_assertions),
        require_drive_smoke: require_smoke,
        isolation_paths: Some(iso),
    });

    let judgment = harness.run(ScenarioId(scenario), &mut driver)?;
    println!(
        "scenario={scenario} passed={} report={}/{scenario}.judgment.json",
        judgment.passed,
        reports.display(),
    );
    for f in &judgment.failures {
        println!("  fail: {f}");
    }
    for h in judgment.hotspots.iter().take(5) {
        println!("  hotspot: {} {}ms ({})", h.name, h.duration_ms, h.reason);
    }
    Ok(judgment.passed)
}

fn propose_baseline(scenario: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let (workspace, scenarios, baselines, reports) = default_dirs();
    let isolation_root = workspace.join("target/perf-harness/isolated-propose");
    if isolation_root.exists() {
        std::fs::remove_dir_all(&isolation_root)?;
    }
    let paths = isolated_ronin_paths(&isolation_root)?;
    let mut driver = SessionPaintDriver::new(paths, scenarios.clone(), Box::new(AlwaysOkSmoke));
    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: workspace,
        baselines_dir: baselines,
        reports_dir: reports,
        scenarios_dir: scenarios,
        budget: budget_rules(),
        require_release: false,
        is_release_build: true,
        require_drive_smoke: false,
        isolation_paths: None,
    });
    // Run paint only (via driver) then propose — not an official judgment accept.
    let timing = {
        use ronin_perf_harness::ChatPaintDriver;
        driver.run_chat_paint_path(ScenarioId(scenario))?
    };
    let path = harness.propose_baseline(ScenarioId(scenario), &timing)?;
    println!(
        "baseline proposal written to {} (explicit accept required to promote)",
        path.display()
    );
    Ok(true)
}

fn accept_baseline(scenario: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let (workspace, scenarios, baselines, reports) = default_dirs();
    let harness = PerfHarness::new(PerfHarnessConfig {
        workspace_root: workspace,
        baselines_dir: baselines,
        reports_dir: reports,
        scenarios_dir: scenarios,
        budget: budget_rules(),
        require_release: false,
        is_release_build: true,
        require_drive_smoke: false,
        isolation_paths: None,
    });
    let dest = harness.accept_baseline_proposal(ScenarioId(scenario))?;
    println!("accepted baseline proposal → {}", dest.display());
    Ok(true)
}

fn run_generated_sweep(n: usize) -> Result<bool, Box<dyn std::error::Error>> {
    let (workspace, scenarios, _baselines, reports) = default_dirs();
    let isolation_root = workspace.join("target/perf-harness/isolated-sweep");
    if isolation_root.exists() {
        std::fs::remove_dir_all(&isolation_root)?;
    }
    let paths = isolated_ronin_paths(&isolation_root)?;
    let messages = generate_scale_messages(n, true);
    let mut driver =
        SessionPaintDriver::new(paths, scenarios, Box::new(AlwaysOkSmoke)).with_messages(messages);
    use ronin_perf_harness::ChatPaintDriver;
    let timing = driver.run_chat_paint_path(ScenarioId("scale_sweep"))?;
    std::fs::create_dir_all(&reports)?;
    let out = reports.join("scale_sweep.timing.json");
    std::fs::write(&out, serde_json::to_vec_pretty(&timing)?)?;
    println!(
        "sweep n={n} parse={}ms render={}ms wall={}ms → {}",
        timing.parse.as_millis(),
        timing.render.as_millis(),
        timing.wall.as_millis(),
        out.display()
    );
    Ok(true)
}
