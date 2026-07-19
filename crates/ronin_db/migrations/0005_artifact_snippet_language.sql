-- Artifact kind + language metadata for code snippet artifacts.
ALTER TABLE artifacts ADD COLUMN kind TEXT NOT NULL DEFAULT 'document';
ALTER TABLE artifacts ADD COLUMN language TEXT;
