//! Paint Timing types.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// One named span from the Chat Paint Path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanTiming {
    /// Span name (e.g. `markdown.parse`, `chat.render`).
    pub name: String,
    /// Duration of the span.
    #[serde(with = "duration_ms")]
    pub duration: Duration,
}

/// Aggregated Paint Timing for a Perf Scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaintTiming {
    /// Time spent in markdown parse.
    #[serde(with = "duration_ms")]
    pub parse: Duration,
    /// Time spent in render/highlight work.
    #[serde(with = "duration_ms")]
    pub render: Duration,
    /// End-to-end wall-clock for the scenario.
    #[serde(with = "duration_ms")]
    pub wall: Duration,
    /// Named spans (primary attribution signal).
    pub spans: Vec<SpanTiming>,
}

mod duration_ms {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(d: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(d.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(ms))
    }
}
