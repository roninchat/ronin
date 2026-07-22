use ronin_app::RoninShell;
use ronin_core::RoninPaths;
use tempfile::TempDir;

fn open_shell() -> (TempDir, RoninShell) {
    let temp = TempDir::new().expect("temp");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open");
    (temp, shell)
}

#[test]
fn select_thread_provider_model_should_persist_per_thread() {
    let (temp, mut shell) = open_shell();
    let thread = shell.create_new_thread().expect("thread");

    shell
        .select_thread_provider_model(&thread.id, "ollama", "llama3.2")
        .expect("select");

    let (provider, model) = shell
        .resolve_thread_provider_and_model(&thread.id)
        .expect("resolve");
    assert_eq!(provider, "ollama");
    assert_eq!(model, "llama3.2");

    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    drop(shell);
    let reopened = RoninShell::open(paths).expect("reopen");
    let (p2, m2) = reopened
        .resolve_thread_provider_and_model(&thread.id)
        .expect("resolve after reopen");
    assert_eq!(p2, "ollama");
    assert_eq!(m2, "llama3.2");
}

#[test]
fn list_available_provider_models_should_return_vec_without_panic() {
    let (_temp, shell) = open_shell();
    let listed = shell.list_available_provider_models().expect("list models");
    // Environment-dependent: just assert the API is callable and well-formed.
    for (provider, models) in &listed {
        assert!(provider == "ollama" || provider == "openai");
        assert!(!models.is_empty());
    }
}
