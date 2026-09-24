//! Shell frame timings.
//!
//! `RONIN_FPS=1` paints fps, the last frame, and the slowest phase.
//! Frames slower than 12ms are traced at most once a second.

use std::time::{Duration, Instant};

/// One finished frame, for the optional on-screen readout.
pub(crate) struct FrameSample {
    pub fps: f32,
    pub last_ms: f32,
    pub slowest: &'static str,
    pub slowest_ms: f32,
    pub overlay: bool,
}

/// Rolling timings for a single window's render path.
pub(crate) struct FrameProfiler {
    overlay: bool,
    origin: Instant,
    phase_origin: Instant,
    phases: Vec<(&'static str, Duration)>,
    window_origin: Instant,
    frames: u32,
    fps: f32,
    last_ms: f32,
    slowest: &'static str,
    slowest_ms: f32,
    last_slow_log: Instant,
}

impl FrameProfiler {
    pub(crate) fn new() -> Self {
        let overlay = std::env::var("RONIN_FPS").is_ok_and(|value| value != "0");
        let now = Instant::now();
        Self {
            overlay,
            origin: now,
            phase_origin: now,
            phases: Vec::new(),
            window_origin: now,
            frames: 0,
            fps: 0.0,
            last_ms: 0.0,
            slowest: "idle",
            slowest_ms: 0.0,
            last_slow_log: now,
        }
    }

    pub(crate) fn begin_frame(&mut self) {
        let now = Instant::now();
        self.origin = now;
        self.phase_origin = now;
        self.phases.clear();
    }

    /// Closes the previous phase and starts `name`.
    pub(crate) fn phase(&mut self, name: &'static str) {
        let now = Instant::now();
        self.phases
            .push((name, now.saturating_duration_since(self.phase_origin)));
        self.phase_origin = now;
    }

    pub(crate) fn end_frame(&mut self) -> FrameSample {
        let now = Instant::now();
        self.phases
            .push(("tail", now.saturating_duration_since(self.phase_origin)));
        let total = now.saturating_duration_since(self.origin);
        self.last_ms = total.as_secs_f32() * 1000.0;
        let (slowest, slowest_dur) = self
            .phases
            .iter()
            .max_by_key(|(_, duration)| *duration)
            .copied()
            .unwrap_or(("none", Duration::ZERO));
        self.slowest = slowest;
        self.slowest_ms = slowest_dur.as_secs_f32() * 1000.0;
        self.frames += 1;
        let window = now.saturating_duration_since(self.window_origin);
        if window >= Duration::from_millis(500) {
            self.fps = self.frames as f32 / window.as_secs_f32();
            self.frames = 0;
            self.window_origin = now;
        }
        if self.last_ms >= 12.0
            && now.saturating_duration_since(self.last_slow_log) >= Duration::from_secs(1)
        {
            self.last_slow_log = now;
            tracing::info!(
                frame_ms = self.last_ms,
                fps = self.fps,
                slowest = self.slowest,
                slowest_ms = self.slowest_ms,
                "slow frame"
            );
        }
        FrameSample {
            fps: self.fps,
            last_ms: self.last_ms,
            slowest: self.slowest,
            slowest_ms: self.slowest_ms,
            overlay: self.overlay,
        }
    }
}

/// Milliseconds until the caret visibility flips, given time since blink start.
pub(crate) fn caret_edge_delay_ms(elapsed_ms: u64, cycle_ms: u64, visible_ms: u64) -> u64 {
    let cycle = cycle_ms.max(1);
    let visible = visible_ms.min(cycle);
    let phase = elapsed_ms % cycle;
    if phase < visible {
        (visible - phase).max(1)
    } else {
        (cycle - phase).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::caret_edge_delay_ms;

    #[test]
    fn caret_edge_lands_on_the_next_visibility_change() {
        assert_eq!(caret_edge_delay_ms(0, 800, 480), 480);
        assert_eq!(caret_edge_delay_ms(100, 800, 480), 380);
        assert_eq!(caret_edge_delay_ms(480, 800, 480), 320);
        assert_eq!(caret_edge_delay_ms(799, 800, 480), 1);
    }
}
