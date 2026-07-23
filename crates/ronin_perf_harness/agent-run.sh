#!/usr/bin/env bash
# Thin AI/`loop` entrypoint for Perf Harness Improvement Signals.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCENARIO="${1:-plain_short}"
EXTRA=("${@:2}")
cd "$ROOT"
cargo run -p ronin_perf_harness --release --quiet -- run "$SCENARIO" --skip-smoke "${EXTRA[@]}"
REPORT="$ROOT/target/perf-harness/reports/${SCENARIO}.judgment.json"
echo "IMPROVEMENT_SIGNAL=$REPORT"
if [[ -f "$REPORT" ]]; then
  cat "$REPORT"
fi
