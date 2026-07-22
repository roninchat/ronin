//! Per-thread lexical workspace index corpus (SQLite FTS5).
//!
//! Lives under Ronin data (`workspace_indexes/{thread_id}.db`), separate from
//! chat DB rows. Build/store/delete (#73) plus lexical search (#74). Attach into
//! chat still requires an explicit user gate in `ronin_core` / shell.

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

/// One FTS candidate hit (path + snippet). Never auto-merged into chat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalSearchHit {
    /// Path relative to the workspace root.
    pub relative_path: String,
    /// Short excerpt around the match for candidate UI.
    pub snippet: String,
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

    /// Returns the stored body for `relative_path`, if present.
    pub fn document_body(&self, relative_path: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT body FROM docs WHERE path = ?1")
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("prepare body: {source}"),
            })?;
        let mut rows = stmt.query(params![relative_path]).map_err(|source| {
            RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("query body: {source}"),
            }
        })?;
        match rows
            .next()
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("next body: {source}"),
            })? {
            Some(row) => {
                let body: String =
                    row.get(0)
                        .map_err(|source| RoninDbError::WorkspaceIndexStore {
                            path: self.path.clone(),
                            message: format!("read body: {source}"),
                        })?;
                Ok(Some(body))
            }
            None => Ok(None),
        }
    }

    /// Lexical FTS search over the corpus. Returns candidate hits only.
    ///
    /// Empty / whitespace-only queries yield no hits. Results are ranked by FTS
    /// relevance and capped by `limit`.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<LexicalSearchHit>> {
        let Some(match_query) = prepare_fts_query(query) else {
            return Ok(Vec::new());
        };
        if limit == 0 {
            return Ok(Vec::new());
        }
        let mut stmt = self
            .conn
            .prepare(
                "SELECT path, snippet(docs_fts, 1, '', '', '…', 32)
                 FROM docs_fts
                 WHERE docs_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            )
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("prepare search: {source}"),
            })?;
        let rows = stmt
            .query_map(params![match_query, limit as i64], |row| {
                Ok(LexicalSearchHit {
                    relative_path: row.get(0)?,
                    snippet: row.get(1)?,
                })
            })
            .map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("search: {source}"),
            })?;
        let mut hits = Vec::new();
        for row in rows {
            hits.push(row.map_err(|source| RoninDbError::WorkspaceIndexStore {
                path: self.path.clone(),
                message: format!("search row: {source}"),
            })?);
        }
        Ok(hits)
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

/// Build a safe FTS5 MATCH query from free-form user text.
///
/// Tokens are stripped to alphanumeric / `_` / `-` / `.` and matched as prefixes.
/// Returns `None` when nothing searchable remains.
pub fn prepare_fts_query(raw: &str) -> Option<String> {
    let tokens: Vec<String> = raw
        .split_whitespace()
        .filter_map(|token| {
            let cleaned: String = token
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
                .collect();
            if cleaned.is_empty() {
                None
            } else {
                Some(format!("{cleaned}*"))
            }
        })
        .collect();
    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join(" "))
    }
}
