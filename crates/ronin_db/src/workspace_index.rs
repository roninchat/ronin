//! Per-thread lexical workspace index corpus (SQLite FTS5).
//!
//! Lives under Ronin data (`workspace_indexes/{thread_id}.db`), separate from
//! chat DB rows. Search/attach gating is #74 — this module is build/store/delete.

use std::path::{Path, PathBuf};

use rusqlite::{params, Connection};

use crate::{Result, RoninDbError};

/// One document to upsert into the lexical corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalIndexDocument {
    /// Path relative to the workspace root.
    pub relative_path: String,
    /// Scrubbed UTF-8 body.
    pub body: String,
    /// Stored body byte length.
    pub byte_len: u64,
}

/// Open handle to a thread's on-disk lexical index database.
pub struct WorkspaceLexicalStore {
    conn: Connection,
    path: PathBuf,
}

impl WorkspaceLexicalStore {
    /// Opens (or creates) a lexical index DB at `path`, applying schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| RoninDbError::WorkspaceIndexStore {
                path: path.clone(),
                message: format!("create index dir: {e}"),
            })?;
        }
        let conn = Connection::open(&path).map_err(|source| RoninDbError::Open {
            path: path.clone(),
            source,
        })?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS docs (
                path TEXT PRIMARY KEY NOT NULL,
                body TEXT NOT NULL,
                byte_len INTEGER NOT NULL
             );
             CREATE VIRTUAL TABLE IF NOT EXISTS docs_fts USING fts5(path, body);",
        )
        .map_err(|source| RoninDbError::WorkspaceIndexStore {
            path: path.clone(),
            message: format!("init schema: {source}"),
        })?;
        Ok(Self { conn, path })
    }

    /// Absolute path of this store file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Replaces the entire corpus with `docs` (clear then insert).
    pub fn replace_documents(&self, docs: &[LexicalIndexDocument]) -> Result<()> {
        let tx = self.conn.unchecked_transaction().map_err(|source| {
            RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("begin replace: {source}"),
            }
        })?;
        tx.execute_batch(
            "DELETE FROM docs_fts;
             DELETE FROM docs;",
        )
        .map_err(|source| RoninDbError::WorkspaceIndexStore {
            path: self.path.clone(),
            message: format!("clear corpus: {source}"),
        })?;
        {
            let mut insert = tx
                .prepare("INSERT INTO docs (path, body, byte_len) VALUES (?1, ?2, ?3)")
                .map_err(|source| RoninDbError::WorkspaceIndexStore {
                    path: self.path.clone(),
                    message: format!("prepare insert: {source}"),
                })?;
            let mut insert_fts = tx
                .prepare("INSERT INTO docs_fts (path, body) VALUES (?1, ?2)")
                .map_err(|source| RoninDbError::WorkspaceIndexStore {
                    path: self.path.clone(),
                    message: format!("prepare fts insert: {source}"),
                })?;
            for doc in docs {
                insert
                    .execute(params![doc.relative_path, doc.body, doc.byte_len as i64])
                    .map_err(|source| RoninDbError::WorkspaceIndexStore {
                        path: self.path.clone(),
                        message: format!("insert doc: {source}"),
                    })?;
                insert_fts
                    .execute(params![doc.relative_path, doc.body])
                    .map_err(|source| RoninDbError::WorkspaceIndexStore {
                        path: self.path.clone(),
                        message: format!("insert fts: {source}"),
                    })?;
            }
        }
        tx.commit()
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("commit replace: {source}"),
            })?;
        Ok(())
    }

    /// Number of stored documents.
    pub fn entry_count(&self) -> Result<u64> {
        let n: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM docs", [], |row| row.get(0))
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("count: {source}"),
            })?;
        Ok(n as u64)
    }

    /// Whether a relative path exists in the corpus (build verification; not search UI).
    pub fn contains_path(&self, relative_path: &str) -> Result<bool> {
        let n: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM docs WHERE path = ?1",
                params![relative_path],
                |row| row.get(0),
            )
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("contains: {source}"),
            })?;
        Ok(n > 0)
    }

    /// Deletes all corpus rows (keeps empty DB file).
    pub fn clear(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "DELETE FROM docs_fts;
                 DELETE FROM docs;",
            )
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("clear: {source}"),
            })?;
        Ok(())
    }
}

/// Removes the lexical store file if it exists.
pub fn delete_workspace_lexical_store(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_file(path).map_err(|e| RoninDbError::WorkspaceIndexStore {
            path: path.to_path_buf(),
            message: format!("remove store: {e}"),
        })?;
    }
    Ok(())
}
