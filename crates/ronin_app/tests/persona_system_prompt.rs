//! Persona / system-prompt customization via the shell boundary.

use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, PersonaMode, RoninPaths, RONIN_SYSTEM_PROMPT,
};
use std::cell::RefCell;
use tempfile::TempDir;

struct CapturingFakeProvider {
    captured: RefCell<Option<ChatRequest>>,
}

impl ChatProvider for CapturingFakeProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        *self.captured.borrow_mut() = Some(request.clone());
        Ok(Box::new(
            vec![ChatStreamEvent::Chunk("ok".into())].into_iter(),
        ))
    }
}

fn open_shell(temp: &TempDir) -> RoninShell {
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    RoninShell::open(paths).expect("open shell")
}

#[test]
fn shell_should_expose_built_in_prompt_as_default_effective_system_prompt() {
    let temp = TempDir::new().expect("temp");
    let shell = open_shell(&temp);
    assert_eq!(shell.effective_system_prompt(), RONIN_SYSTEM_PROMPT);
    assert_eq!(shell.persona().mode, PersonaMode::Append);
    assert!(shell.persona().text.is_empty());
}

#[test]
fn shell_should_persist_persona_and_use_it_in_provider_requests() {
    let temp = TempDir::new().expect("temp");
    let mut shell = open_shell(&temp);

    shell
        .set_persona(PersonaMode::Append, "Always answer in haiku.")
        .expect("set persona");

    assert!(shell
        .effective_system_prompt()
        .contains("Always answer in haiku."));
    assert!(shell.effective_system_prompt().contains("Ronin"));

    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    let provider = CapturingFakeProvider {
        captured: RefCell::new(None),
    };
    shell
        .send_message_with_provider(&thread_id, "Hello", &provider, "test-model")
        .expect("send");

    let request = provider.captured.borrow();
    let request = request.as_ref().expect("captured");
    assert_eq!(request.messages[0].role, "system");
    assert!(
        request.messages[0]
            .content
            .contains("Always answer in haiku."),
        "provider must receive custom persona: {}",
        request.messages[0].content
    );
    assert!(
        request.messages[0].content.contains("Ronin"),
        "append mode must keep built-in prompt visible in the request"
    );
    assert_eq!(
        request.system_prompt.as_deref(),
        Some(request.messages[0].content.as_str())
    );
}

#[test]
fn shell_replace_persona_should_omit_built_in_from_provider_request() {
    let temp = TempDir::new().expect("temp");
    let mut shell = open_shell(&temp);

    shell
        .set_persona(PersonaMode::Replace, "You are a dictionary.")
        .expect("set persona");

    assert_eq!(shell.effective_system_prompt(), "You are a dictionary.");

    let thread_id = shell.state().selected_thread_id.clone().expect("thread");
    let provider = CapturingFakeProvider {
        captured: RefCell::new(None),
    };
    shell
        .send_message_with_provider(&thread_id, "Define ronin", &provider, "test-model")
        .expect("send");

    let request = provider.captured.borrow();
    let content = &request.as_ref().expect("captured").messages[0].content;
    assert_eq!(content, "You are a dictionary.");
    assert!(!content.contains("You are Ronin"));
}

#[test]
fn shell_import_export_provider_config_should_not_touch_persona() {
    let temp = TempDir::new().expect("temp");
    let mut shell = open_shell(&temp);

    shell
        .set_persona(PersonaMode::Append, "Keep my persona.")
        .expect("persona");
    shell
        .session()
        .save_config(&{
            let mut cfg = shell.session().load_config().unwrap();
            cfg.ollama.base_url = "http://before-export:11434".into();
            cfg
        })
        .expect("seed url");

    let path = temp.path().join("export.toml");
    shell.export_provider_config_to_file(&path).expect("export");

    shell
        .set_persona(PersonaMode::Replace, "Changed after export")
        .expect("change persona");
    shell
        .import_provider_config_from_file(&path)
        .expect("import");

    assert_eq!(shell.persona().mode, PersonaMode::Replace);
    assert_eq!(shell.persona().text, "Changed after export");
    assert_eq!(
        shell.session().load_config().unwrap().ollama.base_url,
        "http://before-export:11434"
    );
}
