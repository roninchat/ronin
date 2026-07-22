//! Compact GPUI quick-mode overlay window.

use std::sync::mpsc;

use gpui::{
    div, prelude::*, px, size, App, Bounds, Context, FocusHandle, FontWeight, KeyDownEvent,
    MouseButton, SharedString, TitlebarOptions, Window, WindowBounds, WindowHandle, WindowOptions,
};
use ronin::{
    composer::ComposerEditor,
    plan_incoming_launch,
    quick_mode::{
        build_quick_chat_request, copy_answer_label, dismiss_hint, open_in_main_label,
        question_placeholder, quick_overlay_title, quick_window_size, resolve_quick_overlay_theme,
        save_to_current_label, save_to_thread_label, QuickModeState, QuickPhase, QuickStreamEvent,
        QUICK_WINDOW_WIDTH,
    },
    InstancePrimary,
};
use ronin_app::RoninShell;
use ronin_core::{
    ChatProvider, ChatStreamEvent, HttpOllamaProvider, OpenAiCompatibleProvider, ThemePreference,
};

use crate::RoninWindow;

/// Opens a centered compact quick-mode overlay window.
pub fn open_quick_overlay_window(
    cx: &mut App,
    shell: RoninShell,
    instance_primary: Option<InstancePrimary>,
    main_window: Option<WindowHandle<RoninWindow>>,
    current_thread_id: Option<String>,
) -> Result<WindowHandle<QuickModeWindow>, String> {
    let (w, h) = quick_window_size();
    let bounds = Bounds::centered(None, size(px(w), px(h)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from(quick_overlay_title())),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            cx.new(|cx| {
                let rem = 14.0;
                let mut composer = ComposerEditor::new();
                composer.set_font_metrics_from_rem(rem);
                composer.set_container_width(QUICK_WINDOW_WIDTH - 48.0);
                let _appearance_subscription =
                    cx.observe_window_appearance(window, |_this, _window, cx| {
                        cx.notify();
                    });
                QuickModeWindow {
                    shell,
                    state: QuickModeState::new(),
                    composer,
                    composer_focus: cx.focus_handle(),
                    stream_rx: None,
                    instance_primary,
                    main_window,
                    current_thread_id,
                    needs_initial_focus: true,
                    status_message: None,
                    _appearance_subscription,
                }
            })
        },
    )
    .map_err(|e| format!("failed to open quick overlay: {e}"))
}

/// Compact one-shot Q&A overlay (not the full Ronin shell).
pub struct QuickModeWindow {
    shell: RoninShell,
    state: QuickModeState,
    composer: ComposerEditor,
    composer_focus: FocusHandle,
    stream_rx: Option<mpsc::Receiver<QuickStreamEvent>>,
    instance_primary: Option<InstancePrimary>,
    main_window: Option<WindowHandle<RoninWindow>>,
    current_thread_id: Option<String>,
    needs_initial_focus: bool,
    status_message: Option<String>,
    _appearance_subscription: gpui::Subscription,
}

impl QuickModeWindow {
    fn pump_instance_ipc(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(primary) = self.instance_primary.as_mut() else {
            return false;
        };
        match primary.try_recv() {
            Ok(Some(incoming)) => {
                let plan = plan_incoming_launch(&incoming);
                tracing::info!(?plan, "quick overlay applying routed launch intent");
                if plan.create_new_thread || plan.open_quick_overlay {
                    // Raise this overlay; full-shell intents open main.
                    if plan.create_new_thread && !plan.open_quick_overlay {
                        let paths = self.shell.session().paths().clone();
                        let primary = self.instance_primary.take();
                        if let Ok(shell) = RoninShell::open(paths) {
                            let _ = crate::open_main_window(
                                cx,
                                shell,
                                primary,
                                plan.attach_paths.clone(),
                                None,
                            );
                        }
                    }
                }
                if plan.focus_window {
                    window.activate_window();
                    cx.activate(true);
                }
                cx.notify();
                true
            }
            Ok(None) => false,
            Err(e) => {
                tracing::warn!(%e, "quick overlay instance ipc recv failed");
                false
            }
        }
    }

    fn resolve_provider(&self) -> (Box<dyn ChatProvider + Send>, String) {
        let (provider_name, model) = self
            .shell
            .resolve_default_provider_and_model()
            .unwrap_or_else(|_| ("ollama".into(), "llama3.2".into()));
        let config = self.shell.session().load_config().ok();
        let provider: Box<dyn ChatProvider + Send> = if provider_name == "openai" {
            let base_url = config
                .as_ref()
                .and_then(|c| c.openai.as_ref())
                .and_then(|o| o.base_url.clone())
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
            Box::new(OpenAiCompatibleProvider::new(base_url))
        } else {
            let base_url = config
                .as_ref()
                .map(|c| c.ollama.base_url.clone())
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            Box::new(HttpOllamaProvider::new(base_url))
        };
        (provider, model)
    }

    fn pump_stream(&mut self) -> bool {
        let Some(rx) = self.stream_rx.as_ref() else {
            return false;
        };
        let mut still_active = true;
        loop {
            match rx.try_recv() {
                Ok(event) => {
                    let done = matches!(event, QuickStreamEvent::Done | QuickStreamEvent::Error(_));
                    self.state.apply_stream_event(event);
                    if done {
                        still_active = false;
                        break;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    if matches!(self.state.phase(), QuickPhase::Streaming) {
                        self.state.fail("stream disconnected");
                    }
                    still_active = false;
                    break;
                }
            }
        }
        if !still_active {
            self.stream_rx = None;
        }
        still_active
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        if !self.state.can_submit() && !matches!(self.state.phase(), QuickPhase::Composing) {
            return;
        }
        let question = self.composer.text().trim().to_string();
        if question.is_empty() {
            return;
        }
        self.state.set_question(question.clone());
        self.state.begin_streaming();
        self.status_message = None;

        let (provider, model) = self.resolve_provider();
        let system = self.shell.effective_system_prompt();
        let request = build_quick_chat_request(&question, &model, &system);
        let (tx, rx) = mpsc::channel();
        self.stream_rx = Some(rx);

        std::thread::spawn(move || match provider.stream_chat(&request) {
            Ok(iter) => {
                for event in iter {
                    match event {
                        ChatStreamEvent::Chunk(chunk) => {
                            if tx.send(QuickStreamEvent::Chunk(chunk)).is_err() {
                                return;
                            }
                        }
                        ChatStreamEvent::Error(err) => {
                            let _ = tx.send(QuickStreamEvent::Error(err));
                            return;
                        }
                    }
                }
                let _ = tx.send(QuickStreamEvent::Done);
            }
            Err(err) => {
                let _ = tx.send(QuickStreamEvent::Error(err.to_string()));
            }
        });
        cx.notify();
    }

    fn copy_answer(&mut self, cx: &mut Context<Self>) {
        if let Some(text) = self.state.copy_answer() {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
            self.status_message = Some("Copied".into());
            cx.notify();
        }
    }

    fn save_new_thread(&mut self, cx: &mut Context<Self>) {
        if !self.state.can_save() {
            return;
        }
        let question = self.state.question().to_string();
        let answer = self.state.answer().to_string();
        let result = if let Some(main) = self.main_window {
            main.update(cx, |win, _, _| {
                win.shell.save_quick_exchange(&question, &answer)
            })
            .ok()
            .and_then(Result::ok)
        } else {
            self.shell.save_quick_exchange(&question, &answer).ok()
        };
        match result {
            Some(thread) => {
                self.state.mark_saved(thread.id);
                self.status_message = Some("Saved".into());
            }
            None => self.status_message = Some("Save failed".into()),
        }
        cx.notify();
    }

    fn save_to_current(&mut self, cx: &mut Context<Self>) {
        if !self.state.can_save() {
            return;
        }
        let Some(thread_id) = self.current_thread_id.clone() else {
            return;
        };
        let question = self.state.question().to_string();
        let answer = self.state.answer().to_string();
        let ok = if let Some(main) = self.main_window {
            main.update(cx, |win, _, _| {
                win.shell
                    .save_quick_exchange_to_thread(&thread_id, &question, &answer)
            })
            .ok()
            .and_then(Result::ok)
            .is_some()
        } else {
            self.shell
                .save_quick_exchange_to_thread(&thread_id, &question, &answer)
                .is_ok()
        };
        if ok {
            self.state.mark_saved(thread_id);
            self.status_message = Some("Saved to current".into());
        } else {
            self.status_message = Some("Save failed".into());
        }
        cx.notify();
    }

    fn open_in_main(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread_id) = self.state.saved_thread_id().map(str::to_string) else {
            return;
        };
        if let Some(main) = self.main_window {
            let _ = main.update(cx, |win, main_window, cx| {
                let _ = win.shell.reload_threads();
                if let Err(e) = win.shell.select_thread(&thread_id) {
                    tracing::error!(%e, "failed to select quick-saved thread");
                }
                main_window.activate_window();
                cx.activate(true);
                cx.notify();
            });
            window.remove_window();
            return;
        }

        // Standalone --quick: open the full shell on the saved thread.
        let paths = self.shell.session().paths().clone();
        let primary = self.instance_primary.take();
        match RoninShell::open(paths) {
            Ok(shell) => {
                match crate::open_main_window(cx, shell, primary, Vec::new(), Some(thread_id)) {
                    Ok(_) => {
                        window.remove_window();
                    }
                    Err(e) => {
                        tracing::error!(%e, "failed to open main window from quick mode");
                        self.status_message = Some("Failed to open Ronin".into());
                        cx.notify();
                    }
                }
            }
            Err(e) => {
                tracing::error!(%e, "failed to open shell for main window from quick mode");
                self.status_message = Some("Failed to open Ronin".into());
                cx.notify();
            }
        }
    }

    fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state.dismiss();
        // If this was a standalone quick launch holding the instance lock, quit
        // the app when no main window is attached; otherwise just close overlay.
        if self.main_window.is_none() {
            cx.quit();
        } else {
            window.remove_window();
        }
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        if key == "escape" {
            self.dismiss(window, cx);
            return;
        }
        if matches!(self.state.phase(), QuickPhase::Streaming) {
            return;
        }
        if key == "enter"
            && !event.keystroke.modifiers.shift
            && (self.state.can_submit() || !self.composer.text().trim().is_empty())
        {
            // Sync question from composer before submit.
            self.state.set_question(self.composer.text().to_string());
            if self.state.can_submit() {
                self.submit(cx);
                return;
            }
        }
        if self.composer.on_key_down(event) {
            if matches!(
                self.state.phase(),
                QuickPhase::Composing | QuickPhase::Complete | QuickPhase::Failed { .. }
            ) {
                // Editing after complete starts a fresh compose cycle on next submit.
                if !matches!(self.state.phase(), QuickPhase::Streaming) {
                    let text = self.composer.text().to_string();
                    if matches!(
                        self.state.phase(),
                        QuickPhase::Complete | QuickPhase::Failed { .. }
                    ) && text != self.state.question()
                    {
                        self.state.set_question(text);
                    }
                }
            }
            cx.notify();
        }
    }
}

impl Render for QuickModeWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let streaming = self.pump_stream();
        let _ = self.pump_instance_ipc(window, cx);
        if self.needs_initial_focus {
            self.needs_initial_focus = false;
            window.focus(&self.composer_focus);
        }
        if streaming {
            cx.notify();
        }

        let theme_preference = self
            .shell
            .session()
            .load_config()
            .map(|c| c.theme)
            .unwrap_or(ThemePreference::System);
        let theme = resolve_quick_overlay_theme(theme_preference, window.appearance());

        let phase = self.state.phase();
        let answer = self.state.answer().to_string();
        let can_save = self.state.can_save();
        let can_open = self.state.can_open_in_main();
        let can_copy = self.state.copy_answer().is_some();
        let has_current = self.current_thread_id.is_some() && can_save;
        let status = self.status_message.clone();
        let error = match &phase {
            QuickPhase::Failed { message } => Some(message.clone()),
            _ => None,
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.app_background)
            .text_color(theme.text_primary)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .font_weight(FontWeight(600.))
                            .child(quick_overlay_title()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(dismiss_hint()),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .child(
                        div()
                            .rounded_lg()
                            .border_1()
                            .border_color(theme.border_strong)
                            .bg(theme.composer_background)
                            .p_3()
                            .min_h(px(72.0))
                            .track_focus(&self.composer_focus)
                            .on_key_down(cx.listener(Self::on_key_down))
                            .child(self.composer.render_text(
                                question_placeholder(),
                                theme.text_primary,
                                theme.text_muted,
                                theme.accent,
                            )),
                    )
                    .child(
                        div()
                            .id("quick-answer")
                            .flex_1()
                            .rounded_lg()
                            .border_1()
                            .border_color(theme.border_subtle)
                            .bg(theme.surface_muted)
                            .p_3()
                            .overflow_y_scroll()
                            .child(match &phase {
                                QuickPhase::Composing if answer.is_empty() => div()
                                    .text_color(theme.text_muted)
                                    .child("Answer appears here…"),
                                QuickPhase::Streaming => div().child(if answer.is_empty() {
                                    "Thinking…".to_string()
                                } else {
                                    answer
                                }),
                                QuickPhase::Failed { .. } => div()
                                    .text_color(theme.accent)
                                    .child(error.unwrap_or_else(|| "Failed".into())),
                                _ => div().child(answer),
                            }),
                    )
                    .child({
                        let mut actions = div().flex().flex_row().gap_2().items_center();
                        if can_copy {
                            actions = actions.child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.surface_hover)
                                    .cursor_pointer()
                                    .child(copy_answer_label())
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| this.copy_answer(cx)),
                                    ),
                            );
                        }
                        if can_save {
                            actions = actions.child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.accent)
                                    .text_color(theme.accent_text)
                                    .cursor_pointer()
                                    .child(save_to_thread_label())
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| this.save_new_thread(cx)),
                                    ),
                            );
                        }
                        if has_current {
                            actions = actions.child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.surface_hover)
                                    .cursor_pointer()
                                    .child(save_to_current_label())
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| this.save_to_current(cx)),
                                    ),
                            );
                        }
                        if can_open {
                            actions = actions.child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.accent_hover)
                                    .text_color(theme.accent_text)
                                    .cursor_pointer()
                                    .child(open_in_main_label())
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                            this.open_in_main(window, cx)
                                        }),
                                    ),
                            );
                        }
                        if let Some(msg) = status {
                            actions = actions
                                .child(div().text_xs().text_color(theme.text_muted).child(msg));
                        }
                        actions
                    }),
            )
    }
}
