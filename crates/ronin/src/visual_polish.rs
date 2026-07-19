//! Premium visual polish: elevation tokens, empty/error presentations, streaming motion.
//!
//! These are presentation-module seams — testable without GPUI pixel assertions.

use gpui::{hsla, point, px, BoxShadow, Hsla};
use ronin_core::ColorScheme;

/// Elevation level for soft shadows on raised surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    /// Code blocks, preview cards, compact chrome.
    Low,
    /// Composer, sidebar, side panels.
    Medium,
    /// Dialogs and modal overlays.
    High,
}

/// Theme-aware shadow metrics for one elevation level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationStyle {
    /// Vertical offset in pixels.
    pub offset_y: f32,
    /// Blur radius in pixels.
    pub blur_radius: f32,
    /// Spread radius in pixels.
    pub spread_radius: f32,
    /// Shadow opacity (0–1) applied to black.
    pub shadow_alpha: f32,
}

impl ElevationStyle {
    /// Builds a GPUI box-shadow list for this elevation style.
    pub fn box_shadows(self) -> Vec<BoxShadow> {
        vec![BoxShadow {
            color: hsla(0., 0., 0., self.shadow_alpha),
            offset: point(px(0.), px(self.offset_y)),
            blur_radius: px(self.blur_radius),
            spread_radius: px(self.spread_radius),
        }]
    }

    /// Shadow color as [`Hsla`] for custom composition.
    pub fn shadow_color(self) -> Hsla {
        hsla(0., 0., 0., self.shadow_alpha)
    }
}

/// Resolves soft-shadow metrics for `level` under `scheme`.
///
/// Light themes use softer (lower-alpha) shadows; dark themes use stronger
/// opacity so elevation remains visible on dark surfaces.
pub fn elevation_style(level: Elevation, scheme: ColorScheme) -> ElevationStyle {
    let (offset_y, blur_radius, spread_radius, light_alpha, dark_alpha) = match level {
        Elevation::Low => (1.0, 4.0, 0.0, 0.08, 0.28),
        Elevation::Medium => (2.0, 10.0, 0.0, 0.10, 0.36),
        Elevation::High => (4.0, 20.0, 0.0, 0.14, 0.45),
    };
    let shadow_alpha = match scheme {
        ColorScheme::Light => light_alpha,
        ColorScheme::Dark => dark_alpha,
    };
    ElevationStyle {
        offset_y,
        blur_radius,
        spread_radius,
        shadow_alpha,
    }
}

/// Major empty-state surfaces called out by the polish AC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyStateKind {
    /// Sidebar thread list is empty.
    NoThreads,
    /// Active thread has no messages yet.
    EmptyThread,
    /// Artifacts panel has nothing saved.
    NoArtifacts,
    /// Memories panel has nothing saved.
    NoMemories,
    /// A search / completion query returned no hits.
    NoSearchResults,
    /// Local Ollama daemon is unreachable.
    OllamaOffline,
    /// Ollama is up but no models are installed.
    NoModelsInstalled,
}

/// Designed empty-state copy: icon glyph, title, body, and optional next step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyStateContent {
    /// Short glyph or symbol (not emoji-heavy).
    pub icon: &'static str,
    /// Primary headline.
    pub title: &'static str,
    /// Supporting explanation.
    pub body: &'static str,
    /// Actionable next-step hint.
    pub action_hint: Option<&'static str>,
}

impl EmptyStateContent {
    /// Formats a multi-line block suitable for text-only surfaces.
    pub fn display_text(self) -> String {
        let mut out = format!("{}\n{}\n{}", self.icon, self.title, self.body);
        if let Some(hint) = self.action_hint {
            out.push('\n');
            out.push_str(hint);
        }
        out
    }
}

/// Builds designed empty-state content for `kind`.
pub fn empty_state(kind: EmptyStateKind) -> EmptyStateContent {
    match kind {
        EmptyStateKind::NoThreads => EmptyStateContent {
            icon: "○",
            title: "No threads yet",
            body: "Start a new chat to keep conversations organized in the sidebar.",
            action_hint: Some("Click “New Chat” to begin."),
        },
        EmptyStateKind::EmptyThread => EmptyStateContent {
            icon: "◌",
            title: "Start a conversation",
            body: "Ask anything — your messages will appear here.",
            action_hint: Some("Type in the composer and press Enter to send."),
        },
        EmptyStateKind::NoArtifacts => EmptyStateContent {
            icon: "◇",
            title: "No artifacts yet",
            body: "Save an assistant message as an artifact to see it here.",
            action_hint: Some("Use “Save as artifact” on an assistant reply."),
        },
        EmptyStateKind::NoMemories => EmptyStateContent {
            icon: "◎",
            title: "No memories yet",
            body: "Saved memories help Ronin recall facts across threads.",
            action_hint: Some("Save a memory from a useful assistant reply."),
        },
        EmptyStateKind::NoSearchResults => EmptyStateContent {
            icon: "⌕",
            title: "No results",
            body: "Nothing matched this search. Try a shorter path or different name.",
            action_hint: Some("Clear a few characters and search again."),
        },
        EmptyStateKind::OllamaOffline => EmptyStateContent {
            icon: "⚠",
            title: "Ollama is not running",
            body: "Ronin cannot reach the local Ollama server.",
            action_hint: Some("Start it with `ollama serve`, or install from https://ollama.com."),
        },
        EmptyStateKind::NoModelsInstalled => EmptyStateContent {
            icon: "◻",
            title: "No models installed",
            body: "Ollama is reachable but has no models to chat with.",
            action_hint: Some("Try: ollama pull llama3.2"),
        },
    }
}

/// Categories of user-visible failures that share one presentation pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Provider configuration or connectivity failure.
    Provider,
    /// Chat stream interrupted or failed mid-response.
    StreamFailure,
    /// Database schema migration failure at startup.
    MigrationFailure,
    /// Attachment / clipboard / screenshot failure.
    Attachment,
}

/// Consistent error chrome: icon, title, detail message, action hint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPresentation {
    /// Short glyph for the error row.
    pub icon: &'static str,
    /// Short headline.
    pub title: &'static str,
    /// Detail including the underlying error text.
    pub message: String,
    /// Optional recovery hint.
    pub action_hint: Option<&'static str>,
}

impl ErrorPresentation {
    /// True when title, message, or hint mentions retrying.
    pub fn body_mentions_retry(&self) -> bool {
        let hint = self.action_hint.unwrap_or("");
        [self.title, self.message.as_str(), hint]
            .iter()
            .any(|s| s.to_lowercase().contains("retry"))
    }

    /// Formats a multi-line block for text-only surfaces.
    pub fn display_text(&self) -> String {
        let mut out = format!("{}\n{}\n{}", self.icon, self.title, self.message);
        if let Some(hint) = self.action_hint {
            out.push('\n');
            out.push_str(hint);
        }
        out
    }
}

/// Builds a consistent, actionable error presentation.
pub fn error_presentation(kind: ErrorKind, detail: &str) -> ErrorPresentation {
    match kind {
        ErrorKind::Provider => ErrorPresentation {
            icon: "!",
            title: "Provider error",
            message: detail.to_string(),
            action_hint: Some("Check provider settings, then try again."),
        },
        ErrorKind::StreamFailure => ErrorPresentation {
            icon: "!",
            title: "Stream failed",
            message: detail.to_string(),
            action_hint: Some("Retry the message, or cancel and send again."),
        },
        ErrorKind::MigrationFailure => ErrorPresentation {
            icon: "!",
            title: "Database migration failed",
            message: detail.to_string(),
            action_hint: Some("Back up your data directory, then restart Ronin."),
        },
        ErrorKind::Attachment => ErrorPresentation {
            icon: "!",
            title: "Attachment error",
            message: detail.to_string(),
            action_hint: Some("Remove the attachment or try a different file."),
        },
    }
}

/// Timing tokens for cursor blink and generating indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamingMotion {
    /// Full cursor blink period in milliseconds.
    pub cursor_cycle_ms: u64,
    /// How long the cursor stays visible within each cycle.
    pub cursor_visible_ms: u64,
    /// Full generating-indicator pulse period in milliseconds.
    pub generate_pulse_ms: u64,
}

/// Returns the default smooth streaming motion profile.
pub fn streaming_motion() -> StreamingMotion {
    StreamingMotion {
        cursor_cycle_ms: 800,
        cursor_visible_ms: 480,
        generate_pulse_ms: 900,
    }
}

/// Whether the caret should paint at `elapsed_ms` under `motion`.
pub fn cursor_visible_at(elapsed_ms: u64, motion: &StreamingMotion) -> bool {
    (elapsed_ms % motion.cursor_cycle_ms) < motion.cursor_visible_ms
}

/// Phase (0..=2) for a three-step generating indicator at `elapsed_ms`.
pub fn generating_pulse_phase(elapsed_ms: u64, motion: &StreamingMotion) -> u8 {
    let step = (motion.generate_pulse_ms / 3).max(1);
    ((elapsed_ms / step) % 3) as u8
}

/// Label for the generating indicator at `elapsed_ms`.
pub fn generating_label(elapsed_ms: u64, motion: &StreamingMotion) -> String {
    let dots = match generating_pulse_phase(elapsed_ms, motion) {
        0 => ".",
        1 => "..",
        _ => "...",
    };
    format!("Generating response{dots}")
}
