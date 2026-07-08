//! UI-facing shell controller backed by `RoninSession` behavior.

use std::sync::mpsc;

use ronin_core::{
    ChatProvider, ChatStreamEvent, ContextAttachmentDraft, Message, MessageRole, MessageStatus,
    RoninError, RoninPaths, RoninSession, Thread,
};

use crate::chat::{build_capped_chat_request, derive_thread_title};
use crate::error::{Result, RoninAppError};
use crate::status::{probe_provider_status, ProviderStatus};
use crate::tools::{execute_tool_call, parse_unexecuted_tool_call};

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
    /// Messages for the selected thread, if loaded.
    pub messages: Option<Vec<Message>>,
    /// Whether older messages were omitted due to context caps.
    pub truncation_notice: bool,
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
    generation_active: bool,
    /// Receiver for streaming updates while a provider response is in flight.
    streaming_rx: Option<mpsc::Receiver<StreamUpdate>>,
    /// ID of the streaming assistant message being built.
    streaming_msg_id: Option<String>,
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
                messages,
                truncation_notice: false,
            },
            generation_active: false,
            streaming_rx: None,
            streaming_msg_id: None,
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
        let _ = self.refresh_provider_status();
        tracing::info!(thread_id, "ronin shell selected thread");
        Ok(())
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

    /// Returns current shell state.
    pub fn state(&self) -> &ShellState {
        &self.state
    }

    /// Returns a reference to the underlying session.
    pub fn session(&self) -> &RoninSession {
        &self.session
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

    /// Returns whether an assistant generation is currently active.
    pub fn is_generation_active(&self) -> bool {
        self.generation_active
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
        if self.generation_active {
            return Err(RoninAppError::GenerationInProgress);
        }
        self.generation_active = true;

        if let Some(user_content) = content {
            self.persist_user_message(thread_id, user_content, attachments)?;
        }

        let assistant_msg = self.session.create_streaming_message(thread_id, "")?;
        let assistant_msg_id = assistant_msg.id.clone();

        let all_msgs = self.session.list_messages(thread_id)?;
        let attachment_context = attachment_context_block(attachments);
        let capped = build_capped_chat_request(
            model,
            &all_msgs,
            &assistant_msg_id,
            attachment_context.as_deref(),
        );
        if capped.truncated {
            self.state.truncation_notice = true;
        }

        self.state.messages = Some(all_msgs);

        // Spawn background thread for streaming.
        let (tx, rx) = mpsc::channel();
        let session_clone = self.session.clone_session()?;
        let thread_id_owned = thread_id.to_string();
        let model_owned = model.to_string();
        let assistant_msg_id_for_thread = assistant_msg_id.clone();

        std::thread::spawn(move || {
            run_streaming_turns(
                &session_clone,
                &tx,
                provider.as_ref(),
                capped.request,
                &thread_id_owned,
                &model_owned,
                assistant_msg_id_for_thread,
                attachment_context,
            );
        });

        self.streaming_rx = Some(rx);
        self.streaming_msg_id = Some(assistant_msg_id);
        Ok(())
    }

    /// Polls for streaming updates from the background provider thread.
    ///
    /// Drains all available chunks from the channel (non-blocking), updating
    /// the in-memory message state.  Returns `true` if streaming is still
    /// active (more chunks may arrive), `false` when done or errored.
    pub fn poll_streaming(&mut self) -> bool {
        let rx = match self.streaming_rx.as_ref() {
            Some(rx) => rx,
            None => return false,
        };

        let msg_id = match self.streaming_msg_id.as_ref() {
            Some(id) => id.clone(),
            None => {
                self.streaming_rx = None;
                self.generation_active = false;
                return false;
            }
        };

        // Drain all available chunks this frame. Each chunk is a single-token
        // delta from the provider. Draining ensures visible throughput matches
        // Ollama's actual token rate regardless of GPUI's repaint frequency.
        loop {
            match rx.try_recv() {
                Ok(StreamUpdate::Chunk(delta)) => {
                    if let Some(ref mut msgs) = self.state.messages {
                        if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                            msg.content.push_str(&delta);
                        }
                    }
                    // Continue draining — more chunks may be ready.
                }
                Ok(StreamUpdate::NextTurn {
                    new_assistant_msg_id,
                }) => {
                    if let Some(ref mut msgs) = self.state.messages {
                        if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                            msg.status = MessageStatus::Complete;
                        }
                    }
                    if let Some(thread_id) = self.state.selected_thread_id.as_deref() {
                        if let Ok(all_msgs) = self.session.list_messages(thread_id) {
                            self.state.messages = Some(all_msgs);
                        }
                    }
                    self.streaming_msg_id = Some(new_assistant_msg_id);
                    return true;
                }
                Ok(StreamUpdate::Done(_final_content)) => {
                    if let Some(ref mut msgs) = self.state.messages {
                        if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                            msg.status = MessageStatus::Complete;
                        }
                    }
                    self.streaming_rx = None;
                    self.streaming_msg_id = None;
                    self.generation_active = false;
                    return false;
                }
                Ok(StreamUpdate::Error(e)) => {
                    if let Some(ref mut msgs) = self.state.messages {
                        if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                            msg.status = MessageStatus::Failed;
                            msg.error_message = Some(e.clone());
                        }
                    }
                    tracing::error!("provider stream error: {e}");
                    self.streaming_rx = None;
                    self.streaming_msg_id = None;
                    self.generation_active = false;
                    return false;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No more chunks available right now, still streaming.
                    return true;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Sender dropped unexpectedly — finalize what we have.
                    self.streaming_rx = None;
                    self.streaming_msg_id = None;
                    self.generation_active = false;
                    return false;
                }
            }
        }
    }

    /// Cancels an active streaming generation.
    pub fn cancel_streaming(&mut self) -> Result<()> {
        if !self.generation_active {
            return Ok(());
        }

        if let Some(msg_id) = self.streaming_msg_id.take() {
            let current_content = self
                .state
                .messages
                .as_ref()
                .and_then(|msgs| msgs.iter().find(|m| m.id == msg_id))
                .map(|m| m.content.as_str())
                .unwrap_or_default();

            self.session.cancel_message(&msg_id, current_content)?;

            if let Some(ref mut msgs) = self.state.messages {
                if let Some(msg) = msgs.iter_mut().find(|m| m.id == msg_id) {
                    msg.status = MessageStatus::Cancelled;
                }
            }

            tracing::info!(message_id = %msg_id, "streaming response cancelled by user");
        }

        self.streaming_rx = None;
        self.generation_active = false;
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
        if self.generation_active {
            return Err(RoninAppError::GenerationInProgress);
        }
        self.generation_active = true;

        let result =
            self.send_message_with_provider_inner(thread_id, content, attachments, provider, model);
        self.generation_active = false;
        result
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
        if self.generation_active {
            return Err(RoninAppError::GenerationInProgress);
        }

        let thread_id = self.state.selected_thread_id.clone().ok_or_else(|| {
            RoninAppError::ThreadNotLoaded {
                thread_id: "".to_string(),
            }
        })?;

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

    /// Regenerates the last assistant message.
    ///
    /// Finds the last assistant message, marks it as cancelled, and re-sends
    /// the preceding user messages to stream a new response.
    pub fn regenerate_last_assistant(
        &mut self,
        thread_id: &str,
        provider: Box<dyn ChatProvider + Send>,
        model: &str,
    ) -> Result<()> {
        if self.generation_active {
            return Err(RoninAppError::GenerationInProgress);
        }

        let msgs = self.session.list_messages(thread_id)?;

        let last_assistant_idx = match msgs.iter().rposition(|m| m.role == MessageRole::Assistant) {
            Some(idx) => idx,
            None => return Ok(()), // no-op if no assistant message
        };

        let last_assistant = &msgs[last_assistant_idx];

        // Ensure we only regenerate complete or cancelled messages
        if last_assistant.status == MessageStatus::Streaming {
            return Err(RoninAppError::GenerationInProgress);
        }

        self.session.delete_message(&last_assistant.id)?;

        let user_content = msgs[..last_assistant_idx]
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::User)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        if user_content.is_empty() {
            return Err(RoninAppError::InvalidMessage);
        }

        self.begin_streaming(thread_id, None, provider, model)
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

        let attachment_context = attachment_context_block(attachments);
        let capped = build_capped_chat_request(
            model,
            &all_msgs,
            &assistant_msg.id,
            attachment_context.as_deref(),
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
                        if let Some(ref mut msgs) = self.state.messages {
                            if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg_id) {
                                msg.status = MessageStatus::Failed;
                                msg.error_message = Some(err.clone());
                            }
                        }
                        return Err(RoninAppError::Session(RoninError::Provider(err)));
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
                if let Some(tool_call) = parse_unexecuted_tool_call(&accumulated) {
                    loop_count += 1;
                    let tool_result = execute_tool_call(&self.session, &tool_call);

                    self.session
                        .create_message(thread_id, MessageRole::System, &tool_result)?;

                    let new_assistant_msg = self.session.create_streaming_message(thread_id, "")?;
                    assistant_msg_id = new_assistant_msg.id.clone();

                    let all_msgs = self.session.list_messages(thread_id)?;
                    let capped = build_capped_chat_request(
                        model,
                        &all_msgs,
                        &assistant_msg_id,
                        attachment_context.as_deref(),
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

    /// Lists all memories.
    pub fn list_memories(&self) -> Result<Vec<ronin_core::Memory>> {
        self.session.list_memories().map_err(Into::into)
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

    /// Lists all artifacts across all threads, newest first.
    pub fn list_all_artifacts(&self) -> Result<Vec<ronin_core::Artifact>> {
        self.session.list_all_artifacts().map_err(Into::into)
    }

    /// Deletes an artifact by ID.
    pub fn delete_artifact(&self, id: &ronin_core::ArtifactId) -> Result<()> {
        self.session.delete_artifact(id).map_err(Into::into)
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
            if let Some(tool_call) = parse_unexecuted_tool_call(&accumulated) {
                loop_count += 1;
                let tool_result = execute_tool_call(session, &tool_call);

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
