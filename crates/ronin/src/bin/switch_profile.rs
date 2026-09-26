//! Times a heavy chat switch and names the slowest component.
//!
//! ```text
//! cargo run -p ronin --release --bin switch_profile -- --html report.html
//! ```

use std::process::ExitCode;
use std::time::Instant;

use ronin::switch_profile::{
    format_switch_profile, heavy_switch_bodies, profile_render_work, rank_phases,
    switch_profile_html, time_phase,
};
use ronin_core::{HttpOllamaProvider, MessageRole, OllamaProvider, RoninPaths, RoninSession};

fn main() -> ExitCode {
    let mut html_path = None;
    let mut seed_dir = None;
    let mut seed_messages = 400usize;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--html" => match args.next() {
                Some(path) => html_path = Some(path),
                None => {
                    eprintln!("switch_profile: --html needs a path");
                    return ExitCode::from(2);
                }
            },
            "--seed" => match args.next() {
                Some(path) => seed_dir = Some(std::path::PathBuf::from(path)),
                None => {
                    eprintln!("switch_profile: --seed needs a directory");
                    return ExitCode::from(2);
                }
            },
            "--messages" => match args.next().and_then(|n| n.parse().ok()) {
                Some(count) => seed_messages = count,
                None => {
                    eprintln!("switch_profile: --messages needs a number");
                    return ExitCode::from(2);
                }
            },
            "--help" | "-h" => {
                println!("usage: switch_profile [--html path] [--seed dir [--messages n]]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("switch_profile: unknown argument {other}");
                return ExitCode::from(2);
            }
        }
    }

    if let Some(dir) = seed_dir {
        return match seed(&dir, seed_messages) {
            Ok(()) => {
                println!(
                    "seeded {} with a {seed_messages}-message chat and a {}-message chat",
                    dir.display(),
                    seed_messages / 2
                );
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("switch_profile: {error}");
                ExitCode::FAILURE
            }
        };
    }

    let profile = match run() {
        Ok(profile) => profile,
        Err(error) => {
            eprintln!("switch_profile: {error}");
            return ExitCode::FAILURE;
        }
    };
    println!("{}", format_switch_profile(&profile));
    if let Some(path) = html_path {
        if let Err(error) = std::fs::write(&path, switch_profile_html(&profile)) {
            eprintln!("switch_profile: failed to write {path}: {error}");
            return ExitCode::FAILURE;
        }
        println!("wrote {path}");
    }
    ExitCode::SUCCESS
}

/// Writes `ronin/` config and data under `dir/config` and `dir/data`, the
/// layout `XDG_CONFIG_HOME=dir/config XDG_DATA_HOME=dir/data` expects.
fn seed(dir: &std::path::Path, messages: usize) -> Result<(), String> {
    let session = RoninSession::open(RoninPaths {
        config_dir: dir.join("config/ronin"),
        data_dir: dir.join("data/ronin"),
    })
    .map_err(|error| error.to_string())?;
    for (index, count) in [messages, messages / 2].into_iter().enumerate() {
        let thread = session.create_thread().map_err(|error| error.to_string())?;
        session
            .update_thread_title(&thread.id, &format!("Long chat {}", index + 1))
            .map_err(|error| error.to_string())?;
        for turn in 0..count {
            let (role, body) = if turn % 2 == 0 {
                (MessageRole::User, user_turn(turn))
            } else {
                (MessageRole::Assistant, assistant_turn(turn))
            };
            session
                .create_message(&thread.id, role, &body)
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn user_turn(turn: usize) -> String {
    format!(
        "Turn {turn}: can you tighten the retry loop and explain why the backoff resets after a success?"
    )
}

fn assistant_turn(turn: usize) -> String {
    let prose = "The loop keeps a **counter** of consecutive failures and doubles the wait each time, capped at `max_delay`. A success clears the counter, so the next failure starts from the base delay again instead of inheriting a long wait from an old outage.";
    let mut body = format!("## Pass {turn}\n\n{prose}\n\n{prose}\n\n- keep the cap\n- reset on success\n- log the attempt number\n");
    if turn % 4 == 1 {
        body.push_str("\n```rust\n");
        for line in 0..24 {
            body.push_str(&format!(
                "    let delay_{line} = base.saturating_mul(1 << attempt.min({line})).min(max_delay);\n"
            ));
        }
        body.push_str("```\n");
    }
    body
}

fn run() -> Result<ronin::switch_profile::SwitchProfile, String> {
    let root = std::env::temp_dir().join(format!("ronin-switch-profile-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let session = RoninSession::open(RoninPaths {
        config_dir: root.join("config"),
        data_dir: root.join("data"),
    })
    .map_err(|error| error.to_string())?;
    let thread = session.create_thread().map_err(|error| error.to_string())?;
    for (index, body) in heavy_switch_bodies().iter().enumerate() {
        let role = if index % 2 == 0 {
            MessageRole::User
        } else {
            MessageRole::Assistant
        };
        session
            .create_message(&thread.id, role, body)
            .map_err(|error| error.to_string())?;
    }

    let load_started = Instant::now();
    let loaded = session
        .list_messages(&thread.id)
        .map_err(|error| error.to_string())?;
    let load_elapsed = load_started.elapsed();

    let attach_started = Instant::now();
    for message in &loaded {
        let _ = session.list_attachments(&message.id);
    }
    let attach_elapsed = attach_started.elapsed();

    let threads_started = Instant::now();
    session.list_threads().map_err(|error| error.to_string())?;
    let threads_elapsed = threads_started.elapsed();

    eprintln!("timing provider health check (this is the old per-switch call)");
    let probe_started = Instant::now();
    let _health = HttpOllamaProvider::new("http://127.0.0.1:11434").check_health();
    let probe_elapsed = probe_started.elapsed();

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
    phases.push(time_phase(
        "refresh_provider_status",
        "check_health",
        probe_elapsed,
    ));
    let profile = rank_phases(
        "synthetic heavy thread",
        loaded.len(),
        code_bytes,
        phases,
        threads_elapsed,
    );
    drop(session);
    let _ = std::fs::remove_dir_all(&root);
    Ok(profile)
}
