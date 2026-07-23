# Agent playbook — Perf Harness

Machine-readable path for coding agents / `loop/` to improve Chat Paint Path performance.

Spec: [#87](https://github.com/roninchat/ronin/issues/87). Glossary: repo-root `CONTEXT.md`. ADRs: `docs/adr/0001`–`0005`.

## Contract

| Item | Value |
|------|--------|
| Binary | `ronin-perf-harness` (`cargo run -p ronin_perf_harness --release -- …`) |
| Official profile | **release only** (`require_release`) |
| Exit 0 | Perf Judgment passed |
| Exit 1 | Judgment failed (budgets) |
| Exit 2 | Harness error (smoke, profile, I/O, unknown scenario) |
| Report | `target/perf-harness/reports/<scenario>.judgment.json` |
| Scenarios | `plain_short`, `heavy_fences`, `long_history` |

### Improvement Signal schema (JSON)

- `kind`: `"improvement_signal"`
- `scenario`, `passed`, `failures[]`, `hotspots[]`, `timing` (`parse`/`render`/`wall` ms + `spans`)
- `build_profile`: `"release"` \| `"debug"`
- `isolation_paths`: optional `{ config_dir, data_dir }`

Agents treat a failing report as the red evidence; they **must not** auto-accept baselines.

## Commands

```bash
# Run one golden (reuse isolated DB; skip OS smoke in headless CI)
cargo run -p ronin_perf_harness --release -- run plain_short --skip-smoke

# Wipe isolated DB then run
cargo run -p ronin_perf_harness --release -- run heavy_fences --skip-smoke --fresh

# Scale sweep (generator; writes timing only)
cargo run -p ronin_perf_harness --release -- generate-sweep 80

# Baseline propose → human/agent explicit accept
cargo run -p ronin_perf_harness --release -- propose-baseline plain_short
cargo run -p ronin_perf_harness --release -- accept-baseline plain_short
```

Helper wrapper (same repo):

```bash
./crates/ronin_perf_harness/agent-run.sh plain_short
```

## Agent loop recipe

1. `agent-run.sh <scenario>` (or `cargo run … --release -- run … --skip-smoke`).
2. If exit ≠ 0, read `target/perf-harness/reports/<scenario>.judgment.json`.
3. Use `failures` + ranked `hotspots` to target Chat Paint Path code.
4. Re-run until pass. Propose baseline only after a real intentional win; **accept** only with explicit approval (ADR-0003).

## Non-goals for agents

- Do not enable `harness` feature in shipping installs.
- Do not touch `~/.config/ronin` / `~/.local/share/ronin`.
- Do not claim GPUI frame timing until window drive lands; current paint path measures parse + syntect render proxy after isolated session seed.
