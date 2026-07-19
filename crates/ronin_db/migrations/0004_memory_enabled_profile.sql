-- Memory enable/disable and profile group for always-on user context.
ALTER TABLE memories ADD COLUMN enabled INTEGER NOT NULL DEFAULT 1;
ALTER TABLE memories ADD COLUMN is_profile INTEGER NOT NULL DEFAULT 0;
