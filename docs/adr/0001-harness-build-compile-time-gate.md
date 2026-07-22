# Compile-time gate the Perf Harness control plane

The Perf Harness needs a real-window control plane and rich Paint Timing export, but Ronin’s trust-before-agency stance forbids a shipping `ronin` that can be driven like a test double. We put that surface behind a compile-time Harness Build (feature/profile), not an always-on or runtime-secret unlock in user installs.
