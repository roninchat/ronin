#![deny(missing_docs)]

//! SQLite persistence for Ronin.

use std::path::{Path, PathBuf};
use std::sync::Once;

use rusqlite::{params, Connection};
use time::OffsetDateTime;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

static TRACING_INIT: Once = Once::new();

const MIGRATIONS: &[(i64, &str)] = &[
    (1, include_str!("../migrations/0001_initial.sql")),
    (
        2,
        include_str!("../migrations/0002_artifacts_memories_attachments.sql"),
    ),
];

/// Result type returned by `ronin_db` operations.
pub type Result<T> = std::result::Result<T, RoninDbError>;

/// Errors returned by Ronin's SQLite persistence layer.
#[derive(Debug, thiserror::Error)]
pub enum RoninDbError {
    /// SQLite could not open the database file.
    #[error("failed to open Ronin database at {path}")]
    Open {
        /// Database path Ronin attempted to open.
        path: PathBuf,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// SQLite foreign-key enforcement could not be enabled.
    #[error("failed to enable SQLite foreign keys")]
    EnableForeignKeys(#[source] rusqlite::Error),

    /// A thread insert failed.
    #[error("failed to create thread")]
    CreateThread(#[source] rusqlite::Error),

    /// A message insert failed.
    #[error("failed to create message")]
    CreateMessage(#[source] rusqlite::Error),

    /// Preparing the message list query failed.
    #[error("failed to prepare message list query")]
    PrepareMessageList(#[source] rusqlite::Error),

    /// Querying rows for the message list failed.
    #[error("failed to query messages")]
    QueryMessages(#[source] rusqlite::Error),

    /// Reading one or more message rows failed.
    #[error("failed to read messages")]
    ReadMessages(#[source] rusqlite::Error),

    /// Updating a message failed.
    #[error("failed to update message {id}")]
    UpdateMessage {
        /// Message id being updated.
        id: String,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// Deleting a message failed.
    #[error("failed to delete message {id}")]
    DeleteMessage {
        /// Message id being deleted.
        id: String,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// Preparing the thread list query failed.
    #[error("failed to prepare thread list query")]
    PrepareThreadList(#[source] rusqlite::Error),

    /// Querying rows for the thread list failed.
    #[error("failed to query threads")]
    QueryThreads(#[source] rusqlite::Error),

    /// Reading one or more thread rows failed.
    #[error("failed to read threads")]
    ReadThreads(#[source] rusqlite::Error),

    /// Updating a thread title failed.
    #[error("failed to update thread {id}")]
    UpdateThread {
        /// Thread id being updated.
        id: String,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// Creating the migration bookkeeping table failed.
    #[error("failed to create schema_migrations table")]
    CreateSchemaMigrations(#[source] rusqlite::Error),

    /// Checking whether a migration has already run failed.
    #[error("failed to check migration status")]
    CheckMigrationStatus(#[source] rusqlite::Error),

    /// Starting a migration transaction failed.
    #[error("failed to start migration transaction")]
    StartMigrationTransaction(#[source] rusqlite::Error),

    /// Applying a migration body failed.
    #[error("failed to apply migration {version}")]
    ApplyMigration {
        /// Migration version being applied.
        version: i64,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// Recording a completed migration failed.
    #[error("failed to record migration {version}")]
    RecordMigration {
        /// Migration version being recorded.
        version: i64,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// Committing a migration transaction failed.
    #[error("failed to commit migration {version}")]
    CommitMigration {
        /// Migration version being committed.
        version: i64,
        /// Underlying SQLite error.
        #[source]
        source: rusqlite::Error,
    },

    /// An artifact operation failed.
    /// An artifact operation failed during creation.
    #[error("failed to create artifact")]
    CreateArtifact(#[source] rusqlite::Error),
    /// An artifact could not be fetched.
    #[error("failed to get artifact")]
    GetArtifact(#[source] rusqlite::Error),
    /// Artifacts could not be listed.
    #[error("failed to list artifacts")]
    ListArtifacts(#[source] rusqlite::Error),
    /// An artifact could not be deleted.
    #[error("failed to delete artifact")]
    DeleteArtifact(#[source] rusqlite::Error),

    /// A memory operation failed during creation.
    #[error("failed to create memory")]
    CreateMemory(#[source] rusqlite::Error),
    /// A memory could not be fetched.
    #[error("failed to get memory")]
    GetMemory(#[source] rusqlite::Error),
    /// Memories could not be listed.
    #[error("failed to list memories")]
    ListMemories(#[source] rusqlite::Error),
    /// A memory could not be updated.
    #[error("failed to update memory")]
    UpdateMemory(#[source] rusqlite::Error),

    /// A memory could not be deleted.
    #[error("failed to delete memory")]
    DeleteMemory(#[source] rusqlite::Error),

    /// An attachment operation failed during creation.
    #[error("failed to create attachment")]
    CreateAttachment(#[source] rusqlite::Error),
    /// An attachment could not be fetched.
    #[error("failed to get attachment")]
    GetAttachment(#[source] rusqlite::Error),
    /// Attachments could not be listed.
    #[error("failed to list attachments")]
    ListAttachments(#[source] rusqlite::Error),
    /// An attachment could not be deleted.
    #[error("failed to delete attachment")]
    DeleteAttachment(#[source] rusqlite::Error),
}

/// Persisted message row returned by `ronin_db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbMessage {
    /// App-generated opaque message identifier stored as SQLite `TEXT`.
    pub id: String,
    /// Owning thread identifier.
    pub thread_id: String,
    /// Message role (`user`, `assistant`, or `system`).
    pub role: String,
    /// Message body content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Message lifecycle status (`complete`, `streaming`, or `error`).
    pub status: String,
    /// Sanitized failure reason when status is `error`.
    pub error_message: Option<String>,
}

/// Persisted thread row returned by `ronin_db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbThread {
    /// App-generated opaque thread identifier stored as SQLite `TEXT`.
    pub id: String,
    /// User-visible title.
    pub title: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
    /// Whether the thread is archived.
    pub archived: bool,
}

/// Persisted artifact row returned by `ronin_db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbArtifact {
    /// App-generated opaque artifact identifier stored as SQLite `TEXT`.
    pub id: String,
    /// Owning thread identifier.
    pub thread_id: String,
    /// Originating message identifier.
    pub message_id: String,
    /// Artifact title.
    pub title: String,
    /// Artifact content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
}

/// Persisted memory row returned by `ronin_db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbMemory {
    /// App-generated opaque memory identifier stored as SQLite `TEXT`.
    pub id: String,
    /// Memory title.
    pub title: String,
    /// Memory content.
    pub content: String,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
    /// Last update timestamp as UTC Unix milliseconds.
    pub updated_at: i64,
}

/// Persisted attachment row returned by `ronin_db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbAttachment {
    /// App-generated opaque attachment identifier stored as SQLite `TEXT`.
    pub id: String,
    /// Owning message identifier.
    pub message_id: String,
    /// Attachment kind (`file` or `clipboard`).
    pub kind: String,
    /// Attachment filename or paste name.
    pub name: String,
    /// MIME type of the attachment.
    pub mime_type: String,
    /// Text content for clipboard attachments.
    pub content: Option<String>,
    /// Path for file attachments.
    pub path: Option<String>,
    /// Creation timestamp as UTC Unix milliseconds.
    pub created_at: i64,
}

/// SQLite-backed Ronin persistence handle.
pub struct RoninDb {
    conn: Connection,
}

impl RoninDb {
    /// Opens the database at `path` and applies pending migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        init_tracing();

        let path = path.as_ref();
        info!(db_path = %path.display(), "opening ronin database");
        let conn = Connection::open(path).map_err(|source| RoninDbError::Open {
            path: path.to_path_buf(),
            source,
        })?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(RoninDbError::EnableForeignKeys)?;

        let db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    /// Creates and persists a new thread titled `New Chat`.
    pub fn create_thread(&self) -> Result<DbThread> {
        let now = unix_timestamp_millis();
        let thread = DbThread {
            id: Uuid::now_v7().to_string(),
            title: "New Chat".to_string(),
            created_at: now,
            updated_at: now,
            archived: false,
        };

        self.conn
            .execute(
                "INSERT INTO threads (id, title, created_at, updated_at, archived) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    thread.id,
                    thread.title,
                    thread.created_at,
                    thread.updated_at,
                    if thread.archived { 1_i64 } else { 0_i64 }
                ],
            )
            .map_err(RoninDbError::CreateThread)?;

        Ok(thread)
    }

    /// Updates the title of a thread and bumps its updated_at timestamp.
    pub fn update_thread_title(&self, id: &str, title: &str) -> Result<()> {
        let now = unix_timestamp_millis();
        self.conn
            .execute(
                "UPDATE threads SET title = ?1, updated_at = ?2 WHERE id = ?3",
                params![title, now, id],
            )
            .map_err(|source| RoninDbError::UpdateThread {
                id: id.to_string(),
                source,
            })?;
        Ok(())
    }

    /// Lists persisted threads in stable creation order.
    pub fn list_threads(&self) -> Result<Vec<DbThread>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, title, created_at, updated_at, archived FROM threads ORDER BY created_at ASC, id ASC",
            )
            .map_err(RoninDbError::PrepareThreadList)?;

        let threads = stmt
            .query_map([], |row| {
                Ok(DbThread {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    archived: row.get::<_, i64>(4)? != 0,
                })
            })
            .map_err(RoninDbError::QueryThreads)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ReadThreads)?;

        Ok(threads)
    }

    /// Creates and persists a new message in the given thread.
    pub fn create_message(
        &self,
        thread_id: &str,
        role: &str,
        content: &str,
        status: &str,
    ) -> Result<DbMessage> {
        let now = unix_timestamp_millis();
        let message = DbMessage {
            id: Uuid::now_v7().to_string(),
            thread_id: thread_id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            created_at: now,
            status: status.to_string(),
            error_message: None,
        };

        self.conn
            .execute(
                "INSERT INTO messages (id, thread_id, role, content, created_at, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    message.id,
                    message.thread_id,
                    message.role,
                    message.content,
                    message.created_at,
                    message.status,
                ],
            )
            .map_err(RoninDbError::CreateMessage)?;

        Ok(message)
    }

    /// Lists messages for a thread in creation order.
    pub fn list_messages_for_thread(&self, thread_id: &str) -> Result<Vec<DbMessage>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, thread_id, role, content, created_at, status, error_message FROM messages WHERE thread_id = ?1 ORDER BY created_at ASC, id ASC",
            )
            .map_err(RoninDbError::PrepareMessageList)?;

        let messages = stmt
            .query_map(params![thread_id], |row| {
                Ok(DbMessage {
                    id: row.get(0)?,
                    thread_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    status: row.get(5)?,
                    error_message: row.get(6)?,
                })
            })
            .map_err(RoninDbError::QueryMessages)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ReadMessages)?;

        Ok(messages)
    }

    /// Replaces a message's content and status (for streaming completion).
    pub fn update_message_content(&self, id: &str, content: &str, status: &str) -> Result<()> {
        self.conn
            .execute(
                "UPDATE messages SET content = ?1, status = ?2 WHERE id = ?3",
                params![content, status, id],
            )
            .map_err(|source| RoninDbError::UpdateMessage {
                id: id.to_string(),
                source,
            })?;
        Ok(())
    }

    /// Deletes a message by ID.
    pub fn delete_message(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM messages WHERE id = ?1", params![id])
            .map_err(|source| RoninDbError::DeleteMessage {
                id: id.to_string(),
                source,
            })?;
        Ok(())
    }

    /// Updates a message's status and error message without changing its content.
    pub fn update_message_status(
        &self,
        id: &str,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<()> {
        self.conn
            .execute(
                "UPDATE messages SET status = ?1, error_message = ?2 WHERE id = ?3",
                params![status, error_message, id],
            )
            .map_err(|source| RoninDbError::UpdateMessage {
                id: id.to_string(),
                source,
            })?;
        Ok(())
    }

    /// Replaces a message's content, status, and error message.
    pub fn update_message_content_and_status(
        &self,
        id: &str,
        content: &str,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<()> {
        self.conn
            .execute(
                "UPDATE messages SET content = ?1, status = ?2, error_message = ?3 WHERE id = ?4",
                params![content, status, error_message, id],
            )
            .map_err(|source| RoninDbError::UpdateMessage {
                id: id.to_string(),
                source,
            })?;
        Ok(())
    }

    /// Finds all messages that are stuck in the 'streaming' status.
    pub fn find_stale_streaming_messages(&self) -> Result<Vec<DbMessage>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, thread_id, role, content, created_at, status, error_message FROM messages WHERE status = 'streaming' ORDER BY created_at ASC, id ASC",
            )
            .map_err(RoninDbError::PrepareMessageList)?;

        let messages = stmt
            .query_map([], |row| {
                Ok(DbMessage {
                    id: row.get(0)?,
                    thread_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    status: row.get(5)?,
                    error_message: row.get(6)?,
                })
            })
            .map_err(RoninDbError::QueryMessages)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ReadMessages)?;

        Ok(messages)
    }

    /// Creates and persists a new artifact.
    pub fn create_artifact(
        &self,
        thread_id: &str,
        message_id: &str,
        title: &str,
        content: &str,
    ) -> Result<DbArtifact> {
        let now = unix_timestamp_millis();
        let artifact = DbArtifact {
            id: Uuid::now_v7().to_string(),
            thread_id: thread_id.to_string(),
            message_id: message_id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
        };

        self.conn
            .execute(
                "INSERT INTO artifacts (id, thread_id, message_id, title, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    artifact.id,
                    artifact.thread_id,
                    artifact.message_id,
                    artifact.title,
                    artifact.content,
                    artifact.created_at,
                ],
            )
            .map_err(RoninDbError::CreateArtifact)?;

        Ok(artifact)
    }

    /// Fetches an artifact by ID.
    pub fn get_artifact(&self, id: &str) -> Result<Option<DbArtifact>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, thread_id, message_id, title, content, created_at FROM artifacts WHERE id = ?1")
            .map_err(RoninDbError::GetArtifact)?;

        let mut rows = stmt.query(params![id]).map_err(RoninDbError::GetArtifact)?;

        if let Some(row) = rows.next().map_err(RoninDbError::GetArtifact)? {
            Ok(Some(DbArtifact {
                id: row.get(0).map_err(RoninDbError::GetArtifact)?,
                thread_id: row.get(1).map_err(RoninDbError::GetArtifact)?,
                message_id: row.get(2).map_err(RoninDbError::GetArtifact)?,
                title: row.get(3).map_err(RoninDbError::GetArtifact)?,
                content: row.get(4).map_err(RoninDbError::GetArtifact)?,
                created_at: row.get(5).map_err(RoninDbError::GetArtifact)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Lists artifacts for a specific thread.
    pub fn list_artifacts_for_thread(&self, thread_id: &str) -> Result<Vec<DbArtifact>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, thread_id, message_id, title, content, created_at FROM artifacts WHERE thread_id = ?1 ORDER BY created_at ASC, id ASC")
            .map_err(RoninDbError::ListArtifacts)?;

        let artifacts = stmt
            .query_map(params![thread_id], |row| {
                Ok(DbArtifact {
                    id: row.get(0)?,
                    thread_id: row.get(1)?,
                    message_id: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .map_err(RoninDbError::ListArtifacts)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ListArtifacts)?;

        Ok(artifacts)
    }

    /// Deletes an artifact by ID.
    pub fn delete_artifact(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM artifacts WHERE id = ?1", params![id])
            .map_err(RoninDbError::DeleteArtifact)?;
        Ok(())
    }

    /// Creates and persists a new memory.
    pub fn create_memory(&self, title: &str, content: &str) -> Result<DbMemory> {
        let now = unix_timestamp_millis();
        let memory = DbMemory {
            id: Uuid::now_v7().to_string(),
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        };

        self.conn
            .execute(
                "INSERT INTO memories (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    memory.id,
                    memory.title,
                    memory.content,
                    memory.created_at,
                    memory.updated_at,
                ],
            )
            .map_err(RoninDbError::CreateMemory)?;

        Ok(memory)
    }

    /// Fetches a memory by ID.
    pub fn get_memory(&self, id: &str) -> Result<Option<DbMemory>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, title, content, created_at, updated_at FROM memories WHERE id = ?1",
            )
            .map_err(RoninDbError::GetMemory)?;

        let mut rows = stmt.query(params![id]).map_err(RoninDbError::GetMemory)?;

        if let Some(row) = rows.next().map_err(RoninDbError::GetMemory)? {
            Ok(Some(DbMemory {
                id: row.get(0).map_err(RoninDbError::GetMemory)?,
                title: row.get(1).map_err(RoninDbError::GetMemory)?,
                content: row.get(2).map_err(RoninDbError::GetMemory)?,
                created_at: row.get(3).map_err(RoninDbError::GetMemory)?,
                updated_at: row.get(4).map_err(RoninDbError::GetMemory)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Lists all memories.
    pub fn list_all_memories(&self) -> Result<Vec<DbMemory>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, content, created_at, updated_at FROM memories ORDER BY created_at ASC, id ASC")
            .map_err(RoninDbError::ListMemories)?;

        let memories = stmt
            .query_map([], |row| {
                Ok(DbMemory {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .map_err(RoninDbError::ListMemories)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ListMemories)?;

        Ok(memories)
    }

    /// Updates a memory by ID.
    pub fn update_memory(&self, id: &str, title: &str, content: &str) -> Result<()> {
        let now = unix_timestamp_millis();
        self.conn
            .execute(
                "UPDATE memories SET title = ?2, content = ?3, updated_at = ?4 WHERE id = ?1",
                params![id, title, content, now],
            )
            .map_err(RoninDbError::UpdateMemory)?;
        Ok(())
    }

    /// Deletes a memory by ID.
    pub fn delete_memory(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM memories WHERE id = ?1", params![id])
            .map_err(RoninDbError::DeleteMemory)?;
        Ok(())
    }

    /// Creates and persists a new attachment.
    pub fn create_attachment(
        &self,
        message_id: &str,
        kind: &str,
        name: &str,
        mime_type: &str,
        content: Option<&str>,
        path: Option<&str>,
    ) -> Result<DbAttachment> {
        let now = unix_timestamp_millis();
        let attachment = DbAttachment {
            id: Uuid::now_v7().to_string(),
            message_id: message_id.to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
            mime_type: mime_type.to_string(),
            content: content.map(String::from),
            path: path.map(String::from),
            created_at: now,
        };

        self.conn
            .execute(
                "INSERT INTO attachments (id, message_id, kind, name, mime_type, content, path, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    attachment.id,
                    attachment.message_id,
                    attachment.kind,
                    attachment.name,
                    attachment.mime_type,
                    attachment.content,
                    attachment.path,
                    attachment.created_at,
                ],
            )
            .map_err(RoninDbError::CreateAttachment)?;

        Ok(attachment)
    }

    /// Fetches an attachment by ID.
    pub fn get_attachment(&self, id: &str) -> Result<Option<DbAttachment>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, message_id, kind, name, mime_type, content, path, created_at FROM attachments WHERE id = ?1")
            .map_err(RoninDbError::GetAttachment)?;

        let mut rows = stmt
            .query(params![id])
            .map_err(RoninDbError::GetAttachment)?;

        if let Some(row) = rows.next().map_err(RoninDbError::GetAttachment)? {
            Ok(Some(DbAttachment {
                id: row.get(0).map_err(RoninDbError::GetAttachment)?,
                message_id: row.get(1).map_err(RoninDbError::GetAttachment)?,
                kind: row.get(2).map_err(RoninDbError::GetAttachment)?,
                name: row.get(3).map_err(RoninDbError::GetAttachment)?,
                mime_type: row.get(4).map_err(RoninDbError::GetAttachment)?,
                content: row.get(5).map_err(RoninDbError::GetAttachment)?,
                path: row.get(6).map_err(RoninDbError::GetAttachment)?,
                created_at: row.get(7).map_err(RoninDbError::GetAttachment)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Lists attachments for a specific message.
    pub fn list_attachments_for_message(&self, message_id: &str) -> Result<Vec<DbAttachment>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, message_id, kind, name, mime_type, content, path, created_at FROM attachments WHERE message_id = ?1 ORDER BY created_at ASC, id ASC")
            .map_err(RoninDbError::ListAttachments)?;

        let attachments = stmt
            .query_map(params![message_id], |row| {
                Ok(DbAttachment {
                    id: row.get(0)?,
                    message_id: row.get(1)?,
                    kind: row.get(2)?,
                    name: row.get(3)?,
                    mime_type: row.get(4)?,
                    content: row.get(5)?,
                    path: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })
            .map_err(RoninDbError::ListAttachments)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(RoninDbError::ListAttachments)?;

        Ok(attachments)
    }

    /// Deletes an attachment by ID.
    pub fn delete_attachment(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM attachments WHERE id = ?1", params![id])
            .map_err(RoninDbError::DeleteAttachment)?;
        Ok(())
    }

    fn apply_migrations(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS schema_migrations (
                    version INTEGER PRIMARY KEY NOT NULL,
                    applied_at INTEGER NOT NULL
                );",
            )
            .map_err(RoninDbError::CreateSchemaMigrations)?;

        for (version, sql) in MIGRATIONS {
            let already_applied: bool = self
                .conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?1)",
                    [version],
                    |row| row.get(0),
                )
                .map_err(RoninDbError::CheckMigrationStatus)?;

            if already_applied {
                debug!(version, "migration already applied");
                continue;
            }

            info!(version, "applying database migration");
            let applied_at = unix_timestamp_millis();
            let tx = self
                .conn
                .unchecked_transaction()
                .map_err(RoninDbError::StartMigrationTransaction)?;
            tx.execute_batch(sql)
                .map_err(|source| RoninDbError::ApplyMigration {
                    version: *version,
                    source,
                })?;
            tx.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                params![version, applied_at],
            )
            .map_err(|source| RoninDbError::RecordMigration {
                version: *version,
                source,
            })?;
            tx.commit()
                .map_err(|source| RoninDbError::CommitMigration {
                    version: *version,
                    source,
                })?;
            info!(version, "database migration applied");
        }

        Ok(())
    }
}

fn unix_timestamp_millis() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp_nanos() as i64 / 1_000_000
}

/// Initializes Ronin's stderr tracing subscriber once per process.
pub fn init_tracing() {
    TRACING_INIT.call_once(|| {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .try_init();
    });
}
