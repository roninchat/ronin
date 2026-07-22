//! UI-facing shell controller backed by `RoninSession` behavior.

use std::sync::mpsc;

use ronin_core::{
    shape_generation_notification, ChatProvider, ChatStreamEvent, ContextAttachmentDraft,
    DesktopNotificationRequest, GenerationNotifyInput, GenerationNotifyKind, HttpOllamaProvider,
    Message, MessageRole, MessageStatus, NotificationPrefs, OllamaProvider,
    OpenAiCompatibleProvider, RoninError, RoninPaths, RoninSession, Thread,
};

use crate::branches::{leaf_under_root, sibling_branch_nav, MessageNode};
use crate::chat::{
    build_capped_chat_request, build_title_generation_request, collect_streamed_title,
    derive_thread_title, may_apply_auto_title, sanitize_generated_title,
};
use crate::error::{Result, RoninAppError};
use crate::status::{
    format_provider_error, probe_provider_status, run_connection_test, ConnectionTestResult,
    ProviderStatus,
};
use crate::tools::next_tool_result;

/// M0 design checkpoint values shown to reviewers before deeper UI work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualDirection {
    /// Visual qualities used to judge the running shell.
    pub assessment_axes: &'static [&'static str],
    /// Concrete visual changes required before deeper UI work proceeds.
    pub required_changes_before_deeper_ui: &'static [&'static str],
    /// Decision about whether Ronin copied/adapted Zed UI code for M0.
    pub reuse_decision: VisualReuseDecision,
}

/// Source/reuse decision for M0 shell visuals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualReuseDecision {
    /// M0 uses custom GPUI layout/styling instead of copied Zed UI code.
    CustomGpui {
        /// Reason selected Zed UI code was not copied or adapted.
        reason: &'static str,
    },
}

/// Update sent from the streaming background thread to the UI.
#[derive(Debug, Clone)]
pub enum StreamUpdate {
    /// A single token delta from the provider.
    Chunk(String),
    /// Streaming finished successfully; final accumulated content for DB.
    Done(String),
    /// Streaming encountered an error.
    Error(String),
    /// Move to the next turn (e.g. after a tool execution)
    NextTurn {
        /// The new assistant message ID to stream into.
        new_assistant_msg_id: String,
    },
}

/// Snapshot of state rendered by the native shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellState {
    /// Native window title.
    pub window_title: String,
    /// Persisted threads visible in sidebar.
    pub threads: Vec<Thread>,
    /// Currently selected thread id, if any.
    pub selected_thread_id: Option<String>,
    /// Sidebar provider/model status.
    pub provider_status: ProviderStatus,
    /// Latest explicit "Test Connection" result, if any.
    pub connection_test: Option<ConnectionTestResult>,
    /// Messages for the selected thread, if loaded.
    pub messages: Option<Vec<Message>>,
    /// Whether older messages were omitted due to context caps.
    pub truncation_notice: bool,
    /// Thread id currently waiting on a model title-generation request, if any.
    pub title_generating_thread_id: Option<String>,
}

fn persist_context_attachments(
    session: &RoninSession,
    message_id: &str,
    attachments: &[ContextAttachmentDraft],
) -> Result<()> {
    for attachment in attachments {
        let path = attachment
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
        session.create_attachment(
            message_id,
            attachment.kind,
            &attachment.name,
            &attachment.mime_type,
            attachment.content.as_deref(),
            path.as_deref(),
        )?;
    }
    Ok(())
}

fn attachment_context_block(attachments: &[ContextAttachmentDraft]) -> Option<String> {
    if attachments.is_empty() {
        return None;
    }

    Some(
        attachments
            .iter()
            .map(|attachment| attachment.context_block.as_str())
            .collect::<Vec<_>>()
            .join("\n\n"),
    )
}

/// Builds the always-on profile memory context block (enabled profile only).
fn profile_memory_context_block(session: &RoninSession) -> Option<String> {
    let memories = session.list_memories().ok()?;
    let active: Vec<_> = memories
        .iter()
        .filter(|m| m.enabled && m.is_profile)
        .collect();
    if active.is_empty() {
        return None;
    }
    Some(
        active
            .iter()
            .map(|m| format!("[Profile memory: {}]\n{}", m.title, m.content))
            .collect::<Vec<_>>()
            .join("\n\n"),
    )
}

fn merge_request_context(
    memory_block: Option<String>,
    attachment_block: Option<String>,
) -> Option<String> {
    match (memory_block, attachment_block) {
        (Some(m), Some(a)) if !m.is_empty() && !a.is_empty() => Some(format!("{m}\n\n{a}")),
        (Some(m), _) if !m.is_empty() => Some(m),
        (_, Some(a)) if !a.is_empty() => Some(a),
        _ => None,
    }
}

fn visible_message_or_attachment_placeholder<'a>(
    content: &'a str,
    attachments: &[ContextAttachmentDraft],
) -> &'a str {
    if content.trim().is_empty() && !attachments.is_empty() {
        "See attached context."
    } else {
        content
    }
}

/// UI-facing shell controller backed by `RoninSession` behavior.
pub struct RoninShell {
    session: RoninSession,
    state: ShellState,
    /// In-flight generations keyed by thread id (concurrent across threads).
    active_generations: std::collections::HashMap<String, ActiveGeneration>,
    /// Thread ids whose titles were set by an explicit user rename.
    manual_titles: std::collections::HashSet<String>,
    /// Receiver for background model title-generation results.
    title_gen_rx: Option<mpsc::Receiver<TitleGenResult>>,
    /// Shaped desktop notifications waiting for the host port to deliver.
    pending_desktop_notifications: Vec<DesktopNotificationRequest>,
}

/// One thread's active streaming generation.
struct ActiveGeneration {
    streaming_rx: mpsc::Receiver<StreamUpdate>,
    streaming_msg_id: String,
    /// Chunks received so far (survives thread switches for live resume).
    accumulated: String,
}

struct TitleGenResult {
    thread_id: String,
    title: Option<String>,
    error: Option<String>,
}

impl RoninShell {
    /// Returns the M0 visual direction checkpoint for human review.
    pub fn m0_visual_direction() -> VisualDirection {
        VisualDirection {
            assessment_axes: &["rounded", "soft", "premium", "Linux-native", "Zed-grade"],
            required_changes_before_deeper_ui: &[],
            reuse_decision: VisualReuseDecision::CustomGpui {
                reason: "Zed UI extraction is unnecessary for the M0 shell; Ronin needs a small custom GPUI layout before deeper chat and markdown polish.",
            },
        }
    }

    /// Opens the shell state with Ollama selected as the provider.
    pub fn open_with_ollama(paths: RoninPaths) -> Result<Self> {
        let provider = ronin_core::HttpOllamaProvider::new("http://localhost:11434");
        Self::open_with_ollama_provider(paths, provider)
    }

    /// Opens the shell with a custom Ollama provider (for testing).
    pub fn open_with_ollama_provider(
        paths: RoninPaths,
        provider: impl ronin_core::OllamaProvider,
    ) -> Result<Self> {
        let session = RoninSession::open(paths)?;
        let mut threads = session.list_threads()?;
        if threads.is_empty() {
            threads.push(session.create_thread()?);
            tracing::info!("ronin shell created initial thread");
        }
        let selected_thread_id = threads.first().map(|thread| thread.id.clone());

        let provider_status = probe_provider_status(&provider, &session);
        tracing::info!(
            thread_count = threads.len(),
            provider_status = ?provider_status,
            "ronin shell state restored with provider"
        );

        Ok(Self::from_session(
            session,
            threads,
            selected_thread_id,
            provider_status,
        ))
    }

    /// Opens the shell state from persisted Ronin paths.
    pub fn open(paths: RoninPaths) -> Result<Self> {
        let session = RoninSession::open(paths)?;
        let mut threads = session.list_threads()?;
        if threads.is_empty() {
            threads.push(session.create_thread()?);
            tracing::info!("ronin shell created initial thread");
        }
        let selected_thread_id = threads.first().map(|thread| thread.id.clone());
        tracing::info!(thread_count = threads.len(), "ronin shell state restored");

        Ok(Self::from_session(
            session,
            threads,
            selected_thread_id,
            ProviderStatus::NotConfigured,
        ))
    }

    /// Opens the shell with a newly created empty thread selected.
    pub fn open_with_new_thread(paths: RoninPaths) -> Result<Self> {
        let session = RoninSession::open(paths)?;
        let mut threads = session.list_threads()?;
        let thread = session.create_thread()?;
        let selected_thread_id = Some(thread.id.clone());
        threads.push(thread);
        tracing::info!(
            thread_count = threads.len(),
            "ronin shell opened with new thread selected"
        );

        Ok(Self::from_session(
            session,
            threads,
            selected_thread_id,
            ProviderStatus::NotConfigured,
        ))
    }

    fn from_session(
        session: RoninSession,
        threads: Vec<Thread>,
        selected_thread_id: Option<String>,
        status: ProviderStatus,
    ) -> Self {
        let messages = selected_thread_id
            .as_deref()
            .and_then(|id| session.list_messages(id).ok());

        Self {
            session,
            state: ShellState {
                window_title: "Ronin".to_string(),
                threads,
                selected_thread_id,
                provider_status: status,
                connection_test: None,
                messages,
                truncation_notice: false,
                title_generating_thread_id: None,
            },
            active_generations: std::collections::HashMap::new(),
            manual_titles: std::collections::HashSet::new(),
            title_gen_rx: None,
            pending_desktop_notifications: Vec::new(),
        }
    }

    /// Drains shaped desktop notifications produced by finished generations.
    pub fn drain_pending_desktop_notifications(&mut self) -> Vec<DesktopNotificationRequest> {
        std::mem::take(&mut self.pending_desktop_notifications)
    }

    /// Queues a generation-completed / generation-failed notification when enabled.
    fn enqueue_generation_notification(
        &mut self,
        kind: GenerationNotifyKind,
        thread_id: &str,
        error_summary: Option<&str>,
    ) {
        let enabled = self
            .session
            .load_config()
            .map(|c| c.notifications.enabled)
            .unwrap_or(true);
        let thread_title = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .map(|t| t.title.clone());
        let input = GenerationNotifyInput {
            kind,
            thread_id: thread_id.to_string(),
            thread_title,
            error_summary: error_summary.map(str::to_string),
        };
        if let Some(request) = shape_generation_notification(&NotificationPrefs { enabled }, &input)
        {
            self.pending_desktop_notifications.push(request);
        }
    }

    /// Creates a new thread from the sidebar action and selects it.
    pub fn create_new_thread(&mut self) -> Result<Thread> {
        let thread = self.session.create_thread()?;
        self.state.selected_thread_id = Some(thread.id.clone());
        self.state.messages = self.session.list_messages(&thread.id).ok();
        self.state.threads.push(thread.clone());
        let _ = self.refresh_provider_status();
        tracing::info!(thread_id = %thread.id, "ronin shell created and selected thread");
        Ok(thread)
    }

    /// Persists a completed quick-mode Q&A into a brand-new thread and selects it.
    pub fn save_quick_exchange(&mut self, question: &str, answer: &str) -> Result<Thread> {
        let thread = self.create_new_thread()?;
        self.save_quick_exchange_to_thread(&thread.id, question, answer)?;
        // Reload so selected state reflects the exchange and derived title.
        if let Ok(threads) = self.session.list_threads() {
            self.state.threads = threads;
        }
        self.state.messages = self.session.list_messages(&thread.id).ok();
        let selected = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread.id)
            .cloned()
            .unwrap_or(thread);
        Ok(selected)
    }

    /// Appends a completed quick-mode Q&A onto an existing thread.
    pub fn save_quick_exchange_to_thread(
        &mut self,
        thread_id: &str,
        question: &str,
        answer: &str,
    ) -> Result<()> {
        self.persist_user_message(thread_id, question, &[])?;
        self.session
            .create_message(thread_id, MessageRole::Assistant, answer)?;
        if self.state.selected_thread_id.as_deref() == Some(thread_id) {
            self.state.messages = self.session.list_messages(thread_id).ok();
        }
        if let Ok(threads) = self.session.list_threads() {
            self.state.threads = threads;
        }
        tracing::info!(%thread_id, "ronin shell saved quick exchange to thread");
        Ok(())
    }

    /// Selects a loaded thread from the sidebar.
    pub fn select_thread(&mut self, thread_id: &str) -> Result<()> {
        let exists = self
            .state
            .threads
            .iter()
            .any(|thread| thread.id.as_str() == thread_id);
        if !exists {
            return Err(RoninAppError::ThreadNotLoaded {
                thread_id: thread_id.to_string(),
            });
        }

        self.state.selected_thread_id = Some(thread_id.to_string());
        self.state.messages = self.session.list_messages(thread_id).ok();
        self.apply_live_stream_overlay(thread_id);
        let _ = self.refresh_provider_status();
        tracing::info!(thread_id, "ronin shell selected thread");
        Ok(())
    }

    /// Overlays in-memory streamed content onto the selected thread's messages.
    fn apply_live_stream_overlay(&mut self, thread_id: &str) {
        let Some(gen) = self.active_generations.get(thread_id) else {
            return;
        };
        let msg_id = gen.streaming_msg_id.clone();
        let accumulated = gen.accumulated.clone();
        if let Some(ref mut msgs) = self.state.messages {
            if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                if accumulated.len() > msg.content.len() {
                    msg.content = accumulated;
                }
                msg.status = MessageStatus::Streaming;
            }
        }
    }

    /// Selects a model and persists the choice to config.
    pub fn select_model(&mut self, model: &str) -> Result<()> {
        self.session.save_selected_model(model)?;
        let is_openai = matches!(
            self.state.provider_status,
            ProviderStatus::OpenAiReady { .. }
                | ProviderStatus::OpenAiError { .. }
                | ProviderStatus::OpenAiNotConfigured
        );
        if is_openai {
            self.state.provider_status = ProviderStatus::OpenAiReady {
                model: model.to_string(),
            };
        } else {
            self.state.provider_status = ProviderStatus::OllamaOnline {
                model: model.to_string(),
            };
        }
        tracing::info!(model, "ronin shell selected model");
        Ok(())
    }

    /// Sets provider + model for a thread, persists both, and refreshes status.
    pub fn select_thread_provider_model(
        &mut self,
        thread_id: &str,
        provider: &str,
        model: &str,
    ) -> Result<()> {
        self.session.set_thread_provider(thread_id, provider)?;
        self.session.set_thread_model(thread_id, model)?;
        self.session.save_selected_model(model)?;
        self.state.threads = self.session.list_threads()?;
        if self.state.selected_thread_id.as_deref() == Some(thread_id) {
            match provider {
                "openai" => {
                    self.state.provider_status = ProviderStatus::OpenAiReady {
                        model: model.to_string(),
                    };
                }
                _ => {
                    self.state.provider_status = ProviderStatus::OllamaOnline {
                        model: model.to_string(),
                    };
                }
            }
        }
        let _ = self.refresh_provider_status();
        tracing::info!(
            thread_id,
            provider,
            model,
            "ronin shell selected thread model"
        );
        Ok(())
    }

    /// Lists available models from configured providers (best-effort).
    ///
    /// Returns `(provider_id, model_names)` pairs. Providers that are offline
    /// or unconfigured are omitted rather than failing the whole list.
    pub fn list_available_provider_models(&self) -> Result<Vec<(String, Vec<String>)>> {
        let config = self.session.load_config()?;
        let mut out = Vec::new();

        let ollama = HttpOllamaProvider::new(&config.ollama.base_url);
        if let Ok(models) = ollama.list_models() {
            if !models.is_empty() {
                out.push(("ollama".to_string(), models));
            }
        }

        let base_url = config
            .openai
            .as_ref()
            .and_then(|o| o.base_url.clone())
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        let openai = OpenAiCompatibleProvider::new(&base_url);
        if let Ok(models) = openai.list_models() {
            if !models.is_empty() {
                out.push(("openai".to_string(), models));
            }
        }

        Ok(out)
    }

    /// Reloads thread list (and selected messages) from the session database.
    pub fn reload_threads(&mut self) -> Result<()> {
        self.state.threads = self.session.list_threads()?;
        if let Some(id) = self.state.selected_thread_id.clone() {
            if self.state.threads.iter().any(|t| t.id == id) {
                self.state.messages = self.session.list_messages(&id).ok();
            } else {
                self.state.selected_thread_id = self.state.threads.first().map(|t| t.id.clone());
                if let Some(id) = self.state.selected_thread_id.clone() {
                    self.state.messages = self.session.list_messages(&id).ok();
                } else {
                    self.state.messages = None;
                }
            }
        }
        Ok(())
    }

    /// Resolves the default provider id and model from global config (no thread required).
    pub fn resolve_default_provider_and_model(&self) -> Result<(String, String)> {
        let config = self.session.load_config()?;
        let provider = config
            .general
            .default_provider
            .clone()
            .unwrap_or_else(|| "ollama".to_string());
        let model = config
            .general
            .default_model
            .clone()
            .unwrap_or_else(|| "llama3.2".to_string());
        Ok((provider, model))
    }

    /// Returns current shell state.
    pub fn state(&self) -> &ShellState {
        &self.state
    }

    /// Returns a reference to the underlying session.
    pub fn session(&self) -> &RoninSession {
        &self.session
    }

    /// Preferred sidebar width in pixels (clamped).
    pub fn sidebar_width(&self) -> f32 {
        let width = self
            .session
            .load_config()
            .map(|c| c.ui.sidebar_width)
            .unwrap_or(ronin_core::SIDEBAR_WIDTH_DEFAULT);
        ronin_core::clamp_sidebar_width(width)
    }

    /// Current persona / system-prompt customization from config.
    pub fn persona(&self) -> ronin_core::PersonaConfig {
        self.session
            .load_config()
            .map(|c| c.persona)
            .unwrap_or_default()
    }

    /// System prompt that will be sent on the next chat request (inspectable).
    pub fn effective_system_prompt(&self) -> String {
        ronin_core::effective_system_prompt(&self.persona())
    }

    /// Sets and persists persona mode + custom text.
    pub fn set_persona(&mut self, mode: ronin_core::PersonaMode, text: &str) -> Result<()> {
        let mut config = self.session.load_config()?;
        config.persona.mode = mode;
        config.persona.text = text.to_string();
        self.session.save_config(&config)?;
        Ok(())
    }

    /// Exports non-secret provider settings to a TOML file.
    pub fn export_provider_config_to_file(&self, path: &std::path::Path) -> Result<()> {
        self.session
            .export_provider_config_to_file(path)
            .map_err(Into::into)
    }

    /// Imports provider settings from a TOML file (validates; preserves persona/theme/UI).
    pub fn import_provider_config_from_file(&self, path: &std::path::Path) -> Result<()> {
        self.session
            .import_provider_config_from_file(path)
            .map_err(Into::into)
    }

    /// Whether the sidebar is collapsed.
    pub fn sidebar_collapsed(&self) -> bool {
        self.session
            .load_config()
            .map(|c| c.ui.sidebar_collapsed)
            .unwrap_or(false)
    }

    /// Sets and persists the preferred sidebar width (clamped).
    pub fn set_sidebar_width(&mut self, width: f32) -> Result<()> {
        let mut config = self.session.load_config()?;
        config.ui.sidebar_width = ronin_core::clamp_sidebar_width(width);
        self.session.save_config(&config)?;
        Ok(())
    }

    /// Sets and persists whether the sidebar is collapsed.
    pub fn set_sidebar_collapsed(&mut self, collapsed: bool) -> Result<()> {
        let mut config = self.session.load_config()?;
        config.ui.sidebar_collapsed = collapsed;
        self.session.save_config(&config)?;
        Ok(())
    }

    /// Toggles sidebar collapse and persists the new state. Returns the new collapsed flag.
    pub fn toggle_sidebar_collapsed(&mut self) -> Result<bool> {
        let next = !self.sidebar_collapsed();
        self.set_sidebar_collapsed(next)?;
        Ok(next)
    }

    /// Resolves the provider and model for a given thread ID, falling back to global config defaults.
    pub fn resolve_thread_provider_and_model(&self, thread_id: &str) -> Result<(String, String)> {
        let thread = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .ok_or_else(|| RoninAppError::ThreadNotLoaded {
                thread_id: thread_id.to_string(),
            })?;

        let config = self.session.load_config()?;

        let provider = thread
            .provider
            .clone()
            .or_else(|| config.general.default_provider.clone())
            .unwrap_or_else(|| "ollama".to_string());

        let model = thread
            .model
            .clone()
            .or_else(|| config.general.default_model.clone())
            .unwrap_or_else(|| "llama3.2".to_string());

        Ok((provider, model))
    }

    /// Binds to and checks health of the active provider for the selected thread, updating the status.
    pub fn refresh_provider_status(&mut self) -> Result<()> {
        let thread_id = match self.state.selected_thread_id.as_deref() {
            Some(id) => id,
            None => {
                self.state.provider_status = ProviderStatus::NotConfigured;
                return Ok(());
            }
        };

        let (provider_name, _) = self.resolve_thread_provider_and_model(thread_id)?;
        let config = self.session.load_config()?;

        self.state.provider_status = if provider_name == "openai" {
            let base_url = config
                .openai
                .as_ref()
                .and_then(|o| o.base_url.clone())
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
            let provider = ronin_core::OpenAiCompatibleProvider::new(&base_url);
            probe_provider_status(&provider, &self.session)
        } else {
            let provider = ronin_core::HttpOllamaProvider::new(&config.ollama.base_url);
            probe_provider_status(&provider, &self.session)
        };

        Ok(())
    }

    /// Tests connectivity (and auth, when applicable) for the active thread's provider.
    ///
    /// Stores the result on [`ShellState::connection_test`] and returns it.
    pub fn test_connection(&mut self) -> Result<ConnectionTestResult> {
        let thread_id = match self.state.selected_thread_id.as_deref() {
            Some(id) => id,
            None => {
                let result = ConnectionTestResult::Failure {
                    message: "No thread selected. Open a chat, then test the connection."
                        .to_string(),
                };
                self.state.connection_test = Some(result.clone());
                return Ok(result);
            }
        };

        let (provider_name, _) = self.resolve_thread_provider_and_model(thread_id)?;
        let config = self.session.load_config()?;

        let result = if provider_name == "openai" {
            let base_url = config
                .openai
                .as_ref()
                .and_then(|o| o.base_url.clone())
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
            let provider = OpenAiCompatibleProvider::new(&base_url);
            self.record_connection_test(&provider)
        } else {
            let provider = HttpOllamaProvider::new(&config.ollama.base_url);
            self.record_connection_test(&provider)
        };

        Ok(result)
    }

    /// Runs a connection test against an injected provider (for tests and UI).
    pub fn record_connection_test(
        &mut self,
        provider: &impl OllamaProvider,
    ) -> ConnectionTestResult {
        let result = run_connection_test(provider);
        self.state.connection_test = Some(result.clone());
        result
    }

    /// Returns whether the currently selected thread has an active generation.
    pub fn is_generation_active(&self) -> bool {
        self.state
            .selected_thread_id
            .as_deref()
            .is_some_and(|id| self.is_thread_generating(id))
    }

    /// Returns whether `thread_id` currently has an in-flight generation.
    pub fn is_thread_generating(&self, thread_id: &str) -> bool {
        self.active_generations.contains_key(thread_id)
    }

    /// Thread ids that currently have an active generation (for sidebar indicators).
    pub fn active_generating_thread_ids(&self) -> Vec<String> {
        self.active_generations.keys().cloned().collect()
    }

    /// Persists a user message and derives a thread title when needed.
    fn persist_user_message(
        &mut self,
        thread_id: &str,
        content: &str,
        attachments: &[ContextAttachmentDraft],
    ) -> Result<()> {
        let content = visible_message_or_attachment_placeholder(content, attachments);
        let user_msg = self
            .session
            .create_message(thread_id, MessageRole::User, content)?;
        persist_context_attachments(&self.session, &user_msg.id, attachments)?;
        self.derive_title_if_new_chat(thread_id, content)?;
        Ok(())
    }

    /// Derives and persists a title when the thread is still called `New Chat`.
    fn derive_title_if_new_chat(&mut self, thread_id: &str, content: &str) -> Result<()> {
        let current_title = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .map(|t| t.title.as_str());
        if current_title == Some("New Chat") {
            let derived = derive_thread_title(content);
            self.session.update_thread_title(thread_id, &derived)?;
            if let Some(thread) = self.state.threads.iter_mut().find(|t| t.id == thread_id) {
                thread.title = derived;
            }
        }
        Ok(())
    }

    /// Renames a thread and persists immediately. Marks the title as manual.
    pub fn rename_thread(&mut self, thread_id: &str, title: &str) -> Result<()> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err(RoninAppError::InvalidThreadTitle);
        }
        self.session.update_thread_title(thread_id, trimmed)?;
        if let Some(thread) = self.state.threads.iter_mut().find(|t| t.id == thread_id) {
            thread.title = trimmed.to_string();
        }
        self.manual_titles.insert(thread_id.to_string());
        Ok(())
    }

    /// Returns whether `thread_id` was explicitly renamed by the user this session.
    pub fn is_manual_title(&self, thread_id: &str) -> bool {
        self.manual_titles.contains(thread_id)
    }

    /// Requests a lightweight model title and applies it when auto-titling allows.
    ///
    /// Returns `true` when a new title was persisted. On failure or when skipped,
    /// leaves the existing (usually first-line-derived) title in place.
    pub fn apply_model_generated_title(
        &mut self,
        thread_id: &str,
        model: &str,
        provider: &dyn ChatProvider,
    ) -> Result<bool> {
        let prepared = self.prepare_auto_title(thread_id)?;
        let Some((first_user, first_assistant)) = prepared else {
            return Ok(false);
        };

        self.state.title_generating_thread_id = Some(thread_id.to_string());
        let request = build_title_generation_request(model, &first_user, &first_assistant);
        let result = (|| -> Result<bool> {
            let stream = provider.stream_chat(&request)?;
            let raw = collect_streamed_title(stream);
            let Some(title) = sanitize_generated_title(&raw) else {
                return Ok(false);
            };
            self.persist_auto_title(thread_id, &first_user, &title)
        })();
        self.state.title_generating_thread_id = None;
        result
    }

    /// Starts background title generation so the UI can show an in-flight hint.
    ///
    /// Returns the provider when generation was skipped so the caller can keep it.
    /// Returns `None` when the provider was moved onto the background thread.
    pub fn begin_model_title_generation(
        &mut self,
        thread_id: &str,
        model: &str,
        provider: Box<dyn ChatProvider + Send>,
    ) -> Result<Option<Box<dyn ChatProvider + Send>>> {
        if self.title_gen_rx.is_some() || self.state.title_generating_thread_id.is_some() {
            return Ok(Some(provider));
        }
        let prepared = self.prepare_auto_title(thread_id)?;
        let Some((first_user, first_assistant)) = prepared else {
            return Ok(Some(provider));
        };

        let thread_id_owned = thread_id.to_string();
        let model_owned = model.to_string();
        self.state.title_generating_thread_id = Some(thread_id_owned.clone());
        let (tx, rx) = mpsc::channel();
        self.title_gen_rx = Some(rx);

        std::thread::spawn(move || {
            let request =
                build_title_generation_request(&model_owned, &first_user, &first_assistant);
            let title = match provider.stream_chat(&request) {
                Ok(stream) => sanitize_generated_title(&collect_streamed_title(stream)),
                Err(e) => {
                    let _ = tx.send(TitleGenResult {
                        thread_id: thread_id_owned,
                        title: None,
                        error: Some(e.to_string()),
                    });
                    return;
                }
            };
            let _ = tx.send(TitleGenResult {
                thread_id: thread_id_owned,
                title,
                error: None,
            });
        });
        Ok(None)
    }

    /// Polls a background title-generation request. Returns `true` while in flight.
    pub fn poll_title_generation(&mut self) -> bool {
        let rx = match self.title_gen_rx.as_ref() {
            Some(rx) => rx,
            None => return false,
        };
        match rx.try_recv() {
            Ok(result) => {
                self.title_gen_rx = None;
                self.state.title_generating_thread_id = None;
                if let Some(err) = result.error {
                    tracing::warn!(%err, "model title generation failed; keeping fallback");
                    return false;
                }
                if let Some(title) = result.title {
                    let first_user = self
                        .session
                        .list_messages(&result.thread_id)
                        .ok()
                        .and_then(|msgs| {
                            msgs.into_iter()
                                .find(|m| m.role == MessageRole::User)
                                .map(|m| m.content)
                        })
                        .unwrap_or_default();
                    match self.persist_auto_title(&result.thread_id, &first_user, &title) {
                        Ok(true) => {
                            tracing::info!(
                                thread_id = %result.thread_id,
                                "applied model-generated thread title"
                            );
                        }
                        Ok(false) => {}
                        Err(e) => tracing::warn!(%e, "failed to persist model title"),
                    }
                }
                false
            }
            Err(mpsc::TryRecvError::Empty) => true,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.title_gen_rx = None;
                self.state.title_generating_thread_id = None;
                false
            }
        }
    }

    fn prepare_auto_title(&self, thread_id: &str) -> Result<Option<(String, String)>> {
        let config = self.session.load_config()?;
        if !config.general.auto_title {
            return Ok(None);
        }

        let current_title = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "New Chat".to_string());

        let messages = self.session.list_messages(thread_id)?;
        let first_user = messages
            .iter()
            .find(|m| m.role == MessageRole::User)
            .map(|m| m.content.clone())
            .unwrap_or_default();
        let first_assistant = messages
            .iter()
            .find(|m| m.role == MessageRole::Assistant && m.status == MessageStatus::Complete)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        if first_user.is_empty() || first_assistant.is_empty() {
            return Ok(None);
        }

        let manual = self.is_manual_title(thread_id);
        if !may_apply_auto_title(&current_title, &first_user, manual) {
            return Ok(None);
        }

        Ok(Some((first_user, first_assistant)))
    }

    fn persist_auto_title(
        &mut self,
        thread_id: &str,
        first_user: &str,
        title: &str,
    ) -> Result<bool> {
        let current = self
            .state
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .map(|t| t.title.as_str())
            .unwrap_or("New Chat");
        if !may_apply_auto_title(current, first_user, self.is_manual_title(thread_id)) {
            return Ok(false);
        }
        self.session.update_thread_title(thread_id, title)?;
        if let Some(thread) = self.state.threads.iter_mut().find(|t| t.id == thread_id) {
            thread.title = title.to_string();
        }
        Ok(true)
    }

    /// Sends a user message in the selected thread.
    ///
    /// Persists the user message immediately. If the thread still has the
    /// default `New Chat` title, derives a title from the first non-empty
    /// prompt line (trimmed, whitespace-collapsed, truncated to ~60 chars).
    ///
    /// Reloads messages from the session into shell state so the UI reflects
    /// the new message.
    pub fn send_message(&mut self, thread_id: &str, content: &str) -> Result<()> {
        self.send_message_with_attachments(thread_id, content, &[])
    }

    /// Sends a user message and persists explicit attachment metadata without a provider.
    pub fn send_message_with_attachments(
        &mut self,
        thread_id: &str,
        content: &str,
        attachments: &[ContextAttachmentDraft],
    ) -> Result<()> {
        self.persist_user_message(thread_id, content, attachments)?;

        // Reload messages so the UI sees the newly persisted message.
        self.state.messages = self.session.list_messages(thread_id).ok();

        Ok(())
    }

    /// Begins a streaming provider response on a background thread.
    ///
    /// Persists the user message, derives a thread title, creates a streaming
    /// assistant placeholder, and spawns a background thread that calls the
    /// provider.  Returns immediately; the caller must poll [`RoninShell::poll_streaming`]
    /// to receive chunks and finalize the response.
    pub fn begin_streaming(
        &mut self,
        thread_id: &str,
        content: Option<&str>,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        self.begin_streaming_with_attachments(thread_id, content, &[], provider, model)
    }

    /// Begins a streaming provider response with explicit context attachments.
    pub fn begin_streaming_with_attachments(
        &mut self,
        thread_id: &str,
        content: Option<&str>,
        attachments: &[ContextAttachmentDraft],
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        if self.is_thread_generating(thread_id) {
            return Err(RoninAppError::GenerationInProgress);
        }

        if let Some(user_content) = content {
            self.persist_user_message(thread_id, user_content, attachments)?;
        }

        let assistant_msg = self.session.create_streaming_message(thread_id, "")?;
        let assistant_msg_id = assistant_msg.id.clone();
        self.spawn_streaming(thread_id, &assistant_msg_id, provider, model, attachments)?;
        Ok(())
    }

    /// Polls for streaming updates from all in-flight generations.
    ///
    /// Drains available chunks (non-blocking). Updates in-memory messages for the
    /// currently selected thread. Background threads still persist via their own
    /// session handles. Returns `true` if the **selected** thread is still
    /// streaming (more chunks may arrive).
    pub fn poll_streaming(&mut self) -> bool {
        let selected = self.state.selected_thread_id.clone();
        let thread_ids: Vec<String> = self.active_generations.keys().cloned().collect();

        for thread_id in thread_ids {
            let finished = self.poll_one_generation(&thread_id);
            if finished {
                self.active_generations.remove(&thread_id);
            }
        }

        selected
            .as_deref()
            .is_some_and(|id| self.is_thread_generating(id))
    }

    /// Polls one thread's generation. Returns `true` when that generation is finished.
    fn poll_one_generation(&mut self, thread_id: &str) -> bool {
        let is_selected = self.state.selected_thread_id.as_deref() == Some(thread_id);

        let Some(gen) = self.active_generations.get(thread_id) else {
            return true;
        };
        let msg_id = gen.streaming_msg_id.clone();

        loop {
            let recv = {
                let Some(gen) = self.active_generations.get(thread_id) else {
                    return true;
                };
                gen.streaming_rx.try_recv()
            };

            match recv {
                Ok(StreamUpdate::Chunk(delta)) => {
                    if let Some(gen) = self.active_generations.get_mut(thread_id) {
                        gen.accumulated.push_str(&delta);
                    }
                    if is_selected {
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                                msg.content.push_str(&delta);
                            }
                        }
                    }
                }
                Ok(StreamUpdate::NextTurn {
                    new_assistant_msg_id,
                }) => {
                    if is_selected {
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                                msg.status = MessageStatus::Complete;
                            }
                        }
                        if let Ok(all_msgs) = self.session.list_messages(thread_id) {
                            self.state.messages = Some(all_msgs);
                        }
                    }
                    if let Some(gen) = self.active_generations.get_mut(thread_id) {
                        gen.streaming_msg_id = new_assistant_msg_id;
                    }
                    return false;
                }
                Ok(StreamUpdate::Done(_final_content)) => {
                    if is_selected {
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                                msg.status = MessageStatus::Complete;
                            }
                        }
                    }
                    self.enqueue_generation_notification(
                        GenerationNotifyKind::Completed,
                        thread_id,
                        None,
                    );
                    return true;
                }
                Ok(StreamUpdate::Error(e)) => {
                    let provider = self
                        .resolve_thread_provider_and_model(thread_id)
                        .map(|(p, _)| p)
                        .unwrap_or_else(|_| "ollama".to_string());
                    let friendly = format_provider_error(&provider, &e);
                    if is_selected {
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                                msg.status = MessageStatus::Failed;
                                msg.error_message = Some(friendly.clone());
                            }
                        }
                    }
                    tracing::error!(%thread_id, "provider stream error: {e}");
                    self.enqueue_generation_notification(
                        GenerationNotifyKind::Failed,
                        thread_id,
                        Some(&friendly),
                    );
                    return true;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    return false;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return true;
                }
            }
        }
    }

    /// Cancels the active streaming generation in the currently viewed thread only.
    pub fn cancel_streaming(&mut self) -> Result<()> {
        let Some(thread_id) = self.state.selected_thread_id.clone() else {
            return Ok(());
        };
        self.cancel_streaming_for_thread(&thread_id)
    }

    /// Cancels an active generation for a specific thread.
    pub fn cancel_streaming_for_thread(&mut self, thread_id: &str) -> Result<()> {
        let Some(gen) = self.active_generations.remove(thread_id) else {
            return Ok(());
        };

        let msg_id = gen.streaming_msg_id;
        let is_selected = self.state.selected_thread_id.as_deref() == Some(thread_id);

        let current_content = if is_selected {
            self.state
                .messages
                .as_ref()
                .and_then(|msgs| msgs.iter().find(|m| m.id == msg_id))
                .map(|m| m.content.clone())
                .unwrap_or_default()
        } else {
            self.session
                .list_messages(thread_id)
                .ok()
                .and_then(|msgs| msgs.into_iter().find(|m| m.id == msg_id))
                .map(|m| m.content)
                .unwrap_or_default()
        };

        self.session.cancel_message(&msg_id, &current_content)?;

        if is_selected {
            if let Some(ref mut msgs) = self.state.messages {
                if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                    msg.status = MessageStatus::Cancelled;
                }
            }
        }

        tracing::info!(%thread_id, message_id = %msg_id, "streaming response cancelled by user");
        Ok(())
    }

    /// Sends a user message and streams an assistant response via a provider.
    ///
    /// Persists the user message immediately, inserts a `streaming` assistant
    /// placeholder, calls the provider, accumulates chunks into the assistant
    /// message, and marks it `complete` on success.
    pub fn send_message_with_provider(
        &mut self,
        thread_id: &str,
        content: &str,
        provider: &dyn ChatProvider,
        model: &str,
    ) -> Result<()> {
        self.send_message_with_provider_and_attachments(thread_id, content, &[], provider, model)
    }

    /// Sends a user message with explicit context attachments and streams a provider response.
    pub fn send_message_with_provider_and_attachments(
        &mut self,
        thread_id: &str,
        content: &str,
        attachments: &[ContextAttachmentDraft],
        provider: &dyn ChatProvider,
        model: &str,
    ) -> Result<()> {
        if self.is_thread_generating(thread_id) {
            return Err(RoninAppError::GenerationInProgress);
        }

        self.send_message_with_provider_inner(thread_id, content, attachments, provider, model)
    }

    /// Retries a failed assistant message.
    ///
    /// Finds the preceding user message, creates a new user message with the same content,
    /// and begins streaming a new response. The original failed message is preserved.
    pub fn retry_message(
        &mut self,
        message_id: &str,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        let thread_id = self.state.selected_thread_id.clone().ok_or_else(|| {
            RoninAppError::ThreadNotLoaded {
                thread_id: "".to_string(),
            }
        })?;

        if self.is_thread_generating(&thread_id) {
            return Err(RoninAppError::GenerationInProgress);
        }

        let msgs = self.session.list_messages(&thread_id)?;

        let failed_idx = msgs
            .iter()
            .position(|m| m.id == message_id)
            .ok_or(RoninAppError::InvalidMessage)?;

        let user_content = msgs[..failed_idx]
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::User)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        if user_content.is_empty() {
            return Err(RoninAppError::InvalidMessage);
        }

        self.begin_streaming(&thread_id, Some(&user_content), provider, model)
    }

    /// Regenerates the last assistant message as a new sibling branch.
    ///
    /// Preserves the previous assistant response and streams a new one under
    /// the same parent user message.
    pub fn regenerate_last_assistant(
        &mut self,
        thread_id: &str,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        if self.is_thread_generating(thread_id) {
            return Err(RoninAppError::GenerationInProgress);
        }

        let msgs = self.session.list_messages(thread_id)?;

        let last_assistant_idx = match msgs.iter().rposition(|m| m.role == MessageRole::Assistant) {
            Some(idx) => idx,
            None => return Ok(()),
        };

        let last_assistant = &msgs[last_assistant_idx];
        if last_assistant.status == MessageStatus::Streaming {
            return Err(RoninAppError::GenerationInProgress);
        }

        let user = msgs[..last_assistant_idx]
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::User)
            .ok_or(RoninAppError::InvalidMessage)?;

        let parent_id = last_assistant
            .parent_id
            .clone()
            .unwrap_or_else(|| user.id.clone());

        let assistant_msg =
            self.session
                .create_streaming_message_with_parent(thread_id, "", Some(&parent_id))?;
        self.spawn_streaming(thread_id, &assistant_msg.id, provider, model, &[])?;
        Ok(())
    }

    /// Edits a sent user message by forking a sibling branch and regenerating.
    pub fn edit_user_message_and_regenerate(
        &mut self,
        message_id: &str,
        new_content: &str,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        let trimmed = new_content.trim();
        if trimmed.is_empty() {
            return Err(RoninAppError::InvalidMessage);
        }

        let all = self.session.list_all_messages(
            self.state
                .selected_thread_id
                .as_deref()
                .ok_or(RoninAppError::InvalidMessage)?,
        )?;
        // Prefer thread id from the message itself.
        let original = all
            .iter()
            .find(|m| m.id == message_id && m.role == MessageRole::User)
            .ok_or(RoninAppError::InvalidMessage)?;
        let thread_id = original.thread_id.clone();
        let parent_id = original.parent_id.clone();

        if self.is_thread_generating(&thread_id) {
            return Err(RoninAppError::GenerationInProgress);
        }

        self.session.create_message_with_parent(
            &thread_id,
            MessageRole::User,
            trimmed,
            parent_id.as_deref(),
        )?;
        self.begin_streaming(&thread_id, None, provider, model)
    }

    /// Returns sibling messages at the fork containing `message_id` (all branches).
    pub fn branch_siblings(&self, thread_id: &str, message_id: &str) -> Result<Vec<Message>> {
        let all = self.session.list_all_messages(thread_id)?;
        let nodes: Vec<MessageNode> = all
            .iter()
            .map(|m| MessageNode {
                id: m.id.clone(),
                parent_id: m.parent_id.clone(),
            })
            .collect();
        let Some(nav) = sibling_branch_nav(&nodes, message_id) else {
            return Ok(vec![all
                .iter()
                .find(|m| m.id == message_id)
                .cloned()
                .ok_or(RoninAppError::InvalidMessage)?]);
        };
        Ok(nav
            .sibling_ids
            .iter()
            .filter_map(|id| all.iter().find(|m| m.id == *id).cloned())
            .collect())
    }

    /// Switches the active conversation path to the branch containing `message_id`.
    pub fn switch_branch(&mut self, thread_id: &str, message_id: &str) -> Result<()> {
        let all = self.session.list_all_messages(thread_id)?;
        if !all.iter().any(|m| m.id == message_id) {
            return Err(RoninAppError::InvalidMessage);
        }
        let nodes: Vec<MessageNode> = all
            .iter()
            .map(|m| MessageNode {
                id: m.id.clone(),
                parent_id: m.parent_id.clone(),
            })
            .collect();
        let leaf = leaf_under_root(&nodes, message_id);
        self.session.set_active_leaf(thread_id, &leaf)?;
        self.state.threads = self.session.list_threads()?;
        self.state.messages = Some(self.session.list_messages(thread_id)?);
        Ok(())
    }

    fn spawn_streaming(
        &mut self,
        thread_id: &str,
        assistant_msg_id: &str,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
        attachments: &[ContextAttachmentDraft],
    ) -> Result<()> {
        let all_msgs = self.session.list_messages(thread_id)?;
        let request_context = merge_request_context(
            profile_memory_context_block(&self.session),
            attachment_context_block(attachments),
        );
        let system_prompt = self.effective_system_prompt();
        let capped = build_capped_chat_request(
            model,
            &all_msgs,
            assistant_msg_id,
            &system_prompt,
            request_context.as_deref(),
        );
        if capped.truncated {
            self.state.truncation_notice = true;
        }
        self.state.messages = Some(all_msgs);

        let (tx, rx) = mpsc::channel();
        let session_clone = self.session.clone_session()?;
        let thread_id_owned = thread_id.to_string();
        let model_owned = model.to_string();
        let assistant_msg_id_owned = assistant_msg_id.to_string();

        std::thread::spawn(move || {
            run_streaming_turns(
                &session_clone,
                &tx,
                provider.as_ref(),
                capped.request,
                &thread_id_owned,
                &model_owned,
                assistant_msg_id_owned,
                system_prompt,
                request_context,
            );
        });

        self.active_generations.insert(
            thread_id.to_string(),
            ActiveGeneration {
                streaming_rx: rx,
                streaming_msg_id: assistant_msg_id.to_string(),
                accumulated: String::new(),
            },
        );
        Ok(())
    }

    fn send_message_with_provider_inner(
        &mut self,
        thread_id: &str,
        content: &str,
        attachments: &[ContextAttachmentDraft],
        provider: &dyn ChatProvider,
        model: &str,
    ) -> Result<()> {
        self.persist_user_message(thread_id, content, attachments)?;

        let assistant_msg = self.session.create_streaming_message(thread_id, "")?;
        let all_msgs = self.session.list_messages(thread_id)?;

        let request_context = merge_request_context(
            profile_memory_context_block(&self.session),
            attachment_context_block(attachments),
        );
        let system_prompt = self.effective_system_prompt();
        let capped = build_capped_chat_request(
            model,
            &all_msgs,
            &assistant_msg.id,
            &system_prompt,
            request_context.as_deref(),
        );
        if capped.truncated {
            self.state.truncation_notice = true;
        }

        self.state.messages = Some(all_msgs);

        // Stream response loop supporting tool calling.
        let mut current_request = capped.request;
        let mut loop_count = 0;
        const MAX_TOOL_LOOPS: usize = 5;
        let mut assistant_msg_id = assistant_msg.id.clone();

        loop {
            let mut accumulated = String::new();
            let stream = provider.stream_chat(&current_request)?;
            for event in stream {
                match event {
                    ChatStreamEvent::Chunk(chunk) => {
                        accumulated.push_str(&chunk);
                        let _ = self
                            .session
                            .complete_message(&assistant_msg_id, &accumulated);
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg_id) {
                                msg.content = accumulated.clone();
                            }
                        }
                    }
                    ChatStreamEvent::Error(err) => {
                        let provider_name = self
                            .resolve_thread_provider_and_model(thread_id)
                            .map(|(p, _)| p)
                            .unwrap_or_else(|_| "ollama".to_string());
                        let friendly = format_provider_error(&provider_name, &err);
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg_id) {
                                msg.status = MessageStatus::Failed;
                                msg.error_message = Some(friendly.clone());
                            }
                        }
                        return Err(RoninAppError::Session(RoninError::Provider(friendly)));
                    }
                }
            }

            self.session
                .complete_message(&assistant_msg_id, &accumulated)?;
            if let Some(ref mut msgs) = self.state.messages {
                if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg_id) {
                    msg.content = accumulated.clone();
                }
            }

            if loop_count < MAX_TOOL_LOOPS {
                if let Some(tool_result) = next_tool_result(&self.session, &accumulated) {
                    loop_count += 1;

                    self.session
                        .create_message(thread_id, MessageRole::System, &tool_result)?;

                    let new_assistant_msg = self.session.create_streaming_message(thread_id, "")?;
                    assistant_msg_id = new_assistant_msg.id.clone();

                    let all_msgs = self.session.list_messages(thread_id)?;
                    let capped = build_capped_chat_request(
                        model,
                        &all_msgs,
                        &assistant_msg_id,
                        &system_prompt,
                        request_context.as_deref(),
                    );
                    current_request = capped.request;
                    self.state.messages = Some(all_msgs);

                    continue;
                }
            }

            // Completed final turn
            if let Some(ref mut msgs) = self.state.messages {
                if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg_id) {
                    msg.status = MessageStatus::Complete;
                }
            }
            break;
        }

        tracing::info!(
            thread_id,
            assistant_msg_id = %assistant_msg.id,
            "assistant response complete"
        );
        Ok(())
    }

    /// Creates a new memory with the given title and content.
    pub fn create_memory(&self, title: &str, content: &str) -> Result<ronin_core::Memory> {
        self.session
            .create_memory(title, content)
            .map_err(Into::into)
    }

    /// Creates a profile-group memory (auto-injected when enabled).
    pub fn create_profile_memory(&self, title: &str, content: &str) -> Result<ronin_core::Memory> {
        self.session
            .create_profile_memory(title, content)
            .map_err(Into::into)
    }

    /// Lists all memories.
    pub fn list_memories(&self) -> Result<Vec<ronin_core::Memory>> {
        self.session.list_memories().map_err(Into::into)
    }

    /// Sets whether a memory is enabled for provider context.
    pub fn set_memory_enabled(&self, id: &ronin_core::MemoryId, enabled: bool) -> Result<()> {
        self.session
            .set_memory_enabled(id, enabled)
            .map_err(Into::into)
    }

    /// Sets whether a memory belongs to the user profile group.
    pub fn set_memory_profile(&self, id: &ronin_core::MemoryId, is_profile: bool) -> Result<()> {
        self.session
            .set_memory_profile(id, is_profile)
            .map_err(Into::into)
    }

    /// Deletes a memory by ID.
    pub fn delete_memory(&self, id: &ronin_core::MemoryId) -> Result<()> {
        self.session.delete_memory(id).map_err(Into::into)
    }

    /// Updates a memory by ID.
    pub fn update_memory(
        &self,
        id: &ronin_core::MemoryId,
        title: &str,
        content: &str,
    ) -> Result<()> {
        self.session
            .update_memory(id, title, content)
            .map_err(Into::into)
    }

    /// Creates a new artifact.
    pub fn create_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
    ) -> Result<ronin_core::Artifact> {
        self.session
            .create_artifact(thread_id, message_id, title, content)
            .map_err(Into::into)
    }

    /// Creates a code-snippet artifact with fence language metadata.
    pub fn create_snippet_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
        language: &str,
    ) -> Result<ronin_core::Artifact> {
        self.session
            .create_snippet_artifact(thread_id, message_id, title, content, language)
            .map_err(Into::into)
    }

    /// Lists all artifacts across all threads, newest first.
    pub fn list_all_artifacts(&self) -> Result<Vec<ronin_core::Artifact>> {
        self.session.list_all_artifacts().map_err(Into::into)
    }

    /// Deletes an artifact by ID.
    pub fn delete_artifact(&self, id: &ronin_core::ArtifactId) -> Result<()> {
        self.session.delete_artifact(id).map_err(Into::into)
    }

    /// Renames and/or edits an artifact's title and content.
    pub fn update_artifact(
        &self,
        id: &ronin_core::ArtifactId,
        title: &str,
        content: &str,
    ) -> Result<()> {
        self.session
            .update_artifact(id, title, content)
            .map_err(Into::into)
    }
}

/// Runs provider streaming turns on the background thread, executing tool
/// calls between turns, persisting debounced partial content, and reporting
/// progress through `tx`.
#[allow(clippy::too_many_arguments)]
fn run_streaming_turns(
    session: &RoninSession,
    tx: &mpsc::Sender<StreamUpdate>,
    provider: &(dyn ChatProvider + Send),
    initial_request: ronin_core::ChatRequest,
    thread_id: &str,
    model: &str,
    mut assistant_msg_id: String,
    system_prompt: String,
    attachment_context: Option<String>,
) {
    const MAX_TOOL_LOOPS: usize = 5;
    const DB_DEBOUNCE_INTERVAL: usize = 20;

    let mut current_request = initial_request;
    let mut loop_count = 0;

    loop {
        let stream = match provider.stream_chat(&current_request) {
            Ok(s) => s,
            Err(e) => {
                let _ = tx.send(StreamUpdate::Error(e.to_string()));
                return;
            }
        };

        let mut accumulated = String::new();
        let mut chunk_count: usize = 0;

        for event in stream {
            match event {
                ChatStreamEvent::Chunk(chunk) => {
                    accumulated.push_str(&chunk);
                    chunk_count += 1;

                    // Debounced DB persistence.
                    if chunk_count.is_multiple_of(DB_DEBOUNCE_INTERVAL) {
                        let _ = session.complete_message(&assistant_msg_id, &accumulated);
                    }

                    // Send delta to UI.
                    if tx.send(StreamUpdate::Chunk(chunk)).is_err() {
                        return; // Receiver dropped
                    }
                }
                ChatStreamEvent::Error(e) => {
                    let _ = session.fail_message(&assistant_msg_id, &accumulated, &e);
                    let _ = tx.send(StreamUpdate::Error(e));
                    return;
                }
            }
        }

        let _ = session.complete_message(&assistant_msg_id, &accumulated);

        if loop_count < MAX_TOOL_LOOPS {
            if let Some(tool_result) = next_tool_result(session, &accumulated) {
                loop_count += 1;

                if let Err(e) = session.create_message(thread_id, MessageRole::System, &tool_result)
                {
                    let _ = tx.send(StreamUpdate::Error(e.to_string()));
                    return;
                }

                let new_assistant_msg = match session.create_streaming_message(thread_id, "") {
                    Ok(m) => m,
                    Err(e) => {
                        let _ = tx.send(StreamUpdate::Error(e.to_string()));
                        return;
                    }
                };
                let new_assistant_msg_id = new_assistant_msg.id.clone();

                if tx
                    .send(StreamUpdate::NextTurn {
                        new_assistant_msg_id: new_assistant_msg_id.clone(),
                    })
                    .is_err()
                {
                    return;
                }

                assistant_msg_id = new_assistant_msg_id;

                let all_msgs = match session.list_messages(thread_id) {
                    Ok(msgs) => msgs,
                    Err(e) => {
                        let _ = tx.send(StreamUpdate::Error(e.to_string()));
                        return;
                    }
                };
                current_request = build_capped_chat_request(
                    model,
                    &all_msgs,
                    &assistant_msg_id,
                    &system_prompt,
                    attachment_context.as_deref(),
                )
                .request;

                continue;
            }
        }

        let _ = tx.send(StreamUpdate::Done(accumulated));
        break;
    }
}
