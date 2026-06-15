#![deny(missing_docs)]

//! UI-facing application shell state for Ronin.

use std::sync::mpsc;

use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, ContextAttachmentDraft, Message, MessageRole,
    MessageStatus, OllamaHealth, OllamaProvider, RoninError, RoninPaths, RoninSession, Thread,
};

/// Result type returned by `ronin_app` operations.
pub type Result<T> = std::result::Result<T, RoninAppError>;

/// Errors returned by Ronin's UI-facing app boundary.
#[derive(Debug, thiserror::Error)]
pub enum RoninAppError {
    /// Ronin session operation failed.
    #[error(transparent)]
    Session(#[from] RoninError),

    /// Requested thread is not loaded in shell state.
    #[error("thread {thread_id} is not loaded")]
    ThreadNotLoaded {
        /// Thread id requested by the UI.
        thread_id: String,
    },

    /// Generation is already in progress.
    #[error("generation in progress")]
    GenerationInProgress,

    /// Action cannot be performed on the target message.
    #[error("invalid message for action")]
    InvalidMessage,
}

/// Basic provider/model status shown in the sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderStatus {
    /// No provider or model is configured yet.
    NotConfigured,
    /// Ollama provider is selected but could not be reached.
    OllamaOffline,
    /// Ollama is reachable and a model is selected.
    OllamaOnline {
        /// Name of the selected model.
        model: String,
    },
    /// Ollama is reachable but no models are installed.
    OllamaNoModels,
}

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

/// Derives a display title from the first non-empty line of a prompt.
///
/// Trims whitespace, collapses repeated whitespace/newlines into single
/// spaces, and truncates to approximately 60 characters.
fn derive_thread_title(prompt: &str) -> String {
    let first_line = prompt.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    let collapsed: String = first_line.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.len() <= 60 {
        collapsed
    } else {
        let mut truncated = collapsed.chars().take(57).collect::<String>();
        truncated.push_str("...");
        truncated
    }
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
        provider: impl OllamaProvider,
    ) -> Result<Self> {
        let session = RoninSession::open(paths)?;
        let mut threads = session.list_threads()?;
        if threads.is_empty() {
            threads.push(session.create_thread()?);
            tracing::info!("ronin shell created initial thread");
        }
        let selected_thread_id = threads.first().map(|thread| thread.id.clone());

        let provider_status = match provider.check_health() {
            OllamaHealth::Online => match provider.list_models() {
                Ok(models) if !models.is_empty() => {
                    let saved = session.load_selected_model().unwrap_or(None);
                    let model = match saved {
                        Some(m) if models.contains(&m) => m,
                        _ => models[0].clone(),
                    };
                    let _ = session.save_selected_model(&model);
                    tracing::info!(
                        thread_count = threads.len(),
                        model_count = models.len(),
                        selected_model = %model,
                        "ronin shell state restored with ollama"
                    );
                    ProviderStatus::OllamaOnline { model }
                }
                _ => {
                    tracing::info!(
                        thread_count = threads.len(),
                        "ollama online but no models installed"
                    );
                    ProviderStatus::OllamaNoModels
                }
            },
            OllamaHealth::Offline => {
                tracing::info!(
                    thread_count = threads.len(),
                    "ollama not reachable — provider offline"
                );
                ProviderStatus::OllamaOffline
            }
        };

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
        tracing::info!(thread_id, "ronin shell selected thread");
        Ok(())
    }

    /// Selects a model and persists the choice to config.
    pub fn select_model(&mut self, model: &str) -> Result<()> {
        self.session.save_selected_model(model)?;
        self.state.provider_status = ProviderStatus::OllamaOnline {
            model: model.to_string(),
        };
        tracing::info!(model, "ronin shell selected model");
        Ok(())
    }

    /// Returns current shell state.
    pub fn state(&self) -> &ShellState {
        &self.state
    }

    /// Returns whether an assistant generation is currently active.
    pub fn is_generation_active(&self) -> bool {
        self.generation_active
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
        let content = visible_message_or_attachment_placeholder(content, attachments);
        let user_msg =
            self.session
                .create_message(thread_id, ronin_core::MessageRole::User, content)?;
        persist_context_attachments(&self.session, &user_msg.id, attachments)?;

        // Derive title if thread is still "New Chat".
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

        // Reload messages so the UI sees the newly persisted message.
        self.state.messages = self.session.list_messages(thread_id).ok();

        Ok(())
    }

    /// Begins a streaming provider response on a background thread.
    ///
    /// Persists the user message, derives a thread title, creates a streaming
    /// assistant placeholder, and spawns a background thread that calls the
    /// provider.  Returns immediately; the caller must poll [`poll_streaming`]
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

        // 1. Persist user message and derive title if provided
        if let Some(user_content) = content {
            let user_content = visible_message_or_attachment_placeholder(user_content, attachments);
            let user_msg =
                self.session
                    .create_message(thread_id, MessageRole::User, user_content)?;
            persist_context_attachments(&self.session, &user_msg.id, attachments)?;

            let current_title = self
                .state
                .threads
                .iter()
                .find(|t| t.id == thread_id)
                .map(|t| t.title.as_str());
            if current_title == Some("New Chat") {
                let derived = derive_thread_title(user_content);
                self.session.update_thread_title(thread_id, &derived)?;
                if let Some(thread) = self.state.threads.iter_mut().find(|t| t.id == thread_id) {
                    thread.title = derived;
                }
            }
        }

        // 3. Create streaming assistant placeholder
        let assistant_msg = self.session.create_streaming_message(thread_id, "")?;
        let assistant_msg_id = assistant_msg.id.clone();
        let assistant_msg_id_for_thread = assistant_msg_id.clone();

        // 4. Load existing messages from DB for context
        let all_msgs = self.session.list_messages(thread_id)?;

        // 5. Build chat request with context caps (same logic as send_message_with_provider_inner)
        const MAX_MESSAGES: usize = 40;
        const MAX_CHARS: usize = 80_000;

        let mut included = Vec::new();
        let mut total_chars = 0usize;
        for msg in all_msgs.iter().rev() {
            if msg.id == assistant_msg_id {
                continue;
            }
            if included.len() >= MAX_MESSAGES {
                self.state.truncation_notice = true;
                break;
            }
            let msg_chars = msg.content.chars().count();
            if total_chars + msg_chars > MAX_CHARS {
                self.state.truncation_notice = true;
                break;
            }
            total_chars += msg_chars;
            included.push(msg);
        }
        included.reverse();

        let mut chat_messages = vec![ronin_core::ChatMessage {
            role: "system".to_string(),
            content: ronin_core::RONIN_SYSTEM_PROMPT.to_string(),
        }];
        if let Some(context) = attachment_context_block(attachments) {
            if context.chars().count() > MAX_CHARS {
                self.state.truncation_notice = true;
            }
            chat_messages.push(ronin_core::ChatMessage {
                role: "system".to_string(),
                content: context,
            });
        }
        chat_messages.extend(included.into_iter().map(|m| ronin_core::ChatMessage {
            role: match m.role {
                MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "assistant".to_string(),
                MessageRole::System => "system".to_string(),
            },
            content: m.content.clone(),
        }));

        self.state.messages = Some(all_msgs);

        let request = ChatRequest {
            model: model.to_string(),
            messages: chat_messages,
            system_prompt: Some(ronin_core::RONIN_SYSTEM_PROMPT.to_string()),
        };

        // 6. Spawn background thread for streaming
        let (tx, rx) = mpsc::channel();
        let session_clone = self.session.clone_session()?;

        std::thread::spawn(move || {
            let stream = match provider.stream_chat(&request) {
                Ok(s) => s,
                Err(e) => {
                    let _ = tx.send(StreamUpdate::Error(e.to_string()));
                    return;
                }
            };

            let mut accumulated = String::new();
            let mut chunk_count: usize = 0;
            const DB_DEBOUNCE_INTERVAL: usize = 20;

            for event in stream {
                match event {
                    ChatStreamEvent::Chunk(chunk) => {
                        accumulated.push_str(&chunk);
                        chunk_count += 1;

                        // Debounced DB persistence — not every token.
                        if chunk_count.is_multiple_of(DB_DEBOUNCE_INTERVAL) {
                            let _ = session_clone
                                .complete_message(&assistant_msg_id_for_thread, &accumulated);
                        }

                        // Send delta (individual token) to UI.
                        if tx.send(StreamUpdate::Chunk(chunk)).is_err() {
                            return; // Receiver dropped
                        }
                    }
                    ChatStreamEvent::Error(e) => {
                        // Persist what we have before reporting error.
                        let _ = session_clone.fail_message(
                            &assistant_msg_id_for_thread,
                            &accumulated,
                            &e,
                        );
                        let _ = tx.send(StreamUpdate::Error(e));
                        return;
                    }
                }
            }

            // Final DB write with complete content.
            let _ = session_clone.complete_message(&assistant_msg_id_for_thread, &accumulated);
            let _ = tx.send(StreamUpdate::Done(accumulated));
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
        if self.generation_active {
            return Err(RoninAppError::GenerationInProgress);
        }
        self.generation_active = true;

        let result =
            self.send_message_with_provider_inner(thread_id, content, &[], provider, model);
        self.generation_active = false;
        result
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

        let mut user_content = String::new();
        for msg in msgs[..failed_idx].iter().rev() {
            if msg.role == MessageRole::User {
                user_content = msg.content.clone();
                break;
            }
        }

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

        // Find the last assistant message
        let last_assistant_idx = msgs.iter().rposition(|m| m.role == MessageRole::Assistant);

        let last_assistant_idx = match last_assistant_idx {
            Some(idx) => idx,
            None => return Ok(()), // no-op if no assistant message
        };

        let last_assistant = &msgs[last_assistant_idx];

        // Ensure we only regenerate complete or cancelled messages
        if last_assistant.status == MessageStatus::Streaming {
            return Err(RoninAppError::GenerationInProgress);
        }

        // Delete it
        self.session.delete_message(&last_assistant.id)?;

        // Find preceding user message
        let mut user_content = String::new();
        for msg in msgs[..last_assistant_idx].iter().rev() {
            if msg.role == MessageRole::User {
                user_content = msg.content.clone();
                break;
            }
        }

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
        // 1. Persist user message and attachment metadata.
        let content = visible_message_or_attachment_placeholder(content, attachments);
        let user_msg =
            self.session
                .create_message(thread_id, ronin_core::MessageRole::User, content)?;
        persist_context_attachments(&self.session, &user_msg.id, attachments)?;

        // 2. Derive title if still "New Chat"
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

        // 3. Create streaming assistant placeholder
        let assistant_msg = self.session.create_streaming_message(thread_id, "")?;

        // 4. Load existing messages from DB for context
        let all_msgs = self.session.list_messages(thread_id)?;

        // 5. Build chat request with context caps
        const MAX_MESSAGES: usize = 40;
        const MAX_CHARS: usize = 80_000;

        // Apply context caps: keep most recent messages within limits.
        let mut included = Vec::new();
        let mut total_chars = 0usize;
        for msg in all_msgs.iter().rev() {
            if msg.id == assistant_msg.id {
                continue;
            }
            if included.len() >= MAX_MESSAGES {
                self.state.truncation_notice = true;
                break;
            }
            let msg_chars = msg.content.chars().count();
            if total_chars + msg_chars > MAX_CHARS {
                self.state.truncation_notice = true;
                break;
            }
            total_chars += msg_chars;
            included.push(msg);
        }
        included.reverse();

        let mut chat_messages: Vec<ronin_core::ChatMessage> = vec![ronin_core::ChatMessage {
            role: "system".to_string(),
            content: ronin_core::RONIN_SYSTEM_PROMPT.to_string(),
        }];
        if let Some(context) = attachment_context_block(attachments) {
            if context.chars().count() > MAX_CHARS {
                self.state.truncation_notice = true;
            }
            chat_messages.push(ronin_core::ChatMessage {
                role: "system".to_string(),
                content: context,
            });
        }
        chat_messages.extend(included.into_iter().map(|m| ronin_core::ChatMessage {
            role: match m.role {
                MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "assistant".to_string(),
                MessageRole::System => "system".to_string(),
            },
            content: m.content.clone(),
        }));

        // 6. Update shell state with current messages
        self.state.messages = Some(all_msgs);

        let request = ChatRequest {
            model: model.to_string(),
            messages: chat_messages,
            system_prompt: Some(ronin_core::RONIN_SYSTEM_PROMPT.to_string()),
        };

        // 7. Stream response
        let mut accumulated = String::new();
        let stream = provider.stream_chat(&request)?;
        for event in stream {
            match event {
                ChatStreamEvent::Chunk(chunk) => {
                    accumulated.push_str(&chunk);
                    // TODO(#6): debounce DB updates, only update on batches
                    let _ = self
                        .session
                        .complete_message(&assistant_msg.id, &accumulated);
                    // Update in-memory state
                    if let Some(ref mut msgs) = self.state.messages {
                        if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg.id) {
                            msg.content.push_str(&chunk);
                        }
                    }
                }
                ChatStreamEvent::Error(e) => {
                    tracing::error!(thread_id, "provider stream error: {e}");
                    self.session
                        .complete_message(&assistant_msg.id, &accumulated)?;
                    return Err(RoninAppError::Session(RoninError::Provider(e)));
                }
            }
        }

        // 8. Finalize assistant message as complete
        self.session
            .complete_message(&assistant_msg.id, &accumulated)?;
        if let Some(ref mut msgs) = self.state.messages {
            if let Some(msg) = msgs.iter_mut().find(|m| m.id == assistant_msg.id) {
                msg.content = accumulated;
                msg.status = MessageStatus::Complete;
            }
        }

        tracing::info!(
            thread_id,
            assistant_msg_id = %assistant_msg.id,
            "assistant response complete"
        );
        Ok(())
    }
}
