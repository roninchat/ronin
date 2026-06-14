## What to build

Implement an OpenAI-compatible provider (`OpenAiCompatibleProvider`) that satisfies the existing `ChatProvider` trait. This means streaming chat, health check, and model listing against any OpenAI-compatible API endpoint. Integrate Secret Service for storing the API key securely, with an environment variable fallback. Share the single-generation guard with the existing Ollama provider.

The provider must stream responses through a channel pipeline to avoid blocking the GPUI main thread during token-by-token processing.

### Provider implementation

- `OpenAiCompatibleProvider` struct holding `base_url`, `api_key` (retrieved at request time, not stored in struct), and an HTTP client
- Implements `ChatProvider::stream_chat` — sends chat completions request, parses SSE, yields `ChatStreamEvent::Chunk` / `Error`
- Implements `OllamaProvider` trait methods (or a shared `ProviderHealth` trait if Extraction is warranted): `check_health`, `list_models`
- Default base URL: `https://api.openai.com/v1`
- Model list fetched from `/models` endpoint

### Channel pipeline for streaming

If the Ollama provider's direct `reqwest::Response` byte stream approach causes UI jank or blocking, introduce a `tokio::sync::mpsc` channel: spawn a background task that reads SSE chunks and sends them to the UI via channel. The UI polls the receiver on each frame.

### Secret Service integration

- Store API key via the Secret Service (use `secret-service` crate or `libsecret` FFI)
- On provider request, retrieve key from Secret Service; if not found, fall back to `OPENAI_API_KEY` environment variable
- Never log or persist the key in plaintext
- Error message when no key is available: "No API key found. Set OPENAI_API_KEY or add a key in settings."

### Config

Add `[openai]` section to `config.toml`:
```toml
[openai]
base_url = "https://api.openai.com/v1"
```

No `api_key` field in config — keys live only in Secret Service or env.

### Generation guard

Extend the existing single-generation guard (`generation_active` flag in `RoninShell`) to cover both Ollama and OpenAI providers. Only one generation runs at a time globally.

## Acceptance criteria

- [x] `OpenAiCompatibleProvider` streams chat responses from an OpenAI-compatible endpoint
- [x] Health check succeeds when endpoint is reachable, fails with clear error when offline
- [x] Model list returns available models from the API
- [x] API key retrieved from Secret Service, falls back to env var
- [x] No plaintext keys in config, logs, errors, or SQLite
- [x] Streaming uses channel pipeline if direct byte stream causes UI blocking
- [x] Single generation guard prevents concurrent generations across both providers
- [x] `cargo test --all-targets --all-features --locked` passes
- [x] Provider works against a local mock server in tests (no real API key needed for CI)

## Completed Summary

Successfully implemented `OpenAiCompatibleProvider` using TDD. The provider retrieves the API key securely via `secret-service` or `OPENAI_API_KEY` environment variable. Chat streaming leverages `reqwest` and a `mpsc` channel pipeline to prevent UI blocking. The HTTP client is reused across requests to optimize performance. Tests have been written to ensure `check_health`, `list_models`, and `stream_chat` work reliably using a local mock TCP server.

## Recommended Skills

- `rust-async` — `async fn stream_response`, `.next().await` on SSE byte lines, `tokio::spawn` for background provider tasks
- `rust-async-pattern` — channel pipeline (`tokio::sync::mpsc`) if direct stream blocks UI; Stream processing, backpressure control
- `rust-concurrency` — `Arc<Mutex<GenerationGuard>>` for single active generation across providers; `Send`/`Sync` bounds for GPUI background tasks
- `rust-auth` — Secret Service integration for API key storage; JWT/key retrieval patterns; env var fallback chain
