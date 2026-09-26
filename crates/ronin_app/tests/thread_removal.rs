//! Deleting and archiving threads from the shell.

use ronin_app::RoninShell;
use ronin_core::{MessageRole, RoninConfig, RoninPaths};
use tempfile::TempDir;

fn open_shell() -> (TempDir, RoninPaths, RoninShell) {
    let temp = TempDir::new().expect("temp");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths.clone()).expect("open");
    (temp, paths, shell)
}

fn thread_with_message(shell: &mut RoninShell, text: &str) -> String {
    let thread = shell.create_new_thread().expect("thread");
    shell
        .session()
        .create_message(&thread.id, MessageRole::User, text)
        .expect("message");
    thread.id
}

#[test]
fn delete_should_drop_thread_and_messages_and_select_a_neighbour() {
    let (_temp, paths, mut shell) = open_shell();
    let keep = thread_with_message(&mut shell, "keep me");
    let doomed = thread_with_message(&mut shell, "delete me");
    shell.select_thread(&doomed).expect("select");

    shell.remove_thread(&doomed, false).expect("delete");

    let state = shell.state();
    assert!(state.thread(&doomed).is_none());
    assert!(state.archived_threads.is_empty());
    assert_ne!(state.selected_thread_id.as_deref(), Some(doomed.as_str()));
    assert!(shell
        .session()
        .list_all_messages(&doomed)
        .expect("list")
        .is_empty());

    let reopened = RoninShell::open(paths).expect("reopen");
    assert!(reopened.state().thread(&doomed).is_none());
    assert!(reopened.state().thread(&keep).is_some());
}

#[test]
fn archive_should_hide_thread_from_sidebar_but_keep_it_openable() {
    let (_temp, paths, mut shell) = open_shell();
    let archived = thread_with_message(&mut shell, "archive me");

    shell.remove_thread(&archived, true).expect("archive");

    let state = shell.state();
    assert!(state.threads.iter().all(|t| t.id != archived));
    assert!(state.archived_threads.iter().any(|t| t.id == archived));
    assert_eq!(
        shell
            .session()
            .list_all_messages(&archived)
            .expect("list")
            .len(),
        1
    );

    shell
        .select_thread(&archived)
        .expect("open archived from search");
    assert!(shell.state().selected_is_archived());

    let reopened = RoninShell::open(paths).expect("reopen");
    assert!(reopened
        .state()
        .archived_threads
        .iter()
        .any(|t| t.id == archived));
    assert_ne!(
        reopened.state().selected_thread_id.as_deref(),
        Some(archived.as_str())
    );
}

#[test]
fn restore_should_return_archived_thread_to_sidebar() {
    let (_temp, _paths, mut shell) = open_shell();
    let id = thread_with_message(&mut shell, "come back");
    shell.remove_thread(&id, true).expect("archive");

    shell.restore_thread(&id).expect("restore");

    assert!(shell.state().threads.iter().any(|t| t.id == id));
    assert!(shell.state().archived_threads.is_empty());
}

#[test]
fn removing_the_last_visible_thread_should_leave_a_fresh_chat() {
    let (_temp, _paths, mut shell) = open_shell();
    let only: Vec<String> = shell.state().threads.iter().map(|t| t.id.clone()).collect();
    for id in &only {
        shell.remove_thread(id, false).expect("delete");
    }

    let state = shell.state();
    assert_eq!(state.threads.len(), 1);
    assert!(!only.contains(&state.threads[0].id));
    assert_eq!(
        state.selected_thread_id.as_deref(),
        Some(state.threads[0].id.as_str())
    );
}

#[test]
fn removal_settings_should_default_to_delete_and_search_archived() {
    let config = RoninConfig::default();
    assert!(!config.general.archive_instead_of_delete);
    assert!(config.general.search_archived);

    let (_temp, paths, _shell) = open_shell();
    std::fs::write(
        paths.config_dir.join("config.toml"),
        "[general]\narchive_instead_of_delete = true\nsearch_archived = false\n",
    )
    .expect("write config");
    let session = ronin_core::RoninSession::open(paths).expect("session");
    let loaded = session.load_config().expect("load");
    assert!(loaded.general.archive_instead_of_delete);
    assert!(!loaded.general.search_archived);
}
