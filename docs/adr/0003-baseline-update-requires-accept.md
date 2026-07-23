# Baseline bumps need an explicit accept step

Perf Budget baselines are the oracle for autonomous improvement. Agents may propose a new baseline after a genuine win, but must not auto-promote or rewrite baselines on every green run — that ratchets away regressions. An explicit accept step (human or equivalent approval) lands the bump.
