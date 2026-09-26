//! Times the work a chat switch does before the next frame.
//!
//! `render_messages` used to pay all of this on the UI thread at once.
//! The ranked phase names the component that owns the slowest slice.

use std::time::{Duration, Instant};

use ronin_core::ColorScheme;

use crate::markdown::{parse_markdown, MarkdownBlock};
use crate::syntax_highlight::highlight_code;

/// Frames slower than this are the same cutoff the on-screen frame profiler uses.
pub const SWITCH_FRAME_BUDGET_MS: f64 = 12.0;

/// One timed slice of a chat switch.
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchPhase {
    /// UI or shell function that owns this work.
    pub component: &'static str,
    /// Leaf function that did the work.
    pub function: &'static str,
    /// Elapsed milliseconds.
    pub millis: f64,
}

/// Ranked timings for one switch.
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchProfile {
    /// What was measured, for the report heading.
    pub label: String,
    /// Messages included in the render timings.
    pub message_count: usize,
    /// Bytes inside fenced code blocks.
    pub code_bytes: usize,
    /// Ranked and unranked phases. The culprit is the slowest entry.
    pub phases: Vec<SwitchPhase>,
    /// Component that owns the slowest phase.
    pub culprit_component: &'static str,
    /// Leaf function of the slowest phase.
    pub culprit_function: &'static str,
    /// Milliseconds of the slowest phase.
    pub culprit_ms: f64,
    /// Frame budget the shell treats as a hitch.
    pub frame_budget_ms: f64,
    /// `culprit_ms` is at least the frame budget.
    pub over_budget: bool,
    /// Time to load every thread row. No longer part of `list_messages`.
    pub avoided_full_thread_list_ms: f64,
}

/// Builds a phase from a finished duration.
pub fn time_phase(
    component: &'static str,
    function: &'static str,
    duration: Duration,
) -> SwitchPhase {
    SwitchPhase {
        component,
        function,
        millis: duration_ms(duration),
    }
}

/// Times markdown parse, syntax highlight, cache clone, and code-line text copies.
///
/// This is the CPU work `render_messages` does through `message_view` and
/// `build_message_view` for completed messages. It does not build GPUI elements.
pub fn profile_render_work(contents: &[&str]) -> (Vec<SwitchPhase>, usize) {
    let scheme = ColorScheme::Dark;
    let parse_started = Instant::now();
    let parsed: Vec<Vec<MarkdownBlock>> =
        contents.iter().map(|text| parse_markdown(text)).collect();
    let parse_elapsed = parse_started.elapsed();

    let mut code_bytes = 0usize;
    let highlight_started = Instant::now();
    let mut highlighted = Vec::new();
    for blocks in &parsed {
        for block in blocks {
            if let MarkdownBlock::CodeBlock { language, content } = block {
                code_bytes += content.len();
                highlighted.push(highlight_code(language.as_deref(), content, scheme));
            }
        }
    }
    let highlight_elapsed = highlight_started.elapsed();

    let clone_started = Instant::now();
    let cloned = std::hint::black_box(highlighted.clone());
    let clone_elapsed = clone_started.elapsed();

    let lines_started = Instant::now();
    for lines in &cloned {
        for line in lines {
            for span in &line.spans {
                std::hint::black_box(span.text.clone());
            }
        }
    }
    let lines_elapsed = lines_started.elapsed();

    let phases = vec![
        time_phase("render_messages", "parse_markdown", parse_elapsed),
        time_phase("render_messages", "highlight_code", highlight_elapsed),
        time_phase("render_messages", "clone_cached_view", clone_elapsed),
        time_phase("render_messages", "render_code_lines", lines_elapsed),
    ];
    (phases, code_bytes)
}

/// Picks the slowest phase and fills the report fields.
pub fn rank_phases(
    label: impl Into<String>,
    message_count: usize,
    code_bytes: usize,
    phases: Vec<SwitchPhase>,
    avoided_full_thread_list: Duration,
) -> SwitchProfile {
    let culprit = phases.iter().max_by(|left, right| {
        left.millis
            .total_cmp(&right.millis)
            .then_with(|| right.function.cmp(left.function))
    });
    let (culprit_component, culprit_function, culprit_ms) = culprit
        .map(|phase| (phase.component, phase.function, phase.millis))
        .unwrap_or(("none", "none", 0.0));
    SwitchProfile {
        label: label.into(),
        message_count,
        code_bytes,
        phases,
        culprit_component,
        culprit_function,
        culprit_ms,
        frame_budget_ms: SWITCH_FRAME_BUDGET_MS,
        over_budget: culprit_ms >= SWITCH_FRAME_BUDGET_MS,
        avoided_full_thread_list_ms: duration_ms(avoided_full_thread_list),
    }
}

/// Assistant bodies with fenced Python, repeated so highlight dominates a frame.
pub fn heavy_switch_bodies() -> Vec<String> {
    let fence = format!("```python\n{}\n```\n", python_sample());
    let user = "Walk through the classifier and point out the expensive branch.";
    let mut bodies = Vec::with_capacity(16);
    for index in 0..8 {
        bodies.push(format!("{user} ({index})"));
        bodies.push(format!("Here is pass {index}.\n\n{fence}"));
    }
    bodies
}

fn python_sample() -> String {
    let body = r#"def classify(rows):
    total = 0
    for row in rows:
        if row.get("ok"):
            total += int(row["n"])
        else:
            total -= 1
    return {"total": total, "empty": len(rows) == 0}
"#;
    body.repeat(30)
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

/// Standalone HTML report for one profile.
pub fn switch_profile_html(profile: &SwitchProfile) -> String {
    let max_ms = profile
        .phases
        .iter()
        .map(|phase| phase.millis)
        .fold(profile.culprit_ms, f64::max)
        .max(1.0);
    let mut rows = String::new();
    let mut ranked = profile.phases.clone();
    ranked.sort_by(|left, right| right.millis.total_cmp(&left.millis));
    for phase in &ranked {
        let width = (phase.millis / max_ms * 100.0).clamp(0.0, 100.0);
        let mark = if phase.function == profile.culprit_function
            && phase.component == profile.culprit_component
        {
            " culprit"
        } else {
            ""
        };
        rows.push_str(&format!(
            r#"<tr class="{mark}">
<td>{component}</td>
<td><code>{function}</code></td>
<td class="num">{millis:.2} ms</td>
<td><div class="bar" style="width:{width:.1}%"></div></td>
</tr>"#,
            mark = mark.trim(),
            component = phase.component,
            function = phase.function,
            millis = phase.millis,
            width = width,
        ));
    }
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Chat switch profile</title>
<style>
body {{ font: 16px/1.45 "Iowan Old Style", Palatino, Georgia, serif; margin: 2rem auto; max-width: 46rem; color: #1c1917; background: #faf7f2; }}
h1 {{ font-size: 1.6rem; font-weight: 600; }}
table {{ width: 100%; border-collapse: collapse; }}
td {{ padding: 0.35rem 0.4rem; vertical-align: middle; border-top: 1px solid #e7e0d6; }}
.num {{ text-align: right; font-variant-numeric: tabular-nums; white-space: nowrap; }}
.bar {{ height: 0.7rem; background: #57534e; border-radius: 2px; }}
tr.culprit .bar {{ background: #9a3412; }}
code {{ font-family: ui-monospace, monospace; font-size: 0.9em; }}
</style>
</head>
<body>
<h1>{label}</h1>
<p>{count} messages, {code} bytes of fenced code. Frame budget is {budget:.0} ms. Slowest phase is <code>{component}</code> / <code>{function}</code> at {ms:.2} ms.</p>
<table>
<tr><th>Component</th><th>Function</th><th>Time</th><th></th></tr>
{rows}
</table>
<p>A full thread-list scan, which <code>list_messages</code> no longer does on this path, measured {avoided:.2} ms.</p>
</body>
</html>"#,
        label = profile.label,
        count = profile.message_count,
        code = profile.code_bytes,
        budget = profile.frame_budget_ms,
        component = profile.culprit_component,
        function = profile.culprit_function,
        ms = profile.culprit_ms,
        rows = rows,
        avoided = profile.avoided_full_thread_list_ms,
    )
}

/// Plain-text report for the `switch_profile` tool.
pub fn format_switch_profile(profile: &SwitchProfile) -> String {
    let mut lines = Vec::new();
    lines.push(format!("chat switch profile: {}", profile.label));
    lines.push(format!(
        "messages {}  code bytes {}  frame budget {:.0} ms",
        profile.message_count, profile.code_bytes, profile.frame_budget_ms
    ));
    lines.push(format!(
        "culprit {} :: {}  {:.2} ms  over budget {}",
        profile.culprit_component,
        profile.culprit_function,
        profile.culprit_ms,
        profile.over_budget
    ));
    let mut ranked = profile.phases.clone();
    ranked.sort_by(|left, right| right.millis.total_cmp(&left.millis));
    for phase in ranked {
        lines.push(format!(
            "  {:>8.2} ms  {:<28} {}",
            phase.millis, phase.component, phase.function
        ));
    }
    lines.push(format!(
        "avoided list_threads scan {:.2} ms (no longer inside list_messages)",
        profile.avoided_full_thread_list_ms
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use ronin_core::{MessageRole, RoninPaths, RoninSession};
    use tempfile::TempDir;

    use super::{
        format_switch_profile, heavy_switch_bodies, profile_render_work, rank_phases, time_phase,
    };

    #[test]
    fn profile_should_attribute_heavy_code_thread_to_render_messages() {
        let temp = TempDir::new().expect("temp dir");
        let session = RoninSession::open(RoninPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        })
        .expect("session");
        let thread = session.create_thread().expect("thread");
        for (index, body) in heavy_switch_bodies().iter().enumerate() {
            let role = if index % 2 == 0 {
                MessageRole::User
            } else {
                MessageRole::Assistant
            };
            session
                .create_message(&thread.id, role, body)
                .expect("message");
        }

        let load_started = Instant::now();
        let loaded = session.list_messages(&thread.id).expect("messages");
        let load_elapsed = load_started.elapsed();
        assert_eq!(loaded.len(), heavy_switch_bodies().len());

        let attach_started = Instant::now();
        for message in &loaded {
            let _ = session.list_attachments(&message.id);
        }
        let attach_elapsed = attach_started.elapsed();

        let threads_started = Instant::now();
        let _ = session.list_threads().expect("threads");
        let threads_elapsed = threads_started.elapsed();

        let contents: Vec<&str> = loaded
            .iter()
            .map(|message| message.content.as_str())
            .collect();
        let (mut phases, code_bytes) = profile_render_work(&contents);
        phases.push(time_phase(
            "RoninSession::list_messages",
            "list_messages",
            load_elapsed,
        ));
        phases.push(time_phase(
            "render_message_attachments",
            "list_attachments",
            attach_elapsed,
        ));
        let profile = rank_phases(
            "synthetic heavy thread",
            loaded.len(),
            code_bytes,
            phases,
            threads_elapsed,
        );
        eprintln!("{}", format_switch_profile(&profile));
        assert_eq!(profile.culprit_component, "render_messages");
        assert!(profile.over_budget);
        assert!(profile.code_bytes > 0);
        let highlight = profile
            .phases
            .iter()
            .find(|phase| phase.function == "highlight_code")
            .expect("highlight phase");
        let parse = profile
            .phases
            .iter()
            .find(|phase| phase.function == "parse_markdown")
            .expect("parse phase");
        assert!(highlight.millis > parse.millis);
    }
}
