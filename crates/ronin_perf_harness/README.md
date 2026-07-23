# Perf Harness (tooling track)

Sensor+judge for **Chat Paint Path** budgets. Not a user-facing Ronin product surface.

See spec [#87](https://github.com/roninchat/ronin/issues/87), `CONTEXT.md`, ADRs 0001–0005.

## Status (v1 foundation)

**Landed:** `PerfHarness` runner seam, isolation, goldens + generator, Paint Timing
(parse + syntect render proxy), Perf Budgets (baseline + ceilings), Improvement Signal,
propose/accept baseline, `harness` feature-gated control-plane command parser, Display Drive Smoke
(env check).

**Not yet (next slices):** live GPUI window launch + control-plane open/scroll drive,
Paint Timing export from real `markdown_view` / list render, focus+key/click Drive Smoke,
`loop/` consumer of the Improvement Signal.

## Agent / loop usage

See **[AGENT.md](./AGENT.md)** for the machine-readable Improvement Signal contract.
Wrappers: `./agent-run.sh <scenario>` or repo-root `./scripts/perf-harness-agent.sh <scenario>`.
Humans can use the same `cargo` commands below.

```bash
# Exploratory (debug); official judgments need --release
cargo run -p ronin_perf_harness -- run plain_short --skip-smoke

cargo run -p ronin_perf_harness --release -- run plain_short --skip-smoke
cargo run -p ronin_perf_harness --release -- run heavy_fences --skip-smoke
cargo run -p ronin_perf_harness --release -- run long_history --skip-smoke

# Propose then explicitly accept a baseline bump (ADR-0003)
cargo run -p ronin_perf_harness --release -- propose-baseline plain_short
cargo run -p ronin_perf_harness --release -- accept-baseline plain_short

# Scale sweep via generator
cargo run -p ronin_perf_harness --release -- generate-sweep 80
```

Omit `--skip-smoke` when a display is available (Drive Smoke checks `WAYLAND_DISPLAY` / `DISPLAY`).

Improvement Signal reports land under `target/perf-harness/reports/`.

## Harness Build gate

Rich control-plane hooks in the `ronin` binary are behind the `harness` cargo feature (ADR-0001). Shipping installs must not enable it.
