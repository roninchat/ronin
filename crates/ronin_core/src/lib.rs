#![deny(missing_docs)]

//! Public application/session boundary for Ronin.

use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use ronin_db::{init_tracing, DbThread, RoninDb, RoninDbError};

/// Result type returned by `ronin_core` operations.
pub type Result<T> = std::result::Result<T, RoninError>;

/// Ollama server health status reported through the provider boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaHealth {
    /// Ollama server is reachable and responding.
    Online,
    /// Ollama server is not reachable.
    Offline,
}

/// Provider boundary for querying Ollama status and available models.
pub trait OllamaProvider {
    /// Checks whether the Ollama server is reachable.
    fn check_health(&self) -> OllamaHealth;
    /// Lists available model names from the provider.
    fn list_models(&self) -> Result<Vec<String>>;
}

/// HTTP-based Ollama provider that queries the Ollama REST API.
pub struct HttpOllamaProvider {
    base_url: String,
}

impl HttpOllamaProvider {
    /// Creates a new provider targeting the given Ollama base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}

impl OllamaProvider for HttpOllamaProvider {
    fn check_health(&self) -> OllamaHealth {
        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
        {
            Ok(c) => c,
            Err(_) => return OllamaHealth::Offline,
        };

        match client.get(format!("{}/api/version", self.base_url)).send() {
            Ok(resp) if resp.status().is_success() => OllamaHealth::Online,
            _ => OllamaHealth::Offline,
        }
    }

    fn list_models(&self) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct ModelEntry {
            name: String,
        }

        #[derive(serde::Deserialize)]
        struct ListResponse {
            models: Vec<ModelEntry>,
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|source| RoninError::Provider(source.to_string()))?;

        let resp: ListResponse = client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .map_err(|source| RoninError::Provider(source.to_string()))?
            .json()
            .map_err(|source| RoninError::Provider(source.to_string()))?;

        Ok(resp.models.into_iter().map(|m| m.name).collect())
    }
}

/// Errors returned by Ronin's public session boundary.
#[derive(Debug, thiserror::Error)]
pub enum RoninError {
    /// Ronin could not create or access its configuration directory.
    #[error("failed to create Ronin config directory at {path}")]
    CreateConfigDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin could not create or access its data directory.
    #[error("failed to create Ronin data directory at {path}")]
    CreateDataDir {
        /// Directory Ronin attempted to create.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Ronin's SQLite persistence layer failed.
    #[error(transparent)]
    Db(#[from] RoninDbError),

    /// Provider operation failed.
    #[error("provider error: {0}")]
    Provider(String),

    /// Ronin configuration read/write failed.
    #[error("config error: {0}")]
    Config(String),
}

/// Filesystem locations used by a Ronin app session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoninPaths {
    /// Directory for user-editable or generated configuration files.
    pub config_dir: PathBuf,
    /// Directory for local application data such as `ronin.db`.
    pub data_dir: PathBuf,
}

/// User-visible conversation thread metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    /// App-generated opaque thread identifier.
    pub id: String,
    /// User-visible title. New M0 threads start as `New Chat`.
    pub title: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
    /// Whether the thread is archived. M0 stores this but has no UI for it yet.
    pub archived: bool,
}

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
    pub fn open(paths: RoninPaths) -> Result<Self> {
        init_tracing();
        tracing::info!("opening ronin session");
        fs::create_dir_all(&paths.config_dir).map_err(|source| RoninError::CreateConfigDir {
            path: paths.config_dir.clone(),
            source,
        })?;
        tracing::info!(config_dir = %paths.config_dir.display(), "ronin config directory ready");

        fs::create_dir_all(&paths.data_dir).map_err(|source| RoninError::CreateDataDir {
            path: paths.data_dir.clone(),
            source,
        })?;
        tracing::info!(data_dir = %paths.data_dir.display(), "ronin data directory ready");

        let db = RoninDb::open(paths.data_dir.join("ronin.db"))?;
        Ok(Self { db, paths })
    }

    /// Creates a new user-visible thread titled `New Chat` and persists it.
    pub fn create_thread(&self) -> Result<Thread> {
        self.db
            .create_thread()
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

    /// Loads the previously selected Ollama model from config, if any.
    pub fn load_selected_model(&self) -> Result<Option<String>> {
        let config_path = self.paths.config_dir.join("ronin_config.json");
        if !config_path.is_file() {
            return Ok(None);
        }
        let data = fs::read_to_string(&config_path)
            .map_err(|e| RoninError::Config(format!("read config: {e}")))?;
        #[derive(serde::Deserialize)]
        struct Config {
            selected_model: Option<String>,
        }
        let config: Config = serde_json::from_str(&data)
            .map_err(|e| RoninError::Config(format!("parse config: {e}")))?;
        Ok(config.selected_model)
    }

    /// Saves the selected Ollama model to config.
    pub fn save_selected_model(&self, model: &str) -> Result<()> {
        let config_path = self.paths.config_dir.join("ronin_config.json");
        #[derive(serde::Serialize)]
        struct Config {
            selected_model: String,
        }
        let config = Config {
            selected_model: model.to_string(),
        };
        let data = serde_json::to_string_pretty(&config)
            .map_err(|e| RoninError::Config(format!("serialize config: {e}")))?;
        fs::write(&config_path, data)
            .map_err(|e| RoninError::Config(format!("write config: {e}")))?;
        tracing::info!(model = %model, "saved selected model to config");
        Ok(())
    }
}

impl From<DbThread> for Thread {
    fn from(thread: DbThread) -> Self {
        Self {
            id: thread.id,
            title: thread.title,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            archived: thread.archived,
        }
    }
}
