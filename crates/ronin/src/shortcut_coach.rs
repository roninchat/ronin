//! First-run shortcut coach steps.

/// One card in the first-run coach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoachStep {
    /// Stable step id.
    pub id: &'static str,
    /// Short title.
    pub title: &'static str,
    /// Body copy with the key chord.
    pub body: &'static str,
}

/// At most four first-run steps: send, new chat, palette, sidebar.
pub fn first_run_steps() -> &'static [CoachStep] {
    &[
        CoachStep {
            id: "send",
            title: "Send a message",
            body: "Press Enter to send. Shift+Enter inserts a new line.",
        },
        CoachStep {
            id: "new-chat",
            title: "Start a new chat",
            body: "Press Ctrl+N to open a new thread.",
        },
        CoachStep {
            id: "palette",
            title: "Jump with the palette",
            body: "Press Ctrl+P to open a thread or run a command.",
        },
        CoachStep {
            id: "sidebar",
            title: "Show the sidebar",
            body: "Use the menu button or Ctrl+B to show or hide the thread list.",
        },
    ]
}

/// Whether the first-run coach should appear.
///
/// Shown only when the user has not dismissed it and shortcut hints are enabled.
pub fn should_show_coach(coach_seen: bool, show_hints: bool) -> bool {
    !coach_seen && show_hints
}
