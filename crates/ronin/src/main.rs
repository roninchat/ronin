use std::process::ExitCode;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, FocusHandle, FontWeight,
    KeyDownEvent, MouseButton, MouseDownEvent, MouseUpEvent, SharedString, TitlebarOptions, Window,
    WindowBounds, WindowOptions,
};
use ronin::{parse_launch_intent, ronin_paths, LaunchIntent, LauncherError};
use ronin_app::{ProviderStatus, RoninAppError, RoninShell, ShellState};
use ronin_core::{HttpOllamaProvider, MessageRole};

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
    let shell = match intent {
        LaunchIntent::OpenPersisted => RoninShell::open(paths)?,
        LaunchIntent::NewThread => RoninShell::open_with_new_thread(paths)?,
        LaunchIntent::OpenWithOllama => RoninShell::open_with_ollama(paths)?,
    };
    let uses_ollama = matches!(intent, LaunchIntent::OpenWithOllama);
    tracing::info!(intent = ?intent, "ronin native shell starting");

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
                cx.new(|cx| RoninWindow {
                    shell,
                    composer_text: String::new(),
                    composer_focus: cx.focus_handle(),
                    chat_provider: if uses_ollama {
                        Some(HttpOllamaProvider::new("http://localhost:11434"))
                    } else {
                        None
                    },
                    needs_initial_focus: true,
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
    composer_text: String,
    composer_focus: FocusHandle,
    chat_provider: Option<HttpOllamaProvider>,
    needs_initial_focus: bool,
}

impl RoninWindow {
    fn send_current_message(&mut self, cx: &mut Context<Self>) {
        let thread_id = match self.shell.state().selected_thread_id.clone() {
            Some(id) => id,
            None => return,
        };
        let text = std::mem::take(&mut self.composer_text);
        if text.trim().is_empty() {
            return;
        }

        let model = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } => model.clone(),
            _ => {
                if let Err(err) = self.shell.send_message(&thread_id, &text) {
                    tracing::error!(%err, "failed to send message");
                }
                cx.notify();
                return;
            }
        };

        match self.chat_provider.take() {
            Some(provider) => {
                let result =
                    self.shell
                        .begin_streaming(&thread_id, &text, Box::new(provider), &model);
                match result {
                    Ok(()) => {
                        cx.notify();
                    }
                    Err(err) => {
                        tracing::error!(%err, "failed to begin streaming");
                        self.chat_provider =
                            Some(HttpOllamaProvider::new("http://localhost:11434"));
                        cx.notify();
                    }
                }
            }
            None => {
                if let Err(err) = self.shell.send_message(&thread_id, &text) {
                    tracing::error!(%err, "failed to send message");
                }
                cx.notify();
            }
        }
    }

    fn pump_streaming(&mut self) -> bool {
        let active = self.shell.poll_streaming();
        if !active && self.chat_provider.is_none() {
            self.chat_provider = Some(HttpOllamaProvider::new("http://localhost:11434"));
        }
        active
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

    fn focus_composer(&mut self, _: &MouseDownEvent, window: &mut Window, _cx: &mut Context<Self>) {
        window.focus(&self.composer_focus);
    }

    fn on_composer_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystroke = &event.keystroke;
        let has_text_mod =
            keystroke.modifiers.alt || keystroke.modifiers.control || keystroke.modifiers.platform;

        if has_text_mod {
            return;
        }

        match keystroke.key.as_str() {
            "enter" => {
                if keystroke.modifiers.shift {
                    self.composer_text.push('\n');
                    cx.notify();
                    return;
                }
                self.send_current_message(cx);
            }
            "backspace" => {
                self.composer_text.pop();
                cx.notify();
            }
            "space" => {
                self.composer_text.push(' ');
                cx.notify();
            }
            key => {
                if key.len() == 1 {
                    self.composer_text.push_str(key);
                    cx.notify();
                }
            }
        }
    }

    fn render_sidebar(
        &self,
        state: &ShellState,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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
        }
    }

    fn render_messages(&self, state: &ShellState, theme: &M0Theme) -> impl IntoElement {
        let messages = match state.messages.as_ref() {
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

        let message_elements = messages.into_iter().filter_map(|msg| {
            if msg.role == MessageRole::System {
                return None;
            }

            let (label, bg) = match msg.role {
                MessageRole::User => ("You", theme.surface_muted),
                MessageRole::Assistant => ("Assistant", theme.surface_selected),
                MessageRole::System => unreachable!(),
            };

            Some(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .mb_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .mb_1()
                            .child(label),
                    )
                    .child(
                        div()
                            .max_w(px(680.0))
                            .rounded_lg()
                            .px_4()
                            .py_3()
                            .bg(bg)
                            .text_color(theme.text_primary)
                            .child(msg.content),
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
        let display_text = if self.composer_text.is_empty() {
            "Ask Ronin anything…".to_string()
        } else {
            self.composer_text.clone()
        };

        let text_color = if self.composer_text.is_empty() {
            theme.text_muted
        } else {
            theme.text_primary
        };

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

        div().p_6().child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .flex_1()
                        .rounded_xl()
                        .border_2()
                        .border_color(border_color)
                        .bg(theme.composer_background)
                        .p_4()
                        .text_color(text_color)
                        .child(display_text)
                        .id("composer")
                        .cursor_text()
                        .track_focus(&self.composer_focus)
                        .on_key_down(cx.listener(Self::on_composer_key_down))
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::focus_composer)),
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
}

impl Render for RoninWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Poll for streaming chunks before rendering.
        let streaming_active = self.pump_streaming();

        // Auto-focus on first render so the text cursor is visible.
        if self.needs_initial_focus {
            self.needs_initial_focus = false;
            window.focus(&self.composer_focus);
        }

        let theme = M0Theme::dark();
        let state = self.shell.state();
        let composer_focused = self.composer_focus.is_focused(window);

        let ui = div()
            .size_full()
            .flex()
            .bg(theme.app_background)
            .text_color(theme.text_primary)
            .font_family("Inter")
            .child(self.render_sidebar(state, &theme, cx))
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
                            .child(
                                Self::current_thread_title(state)
                                    .map(|t| t.to_string())
                                    .unwrap_or_else(|| "New Chat".to_string()),
                            ),
                    )
                    .child(self.render_messages(state, &theme))
                    .child(self.render_composer(&theme, composer_focused, cx)),
            );

        // If streaming is active, schedule a repaint on the next animation frame.
        // NOTE: cx.notify() is a no-op here because GPUI's refresh() guard
        // skips invalidation while the window is already drawing (inside render).
        // request_animation_frame() defers the notify to the next frame callback,
        // reliably driving continuous repaints during streaming.
        if streaming_active {
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
