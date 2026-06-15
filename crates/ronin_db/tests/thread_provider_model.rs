use ronin_db::RoninDb;
use tempfile::TempDir;

fn open_test_db() -> (RoninDb, TempDir) {
    let temp = TempDir::new().expect("temp dir");
    let db = RoninDb::open(temp.path().join("test.db")).expect("open db");
    (db, temp)
}

#[test]
fn create_thread_should_store_provider_and_model_as_none_by_default() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");

    assert!(thread.provider.is_none());
    assert!(thread.model.is_none());

    let threads = db.list_threads().expect("list threads");
    assert_eq!(threads.len(), 1);
    assert!(threads[0].provider.is_none());
    assert!(threads[0].model.is_none());
}

#[test]
fn create_thread_with_provider_should_round_trip_provider_and_model() {
    let (db, _temp) = open_test_db();
    let thread = db
        .create_thread_with_provider(Some("ollama"), Some("llama3.2"))
        .expect("create thread");

    assert_eq!(thread.provider.as_deref(), Some("ollama"));
    assert_eq!(thread.model.as_deref(), Some("llama3.2"));

    let threads = db.list_threads().expect("list threads");
    assert_eq!(threads.len(), 1);
    assert_eq!(threads[0].provider.as_deref(), Some("ollama"));
    assert_eq!(threads[0].model.as_deref(), Some("llama3.2"));
}

#[test]
fn update_thread_provider_should_persist_new_provider() {
    let (db, _temp) = open_test_db();
    let thread = db.create_thread().expect("create thread");

    db.update_thread_provider(&thread.id, Some("openai"))
        .expect("update provider");

    let threads = db.list_threads().expect("list threads");
    assert_eq!(threads[0].provider.as_deref(), Some("openai"));
}

#[test]
fn update_thread_model_should_persist_new_model() {
    let (db, _temp) = open_test_db();
    let thread = db
        .create_thread_with_provider(Some("ollama"), Some("llama3.2"))
        .expect("create thread");

    db.update_thread_model(&thread.id, Some("mistral"))
        .expect("update model");

    let threads = db.list_threads().expect("list threads");
    assert_eq!(threads[0].model.as_deref(), Some("mistral"));
    assert_eq!(
        threads[0].provider.as_deref(),
        Some("ollama"),
        "provider should be unchanged"
    );
}

#[test]
fn update_thread_provider_should_allow_clearing_to_none() {
    let (db, _temp) = open_test_db();
    let thread = db
        .create_thread_with_provider(Some("ollama"), Some("llama3.2"))
        .expect("create thread");

    db.update_thread_provider(&thread.id, None)
        .expect("clear provider");

    let threads = db.list_threads().expect("list threads");
    assert!(threads[0].provider.is_none());
}
