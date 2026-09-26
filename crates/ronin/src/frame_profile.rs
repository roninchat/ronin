//! Shell frame timings.
//!
//! `RONIN_FPS=1` paints fps, the last frame, and the slowest phase.
//! Frames slower than 12ms are traced at most once a second.
//!
//! `render` only builds the element tree. GPUI lays it out and paints it
//! afterwards, so [`FrameProfiler::probe`] adds a zero-size element at the end
//! of the tree that stamps when prepaint and paint reach it. The next frame
//! folds those stamps into the previous frame's layout and paint times.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{canvas, div, prelude::*, px};

/// One finished frame, for the optional on-screen readout.
pub(crate) struct FrameSample {
    pub fps: f32,
    pub last_ms: f32,
    pub slowest: &'static str,
    pub slowest_ms: f32,
    pub overlay: bool,
}

/// Render, layout, and paint time of one whole frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FrameCost {
    pub render_ms: f32,
    pub layout_ms: f32,
    pub paint_ms: f32,
    /// Message rows built during the frame.
    pub rows: u32,
}

impl FrameCost {
    pub(crate) fn total_ms(&self) -> f32 {
        self.render_ms + self.layout_ms + self.paint_ms
    }
}

#[derive(Default)]
struct FrameMarks {
    render_done: Cell<Option<Instant>>,
    layout_done: Cell<Option<Instant>>,
    paint_done: Cell<Option<Instant>>,
    rows: Cell<u32>,
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
    marks: Rc<FrameMarks>,
    pending_render_ms: Option<f32>,
    label: Option<&'static str>,
    pending_label: Option<&'static str>,
    recorded: Vec<(&'static str, FrameCost)>,
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
            marks: Rc::new(FrameMarks::default()),
            pending_render_ms: None,
            label: None,
            pending_label: None,
            recorded: Vec::new(),
        }
    }

    /// Frames rendered from now on are recorded under `label`. `None` stops recording.
    pub(crate) fn set_label(&mut self, label: Option<&'static str>) {
        self.label = label;
    }

    /// Every labeled frame finished so far, oldest first.
    pub(crate) fn recorded(&self) -> &[(&'static str, FrameCost)] {
        &self.recorded
    }

    pub(crate) fn begin_frame(&mut self) {
        self.fold_previous_frame();
        let now = Instant::now();
        self.origin = now;
        self.phase_origin = now;
        self.phases.clear();
    }

    fn fold_previous_frame(&mut self) {
        let Some(render_ms) = self.pending_render_ms.take() else {
            return;
        };
        let label = self.pending_label.take();
        let render_done = self.marks.render_done.take();
        let layout_done = self.marks.layout_done.take();
        let paint_done = self.marks.paint_done.take();
        let rows = self.marks.rows.replace(0);
        let (Some(render_done), Some(layout_done), Some(paint_done)) =
            (render_done, layout_done, paint_done)
        else {
            return;
        };
        let cost = FrameCost {
            render_ms,
            layout_ms: ms(layout_done.saturating_duration_since(render_done)),
            paint_ms: ms(paint_done.saturating_duration_since(layout_done)),
            rows,
        };
        if cost.total_ms() >= 12.0
            && Instant::now().saturating_duration_since(self.last_slow_log)
                >= Duration::from_secs(1)
        {
            self.last_slow_log = Instant::now();
            tracing::info!(
                render_ms = cost.render_ms,
                layout_ms = cost.layout_ms,
                paint_ms = cost.paint_ms,
                slowest = self.slowest,
                "slow frame"
            );
        }
        if let Some(label) = label {
            self.recorded.push((label, cost));
        }
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
        self.last_ms = ms(total);
        let (slowest, slowest_dur) = self
            .phases
            .iter()
            .max_by_key(|(_, duration)| *duration)
            .copied()
            .unwrap_or(("none", Duration::ZERO));
        self.slowest = slowest;
        self.slowest_ms = ms(slowest_dur);
        self.frames += 1;
        let window = now.saturating_duration_since(self.window_origin);
        if window >= Duration::from_millis(500) {
            self.fps = self.frames as f32 / window.as_secs_f32();
            self.frames = 0;
            self.window_origin = now;
        }
        self.pending_render_ms = Some(self.last_ms);
        self.pending_label = self.label;
        self.marks.render_done.set(Some(now));
        FrameSample {
            fps: self.fps,
            last_ms: self.last_ms,
            slowest: self.slowest,
            slowest_ms: self.slowest_ms,
            overlay: self.overlay,
        }
    }

    /// Called once per message row built, so summaries show how much of the thread each frame touched.
    pub(crate) fn count_row(&self) {
        self.marks.rows.set(self.marks.rows.get() + 1);
    }

    /// Zero-size element that must be the last child of the window root.
    pub(crate) fn probe(&self) -> impl IntoElement {
        let layout_marks = self.marks.clone();
        let paint_marks = self.marks.clone();
        div().absolute().size(px(0.)).child(
            canvas(
                move |_, _, _| layout_marks.layout_done.set(Some(Instant::now())),
                move |_, _, _, _| paint_marks.paint_done.set(Some(Instant::now())),
            )
            .size(px(0.)),
        )
    }
}

fn ms(duration: Duration) -> f32 {
    duration.as_secs_f32() * 1000.0
}

/// Summary of labeled frames: count, p50/p95/max total, mean render/layout/paint.
pub(crate) fn summarize_frames(recorded: &[(&'static str, FrameCost)]) -> String {
    let mut labels: Vec<&'static str> = Vec::new();
    for (label, _) in recorded {
        if !labels.contains(label) {
            labels.push(label);
        }
    }
    let mut out = String::new();
    for label in labels {
        let costs: Vec<FrameCost> = recorded
            .iter()
            .filter(|(l, _)| *l == label)
            .map(|(_, c)| *c)
            .collect();
        let mut totals: Vec<f32> = costs.iter().map(FrameCost::total_ms).collect();
        totals.sort_by(f32::total_cmp);
        let pick = |q: f32| totals[((totals.len() - 1) as f32 * q).round() as usize];
        let n = costs.len() as f32;
        let mean = |f: fn(&FrameCost) -> f32| costs.iter().map(f).sum::<f32>() / n;
        let over = totals.iter().filter(|t| **t > 16.7).count();
        out.push_str(&format!(
            "{label}: frames {} p50 {:.1} ms p95 {:.1} ms max {:.1} ms over16.7 {} | mean render {:.1} layout {:.1} paint {:.1} rows {:.1}\n",
            costs.len(),
            pick(0.5),
            pick(0.95),
            totals[totals.len() - 1],
            over,
            mean(|c| c.render_ms),
            mean(|c| c.layout_ms),
            mean(|c| c.paint_ms),
            mean(|c| c.rows as f32),
        ));
    }
    out
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
    use super::{caret_edge_delay_ms, summarize_frames, FrameCost};

    #[test]
    fn caret_edge_lands_on_the_next_visibility_change() {
        assert_eq!(caret_edge_delay_ms(0, 800, 480), 480);
        assert_eq!(caret_edge_delay_ms(100, 800, 480), 380);
        assert_eq!(caret_edge_delay_ms(480, 800, 480), 320);
        assert_eq!(caret_edge_delay_ms(799, 800, 480), 1);
    }

    #[test]
    fn summarize_frames_should_group_by_label() {
        let cost = |ms| FrameCost {
            render_ms: ms,
            layout_ms: ms,
            paint_ms: 0.0,
            rows: 0,
        };
        let text = summarize_frames(&[("a", cost(1.0)), ("b", cost(10.0)), ("a", cost(3.0))]);
        assert!(text.starts_with("a: frames 2"));
        assert!(text.contains("b: frames 1"));
        assert!(text.contains("over16.7 1"));
    }
}
