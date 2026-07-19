//! Shell-level branch creation, edit, regenerate, and navigation.

use std::thread;
use std::time::Duration;

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, RoninPaths,
};
use tempfile::TempDir;

struct FakeProvider {
    chunks: Vec<ChatStreamEvent>,
}

impl ChatProvider for FakeProvider {
    fn stream_chat(
        &self,
        _request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        Ok(Box::new(self.chunks.clone().into_iter()))
    }
}

fn setup() -> (RoninShell, String, TempDir) {
    let temp = TempDir::new().unwrap();
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).unwrap();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    (shell, thread_id, temp)
}

fn drain(shell: &mut RoninShell) {
    thread::sleep(Duration::from_millis(50));
    while shell.poll_streaming() {
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn regenerate_should_preserve_original_assistant_as_sibling_branch() {
    let (mut shell, thread_id, _temp) = setup();
    shell
        .begin_streaming(
            &thread_id,
            Some("Hello"),
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("Old".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);

    let first_assistant = shell.state().messages.as_ref().unwrap()[1].id.clone();

    shell
        .regenerate_last_assistant(
            &thread_id,
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("New".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);

    let path = shell.state().messages.as_ref().unwrap();
    assert_eq!(path.len(), 2);
    assert_eq!(path[1].content, "New");
    assert_ne!(path[1].id, first_assistant);

    // Original still exists as sibling and is navigable.
    let siblings = shell
        .branch_siblings(&thread_id, &path[1].id)
        .expect("siblings");
    assert_eq!(siblings.len(), 2);
    assert!(siblings.iter().any(|m| m.id == first_assistant));
    assert!(siblings.iter().any(|m| m.content == "Old"));

    shell
        .switch_branch(&thread_id, &first_assistant)
        .expect("switch");
    let path = shell.state().messages.as_ref().unwrap();
    assert_eq!(path[1].id, first_assistant);
    assert_eq!(path[1].content, "Old");
}

#[test]
fn edit_user_message_should_fork_new_branch_preserving_original() {
    let (mut shell, thread_id, _temp) = setup();
    shell
        .begin_streaming(
            &thread_id,
            Some("Original question"),
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("Answer A".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);

    let original_user = shell.state().messages.as_ref().unwrap()[0].id.clone();
    let original_assistant = shell.state().messages.as_ref().unwrap()[1].id.clone();

    shell
        .edit_user_message_and_regenerate(
            &original_user,
            "Edited question",
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("Answer B".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);

    let path = shell.state().messages.as_ref().unwrap();
    assert_eq!(path.len(), 2);
    assert_eq!(path[0].content, "Edited question");
    assert_eq!(path[1].content, "Answer B");
    assert_ne!(path[0].id, original_user);

    let siblings = shell.branch_siblings(&thread_id, &path[0].id).unwrap();
    assert_eq!(siblings.len(), 2);
    assert!(siblings.iter().any(|m| m.id == original_user));

    shell.switch_branch(&thread_id, &original_user).unwrap();
    let path = shell.state().messages.as_ref().unwrap();
    assert_eq!(path[0].id, original_user);
    assert_eq!(path[0].content, "Original question");
    assert_eq!(path[1].id, original_assistant);
    assert_eq!(path[1].content, "Answer A");
}

#[test]
fn branch_state_should_persist_across_shell_reopen() {
    let (mut shell, thread_id, temp) = setup();
    shell
        .begin_streaming(
            &thread_id,
            Some("Hi"),
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("One".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);
    let leaf = shell.state().messages.as_ref().unwrap()[1].id.clone();

    shell
        .regenerate_last_assistant(
            &thread_id,
            Box::new(FakeProvider {
                chunks: vec![ChatStreamEvent::Chunk("Two".into())],
            }),
            "m",
        )
        .unwrap();
    drain(&mut shell);
    let new_leaf = shell.state().messages.as_ref().unwrap()[1].id.clone();
    assert_ne!(leaf, new_leaf);

    drop(shell);
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut reopened = RoninShell::open(paths).unwrap();
    reopened.select_thread(&thread_id).unwrap();
    let path = reopened.state().messages.as_ref().unwrap();
    assert_eq!(path[1].id, new_leaf);
    assert_eq!(path[1].content, "Two");

    reopened.switch_branch(&thread_id, &leaf).unwrap();
    let path = reopened.state().messages.as_ref().unwrap();
    assert_eq!(path[1].content, "One");
}
