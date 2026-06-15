use ronin_core::{clear_model_cache, get_cached_models, get_model_cache, CachedModels};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn test_cache_should_populate_on_first_fetch() {
    clear_model_cache("test_provider_1");

    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let fetch_fn = move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        Ok(vec!["model-a".to_string(), "model-b".to_string()])
    };

    let result = get_cached_models(
        "test_provider_1",
        Duration::from_secs(4),
        Duration::from_secs(5),
        fetch_fn,
    );

    assert!(result.is_ok());
    let models = result.unwrap();
    assert_eq!(models, vec!["model-a", "model-b"]);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_second_fetch_within_ttl_returns_cached_without_api_call() {
    clear_model_cache("test_provider_2");

    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let fetch_fn = move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        Ok(vec!["model-c".to_string()])
    };

    // First fetch
    let r1 = get_cached_models(
        "test_provider_2",
        Duration::from_secs(4),
        Duration::from_secs(5),
        fetch_fn.clone(),
    );
    assert_eq!(r1.unwrap(), vec!["model-c"]);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    // Second fetch
    let r2 = get_cached_models(
        "test_provider_2",
        Duration::from_secs(4),
        Duration::from_secs(5),
        fetch_fn,
    );
    assert_eq!(r2.unwrap(), vec!["model-c"]);
    assert_eq!(call_count.load(Ordering::SeqCst), 1); // call_count should still be 1!
}

#[test]
fn test_fetch_after_ttl_expiry_makes_fresh_api_call() {
    clear_model_cache("test_provider_3");

    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let fetch_fn = move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        Ok(vec!["model-d".to_string()])
    };

    // Populate cache with an expired entry
    {
        let mut cache = get_model_cache().blocking_write();
        cache.insert(
            "test_provider_3".to_string(),
            CachedModels {
                models: vec!["old-model".to_string()],
                fetched_at: Instant::now() - Duration::from_secs(10), // 10 seconds ago
                ttl: Duration::from_secs(5),                          // Expired after 5 seconds
                is_fetching: false,
            },
        );
    }

    let result = get_cached_models(
        "test_provider_3",
        Duration::from_secs(2), // stale at 2s
        Duration::from_secs(5), // expired at 5s
        fetch_fn,
    );

    assert_eq!(result.unwrap(), vec!["model-d"]);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_clear_model_cache_invalidates_cache() {
    clear_model_cache("test_provider_4");

    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let fetch_fn = move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        Ok(vec!["model-e".to_string()])
    };

    // First fetch
    let _ = get_cached_models(
        "test_provider_4",
        Duration::from_secs(4),
        Duration::from_secs(5),
        fetch_fn.clone(),
    );

    // Clear it
    clear_model_cache("test_provider_4");

    // Second fetch (should call fetch_fn again)
    let result = get_cached_models(
        "test_provider_4",
        Duration::from_secs(4),
        Duration::from_secs(5),
        fetch_fn,
    );

    assert_eq!(result.unwrap(), vec!["model-e"]);
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_stale_data_triggers_background_refresh_without_blocking() {
    clear_model_cache("test_provider_5");

    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let fetch_fn = move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        Ok(vec!["model-fresh".to_string()])
    };

    // Populate cache with stale but not expired entry
    {
        let mut cache = get_model_cache().blocking_write();
        cache.insert(
            "test_provider_5".to_string(),
            CachedModels {
                models: vec!["model-stale".to_string()],
                fetched_at: Instant::now() - Duration::from_secs(3), // 3s ago
                ttl: Duration::from_secs(5),                         // Expired after 5s
                is_fetching: false,
            },
        );
    }

    let result = get_cached_models(
        "test_provider_5",
        Duration::from_secs(2), // Stale after 2s (3s ago is stale!)
        Duration::from_secs(5), // Expired after 5s (3s ago is not expired!)
        fetch_fn,
    );

    // Should return stale models immediately
    assert_eq!(result.unwrap(), vec!["model-stale"]);

    // Give the background thread a tiny bit of time to complete
    std::thread::sleep(Duration::from_millis(50));

    // Verify background thread executed fetch_fn
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    // Verify cache has been updated to the fresh data
    {
        let cache = get_model_cache().blocking_read();
        let cached = cache.get("test_provider_5");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().models, vec!["model-fresh"]);
        assert!(!cached.unwrap().is_fetching);
    }
}
