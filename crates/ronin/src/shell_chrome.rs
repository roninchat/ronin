//! Titlebar, icon rail, overflow menus, palettes, settings, and the first-run coach.

use gpui::{
    deferred, div, prelude::*, px, App, Context, Decorations, FontWeight, KeyDownEvent,
    MouseButton, MouseDownEvent, SharedString, Window, WindowControlArea,
};
use ronin::chrome::{
    message_overflow_items_with_flags, thread_overflow_items, window_overflow_items, OverflowItem,
    ICON_RAIL_WIDTH, TITLEBAR_HEIGHT,
};
use ronin::command_palette::{
    command_catalog, filter_items, quick_items, PaletteAction, PaletteItem, PaletteMode,
};
use ronin::icons::{icon, IconName};
use ronin::plus_menu::{plus_menu_items, PlusMenuItem};
use ronin::settings_view::{
    artifacts_toggle_label, auto_title_toggle_label, memories_toggle_label,
    notifications_toggle_label, scale_label, section_label, shortcut_hints_toggle_label,
    theme_label, visible_sections, SettingsSection,
};
use ronin::shortcut_coach::{first_run_steps, should_show_coach};
use ronin::theme::M0Theme;
use ronin_app::ProviderStatus;
use ronin_core::{clamp_ui_scale, ThemePreference, UI_SCALE_DEFAULT};

use super::{ChromeMenu, RoninWindow};

fn theme_preference_label(pref: ThemePreference) -> &'static str {
    match pref {
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
        ThemePreference::System => "System",
    }
}

pub(crate) fn icon_hit(
    id: impl Into<SharedString>,
    name: IconName,
    color: gpui::Hsla,
    hover_bg: gpui::Hsla,
    listener: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id.into())
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(move |style| style.bg(hover_bg))
        .active(move |style| style.bg(hover_bg))
        .on_mouse_down(MouseButton::Left, listener)
        .child(icon(name, color, 16.0))
}

fn overflow_row(
    item: OverflowItem,
    theme: &M0Theme,
    listener: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let mut row = div()
        .id(SharedString::from(item.id))
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap_3()
        .px_3()
        .py_1()
        .rounded_md()
        .cursor_pointer()
        .hover(|style| style.bg(theme.surface_hover))
        .on_mouse_down(MouseButton::Left, listener)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(icon(item.icon, theme.text_primary, 14.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .child(item.label),
                ),
        );
    if let Some(keys) = item.keys {
        row = row.child(div().text_xs().text_color(theme.text_muted).child(keys));
    }
    row
}

impl RoninWindow {
    pub(crate) fn refresh_ui_prefs(&mut self) {
        let Ok(config) = self.shell.session().load_config() else {
            return;
        };
        self.theme_preference = config.theme;
        self.ui_scale = clamp_ui_scale(config.ui.scale);
        self.memories_enabled = config.features.memories;
        self.artifacts_enabled = config.features.artifacts;
        self.show_shortcut_hints = config.ui.show_shortcut_hints;
        self.coach_seen = config.ui.shortcut_coach_seen;
        self.notifications_enabled = config.notifications.enabled;
        self.auto_title = config.general.auto_title;
        self.composer_rem = 14.0 * self.ui_scale;
        self.composer.set_font_metrics_from_rem(self.composer_rem);
    }

    pub(crate) fn apply_zoom_delta(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.apply_zoom_value(self.ui_scale + delta, cx);
    }

    pub(crate) fn apply_zoom_value(&mut self, scale: f32, cx: &mut Context<Self>) {
        if let Err(e) = self.shell.set_ui_scale(scale) {
            tracing::error!(%e, "failed to persist ui scale");
            return;
        }
        self.refresh_ui_prefs();
        cx.notify();
    }

    pub(crate) fn open_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.command_palette.close();
        self.chrome_menu = None;
        self.settings.open();
        window.focus(&self.composer_focus);
        cx.notify();
    }

    pub(crate) fn close_settings(&mut self, cx: &mut Context<Self>) {
        self.settings.close();
        cx.notify();
    }

    /// Moves the settings section without closing the overlay.
    pub(crate) fn cycle_settings_section(&mut self, delta: i32, cx: &mut Context<Self>) {
        let sections = visible_sections(self.settings.search());
        if sections.is_empty() {
            return;
        }
        let current = sections
            .iter()
            .position(|section| *section == self.settings.section())
            .unwrap_or(0);
        let len = sections.len() as i32;
        let next = (current as i32 + delta).rem_euclid(len) as usize;
        self.settings.set_section(sections[next]);
        cx.notify();
    }

    pub(crate) fn toggle_quick_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.command_palette.is_open() && self.command_palette.mode() == PaletteMode::Quick {
            self.close_palette(window, cx);
            return;
        }
        self.open_palette(PaletteMode::Quick, window, cx);
    }

    pub(crate) fn toggle_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.command_palette.is_open() && self.command_palette.mode() == PaletteMode::Commands {
            self.close_palette(window, cx);
            return;
        }
        self.open_palette(PaletteMode::Commands, window, cx);
    }

    fn open_palette(&mut self, mode: PaletteMode, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.close();
        self.chrome_menu = None;
        match mode {
            PaletteMode::Quick => self.command_palette.open_quick(),
            PaletteMode::Commands => self.command_palette.open_commands(),
        }
        self.palette_editor.set_text(String::new());
        self.palette_editor
            .set_font_metrics_from_rem(self.composer_rem);
        self.palette_editor.set_container_width(420.0);
        window.focus(&self.palette_focus);
        cx.notify();
    }

    pub(crate) fn close_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.command_palette.close();
        window.focus(&self.composer_focus);
        cx.notify();
    }

    pub(crate) fn toggle_window_menu(&mut self, cx: &mut Context<Self>) {
        self.chrome_menu = match self.chrome_menu {
            Some(ChromeMenu::Window) => None,
            _ => Some(ChromeMenu::Window),
        };
        cx.notify();
    }

    pub(crate) fn dismiss_chrome_overlays(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut changed = false;
        if self.command_palette.is_open() {
            self.command_palette.close();
            changed = true;
        }
        if self.settings.is_open() {
            self.settings.close();
            changed = true;
        }
        if self.chrome_menu.take().is_some() {
            changed = true;
        }
        if changed {
            window.focus(&self.composer_focus);
            cx.notify();
        }
    }

    fn palette_items(&self) -> Vec<PaletteItem> {
        let titles: Vec<(usize, String)> = self
            .shell
            .state()
            .threads
            .iter()
            .enumerate()
            .map(|(index, thread)| (index, thread.title.clone()))
            .collect();
        let title_refs: Vec<(usize, &str)> = titles
            .iter()
            .map(|(index, title)| (*index, title.as_str()))
            .collect();
        let catalog = match self.command_palette.mode() {
            PaletteMode::Quick => quick_items(&title_refs),
            PaletteMode::Commands => command_catalog(),
        };
        filter_items(&catalog, self.command_palette.query())
    }

    pub(crate) fn confirm_palette_selection(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let items = self.palette_items();
        if items.is_empty() {
            return;
        }
        let index = self
            .command_palette
            .selected_index()
            .min(items.len().saturating_sub(1));
        let Some(item) = items.get(index).cloned() else {
            return;
        };
        self.command_palette.close();
        self.apply_palette_action(item, window, cx);
    }

    pub(crate) fn apply_palette_action(
        &mut self,
        item: PaletteItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match item.action {
            PaletteAction::NewThread => self.create_new_thread_shortcut(window, cx),
            PaletteAction::ToggleSidebar => self.toggle_sidebar(cx),
            PaletteAction::OpenSettings => self.open_settings(window, cx),
            PaletteAction::OpenSearch => self.toggle_search_panel(window, cx),
            PaletteAction::Screenshot => self.take_screenshot_action(cx),
            PaletteAction::ScreenshotWindow => self.take_window_screenshot_action(cx),
            PaletteAction::FocusComposer => self.focus_composer_shortcut(window, cx),
            PaletteAction::ToggleHelp => {
                let count = self.thread_count();
                let input = ronin::keyboard_nav::KeyInput {
                    key: "/",
                    control: true,
                    shift: false,
                    alt: false,
                };
                let (_, action) = self.keyboard_nav.handle_key(input, count);
                self.apply_nav_action(action, window, cx);
            }
            PaletteAction::ZoomIn => self.apply_zoom_delta(0.1, cx),
            PaletteAction::ZoomOut => self.apply_zoom_delta(-0.1, cx),
            PaletteAction::ZoomReset => self.apply_zoom_value(UI_SCALE_DEFAULT, cx),
            PaletteAction::OpenMemories => self.toggle_memories_panel(cx),
            PaletteAction::OpenArtifacts => self.toggle_artifacts_panel(cx),
            PaletteAction::SelectThread => {
                if let Some(index) = item.thread_index {
                    let thread_id = self.shell.state().threads.get(index).map(|t| t.id.clone());
                    if let Some(id) = thread_id {
                        if let Err(e) = self.shell.select_thread(&id) {
                            tracing::error!(%e, "failed to select thread from palette");
                        } else {
                            self.focus_composer_shortcut(window, cx);
                        }
                    }
                }
            }
        }
        cx.notify();
    }

    pub(crate) fn toggle_memories_panel(&mut self, cx: &mut Context<Self>) {
        if !self.memories_enabled {
            self.settings.set_section(SettingsSection::General);
            self.settings.open();
            cx.notify();
            return;
        }
        self.memories_panel_open = !self.memories_panel_open;
        if self.memories_panel_open {
            self.memory_management.open();
        } else {
            self.memory_management.close();
        }
        cx.notify();
    }

    pub(crate) fn toggle_artifacts_panel(&mut self, cx: &mut Context<Self>) {
        if !self.artifacts_enabled {
            self.settings.set_section(SettingsSection::General);
            self.settings.open();
            cx.notify();
            return;
        }
        self.artifacts_panel_open = !self.artifacts_panel_open;
        cx.notify();
    }

    pub(crate) fn apply_overflow_item(
        &mut self,
        item_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let menu = self.chrome_menu.take();
        match (menu, item_id) {
            (Some(ChromeMenu::Window), "settings") => self.open_settings(window, cx),
            (Some(ChromeMenu::Window), "shortcuts") => {
                let input = ronin::keyboard_nav::KeyInput {
                    key: "/",
                    control: true,
                    shift: false,
                    alt: false,
                };
                let count = self.thread_count();
                let (_, action) = self.keyboard_nav.handle_key(input, count);
                self.apply_nav_action(action, window, cx);
            }
            (Some(ChromeMenu::Window), "zoom-in") => self.apply_zoom_delta(0.1, cx),
            (Some(ChromeMenu::Window), "zoom-out") => self.apply_zoom_delta(-0.1, cx),
            (Some(ChromeMenu::Window), "zoom-reset") => self.apply_zoom_value(UI_SCALE_DEFAULT, cx),
            (Some(ChromeMenu::Window), "toggle-sidebar") => self.toggle_sidebar(cx),
            (Some(ChromeMenu::Thread { id }), "rename") => {
                self.begin_thread_rename(&id, window, cx);
            }
            (Some(ChromeMenu::Thread { id }), "export") => {
                let title = self
                    .shell
                    .state()
                    .threads
                    .iter()
                    .find(|thread| thread.id == id)
                    .map(|thread| thread.title.clone())
                    .unwrap_or_default();
                self.copy_to_clipboard(id, title, cx);
            }
            (Some(ChromeMenu::Thread { .. }), "delete") => {
                tracing::info!("thread delete is not wired in this shell");
                cx.notify();
            }
            (Some(ChromeMenu::Message { content, id, .. }), "copy") => {
                self.copy_to_clipboard(id, content, cx);
            }
            (
                Some(ChromeMenu::Message {
                    content,
                    is_assistant,
                    ..
                }),
                "save-memory",
            ) if is_assistant => {
                self.save_as_memory(content, cx);
            }
            (
                Some(ChromeMenu::Message {
                    thread_id,
                    id,
                    content,
                    is_assistant,
                    ..
                }),
                "save-artifact",
            ) if is_assistant => {
                self.save_as_artifact(thread_id, id, content, cx);
            }
            (Some(ChromeMenu::Message { id, is_failed, .. }), "retry") if is_failed => {
                if !self.shell.is_generation_active() {
                    self.retry_failed_message(id, cx);
                }
            }
            (
                Some(ChromeMenu::Message {
                    is_last_assistant, ..
                }),
                "regenerate",
            ) if is_last_assistant => {
                if !self.shell.is_generation_active() {
                    self.regenerate_last_assistant(cx);
                }
            }
            (
                Some(ChromeMenu::Message {
                    id,
                    content,
                    can_edit,
                    ..
                }),
                "edit",
            ) if can_edit => {
                if !self.shell.is_generation_active() {
                    self.begin_message_edit(&id, &content, window, cx);
                }
            }
            _ => cx.notify(),
        }
    }

    pub(crate) fn apply_plus_item(
        &mut self,
        item: PlusMenuItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.chrome_menu = None;
        match item.insert {
            "@screenshot" => self.take_screenshot_action(cx),
            "@screenshot:window" => self.take_window_screenshot_action(cx),
            insert => {
                self.composer.insert_str(insert);
                self.focus_composer_shortcut(window, cx);
            }
        }
        cx.notify();
    }

    pub(crate) fn on_palette_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        let count = self.palette_items().len();
        match key {
            "escape" => {
                self.close_palette(window, cx);
            }
            "enter" => self.confirm_palette_selection(window, cx),
            "up" => {
                self.command_palette.move_sel(-1, count);
                cx.notify();
            }
            "down" => {
                self.command_palette.move_sel(1, count);
                cx.notify();
            }
            _ => {
                if self.palette_editor.on_key_down(event) {
                    self.command_palette
                        .set_query(self.palette_editor.text().to_string());
                    cx.notify();
                    return;
                }
                if event.keystroke.modifiers.control {
                    return;
                }
                if let Some(ref kc) = event.keystroke.key_char {
                    for ch in kc.chars() {
                        if !ch.is_control() {
                            self.palette_editor.insert_char(ch);
                        }
                    }
                    self.command_palette
                        .set_query(self.palette_editor.text().to_string());
                    cx.notify();
                }
            }
        }
    }

    pub(crate) fn cycle_theme(&mut self, cx: &mut Context<Self>) {
        let next = match self.theme_preference {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        };
        if let Ok(mut config) = self.shell.session().load_config() {
            config.theme = next;
            if let Err(e) = self.shell.session().save_config(&config) {
                tracing::error!(%e, "failed to save theme preference");
            } else {
                self.theme_preference = next;
            }
        }
        cx.notify();
    }

    pub(crate) fn persist_config_flag(
        &mut self,
        write: impl FnOnce(&mut ronin_core::RoninConfig),
        cx: &mut Context<Self>,
    ) {
        match self.shell.session().load_config() {
            Ok(mut config) => {
                write(&mut config);
                if let Err(e) = self.shell.session().save_config(&config) {
                    tracing::error!(%e, "failed to save config");
                } else {
                    self.refresh_ui_prefs();
                }
            }
            Err(e) => tracing::error!(%e, "failed to load config"),
        }
        cx.notify();
    }

    pub(crate) fn dismiss_coach(&mut self, cx: &mut Context<Self>) {
        if let Err(e) = self.shell.mark_shortcut_coach_seen() {
            tracing::error!(%e, "failed to persist shortcut coach dismissal");
        }
        self.coach_seen = true;
        cx.notify();
    }

    pub(crate) fn render_titlebar(
        &self,
        theme: &M0Theme,
        title: &str,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let model_label = self
            .active_model_name()
            .unwrap_or("Select model")
            .to_string();
        let client_controls = matches!(window.window_decorations(), Decorations::Client { .. });

        let mut bar = div()
            .id("titlebar")
            .w_full()
            .h(px(TITLEBAR_HEIGHT))
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .gap_1()
            .bg(theme.sidebar_background)
            .border_b_1()
            .border_color(theme.border_subtle)
            .window_control_area(WindowControlArea::Drag)
            .child(icon_hit(
                "titlebar-hamburger",
                if self.sidebar_collapsed {
                    IconName::PanelLeft
                } else {
                    IconName::PanelLeftClose
                },
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, _, cx| this.toggle_sidebar(cx)),
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .px_2()
                    .text_sm()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.text_primary)
                    .whitespace_nowrap()
                    .child(title.to_string()),
            )
            .child(
                div()
                    .id("titlebar-model")
                    .rounded_md()
                    .px_2()
                    .py_1()
                    .text_xs()
                    .bg(theme.surface_muted)
                    .text_color(theme.text_primary)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.surface_hover))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.open_model_picker();
                            cx.notify();
                        }),
                    )
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .child(model_label)
                    .child(icon(IconName::ChevronsUpDown, theme.text_muted, 12.0)),
            )
            .child(
                div()
                    .relative()
                    .child(icon_hit(
                        "titlebar-overflow",
                        IconName::EllipsisVertical,
                        theme.text_primary,
                        theme.surface_hover,
                        cx.listener(|this, _, _, cx| this.toggle_window_menu(cx)),
                    ))
                    .when(matches!(self.chrome_menu, Some(ChromeMenu::Window)), |el| {
                        el.child(self.render_window_overflow(theme, cx))
                    }),
            );

        if client_controls {
            bar = bar
                .child(
                    div()
                        .window_control_area(WindowControlArea::Min)
                        .child(icon_hit(
                            "win-min",
                            IconName::Minus,
                            theme.text_muted,
                            theme.surface_hover,
                            |_, window, _| window.minimize_window(),
                        )),
                )
                .child(
                    div()
                        .window_control_area(WindowControlArea::Max)
                        .child(icon_hit(
                            "win-max",
                            IconName::Maximize,
                            theme.text_muted,
                            theme.surface_hover,
                            |_, window, _| window.zoom_window(),
                        )),
                )
                .child(
                    div()
                        .window_control_area(WindowControlArea::Close)
                        .child(icon_hit(
                            "win-close",
                            IconName::X,
                            theme.text_muted,
                            theme.surface_hover,
                            |_, _, cx| cx.quit(),
                        )),
                );
        }

        bar
    }

    fn render_window_overflow(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let mut menu = self.menu_surface(theme);
        for item in window_overflow_items() {
            let id = item.id;
            menu = menu.child(overflow_row(
                *item,
                theme,
                cx.listener(move |this, _, window, cx| {
                    this.apply_overflow_item(id, window, cx);
                }),
            ));
        }
        deferred(menu).with_priority(1)
    }

    pub(crate) fn render_thread_overflow(
        &self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut menu = self.menu_surface(theme);
        for item in thread_overflow_items() {
            let id = item.id;
            menu = menu.child(overflow_row(
                *item,
                theme,
                cx.listener(move |this, _, window, cx| {
                    this.apply_overflow_item(id, window, cx);
                }),
            ));
        }
        deferred(menu).with_priority(1)
    }

    pub(crate) fn render_message_overflow(
        &self,
        theme: &M0Theme,
        is_assistant: bool,
        is_failed: bool,
        is_last_assistant: bool,
        can_edit: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let items = message_overflow_items_with_flags(
            is_assistant,
            is_failed,
            is_last_assistant,
            self.memories_enabled,
            self.artifacts_enabled,
            is_failed,
            is_last_assistant,
            can_edit,
        );
        let mut menu = self.menu_surface(theme);
        for item in items {
            let id = item.id;
            menu = menu.child(overflow_row(
                item,
                theme,
                cx.listener(move |this, _, window, cx| {
                    this.apply_overflow_item(id, window, cx);
                }),
            ));
        }
        deferred(menu).with_priority(1)
    }

    fn menu_surface(&self, theme: &M0Theme) -> gpui::Div {
        div()
            .absolute()
            .top_full()
            .right_0()
            .mt_1()
            .min_w(px(220.0))
            .rounded_lg()
            .border_1()
            .border_color(theme.border_subtle)
            .bg(theme.sidebar_background)
            .p_1()
            .shadow(
                ronin::visual_polish::elevation_style(
                    ronin::visual_polish::Elevation::High,
                    theme.color_scheme,
                )
                .box_shadows(),
            )
            .flex()
            .flex_col()
            .gap_1()
            .occlude()
    }

    pub(crate) fn render_icon_rail(
        &self,
        theme: &M0Theme,
        sidebar_focused: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_border = if sidebar_focused {
            theme.accent
        } else {
            theme.border_subtle
        };
        let mut rail = div()
            .id("icon-rail")
            .w(px(ICON_RAIL_WIDTH))
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .py_2()
            .bg(theme.sidebar_background)
            .border_r_1()
            .border_color(focus_border)
            .track_focus(&self.sidebar_focus)
            .child(icon_hit(
                "rail-new",
                IconName::MessageSquarePlus,
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, window, cx| this.create_new_thread_shortcut(window, cx)),
            ))
            .child(icon_hit(
                "rail-search",
                IconName::Search,
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, window, cx| this.toggle_search_panel(window, cx)),
            ));
        if self.memories_enabled {
            rail = rail.child(icon_hit(
                "rail-memories",
                IconName::Sparkles,
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, _, cx| this.toggle_memories_panel(cx)),
            ));
        }
        if self.artifacts_enabled {
            rail = rail.child(icon_hit(
                "rail-artifacts",
                IconName::Box,
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, _, cx| this.toggle_artifacts_panel(cx)),
            ));
        }
        rail.child(div().flex_1())
            .child(self.render_provider_dot(theme, cx))
            .child(icon_hit(
                "rail-user",
                IconName::User,
                theme.text_primary,
                theme.surface_hover,
                cx.listener(|this, _, window, cx| this.open_settings(window, cx)),
            ))
    }

    fn render_provider_dot(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let (label, ok) = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } | ProviderStatus::OpenAiReady { model } => {
                (model.clone(), true)
            }
            ProviderStatus::OllamaOffline => ("Ollama offline".into(), false),
            ProviderStatus::OllamaNoModels => ("No models".into(), false),
            ProviderStatus::OpenAiError { .. } | ProviderStatus::OpenAiNotConfigured => {
                ("OpenAI".into(), false)
            }
            ProviderStatus::NotConfigured => ("No provider".into(), false),
        };
        let color = if ok { theme.accent } else { theme.text_muted };
        div()
            .id("rail-provider")
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .hover(|style| style.bg(theme.surface_hover))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.open_model_picker();
                    cx.notify();
                }),
            )
            .child(div().size(px(8.0)).rounded_full().bg(color))
            .child(div().invisible().child(label))
    }

    pub(crate) fn render_plus_menu(
        &self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut menu = div()
            .absolute()
            .bottom_full()
            .left_0()
            .mb_1()
            .min_w(px(220.0))
            .rounded_lg()
            .border_1()
            .border_color(theme.border_subtle)
            .bg(theme.sidebar_background)
            .p_1()
            .shadow(
                ronin::visual_polish::elevation_style(
                    ronin::visual_polish::Elevation::High,
                    theme.color_scheme,
                )
                .box_shadows(),
            )
            .flex()
            .flex_col()
            .gap_1()
            .occlude();
        for item in plus_menu_items(self.memories_enabled, self.artifacts_enabled) {
            menu = menu.child(
                div()
                    .id(SharedString::from(item.insert))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.surface_hover))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            this.apply_plus_item(item, window, cx);
                        }),
                    )
                    .child(icon(item.icon, theme.text_primary, 14.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(item.label),
                    ),
            );
        }
        deferred(menu).with_priority(1)
    }

    pub(crate) fn render_command_palette(
        &mut self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let items = self.palette_items();
        let selected = self.command_palette.selected_index();
        let heading = match self.command_palette.mode() {
            PaletteMode::Quick => "Jump to thread",
            PaletteMode::Commands => "Commands",
        };
        let mut list = div()
            .id("palette-list")
            .flex()
            .flex_col()
            .max_h(px(320.0))
            .overflow_y_scroll();
        if items.is_empty() {
            list = list.child(
                div()
                    .px_3()
                    .py_2()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("No matches"),
            );
        } else {
            for (index, item) in items.iter().enumerate() {
                let bg = if index == selected {
                    theme.surface_selected
                } else {
                    theme.sidebar_background
                };
                let action_item = item.clone();
                let mut row = div()
                    .id(SharedString::from(format!("palette-{index}")))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .bg(bg)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.surface_hover))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.command_palette.close();
                            this.apply_palette_action(action_item.clone(), window, cx);
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(icon(item.icon, theme.text_primary, 14.0))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text_primary)
                                    .child(item.label.clone()),
                            ),
                    );
                if let Some(keys) = item.keys {
                    row = row.child(div().text_xs().text_color(theme.text_muted).child(keys));
                }
                list = list.child(row);
            }
        }

        div()
            .absolute()
            .inset_0()
            .flex()
            .justify_center()
            .pt(px(64.0))
            .bg(gpui::hsla(0.0, 0.0, 0.0, 0.35))
            .occlude()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.close_palette(window, cx)),
            )
            .child(ronin::companion::grain_overlay(theme.text_muted))
            .child(
                div()
                    .w(px(480.0))
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.sidebar_background)
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .shadow(
                        ronin::visual_polish::elevation_style(
                            ronin::visual_polish::Elevation::High,
                            theme.color_scheme,
                        )
                        .box_shadows(),
                    )
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(div().text_xs().text_color(theme.text_muted).child(heading))
                    .child(
                        div()
                            .rounded_md()
                            .border_1()
                            .border_color(theme.accent)
                            .px_2()
                            .py_1()
                            .track_focus(&self.palette_focus)
                            .on_key_down(cx.listener(Self::on_palette_key_down))
                            .child(self.palette_editor.render_text_with_selection(
                                "Type to filter…",
                                theme.text_primary,
                                theme.text_muted,
                                theme.accent,
                                theme.surface_selected,
                            )),
                    )
                    .child(list),
            )
    }

    pub(crate) fn render_settings_overlay(
        &self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let sections = visible_sections(self.settings.search());
        let active = self.settings.section();
        let mut nav = div().w(px(160.0)).flex().flex_col().gap_1();
        for section in sections {
            let selected = section == active;
            nav = nav.child(
                div()
                    .id(SharedString::from(section_label(section)))
                    .rounded_md()
                    .px_3()
                    .py_1()
                    .bg(if selected {
                        theme.surface_selected
                    } else {
                        theme.sidebar_background
                    })
                    .border_1()
                    .border_color(if selected {
                        theme.accent
                    } else {
                        theme.sidebar_background
                    })
                    .text_sm()
                    .text_color(theme.text_primary)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.surface_hover))
                    .occlude()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.settings.set_section(section);
                            cx.notify();
                            cx.stop_propagation();
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.settings.set_section(section);
                            cx.notify();
                            cx.stop_propagation();
                        }),
                    )
                    .child(section_label(section)),
            );
        }

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::hsla(0.0, 0.0, 0.0, 0.35))
            .occlude()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.close_settings(cx)),
            )
            .child(ronin::companion::grain_overlay(theme.text_muted))
            .child(
                div()
                    .w(px(720.0))
                    .h(px(480.0))
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.app_background)
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .shadow(
                        ronin::visual_polish::elevation_style(
                            ronin::visual_polish::Elevation::High,
                            theme.color_scheme,
                        )
                        .box_shadows(),
                    )
                    .occlude()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(
                        div()
                            .w(px(180.0))
                            .h_full()
                            .p_3()
                            .bg(theme.sidebar_background)
                            .border_r_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .font_weight(FontWeight(600.))
                                    .text_color(theme.text_primary)
                                    .child("Settings"),
                            )
                            .child(nav),
                    )
                    .child(
                        div()
                            .id("settings-body")
                            .flex_1()
                            .p_6()
                            .overflow_y_scroll()
                            .child(self.render_settings_section(active, theme, cx)),
                    ),
            )
    }

    fn render_settings_section(
        &self,
        section: SettingsSection,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match section {
            SettingsSection::General => div()
                .flex()
                .flex_col()
                .gap_3()
                .child(self.settings_toggle(
                    memories_toggle_label(),
                    self.memories_enabled,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        let next = !this.memories_enabled;
                        if let Err(e) = this.shell.set_features_memories(next) {
                            tracing::error!(%e, "failed to set memories flag");
                        }
                        this.refresh_ui_prefs();
                        cx.notify();
                    }),
                ))
                .child(self.settings_toggle(
                    artifacts_toggle_label(),
                    self.artifacts_enabled,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        let next = !this.artifacts_enabled;
                        if let Err(e) = this.shell.set_features_artifacts(next) {
                            tracing::error!(%e, "failed to set artifacts flag");
                        }
                        this.refresh_ui_prefs();
                        cx.notify();
                    }),
                ))
                .child(self.settings_toggle(
                    shortcut_hints_toggle_label(),
                    self.show_shortcut_hints,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        let next = !this.show_shortcut_hints;
                        if let Err(e) = this.shell.set_show_shortcut_hints(next) {
                            tracing::error!(%e, "failed to set shortcut hints");
                        }
                        this.refresh_ui_prefs();
                        cx.notify();
                    }),
                ))
                .child(self.settings_toggle(
                    notifications_toggle_label(),
                    self.notifications_enabled,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        let next = !this.notifications_enabled;
                        this.persist_config_flag(|config| config.notifications.enabled = next, cx);
                    }),
                ))
                .child(self.settings_toggle(
                    auto_title_toggle_label(),
                    self.auto_title,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        let next = !this.auto_title;
                        this.persist_config_flag(|config| config.general.auto_title = next, cx);
                    }),
                )),
            SettingsSection::Appearance => div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().child(theme_label()))
                        .child(
                            div()
                                .id("theme-cycle")
                                .rounded_md()
                                .px_3()
                                .py_1()
                                .bg(theme.surface_muted)
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.surface_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| this.cycle_theme(cx)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .when(
                                            self.theme_preference == ThemePreference::Light,
                                            |row| {
                                                row.child(icon(
                                                    IconName::Sun,
                                                    theme.text_primary,
                                                    14.0,
                                                ))
                                            },
                                        )
                                        .when(
                                            self.theme_preference == ThemePreference::Dark,
                                            |row| {
                                                row.child(icon(
                                                    IconName::Moon,
                                                    theme.text_primary,
                                                    14.0,
                                                ))
                                            },
                                        )
                                        .child(theme_preference_label(self.theme_preference)),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().child(scale_label()))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_2()
                                .child(
                                    div()
                                        .id("zoom-out")
                                        .rounded_md()
                                        .px_2()
                                        .py_1()
                                        .bg(theme.surface_muted)
                                        .cursor_pointer()
                                        .child(icon(IconName::ZoomOut, theme.text_primary, 14.0))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.apply_zoom_delta(-0.1, cx)
                                            }),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.text_muted)
                                        .child(format!("{:.0}%", self.ui_scale * 100.0)),
                                )
                                .child(
                                    div()
                                        .id("zoom-in")
                                        .rounded_md()
                                        .px_2()
                                        .py_1()
                                        .bg(theme.surface_muted)
                                        .cursor_pointer()
                                        .child(icon(IconName::ZoomIn, theme.text_primary, 14.0))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.apply_zoom_delta(0.1, cx)
                                            }),
                                        ),
                                ),
                        ),
                ),
            SettingsSection::Models => div()
                .flex()
                .flex_col()
                .gap_3()
                .child(self.render_provider_status_line(theme))
                .child(
                    div()
                        .id("settings-test-connection")
                        .text_sm()
                        .text_color(theme.accent)
                        .cursor_pointer()
                        .child(ronin::provider_settings::test_connection_button_label())
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                match this.shell.test_connection() {
                                    Ok(_) => {}
                                    Err(e) => {
                                        tracing::error!("provider connection test failed: {e}")
                                    }
                                }
                                cx.notify();
                            }),
                        ),
                )
                .child(
                    div()
                        .id("settings-model-picker")
                        .rounded_md()
                        .px_3()
                        .py_2()
                        .bg(theme.surface_muted)
                        .cursor_pointer()
                        .child("Choose model")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.open_model_picker();
                                cx.notify();
                            }),
                        ),
                ),
            SettingsSection::Keyboard => {
                let mut list = div().flex().flex_col().gap_2();
                for hint in ronin::keyboard_nav::shortcut_catalog() {
                    list = list.child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(div().text_sm().child(hint.action))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(hint.keys),
                            ),
                    );
                }
                list
            }
            SettingsSection::Privacy => div().text_sm().text_color(theme.text_muted).child(
                "Folder listing still uses the local-knowledge allow/deny lists in config.toml.",
            ),
            SettingsSection::Advanced => div()
                .text_sm()
                .text_color(theme.text_muted)
                .child("Clipboard watch and logging stay in config.toml for this release."),
        }
    }

    fn settings_toggle(
        &self,
        label: &'static str,
        on: bool,
        theme: &M0Theme,
        listener: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .id(SharedString::from(label))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(div().text_sm().child(label))
            .child(
                div()
                    .rounded_md()
                    .px_3()
                    .py_1()
                    .bg(if on {
                        theme.accent
                    } else {
                        theme.surface_muted
                    })
                    .text_color(if on {
                        theme.accent_text
                    } else {
                        theme.text_primary
                    })
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, listener)
                    .child(if on { "On" } else { "Off" }),
            )
    }

    fn render_provider_status_line(&self, theme: &M0Theme) -> gpui::Div {
        let text = match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } => format!("Ollama · {model}"),
            ProviderStatus::OpenAiReady { model } => format!("OpenAI · {model}"),
            ProviderStatus::OllamaOffline => "Ollama offline".into(),
            ProviderStatus::OllamaNoModels => "Ollama online, no models".into(),
            ProviderStatus::OpenAiNotConfigured => "OpenAI not configured".into(),
            ProviderStatus::OpenAiError { message } => message.clone(),
            ProviderStatus::NotConfigured => "No provider configured".into(),
        };
        div().text_sm().text_color(theme.text_primary).child(text)
    }

    pub(crate) fn render_shortcut_coach(
        &self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Div> {
        if !should_show_coach(self.coach_seen, self.show_shortcut_hints) {
            return None;
        }
        let mut steps = div().flex().flex_col().gap_2();
        for step in first_run_steps() {
            steps = steps.child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight(600.))
                            .child(step.title),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(step.body),
                    ),
            );
        }
        Some(
            div()
                .absolute()
                .bottom(px(96.0))
                .right(px(24.0))
                .w(px(280.0))
                .rounded_xl()
                .border_1()
                .border_color(theme.border_subtle)
                .bg(theme.sidebar_background)
                .p_4()
                .flex()
                .flex_col()
                .gap_3()
                .shadow(
                    ronin::visual_polish::elevation_style(
                        ronin::visual_polish::Elevation::High,
                        theme.color_scheme,
                    )
                    .box_shadows(),
                )
                .occlude()
                .child(div().font_weight(FontWeight(600.)).child("A few shortcuts"))
                .child(steps)
                .child(
                    div()
                        .id("dismiss-coach")
                        .rounded_md()
                        .px_3()
                        .py_1()
                        .bg(theme.accent)
                        .text_color(theme.accent_text)
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| this.dismiss_coach(cx)),
                        )
                        .child("Got it"),
                ),
        )
    }

    pub(crate) fn compact_provider_footer(
        &self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.render_provider_status_line(theme)
            .id("sidebar-provider")
            .text_xs()
            .text_color(theme.text_muted)
            .cursor_pointer()
            .hover(|style| style.text_color(theme.accent))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.open_model_picker();
                    cx.notify();
                }),
            )
    }
}
