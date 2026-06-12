# Technical Choices

## M0 persistence tracer bullet

The first persistence tracer bullet uses stable, common Rust crates:

- `thiserror` for typed library errors that callers can inspect or present with context.
- `rusqlite` with bundled SQLite for local-first persistence without requiring a system SQLite install.
- `uuid` UUIDv7 for app-generated opaque, roughly time-ordered IDs stored as SQLite `TEXT`.
- `time` for UTC Unix millisecond timestamps at storage boundaries.
- `tracing` and `tracing-subscriber` for stderr startup/config/database migration events.
- `tempfile` for integration-style behavior tests against isolated local paths.
