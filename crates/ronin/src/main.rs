use std::process::ExitCode;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, MouseButton, SharedString,
    TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use ronin::{parse_launch_intent, ronin_paths, LaunchIntent, LauncherError};
use ronin_app::{ProviderStatus, RoninAppError, RoninShell};

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
            |_, cx| cx.new(|_| RoninWindow { shell }),
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
}

impl RoninWindow {
    fn create_new_thread(
        &mut self,
        _: &gpui::MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.shell.create_new_thread() {
            Ok(_) => cx.notify(),
            Err(error) => tracing::error!(%error, "failed to create thread from sidebar"),
        }
    }

    fn select_thread(
        &mut self,
        thread_id: String,
        _: &gpui::MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.shell.select_thread(&thread_id) {
            Ok(()) => cx.notify(),
            Err(error) => tracing::error!(%error, "failed to select thread from sidebar"),
        }
    }
}

impl Render for RoninWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = M0Theme::dark();
        let state = self.shell.state();
        let selected_thread_id = state.selected_thread_id.as_deref();
        let selected_thread = state
            .threads
            .iter()
            .find(|thread| Some(thread.id.as_str()) == selected_thread_id);
        let status = match state.provider_status {
            ProviderStatus::NotConfigured => "Provider: not configured\nModel: not selected".to_string(),
            ProviderStatus::OllamaOffline => {
                "Provider: ollama\nModel: offline\n\nollama not reachable — is the server running?".to_string()
            }
            ProviderStatus::OllamaOnline { ref model } => {
                format!("Provider: ollama\nModel: {model}")
            }
            ProviderStatus::OllamaNoModels => {
                "Provider: ollama\nModel: none\n\nNo models installed.\nTry: ollama pull llama3.2".to_string()
            }
        };

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
            .size_full()
            .flex()
            .bg(theme.app_background)
            .text_color(theme.text_primary)
            .font_family("Inter")
            .child(
                // Sidebar
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
                            .font_weight(gpui::FontWeight(600.))
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
                            .font_weight(gpui::FontWeight(500.))
                            .child("New Chat")
                            .hover(|style| style.bg(theme.accent_hover).cursor_pointer())
                            .on_mouse_up(MouseButton::Left, on_new_chat),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text_muted)
                                    .child("Threads"),
                            )
                            .children(thread_rows),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .rounded_lg()
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.surface_muted)
                            .p_3()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child(status),
                    ),
            )
            .child(
                // Main view
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
                                selected_thread
                                    .map(|thread| thread.title.clone())
                                    .unwrap_or_else(|| "New Chat".to_string()),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_6()
                            .text_color(theme.text_muted)
                            .child("Start a conversation. Messages will appear here."),
                    )
                    .child(
                        div().p_6().child(
                            div()
                                .rounded_xl()
                                .border_1()
                                .border_color(theme.border_strong)
                                .bg(theme.composer_background)
                                .p_4()
                                .text_color(theme.text_muted)
                                .child("Ask Ronin anything…"),
                        ),
                    ),
            )
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
            // Catppuccin Mocha
            // https://catppuccin.com/palette
            app_background: rgb(0x1e1e2e).into(),      // base
            sidebar_background: rgb(0x181825).into(),  // mantle
            surface_muted: rgb(0x313244).into(),       // surface0
            surface_hover: rgb(0x45475a).into(),       // surface1
            surface_selected: rgb(0x585b70).into(),    // surface2
            composer_background: rgb(0x11111b).into(), // crust
            border_subtle: rgb(0x313244).into(),       // surface0
            border_strong: rgb(0x45475a).into(),       // surface1
            text_primary: rgb(0xcdd6f4).into(),        // text
            text_muted: rgb(0xa6adc8).into(),          // subtext0
            accent: rgb(0xcba6f7).into(),              // mauve
            accent_hover: rgb(0xb4befe).into(),        // lavender
            accent_text: rgb(0x11111b).into(),         // crust
        }
    }
}
