//! Model picker presentation: provider grouping, capabilities, keyboard nav.
//!
//! Public seams — testable without GPUI.

use crate::context_indicator::{format_token_count, resolve_model_context_window};
use ronin_core::ColorScheme;

/// Which backend a model belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelProviderKind {
    /// Local Ollama models.
    Ollama,
    /// OpenAI-compatible HTTP API.
    OpenAi,
}

impl ModelProviderKind {
    /// User-visible provider label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ollama => "Ollama",
            Self::OpenAi => "OpenAI",
        }
    }

    /// Stable id used when persisting thread provider (`"ollama"` / `"openai"`).
    pub fn id(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::OpenAi => "openai",
        }
    }

    /// Parses a stored provider id.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "ollama" => Some(Self::Ollama),
            "openai" => Some(Self::OpenAi),
            _ => None,
        }
    }
}

/// Known capability metadata for a model name.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModelCapabilities {
    /// Approximate context window in tokens, when known.
    pub context_window_tokens: Option<usize>,
    /// Whether the model is known to accept image / vision inputs.
    pub supports_vision: bool,
}

impl ModelCapabilities {
    /// Short capability chips for UI (e.g. `128k ctx`, `vision`).
    pub fn summary_parts(&self) -> Vec<String> {
        let mut parts = Vec::new();
        if let Some(tokens) = self.context_window_tokens {
            parts.push(format!("{} ctx", format_token_count(tokens)));
        }
        if self.supports_vision {
            parts.push("vision".to_string());
        }
        parts
    }
}

/// One selectable row in the model picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelPickerEntry {
    /// Provider kind.
    pub provider: ModelProviderKind,
    /// Provider display label.
    pub provider_label: &'static str,
    /// Model identifier / name.
    pub model_name: String,
    /// Inferred capabilities.
    pub capabilities: ModelCapabilities,
    /// Whether this entry is the thread's currently active model.
    pub is_active: bool,
}

/// Keyboard keys handled by the model picker overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelPickerKey {
    /// Move highlight up.
    Up,
    /// Move highlight down.
    Down,
    /// Confirm selection.
    Enter,
    /// Dismiss without selecting.
    Escape,
}

/// Result of handling a picker key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelPickerAction {
    /// No-op.
    None,
    /// Highlight moved.
    HighlightChanged {
        /// New selected index.
        index: usize,
    },
    /// User confirmed an entry (picker closes).
    Select {
        /// Selected entry index.
        index: usize,
    },
    /// User dismissed the picker.
    Dismiss,
}

/// Open/closed + highlight state for the model picker overlay.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModelPickerState {
    open: bool,
    selected: usize,
}

impl ModelPickerState {
    /// Whether the overlay is visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Opens the picker, resetting highlight to the first entry.
    pub fn open_with_count(&mut self, _entry_count: usize) {
        self.open = true;
        self.selected = 0;
    }

    /// Closes the picker.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Current highlight index.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Handles a key while the picker is open.
    pub fn handle_key(&mut self, key: ModelPickerKey, entry_count: usize) -> ModelPickerAction {
        if !self.open {
            return ModelPickerAction::None;
        }
        match key {
            ModelPickerKey::Escape => {
                self.close();
                ModelPickerAction::Dismiss
            }
            ModelPickerKey::Enter => {
                if entry_count == 0 {
                    self.close();
                    return ModelPickerAction::Dismiss;
                }
                let index = self.selected.min(entry_count.saturating_sub(1));
                self.close();
                ModelPickerAction::Select { index }
            }
            ModelPickerKey::Down => {
                if entry_count == 0 {
                    return ModelPickerAction::None;
                }
                let next = (self.selected + 1) % entry_count;
                self.selected = next;
                ModelPickerAction::HighlightChanged { index: next }
            }
            ModelPickerKey::Up => {
                if entry_count == 0 {
                    return ModelPickerAction::None;
                }
                let next = if self.selected == 0 {
                    entry_count - 1
                } else {
                    self.selected - 1
                };
                self.selected = next;
                ModelPickerAction::HighlightChanged { index: next }
            }
        }
    }
}

/// Infers capabilities from a model name (heuristic catalog).
pub fn infer_model_capabilities(model_name: &str) -> ModelCapabilities {
    let base = model_name
        .split([':', '/', '@'])
        .next()
        .unwrap_or(model_name)
        .to_ascii_lowercase();
    let context_window_tokens = resolve_model_context_window(model_name);
    let supports_vision = base.starts_with("gpt-4o")
        || base.starts_with("chatgpt-4o")
        || base.starts_with("llava")
        || base.starts_with("bakllava")
        || base.starts_with("moondream")
        || base.contains("vision");
    ModelCapabilities {
        context_window_tokens,
        supports_vision,
    }
}

/// Joins capability chips into a single summary string.
pub fn format_capability_summary(caps: &ModelCapabilities) -> String {
    caps.summary_parts().join(" · ")
}

/// Builds flat picker entries from provider → model-name lists.
///
/// Providers are ordered Ollama then OpenAI regardless of input order.
pub fn build_picker_entries(
    provider_models: &[(ModelProviderKind, Vec<String>)],
) -> Vec<ModelPickerEntry> {
    let order = [ModelProviderKind::Ollama, ModelProviderKind::OpenAi];
    let mut out = Vec::new();
    for kind in order {
        for (provider, models) in provider_models {
            if *provider != kind {
                continue;
            }
            for name in models {
                out.push(ModelPickerEntry {
                    provider: kind,
                    provider_label: kind.label(),
                    model_name: name.clone(),
                    capabilities: infer_model_capabilities(name),
                    is_active: false,
                });
            }
        }
    }
    out
}

/// Groups flat entries into provider sections (Ollama first).
pub fn group_entries_by_provider(
    entries: &[ModelPickerEntry],
) -> Vec<(ModelProviderKind, Vec<&ModelPickerEntry>)> {
    let order = [ModelProviderKind::Ollama, ModelProviderKind::OpenAi];
    let mut out = Vec::new();
    for kind in order {
        let group: Vec<&ModelPickerEntry> = entries.iter().filter(|e| e.provider == kind).collect();
        if !group.is_empty() {
            out.push((kind, group));
        }
    }
    out
}

/// Marks the entry matching `provider_id` + `model_name` as active.
pub fn mark_active_entry(
    entries: &[ModelPickerEntry],
    provider_id: &str,
    model_name: &str,
) -> Vec<ModelPickerEntry> {
    entries
        .iter()
        .map(|e| {
            let mut clone = e.clone();
            clone.is_active = e.provider.id() == provider_id && e.model_name == model_name;
            clone
        })
        .collect()
}

/// Index of the active entry, if any.
pub fn active_entry_index(entries: &[ModelPickerEntry]) -> Option<usize> {
    entries.iter().position(|e| e.is_active)
}

/// Opens the picker with the active entry highlighted (or index 0).
pub fn open_picker_at_active(state: &mut ModelPickerState, entries: &[ModelPickerEntry]) {
    let index = active_entry_index(entries).unwrap_or(0);
    state.open = true;
    state.selected = if entries.is_empty() {
        0
    } else {
        index.min(entries.len() - 1)
    };
}

/// Builds picker entries from shell `(provider_id, models)` lists and marks active.
///
/// Unknown provider ids are skipped. Order is Ollama then OpenAI.
pub fn entries_from_listed_providers(
    listed: &[(String, Vec<String>)],
    active_provider: Option<&str>,
    active_model: Option<&str>,
) -> Vec<ModelPickerEntry> {
    let mut typed: Vec<(ModelProviderKind, Vec<String>)> = Vec::new();
    for (id, models) in listed {
        if let Some(kind) = ModelProviderKind::from_id(id) {
            typed.push((kind, models.clone()));
        }
    }
    let entries = build_picker_entries(&typed);
    match (active_provider, active_model) {
        (Some(provider), Some(model)) => mark_active_entry(&entries, provider, model),
        _ => entries,
    }
}

/// Rebuilds entries from fresh provider lists while the picker is open.
///
/// Clamps highlight to a valid index after the list changes. Marks the
/// thread's active model.
pub fn refresh_picker_entries(
    state: &mut ModelPickerState,
    provider_models: &[(ModelProviderKind, Vec<String>)],
    active_provider: &str,
    active_model: &str,
) -> Vec<ModelPickerEntry> {
    let built = build_picker_entries(provider_models);
    let entries = mark_active_entry(&built, active_provider, active_model);

    if state.is_open() {
        if entries.is_empty() {
            state.selected = 0;
        } else if state.selected >= entries.len() {
            state.selected = entries.len() - 1;
        }
    }
    entries
}

/// Visual tone for a picker row (theme mapping happens in [`picker_row_colors`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickerRowTone {
    /// Idle row.
    Default,
    /// Keyboard / hover highlight.
    Highlighted,
    /// Currently active thread model (not highlighted).
    Active,
}

/// Resolves row tone from highlight + active flags (highlight wins).
pub fn picker_row_tone(highlighted: bool, is_active: bool) -> PickerRowTone {
    if highlighted {
        PickerRowTone::Highlighted
    } else if is_active {
        PickerRowTone::Active
    } else {
        PickerRowTone::Default
    }
}

/// Theme-aware colors for a model picker row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PickerRowColors {
    /// Row background.
    pub background: gpui::Hsla,
    /// Primary text.
    pub text: gpui::Hsla,
    /// Secondary / capability text.
    pub text_muted: gpui::Hsla,
}

/// Resolves picker row colors for the given scheme and tone.
pub fn picker_row_colors(scheme: ColorScheme, tone: PickerRowTone) -> PickerRowColors {
    use crate::theme::M0Theme;
    let theme = M0Theme::for_scheme(scheme);
    match tone {
        PickerRowTone::Default => PickerRowColors {
            background: theme.surface_muted,
            text: theme.text_primary,
            text_muted: theme.text_muted,
        },
        PickerRowTone::Highlighted => PickerRowColors {
            background: theme.accent,
            text: theme.accent_text,
            text_muted: theme.accent_text,
        },
        PickerRowTone::Active => PickerRowColors {
            background: theme.surface_selected,
            text: theme.text_primary,
            text_muted: theme.text_muted,
        },
    }
}
