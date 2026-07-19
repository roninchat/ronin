-- Message parent links and per-thread active leaf for conversation branches.
ALTER TABLE messages ADD COLUMN parent_id TEXT REFERENCES messages(id) ON DELETE SET NULL;
ALTER TABLE threads ADD COLUMN active_leaf_id TEXT REFERENCES messages(id) ON DELETE SET NULL;
