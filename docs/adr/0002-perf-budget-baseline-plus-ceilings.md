# Perf budgets are baseline regression plus disaster ceilings

Chat Paint Path Perf Judgments must be stable enough for an autonomous improvement loop, but still catch catastrophes. Primary fail is regression against a recorded baseline (percent/ms delta); absolute ceilings are a secondary tripwire only. Absolute-only budgets are too machine-noisy; baseline-only can miss multi-second disasters.
