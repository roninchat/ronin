use std::process::ExitCode;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, FocusHandle, FontWeight,
    KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollHandle,
    SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use ronin::{
    composer::ComposerEditor, parse_launch_intent, ronin_paths, LaunchIntent, LauncherError,
};
use ronin_app::{ProviderStatus, RoninAppError, RoninShell, ShellState};
use ronin_core::{
    clipboard_attachment, parse_context_tools, read_file_attachment, ChatProvider,
    ContextAttachmentDraft, ContextToolRef, HttpOllamaProvider, MessageRole,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ronin: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), RunError> {
    let intent = parse_launch_intent(std::env::args().skip(1))?;
    let paths = ronin_paths()?;
    let attach_paths = match &intent {
        LaunchIntent::OpenPersisted { attach_paths }
        | LaunchIntent::NewThread { attach_paths }
        | LaunchIntent::OpenWithOllama { attach_paths } => attach_paths.clone(),
    };
    let mut shell = match intent {
        LaunchIntent::OpenPersisted { .. } if !attach_paths.is_empty() => {
            RoninShell::open_with_new_thread(paths)?
        }
        LaunchIntent::OpenPersisted { .. } => RoninShell::open(paths)?,
        LaunchIntent::NewThread { .. } => RoninShell::open_with_new_thread(paths)?,
        LaunchIntent::OpenWithOllama { .. } => RoninShell::open_with_ollama(paths)?,
    };
    let _ = shell.refresh_provider_status();
    tracing::info!(intent = ?intent, "ronin launch intent parsed");

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1120.0), px(760.0)), cx);
        let window_result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Ronin")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    let rem = 14.0; // default rem in pixels for GPUI 0.2.x
                    let mut composer = ComposerEditor::new();
                    composer.set_font_metrics_from_rem(rem);
                    RoninWindow {
                        shell,
                        composer,
                        preattached_files: attach_paths.clone(),
                        attachment_errors: Vec::new(),
                        composer_focus: cx.focus_handle(),
                        chat_provider: None,
                        needs_initial_focus: true,
                        copied_state: None,
                        memories_panel_open: false,
                        artifacts_panel_open: false,
                        parsed_messages: std::collections::HashMap::new(),
                        completion_index: 0,
                        pending_clipboard_read: None,
                        composer_rem: rem,
                        composer_scroll_handle: ScrollHandle::new(),
                        blink_start: std::time::Instant::now(),
                    }
                })
            },
        );

        match window_result {
            Ok(_) => {
                tracing::info!("ronin native window opened");
                cx.activate(true);
            }
            Err(error) => {
                tracing::error!(%error, "failed to open ronin native window");
                cx.quit();
            }
        }
    });

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum RunError {
    #[error(transparent)]
    Launcher(#[from] LauncherError),
    #[error(transparent)]
    App(#[from] RoninAppError),
}

struct RoninWindow {
    shell: RoninShell,
    composer: ComposerEditor,
    preattached_files: Vec<std::path::PathBuf>,
    attachment_errors: Vec<String>,
    composer_focus: FocusHandle,
    chat_provider: Option<Box<dyn ChatProvider + Send>>,
    needs_initial_focus: bool,
    copied_state: Option<(String, std::time::Instant)>,
    memories_panel_open: bool,
    artifacts_panel_open: bool,
    parsed_messages:
        std::collections::HashMap<String, (usize, Vec<ronin::markdown::MarkdownBlock>)>,
    completion_index: usize,
    pending_clipboard_read: Option<std::sync::mpsc::Receiver<Result<String, arboard::Error>>>,
    composer_rem: f32,
    composer_scroll_handle: ScrollHandle,
    blink_start: std::time::Instant,
}

impl RoninWindow {
    fn resolve_active_chat_provider(&self) -> Option<Box<dyn ChatProvider + Send>> {
        let thread_id = self.shell.state().selected_thread_id.as_deref()?;
        let (provider_name, _) = self
            .shell
            .resolve_thread_provider_and_model(thread_id)
            .ok()?;
        let config = self.shell.session().load_config().ok()?;
        if provider_name == "openai" {
            let base_url = config
                .openai
                .as_ref()
                .and_then(|o| o.base_url.clone())
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
            Some(Box::new(ronin_core::OpenAiCompatibleProvider::new(
                base_url,
            )))
        } else {
            let base_url = config.ollama.base_url.clone();
            Some(Box::new(HttpOllamaProvider::new(base_url)))
        }
    }

    // ── context / attachments ──

    fn resolve_context_attachments(&mut self, text: &str) -> (String, Vec<ContextAttachmentDraft>) {
        self.attachment_errors.clear();
        let parsed = parse_context_tools(text);
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let mut attachments = Vec::new();

        for path in &self.preattached_files {
            match read_file_attachment(path, &cwd) {
                Ok(a) => attachments.push(a),
                Err(e) => self.attachment_errors.push(e.to_string()),
            }
        }

        for r in parsed.refs {
            match r {
                ContextToolRef::File(path) => match read_file_attachment(&path, &cwd) {
                    Ok(a) => attachments.push(a),
                    Err(e) => self.attachment_errors.push(e.to_string()),
                },
                ContextToolRef::Clipboard => {
                    match arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                        Ok(t) => attachments.push(clipboard_attachment(&t)),
                        Err(e) => self
                            .attachment_errors
                            .push(format!("failed to read clipboard: {e}")),
                    }
                }
                ContextToolRef::Memory(id) => {
                    if let Ok(mems) = self.shell.list_memories() {
                        if let Some(m) = mems.into_iter().find(|m| m.id.0 == id) {
                            attachments.push(ronin_core::memory_attachment(&m));
                        } else {
                            self.attachment_errors
                                .push(format!("Memory not found: {id}"));
                        }
                    } else {
                        self.attachment_errors
                            .push(format!("Failed to list memories for {id}"));
                    }
                }
                ContextToolRef::Artifact(id) => {
                    if let Ok(arts) = self.shell.list_all_artifacts() {
                        if let Some(a) = arts.into_iter().find(|a| a.id.0 == id) {
                            attachments.push(ronin_core::artifact_attachment(&a));
                        } else {
                            self.attachment_errors
                                .push(format!("Artifact not found: {id}"));
                        }
                    } else {
                        self.attachment_errors
                            .push(format!("Failed to list artifacts for {id}"));
                    }
                }
            }
        }

        (parsed.visible_message, attachments)
    }

    fn send_current_message(&mut self, cx: &mut Context<Self>) {
        let thread_id = match self.shell.state().selected_thread_id.clone() {
            Some(id) => id,
            None => return,
        };
        let text = self.composer.take_text();
        let (visible_text, attachments) = self.resolve_context_attachments(&text);
        if visible_text.trim().is_empty() && attachments.is_empty() {
            self.composer.set_text(text);
            return;
        }

        let model = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } | ProviderStatus::OpenAiReady { model } => {
                model.clone()
            }
            _ => {
                if let Err(err) = self.shell.send_message_with_attachments(
                    &thread_id,
                    &visible_text,
                    &attachments,
                ) {
                    tracing::error!(%err, "failed to send message");
                } else {
                    self.preattached_files.clear();
                }
                cx.notify();
                return;
            }
        };

        let provider = self
            .chat_provider
            .take()
            .or_else(|| self.resolve_active_chat_provider());
        match provider {
            Some(provider) => {
                let result = self.shell.begin_streaming_with_attachments(
                    &thread_id,
                    Some(&visible_text),
                    &attachments,
                    provider,
                    &model,
                );
                match result {
                    Ok(()) => {
                        self.preattached_files.clear();
                        cx.notify();
                    }
                    Err(err) => {
                        tracing::error!(%err, "failed to begin streaming");
                        self.chat_provider = self.resolve_active_chat_provider();
                        cx.notify();
                    }
                }
            }
            None => {
                if let Err(err) = self.shell.send_message_with_attachments(
                    &thread_id,
                    &visible_text,
                    &attachments,
                ) {
                    tracing::error!(%err, "failed to send message");
                } else {
                    self.preattached_files.clear();
                }
                cx.notify();
            }
        }
    }

    fn attachment_pill_labels(&self) -> Vec<String> {
        let parsed = parse_context_tools(self.composer.text());
        let mut labels: Vec<String> = self
            .preattached_files
            .iter()
            .map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("attached file")
                    .to_string()
            })
            .collect();

        labels.extend(parsed.refs.into_iter().map(|r| {
            match r {
                ContextToolRef::File(path) => std::path::Path::new(&path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(path.as_str())
                    .to_string(),
                ContextToolRef::Clipboard => "clipboard".to_string(),
                ContextToolRef::Memory(id) => format!("memory:{}", id),
                ContextToolRef::Artifact(id) => format!("artifact:{}", id),
            }
        }));
        labels
    }

    fn remove_preattached_file(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.preattached_files.len() {
            self.preattached_files.remove(index);
        }
        cx.notify();
    }

    // ── completions ──

    fn command_completion(&self) -> Option<(String, String, usize, usize)> {
        let cursor = self.composer.cursor();
        let text = self.composer.text();
        let ts = text[..cursor]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let token = &text[ts..cursor];
        if !token.starts_with('@') {
            return None;
        }
        let tl = token.to_ascii_lowercase();
        [
            ("@file:", "Attach file"),
            ("@memory:", "Attach memory"),
            ("@clipboard", "Attach clipboard"),
        ]
        .iter()
        .find(|(c, _)| c.to_ascii_lowercase().starts_with(&tl) && *c != tl)
        .map(|(c, l)| (c.to_string(), l.to_string(), ts, cursor))
    }

    fn memory_completions(&self) -> Vec<(String, String)> {
        let cursor = self.composer.cursor();
        let text = self.composer.text();
        let ts = text[..cursor]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let token = &text[ts..cursor];
        let prefix = match token.strip_prefix("@memory:") {
            Some(p) => p,
            None => return Vec::new(),
        };

        let memories = self.shell.list_memories().unwrap_or_default();
        let mut matches: Vec<(String, String)> = memories
            .into_iter()
            .filter(|m| {
                m.id.0.starts_with(prefix)
                    || m.title.to_lowercase().contains(&prefix.to_lowercase())
            })
            .map(|m| (m.id.0, m.title))
            .collect();
        matches.truncate(8);
        matches
    }

    fn file_path_completions(&self) -> Vec<String> {
        let cursor = self.composer.cursor();
        let text = self.composer.text();
        let ts = text[..cursor]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let token = &text[ts..cursor];
        let prefix = match token.strip_prefix("@file:") {
            Some(p) => {
                if p.starts_with('"') {
                    p.strip_prefix('"').unwrap().trim_end_matches('"')
                } else {
                    p
                }
            }
            None => return Vec::new(),
        };

        // Normalize: strip trailing / to get dir, extract file name prefix
        let prefix_path = std::path::Path::new(prefix);
        let (dir, file_prefix) = if prefix.is_empty() || prefix == "/" {
            // Empty or just "/" — list root or home
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
            (std::path::PathBuf::from(home), String::new())
        } else if prefix.ends_with('/') {
            // Explicit directory — list its contents
            let d = prefix_path.to_path_buf();
            if d.is_dir() {
                (d, String::new())
            } else {
                // Try resolving via HOME
                let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                let full = std::path::PathBuf::from(&home).join(prefix_path);
                if full.is_dir() {
                    (full, String::new())
                } else {
                    (
                        full.parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| std::path::PathBuf::from(&home)),
                        full.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string(),
                    )
                }
            }
        } else if prefix_path.is_dir() {
            // Path is an existing directory — list its contents
            (prefix_path.to_path_buf(), String::new())
        } else {
            // Path is a partial — get parent dir and file name prefix
            let parent = prefix_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                    std::path::PathBuf::from(&home)
                });
            // If prefix starts with /, resolve absolute; otherwise try relative then HOME fallback
            let dir = if prefix.starts_with('/') {
                parent
            } else if parent.as_os_str().is_empty() {
                // Just a filename — search cwd first, fallback to HOME
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            } else if parent.is_dir() {
                parent
            } else {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                let full = std::path::PathBuf::from(&home).join(&parent);
                if full.is_dir() {
                    full
                } else {
                    parent
                }
            };
            let fname = prefix_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            (dir, fname)
        };

        let file_prefix_lower = file_prefix.to_ascii_lowercase();

        let mut matches = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                // Filter . and ..
                if name == "." || name == ".." {
                    continue;
                }
                // Case-insensitive prefix match
                let name_lower = name.to_ascii_lowercase();
                if !file_prefix_lower.is_empty() && !name_lower.starts_with(&file_prefix_lower) {
                    continue;
                }
                let suffix = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    "/"
                } else {
                    ""
                };
                matches.push(format!("{name}{suffix}"));
            }
        }
        matches.sort_by(|a, b| {
            // Directories first, then alphabetical
            let a_dir = a.ends_with('/');
            let b_dir = b.ends_with('/');
            b_dir.cmp(&a_dir).then_with(|| a.cmp(b))
        });
        matches.truncate(8);
        matches
    }

    fn accept_command_completion(&mut self) -> bool {
        if let Some((command, _, start, end)) = self.command_completion() {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
            let cmd = if command == "@file:" {
                format!("@file:{home}/")
            } else {
                command.to_string()
            };
            self.composer.replace_range(start, end, &cmd);
            if command == "@clipboard" {
                self.composer.insert_str(" ");
            }
            self.completion_index = 0;
            return true;
        }

        let mem_completions = self.memory_completions();
        if !mem_completions.is_empty() {
            let idx = self.completion_index.min(mem_completions.len() - 1);
            let (chosen_id, _) = &mem_completions[idx];
            let cursor = self.composer.cursor();
            let text = self.composer.text();
            let ts = text[..cursor]
                .rfind(char::is_whitespace)
                .map(|i| i + 1)
                .unwrap_or(0);

            let replacement = format!("@memory:{chosen_id} ");
            self.composer.replace_range(ts, cursor, &replacement);
            self.completion_index = 0;
            return true;
        }

        let completions = self.file_path_completions();
        if completions.is_empty() {
            self.completion_index = 0;
            return false;
        }
        let idx = self.completion_index.min(completions.len() - 1);
        let chosen = &completions[idx];
        let cursor = self.composer.cursor();
        let text = self.composer.text();
        let ts = text[..cursor]
            .rfind(char::is_whitespace)
            .map(|i| i + 1)
            .unwrap_or(0);
        let token = &text[ts..cursor];
        let prefix = token.strip_prefix("@file:").unwrap_or(token);
        let quoted = prefix.starts_with('"');
        let head = if quoted { "@file:\"" } else { "@file:" };
        let base = prefix
            .strip_prefix('"')
            .map(|q| q.trim_end_matches('"'))
            .unwrap_or(prefix);

        // Build directory prefix: include trailing /
        let base_path = std::path::Path::new(base);
        let dir_str = if base.ends_with('/') {
            // Already has trailing / — keep dir as-is
            if base == "/" || base.is_empty() {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                format!("{home}/")
            } else {
                base.to_string()
            }
        } else if base.is_empty() {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
            format!("{home}/")
        } else if base_path.is_dir() {
            format!("{base}/")
        } else {
            base_path
                .parent()
                .and_then(|p| p.to_str())
                .filter(|d| !d.is_empty())
                .map(|d| format!("{d}/"))
                .unwrap_or_else(|| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                    format!("{home}/")
                })
        };

        let repl = format!("{head}{dir_str}{chosen}");
        self.composer.replace_range(ts, cursor, &repl);
        self.completion_index = 0;
        true
    }

    // ── keyboard / mouse ──

    fn on_composer_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ks = &event.keystroke;
        let ctrl = ks.modifiers.control;
        let shift = ks.modifiers.shift;
        let alt_or_plat = ks.modifiers.alt || ks.modifiers.platform;

        if alt_or_plat {
            return;
        }

        // Reset blink timer on any user input so cursor is visible during typing
        self.blink_start = std::time::Instant::now();
        self.composer.cursor_visible = true;

        // Check for file-path completion list to handle up/down navigation
        let file_matches = self.file_path_completions();
        let has_files = !file_matches.is_empty();
        let cmd_completion = self.command_completion();
        let has_cmd = cmd_completion.is_some();

        // up/down: navigate file completions if visible, else delegate to composer
        match ks.key.as_str() {
            "up" if has_files => {
                self.completion_index = self.completion_index.saturating_sub(1);
                cx.notify();
                return;
            }
            "down" if has_files => {
                let max = file_matches.len().saturating_sub(1);
                self.completion_index = (self.completion_index + 1).min(max);
                cx.notify();
                return;
            }
            _ => {}
        }

        // Delegate to editor (backspace, delete, arrows, home, end, ctrl+a)
        // But skip up/down when completions visible (already handled above)
        let skip_composer = (has_files || has_cmd) && matches!(ks.key.as_str(), "up" | "down");
        if !skip_composer && self.composer.on_key_down(event) {
            cx.notify();
            return;
        }

        match ks.key.as_str() {
            "enter" => {
                if shift {
                    self.composer.insert_str("\n");
                    cx.notify();
                    return;
                }
                if self.accept_command_completion() {
                    cx.notify();
                    return;
                }
                self.send_current_message(cx);
            }
            "tab" => {
                if self.accept_command_completion() {
                    cx.notify();
                }
            }
            "escape" => {
                self.cancel_generation(cx);
            }
            "v" if ctrl => {
                if self.pending_clipboard_read.is_none() {
                    let (tx, rx) = std::sync::mpsc::channel();
                    std::thread::spawn(move || {
                        let result = arboard::Clipboard::new().and_then(|mut c| c.get_text());
                        let _ = tx.send(result);
                    });
                    self.pending_clipboard_read = Some(rx);
                }
            }
            "c" if ctrl => {
                if let Some(text) = self.composer.selected_text() {
                    if let Ok(mut c) = arboard::Clipboard::new() {
                        let _ = c.set_text(text);
                    }
                }
            }
            "x" if ctrl => {
                if let Some(text) = self.composer.selected_text() {
                    if let Ok(mut c) = arboard::Clipboard::new() {
                        let _ = c.set_text(text);
                    }
                }
                self.composer.delete_before_cursor(); // selection delete
                cx.notify();
            }
            "space" if !ctrl => {
                self.composer.insert_char(' ');
                cx.notify();
            }
            _ => {
                if ctrl {
                    return;
                }
                if let Some(ref kc) = ks.key_char {
                    for ch in kc.chars() {
                        self.composer.insert_char(ch);
                    }
                    cx.notify();
                }
            }
        }
    }

    fn on_composer_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // NOTE: MouseDownEvent.position is window-relative in GPUI 0.2,
        // not element-relative. For now, clicking anywhere in composer
        // moves cursor to end. Full pixel positioning needs element bounds.
        self.composer.click_at_end();
        cx.notify();
    }

    fn on_composer_mouse_move(
        &mut self,
        _event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Mouse drag extends selection from anchor to end of text
        // (pixel-perfect drag needs element-relative coords, not available in GPUI 0.2)
        if self.composer.drag_to_end() {
            cx.notify();
        }
    }

    fn on_composer_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.composer.end_drag();
    }

    // ── streaming / sidebar / messages ──

    fn pump_streaming(&mut self) -> bool {
        let active = self.shell.poll_streaming();
        if !active && self.chat_provider.is_none() {
            self.chat_provider = self.resolve_active_chat_provider();
        }
        active
    }

    fn cancel_generation(&mut self, cx: &mut Context<Self>) {
        if let Err(e) = self.shell.cancel_streaming() {
            tracing::error!(%e, "failed to cancel generation");
        }
        cx.notify();
    }

    fn create_new_thread(&mut self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        match self.shell.create_new_thread() {
            Ok(_) => {
                window.focus(&self.composer_focus);
                cx.notify();
            }
            Err(error) => tracing::error!(%error, "failed to create thread from sidebar"),
        }
    }

    fn select_thread(
        &mut self,
        thread_id: String,
        _: &MouseUpEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.shell.select_thread(&thread_id) {
            Ok(()) => {
                window.focus(&self.composer_focus);
                cx.notify();
            }
            Err(error) => tracing::error!(%error, "failed to select thread from sidebar"),
        }
    }

    fn copy_to_clipboard(&mut self, id: String, text: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
        self.copied_state = Some((id, std::time::Instant::now()));
        cx.notify();
    }

    fn save_as_memory(&mut self, text: String, cx: &mut Context<Self>) {
        let title: String = text.chars().take(60).collect();
        let content = text;
        if let Err(e) = self.shell.create_memory(&title, &content) {
            tracing::error!(%e, "failed to save memory");
        } else {
            self.memories_panel_open = true;
            cx.notify();
        }
    }

    fn save_as_artifact(
        &mut self,
        thread_id: String,
        message_id: String,
        text: String,
        cx: &mut Context<Self>,
    ) {
        let title: String = text.chars().take(60).collect();
        let content = text;
        if let Err(e) = self
            .shell
            .create_artifact(&thread_id, &message_id, &title, &content)
        {
            tracing::error!(%e, "failed to save artifact");
        } else {
            self.artifacts_panel_open = true;
            cx.notify();
        }
    }

    fn on_global_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystroke = &event.keystroke;
        match keystroke.key.as_str() {
            "n" if keystroke.modifiers.control => self.create_new_thread_shortcut(window, cx),
            "l" | "k" if keystroke.modifiers.control => self.focus_composer_shortcut(window, cx),
            "r" if keystroke.modifiers.control => self.retry_generation_shortcut(window, cx),
            "g" if keystroke.modifiers.control && keystroke.modifiers.shift => {
                self.regenerate_message_shortcut(cx);
            }
            "escape" => self.cancel_generation(cx),
            _ => {}
        }
    }

    fn create_new_thread_shortcut(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.shell.create_new_thread() {
            Ok(_) => {
                window.focus(&self.composer_focus);
                cx.notify();
            }
            Err(e) => tracing::error!(%e, "failed to create new thread via shortcut"),
        }
    }

    fn focus_composer_shortcut(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.composer_focus);
        cx.notify();
    }

    fn retry_generation_shortcut(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let msgs = match &self.shell.state().messages {
            Some(m) => m,
            None => return,
        };
        if let Some(msg) = msgs.iter().rfind(|m| {
            m.status == ronin_core::MessageStatus::Failed
                || m.status == ronin_core::MessageStatus::Error
        }) {
            let id = msg.id.clone();
            self.retry_failed_message(id, cx);
        }
    }

    fn regenerate_message_shortcut(&mut self, cx: &mut Context<Self>) {
        self.regenerate_last_assistant(cx);
    }

    fn retry_failed_message(&mut self, message_id: String, cx: &mut Context<Self>) {
        if self.shell.is_generation_active() {
            return;
        }
        let provider = self
            .chat_provider
            .take()
            .or_else(|| self.resolve_active_chat_provider());
        let provider = match provider {
            Some(p) => p,
            None => Box::new(HttpOllamaProvider::new("http://localhost:11434")),
        };
        let model = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } | ProviderStatus::OpenAiReady { model } => {
                model.clone()
            }
            _ => {
                self.chat_provider = Some(provider);
                return;
            }
        };
        if let Err(e) = self.shell.retry_message(&message_id, provider, &model) {
            tracing::error!(%e, "failed to retry message");
            self.chat_provider = self.resolve_active_chat_provider();
        }
        cx.notify();
    }

    fn regenerate_last_assistant(&mut self, cx: &mut Context<Self>) {
        if self.shell.is_generation_active() {
            return;
        }
        let thread_id = match self.shell.state().selected_thread_id.clone() {
            Some(id) => id,
            None => return,
        };
        let provider = self
            .chat_provider
            .take()
            .or_else(|| self.resolve_active_chat_provider());
        let provider = match provider {
            Some(p) => p,
            None => Box::new(HttpOllamaProvider::new("http://localhost:11434")),
        };
        let model = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } | ProviderStatus::OpenAiReady { model } => {
                model.clone()
            }
            _ => {
                self.chat_provider = Some(provider);
                return;
            }
        };
        if let Err(e) = self
            .shell
            .regenerate_last_assistant(&thread_id, provider, &model)
        {
            tracing::error!(%e, "failed to regenerate message");
            self.chat_provider = self.resolve_active_chat_provider();
        }
        cx.notify();
    }

    // ── rendering ──

    fn render_sidebar(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.shell.state();
        let selected_thread_id = state.selected_thread_id.as_deref();
        let on_new_chat = cx.listener(Self::create_new_thread);

        let thread_rows = state.threads.iter().map(|thread| {
            let is_selected = Some(thread.id.as_str()) == selected_thread_id;
            let thread_id = thread.id.clone();
            let row_bg = if is_selected {
                theme.surface_selected
            } else {
                theme.surface_muted
            };
            div()
                .rounded_md()
                .px_3()
                .py_2()
                .bg(row_bg)
                .text_color(theme.text_primary)
                .child(thread.title.clone())
                .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                .on_mouse_up(MouseButton::Left, {
                    let id = thread_id.clone();
                    cx.listener(move |this, event, window, cx| {
                        this.select_thread(id.clone(), event, window, cx);
                    })
                })
        });

        div()
            .w(px(280.0))
            .h_full()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(theme.sidebar_background)
            .border_r_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.text_primary)
                    .child("Ronin"),
            )
            .child(
                div()
                    .rounded_lg()
                    .px_3()
                    .py_2()
                    .bg(theme.accent)
                    .text_color(theme.accent_text)
                    .font_weight(FontWeight(500.))
                    .child("New Chat")
                    .hover(|style| style.bg(theme.accent_hover).cursor_pointer())
                    .on_mouse_up(MouseButton::Left, on_new_chat),
            )
            .child(
                div()
                    .rounded_lg()
                    .px_3()
                    .py_2()
                    .bg(theme.surface_muted)
                    .text_color(theme.text_primary)
                    .font_weight(FontWeight(500.))
                    .child("Memories")
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                                this.memories_panel_open = !this.memories_panel_open;
                                cx.notify();
                            }),
                        ),
                )
                .child(
                    div()
                        .rounded_lg()
                        .px_3()
                        .py_2()
                        .bg(theme.surface_muted)
                        .text_color(theme.text_primary)
                        .font_weight(FontWeight(500.))
                        .child("Artifacts")
                        .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.artifacts_panel_open = !this.artifacts_panel_open;
                                cx.notify();
                            }),
                        ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .id("sidebar-scroll")
                    .overflow_y_scroll()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .mb_2()
                            .child("Threads"),
                    )
                    .child(div().flex().flex_col().gap_2().children(thread_rows))
                    .child(if state.truncation_notice {
                        div()
                            .text_xs()
                            .text_color(theme.accent)
                            .mt_2()
                            .child("Some older messages were omitted.")
                    } else {
                        div()
                    }),
            )
            .child(
                div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .p_3()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child(self.render_provider_status(state)),
            )
    }

    fn render_provider_status(&self, state: &ShellState) -> String {
        match state.provider_status {
            ProviderStatus::NotConfigured => {
                "Provider: not configured\nModel: not selected".to_string()
            }
            ProviderStatus::OllamaOffline => {
                "Provider: ollama\nModel: offline\n\nollama not reachable — is the server running?"
                    .to_string()
            }
            ProviderStatus::OllamaOnline { ref model } => {
                format!("Provider: ollama\nModel: {model}")
            }
            ProviderStatus::OllamaNoModels => {
                "Provider: ollama\nModel: none\n\nNo models installed.\nTry: ollama pull llama3.2"
                    .to_string()
            }
            ProviderStatus::OpenAiReady { ref model } => {
                format!("Provider: openai\nModel: {model}")
            }
            ProviderStatus::OpenAiError { ref message } => {
                format!("Provider: openai\nModel: error\n\n{message}")
            }
            ProviderStatus::OpenAiNotConfigured => {
                "Provider: openai\nModel: none\n\nOpenAI not configured.\nSet OPENAI_API_KEY environment variable or add it to settings."
                    .to_string()
            }
        }
    }

    fn render_messages(&mut self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let messages_opt = self.shell.state().messages.clone();
        let messages = match messages_opt {
            Some(msgs) if !msgs.is_empty() => msgs.clone(),
            _ => {
                return div()
                    .flex_1()
                    .p_6()
                    .text_color(theme.text_muted)
                    .id("empty-messages")
                    .child("Start a conversation. Messages will appear here.");
            }
        };

        let is_generating = self.shell.is_generation_active();
        let last_assistant_id = messages
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::Assistant)
            .map(|m| m.id.clone());

        let message_elements = messages.into_iter().filter_map(|msg| {
            if msg.role == MessageRole::System {
                return None;
            }
            let (label, bg) = match msg.role {
                MessageRole::User => ("You", theme.surface_muted),
                MessageRole::Assistant => ("Assistant", theme.surface_selected),
                MessageRole::System => unreachable!(),
            };
            let raw_content = msg.content.clone();
            let is_copied = self.copied_state.as_ref().map(|(id, _)| id) == Some(&msg.id);
            let copy_text = if is_copied { "Copied!" } else { "Copy" };
            let mut message_body = div().w_full().flex().flex_col().gap_3();

            let blocks = if let Some((len, cached_blocks)) = self.parsed_messages.get(&msg.id) {
                if *len == msg.content.len() {
                    Some(cached_blocks.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let blocks = blocks.unwrap_or_else(|| {
                let parsed = ronin::markdown::parse_markdown(&msg.content);
                self.parsed_messages
                    .insert(msg.id.clone(), (msg.content.len(), parsed.clone()));
                parsed
            });

            for (block_idx, block) in blocks.into_iter().enumerate() {
                let block_el = match block {
                    ronin::markdown::MarkdownBlock::Paragraph(inlines) => {
                        let mut p = div().w_full().flex().flex_row().flex_wrap().gap_1();
                        for inline in inlines {
                            match inline {
                                ronin::markdown::Inline::Text(text) => {
                                    for word in text.split(' ') {
                                        if !word.is_empty() {
                                            p = p.child(div().child(word.to_string()));
                                        }
                                    }
                                }
                                ronin::markdown::Inline::Code(code) => {
                                    p = p.child(
                                        div()
                                            .bg(theme.surface_muted)
                                            .rounded_sm()
                                            .px_1()
                                            .font_family("Courier New")
                                            .text_color(theme.accent)
                                            .child(code),
                                    );
                                }
                            }
                        }
                        p
                    }
                    ronin::markdown::MarkdownBlock::CodeBlock { language, content } => {
                        let lang_label = language.unwrap_or_else(|| "text".to_string());
                        let mut code_lines = div()
                            .id(gpui::SharedString::from(format!(
                                "{}-code-scroll-{}",
                                msg.id, block_idx
                            )))
                            .w_full()
                            .font_family("Courier New")
                            .flex()
                            .flex_col()
                            .overflow_x_scroll();
                        for line in content.split('\n') {
                            code_lines = code_lines.child(div().child(line.to_string()));
                        }
                        let block_id = format!("{}-code-{}", msg.id, block_idx);
                        let is_block_copied =
                            self.copied_state.as_ref().map(|(id, _)| id) == Some(&block_id);
                        let block_copy_text = if is_block_copied { "Copied!" } else { "Copy" };
                        let code_content = content.clone();
                        let header = div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(lang_label),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if is_block_copied {
                                        theme.text_primary
                                    } else {
                                        theme.accent
                                    })
                                    .cursor_pointer()
                                    .child(block_copy_text)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener({
                                            let block_id = block_id.clone();
                                            let code_content = code_content.clone();
                                            move |this, _, _, cx| {
                                                this.copy_to_clipboard(
                                                    block_id.clone(),
                                                    code_content.clone(),
                                                    cx,
                                                );
                                            }
                                        }),
                                    ),
                            );
                        div()
                            .w_full()
                            .overflow_hidden()
                            .bg(theme.surface_hover)
                            .rounded_md()
                            .p_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(header)
                            .child(code_lines)
                    }
                    ronin::markdown::MarkdownBlock::List(items) => {
                        let mut list_div = div().w_full().flex().flex_col().gap_1().pl_4();
                        for item in items {
                            let mut li_content = div().flex().flex_row().flex_wrap().gap_1();
                            for inline in item.inlines {
                                match inline {
                                    ronin::markdown::Inline::Text(text) => {
                                        for word in text.split(' ') {
                                            if !word.is_empty() {
                                                li_content =
                                                    li_content.child(div().child(word.to_string()));
                                            }
                                        }
                                    }
                                    ronin::markdown::Inline::Code(code) => {
                                        li_content = li_content.child(
                                            div()
                                                .bg(theme.surface_muted)
                                                .rounded_sm()
                                                .px_1()
                                                .font_family("Courier New")
                                                .text_color(theme.accent)
                                                .child(code),
                                        );
                                    }
                                }
                            }
                            list_div = list_div.child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap_2()
                                    .child(div().child("•"))
                                    .child(li_content),
                            );
                        }
                        list_div
                    }
                };
                message_body = message_body.child(block_el);
            }

            let is_last_assistant = Some(&msg.id) == last_assistant_id.as_ref();
            let mut message_actions = div()
                .flex()
                .flex_row()
                .gap_3()
                .child(
                    div()
                        .text_xs()
                        .text_color(if is_copied {
                            theme.text_primary
                        } else {
                            theme.accent
                        })
                        .cursor_pointer()
                        .child(copy_text)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let msg_id = msg.id.clone();
                                let raw_content = raw_content.clone();
                                move |this, _, _, cx| {
                                    this.copy_to_clipboard(msg_id.clone(), raw_content.clone(), cx);
                                }
                            }),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.accent)
                        .cursor_pointer()
                        .child("Save as memory")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let raw_content = raw_content.clone();
                                move |this, _, _, cx| {
                                    this.save_as_memory(raw_content.clone(), cx);
                                }
                            }),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.accent)
                        .cursor_pointer()
                        .child("Save as artifact")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let thread_id = msg.thread_id.clone();
                                let msg_id = msg.id.clone();
                                let raw_content = raw_content.clone();
                                move |this, _, _, cx| {
                                    this.save_as_artifact(
                                        thread_id.clone(),
                                        msg_id.clone(),
                                        raw_content.clone(),
                                        cx,
                                    );
                                }
                            }),
                        ),
                );

            if msg.role == MessageRole::Assistant
                && (msg.status == ronin_core::MessageStatus::Failed
                    || msg.status == ronin_core::MessageStatus::Error)
            {
                message_actions = message_actions.child(
                    div()
                        .text_xs()
                        .text_color(if is_generating {
                            theme.text_muted
                        } else {
                            theme.accent
                        })
                        .cursor_pointer()
                        .child("Retry")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let msg_id = msg.id.clone();
                                move |this, _, _, cx| {
                                    if !this.shell.is_generation_active() {
                                        this.retry_failed_message(msg_id.clone(), cx);
                                    }
                                }
                            }),
                        ),
                );
            }

            if is_last_assistant
                && (msg.status == ronin_core::MessageStatus::Complete
                    || msg.status == ronin_core::MessageStatus::Cancelled)
            {
                message_actions = message_actions.child(
                    div()
                        .text_xs()
                        .text_color(if is_generating {
                            theme.text_muted
                        } else {
                            theme.accent
                        })
                        .cursor_pointer()
                        .child("Regenerate")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                move |this, _, _, cx| {
                                    if !this.shell.is_generation_active() {
                                        this.regenerate_last_assistant(cx);
                                    }
                                }
                            }),
                        ),
                );
            }

            Some(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .mb_4()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .mb_1()
                            .child(div().text_xs().text_color(theme.text_muted).child(label))
                            .child(message_actions),
                    )
                    .child(
                        div()
                            .w_full()
                            .rounded_lg()
                            .px_4()
                            .py_3()
                            .bg(bg)
                            .text_color(theme.text_primary)
                            .child(message_body),
                    ),
            )
        });

        let mut container = div()
            .flex_1()
            .p_6()
            .flex()
            .flex_col()
            .gap_2()
            .id("message-scroll")
            .overflow_y_scroll();
        for el in message_elements {
            container = container.child(el);
        }
        if is_generating {
            container = container.child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child("● Generating response…"),
            );
        }
        container
    }

    fn render_composer(
        &self,
        theme: &M0Theme,
        composer_focused: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_generating = self.shell.is_generation_active();
        let send_btn_bg = if is_generating {
            theme.surface_muted
        } else {
            theme.accent
        };
        let border_color = if composer_focused {
            theme.accent
        } else {
            theme.border_strong
        };

        let mut composer = div().p_6().flex().flex_col().gap_2();

        // attachment pills
        let pill_labels = self.attachment_pill_labels();
        if !pill_labels.is_empty() {
            let preattached_count = self.preattached_files.len();
            let mut pills = div().flex().flex_row().flex_wrap().gap_2();
            for (index, label) in pill_labels.into_iter().enumerate() {
                let mut pill = div()
                    .rounded_lg()
                    .px_3()
                    .py_1()
                    .bg(theme.surface_muted)
                    .text_color(theme.text_primary)
                    .text_xs()
                    .child(format!("📎 {label}"));
                if index < preattached_count {
                    pill = pill.child(
                        div()
                            .ml_2()
                            .text_color(theme.accent)
                            .child("×")
                            .cursor_pointer()
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.remove_preattached_file(index, cx);
                                }),
                            ),
                    );
                }
                pills = pills.child(pill);
            }
            composer = composer.child(pills);
        }

        // attachment errors
        for error in &self.attachment_errors {
            composer = composer.child(
                div()
                    .text_xs()
                    .text_color(theme.accent)
                    .child(error.clone()),
            );
        }

        // command completion dropdown
        if let Some((command, label, _, _)) = self.command_completion() {
            composer = composer.child(
                div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .px_3()
                    .py_2()
                    .text_sm()
                    .text_color(theme.text_primary)
                    .child(format!("{command} — {label}  (Tab/Enter)"))
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.accept_command_completion();
                            cx.notify();
                        }),
                    ),
            );
        }

        // memory completions dropdown
        let mem_matches = self.memory_completions();
        if !mem_matches.is_empty() {
            let ci = self
                .completion_index
                .min(mem_matches.len().saturating_sub(1));
            let mut dropdown = div()
                .rounded_lg()
                .border_1()
                .border_color(theme.border_subtle)
                .bg(theme.surface_muted)
                .flex()
                .flex_col()
                .overflow_hidden();
            for (i, (id, title)) in mem_matches.iter().enumerate() {
                let bg = if i == ci {
                    theme.surface_selected
                } else {
                    theme.surface_muted
                };
                let mut entry = div()
                    .px_3()
                    .py_1()
                    .text_sm()
                    .text_color(theme.text_primary)
                    .bg(bg)
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .child(format!("{title} ({id})"));
                if i == ci {
                    entry = entry.bg(theme.surface_selected);
                }
                entry = entry.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.completion_index = i;
                        this.accept_command_completion();
                        cx.notify();
                    }),
                );
                dropdown = dropdown.child(entry);
            }
            composer = composer.child(dropdown);
        }

        // file path completions dropdown
        let file_matches = self.file_path_completions();
        if !file_matches.is_empty() {
            let ci = self
                .completion_index
                .min(file_matches.len().saturating_sub(1));
            let mut dropdown = div()
                .rounded_lg()
                .border_1()
                .border_color(theme.border_subtle)
                .bg(theme.surface_muted)
                .flex()
                .flex_col()
                .overflow_hidden();
            for (i, item) in file_matches.iter().enumerate() {
                let bg = if i == ci {
                    theme.surface_selected
                } else {
                    theme.surface_muted
                };
                let mut entry = div()
                    .px_3()
                    .py_1()
                    .text_sm()
                    .text_color(theme.text_primary)
                    .bg(bg)
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .child(item.clone());
                if i == ci {
                    entry = entry.bg(theme.surface_selected);
                }
                entry = entry.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.completion_index = i;
                        this.accept_command_completion();
                        cx.notify();
                    }),
                );
                dropdown = dropdown.child(entry);
            }
            composer = composer.child(dropdown);
        }

        // Auto-scroll: only follow cursor when on the last visual line
        let max_input_h = self.composer_rem * 3.0 * 6.0 + self.composer_rem * 4.0 * 2.0;
        let lines = self.composer.visual_lines();
        let cursor_line = self.composer.visual_line_index(self.composer.cursor());
        if cursor_line >= lines.len().saturating_sub(1) {
            self.composer_scroll_handle.scroll_to_bottom();
        }
        composer.child(
            div()
                .flex()
                .items_end()
                .gap_2()
                .child(
                    div()
                        .flex_1()
                        .rounded_xl()
                        .border_2()
                        .border_color(border_color)
                        .bg(theme.composer_background)
                        .p_4()
                        .flex()
                        .flex_col()
                        .id("composer")
                        .overflow_y_scroll()
                        .track_scroll(&self.composer_scroll_handle)
                        .max_h(px(max_input_h))
                        .cursor_text()
                        .track_focus(&self.composer_focus)
                        .on_key_down(cx.listener(Self::on_composer_key_down))
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::on_composer_mouse_down))
                        .on_mouse_move(cx.listener(Self::on_composer_mouse_move))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::on_composer_mouse_up))
                        .child(self.composer.render_text(
                            "Ask Ronin anything…",
                            theme.text_primary,
                            theme.text_muted,
                            theme.accent,
                        )),
                )
                .child(
                    div()
                        .rounded_lg()
                        .px_4()
                        .py_2()
                        .bg(send_btn_bg)
                        .text_color(theme.accent_text)
                        .font_weight(FontWeight(500.))
                        .child("Send")
                        .hover(|style| {
                            if !is_generating {
                                style.bg(theme.accent_hover).cursor_pointer()
                            } else {
                                style
                            }
                        })
                        .on_mouse_up(MouseButton::Left, {
                            cx.listener(move |this, _event, _window, cx| {
                                if !this.shell.is_generation_active() {
                                    this.send_current_message(cx);
                                }
                            })
                        }),
                ),
        )
    }

    fn render_memories_panel(
        &mut self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let memories = self.shell.list_memories().unwrap_or_default();

        let mut list = div()
            .flex()
            .flex_col()
            .gap_2()
            .id("memories-list")
            .overflow_y_scroll()
            .w_full()
            .h_full();
        for mem in memories {
            let id = mem.id.clone();
            list = list.child(
                div()
                    .p_3()
                    .bg(theme.surface_hover)
                    .rounded_md()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().font_weight(FontWeight(600.)).child(mem.title))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(mem.content.chars().take(100).collect::<String>()),
                    )
                    .child(
                        div().flex().flex_row().gap_2().mt_2().child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child("Delete")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = id.clone();
                                        move |this, _, _, cx| {
                                            this.shell.delete_memory(&id).ok();
                                            cx.notify();
                                        }
                                    }),
                                ),
                        ),
                    ),
            );
        }

        div()
            .w(px(320.0))
            .h_full()
            .bg(theme.sidebar_background)
            .border_l_1()
            .border_color(theme.border_subtle)
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight(600.))
                            .child("Memories"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .cursor_pointer()
                            .child("Close")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.memories_panel_open = false;
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            .child(list)
    }

    fn render_artifacts_panel(
        &mut self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let artifacts = self.shell.list_all_artifacts().unwrap_or_default();

        let mut list = div()
            .flex()
            .flex_col()
            .gap_2()
            .id("artifacts-list")
            .overflow_y_scroll()
            .w_full()
            .h_full();
        for art in artifacts {
            let id = art.id.clone();
            list = list.child(
                div()
                    .p_3()
                    .bg(theme.surface_hover)
                    .rounded_md()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().font_weight(FontWeight(600.)).child(art.title))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(art.content.chars().take(100).collect::<String>()),
                    )
                    .child(
                        div().flex().flex_row().gap_2().mt_2().child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child("Delete")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = id.clone();
                                        move |this, _, _, cx| {
                                            this.shell.delete_artifact(&id).ok();
                                            cx.notify();
                                        }
                                    }),
                                ),
                        ),
                    ),
            );
        }

        div()
            .w(px(320.0))
            .h_full()
            .bg(theme.sidebar_background)
            .border_l_1()
            .border_color(theme.border_subtle)
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight(600.))
                            .child("Artifacts"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .cursor_pointer()
                            .child("Close")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.artifacts_panel_open = false;
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            .child(list)
    }
}

impl Render for RoninWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let streaming_active = self.pump_streaming();
        if self.needs_initial_focus {
            self.needs_initial_focus = false;
            window.focus(&self.composer_focus);
        }
        let theme = M0Theme::dark();
        let composer_focused = self.composer_focus.is_focused(window);

        // Blink cursor: 530ms visible, 470ms hidden cycle
        let blink_elapsed = self.blink_start.elapsed().as_millis() as u64;
        self.composer.cursor_visible = (blink_elapsed % 1000) < 530;

        // Poll for completed clipboard reads (non-blocking background paste)
        if let Some(ref rx) = self.pending_clipboard_read {
            if let Ok(result) = rx.try_recv() {
                self.pending_clipboard_read = None;
                match result {
                    Ok(text) => {
                        self.composer.insert_str(&text);
                    }
                    Err(e) => {
                        tracing::warn!("clipboard paste failed: {e}");
                    }
                }
            }
        }

        // Estimate composer text container width for layout
        let rem = self.composer_rem;
        let sidebar_w = 200.0;
        let outer_pad = rem * 6.0 * 2.0; // p_6 left+right
        let inner_pad = rem * 4.0 * 2.0; // p_4 left+right
        let border_w = 4.0; // border_2
        let send_btn_w = 80.0;
        let gap_w = rem * 0.5; // gap_2
        let text_w = 1120.0 - sidebar_w - outer_pad - inner_pad - border_w - send_btn_w - gap_w;
        self.composer.set_container_width(text_w.max(100.0));

        let sidebar = self.render_sidebar(&theme, cx);
        let title = Self::current_thread_title(self.shell.state())
            .map(|t| t.to_string())
            .unwrap_or_else(|| "New Chat".to_string());
        let messages = self.render_messages(&theme, cx);
        let composer = self.render_composer(&theme, composer_focused, cx);

        let mut ui = div()
            .size_full()
            .flex()
            .bg(theme.app_background)
            .text_color(theme.text_primary)
            .font_family("Inter")
            .on_key_down(cx.listener(Self::on_global_key_down))
            .child(sidebar)
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .px_6()
                            .py_4()
                            .child(title),
                    )
                    .child(messages)
                    .child(composer),
            );

        if self.memories_panel_open {
            ui = ui.child(self.render_memories_panel(&theme, cx));
        }

        if self.artifacts_panel_open {
            ui = ui.child(self.render_artifacts_panel(&theme, cx));
        }

        let mut needs_frame = streaming_active || composer_focused;
        if let Some((_, time)) = self.copied_state.as_ref() {
            if time.elapsed().as_secs_f32() >= 1.0 {
                self.copied_state = None;
                needs_frame = true;
            } else {
                needs_frame = true;
            }
        }
        if needs_frame {
            window.request_animation_frame();
        }
        ui
    }
}

impl RoninWindow {
    fn current_thread_title(state: &ShellState) -> Option<&str> {
        state
            .threads
            .iter()
            .find(|t| Some(t.id.as_str()) == state.selected_thread_id.as_deref())
            .map(|t| t.title.as_str())
    }
}

#[derive(Clone, Copy)]
struct M0Theme {
    app_background: gpui::Hsla,
    sidebar_background: gpui::Hsla,
    surface_muted: gpui::Hsla,
    surface_hover: gpui::Hsla,
    surface_selected: gpui::Hsla,
    composer_background: gpui::Hsla,
    border_subtle: gpui::Hsla,
    border_strong: gpui::Hsla,
    text_primary: gpui::Hsla,
    text_muted: gpui::Hsla,
    accent: gpui::Hsla,
    accent_hover: gpui::Hsla,
    accent_text: gpui::Hsla,
}

impl M0Theme {
    fn dark() -> Self {
        Self {
            app_background: rgb(0x1e1e2e).into(),
            sidebar_background: rgb(0x181825).into(),
            surface_muted: rgb(0x313244).into(),
            surface_hover: rgb(0x45475a).into(),
            surface_selected: rgb(0x585b70).into(),
            composer_background: rgb(0x11111b).into(),
            border_subtle: rgb(0x313244).into(),
            border_strong: rgb(0x45475a).into(),
            text_primary: rgb(0xcdd6f4).into(),
            text_muted: rgb(0xa6adc8).into(),
            accent: rgb(0xcba6f7).into(),
            accent_hover: rgb(0xb4befe).into(),
            accent_text: rgb(0x11111b).into(),
        }
    }
}
