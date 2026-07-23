//! Compile-time gated Perf Harness control plane (ADR-0001).
//!
//! Enabled only with `--features harness`. Shipping installs must omit this feature.

/// Commands the Perf Harness may send to a Harness Build window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessControlCommand {
    /// Select/open a fixture thread by id.
    OpenThread {
        /// Thread id in the isolated DB.
        thread_id: String,
    },
    /// Scroll the message list by a delta (positive = down).
    ScrollMessages {
        /// Scroll delta in logical pixels.
        delta: i32,
    },
    /// No-op ping used by Drive Smoke / liveness.
    Ping,
}

/// Parses a single-line control-plane command (`open <id>`, `scroll <n>`, `ping`).
pub fn parse_harness_command(line: &str) -> Option<HarnessControlCommand> {
    let line = line.trim();
    if line == "ping" {
        return Some(HarnessControlCommand::Ping);
    }
    if let Some(rest) = line.strip_prefix("open ") {
        let thread_id = rest.trim();
        if !thread_id.is_empty() {
            return Some(HarnessControlCommand::OpenThread {
                thread_id: thread_id.to_string(),
            });
        }
    }
    if let Some(rest) = line.strip_prefix("scroll ") {
        if let Ok(delta) = rest.trim().parse::<i32>() {
            return Some(HarnessControlCommand::ScrollMessages { delta });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_control_plane_commands() {
        assert_eq!(
            parse_harness_command("ping"),
            Some(HarnessControlCommand::Ping)
        );
        assert_eq!(
            parse_harness_command("open thr_1"),
            Some(HarnessControlCommand::OpenThread {
                thread_id: "thr_1".into()
            })
        );
        assert_eq!(
            parse_harness_command("scroll -40"),
            Some(HarnessControlCommand::ScrollMessages { delta: -40 })
        );
        assert_eq!(parse_harness_command("nope"), None);
    }

    #[test]
    fn parse_command_matrix_dense() {
        for id in 0..80 {
            let line = format!("open thread_{id}");
            let cmd = parse_harness_command(&line).expect("open");
            match cmd {
                HarnessControlCommand::OpenThread { thread_id } => {
                    assert_eq!(thread_id, format!("thread_{id}"));
                }
                other => panic!("{other:?}"),
            }
        }
        for delta in -40..40 {
            let line = format!("scroll {delta}");
            assert_eq!(
                parse_harness_command(&line),
                Some(HarnessControlCommand::ScrollMessages { delta })
            );
        }
        for junk in [
            "", " ", "open", "open ", "scroll", "scroll x", "pong", "OPEN x",
        ] {
            assert_eq!(parse_harness_command(junk), None, "{junk:?}");
        }
    }
}
