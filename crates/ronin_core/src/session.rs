//! Open Ronin application session backed by local filesystem state.

use std::fs;

use ronin_db::{
    default_log_dir, init_tracing_with, FileLogOptions, RoninDb, DEFAULT_MAX_LOG_FILE_BYTES,
};

use crate::config::{LoggingConfig, RoninConfig};
use crate::domain::{
    Artifact, ArtifactId, Attachment, AttachmentId, AttachmentKind, Memory, MemoryId, Message,
    MessageRole, RoninPaths, Thread,
};
use crate::error::{Result, RoninError};

/// Open Ronin application session backed by local filesystem state.
pub struct RoninSession {
    db: RoninDb,
    paths: RoninPaths,
}

impl RoninSession {
    /// Opens a Ronin session against the provided paths.
    ///
    /// Creates the config and data directories when they do not already exist,
    /// opens `ronin.db` in the data directory, and applies pending migrations.
    /// Also repairs any stale `streaming` messages left by a prior unclean exit.
    pub fn open(paths: RoninPaths) -> Result<Self> {
        let session = Self::open_connection(paths)?;
        session.repair_stale_streaming_messages()?;
        Ok(session)
    }

    /// Opens a second connection to an already-running session's database.
    ///
    /// Unlike [`RoninSession::open`], this does **not** repair streaming messages,
    /// so background generation workers do not mark live streams as failed.
    fn open_connection(paths: RoninPaths) -> Result<Self> {
        fs::create_dir_all(&paths.config_dir).map_err(|source| RoninError::CreateConfigDir {
            path: paths.config_dir.clone(),
            source,
        })?;
        fs::create_dir_all(&paths.data_dir).map_err(|source| RoninError::CreateDataDir {
            path: paths.data_dir.clone(),
            source,
        })?;

        let logging = peek_logging_config(&paths.config_dir);
        let cache_home = std::env::var("XDG_CACHE_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|_| {
                std::env::var("HOME").map(|home| std::path::PathBuf::from(home).join(".cache"))
            })
            .unwrap_or_else(|_| std::path::PathBuf::from(".cache"));
        init_tracing_with(FileLogOptions {
            enabled: logging.file_enabled,
            log_dir: default_log_dir(&cache_home),
            max_file_bytes: if logging.max_file_bytes == 0 {
                DEFAULT_MAX_LOG_FILE_BYTES
            } else {
                logging.max_file_bytes
            },
        });

        tracing::info!("opening ronin session");
        tracing::info!(config_dir = %paths.config_dir.display(), "ronin config directory ready");
        tracing::info!(data_dir = %paths.data_dir.display(), "ronin data directory ready");

        let db = RoninDb::open(paths.data_dir.join("ronin.db"))?;
        Ok(Self { db, paths })
    }

    /// Returns the filesystem paths for this session.
    pub fn paths(&self) -> &RoninPaths {
        &self.paths
    }

    fn repair_stale_streaming_messages(&self) -> Result<()> {
        let stale_msgs = self.db.find_stale_streaming_messages()?;
        for msg in stale_msgs {
            tracing::info!(message_id = %msg.id, "repairing stale streaming message on startup");
            self.db.update_message_status(
                &msg.id,
                "failed",
                Some("Generation interrupted because Ronin exited before the response completed."),
            )?;
        }
        Ok(())
    }

    /// Creates a new user-visible thread titled `New Chat` and persists it.
    pub fn create_thread(&self) -> Result<Thread> {
        let config = self.load_config()?;
        self.db
            .create_thread_with_provider(
                config.general.default_provider.as_deref(),
                config.general.default_model.as_deref(),
            )
            .map(Thread::from)
            .map_err(Into::into)
    }

    /// Lists persisted threads in stable creation order.
    pub fn list_threads(&self) -> Result<Vec<Thread>> {
        self.db
            .list_threads()
            .map(|threads| threads.into_iter().map(Thread::from).collect())
            .map_err(Into::into)
    }

    /// Updates a thread's title and bumps its updated_at timestamp.
    pub fn update_thread_title(&self, thread_id: &str, title: &str) -> Result<()> {
        self.db
            .update_thread_title(thread_id, title)
            .map_err(Into::into)
    }

    /// Updates a thread's provider and bumps its updated_at timestamp.
    pub fn set_thread_provider(&self, thread_id: &str, provider: &str) -> Result<()> {
        self.db
            .update_thread_provider(thread_id, Some(provider))
            .map_err(Into::into)
    }

    /// Updates a thread's model and bumps its updated_at timestamp.
    pub fn set_thread_model(&self, thread_id: &str, model: &str) -> Result<()> {
        self.db
            .update_thread_model(thread_id, Some(model))
            .map_err(Into::into)
    }

    /// Returns the opt-in workspace root bound to a thread, if any.
    pub fn thread_workspace_root(&self, thread_id: &str) -> Result<Option<std::path::PathBuf>> {
        let root = self
            .list_threads()?
            .into_iter()
            .find(|t| t.id == thread_id)
            .and_then(|t| t.workspace_root);
        Ok(root)
    }

    /// Binds an explicit workspace root to a thread (opt-in; never auto-detected).
    ///
    /// The path must be an existing directory. Stored as an absolute path.
    /// Binding does not attach files or inject context into the model.
    pub fn set_thread_workspace_root(
        &self,
        thread_id: &str,
        root: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let root = root.as_ref();
        let meta = std::fs::metadata(root).map_err(|_| RoninError::InvalidWorkspaceRoot {
            path: root.to_path_buf(),
        })?;
        if !meta.is_dir() {
            return Err(RoninError::InvalidWorkspaceRoot {
                path: root.to_path_buf(),
            });
        }
        let absolute = root.canonicalize().unwrap_or_else(|_| {
            if root.is_absolute() {
                root.to_path_buf()
            } else {
                std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                    .join(root)
            }
        });
        self.db
            .update_thread_workspace_root(thread_id, Some(absolute.to_string_lossy().as_ref()))
            .map_err(Into::into)
    }

    /// Clears the workspace root binding on a thread.
    pub fn clear_thread_workspace_root(&self, thread_id: &str) -> Result<()> {
        self.db
            .update_thread_workspace_root(thread_id, None)
            .map_err(Into::into)
    }

    /// Creates and persists a new message in the given thread.
    pub fn create_message(
        &self,
        thread_id: &str,
        role: MessageRole,
        content: &str,
    ) -> Result<Message> {
        let parent = self.next_parent_id(thread_id)?;
        self.create_message_with_explicit_parent(thread_id, role, content, parent.as_deref())
    }

    /// Creates a message with an explicit parent (`None` = root / sibling of roots).
    pub fn create_message_with_parent(
        &self,
        thread_id: &str,
        role: MessageRole,
        content: &str,
        parent_id: Option<&str>,
    ) -> Result<Message> {
        self.create_message_with_explicit_parent(thread_id, role, content, parent_id)
    }

    fn create_message_with_explicit_parent(
        &self,
        thread_id: &str,
        role: MessageRole,
        content: &str,
        parent_id: Option<&str>,
    ) -> Result<Message> {
        let db_role = match role {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::System => "system",
        };
        let message = self
            .db
            .create_message_with_parent(thread_id, db_role, content, "complete", parent_id)
            .map(Message::from)?;
        self.db
            .set_thread_active_leaf(thread_id, Some(&message.id))?;
        Ok(message)
    }

    /// Creates and persists a streaming assistant message placeholder.
    pub fn create_streaming_message(&self, thread_id: &str, content: &str) -> Result<Message> {
        let parent = self.next_parent_id(thread_id)?;
        self.create_streaming_message_with_parent(thread_id, content, parent.as_deref())
    }

    /// Creates a streaming assistant message under an explicit parent.
    pub fn create_streaming_message_with_parent(
        &self,
        thread_id: &str,
        content: &str,
        parent_id: Option<&str>,
    ) -> Result<Message> {
        let message = self
            .db
            .create_message_with_parent(thread_id, "assistant", content, "streaming", parent_id)
            .map(Message::from)?;
        self.db
            .set_thread_active_leaf(thread_id, Some(&message.id))?;
        Ok(message)
    }

    fn next_parent_id(&self, thread_id: &str) -> Result<Option<String>> {
        let threads = self.list_threads()?;
        if let Some(leaf) = threads
            .iter()
            .find(|t| t.id == thread_id)
            .and_then(|t| t.active_leaf_id.clone())
        {
            return Ok(Some(leaf));
        }
        let msgs = self.db.list_messages_for_thread(thread_id)?;
        Ok(msgs.last().map(|m| m.id.clone()))
    }

    /// Lists messages on the active conversation path for a thread.
    pub fn list_messages(&self, thread_id: &str) -> Result<Vec<Message>> {
        let all = self.list_all_messages(thread_id)?;
        let leaf = self
            .list_threads()?
            .into_iter()
            .find(|t| t.id == thread_id)
            .and_then(|t| t.active_leaf_id);
        if leaf.is_none() && all.iter().all(|m| m.parent_id.is_none()) {
            return Ok(all);
        }
        Ok(resolve_path_messages(&all, leaf.as_deref()))
    }

    /// Lists every persisted message in the thread (all branches).
    pub fn list_all_messages(&self, thread_id: &str) -> Result<Vec<Message>> {
        self.db
            .list_messages_for_thread(thread_id)
            .map(|msgs| msgs.into_iter().map(Message::from).collect())
            .map_err(Into::into)
    }

    /// Sets the active leaf tip for branch navigation.
    pub fn set_active_leaf(&self, thread_id: &str, leaf_id: &str) -> Result<()> {
        self.db
            .set_thread_active_leaf(thread_id, Some(leaf_id))
            .map_err(Into::into)
    }

    /// Replaces an assistant message's content and sets status to complete.
    pub fn complete_message(&self, message_id: &str, content: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "complete", None)
            .map_err(Into::into)
    }

    /// Cancels a streaming message, saving partial output.
    pub fn cancel_message(&self, message_id: &str, content: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "cancelled", None)
            .map_err(Into::into)
    }

    /// Deletes a message.
    pub fn delete_message(&self, message_id: &str) -> Result<()> {
        self.db.delete_message(message_id).map_err(Into::into)
    }

    /// Fails a message with an error.
    pub fn fail_message(&self, message_id: &str, content: &str, error_message: &str) -> Result<()> {
        self.db
            .update_message_content_and_status(message_id, content, "failed", Some(error_message))
            .map_err(Into::into)
    }

    /// Loads the previously selected Ollama model from config, if any.
    pub fn load_selected_model(&self) -> Result<Option<String>> {
        let config = self.load_config()?;
        Ok(config.general.default_model)
    }

    /// Creates an independent session handle pointing at the same database.
    ///
    /// Opens a separate SQLite connection so the caller can write from a
    /// background thread without blocking the main session. Does not repair
    /// streaming messages (those belong to the live main-session generations).
    pub fn clone_session(&self) -> Result<Self> {
        Self::open_connection(self.paths.clone())
    }

    /// Saves the selected Ollama model to config.
    pub fn save_selected_model(&self, model: &str) -> Result<()> {
        let mut config = self.load_config()?;
        config.general.default_model = Some(model.to_string());
        let config_path = self.paths.config_dir.join("config.toml");
        let data = toml::to_string_pretty(&config)
            .map_err(|e| RoninError::Config(format!("serialize config: {e}")))?;
        fs::write(&config_path, data)
            .map_err(|e| RoninError::Config(format!("write config: {e}")))?;
        tracing::info!(model = %model, "saved selected model to config");
        Ok(())
    }

    /// Loads the config.toml file if it exists.
    pub fn load_config(&self) -> Result<RoninConfig> {
        let config_path = self.paths.config_dir.join("config.toml");
        if !config_path.is_file() {
            return Ok(RoninConfig::default());
        }
        let data = fs::read_to_string(&config_path)
            .map_err(|e| RoninError::Config(format!("read config.toml: {e}")))?;
        toml::from_str(&data).map_err(|e| RoninError::Config(format!("parse config.toml: {e}")))
    }

    /// Writes the full configuration to config.toml.
    pub fn save_config(&self, config: &RoninConfig) -> Result<()> {
        let config_path = self.paths.config_dir.join("config.toml");
        let data = toml::to_string_pretty(config)
            .map_err(|e| RoninError::Config(format!("serialize config: {e}")))?;
        fs::write(&config_path, data)
            .map_err(|e| RoninError::Config(format!("write config: {e}")))?;
        Ok(())
    }

    /// Exports non-secret provider settings to a TOML file.
    pub fn export_provider_config_to_file(&self, path: &std::path::Path) -> Result<()> {
        let config = self.load_config()?;
        let data =
            crate::config::export_provider_config_toml(&config).map_err(RoninError::Config)?;
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| RoninError::Config(format!("create export directory: {e}")))?;
            }
        }
        fs::write(path, data).map_err(|e| RoninError::Config(format!("write export file: {e}")))?;
        tracing::info!(path = %path.display(), "exported provider config");
        Ok(())
    }

    /// Imports provider settings from a TOML file, validating before apply.
    pub fn import_provider_config_from_file(&self, path: &std::path::Path) -> Result<()> {
        let data = fs::read_to_string(path)
            .map_err(|e| RoninError::Config(format!("read import file: {e}")))?;
        let current = self.load_config()?;
        let merged = crate::config::import_provider_config_toml(&current, &data)
            .map_err(RoninError::Config)?;
        self.save_config(&merged)?;
        tracing::info!(path = %path.display(), "imported provider config");
        Ok(())
    }

    /// Creates a new artifact.
    pub fn create_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
    ) -> Result<Artifact> {
        self.db
            .create_artifact(thread_id, message_id, title, content)
            .map(Artifact::from)
            .map_err(Into::into)
    }

    /// Creates a code-snippet artifact, preserving fence language metadata.
    pub fn create_snippet_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
        language: &str,
    ) -> Result<Artifact> {
        self.db
            .create_snippet_artifact(thread_id, message_id, title, content, language)
            .map(Artifact::from)
            .map_err(Into::into)
    }

    /// Lists artifacts for a thread.
    pub fn list_artifacts(&self, thread_id: &str) -> Result<Vec<Artifact>> {
        self.db
            .list_artifacts_for_thread(thread_id)
            .map(|artifacts| artifacts.into_iter().map(Artifact::from).collect())
            .map_err(Into::into)
    }

    /// Lists all artifacts across all threads, newest first.
    pub fn list_all_artifacts(&self) -> Result<Vec<Artifact>> {
        self.db
            .list_all_artifacts()
            .map(|artifacts| artifacts.into_iter().map(Artifact::from).collect())
            .map_err(Into::into)
    }

    /// Deletes an artifact.
    pub fn delete_artifact(&self, id: &ArtifactId) -> Result<()> {
        self.db.delete_artifact(&id.0).map_err(Into::into)
    }

    /// Renames and/or edits an artifact's title and content.
    pub fn update_artifact(&self, id: &ArtifactId, title: &str, content: &str) -> Result<()> {
        self.db
            .update_artifact(&id.0, title, content)
            .map_err(Into::into)
    }

    /// Creates a new memory.
    pub fn create_memory(&self, title: &str, content: &str) -> Result<Memory> {
        self.db
            .create_memory(title, content)
            .map(Memory::from)
            .map_err(Into::into)
    }

    /// Creates a profile-group memory (always-on user context when enabled).
    pub fn create_profile_memory(&self, title: &str, content: &str) -> Result<Memory> {
        self.db
            .create_memory_with_flags(title, content, true, true)
            .map(Memory::from)
            .map_err(Into::into)
    }

    /// Lists all memories.
    pub fn list_memories(&self) -> Result<Vec<Memory>> {
        self.db
            .list_all_memories()
            .map(|memories| memories.into_iter().map(Memory::from).collect())
            .map_err(Into::into)
    }

    /// Updates a memory.
    pub fn update_memory(&self, id: &MemoryId, title: &str, content: &str) -> Result<()> {
        self.db
            .update_memory(&id.0, title, content)
            .map_err(Into::into)
    }

    /// Sets whether a memory is enabled for provider context.
    pub fn set_memory_enabled(&self, id: &MemoryId, enabled: bool) -> Result<()> {
        self.db
            .set_memory_enabled(&id.0, enabled)
            .map_err(Into::into)
    }

    /// Sets whether a memory belongs to the user profile group.
    pub fn set_memory_profile(&self, id: &MemoryId, is_profile: bool) -> Result<()> {
        self.db
            .set_memory_profile(&id.0, is_profile)
            .map_err(Into::into)
    }

    /// Deletes a memory.
    pub fn delete_memory(&self, id: &MemoryId) -> Result<()> {
        self.db.delete_memory(&id.0).map_err(Into::into)
    }

    /// Creates a new attachment.
    pub fn create_attachment(
        &self,
        message_id: &str,
        kind: AttachmentKind,
        name: &str,
        mime_type: &str,
        content: Option<&str>,
        path: Option<&str>,
    ) -> Result<Attachment> {
        let db_kind = match kind {
            AttachmentKind::File => "file",
            AttachmentKind::Clipboard => "clipboard",
            AttachmentKind::Memory => "memory",
            AttachmentKind::Artifact => "artifact",
            AttachmentKind::Image => "image",
            AttachmentKind::Screenshot => "screenshot",
            AttachmentKind::Folder => "folder",
        };
        self.db
            .create_attachment(message_id, db_kind, name, mime_type, content, path)
            .map(Attachment::from)
            .map_err(Into::into)
    }

    /// Lists attachments for a message.
    pub fn list_attachments(&self, message_id: &str) -> Result<Vec<Attachment>> {
        self.db
            .list_attachments_for_message(message_id)
            .map(|attachments| attachments.into_iter().map(Attachment::from).collect())
            .map_err(Into::into)
    }

    /// Deletes an attachment.
    pub fn delete_attachment(&self, id: &AttachmentId) -> Result<()> {
        self.db.delete_attachment(&id.0).map_err(Into::into)
    }
}

fn resolve_path_messages(all: &[Message], active_leaf_id: Option<&str>) -> Vec<Message> {
    let Some(leaf) = active_leaf_id else {
        return all.to_vec();
    };
    if !all.iter().any(|m| m.id == leaf) {
        return all.to_vec();
    }
    let by_id: std::collections::HashMap<&str, &Message> =
        all.iter().map(|m| (m.id.as_str(), m)).collect();
    let mut path = Vec::new();
    let mut current = Some(leaf);
    let mut guard = 0usize;
    while let Some(id) = current {
        guard += 1;
        if guard > all.len() + 2 {
            break;
        }
        let Some(msg) = by_id.get(id).copied() else {
            break;
        };
        path.push(msg.clone());
        current = msg.parent_id.as_deref();
    }
    path.reverse();
    path
}

/// Reads `[logging]` from config.toml without requiring an open database.
fn peek_logging_config(config_dir: &std::path::Path) -> LoggingConfig {
    let path = config_dir.join("config.toml");
    let Ok(data) = fs::read_to_string(path) else {
        return LoggingConfig::default();
    };
    toml::from_str::<RoninConfig>(&data)
        .map(|c| c.logging)
        .unwrap_or_default()
}
