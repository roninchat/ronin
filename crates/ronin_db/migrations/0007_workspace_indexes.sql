-- Per-thread lexical workspace index metadata (M3.0 #73).
-- Corpus bodies live in data_dir/workspace_indexes/{thread_id}.db (FTS).

CREATE TABLE workspace_indexes (
    thread_id TEXT PRIMARY KEY NOT NULL,
    phase TEXT NOT NULL,
    workspace_root TEXT,
    entry_count INTEGER NOT NULL DEFAULT 0,
    byte_count INTEGER NOT NULL DEFAULT 0,
    truncated INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    storage_relpath TEXT,
    built_at INTEGER,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE
);
