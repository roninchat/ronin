//! Thread rename persistence and model title-generation seams.

use ronin_app::{
    build_title_generation_request, collect_streamed_title, derive_thread_title,
    may_apply_auto_title, sanitize_generated_title, RoninShell,
};
use ronin_core::{
    ChatProvider, ChatRequest, ChatStreamEvent, MessageRole, RoninConfig, RoninPaths,
};
use std::cell::RefCell;
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
fn derive_thread_title_should_collapse_and_truncate() {
    let title = derive_thread_title("  Hello,   Ronin!\nMore  ");
    assert_eq!(title, "Hello, Ronin!");
    let long = derive_thread_title(&format!("{}{}", "A".repeat(70), " tail"));
    assert!(long.len() <= 60);
    assert!(long.ends_with("..."));
}

#[test]
fn sanitize_generated_title_should_strip_quotes_and_fluff() {
    assert_eq!(
        sanitize_generated_title("  \"Rust Tips\"  \nmore"),
        Some("Rust Tips".into())
    );
    assert_eq!(sanitize_generated_title("   "), None);
    assert_eq!(
        sanitize_generated_title("Title: Deep Learning Basics"),
        Some("Deep Learning Basics".into())
    );
}

#[test]
fn title_generation_request_should_be_lightweight() {
    let req = build_title_generation_request(
        "test-model",
        &"u".repeat(2_000),
        &"a".repeat(2_000),
    );
    assert_eq!(req.model, "test-model");
    let total: usize = req.messages.iter().map(|m| m.content.len()).sum();
    assert!(
        total < 2_500,
        "request should truncate excerpts, got {total} chars"
    );
    assert!(
        req.messages.iter().any(|m| m.role == "system"),
        "needs short system instruction"
    );
    let joined = req
        .messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        joined.to_lowercase().contains("title"),
        "prompt should ask for a title"
    );
}

#[test]
fn collect_streamed_title_should_join_chunks() {
    let events = [
        ChatStreamEvent::Chunk("Rust ".into()),
        ChatStreamEvent::Chunk("Tips".into()),
    ];
    assert_eq!(collect_streamed_title(events.into_iter()), "Rust Tips");
}

#[test]
fn rename_thread_should_persist_immediately() {
    let (temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();

    shell
        .rename_thread(&thread_id, "  My Custom Title  ")
        .expect("rename");
    assert_eq!(shell.state().threads[0].title, "My Custom Title");

    drop(shell);
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let reopened = RoninShell::open(paths).expect("reopen");
    let title = reopened
        .state()
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .map(|t| t.title.as_str())
        .unwrap();
    assert_eq!(title, "My Custom Title");
}

#[test]
fn may_apply_auto_title_should_respect_manual_rename_and_provisional() {
    let first = "Hello, Ronin!";
    let provisional = derive_thread_title(first);
    assert!(may_apply_auto_title(&provisional, first, false));
    assert!(may_apply_auto_title("New Chat", first, false));
    assert!(!may_apply_auto_title("My Custom Title", first, false));
    assert!(!may_apply_auto_title(&provisional, first, true));
}

#[test]
fn rename_thread_should_mark_manual_and_block_auto_apply() {
    let (_temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    shell
        .send_message(&thread_id, "Hello, Ronin!")
        .expect("send");
    let provisional = shell.state().threads[0].title.clone();
    assert!(may_apply_auto_title(
        &provisional,
        "Hello, Ronin!",
        shell.is_manual_title(&thread_id)
    ));

    shell.rename_thread(&thread_id, "Locked").expect("rename");
    assert!(shell.is_manual_title(&thread_id));
    assert!(!may_apply_auto_title(
        "Locked",
        "Hello, Ronin!",
        shell.is_manual_title(&thread_id)
    ));
}

#[test]
fn auto_title_config_defaults_true_and_can_disable() {
    let (temp, shell) = open_shell();
    let cfg = shell.session().load_config().expect("cfg");
    assert!(cfg.general.auto_title);

    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::write(
        paths.config_dir.join("config.toml"),
        "[general]\nauto_title = false\n",
    )
    .unwrap();
    let session = ronin_core::RoninSession::open(paths).expect("session");
    let cfg = session.load_config().expect("load");
    assert!(!cfg.general.auto_title);
}

struct TitleCapturingProvider {
    chunks: Vec<ChatStreamEvent>,
    captured: RefCell<Option<ChatRequest>>,
}

impl ChatProvider for TitleCapturingProvider {
    fn stream_chat(
        &self,
        request: &ChatRequest,
    ) -> ronin_core::Result<Box<dyn Iterator<Item = ChatStreamEvent> + '_>> {
        *self.captured.borrow_mut() = Some(request.clone());
        Ok(Box::new(self.chunks.clone().into_iter()))
    }
}

#[test]
fn apply_model_title_should_replace_provisional_when_allowed() {
    let (_temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    shell
        .send_message(&thread_id, "How do I install Rust?")
        .expect("send");
    assert_eq!(
        shell.state().threads[0].title,
        "How do I install Rust?"
    );

    // Simulate completed first exchange messages already present from send_message
    // (user only). Create assistant reply so exchange exists.
    shell
        .session()
        .create_message(&thread_id, MessageRole::Assistant, "Use rustup.")
        .expect("assistant");
    shell.select_thread(&thread_id).expect("reload");

    let provider = TitleCapturingProvider {
        chunks: vec![ChatStreamEvent::Chunk("Installing Rust".into())],
        captured: RefCell::new(None),
    };

    let applied = shell
        .apply_model_generated_title(&thread_id, "test-model", &provider)
        .expect("apply");
    assert!(applied);
    assert_eq!(shell.state().threads[0].title, "Installing Rust");
    assert!(provider.captured.borrow().is_some());
}

#[test]
fn apply_model_title_should_skip_when_auto_title_disabled() {
    let (temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    shell
        .send_message(&thread_id, "Hello, Ronin!")
        .expect("send");

    let mut cfg = RoninConfig::default();
    cfg.general.auto_title = false;
    shell.session().save_config(&cfg).expect("save");

    // recreate shell to pick up... actually load_config each call
    let provider = TitleCapturingProvider {
        chunks: vec![ChatStreamEvent::Chunk("Should Not Apply".into())],
        captured: RefCell::new(None),
    };
    let applied = shell
        .apply_model_generated_title(&thread_id, "test-model", &provider)
        .expect("apply");
    assert!(!applied);
    assert_eq!(shell.state().threads[0].title, "Hello, Ronin!");
    let _ = temp;
}

#[test]
fn apply_model_title_should_not_override_manual_rename() {
    let (_temp, mut shell) = open_shell();
    let thread_id = shell.state().selected_thread_id.clone().unwrap();
    shell
        .send_message(&thread_id, "Hello, Ronin!")
        .expect("send");
    shell
        .session()
        .create_message(&thread_id, MessageRole::Assistant, "Hi!")
        .expect("assistant");
    shell.rename_thread(&thread_id, "Manual").expect("rename");

    let provider = TitleCapturingProvider {
        chunks: vec![ChatStreamEvent::Chunk("Auto Title".into())],
        captured: RefCell::new(None),
    };
    let applied = shell
        .apply_model_generated_title(&thread_id, "test-model", &provider)
        .expect("apply");
    assert!(!applied);
    assert_eq!(shell.state().threads[0].title, "Manual");
}
