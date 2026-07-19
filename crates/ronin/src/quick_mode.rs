//! Quick mode overlay: compact one-shot question/answer surface.
//!
//! Public seams — testable without GPUI pixels. The native window wires these
//! into a centered overlay; IPC routes `--quick` to the same state machine.

use ronin_core::{ChatMessage, ChatRequest};

/// Compact overlay width in logical pixels.
pub const QUICK_WINDOW_WIDTH: f32 = 560.0;
/// Compact overlay height in logical pixels.
pub const QUICK_WINDOW_HEIGHT: f32 = 420.0;

/// Lifecycle phase of a quick-mode session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuickPhase {
    /// User is editing the question (or empty).
    Composing,
    /// Provider stream is in flight.
    Streaming,
    /// Stream finished successfully.
    Complete,
    /// Stream failed.
    Failed {
        /// User-visible failure reason.
        message: String,
    },
}

/// Events from an ephemeral (non-persisted) quick-mode provider stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuickStreamEvent {
    /// Partial answer text.
    Chunk(String),
    /// Stream finished successfully.
    Done,
    /// Stream failed with a user-visible reason.
    Error(String),
}

/// Presentation/controller state for the quick-mode overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickModeState {
    question: String,
    answer: String,
    phase: QuickPhase,
    saved_thread_id: Option<String>,
    dismissed: bool,
}

impl Default for QuickModeState {
    fn default() -> Self {
        Self::new()
    }
}

impl QuickModeState {
    /// Creates an empty quick-mode session ready for input.
    pub fn new() -> Self {
        Self {
            question: String::new(),
            answer: String::new(),
            phase: QuickPhase::Composing,
            saved_thread_id: None,
            dismissed: false,
        }
    }

    /// Current lifecycle phase.
    pub fn phase(&self) -> QuickPhase {
        self.phase.clone()
    }

    /// Question text.
    pub fn question(&self) -> &str {
        &self.question
    }

    /// Accumulated answer text.
    pub fn answer(&self) -> &str {
        &self.answer
    }

    /// Whether Esc (or equivalent) has dismissed the overlay.
    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    /// Thread id after a successful save, if any.
    pub fn saved_thread_id(&self) -> Option<&str> {
        self.saved_thread_id.as_deref()
    }

    /// Updates the question draft.
    pub fn set_question(&mut self, question: impl Into<String>) {
        self.question = question.into();
        if !matches!(self.phase, QuickPhase::Streaming) {
            self.phase = QuickPhase::Composing;
            self.answer.clear();
            self.saved_thread_id = None;
        }
    }

    /// Marks generation as started and clears any prior answer.
    pub fn begin_streaming(&mut self) {
        self.answer.clear();
        self.saved_thread_id = None;
        self.phase = QuickPhase::Streaming;
    }

    /// Appends a streamed answer chunk.
    pub fn append_chunk(&mut self, chunk: &str) {
        if matches!(self.phase, QuickPhase::Streaming) {
            self.answer.push_str(chunk);
        }
    }

    /// Marks the stream as successfully complete.
    pub fn finish_streaming(&mut self) {
        if matches!(self.phase, QuickPhase::Streaming) {
            self.phase = QuickPhase::Complete;
        }
    }

    /// Marks the stream as failed with a user-visible reason.
    pub fn fail(&mut self, message: impl Into<String>) {
        self.phase = QuickPhase::Failed {
            message: message.into(),
        };
    }

    /// Applies one ephemeral stream event to this state.
    pub fn apply_stream_event(&mut self, event: QuickStreamEvent) {
        match event {
            QuickStreamEvent::Chunk(chunk) => self.append_chunk(&chunk),
            QuickStreamEvent::Done => self.finish_streaming(),
            QuickStreamEvent::Error(message) => self.fail(message),
        }
    }

    /// Dismisses the overlay (Esc).
    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }

    /// Returns the answer for clipboard copy when one is available.
    pub fn copy_answer(&self) -> Option<String> {
        let answer = self.answer.trim();
        if answer.is_empty() {
            None
        } else {
            Some(answer.to_string())
        }
    }

    /// Whether the completed exchange can be saved to a thread.
    pub fn can_save(&self) -> bool {
        matches!(self.phase, QuickPhase::Complete)
            && !self.question.trim().is_empty()
            && !self.answer.trim().is_empty()
    }

    /// Records that the exchange was persisted under `thread_id`.
    pub fn mark_saved(&mut self, thread_id: impl Into<String>) {
        self.saved_thread_id = Some(thread_id.into());
    }

    /// Whether the saved thread can be opened in the main window.
    pub fn can_open_in_main(&self) -> bool {
        self.saved_thread_id.is_some()
    }

    /// Whether Enter should submit the question for generation.
    pub fn can_submit(&self) -> bool {
        matches!(self.phase, QuickPhase::Composing) && !self.question.trim().is_empty()
    }
}

/// Builds a one-shot provider request for the quick overlay (no thread history).
pub fn build_quick_chat_request(
    question: &str,
    model: &str,
    system_prompt: &str,
) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: question.to_string(),
        }],
        system_prompt: Some(system_prompt.to_string()),
    }
}

/// Logical size of the quick overlay window.
pub fn quick_window_size() -> (f32, f32) {
    (QUICK_WINDOW_WIDTH, QUICK_WINDOW_HEIGHT)
}

/// Label for the copy-answer action.
pub fn copy_answer_label() -> &'static str {
    "Copy"
}

/// Label for saving the exchange into a chat thread.
pub fn save_to_thread_label() -> &'static str {
    "Save to thread"
}

/// Label for appending the exchange to the currently selected main-window thread.
pub fn save_to_current_label() -> &'static str {
    "Save to current"
}

/// Label for opening the saved thread in the main Ronin window.
pub fn open_in_main_label() -> &'static str {
    "Open in Ronin"
}

/// Footer hint for dismissing the overlay.
pub fn dismiss_hint() -> &'static str {
    "Esc to dismiss"
}

/// Placeholder shown in the empty question field.
pub fn question_placeholder() -> &'static str {
    "Ask Ronin…"
}

/// Title shown in the quick overlay chrome.
pub fn quick_overlay_title() -> &'static str {
    "Quick"
}

/// Resolves the quick overlay palette from config preference and desktop appearance.
///
/// Quick mode shares the shell theme so light / dark / system preferences stay in sync.
pub fn resolve_quick_overlay_theme(
    preference: ronin_core::ThemePreference,
    appearance: gpui::WindowAppearance,
) -> crate::theme::M0Theme {
    crate::theme::resolve_shell_theme(preference, appearance)
}
