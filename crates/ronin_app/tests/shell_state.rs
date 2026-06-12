use ronin_app::{ProviderStatus, RoninShell, VisualReuseDecision};
use ronin_core::{RoninPaths, RoninSession};
use tempfile::TempDir;

#[test]
fn shell_should_create_usable_selected_thread_when_none_exists() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let shell = RoninShell::open(paths).expect("open shell");
    let state = shell.state();

    assert_eq!(state.threads.len(), 1);
    assert_eq!(state.threads[0].title, "New Chat");
    assert_eq!(
        state.selected_thread_id.as_deref(),
        Some(state.threads[0].id.as_str())
    );
}

#[test]
fn shell_should_create_and_select_new_thread_from_sidebar_action() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths).expect("open shell");
    let initial_thread_id = shell
        .state()
        .selected_thread_id
        .clone()
        .expect("initial selected thread");

    let created = shell.create_new_thread().expect("create new thread");

    assert_ne!(created.id, initial_thread_id);
    assert_eq!(shell.state().threads.len(), 2);
    assert_eq!(
        shell.state().selected_thread_id.as_deref(),
        Some(created.id.as_str())
    );
}

#[test]
fn shell_should_select_thread_from_sidebar_action() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths).expect("open shell");
    let created = shell.create_new_thread().expect("create new thread");
    let first_id = shell.state().threads[0].id.clone();

    shell.select_thread(&first_id).expect("select first thread");

    assert_eq!(
        shell.state().selected_thread_id.as_deref(),
        Some(first_id.as_str())
    );
    assert_ne!(
        shell.state().selected_thread_id.as_deref(),
        Some(created.id.as_str())
    );
}

#[test]
fn shell_should_restore_persisted_threads_and_select_first_thread() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let session = RoninSession::open(paths.clone()).expect("open setup session");
    let first = session.create_thread().expect("create first thread");
    let second = session.create_thread().expect("create second thread");
    drop(session);

    let shell = RoninShell::open(paths).expect("open shell");
    let state = shell.state();

    assert_eq!(state.window_title, "Ronin");
    assert_eq!(state.threads, vec![first, second]);
    assert_eq!(
        state.selected_thread_id.as_deref(),
        Some(state.threads[0].id.as_str())
    );
    assert_eq!(state.provider_status, ProviderStatus::NotConfigured);
}

#[test]
fn shell_should_expose_m0_visual_direction_for_design_checkpoint() {
    let direction = RoninShell::m0_visual_direction();

    assert_eq!(
        direction.assessment_axes,
        ["rounded", "soft", "premium", "Linux-native", "Zed-grade"]
    );
    assert!(direction.required_changes_before_deeper_ui.is_empty());
    assert!(matches!(
        direction.reuse_decision,
        VisualReuseDecision::CustomGpui { .. }
    ));
}
