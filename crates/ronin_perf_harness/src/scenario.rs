//! Scenario ids and golden content loading.

use std::path::Path;

use crate::error::HarnessError;

/// Identifies a Perf Scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioId<'a>(pub &'a str);

/// Kind of Chat Paint Golden / generated content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioKind {
    /// Short plain markdown thread.
    PlainShort,
    /// Heavy fences + syntax highlight.
    HeavyFences,
    /// Long history open/scroll corpus.
    LongHistory,
}

impl ScenarioKind {
    /// Parses a scenario id string.
    pub fn parse(id: &str) -> Result<Self, HarnessError> {
        match id {
            "plain_short" => Ok(Self::PlainShort),
            "heavy_fences" => Ok(Self::HeavyFences),
            "long_history" => Ok(Self::LongHistory),
            other => Err(HarnessError::Scenario(format!("unknown scenario: {other}"))),
        }
    }

    /// Stable id string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlainShort => "plain_short",
            Self::HeavyFences => "heavy_fences",
            Self::LongHistory => "long_history",
        }
    }
}

/// Loads message bodies for a golden scenario from `scenarios/goldens/{id}.json`.
pub fn load_scenario_messages(
    scenarios_dir: &Path,
    scenario: ScenarioId<'_>,
) -> Result<Vec<String>, HarnessError> {
    let path = scenarios_dir
        .join("goldens")
        .join(format!("{}.json", scenario.0));
    let bytes = std::fs::read(&path)
        .map_err(|e| HarnessError::Scenario(format!("missing golden {}: {e}", path.display())))?;
    let messages: Vec<String> = serde_json::from_slice(&bytes)?;
    if messages.is_empty() {
        return Err(HarnessError::Scenario(format!(
            "golden {} has no messages",
            scenario.0
        )));
    }
    Ok(messages)
}

/// Builds synthetic scale-sweep messages (generator; not checked into git as huge corpora).
pub fn generate_scale_messages(message_count: usize, with_fences: bool) -> Vec<String> {
    (0..message_count)
        .map(|i| {
            if with_fences && i % 3 == 0 {
                format!(
                    "### Message {i}\n\n```rust\nfn item_{i}() {{\n    let x = {i};\n    println!(\"{{x}}\");\n}}\n```\n"
                )
            } else {
                format!("Message {i}: plain paragraph about Chat Paint Path timing.\n")
            }
        })
        .collect()
}
