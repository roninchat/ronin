//! Seeds isolated RoninSession with scenario messages and measures Chat Paint Path.

use ronin_core::{MessageRole, RoninPaths, RoninSession};

use crate::error::HarnessError;
use crate::paint::{at_least_one_ms, measure_chat_paint, ChatPaintDriver, DriveSmoke};
use crate::scenario::{load_scenario_messages, ScenarioId};
use crate::timing::PaintTiming;

/// Control-plane style driver: load goldens into isolated session, measure paint path.
pub struct SessionPaintDriver {
    paths: RoninPaths,
    scenarios_dir: std::path::PathBuf,
    smoke: Box<dyn DriveSmoke + Send>,
    messages_override: Option<Vec<String>>,
}

impl SessionPaintDriver {
    /// Creates a driver bound to isolated paths and scenario directory.
    pub fn new(
        paths: RoninPaths,
        scenarios_dir: std::path::PathBuf,
        smoke: Box<dyn DriveSmoke + Send>,
    ) -> Self {
        Self {
            paths,
            scenarios_dir,
            smoke,
            messages_override: None,
        }
    }

    /// Uses generator/override messages instead of a golden file.
    pub fn with_messages(mut self, messages: Vec<String>) -> Self {
        self.messages_override = Some(messages);
        self
    }

    fn seed_and_collect(&self, scenario: ScenarioId<'_>) -> Result<Vec<String>, HarnessError> {
        let messages = if let Some(ref m) = self.messages_override {
            m.clone()
        } else {
            load_scenario_messages(&self.scenarios_dir, scenario)?
        };

        let session = RoninSession::open(self.paths.clone())
            .map_err(|e| HarnessError::Session(e.to_string()))?;
        let thread = session
            .create_thread()
            .map_err(|e| HarnessError::Session(e.to_string()))?;
        for (i, body) in messages.iter().enumerate() {
            let role = if i % 2 == 0 {
                MessageRole::User
            } else {
                MessageRole::Assistant
            };
            session
                .create_message(&thread.id, role, body)
                .map_err(|e| HarnessError::Session(e.to_string()))?;
        }
        Ok(messages)
    }
}

impl ChatPaintDriver for SessionPaintDriver {
    fn run_chat_paint_path(
        &mut self,
        scenario: ScenarioId<'_>,
    ) -> Result<PaintTiming, HarnessError> {
        let messages = self.seed_and_collect(scenario)?;
        let mut timing = measure_chat_paint(&messages);
        timing.parse = at_least_one_ms(timing.parse);
        timing.render = at_least_one_ms(timing.render);
        timing.wall = at_least_one_ms(timing.wall);
        for span in &mut timing.spans {
            span.duration = at_least_one_ms(span.duration);
        }
        Ok(timing)
    }
}

impl DriveSmoke for SessionPaintDriver {
    fn run_drive_smoke(&mut self) -> Result<(), HarnessError> {
        self.smoke.run_drive_smoke()
    }
}
