-- Opt-in thread workspace root for relative @file / @folder resolution (M3.0 #70).
ALTER TABLE threads ADD COLUMN workspace_root TEXT;
