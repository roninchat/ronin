use ronin_app::RoninShell;
use ronin_core::{ChatProvider, ChatRequest, ChatStreamEvent, RoninPaths};
use std::cell::RefCell;
use tempfile::TempDir;

struct CapturingProvider {
    captured_requests: RefCell<Vec<ChatRequest>>,
}

impl ChatProvider for CapturingProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        self.captured_requests.borrow_mut().push(request.clone());
        Ok(Box::new(
            vec![ChatStreamEvent::Chunk("ok".to_string())].into_iter(),
        ))
    }
}

fn open_shell() -> (TempDir, RoninShell) {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open shell");
    (temp, shell)
}

#[test]
fn enabled_profile_memory_should_be_injected_into_provider_context() {
    let (_temp, mut shell) = open_shell();
    shell
        .create_profile_memory("Name", "Ada Lovelace")
        .expect("create profile");
    shell
        .create_memory("Scratch", "not profile")
        .expect("create regular");

    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    let provider = CapturingProvider {
        captured_requests: RefCell::new(Vec::new()),
    };
    shell
        .send_message_with_provider(&thread_id, "Hello", &provider, "test-model")
        .expect("send");

    let reqs = provider.captured_requests.borrow();
    assert_eq!(reqs.len(), 1);
    let context_msgs: Vec<_> = reqs[0]
        .messages
        .iter()
        .filter(|m| m.role == "system")
        .collect();
    assert!(
        context_msgs
            .iter()
            .any(|m| m.content.contains("Ada Lovelace") && m.content.contains("Profile memory")),
        "enabled profile memory should appear in system context: {context_msgs:?}"
    );
    assert!(
        !context_msgs
            .iter()
            .any(|m| m.content.contains("not profile")),
        "regular memories must not auto-inject"
    );
}

#[test]
fn disabled_profile_memory_should_be_excluded_from_provider_context() {
    let (_temp, mut shell) = open_shell();
    let mem = shell
        .create_profile_memory("Secret", "should-not-appear")
        .expect("create profile");
    shell.set_memory_enabled(&mem.id, false).expect("disable");

    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    let provider = CapturingProvider {
        captured_requests: RefCell::new(Vec::new()),
    };
    shell
        .send_message_with_provider(&thread_id, "Hello", &provider, "test-model")
        .expect("send");

    let reqs = provider.captured_requests.borrow();
    assert_eq!(reqs.len(), 1);
    for msg in &reqs[0].messages {
        assert!(
            !msg.content.contains("should-not-appear"),
            "disabled memory leaked into request: {}",
            msg.content
        );
    }
}

#[test]
fn memory_enable_and_profile_flags_should_persist_via_shell() {
    let (temp, shell) = open_shell();
    let mem = shell.create_memory("Prefs", "tea").expect("create");
    shell.set_memory_profile(&mem.id, true).expect("profile");
    shell.set_memory_enabled(&mem.id, false).expect("disable");

    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    drop(shell);
    let reopened = RoninShell::open(paths).expect("reopen");
    let listed = reopened.list_memories().expect("list");
    assert_eq!(listed.len(), 1);
    assert!(!listed[0].enabled);
    assert!(listed[0].is_profile);
}
