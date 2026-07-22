//! Open Ronin application session backed by local filesystem state.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use ronin_db::{
    default_log_dir, delete_workspace_lexical_store, init_tracing_with, DbWorkspaceIndexMeta,
    FileLogOptions, LexicalIndexDocument, RoninDb, WorkspaceLexicalStore,
    DEFAULT_MAX_LOG_FILE_BYTES,
};

use crate::config::{LoggingConfig, RoninConfig};
use crate::domain::{
    Artifact, ArtifactId, Attachment, AttachmentId, AttachmentKind, Memory, MemoryId, Message,
    MessageRole, RoninPaths, Thread,
};
use crate::error::{Result, RoninError};
use crate::workspace_index::{
    collect_workspace_index_documents, workspace_index_storage_path, WorkspaceIndexCaps,
    WorkspaceIndexInfo, WorkspaceIndexPhase, WORKSPACE_INDEX_STORAGE_DIR,
};

/// Open Ronin application session backed by local filesystem state.
pub struct RoninSession {
    db: RoninDb,
    paths: RoninPaths,
    /// Per-thread cancel flags for in-progress one-shot index builds.
    index_cancel_flags: Mutex<HashMap<String, Arc<AtomicBool>>>,
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
        Ok(Self {
            db,
            paths,
            index_cancel_flags: Mutex::new(HashMap::new()),
        })
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

    /// Returns status for a thread's lexical workspace index (absent when none).
    pub fn workspace_index_info(&self, thread_id: &str) -> Result<WorkspaceIndexInfo> {
        Ok(match self.db.get_workspace_index_meta(thread_id)? {
            Some(meta) => self.info_from_meta(meta),
            None => WorkspaceIndexInfo::absent(),
        })
    }

    /// Explicit one-shot “Index this workspace” build for a thread with a bound root.
    ///
    /// Does not start on session open. Never merges corpus into chat assembly.
    pub fn build_workspace_index(&self, thread_id: &str) -> Result<WorkspaceIndexInfo> {
        self.build_workspace_index_with_caps(thread_id, &WorkspaceIndexCaps::default())
    }

    /// Rebuilds the lexical index (same as build; replaces any prior corpus).
    pub fn rebuild_workspace_index(&self, thread_id: &str) -> Result<WorkspaceIndexInfo> {
        self.build_workspace_index(thread_id)
    }

    /// One-shot build with explicit caps (tests / host orchestration).
    pub fn build_workspace_index_with_caps(
        &self,
        thread_id: &str,
        caps: &WorkspaceIndexCaps,
    ) -> Result<WorkspaceIndexInfo> {
        let cancel = Arc::new(AtomicBool::new(false));
        self.build_workspace_index_cancellable(thread_id, caps, cancel)
    }

    /// Build that observes a shared cancel flag (cooperative cancel / host threads).
    pub fn build_workspace_index_cancellable(
        &self,
        thread_id: &str,
        caps: &WorkspaceIndexCaps,
        cancel: Arc<AtomicBool>,
    ) -> Result<WorkspaceIndexInfo> {
        {
            let mut flags = self
                .index_cancel_flags
                .lock()
                .map_err(|_| RoninError::WorkspaceIndexCancelLock)?;
            flags.insert(thread_id.to_string(), Arc::clone(&cancel));
        }
        let result = self.run_workspace_index_build(thread_id, caps, &cancel);
        {
            let mut flags = self
                .index_cancel_flags
                .lock()
                .map_err(|_| RoninError::WorkspaceIndexCancelLock)?;
            flags.remove(thread_id);
        }
        result
    }

    /// Requests cancel of an in-progress index build for `thread_id`.
    pub fn cancel_workspace_index(&self, thread_id: &str) -> Result<()> {
        let flags = self
            .index_cancel_flags
            .lock()
            .map_err(|_| RoninError::WorkspaceIndexCancelLock)?;
        if let Some(flag) = flags.get(thread_id) {
            flag.store(true, Ordering::SeqCst);
        }
        Ok(())
    }

    /// Deletes index metadata and on-disk lexical corpus for a thread.
    pub fn delete_workspace_index(&self, thread_id: &str) -> Result<()> {
        let storage = workspace_index_storage_path(&self.paths.data_dir, thread_id);
        delete_workspace_lexical_store(&storage)?;
        self.db.delete_workspace_index_meta(thread_id)?;
        Ok(())
    }

    /// Absolute path where this thread's lexical index DB would live.
    pub fn workspace_index_storage_path_for(&self, thread_id: &str) -> PathBuf {
        workspace_index_storage_path(&self.paths.data_dir, thread_id)
    }

    fn run_workspace_index_build(
        &self,
        thread_id: &str,
        caps: &WorkspaceIndexCaps,
        cancel: &AtomicBool,
    ) -> Result<WorkspaceIndexInfo> {
        let root = self.thread_workspace_root(thread_id)?.ok_or_else(|| {
            RoninError::WorkspaceIndex(
                "thread has no workspace root; set one before indexing".into(),
            )
        })?;

        let storage = workspace_index_storage_path(&self.paths.data_dir, thread_id);
        let storage_relpath = format!("{WORKSPACE_INDEX_STORAGE_DIR}/{thread_id}.db");
        let now = unix_now_ms();
        let prior_done = self
            .db
            .get_workspace_index_meta(thread_id)?
            .filter(|m| m.phase == WorkspaceIndexPhase::Done.as_str());

        self.db.upsert_workspace_index_meta(&DbWorkspaceIndexMeta {
            thread_id: thread_id.to_string(),
            phase: WorkspaceIndexPhase::Running.as_str().to_string(),
            workspace_root: Some(root.to_string_lossy().into_owned()),
            entry_count: 0,
            byte_count: 0,
            truncated: false,
            error_message: None,
            storage_relpath: Some(storage_relpath.clone()),
            built_at: None,
            updated_at: now,
        })?;

        let policy = self.folder_list_policy()?;
        let collected = collect_workspace_index_documents(&root, &policy, caps, cancel);

        if let Some(err) = collected.error_message {
            let meta = DbWorkspaceIndexMeta {
                thread_id: thread_id.to_string(),
                phase: WorkspaceIndexPhase::Failed.as_str().to_string(),
                workspace_root: Some(root.to_string_lossy().into_owned()),
                entry_count: prior_done.as_ref().map(|p| p.entry_count).unwrap_or(0),
                byte_count: prior_done.as_ref().map(|p| p.byte_count).unwrap_or(0),
                truncated: false,
                error_message: Some(err),
                storage_relpath: Some(storage_relpath),
                built_at: prior_done.as_ref().and_then(|p| p.built_at),
                updated_at: unix_now_ms(),
            };
            self.db.upsert_workspace_index_meta(&meta)?;
            // Leave any prior on-disk corpus untouched on failure.
            return Ok(self.info_from_meta(meta));
        }

        if collected.cancelled {
            let (entry_count, byte_count, storage, built_at) = match prior_done {
                Some(p) => (p.entry_count, p.byte_count, p.storage_relpath, p.built_at),
                None => (0, 0, Some(storage_relpath.clone()), None),
            };
            let meta = DbWorkspaceIndexMeta {
                thread_id: thread_id.to_string(),
                phase: WorkspaceIndexPhase::Cancelled.as_str().to_string(),
                workspace_root: Some(root.to_string_lossy().into_owned()),
                entry_count,
                byte_count,
                truncated: true,
                error_message: None,
                storage_relpath: storage,
                built_at,
                updated_at: unix_now_ms(),
            };
            self.db.upsert_workspace_index_meta(&meta)?;
            // Do not replace on-disk corpus on cancel — preserve prior Done index.
            return Ok(self.info_from_meta(meta));
        }

        let byte_count = collected.byte_count;
        let truncated = collected.truncated;
        let docs: Vec<LexicalIndexDocument> = collected
            .documents
            .into_iter()
            .map(|d| LexicalIndexDocument {
                relative_path: d.relative_path,
                body: d.body,
                byte_len: d.byte_len,
            })
            .collect();

        let store = WorkspaceLexicalStore::open(&storage)?;
        store.replace_documents(&docs)?;

        let built_at = unix_now_ms();
        let meta = DbWorkspaceIndexMeta {
            thread_id: thread_id.to_string(),
            phase: WorkspaceIndexPhase::Done.as_str().to_string(),
            workspace_root: Some(root.to_string_lossy().into_owned()),
            entry_count: docs.len() as i64,
            byte_count: byte_count as i64,
            truncated,
            error_message: None,
            storage_relpath: Some(storage_relpath),
            built_at: Some(built_at),
            updated_at: built_at,
        };
        self.db.upsert_workspace_index_meta(&meta)?;
        Ok(self.info_from_meta(meta))
    }

    fn info_from_meta(&self, meta: DbWorkspaceIndexMeta) -> WorkspaceIndexInfo {
        let phase = WorkspaceIndexPhase::parse(&meta.phase).unwrap_or(WorkspaceIndexPhase::Failed);
        let storage_path = meta
            .storage_relpath
            .as_ref()
            .map(|rel| self.paths.data_dir.join(rel));
        WorkspaceIndexInfo {
            phase,
            workspace_root: meta.workspace_root.map(PathBuf::from),
            entry_count: meta.entry_count as u64,
            byte_count: meta.byte_count as u64,
            truncated: meta.truncated,
            error_message: meta.error_message,
            storage_path,
            built_at_ms: meta.built_at,
        }
    }

    /// Builds the folder-list policy from persisted local-knowledge preferences.
    pub fn folder_list_policy(&self) -> Result<crate::FolderListPolicy> {
        let lk = self.load_config()?.local_knowledge;
        Ok(crate::FolderListPolicy {
            honor_gitignore: true,
            apply_built_in_deny: true,
            never_list: lk.never_list.iter().map(PathBuf::from).collect(),
            allowlist_enabled: lk.allowlist_enabled,
            allowlist: lk.allowlist.iter().map(PathBuf::from).collect(),
        })
    }

    /// Returns persisted never-list paths (never-list / never-index).
    pub fn list_never_list_paths(&self) -> Result<Vec<PathBuf>> {
        Ok(self
            .load_config()?
            .local_knowledge
            .never_list
            .into_iter()
            .map(PathBuf::from)
            .collect())
    }

    /// Marks a directory as never-list / never-index.
    pub fn add_never_list_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.mutate_path_list(true, path, true)
    }

    /// Removes a path from the never-list registry.
    pub fn remove_never_list_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.mutate_path_list(true, path, false)
    }

    /// Whether folder allowlist mode is enabled.
    pub fn folder_allowlist_enabled(&self) -> Result<bool> {
        Ok(self.load_config()?.local_knowledge.allowlist_enabled)
    }

    /// Enables or disables folder allowlist mode.
    pub fn set_folder_allowlist_enabled(&self, enabled: bool) -> Result<()> {
        let mut config = self.load_config()?;
        if config.local_knowledge.allowlist_enabled != enabled {
            config.local_knowledge.allowlist_enabled = enabled;
            self.save_config(&config)?;
        }
        Ok(())
    }

    /// Returns approved allowlist roots.
    pub fn list_folder_allowlist_roots(&self) -> Result<Vec<PathBuf>> {
        Ok(self
            .load_config()?
            .local_knowledge
            .allowlist
            .into_iter()
            .map(PathBuf::from)
            .collect())
    }

    /// Adds an approved root for allowlist mode.
    pub fn add_folder_allowlist_root(&self, path: impl AsRef<Path>) -> Result<()> {
        self.mutate_path_list(false, path, true)
    }

    /// Removes a root from the folder allowlist.
    pub fn remove_folder_allowlist_root(&self, path: impl AsRef<Path>) -> Result<()> {
        self.mutate_path_list(false, path, false)
    }

    fn mutate_path_list(&self, never_list: bool, path: impl AsRef<Path>, add: bool) -> Result<()> {
        let absolute = if add {
            require_existing_dir(path.as_ref())?
        } else {
            crate::absolutize_path(path.as_ref())
        };
        let key = absolute.to_string_lossy().into_owned();
        let mut config = self.load_config()?;
        let list = if never_list {
            &mut config.local_knowledge.never_list
        } else {
            &mut config.local_knowledge.allowlist
        };
        let before = list.len();
        if add {
            if !list.iter().any(|p| p == &key) {
                list.push(key);
            }
        } else {
            list.retain(|p| p != &key);
        }
        if list.len() != before {
            self.save_config(&config)?;
        }
        Ok(())
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

fn require_existing_dir(path: &Path) -> Result<PathBuf> {
    let meta = std::fs::metadata(path).map_err(|_| RoninError::InvalidPrivacyPath {
        path: path.to_path_buf(),
    })?;
    if !meta.is_dir() {
        return Err(RoninError::InvalidPrivacyPath {
            path: path.to_path_buf(),
        });
    }
    Ok(crate::absolutize_path(path))
}

fn unix_now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
