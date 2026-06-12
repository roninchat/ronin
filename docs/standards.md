# Rust Coding Standards

This document defines the Rust coding rules for this repository. It is based on Apollo GraphQL's Rust best practices handbook.

## Baseline Expectations

- Write idiomatic, readable Rust first; optimize only with evidence.
- Keep changes small, focused, and consistent with surrounding code.
- Prefer compile-time safety over runtime checks when the model is simple enough to encode in types.
- Run formatting, linting, and tests before considering work complete.

Required validation before submitting Rust changes:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
```

For performance-sensitive changes, also benchmark or profile in release mode:

```sh
cargo test --release
```

## Product Invariants

Ronin is a native Linux intelligence workbench, not a browser chat clone.

- Local-first by default: chat history and product data live locally unless an explicit provider request is made.
- User controls context: do not attach files, clipboard, screenshots, artifacts, memories, or other context silently.
- Trust before agency: V1 must not introduce autonomous file writes, shell execution, browser automation, hidden memory extraction, or background indexing.
- Secrets must never be stored in plaintext config, SQLite, logs, errors, snapshots, or test fixtures.
- Prefer useful end-to-end vertical slices over broad horizontal scaffolding.
- Native UI work should preserve a fast, polished, Linux-native feel.

## Workspace and Crate Boundaries

Keep dependencies flowing through explicit product boundaries. The PRD target architecture is:

- `crates/ronin`: binary, CLI parsing, process startup, and native app launch.
- `crates/ronin_app`: UI-facing application shell and product behavior. It owns user actions and session orchestration, but must not depend on GPUI rendering APIs.
- `crates/ronin_core`: domain types and public domain concepts. Keep it independent of GPUI, SQLite, Zed crates, and HTTP clients.
- `crates/ronin_db`: SQLite schema, migrations, database connections, and repository-style persistence APIs. Do not expose `rusqlite::Connection` outside this crate.
- Future `ronin_providers`: Ronin-owned provider traits and provider adapters. Depend on domain types rather than UI or database internals.
- Future `ronin_ui`: GPUI views and reusable UI components. It should call `ronin_app` instead of orchestrating product behavior directly.

When existing M0 code has not fully reached this target boundary, prefer changes that move toward it and avoid deepening cross-layer coupling. Until future crates exist, keep the same boundary discipline inside existing crates. For example, GPUI rendering currently lives at the binary/native boundary, while app state and user actions belong behind testable `ronin_app`/`ronin_core` interfaces.

Prefer small concrete APIs such as `RoninDb` and `RoninSession` over broad traits until tests, alternate implementations, or runtime polymorphism justify abstraction.

## Ronin Data, Config, and Persistence

- Use XDG-style Linux paths:
  - config: `~/.config/ronin/config.toml`
  - data: `~/.local/share/ronin/ronin.db`
  - cache/logs/temp later: `~/.cache/ronin/`
- TOML config is for preferences and non-secret provider settings, not chat/product data.
- SQLite is for product data: threads, messages, attachments metadata, artifacts, memories, and related state.
- Use `rusqlite` behind `ronin_db`; no other crate should depend on concrete SQLite connections.
- Use simple numbered SQL migrations under the owning crate's `migrations/` directory.
- Apply pending migrations on startup and return clear typed errors if migration fails.
- Track applied migrations in `schema_migrations` with `version` and `applied_at`.
- Keep M0 schema focused on implemented behavior; add attachments, artifacts, and memories in later migrations when their workflows exist.
- Store application-generated IDs as opaque SQLite `TEXT` values. Prefer UUIDv7 when crate support is straightforward.
- Prefer domain ID newtypes/wrappers over passing raw strings everywhere when the model grows enough to justify it.
- Store timestamps at persistence boundaries as UTC Unix milliseconds in SQLite `INTEGER` columns. Convert to richer time types at domain boundaries when useful.
- Persist streaming assistant messages with debounced partial updates; do not write every provider token/chunk to SQLite.
- Store sanitized, user-actionable failure reasons only. Do not store secrets or raw provider/debug payloads in `error_message`.

## Providers, Context, and Secrets

- Define Ronin-owned provider traits before binding product behavior to any concrete provider implementation.
- Ollama is the local-first provider path; OpenAI-compatible providers require explicit configuration and secret handling.
- API keys belong in Linux Secret Service or explicit environment fallback, never plaintext config or SQLite.
- Provider requests must include only explicit user conversation/context plus Ronin's built-in capability-boundary system prompt.
- Do not include hidden user context in system prompts or provider payloads.
- Apply context caps before provider requests and make omitted context visible to the user when it affects the request.

## Tracing and Diagnostics

- Use `tracing` for library/application diagnostics; avoid `println!`/`eprintln!` outside binary-level user-facing fatal messages.
- In M0, emit tracing to stderr. Persistent file logs are deferred.
- Prefer structured fields for paths, IDs, migration versions, provider status, and operation context.
- Log useful lifecycle events: app startup, config load/create, database open/migrations, provider status, model list failures, stream start/end/cancel/failure.
- Never log prompt/message content, API keys, secrets, raw provider payloads, or URLs containing credentials.

## Formatting and Style

- Use `rustfmt`; do not hand-format around it.
- Prefer clear names over abbreviations.
- Keep functions small enough to explain one operation or decision.
- Prefer expression-oriented Rust when it improves clarity.
- Avoid clever code that obscures ownership, control flow, or error propagation.

## Ownership, Borrowing, and Cloning

- Prefer borrowing over cloning.
- Use `&T` instead of taking ownership when the function only reads the value.
- Use `&str` instead of `String` in parameters unless ownership is required.
- Use `&[T]` instead of `Vec<T>` in parameters unless ownership or vector-specific mutation is required.
- Avoid `.clone()` as a shortcut around borrow checker issues. Clone only when ownership is actually needed or the clone is intentionally cheaper/simpler than a more complex lifetime model.
- Avoid cloning inside loops unless justified.
- Small `Copy` types may be passed by value. As a rule of thumb, simple values up to roughly 24 bytes are fine by value.
- Use `Cow<'_, T>` when an API may return either borrowed or owned data and avoiding allocation matters.

Preferred:

```rust
fn render_name(name: &str) -> String {
    format!("user:{name}")
}

fn sum(values: &[u64]) -> u64 {
    values.iter().sum()
}
```

Avoid:

```rust
fn render_name(name: String) -> String {
    format!("user:{name}")
}

fn sum(values: Vec<u64>) -> u64 {
    values.iter().sum()
}
```

## Error Handling

- Return `Result<T, E>` for fallible operations.
- Do not use `panic!`, `unwrap()`, or `expect()` in production code.
- `unwrap()` and `expect()` are acceptable in tests when they keep the test focused and failure output remains clear.
- Prefer the `?` operator for straightforward error propagation.
- Use structured errors for recoverable/domain failures.
- Use `thiserror` for library-style crates and domain errors.
- Use `anyhow` only at binary/application boundaries where errors are reported rather than matched by callers.
- Include enough context for operators and developers to understand what failed.

Preferred:

```rust
fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}
```

Avoid:

```rust
fn load_config(path: &Path) -> Config {
    let contents = std::fs::read_to_string(path).unwrap();
    toml::from_str(&contents).unwrap()
}
```

## Linting

- Treat Clippy warnings as build failures.
- Use the required Clippy command from this document before submitting changes.
- Pay particular attention to:
  - `redundant_clone`
  - `large_enum_variant`
  - `needless_collect`
  - `clippy::perf`
- Prefer fixing lints over suppressing them.
- If a suppression is necessary, use `#[expect(clippy::lint_name)]` instead of `#[allow(...)]`, and include a justification comment.

Example:

```rust
// The generated schema shape intentionally mirrors the external API.
#[expect(clippy::large_enum_variant)]
enum ApiNode {
    Small(SmallNode),
    Large(LargeNode),
}
```

## Performance

- Do not assume performance; measure it.
- Benchmark and profile with `--release`.
- Avoid redundant allocation and cloning, especially in loops and hot paths.
- Prefer iterators and zero-cost abstractions when they keep code clear.
- Avoid intermediate `.collect()` calls unless a collection is actually needed.
- Consider boxing large enum variants if `clippy::large_enum_variant` reports a real size problem.
- Choose stack allocation by default; use heap allocation intentionally for ownership, dynamic sizing, or large values.

Preferred:

```rust
let active_count = users.iter().filter(|user| user.active).count();
```

Avoid:

```rust
let active_users: Vec<_> = users.iter().filter(|user| user.active).collect();
let active_count = active_users.len();
```

## Iterators and Collections

- Prefer iterator adapters over manual loops when they improve readability.
- Prefer `.iter()` when borrowing collection items.
- Use `.into_iter()` when consuming the collection is intended.
- Avoid collecting into `Vec` just to iterate again.
- Use collection types that match the access pattern: `Vec` for ordered sequences, `HashMap`/`BTreeMap` for lookup, `HashSet`/`BTreeSet` for uniqueness.

## Generics, Traits, and Dispatch

- Prefer generics and static dispatch for performance-critical paths.
- Use `dyn Trait` when runtime polymorphism is required, such as heterogeneous collections or plugin-style boundaries.
- Box trait objects at API boundaries rather than deep inside business logic.
- Keep trait APIs minimal and behavior-focused.
- Do not introduce traits only to mock code unless the abstraction is useful in production too.

## Type-State and Compile-Time Invariants

Use the type-state pattern when it makes invalid states unrepresentable without adding excessive complexity.

Good candidates:

- Connection/session lifecycles.
- Builders with required fields.
- Resources that must be initialized before use.
- State transitions where invalid calls should fail at compile time.

Example:

```rust
use std::marker::PhantomData;

struct Connection<State> {
    id: String,
    _state: PhantomData<State>,
}

struct Disconnected;
struct Connected;

impl Connection<Disconnected> {
    fn connect(self) -> Connection<Connected> {
        Connection {
            id: self.id,
            _state: PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn send(&self, data: &[u8]) {
        // only connected sessions can send
        let _ = data;
    }
}
```

Do not use type-state when a simple enum, validation function, or `Result` would be clearer.

## Pointers, Sharing, and Concurrency

- Prefer plain references (`&T`, `&mut T`) when ownership does not need to be shared.
- Use `Box<T>` for owned heap allocation, recursive types, or trait objects.
- Use `Rc<T>` only for single-threaded shared ownership.
- Use `Arc<T>` for thread-safe shared ownership.
- Use `RefCell<T>`/`Mutex<T>`/`RwLock<T>` only when interior mutability is necessary.
- Be explicit about thread-safety expectations. Understand whether types are `Send` and `Sync` before moving them across threads.
- Avoid holding locks across await points or expensive operations.

## Comments and Documentation

- Comments should explain why, not restate what the code does.
- Public APIs should have `///` documentation explaining purpose, behavior, and important failure modes.
- Include doc tests for public APIs when an example clarifies usage.
- Every `TODO` must reference an issue or tracking item.

Preferred:

```rust
// TODO(#42): Replace polling with event-driven updates once the watcher API lands.
```

Avoid:

```rust
// TODO: fix later
```

Library crates in this workspace must enable missing-docs enforcement at the crate boundary:

```rust
#![deny(missing_docs)]
```

Document public types, fields, functions, and important failure modes. Keep documentation aligned with the crate boundary: persistence docs in `ronin_db`, domain/session docs in `ronin_core`, UI-facing behavior docs in `ronin_app`, and launcher docs in `ronin`.

## Testing

- Build product behavior with vertical TDD tracer bullets when changing core flows: write one behavior test, watch it fail, implement the minimum code to pass, then refactor after green.
- Name tests descriptively: `function_should_expected_behavior_when_condition()`.
- Test behavior, not implementation details.
- Use public interfaces where possible so tests survive internal refactors.
- Prefer one logical assertion per test when practical. Multiple assertions are acceptable when they verify one behavior or state transition.
- Use table-driven tests for multiple input/output cases.
- Use doc tests for public API examples.
- Snapshot testing may be used for generated output or large structured responses when textual diffs are useful.
- Tests may use `unwrap()`/`expect()` to keep setup concise, but assertion failures should remain understandable.
- For filesystem, config, persistence, and shell-state behavior, prefer integration-style tests using `tempfile::TempDir` so tests never touch real user config/data paths.
- GPUI rendering may be manually validated when automated UI tests are impractical, but app logic behind UI actions should be covered through testable public interfaces.

Example:

```rust
#[test]
fn parse_port_should_return_error_when_value_is_not_numeric() {
    let result = parse_port("abc");

    assert!(matches!(result, Err(ParsePortError::InvalidNumber(_))));
}
```

## Dependency and API Boundaries

- Prefer standard library types and existing workspace dependencies before adding new crates.
- Add dependencies only when they reduce complexity enough to justify maintenance cost.
- Keep shared dependencies in `[workspace.dependencies]` when they are used by more than one crate.
- Use current project choices unless there is a strong reason to change:
  - `thiserror` for typed library/domain errors.
  - `rusqlite` with bundled SQLite for local-first persistence.
  - `uuid` UUIDv7 for opaque app-generated IDs.
  - `time` for UTC timestamp conversion at storage boundaries.
  - `tracing`/`tracing-subscriber` for diagnostics.
  - `tempfile` for isolated filesystem integration tests.
- Keep domain errors and domain types close to the crate that owns the behavior.
- Convert broad application errors into typed domain errors before crossing library boundaries.
- Avoid leaking implementation-specific types through public APIs unless intentional.
- Track copied or adapted GPL Zed code in `docs/zed-reuse.md` with source path, commit, reuse type, and notes.

## Unsafe Rust

- Avoid `unsafe` unless there is no practical safe alternative.
- Every `unsafe` block must include a `SAFETY:` comment explaining the invariants that make it sound.
- Keep unsafe blocks as small as possible.
- Wrap unsafe internals in safe APIs when possible.
- Add tests around unsafe behavior, but remember tests do not prove soundness.

Example:

```rust
// SAFETY: `ptr` is created from a valid mutable reference above and remains
// exclusive for the duration of this block.
unsafe {
    *ptr = 1;
}
```

## Review Checklist

Before merging Rust code, verify:

- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo clippy --all-targets --all-features --locked -- -D warnings` passes.
- [ ] `cargo test --all-targets --all-features --locked` passes.
- [ ] Public APIs use borrowed parameters where ownership is not required.
- [ ] No production `unwrap()`, `expect()`, or `panic!` were introduced.
- [ ] Errors are typed and contextual where callers need to react.
- [ ] Clones and allocations are intentional.
- [ ] Tests cover the behavior changed through public interfaces where practical.
- [ ] Filesystem/config/persistence tests use isolated temp paths, not real user paths.
- [ ] Product changes preserve explicit context, local-first behavior, and no hidden memory/context collection.
- [ ] Provider, logging, and persistence changes do not expose secrets, prompts, raw payloads, or credential-bearing URLs.
- [ ] SQLite changes include focused migrations and persistence tests when schema behavior changes.
- [ ] Comments explain non-obvious rationale rather than repeating the code.
