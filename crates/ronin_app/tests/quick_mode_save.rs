//! Quick mode: persist one-shot Q&A into chat threads.

use ronin_app::RoninShell;
use ronin_core::{MessageRole, RoninPaths};
use tempfile::TempDir;

fn paths(temp: &TempDir) -> RoninPaths {
    RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    }
}

#[test]
fn save_quick_exchange_should_create_thread_with_question_and_answer() {
    let temp = TempDir::new().expect("temp");
    let mut shell = RoninShell::open(paths(&temp)).expect("open");
    let before = shell.state().threads.len();

    let thread = shell
        .save_quick_exchange("What is Ronin?", "A local AI assistant.")
        .expect("save");

    assert_eq!(shell.state().threads.len(), before + 1);
    assert_eq!(
        shell.state().selected_thread_id.as_deref(),
        Some(thread.id.as_str())
    );
    let messages = shell.session().list_messages(&thread.id).expect("messages");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, MessageRole::User);
    assert_eq!(messages[0].content, "What is Ronin?");
    assert_eq!(messages[1].role, MessageRole::Assistant);
    assert_eq!(messages[1].content, "A local AI assistant.");
    assert_ne!(thread.title, "New Chat");
}

#[test]
fn save_quick_exchange_to_thread_should_append_to_existing() {
    let temp = TempDir::new().expect("temp");
    let mut shell = RoninShell::open(paths(&temp)).expect("open");
    let existing = shell.state().selected_thread_id.clone().expect("selected");
    shell.send_message(&existing, "Prior turn").expect("prior");

    shell
        .save_quick_exchange_to_thread(&existing, "Quick Q", "Quick A")
        .expect("append");

    let messages = shell.session().list_messages(&existing).expect("messages");
    assert!(messages.iter().any(|m| m.content == "Prior turn"));
    assert!(messages.iter().any(|m| m.content == "Quick Q"));
    assert!(messages.iter().any(|m| m.content == "Quick A"));
}
