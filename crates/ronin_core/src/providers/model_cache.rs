//! Global TTL cache for provider model lists with stale-while-revalidate refresh.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::error::Result;

/// Cached list of models for a provider.
#[derive(Debug, Clone)]
pub struct CachedModels {
    /// List of model names.
    pub models: Vec<String>,
    /// Time when the models were fetched.
    pub fetched_at: Instant,
    /// Time-to-live duration for the cache.
    pub ttl: Duration,
    /// Whether a background fetch is currently in progress.
    pub is_fetching: bool,
}

static MODEL_CACHE: std::sync::OnceLock<RwLock<HashMap<String, CachedModels>>> =
    std::sync::OnceLock::new();

/// Returns a reference to the global model cache.
pub fn get_model_cache() -> &'static RwLock<HashMap<String, CachedModels>> {
    MODEL_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Clears the model list cache for the given provider.
pub fn clear_model_cache(provider_name: &str) {
    if let Some(lock) = MODEL_CACHE.get() {
        let mut cache = lock.blocking_write();
        cache.remove(provider_name);
    }
}

/// Helper to get cached models or fetch them using a closure.
pub fn get_cached_models<F>(
    provider_name: &str,
    stale_threshold: Duration,
    ttl: Duration,
    fetch_fn: F,
) -> Result<Vec<String>>
where
    F: Fn() -> Result<Vec<String>> + Send + Sync + 'static,
{
    let provider_key = provider_name.to_string();

    // 1. Read lock check (fast path)
    {
        let cache = get_model_cache().blocking_read();
        if let Some(cached) = cache.get(&provider_key) {
            let elapsed = cached.fetched_at.elapsed();
            if elapsed < stale_threshold {
                return Ok(cached.models.clone());
            }
        }
    }

    // 2. Write lock check for stale / expired / miss
    let mut cache = get_model_cache().blocking_write();
    if let Some(cached) = cache.get_mut(&provider_key) {
        let elapsed = cached.fetched_at.elapsed();
        if elapsed < stale_threshold {
            return Ok(cached.models.clone());
        } else if elapsed < ttl {
            // Stale but not expired. Spawn background refresh.
            if !cached.is_fetching {
                cached.is_fetching = true;
                let provider_key_clone = provider_key.clone();
                std::thread::spawn(move || match fetch_fn() {
                    Ok(fresh_models) => {
                        let mut cache = get_model_cache().blocking_write();
                        cache.insert(
                            provider_key_clone,
                            CachedModels {
                                models: fresh_models,
                                fetched_at: Instant::now(),
                                ttl,
                                is_fetching: false,
                            },
                        );
                    }
                    Err(e) => {
                        let mut cache = get_model_cache().blocking_write();
                        if let Some(cached) = cache.get_mut(&provider_key_clone) {
                            cached.is_fetching = false;
                        }
                        tracing::warn!(
                            "Background refresh failed for {}: {:?}",
                            provider_key_clone,
                            e
                        );
                    }
                });
            }
            return Ok(cached.models.clone());
        }
    }

    // 3. Expired or Miss. Do blocking fetch.
    drop(cache);

    let fresh_models = fetch_fn()?;

    let mut cache = get_model_cache().blocking_write();
    cache.insert(
        provider_key,
        CachedModels {
            models: fresh_models.clone(),
            fetched_at: Instant::now(),
            ttl,
            is_fetching: false,
        },
    );

    Ok(fresh_models)
}
