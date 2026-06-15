use ronin_app::RoninShell;
use ronin_core::{ChatProvider, ChatRequest, ChatStreamEvent, RoninPaths};
use std::cell::RefCell;
use tempfile::TempDir;

struct MultiTurnFakeProvider {
    responses: RefCell<Vec<String>>,
    captured_requests: RefCell<Vec<ChatRequest>>,
}

impl ChatProvider for MultiTurnFakeProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        self.captured_requests.borrow_mut().push(request.clone());
        let next_response = self.responses.borrow_mut().remove(0);
        Ok(Box::new(
            vec![ChatStreamEvent::Chunk(next_response)].into_iter(),
        ))
    }
}

#[test]
fn shell_should_execute_tool_calls_and_perform_multi_turn_generation() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };

    let mut shell = RoninShell::open(paths).expect("open shell");

    // Create some memories via shell
    let name_mem = shell
        .create_memory("Name", "Charlie")
        .expect("create memory");
    shell.create_memory("Color", "Blue").expect("create memory");

    let thread_id = shell.state().selected_thread_id.clone().expect("thread id");

    let provider = MultiTurnFakeProvider {
        responses: RefCell::new(vec![
            "Let's check memories. [TOOL_CALL: list_memories]".to_string(),
            format!(
                "Found it! [TOOL_CALL: get_memory, id: \"{}\"]",
                name_mem.id.0
            ),
            "Your name is Charlie!".to_string(),
        ]),
        captured_requests: RefCell::new(Vec::new()),
    };

    shell
        .send_message_with_provider(&thread_id, "What is my name?", &provider, "test-model")
        .expect("send message");

    // Let's assert on captured requests
    let reqs = provider.captured_requests.borrow();
    assert_eq!(reqs.len(), 3, "Should have performed 3 turns");

    // Turn 1 request: system prompt + user message. No memories list yet.
    assert_eq!(reqs[0].messages.len(), 2);
    assert!(!reqs[0].messages[0].content.contains("Charlie"));

    // Turn 2 request: contains assistant's tool call and the tool result of list_memories
    assert_eq!(reqs[1].messages.len(), 4);
    assert_eq!(reqs[1].messages[2].role, "assistant");
    assert!(reqs[1].messages[2]
        .content
        .contains("[TOOL_CALL: list_memories]"));
    assert_eq!(reqs[1].messages[3].role, "system");
    assert!(reqs[1].messages[3]
        .content
        .contains("[TOOL_RESULT: list_memories"));
    assert!(reqs[1].messages[3].content.contains("Name"));

    // Turn 3 request: contains assistant's tool call get_memory and the result containing Charlie
    assert_eq!(reqs[2].messages.len(), 6);
    assert_eq!(reqs[2].messages[4].role, "assistant");
    assert!(reqs[2].messages[4]
        .content
        .contains("[TOOL_CALL: get_memory"));
    assert_eq!(reqs[2].messages[5].role, "system");
    assert!(reqs[2].messages[5]
        .content
        .contains("[TOOL_RESULT: get_memory"));
    assert!(reqs[2].messages[5].content.contains("Charlie"));

    // Finally, check that the assistant messages stored in the shell state have the entire history
    let state = shell.state();
    let msgs = state.messages.as_ref().expect("messages");
    assert_eq!(msgs.len(), 6, "Should have 6 messages in thread");
    assert_eq!(msgs[0].role, ronin_core::MessageRole::User);
    assert_eq!(msgs[1].role, ronin_core::MessageRole::Assistant);
    assert_eq!(msgs[2].role, ronin_core::MessageRole::System);
    assert_eq!(msgs[3].role, ronin_core::MessageRole::Assistant);
    assert_eq!(msgs[4].role, ronin_core::MessageRole::System);
    assert_eq!(msgs[5].role, ronin_core::MessageRole::Assistant);

    assert!(msgs[1].content.contains("Let's check memories."));
    assert!(msgs[2].content.contains("[TOOL_RESULT: list_memories"));
    assert!(msgs[3].content.contains("Found it!"));
    assert!(msgs[4].content.contains("[TOOL_RESULT: get_memory"));
    assert!(msgs[5].content.contains("Your name is Charlie!"));
}
