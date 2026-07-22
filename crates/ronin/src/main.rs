use std::process::ExitCode;

use gpui::{
    div, hsla, img, point, prelude::*, px, size, App, Application, Bounds, ClipboardEntry, Context,
    DragMoveEvent, ExternalPaths, FocusHandle, FontWeight, KeyDownEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollHandle, SharedString, Subscription,
    TitlebarOptions, Window, WindowBounds, WindowHandle, WindowOptions,
};
use ronin::{
    acquire_instance,
    artifacts_panel::{
        artifact_kind_badge, artifact_preview_card, artifacts_empty_state,
        save_code_block_as_snippet_label, snippet_title_from_language, ArtifactsPanelState,
        ARTIFACT_KIND_BADGE,
    },
    attachment_preview::{preview_from_attachment, preview_from_draft, AttachmentPreview},
    attachment_size::{AttachmentSizeWarnState, DEFAULT_ATTACHMENT_WARN_CHARS},
    completions,
    composer::ComposerEditor,
    composer_ingest::{
        drop_overlay_should_show, ingest_dropped_paths, paste_image_bytes, paste_rgba_image,
    },
    composer_pickers::{
        detect_active_picker, move_picker_selection, ActivePicker, AtAttachmentKind, PickerItem,
        PickerKind, SlashActionKind,
    },
    context_indicator::{
        fill_level_color, project_context_indicator, ContextEstimateInput, ContextIndicator,
    },
    folder_attach::FolderAttachState,
    global_search::{
        artifact_document, group_hits_by_kind, memory_document, search, thread_message_document,
        thread_title_document, SearchContentKind, SearchDatePreset, SearchDocument, SearchHit,
        SearchPanelState,
    },
    instance_runtime_dir,
    keyboard_nav::{
        shortcut_catalog, FocusRegion, KeyInput, KeyboardNavState, NavAction, ScrollDirection,
    },
    memory_management::{
        group_memory_cards, memory_context_indicator, MemoryListItem, MemoryManagementState,
        PROFILE_GROUP_LABEL,
    },
    message_branches::{branch_nav_label, edit_draft_commit, MessageEditState},
    model_picker::{
        entries_from_listed_providers, format_capability_summary, group_entries_by_provider,
        open_picker_at_active, picker_row_colors, picker_row_tone, refresh_picker_entries,
        ModelPickerAction, ModelPickerEntry, ModelPickerKey, ModelPickerState, ModelProviderKind,
    },
    parse_launch_intent, plan_incoming_launch,
    provider_settings::{
        connection_test_is_success, format_connection_test_result, test_connection_button_label,
    },
    ronin_paths,
    screenshot_capture::PortalOrFallbackScreenshotCapturer,
    theme::{resolve_shell_theme, M0Theme},
    thread_titles::{
        format_sidebar_thread_title, title_generation_status_label, ThreadRenameState,
    },
    visual_polish::{
        cursor_visible_at, elevation_style, empty_state, error_presentation, generating_label,
        streaming_motion, Elevation, EmptyStateContent, EmptyStateKind, ErrorKind,
        ErrorPresentation,
    },
    InstancePrimary, LaunchIntent, LauncherError,
};
use ronin_app::{
    format_provider_error, ProviderStatus, RoninAppError, RoninShell, ShellState, MAX_CHARS,
    MAX_MESSAGES,
};
use ronin_core::{
    clamp_sidebar_width, clipboard_attachment, list_folder_entries, parse_context_tools,
    read_file_attachment, screenshot_attachment, ChatProvider, ContextAttachmentDraft,
    ContextToolRef, HttpOllamaProvider, MessageRole, MessageStatus, ScreenshotCapturer,
    ThemePreference,
};

mod quick_overlay;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let detail = error.to_string();
            if detail.to_lowercase().contains("migration") {
                let err = error_presentation(ErrorKind::MigrationFailure, &detail);
                eprintln!("{}", err.display_text());
            } else {
                eprintln!("ronin: {error}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), RunError> {
    let intent = parse_launch_intent(std::env::args().skip(1))?;
    let runtime_dir = instance_runtime_dir()?;
    let instance = acquire_instance(&runtime_dir, &intent)?;
    let Some(instance_primary) = instance.into_primary() else {
        tracing::info!("ronin hand-off complete; secondary process exiting");
        return Ok(());
    };

    let paths = ronin_paths()?;
    let attach_paths = match &intent {
        LaunchIntent::OpenPersisted { attach_paths }
        | LaunchIntent::NewThread { attach_paths }
        | LaunchIntent::OpenWithOllama { attach_paths }
        | LaunchIntent::Quick { attach_paths } => attach_paths.clone(),
    };
    let is_quick = matches!(intent, LaunchIntent::Quick { .. });
    let mut shell = match intent {
        LaunchIntent::OpenPersisted { .. } if !attach_paths.is_empty() => {
            RoninShell::open_with_new_thread(paths)?
        }
        LaunchIntent::OpenPersisted { .. } => RoninShell::open(paths)?,
        LaunchIntent::NewThread { .. } => RoninShell::open_with_new_thread(paths)?,
        LaunchIntent::Quick { .. } => RoninShell::open(paths)?,
        LaunchIntent::OpenWithOllama { .. } => RoninShell::open_with_ollama(paths)?,
    };
    let _ = shell.refresh_provider_status();
    tracing::info!(intent = ?intent, "ronin launch intent parsed");

    Application::new().run(move |cx: &mut App| {
        if is_quick {
            match quick_overlay::open_quick_overlay_window(
                cx,
                shell,
                Some(instance_primary),
                None,
                None,
            ) {
                Ok(_) => {
                    tracing::info!("ronin quick overlay opened");
                    cx.activate(true);
                }
                Err(error) => {
                    tracing::error!(%error, "failed to open ronin quick overlay");
                    cx.quit();
                }
            }
            return;
        }

        match open_main_window(cx, shell, Some(instance_primary), attach_paths, None) {
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

/// Opens the full Ronin shell window, optionally selecting `select_thread_id`.
pub(crate) fn open_main_window(
    cx: &mut App,
    mut shell: RoninShell,
    instance_primary: Option<InstancePrimary>,
    attach_paths: Vec<std::path::PathBuf>,
    select_thread_id: Option<String>,
) -> Result<WindowHandle<RoninWindow>, String> {
    let _ = shell.reload_threads();
    if let Some(thread_id) = select_thread_id.as_deref() {
        if let Err(e) = shell.select_thread(thread_id) {
            tracing::warn!(%e, %thread_id, "could not select thread when opening main window");
        }
    }
    let _ = shell.refresh_provider_status();

    let bounds = Bounds::centered(None, size(px(1120.0), px(760.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Ronin")),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            cx.new(|cx| {
                let rem = 14.0; // default rem in pixels for GPUI 0.2.x
                let mut composer = ComposerEditor::new();
                composer.set_font_metrics_from_rem(rem);
                let _appearance_subscription =
                    cx.observe_window_appearance(window, |_this, _window, cx| {
                        cx.notify();
                    });
                let sidebar_width = shell.sidebar_width();
                let sidebar_collapsed = shell.sidebar_collapsed();
                RoninWindow {
                    shell,
                    composer,
                    instance_primary,
                    preattached_files: attach_paths.clone(),
                    attachment_errors: Vec::new(),
                    composer_focus: cx.focus_handle(),
                    sidebar_focus: cx.focus_handle(),
                    messages_focus: cx.focus_handle(),
                    chat_provider: None,
                    needs_initial_focus: true,
                    copied_state: None,
                    memories_panel_open: false,
                    memory_management: MemoryManagementState::default(),
                    artifacts_panel_open: false,
                    artifacts_panel: ArtifactsPanelState::default(),
                    artifact_title_editor: None,
                    artifact_content_editor: None,
                    artifact_title_focus: cx.focus_handle(),
                    artifact_content_focus: cx.focus_handle(),
                    pending_attachments: Vec::new(),
                    pending_folder_attaches: Vec::new(),
                    attachment_size_warn: AttachmentSizeWarnState::default(),
                    screenshot_capturer: Box::new(PortalOrFallbackScreenshotCapturer),
                    parsed_messages: std::collections::HashMap::new(),
                    completion_index: 0,
                    picker_suppressed: None,
                    pending_clipboard_read: None,
                    composer_rem: rem,
                    composer_scroll_handle: ScrollHandle::new(),
                    message_scroll_handle: ScrollHandle::new(),
                    blink_start: std::time::Instant::now(),
                    sidebar_width,
                    sidebar_collapsed,
                    sidebar_drag: None,
                    keyboard_nav: KeyboardNavState::new(),
                    file_drop_active: false,
                    thread_rename: ThreadRenameState::default(),
                    thread_rename_editor: None,
                    thread_rename_focus: cx.focus_handle(),
                    message_edit: MessageEditState::default(),
                    message_edit_editor: None,
                    message_edit_focus: cx.focus_handle(),
                    search_panel: SearchPanelState::default(),
                    search_query_editor: ComposerEditor::new(),
                    search_focus: cx.focus_handle(),
                    pending_scroll_message_id: None,
                    model_picker: ModelPickerState::default(),
                    model_picker_entries: Vec::new(),
                    quick_overlay: None,
                    _appearance_subscription,
                }
            })
        },
    )
    .map_err(|e| format!("failed to open main window: {e}"))
}

#[derive(Debug, thiserror::Error)]
enum RunError {
    #[error(transparent)]
    Launcher(#[from] LauncherError),
    #[error(transparent)]
    App(#[from] RoninAppError),
    #[error(transparent)]
    Instance(#[from] ronin::InstanceError),
}

struct RoninWindow {
    shell: RoninShell,
    composer: ComposerEditor,
    /// Single-instance IPC listener (None only in tests / if hand-off skipped).
    instance_primary: Option<InstancePrimary>,
    preattached_files: Vec<std::path::PathBuf>,
    attachment_errors: Vec<String>,
    composer_focus: FocusHandle,
    sidebar_focus: FocusHandle,
    messages_focus: FocusHandle,
    chat_provider: Option<Box<dyn ChatProvider + Send>>,
    needs_initial_focus: bool,
    copied_state: Option<(String, std::time::Instant)>,
    memories_panel_open: bool,
    memory_management: MemoryManagementState,
    artifacts_panel_open: bool,
    artifacts_panel: ArtifactsPanelState,
    artifact_title_editor: Option<ComposerEditor>,
    artifact_content_editor: Option<ComposerEditor>,
    artifact_title_focus: FocusHandle,
    artifact_content_focus: FocusHandle,
    pending_attachments: Vec<ContextAttachmentDraft>,
    pending_folder_attaches: Vec<FolderAttachState>,
    attachment_size_warn: AttachmentSizeWarnState,
    screenshot_capturer: Box<dyn ScreenshotCapturer + Send>,
    parsed_messages:
        std::collections::HashMap<String, (usize, Vec<ronin::markdown::MarkdownBlock>)>,
    completion_index: usize,
    /// When set, hides the `@`/`/` picker until the trigger token changes.
    picker_suppressed: Option<String>,
    pending_clipboard_read: Option<std::sync::mpsc::Receiver<Result<String, arboard::Error>>>,
    composer_rem: f32,
    composer_scroll_handle: ScrollHandle,
    message_scroll_handle: ScrollHandle,
    blink_start: std::time::Instant,
    sidebar_width: f32,
    sidebar_collapsed: bool,
    sidebar_drag: Option<SidebarDrag>,
    keyboard_nav: KeyboardNavState,
    file_drop_active: bool,
    thread_rename: ThreadRenameState,
    thread_rename_editor: Option<ComposerEditor>,
    thread_rename_focus: FocusHandle,
    message_edit: MessageEditState,
    message_edit_editor: Option<ComposerEditor>,
    message_edit_focus: FocusHandle,
    search_panel: SearchPanelState,
    search_query_editor: ComposerEditor,
    search_focus: FocusHandle,
    pending_scroll_message_id: Option<String>,
    model_picker: ModelPickerState,
    model_picker_entries: Vec<ModelPickerEntry>,
    /// Open compact quick-mode overlay, if any.
    quick_overlay: Option<WindowHandle<quick_overlay::QuickModeWindow>>,
    _appearance_subscription: Subscription,
}

#[derive(Debug, Clone, Copy)]
struct SidebarDrag {
    start_x: f32,
    start_width: f32,
}

/// Narrow rail width when the sidebar is collapsed (expand affordance).
const SIDEBAR_COLLAPSED_RAIL: f32 = 40.0;

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

    fn paste_dest_dir(&self) -> std::path::PathBuf {
        self.shell.session().paths().data_dir.join("pasted-images")
    }

    fn apply_dropped_paths(&mut self, paths: &[std::path::PathBuf], cx: &mut Context<Self>) {
        self.file_drop_active = false;
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let result = ingest_dropped_paths(paths, &cwd);
        self.attachment_errors.clear();
        self.pending_attachments.extend(result.drafts);
        self.pending_folder_attaches.extend(result.folders);
        self.attachment_errors.extend(result.errors);
        self.attachment_size_warn.clear();
        cx.notify();
    }

    fn render_drop_overlay(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let overlay_bg = hsla(
            theme.app_background.h,
            theme.app_background.s,
            theme.app_background.l,
            0.82,
        );
        div()
            .id("file-drop-overlay")
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .right(px(0.))
            .bottom(px(0.))
            .flex()
            .items_center()
            .justify_center()
            .bg(overlay_bg)
            .border_2()
            .border_color(theme.accent)
            .can_drop(|value, _, _| value.is::<ExternalPaths>())
            .on_drop(cx.listener(Self::on_external_paths_drop))
            .child(
                div()
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.accent)
                    .bg(theme.surface_muted)
                    .px_6()
                    .py_4()
                    .text_lg()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.accent)
                    .child("Drop files to attach"),
            )
    }

    fn try_paste_clipboard_image(&mut self, cx: &mut Context<Self>) -> bool {
        let dest = self.paste_dest_dir();

        if let Some(item) = cx.read_from_clipboard() {
            for entry in item.entries() {
                if let ClipboardEntry::Image(image) = entry {
                    match paste_image_bytes(&image.bytes, image.format.mime_type(), &dest) {
                        Ok(draft) => {
                            self.attachment_errors.clear();
                            self.pending_attachments.push(draft);
                            cx.notify();
                            return true;
                        }
                        Err(e) => {
                            self.attachment_errors.clear();
                            self.attachment_errors.push(e);
                            cx.notify();
                            return true;
                        }
                    }
                }
            }
        }

        match arboard::Clipboard::new().and_then(|mut c| c.get_image()) {
            Ok(img) => match paste_rgba_image(img.width, img.height, &img.bytes, &dest) {
                Ok(draft) => {
                    self.attachment_errors.clear();
                    self.pending_attachments.push(draft);
                    cx.notify();
                    true
                }
                Err(e) => {
                    self.attachment_errors.clear();
                    self.attachment_errors.push(e);
                    cx.notify();
                    true
                }
            },
            Err(_) => false,
        }
    }

    fn on_external_paths_drag_move(
        &mut self,
        _event: &DragMoveEvent<ExternalPaths>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.file_drop_active {
            self.file_drop_active = true;
            cx.notify();
        }
    }

    fn on_external_paths_drop(
        &mut self,
        paths: &ExternalPaths,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.apply_dropped_paths(paths.paths(), cx);
    }

    fn active_model_name(&self) -> Option<&str> {
        match &self.shell.state().provider_status {
            ProviderStatus::OllamaOnline { model } | ProviderStatus::OpenAiReady { model } => {
                Some(model.as_str())
            }
            _ => None,
        }
    }

    fn pending_attachment_chars(&self) -> usize {
        let mut chars = self
            .pending_attachments
            .iter()
            .map(|a| a.context_block.chars().count())
            .sum::<usize>();
        for folder in &self.pending_folder_attaches {
            if let Ok(draft) = folder.to_context_draft() {
                chars += draft.context_block.chars().count();
            }
        }
        for path in &self.preattached_files {
            chars += path.to_string_lossy().chars().count() + 32;
        }
        if let Some(block) =
            ronin::memory_management::memory_context_block(&self.memory_list_items())
        {
            chars += block.chars().count();
        }
        chars
    }

    fn materialize_folder_attachments(&mut self) -> Vec<ContextAttachmentDraft> {
        let mut out = Vec::new();
        let folders = std::mem::take(&mut self.pending_folder_attaches);
        for folder in folders {
            match folder.to_context_draft() {
                Ok(draft) => out.push(draft),
                Err(e) => self.attachment_errors.push(e.to_string()),
            }
        }
        out
    }

    fn attachment_warn_threshold(&self) -> usize {
        self.shell
            .session()
            .load_config()
            .map(|c| c.general.attachment_warn_chars)
            .unwrap_or(DEFAULT_ATTACHMENT_WARN_CHARS)
    }

    fn current_context_indicator(&self) -> ContextIndicator {
        let message_contents: Vec<String> = self
            .shell
            .state()
            .messages
            .as_ref()
            .map(|msgs| {
                msgs.iter()
                    .filter(|m| m.status != MessageStatus::Streaming)
                    .map(|m| m.content.clone())
                    .collect()
            })
            .unwrap_or_default();

        project_context_indicator(ContextEstimateInput {
            message_contents: &message_contents,
            composer_text: self.composer.text(),
            attachment_chars: self.pending_attachment_chars(),
            system_prompt_chars: self.shell.effective_system_prompt().chars().count(),
            model_name: self.active_model_name(),
            max_messages: MAX_MESSAGES,
            max_chars: MAX_CHARS,
        })
    }

    fn render_context_indicator(&self, theme: &M0Theme) -> impl IntoElement {
        let indicator = self.current_context_indicator();
        let color = fill_level_color(indicator.level, theme.color_scheme);
        let bar_w = (indicator.fill_ratio.clamp(0.0, 1.0) * 72.0).max(2.0);

        let mut row = div()
            .id("context-indicator")
            .flex()
            .flex_row()
            .items_center()
            .justify_end()
            .gap_2()
            .child(
                div()
                    .w(px(72.0))
                    .h(px(4.0))
                    .rounded_full()
                    .bg(theme.border_subtle)
                    .overflow_hidden()
                    .child(div().h_full().w(px(bar_w)).rounded_full().bg(color)),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color)
                    .child(indicator.summary_label.clone()),
            );

        if let Some(omission) = indicator.omission_label {
            row = row.child(div().text_xs().text_color(theme.text_muted).child(omission));
        }

        row
    }

    fn memory_list_items(&self) -> Vec<MemoryListItem> {
        self.shell
            .list_memories()
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                MemoryListItem::from_fields(
                    m.id.0,
                    m.title,
                    m.content,
                    m.enabled,
                    m.is_profile,
                    m.created_at,
                )
            })
            .collect()
    }

    fn render_memory_context_indicator(&self, theme: &M0Theme) -> Option<impl IntoElement> {
        let items = self.memory_list_items();
        let indicator = memory_context_indicator(&items)?;
        Some(
            div()
                .id("memory-context-indicator")
                .flex()
                .flex_row()
                .items_center()
                .justify_end()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight(500.))
                        .text_color(theme.accent)
                        .child(indicator.summary_label),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(indicator.detail_label),
                ),
        )
    }

    // ── context / attachments ──

    fn resolve_context_attachments(&mut self, text: &str) -> (String, Vec<ContextAttachmentDraft>) {
        self.attachment_errors.clear();
        let parsed = parse_context_tools(text);
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let mut attachments = self.pending_attachments.clone();
        attachments.extend(self.materialize_folder_attachments());

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
                ContextToolRef::Folder(path) => match list_folder_entries(&path, &cwd) {
                    Ok(listing) => {
                        let state = FolderAttachState::from_listing(listing);
                        match state.to_context_draft() {
                            Ok(a) => attachments.push(a),
                            Err(e) => self.attachment_errors.push(e.to_string()),
                        }
                    }
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
                            if m.enabled {
                                attachments.push(ronin_core::memory_attachment(&m));
                            } else {
                                self.attachment_errors
                                    .push(format!("Memory is disabled: {id}"));
                            }
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
                ContextToolRef::Screenshot => match self.capture_screenshot_attachment() {
                    Ok(draft) => attachments.push(draft),
                    Err(e) => self.attachment_errors.push(e),
                },
            }
        }

        (parsed.visible_message, attachments)
    }

    fn capture_screenshot_attachment(&self) -> Result<ContextAttachmentDraft, String> {
        self.capture_screenshot_attachment_with_preference(
            ronin_core::ScreenshotTargetPreference::Interactive,
        )
    }

    fn capture_screenshot_attachment_with_preference(
        &self,
        preference: ronin_core::ScreenshotTargetPreference,
    ) -> Result<ContextAttachmentDraft, String> {
        let dest_dir = self.shell.session().paths().data_dir.join("screenshots");
        let path = self
            .screenshot_capturer
            .capture_with_preference(&dest_dir, preference)
            .map_err(|e| e.to_string())?;
        screenshot_attachment(&path).map_err(|e| e.to_string())
    }

    fn take_screenshot_action(&mut self, cx: &mut Context<Self>) {
        self.take_screenshot_action_with_preference(
            ronin_core::ScreenshotTargetPreference::Interactive,
            cx,
        );
    }

    fn take_window_screenshot_action(&mut self, cx: &mut Context<Self>) {
        self.take_screenshot_action_with_preference(
            ronin_core::ScreenshotTargetPreference::Window,
            cx,
        );
    }

    fn take_screenshot_action_with_preference(
        &mut self,
        preference: ronin_core::ScreenshotTargetPreference,
        cx: &mut Context<Self>,
    ) {
        match self.capture_screenshot_attachment_with_preference(preference) {
            Ok(draft) => {
                self.pending_attachments.push(draft);
                cx.notify();
            }
            Err(e) => {
                self.attachment_errors.push(e);
                cx.notify();
            }
        }
    }

    fn composer_attachment_previews(&self) -> Vec<AttachmentPreview> {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let mut drafts = self.pending_attachments.clone();
        for path in &self.preattached_files {
            if let Ok(draft) = read_file_attachment(path, &cwd) {
                drafts.push(draft);
            }
        }
        let parsed = parse_context_tools(self.composer.text());
        for tool_ref in parsed.refs {
            match tool_ref {
                ContextToolRef::File(path) => {
                    if let Ok(draft) = read_file_attachment(&path, &cwd) {
                        drafts.push(draft);
                    }
                }
                ContextToolRef::Folder(path) => {
                    if let Ok(listing) = list_folder_entries(&path, &cwd) {
                        let state = FolderAttachState::from_listing(listing);
                        if let Ok(draft) = state.to_context_draft() {
                            drafts.push(draft);
                        }
                    }
                }
                ContextToolRef::Screenshot => {
                    drafts.push(ContextAttachmentDraft {
                        kind: ronin_core::AttachmentKind::Screenshot,
                        name: "screenshot (captures on send)".into(),
                        mime_type: "image/png".into(),
                        content: None,
                        path: None,
                        context_block: "[Screenshot]".into(),
                        size_bytes: None,
                    });
                }
                ContextToolRef::Clipboard => {
                    if let Ok(text) = arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                        drafts.push(clipboard_attachment(&text));
                    }
                }
                ContextToolRef::Memory(_) | ContextToolRef::Artifact(_) => {}
            }
        }
        drafts.iter().map(preview_from_draft).collect()
    }

    fn render_composer_attachment_preview(
        &self,
        preview: &AttachmentPreview,
        theme: &M0Theme,
    ) -> gpui::Div {
        match preview {
            AttachmentPreview::Image {
                name,
                path,
                mime_type,
                size_bytes,
                ..
            } => {
                let mut card = div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .p_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .w(px(140.0));
                if path.exists() {
                    card = card.child(
                        img(path.clone())
                            .h(px(72.0))
                            .w_full()
                            .object_fit(gpui::ObjectFit::Cover),
                    );
                }
                card.child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(name.clone()),
                )
                .child(div().text_xs().text_color(theme.text_muted).child(format!(
                        "{mime_type}{}",
                        size_bytes
                            .map(|b| format!(
                                " · {}",
                                ronin::attachment_preview::format_size_bytes(b)
                            ))
                            .unwrap_or_default()
                    )))
            }
            AttachmentPreview::File {
                name,
                mime_type,
                size_label,
                snippet,
                ..
            } => {
                let mut card = div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .p_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .max_w(px(220.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.text_primary)
                            .child(name.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.accent)
                            .child(format!("{mime_type} · {size_label}")),
                    );
                if let Some(snippet) = snippet {
                    card = card.child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(snippet.clone()),
                    );
                }
                card
            }
            AttachmentPreview::Text { name, snippet, .. } => {
                let mut card = div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .p_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(name.clone()),
                    );
                if let Some(snippet) = snippet {
                    card = card.child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(snippet.clone()),
                    );
                }
                card
            }
        }
    }

    fn render_message_attachments(&self, message_id: &str, theme: &M0Theme) -> Option<gpui::Div> {
        let attachments = self.shell.session().list_attachments(message_id).ok()?;
        if attachments.is_empty() {
            return None;
        }
        let mut row = div().flex().flex_row().flex_wrap().gap_2().mt_2();
        for attachment in &attachments {
            let preview = preview_from_attachment(attachment);
            row = row.child(self.render_composer_attachment_preview(&preview, theme));
        }
        Some(row)
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

        let threshold = self.attachment_warn_threshold();
        self.attachment_size_warn.evaluate(&attachments, threshold);
        if self.attachment_size_warn.should_block_send() {
            // Restore composer so the user can trim attachments or proceed.
            self.composer.set_text(text);
            // Put folder drafts back into pending if we already materialized them
            // via resolve — they were moved into `attachments`. Keep as pending
            // file drafts for simplicity.
            for draft in &attachments {
                if matches!(draft.kind, ronin_core::AttachmentKind::Folder) {
                    // Already included in attachments list; leave on pending for retry.
                    if !self
                        .pending_attachments
                        .iter()
                        .any(|d| d.context_block == draft.context_block)
                    {
                        self.pending_attachments.push(draft.clone());
                    }
                }
            }
            cx.notify();
            return;
        }
        self.attachment_size_warn.clear();

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
                    self.pending_attachments.clear();
                    self.pending_folder_attaches.clear();
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
                        self.pending_attachments.clear();
                        self.pending_folder_attaches.clear();
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
                    self.pending_attachments.clear();
                    self.pending_folder_attaches.clear();
                }
                cx.notify();
            }
        }
    }

    fn remove_preattached_file(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.preattached_files.len() {
            self.preattached_files.remove(index);
        }
        cx.notify();
    }

    // ── completions ──

    fn picker_suppress_key(picker: &ActivePicker) -> String {
        format!("{:?}:{}:{}", picker.kind, picker.token_start, picker.query)
    }

    fn active_picker(&self) -> Option<ActivePicker> {
        let picker = detect_active_picker(self.composer.text(), self.composer.cursor())?;
        let key = Self::picker_suppress_key(&picker);
        if self.picker_suppressed.as_deref() == Some(key.as_str()) {
            return None;
        }
        Some(picker)
    }

    fn clear_stale_picker_suppress(&mut self) {
        let Some(suppressed) = self.picker_suppressed.as_ref() else {
            return;
        };
        match detect_active_picker(self.composer.text(), self.composer.cursor()) {
            Some(picker) if Self::picker_suppress_key(&picker) == *suppressed => {}
            _ => self.picker_suppressed = None,
        }
    }

    fn dismiss_active_picker(&mut self) -> bool {
        let Some(picker) = detect_active_picker(self.composer.text(), self.composer.cursor())
        else {
            return false;
        };
        self.picker_suppressed = Some(Self::picker_suppress_key(&picker));
        self.completion_index = 0;
        true
    }

    fn command_completion(&self) -> Option<completions::CommandCompletion> {
        completions::command_completion(self.composer.text(), self.composer.cursor())
    }

    fn memory_completions(&self) -> Vec<(String, String)> {
        let prefix = match completions::memory_completion_prefix(
            self.composer.text(),
            self.composer.cursor(),
        ) {
            Some(p) => p.to_string(),
            None => return Vec::new(),
        };
        let memories = self.shell.list_memories().unwrap_or_default();
        completions::filter_memory_completions(
            &prefix,
            memories
                .into_iter()
                .filter(|m| m.enabled)
                .map(|m| (m.id.0, m.title)),
        )
    }

    fn artifact_completions(&self) -> Vec<(String, String)> {
        let prefix = match completions::artifact_completion_prefix(
            self.composer.text(),
            self.composer.cursor(),
        ) {
            Some(p) => p.to_string(),
            None => return Vec::new(),
        };
        let artifacts = self.shell.list_all_artifacts().unwrap_or_default();
        completions::filter_artifact_completions(
            &prefix,
            artifacts.into_iter().map(|a| (a.id.0, a.title)),
        )
    }

    fn file_path_completions(&self) -> Vec<String> {
        completions::file_path_completions(self.composer.text(), self.composer.cursor())
    }

    fn accept_command_completion(&mut self) -> bool {
        if let Some(picker) = detect_active_picker(self.composer.text(), self.composer.cursor()) {
            if self.picker_suppressed.as_ref() != Some(&Self::picker_suppress_key(&picker))
                && !picker.items.is_empty()
            {
                let idx = self.completion_index.min(picker.items.len() - 1);
                let item = picker.items[idx];
                return self.accept_picker_item(&picker, item);
            }
        }

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

        let art_completions = self.artifact_completions();
        if !art_completions.is_empty() {
            let idx = self.completion_index.min(art_completions.len() - 1);
            let (chosen_id, _) = &art_completions[idx];
            let cursor = self.composer.cursor();
            let text = self.composer.text();
            let ts = text[..cursor]
                .rfind(char::is_whitespace)
                .map(|i| i + 1)
                .unwrap_or(0);

            let replacement = format!("@artifact:{chosen_id} ");
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

        let dir_str = completions::completion_dir_prefix(base);
        let repl = format!("{head}{dir_str}{chosen}");
        self.composer.replace_range(ts, cursor, &repl);
        self.completion_index = 0;
        true
    }

    fn accept_picker_item(&mut self, picker: &ActivePicker, item: PickerItem) -> bool {
        let cursor = self.composer.cursor();
        match picker.kind {
            PickerKind::AtAttachment => {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
                match item.at_kind() {
                    Some(AtAttachmentKind::Screenshot | AtAttachmentKind::ScreenshotWindow) => {
                        let preference = match item.at_kind() {
                            Some(AtAttachmentKind::ScreenshotWindow) => {
                                ronin_core::ScreenshotTargetPreference::Window
                            }
                            _ => ronin_core::ScreenshotTargetPreference::Interactive,
                        };
                        self.composer.replace_range(picker.token_start, cursor, "");
                        self.completion_index = 0;
                        self.picker_suppressed = None;
                        match self.capture_screenshot_attachment_with_preference(preference) {
                            Ok(draft) => self.pending_attachments.push(draft),
                            Err(e) => self.attachment_errors.push(e),
                        }
                        true
                    }
                    Some(AtAttachmentKind::Clipboard) => {
                        self.composer.replace_range(picker.token_start, cursor, "");
                        self.completion_index = 0;
                        self.picker_suppressed = None;
                        match arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                            Ok(text) => {
                                self.pending_attachments.push(clipboard_attachment(&text));
                            }
                            Err(e) => self
                                .attachment_errors
                                .push(format!("failed to read clipboard: {e}")),
                        }
                        true
                    }
                    other => {
                        let replacement = match other {
                            Some(AtAttachmentKind::File) => format!("@file:{home}/"),
                            Some(AtAttachmentKind::Folder) => format!("@folder:{home}/"),
                            Some(AtAttachmentKind::Artifact) => "@artifact:".to_string(),
                            Some(AtAttachmentKind::Memory) => "@memory:".to_string(),
                            _ => item.insert.to_string(),
                        };
                        self.composer
                            .replace_range(picker.token_start, cursor, &replacement);
                        self.completion_index = 0;
                        self.picker_suppressed = None;
                        true
                    }
                }
            }
            PickerKind::SlashAction => {
                self.composer.replace_range(picker.token_start, cursor, "");
                self.completion_index = 0;
                self.picker_suppressed = None;
                match item.slash_kind() {
                    Some(SlashActionKind::NewThread) => {
                        let _ = self.shell.create_new_thread();
                    }
                    Some(SlashActionKind::ClearComposer) => {
                        self.composer.set_text(String::new());
                    }
                    Some(SlashActionKind::SwitchModel) => {
                        self.open_model_picker();
                    }
                    None => {}
                }
                true
            }
        }
    }

    // ── keyboard / mouse ──

    fn on_composer_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
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
        let mem_matches = self.memory_completions();
        let has_mem = !mem_matches.is_empty();
        let art_matches = self.artifact_completions();
        let has_art = !art_matches.is_empty();
        let cmd_completion = self.command_completion();
        let has_cmd = cmd_completion.is_some();
        let active_picker = self.active_picker();
        let picker_len = active_picker.as_ref().map(|p| p.items.len()).unwrap_or(0);
        let has_picker = picker_len > 0;
        let list_len = if has_picker {
            picker_len
        } else if has_files {
            file_matches.len()
        } else if has_mem {
            mem_matches.len()
        } else if has_art {
            art_matches.len()
        } else {
            0
        };
        let has_list = list_len > 0;

        // up/down: navigate pickers / completions if visible, else delegate to composer
        match ks.key.as_str() {
            "up" if has_list => {
                self.completion_index = if has_picker {
                    move_picker_selection(self.completion_index, list_len, -1)
                } else {
                    self.completion_index.saturating_sub(1)
                };
                cx.notify();
                return;
            }
            "down" if has_list => {
                self.completion_index = if has_picker {
                    move_picker_selection(self.completion_index, list_len, 1)
                } else {
                    let max = list_len.saturating_sub(1);
                    (self.completion_index + 1).min(max)
                };
                cx.notify();
                return;
            }
            _ => {}
        }

        // Delegate to editor (backspace, delete, arrows, home, end, ctrl+a)
        // But skip up/down when completions visible (already handled above)
        let skip_composer =
            (has_list || has_cmd || has_picker) && matches!(ks.key.as_str(), "up" | "down");
        if !skip_composer && self.composer.on_key_down(event) {
            self.clear_stale_picker_suppress();
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
                    return;
                }
                let input = KeyInput {
                    key: "tab",
                    control: false,
                    shift,
                    alt: false,
                };
                let (consumed, action) = self.keyboard_nav.handle_key(input, self.thread_count());
                if consumed {
                    self.apply_nav_action(action, window, cx);
                }
            }
            "escape" => {
                if self.dismiss_active_picker() {
                    cx.notify();
                    return;
                }
                self.cancel_generation(cx);
            }
            "v" if ctrl => {
                if self.try_paste_clipboard_image(cx) {
                    return;
                }
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Click in composer (outside picker rows) dismisses an open picker.
        let _ = self.dismiss_active_picker();
        self.keyboard_nav
            .set_focus(FocusRegion::Composer, self.thread_count());
        window.focus(&self.composer_focus);
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

    fn try_auto_title_after_exchange(&mut self) {
        let Some(thread_id) = self.shell.state().selected_thread_id.clone() else {
            return;
        };
        let Some(model) = self.active_model_name().map(str::to_string) else {
            return;
        };
        let Some(provider) = self
            .chat_provider
            .take()
            .or_else(|| self.resolve_active_chat_provider())
        else {
            return;
        };
        match self
            .shell
            .begin_model_title_generation(&thread_id, &model, provider)
        {
            Ok(None) => tracing::info!(%thread_id, "started model title generation"),
            Ok(Some(provider)) => {
                self.chat_provider = Some(provider);
            }
            Err(e) => tracing::warn!(%e, %thread_id, "failed to start model title generation"),
        }
    }

    fn pump_streaming(&mut self) -> bool {
        let was_generating = self.shell.is_generation_active();
        let active = self.shell.poll_streaming();
        let title_active = self.shell.poll_title_generation();
        if !active && self.chat_provider.is_none() {
            self.chat_provider = self.resolve_active_chat_provider();
        }
        if was_generating && !active {
            self.try_auto_title_after_exchange();
        }
        active || title_active
    }

    /// Polls single-instance IPC and applies routed CLI intents.
    fn pump_instance_ipc(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(primary) = self.instance_primary.as_mut() else {
            return false;
        };
        match primary.try_recv() {
            Ok(Some(incoming)) => {
                let plan = plan_incoming_launch(&incoming);
                tracing::info!(?plan, "applying routed launch intent");
                if plan.open_quick_overlay {
                    self.open_quick_overlay(window, cx);
                } else if plan.create_new_thread {
                    if let Err(e) = self.shell.create_new_thread() {
                        tracing::error!(%e, "failed to create thread from routed intent");
                    }
                }
                if !plan.attach_paths.is_empty() {
                    self.preattached_files.extend(plan.attach_paths);
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
                tracing::warn!(%e, "instance ipc recv failed");
                false
            }
        }
    }

    fn open_quick_overlay(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(handle) = self.quick_overlay {
            let _ = handle.update(cx, |_view, window, cx| {
                window.activate_window();
                cx.activate(true);
            });
            return;
        }
        let paths = self.shell.session().paths().clone();
        let current_thread_id = self.shell.state().selected_thread_id.clone();
        let Ok(shell) = RoninShell::open(paths) else {
            tracing::error!("failed to open shell for quick overlay");
            return;
        };
        let Some(main) = window.window_handle().downcast::<RoninWindow>() else {
            tracing::error!("failed to downcast main window handle for quick overlay");
            return;
        };
        match quick_overlay::open_quick_overlay_window(
            cx,
            shell,
            None,
            Some(main),
            current_thread_id,
        ) {
            Ok(handle) => {
                self.quick_overlay = Some(handle);
                tracing::info!("ronin quick overlay opened from running instance");
            }
            Err(e) => tracing::error!(%e, "failed to open quick overlay from IPC"),
        }
    }

    fn begin_thread_rename(
        &mut self,
        thread_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let title = self
            .shell
            .state()
            .threads
            .iter()
            .find(|t| t.id == thread_id)
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "New Chat".to_string());
        self.thread_rename
            .begin_rename(thread_id.to_string(), title.clone());
        let mut editor = ComposerEditor::new();
        editor.set_font_metrics_from_rem(self.composer_rem);
        editor.set_container_width(self.sidebar_width.max(120.0) - 24.0);
        editor.set_text(title);
        self.thread_rename_editor = Some(editor);
        window.focus(&self.thread_rename_focus);
        cx.notify();
    }

    fn commit_thread_rename(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.thread_rename_editor.as_ref() else {
            self.thread_rename.cancel();
            return;
        };
        self.thread_rename.update_draft(editor.text().to_string());
        let Some(committed) = self.thread_rename.commit() else {
            cx.notify();
            return;
        };
        self.thread_rename_editor = None;
        if let Err(e) = self
            .shell
            .rename_thread(&committed.thread_id, &committed.draft)
        {
            tracing::error!(%e, "failed to rename thread");
            // Restore editing so the user can retry / Esc cancel.
            self.thread_rename
                .begin_rename(committed.thread_id, committed.draft);
            let mut editor = ComposerEditor::new();
            editor.set_text(
                self.thread_rename
                    .editing()
                    .map(|d| d.draft.clone())
                    .unwrap_or_default(),
            );
            self.thread_rename_editor = Some(editor);
        }
        cx.notify();
    }

    fn cancel_thread_rename(&mut self, cx: &mut Context<Self>) {
        self.thread_rename.cancel();
        self.thread_rename_editor = None;
        cx.notify();
    }

    fn begin_message_edit(
        &mut self,
        message_id: &str,
        content: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message_edit
            .begin_edit(message_id.to_string(), content.to_string());
        let mut editor = ComposerEditor::new();
        editor.set_font_metrics_from_rem(self.composer_rem);
        editor.set_container_width(600.0);
        editor.set_text(content.to_string());
        self.message_edit_editor = Some(editor);
        window.focus(&self.message_edit_focus);
        cx.notify();
    }

    fn commit_message_edit(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.message_edit_editor.as_ref() else {
            self.message_edit.cancel();
            return;
        };
        self.message_edit.update_draft(editor.text().to_string());
        let Some(committed) = edit_draft_commit(&mut self.message_edit) else {
            cx.notify();
            return;
        };
        self.message_edit_editor = None;

        let model = match self.active_model_name() {
            Some(m) => m.to_string(),
            None => {
                tracing::error!("cannot edit message without a model");
                return;
            }
        };
        let Some(provider) = self
            .chat_provider
            .take()
            .or_else(|| self.resolve_active_chat_provider())
        else {
            tracing::error!("cannot edit message without a provider");
            return;
        };
        if let Err(e) = self.shell.edit_user_message_and_regenerate(
            &committed.message_id,
            &committed.draft,
            provider,
            &model,
        ) {
            tracing::error!(%e, "failed to edit message and regenerate");
        }
        cx.notify();
    }

    fn cancel_message_edit(&mut self, cx: &mut Context<Self>) {
        self.message_edit.cancel();
        self.message_edit_editor = None;
        cx.notify();
    }

    fn switch_to_branch(&mut self, thread_id: &str, message_id: &str, cx: &mut Context<Self>) {
        if let Err(e) = self.shell.switch_branch(thread_id, message_id) {
            tracing::error!(%e, "failed to switch branch");
        }
        cx.notify();
    }

    fn on_thread_rename_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        if key == "enter" {
            self.commit_thread_rename(cx);
            return;
        }
        if key == "escape" {
            self.cancel_thread_rename(cx);
            return;
        }
        if let Some(editor) = self.thread_rename_editor.as_mut() {
            if editor.on_key_down(event) {
                if let Some(draft) = self.thread_rename.editing() {
                    let _ = draft; // keep state; draft synced on commit from editor
                }
                self.thread_rename.update_draft(editor.text().to_string());
                cx.notify();
            }
        }
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
                self.keyboard_nav
                    .set_focus(FocusRegion::Composer, self.thread_count());
                window.focus(&self.composer_focus);
                cx.notify();
            }
            Err(error) => tracing::error!(%error, "failed to create thread from sidebar"),
        }
    }

    fn select_thread(
        &mut self,
        thread_id: String,
        event: &MouseUpEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.click_count >= 2 {
            self.begin_thread_rename(&thread_id, window, cx);
            return;
        }
        match self.shell.select_thread(&thread_id) {
            Ok(()) => {
                self.keyboard_nav
                    .set_focus(FocusRegion::Composer, self.thread_count());
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
            self.memory_management.open();
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

    fn save_code_block_as_snippet(
        &mut self,
        thread_id: String,
        message_id: String,
        language: Option<String>,
        content: String,
        cx: &mut Context<Self>,
    ) {
        let title = snippet_title_from_language(language.as_deref());
        let lang = language.unwrap_or_default();
        if let Err(e) =
            self.shell
                .create_snippet_artifact(&thread_id, &message_id, &title, &content, &lang)
        {
            tracing::error!(%e, "failed to save snippet artifact");
        } else {
            self.artifacts_panel_open = true;
            cx.notify();
        }
    }

    fn toggle_search_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_panel.toggle();
        if self.search_panel.is_open() {
            self.search_query_editor
                .set_font_metrics_from_rem(self.composer_rem);
            self.search_query_editor.set_container_width(420.0);
            window.focus(&self.search_focus);
        }
        cx.notify();
    }

    fn build_search_corpus(&self) -> Vec<SearchDocument> {
        let mut docs = Vec::new();
        let threads = self.shell.state().threads.clone();
        for thread in &threads {
            docs.push(thread_title_document(
                &thread.id,
                &thread.title,
                thread.provider.as_deref(),
                thread.model.as_deref(),
                thread.created_at,
            ));
            if let Ok(messages) = self.shell.session().list_all_messages(&thread.id) {
                for msg in messages {
                    if msg.role == MessageRole::System {
                        continue;
                    }
                    docs.push(thread_message_document(
                        &thread.id,
                        &msg.id,
                        &thread.title,
                        &msg.content,
                        thread.provider.as_deref(),
                        thread.model.as_deref(),
                        msg.created_at,
                    ));
                }
            }
        }
        if let Ok(artifacts) = self.shell.list_all_artifacts() {
            for art in artifacts {
                docs.push(artifact_document(
                    &art.id.0,
                    &art.title,
                    &art.content,
                    &art.thread_id,
                    art.created_at,
                ));
            }
        }
        if let Ok(memories) = self.shell.list_memories() {
            for mem in memories {
                docs.push(memory_document(
                    &mem.id.0,
                    &mem.title,
                    &mem.content,
                    mem.created_at,
                ));
            }
        }
        docs
    }

    fn current_search_hits(&self) -> Vec<SearchHit> {
        let docs = self.build_search_corpus();
        search(
            self.search_query_editor.text(),
            &docs,
            self.search_panel.filters(),
        )
    }

    fn search_hits_display_order(hits: &[SearchHit]) -> Vec<&SearchHit> {
        group_hits_by_kind(hits)
            .into_iter()
            .flat_map(|(_, group)| group)
            .collect()
    }

    fn sync_search_query_from_editor(&mut self) {
        let text = self.search_query_editor.text().to_string();
        if text != self.search_panel.query() {
            self.search_panel.set_query(text);
        }
    }

    fn activate_search_hit(
        &mut self,
        hit: &SearchHit,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match hit.document.kind {
            SearchContentKind::Thread => {
                if let Some(thread_id) = hit.document.thread_id.clone() {
                    if let Err(error) = self.shell.select_thread(&thread_id) {
                        tracing::error!(%error, "failed to open thread from search");
                    } else {
                        self.pending_scroll_message_id = hit.document.message_id.clone();
                        self.keyboard_nav
                            .set_focus(FocusRegion::Messages, self.thread_count());
                        window.focus(&self.messages_focus);
                    }
                }
            }
            SearchContentKind::Artifact => {
                if let Some(thread_id) = hit.document.thread_id.as_deref() {
                    let _ = self.shell.select_thread(thread_id);
                }
                self.artifacts_panel_open = true;
                if let Ok(artifacts) = self.shell.list_all_artifacts() {
                    if let Some(art) = artifacts.iter().find(|a| a.id.0 == hit.document.id) {
                        self.begin_artifact_edit(
                            art.id.0.clone(),
                            art.title.clone(),
                            art.content.clone(),
                            cx,
                        );
                    }
                }
            }
            SearchContentKind::Memory => {
                self.memories_panel_open = true;
                self.memory_management.open();
            }
        }
        self.search_panel.close();
        cx.notify();
    }

    fn on_search_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        if key == "escape" {
            self.search_panel.close();
            cx.notify();
            return;
        }

        let hits = self.current_search_hits();
        match key {
            "up" => {
                let count = Self::search_hits_display_order(&hits).len();
                self.search_panel.move_selection(-1, count);
                cx.notify();
                return;
            }
            "down" => {
                let count = Self::search_hits_display_order(&hits).len();
                self.search_panel.move_selection(1, count);
                cx.notify();
                return;
            }
            "enter" => {
                let display = Self::search_hits_display_order(&hits);
                if let Some(hit) = display.get(self.search_panel.selected()) {
                    let hit = (*hit).clone();
                    self.activate_search_hit(&hit, window, cx);
                }
                return;
            }
            _ => {}
        }

        if self.search_query_editor.on_key_down(event) {
            self.sync_search_query_from_editor();
            cx.notify();
            return;
        }

        let ks = &event.keystroke;
        if ks.modifiers.control || ks.modifiers.alt || ks.modifiers.platform {
            return;
        }
        if ks.key.as_str() == "space" {
            self.search_query_editor.insert_char(' ');
            self.sync_search_query_from_editor();
            cx.notify();
            return;
        }
        if let Some(ref kc) = ks.key_char {
            for ch in kc.chars() {
                self.search_query_editor.insert_char(ch);
            }
            self.sync_search_query_from_editor();
            cx.notify();
        }
    }

    fn toggle_search_kind_filter(&mut self, kind: SearchContentKind, cx: &mut Context<Self>) {
        let filters = self.search_panel.filters_mut();
        if let Some(pos) = filters.kinds.iter().position(|k| *k == kind) {
            filters.kinds.remove(pos);
        } else {
            filters.kinds.push(kind);
        }
        cx.notify();
    }

    fn set_search_provider_filter(&mut self, provider: Option<String>, cx: &mut Context<Self>) {
        self.search_panel.filters_mut().provider = provider;
        cx.notify();
    }

    fn set_search_model_filter(&mut self, model: Option<String>, cx: &mut Context<Self>) {
        self.search_panel.filters_mut().model = model;
        cx.notify();
    }

    fn set_search_date_preset(&mut self, preset: SearchDatePreset, cx: &mut Context<Self>) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        self.search_panel.set_date_preset(preset, now_ms);
        cx.notify();
    }

    fn on_global_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystroke = &event.keystroke;

        // Model picker captures navigation while open
        if self.model_picker.is_open() {
            let count = self.model_picker_entries.len();
            let key = match keystroke.key.as_str() {
                "up" => Some(ModelPickerKey::Up),
                "down" => Some(ModelPickerKey::Down),
                "enter" => Some(ModelPickerKey::Enter),
                "escape" => Some(ModelPickerKey::Escape),
                _ => None,
            };
            if let Some(key) = key {
                let action = self.model_picker.handle_key(key, count);
                self.apply_model_picker_action(action, cx);
                return;
            }
        }
        let thread_count = self.shell.state().threads.len();
        let input = KeyInput {
            key: keystroke.key.as_str(),
            control: keystroke.modifiers.control,
            shift: keystroke.modifiers.shift,
            alt: keystroke.modifiers.alt || keystroke.modifiers.platform,
        };

        // Global focus / help chords (work from any region; Ctrl+B remains collapse)
        let global_nav = input.control && matches!(input.key, "1" | "2" | "3" | "/" | "?" | "f")
            || (input.key == "escape"
                && (self.keyboard_nav.help_visible() || self.search_panel.is_open()));
        if global_nav {
            if input.key == "escape" && self.search_panel.is_open() {
                self.search_panel.close();
                cx.notify();
                return;
            }
            let (consumed, action) = self.keyboard_nav.handle_key(input, thread_count);
            if consumed {
                self.apply_nav_action(action, window, cx);
                return;
            }
        }

        // Sidebar / message list navigation when those regions own focus
        match self.keyboard_nav.focus() {
            FocusRegion::Sidebar | FocusRegion::Messages => {
                let (consumed, action) = self.keyboard_nav.handle_key(input, thread_count);
                if consumed {
                    self.apply_nav_action(action, window, cx);
                    return;
                }
            }
            FocusRegion::Composer => {}
        }

        match keystroke.key.as_str() {
            "n" if keystroke.modifiers.control => self.create_new_thread_shortcut(window, cx),
            "l" | "k" if keystroke.modifiers.control => self.focus_composer_shortcut(window, cx),
            "r" if keystroke.modifiers.control => self.retry_generation_shortcut(window, cx),
            "g" if keystroke.modifiers.control && keystroke.modifiers.shift => {
                self.regenerate_message_shortcut(cx);
            }
            "escape" => self.cancel_generation(cx),
            "b" if keystroke.modifiers.control => self.toggle_sidebar(cx),
            _ => {}
        }
    }

    fn rebuild_model_picker_entries(&mut self) {
        let listed = self
            .shell
            .list_available_provider_models()
            .unwrap_or_default();
        let active = self
            .shell
            .state()
            .selected_thread_id
            .as_ref()
            .and_then(|id| self.shell.resolve_thread_provider_and_model(id).ok());
        let active_provider = active.as_ref().map(|(p, _)| p.as_str());
        let active_model = active.as_ref().map(|(_, m)| m.as_str());
        if self.model_picker.is_open() {
            let typed: Vec<(ModelProviderKind, Vec<String>)> = listed
                .iter()
                .filter_map(|(id, models)| {
                    ModelProviderKind::from_id(id).map(|kind| (kind, models.clone()))
                })
                .collect();
            self.model_picker_entries = refresh_picker_entries(
                &mut self.model_picker,
                &typed,
                active_provider.unwrap_or("ollama"),
                active_model.unwrap_or(""),
            );
        } else {
            self.model_picker_entries =
                entries_from_listed_providers(&listed, active_provider, active_model);
        }
    }

    fn open_model_picker(&mut self) {
        self.rebuild_model_picker_entries();
        open_picker_at_active(&mut self.model_picker, &self.model_picker_entries);
    }

    fn apply_model_picker_action(&mut self, action: ModelPickerAction, cx: &mut Context<Self>) {
        match action {
            ModelPickerAction::None | ModelPickerAction::HighlightChanged { .. } => {}
            ModelPickerAction::Dismiss => {}
            ModelPickerAction::Select { index } => {
                if let Some(entry) = self.model_picker_entries.get(index).cloned() {
                    if let Some(thread_id) = self.shell.state().selected_thread_id.clone() {
                        if let Err(e) = self.shell.select_thread_provider_model(
                            &thread_id,
                            entry.provider.id(),
                            &entry.model_name,
                        ) {
                            tracing::error!(%e, "failed to select model from picker");
                        } else {
                            self.chat_provider = self.resolve_active_chat_provider();
                        }
                    }
                }
            }
        }
        cx.notify();
    }

    fn apply_nav_action(&mut self, action: NavAction, window: &mut Window, cx: &mut Context<Self>) {
        match action {
            NavAction::None | NavAction::ThreadHighlightChanged { .. } | NavAction::ToggleHelp => {
                cx.notify();
            }
            NavAction::ToggleSearch => {
                self.toggle_search_panel(window, cx);
            }
            NavAction::FocusChanged(region) => {
                self.apply_focus_region(region, window, cx);
            }
            NavAction::SelectThread { index } => {
                let thread_id = self.shell.state().threads.get(index).map(|t| t.id.clone());
                if let Some(id) = thread_id {
                    if let Err(e) = self.shell.select_thread(&id) {
                        tracing::error!(%e, "failed to select thread via keyboard");
                    } else {
                        window.focus(&self.composer_focus);
                        self.keyboard_nav
                            .set_focus(FocusRegion::Composer, self.shell.state().threads.len());
                    }
                }
                cx.notify();
            }
            NavAction::ScrollMessages(direction) => {
                self.scroll_message_list(direction);
                cx.notify();
            }
        }
    }

    fn apply_focus_region(
        &mut self,
        region: FocusRegion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match region {
            FocusRegion::Sidebar => {
                if self.sidebar_collapsed {
                    if let Err(e) = self.shell.set_sidebar_collapsed(false) {
                        tracing::error!(%e, "failed to expand sidebar for focus");
                    } else {
                        self.sidebar_collapsed = false;
                    }
                }
                window.focus(&self.sidebar_focus);
            }
            FocusRegion::Messages => {
                window.focus(&self.messages_focus);
            }
            FocusRegion::Composer => {
                window.focus(&self.composer_focus);
            }
        }
        cx.notify();
    }

    fn scroll_message_list(&self, direction: ScrollDirection) {
        let offset = self.message_scroll_handle.offset();
        let max = self.message_scroll_handle.max_offset();
        let page = px(280.0);
        let delta = match direction {
            ScrollDirection::Up => page,
            ScrollDirection::Down => -page,
        };
        let new_y = (offset.y + delta).clamp(-max.height, px(0.0));
        self.message_scroll_handle
            .set_offset(point(offset.x, new_y));
    }

    fn thread_count(&self) -> usize {
        self.shell.state().threads.len()
    }

    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        match self.shell.toggle_sidebar_collapsed() {
            Ok(collapsed) => {
                self.sidebar_collapsed = collapsed;
                cx.notify();
            }
            Err(e) => tracing::error!(%e, "failed to toggle sidebar"),
        }
    }

    fn on_sidebar_resize_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.file_drop_active && event.pressed_button.is_none() {
            self.file_drop_active = false;
            cx.notify();
        }
        let Some(drag) = self.sidebar_drag else {
            return;
        };
        let x: f32 = event.position.x.into();
        self.sidebar_width = clamp_sidebar_width(drag.start_width + (x - drag.start_x));
        cx.notify();
    }

    fn on_sidebar_resize_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.sidebar_drag.take().is_none() {
            return;
        }
        if let Err(e) = self.shell.set_sidebar_width(self.sidebar_width) {
            tracing::error!(%e, "failed to persist sidebar width");
        }
        cx.notify();
    }

    fn create_new_thread_shortcut(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.shell.create_new_thread() {
            Ok(_) => {
                self.keyboard_nav
                    .set_focus(FocusRegion::Composer, self.thread_count());
                window.focus(&self.composer_focus);
                cx.notify();
            }
            Err(e) => tracing::error!(%e, "failed to create new thread via shortcut"),
        }
    }

    fn focus_composer_shortcut(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.keyboard_nav
            .set_focus(FocusRegion::Composer, self.thread_count());
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

    fn render_empty_state(&self, content: EmptyStateContent, theme: &M0Theme) -> impl IntoElement {
        let mut block = div()
            .p_4()
            .rounded_lg()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_lg()
                    .text_color(theme.text_muted)
                    .child(content.icon),
            )
            .child(
                div()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.text_primary)
                    .child(content.title),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child(content.body),
            );
        if let Some(hint) = content.action_hint {
            block = block.child(div().text_xs().text_color(theme.accent).child(hint));
        }
        block
    }

    fn render_error_presentation(
        &self,
        err: &ErrorPresentation,
        theme: &M0Theme,
    ) -> impl IntoElement {
        let mut block = div()
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(theme.border_strong)
            .bg(theme.surface_muted)
            .flex()
            .flex_col()
            .gap_1()
            .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight(700.))
                            .text_color(theme.accent)
                            .child(err.icon),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.text_primary)
                            .child(err.title),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(err.message.clone()),
            );
        if let Some(hint) = err.action_hint {
            block = block.child(div().text_xs().text_color(theme.accent).child(hint));
        }
        block
    }

    fn render_sidebar(
        &mut self,
        theme: &M0Theme,
        sidebar_focused: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if self.sidebar_collapsed {
            let focus_border = if sidebar_focused {
                theme.accent
            } else {
                theme.border_subtle
            };
            return div()
                .w(px(SIDEBAR_COLLAPSED_RAIL))
                .h_full()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .p_2()
                .bg(theme.sidebar_background)
                .border_r_2()
                .border_color(focus_border)
                .shadow(elevation_style(Elevation::Medium, theme.color_scheme).box_shadows())
                .track_focus(&self.sidebar_focus)
                .child(
                    div()
                        .rounded_md()
                        .px_2()
                        .py_2()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .hover(|style| style.bg(theme.surface_hover))
                        .child("»")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| this.toggle_sidebar(cx)),
                        ),
                )
                .child(
                    div()
                        .rounded_md()
                        .px_2()
                        .py_2()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .hover(|style| style.bg(theme.surface_hover))
                        .child("⌕")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| {
                                this.toggle_search_panel(window, cx);
                            }),
                        ),
                )
                .into_any_element();
        }

        let selected_thread_id = self.shell.state().selected_thread_id.clone();
        let threads = self.shell.state().threads.clone();
        let truncation_notice = self.shell.state().truncation_notice;
        let title_generating = self.shell.state().title_generating_thread_id.is_some();
        let generating_ids = self.shell.active_generating_thread_ids();
        let on_new_chat = cx.listener(Self::create_new_thread);
        let threads_empty = threads.is_empty();

        let mut thread_list = div().flex().flex_col().gap_2().min_w_0().w_full();
        if threads_empty {
            thread_list = thread_list
                .child(self.render_empty_state(empty_state(EmptyStateKind::NoThreads), theme));
        } else {
            for (index, thread) in threads.iter().enumerate() {
                let is_selected = Some(thread.id.as_str()) == selected_thread_id.as_deref();
                let is_highlighted =
                    sidebar_focused && self.keyboard_nav.thread_highlight() == Some(index);
                let thread_id = thread.id.clone();
                let is_generating = generating_ids.iter().any(|id| id == &thread_id);
                let is_renaming = self
                    .thread_rename
                    .editing()
                    .map(|d| d.thread_id.as_str() == thread_id.as_str())
                    .unwrap_or(false);
                let row_bg = if is_selected {
                    theme.surface_selected
                } else {
                    theme.surface_muted
                };
                let mut row = div()
                    .w_full()
                    .min_w_0()
                    .rounded_md()
                    .px_3()
                    .py_2()
                    .bg(row_bg)
                    .text_color(theme.text_primary)
                    .overflow_hidden()
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .on_mouse_up(MouseButton::Left, {
                        let id = thread_id.clone();
                        cx.listener(move |this, event, window, cx| {
                            this.select_thread(id.clone(), event, window, cx);
                        })
                    })
                    .on_mouse_up(MouseButton::Right, {
                        let id = thread_id.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.begin_thread_rename(&id, window, cx);
                        })
                    });

                if is_renaming {
                    if let Some(editor) = self.thread_rename_editor.as_mut() {
                        editor.set_container_width(self.sidebar_width.max(120.0) - 24.0);
                        row = row
                            .child(
                                div()
                                    .w_full()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(theme.accent)
                                    .bg(theme.composer_background)
                                    .px_2()
                                    .py_1()
                                    .track_focus(&self.thread_rename_focus)
                                    .on_key_down(cx.listener(Self::on_thread_rename_key_down))
                                    .child(editor.render_text(
                                        "Thread title",
                                        theme.text_primary,
                                        theme.text_muted,
                                        theme.accent,
                                    )),
                            )
                            .child(
                                div()
                                    .mt_1()
                                    .flex()
                                    .flex_row()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.accent)
                                            .cursor_pointer()
                                            .child("Rename")
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    this.commit_thread_rename(cx);
                                                }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .cursor_pointer()
                                            .child("Cancel")
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    this.cancel_thread_rename(cx);
                                                }),
                                            ),
                                    ),
                            );
                    }
                } else {
                    row = row
                        .truncate()
                        .child(format_sidebar_thread_title(&thread.title, is_generating));
                }

                if is_highlighted {
                    row = row.border_1().border_color(theme.accent);
                }
                thread_list = thread_list.child(row);
            }
        }

        let resize_handle = div()
            .id("sidebar-resize-handle")
            .absolute()
            .top(px(0.))
            .right(px(0.))
            .bottom(px(0.))
            .w(px(5.))
            .cursor_col_resize()
            .hover(|style| style.bg(theme.accent))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.sidebar_drag = Some(SidebarDrag {
                        start_x: event.position.x.into(),
                        start_width: this.sidebar_width,
                    });
                    cx.notify();
                }),
            );

        let focus_border = if sidebar_focused {
            theme.accent
        } else {
            theme.border_subtle
        };

        div()
            .relative()
            .w(px(self.sidebar_width))
            .h_full()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .min_w_0()
            .overflow_hidden()
            .bg(theme.sidebar_background)
            .border_r_2()
            .border_color(focus_border)
            .shadow(elevation_style(Elevation::Medium, theme.color_scheme).box_shadows())
            .track_focus(&self.sidebar_focus)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.keyboard_nav
                        .set_focus(FocusRegion::Sidebar, this.thread_count());
                    window.focus(&this.sidebar_focus);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .min_w_0()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.text_primary)
                            .truncate()
                            .child("Ronin"),
                    )
                    .child(
                        div()
                            .rounded_md()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .cursor_pointer()
                            .hover(|style| {
                                style.bg(theme.surface_hover).text_color(theme.text_primary)
                            })
                            .child("«")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| this.toggle_sidebar(cx)),
                            ),
                    ),
            )
            .child(
                div()
                    .rounded_lg()
                    .px_3()
                    .py_2()
                    .bg(theme.accent)
                    .text_color(theme.accent_text)
                    .font_weight(FontWeight(500.))
                    .truncate()
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
                    .truncate()
                    .child("Memories")
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.memories_panel_open = !this.memories_panel_open;
                            if this.memories_panel_open {
                                this.memory_management.open();
                            } else {
                                this.memory_management.close();
                            }
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
                    .truncate()
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
                    .rounded_lg()
                    .px_3()
                    .py_2()
                    .bg(theme.surface_muted)
                    .text_color(theme.text_primary)
                    .font_weight(FontWeight(500.))
                    .truncate()
                    .child("Search")
                    .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            this.toggle_search_panel(window, cx);
                        }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .id("sidebar-scroll")
                    .overflow_y_scroll()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .mb_2()
                            .child("Threads"),
                    )
                    .child(
                        if let Some(hint) = title_generation_status_label(title_generating) {
                            div().text_xs().text_color(theme.accent).mb_2().child(hint)
                        } else {
                            div()
                        },
                    )
                    .child(thread_list)
                    .child(if truncation_notice {
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
                    .min_w_0()
                    .overflow_hidden()
                    .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows())
                    .child(self.render_provider_status(self.shell.state(), theme, cx)),
            )
            .child(resize_handle)
            .into_any_element()
    }

    fn render_provider_status(
        &self,
        state: &ShellState,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let status_body = match &state.provider_status {
            ProviderStatus::NotConfigured => div()
                .flex()
                .flex_col()
                .gap_1()
                .child("Provider: not configured")
                .child("Model: not selected")
                .into_any_element(),
            ProviderStatus::OllamaOffline => self
                .render_empty_state(empty_state(EmptyStateKind::OllamaOffline), theme)
                .into_any_element(),
            ProviderStatus::OllamaOnline { model } => div()
                .cursor_pointer()
                .hover(|s| s.text_color(theme.accent))
                .child(format!("Provider: ollama\nModel: {model}"))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.open_model_picker();
                        cx.notify();
                    }),
                )
                .into_any_element(),
            ProviderStatus::OllamaNoModels => self
                .render_empty_state(empty_state(EmptyStateKind::NoModelsInstalled), theme)
                .into_any_element(),
            ProviderStatus::OpenAiReady { model } => div()
                .cursor_pointer()
                .hover(|s| s.text_color(theme.accent))
                .child(format!("Provider: openai\nModel: {model}"))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.open_model_picker();
                        cx.notify();
                    }),
                )
                .into_any_element(),
            ProviderStatus::OpenAiError { message } => {
                let err = error_presentation(ErrorKind::Provider, message);
                self.render_error_presentation(&err, theme)
                    .into_any_element()
            }
            ProviderStatus::OpenAiNotConfigured => {
                let detail = format_provider_error(
                    "openai",
                    "No API key found. Set OPENAI_API_KEY or add a key in settings.",
                );
                let err = error_presentation(ErrorKind::Provider, &detail);
                self.render_error_presentation(&err, theme)
                    .into_any_element()
            }
        };

        let mut column = div().flex().flex_col().gap_1().child(status_body);

        column = column.child(
            div()
                .text_xs()
                .text_color(theme.accent)
                .cursor_pointer()
                .hover(|s| s.text_color(theme.accent_hover))
                .child(test_connection_button_label())
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        match this.shell.test_connection() {
                            Ok(result) => {
                                tracing::info!(
                                    success = result.is_success(),
                                    "provider connection test finished"
                                );
                            }
                            Err(e) => {
                                tracing::error!("provider connection test failed: {e}");
                            }
                        }
                        cx.notify();
                    }),
                ),
        );

        if let Some(result) = &state.connection_test {
            let success = connection_test_is_success(result);
            column = column.child(
                div()
                    .text_xs()
                    .text_color(if success {
                        theme.accent
                    } else {
                        theme.text_primary
                    })
                    .child(format_connection_test_result(result)),
            );
        }

        column.into_any_element()
    }

    fn render_messages(
        &mut self,
        theme: &M0Theme,
        messages_focused: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let messages_opt = self.shell.state().messages.clone();
        let messages = match messages_opt {
            Some(msgs) if !msgs.is_empty() => msgs.clone(),
            _ => {
                return div().flex_1().p_6().id("empty-messages").child(
                    self.render_empty_state(empty_state(EmptyStateKind::EmptyThread), theme),
                );
            }
        };

        let is_generating = self.shell.is_generation_active();
        let last_assistant_id = messages
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::Assistant)
            .map(|m| m.id.clone());
        let scroll_target = self.pending_scroll_message_id.clone();
        let visible_ids: Vec<String> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| m.id.clone())
            .collect();
        if let Some(target) = scroll_target.as_ref() {
            if let Some(idx) = visible_ids.iter().position(|id| id == target) {
                let max = self.message_scroll_handle.max_offset();
                if max.height > px(0.0) {
                    let frac = idx as f32 / visible_ids.len().max(1) as f32;
                    let new_y = (-max.height * frac).clamp(-max.height, px(0.0));
                    self.message_scroll_handle.set_offset(point(px(0.0), new_y));
                }
            }
        }

        let message_elements: Vec<_> = messages
            .into_iter()
            .filter_map(|msg| {
                if msg.role == MessageRole::System {
                    return None;
                }
                let (label, bg) = match msg.role {
                    MessageRole::User => ("You", theme.surface_muted),
                    MessageRole::Assistant => ("Assistant", theme.surface_selected),
                    MessageRole::System => unreachable!(),
                };
                let is_search_match = scroll_target.as_deref() == Some(msg.id.as_str());
                let raw_content = msg.content.clone();
                let is_copied = self.copied_state.as_ref().map(|(id, _)| id) == Some(&msg.id);
                let copy_text = if is_copied { "Copied!" } else { "Copy" };
                let editing_this = self
                    .message_edit
                    .editing()
                    .map(|d| d.message_id == msg.id)
                    .unwrap_or(false);
                let mut message_body = div().w_full().min_w_0().flex().flex_col().gap_3();

                if !editing_this {
                    let blocks =
                        if let Some((len, cached_blocks)) = self.parsed_messages.get(&msg.id) {
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
                                ronin::markdown_view::render_inline_flow(&inlines, theme)
                            }
                            ronin::markdown::MarkdownBlock::CodeBlock { language, content } => {
                                let lang_label = language
                                    .clone()
                                    .filter(|l| !l.is_empty())
                                    .unwrap_or_else(|| "text".to_string());
                                let code_lines =
                                    ronin::markdown_view::render_highlighted_code_lines(
                                        language.as_deref(),
                                        &content,
                                        theme,
                                    )
                                    .id(gpui::SharedString::from(format!(
                                        "{}-code-scroll-{}",
                                        msg.id, block_idx
                                    )))
                                    .overflow_x_scroll();
                                let block_id = format!("{}-code-{}", msg.id, block_idx);
                                let is_block_copied =
                                    self.copied_state.as_ref().map(|(id, _)| id) == Some(&block_id);
                                let block_copy_text =
                                    if is_block_copied { "Copied!" } else { "Copy" };
                                let code_content = content.clone();
                                let code_language = language.clone();
                                let save_thread_id = msg.thread_id.clone();
                                let save_message_id = msg.id.clone();
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
                                            .flex()
                                            .flex_row()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.accent)
                                                    .cursor_pointer()
                                                    .child(save_code_block_as_snippet_label())
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener({
                                                            let save_thread_id =
                                                                save_thread_id.clone();
                                                            let save_message_id =
                                                                save_message_id.clone();
                                                            let code_content = code_content.clone();
                                                            let code_language =
                                                                code_language.clone();
                                                            move |this, _, _, cx| {
                                                                this.save_code_block_as_snippet(
                                                                    save_thread_id.clone(),
                                                                    save_message_id.clone(),
                                                                    code_language.clone(),
                                                                    code_content.clone(),
                                                                    cx,
                                                                );
                                                            }
                                                        }),
                                                    ),
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
                                            ),
                                    );
                                div()
                                    .w_full()
                                    .min_w_0()
                                    .overflow_hidden()
                                    .bg(theme.surface_hover)
                                    .rounded_md()
                                    .p_3()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .shadow(
                                        elevation_style(Elevation::Low, theme.color_scheme)
                                            .box_shadows(),
                                    )
                                    .child(header)
                                    .child(code_lines)
                            }
                            ronin::markdown::MarkdownBlock::List(items) => {
                                let mut list_div =
                                    div().w_full().min_w_0().flex().flex_col().gap_1().pl_4();
                                for item in items {
                                    let li_content = ronin::markdown_view::render_inline_flow(
                                        &item.inlines,
                                        theme,
                                    );
                                    list_div = list_div.child(
                                        div()
                                            .w_full()
                                            .min_w_0()
                                            .flex()
                                            .flex_row()
                                            .gap_2()
                                            .child(div().flex_shrink_0().child("•"))
                                            .child(div().flex_1().min_w_0().child(li_content)),
                                    );
                                }
                                list_div
                            }
                        };
                        message_body = message_body.child(block_el);
                    }

                    if let Some(attachments_row) = self.render_message_attachments(&msg.id, theme) {
                        message_body = message_body.child(attachments_row);
                    }
                } // !editing_this

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
                                        this.copy_to_clipboard(
                                            msg_id.clone(),
                                            raw_content.clone(),
                                            cx,
                                        );
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

                if msg.role == MessageRole::User {
                    if editing_this {
                        if let Some(editor) = self.message_edit_editor.as_mut() {
                            editor.set_container_width(560.0);
                            message_body = message_body.child(
                                div()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(theme.accent)
                                    .bg(theme.composer_background)
                                    .p_2()
                                    .track_focus(&self.message_edit_focus)
                                    .on_key_down(cx.listener(
                                        |this, event: &KeyDownEvent, _, cx| {
                                            if event.keystroke.key.as_str() == "enter"
                                                && event.keystroke.modifiers.control
                                            {
                                                this.commit_message_edit(cx);
                                                return;
                                            }
                                            if event.keystroke.key.as_str() == "escape" {
                                                this.cancel_message_edit(cx);
                                                return;
                                            }
                                            if let Some(ed) = this.message_edit_editor.as_mut() {
                                                if ed.on_key_down(event) {
                                                    this.message_edit
                                                        .update_draft(ed.text().to_string());
                                                    cx.notify();
                                                }
                                            }
                                        },
                                    ))
                                    .child(editor.render_text(
                                        "Edit message",
                                        theme.text_primary,
                                        theme.text_muted,
                                        theme.accent,
                                    )),
                            );
                            message_actions = message_actions
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .cursor_pointer()
                                        .child("Save & regenerate")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.commit_message_edit(cx);
                                            }),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .cursor_pointer()
                                        .child("Cancel")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.cancel_message_edit(cx);
                                            }),
                                        ),
                                );
                        }
                    } else {
                        message_actions = message_actions.child(
                            div()
                                .text_xs()
                                .text_color(if is_generating {
                                    theme.text_muted
                                } else {
                                    theme.accent
                                })
                                .cursor_pointer()
                                .child("Edit")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let msg_id = msg.id.clone();
                                        let content = raw_content.clone();
                                        move |this, _, window, cx| {
                                            if !this.shell.is_generation_active() {
                                                this.begin_message_edit(
                                                    &msg_id, &content, window, cx,
                                                );
                                            }
                                        }
                                    }),
                                ),
                        );
                    }
                }

                if let Ok(siblings) = self.shell.branch_siblings(&msg.thread_id, &msg.id) {
                    if siblings.len() >= 2 {
                        if let Some(idx) = siblings.iter().position(|s| s.id == msg.id) {
                            let prev_id = if idx > 0 {
                                Some(siblings[idx - 1].id.clone())
                            } else {
                                None
                            };
                            let next_id = siblings.get(idx + 1).map(|s| s.id.clone());
                            let thread_id = msg.thread_id.clone();
                            message_actions = message_actions
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(branch_nav_label(idx, siblings.len())),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if prev_id.is_some() {
                                            theme.accent
                                        } else {
                                            theme.text_muted
                                        })
                                        .cursor_pointer()
                                        .child("‹")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener({
                                                let thread_id = thread_id.clone();
                                                move |this, _, _, cx| {
                                                    if let Some(id) = prev_id.clone() {
                                                        this.switch_to_branch(&thread_id, &id, cx);
                                                    }
                                                }
                                            }),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if next_id.is_some() {
                                            theme.accent
                                        } else {
                                            theme.text_muted
                                        })
                                        .cursor_pointer()
                                        .child("›")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener({
                                                let thread_id = thread_id.clone();
                                                move |this, _, _, cx| {
                                                    if let Some(id) = next_id.clone() {
                                                        this.switch_to_branch(&thread_id, &id, cx);
                                                    }
                                                }
                                            }),
                                        ),
                                );
                        }
                    }
                }

                if msg.role == MessageRole::Assistant
                    && (msg.status == ronin_core::MessageStatus::Failed
                        || msg.status == ronin_core::MessageStatus::Error)
                {
                    let detail = msg
                        .error_message
                        .clone()
                        .unwrap_or_else(|| "The response stream was interrupted.".to_string());
                    let err = error_presentation(ErrorKind::StreamFailure, &detail);
                    message_body = message_body.child(self.render_error_presentation(&err, theme));
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
                        .min_w_0()
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
                        .child({
                            let mut bubble = div()
                                .w_full()
                                .min_w_0()
                                .overflow_hidden()
                                .rounded_lg()
                                .px_4()
                                .py_3()
                                .bg(bg)
                                .text_color(theme.text_primary);
                            if is_search_match {
                                bubble = bubble.border_2().border_color(theme.accent);
                            }
                            bubble.child(message_body)
                        }),
                )
            })
            .collect();

        let mut container = div()
            .flex_1()
            .p_6()
            .flex()
            .flex_col()
            .gap_2()
            .id("message-scroll")
            .overflow_y_scroll()
            .track_scroll(&self.message_scroll_handle)
            .track_focus(&self.messages_focus)
            .border_l_2()
            .border_color(if messages_focused {
                theme.accent
            } else {
                theme.app_background
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    let _ = this.dismiss_active_picker();
                    this.keyboard_nav
                        .set_focus(FocusRegion::Messages, this.thread_count());
                    window.focus(&this.messages_focus);
                    cx.notify();
                }),
            );
        for el in message_elements {
            container = container.child(el);
        }
        if is_generating {
            let motion = streaming_motion();
            let elapsed = self.blink_start.elapsed().as_millis() as u64;
            container = container.child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(generating_label(elapsed, &motion)),
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

        let mut composer = div()
            .id("composer")
            .p_6()
            .flex()
            .flex_col()
            .gap_2()
            .can_drop(|value, _, _| value.is::<ExternalPaths>())
            .on_drop(cx.listener(Self::on_external_paths_drop));

        // Screenshot actions (interactive + window-targeted when portal supports it)
        composer = composer.child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(
                    div()
                        .rounded_lg()
                        .px_3()
                        .py_1()
                        .bg(theme.surface_muted)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .child("Screenshot")
                        .hover(|style| style.bg(theme.surface_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.take_screenshot_action(cx);
                            }),
                        ),
                )
                .child(
                    div()
                        .rounded_lg()
                        .px_3()
                        .py_1()
                        .bg(theme.surface_muted)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .child("Window")
                        .hover(|style| style.bg(theme.surface_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.take_window_screenshot_action(cx);
                            }),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("or type @screenshot"),
                ),
        );

        // Folder attach file selection
        if !self.pending_folder_attaches.is_empty() {
            for (folder_idx, folder) in self.pending_folder_attaches.iter().enumerate() {
                let mut panel = div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.text_primary)
                            .child(format!(
                                "Folder: {} ({} selected)",
                                folder.name(),
                                folder.selected_count()
                            )),
                    );
                if folder.listing().truncated {
                    panel = panel.child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(ronin::folder_attach::folder_truncated_hint()),
                    );
                }
                for entry in folder.listing().entries.iter().take(40) {
                    let rel = entry.relative_path.clone();
                    let selected = folder.is_selected(&rel);
                    panel = panel.child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .items_center()
                            .cursor_pointer()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if selected {
                                        theme.accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .child(if selected { "[x]" } else { "[ ]" }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .child(rel.clone()),
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    if let Some(folder) =
                                        this.pending_folder_attaches.get_mut(folder_idx)
                                    {
                                        folder.toggle_file(&rel);
                                    }
                                    this.attachment_size_warn.clear();
                                    cx.notify();
                                }),
                            ),
                    );
                }
                composer = composer.child(panel);
            }
        }

        // Attachment size warning
        if let Some(warn) = self.attachment_size_warn.warning() {
            let msg = warn.message.clone();
            composer = composer.child(
                div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.accent)
                    .bg(theme.surface_hover)
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_xs().text_color(theme.text_primary).child(msg))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.accent)
                                    .text_xs()
                                    .text_color(theme.accent_text)
                                    .cursor_pointer()
                                    .child("Proceed anyway")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.attachment_size_warn.acknowledge_and_proceed();
                                            cx.notify();
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .bg(theme.surface_muted)
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .cursor_pointer()
                                    .child("Dismiss")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.attachment_size_warn.clear();
                                            cx.notify();
                                        }),
                                    ),
                            ),
                    ),
            );
        }

        // Attachment previews (thumbnails / file metadata)
        let previews = self.composer_attachment_previews();
        if !previews.is_empty() {
            let mut row = div().flex().flex_row().flex_wrap().gap_2();
            for (index, preview) in previews.into_iter().enumerate() {
                let pending_len = self.pending_attachments.len();
                let preattached_len = self.preattached_files.len();
                let mut card = self.render_composer_attachment_preview(&preview, theme);
                if index < pending_len + preattached_len {
                    card = card.child(
                        div()
                            .text_xs()
                            .text_color(theme.accent)
                            .cursor_pointer()
                            .child("×")
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    if index < this.pending_attachments.len() {
                                        this.pending_attachments.remove(index);
                                    } else {
                                        let pre_idx = index - this.pending_attachments.len();
                                        this.remove_preattached_file(pre_idx, cx);
                                        return;
                                    }
                                    cx.notify();
                                }),
                            ),
                    );
                }
                row = row.child(card);
            }
            composer = composer.child(row);
        }

        // attachment errors
        for error in &self.attachment_errors {
            let err = error_presentation(ErrorKind::Attachment, error);
            composer = composer.child(self.render_error_presentation(&err, theme));
        }

        // Unobtrusive context/token size indicator
        composer = composer.child(self.render_context_indicator(theme));
        if let Some(memory_ind) = self.render_memory_context_indicator(theme) {
            composer = composer.child(memory_ind);
        }

        // @ attachment / / action picker dropdown
        if let Some(picker) = self.active_picker() {
            if !picker.items.is_empty() {
                let ci = self
                    .completion_index
                    .min(picker.items.len().saturating_sub(1));
                let mut dropdown = div()
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.border_subtle)
                    .bg(theme.surface_muted)
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows());
                for (i, item) in picker.items.iter().enumerate() {
                    let bg = if i == ci {
                        theme.surface_selected
                    } else {
                        theme.surface_muted
                    };
                    let label = format!("{} — {}", item.insert, item.label);
                    let mut entry = div()
                        .px_3()
                        .py_1()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .bg(bg)
                        .hover(|style| style.bg(theme.surface_hover).cursor_pointer())
                        .child(label);
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
                .overflow_hidden()
                .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows());
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

        // artifact completions dropdown (same list as the Artifacts panel)
        let art_matches = self.artifact_completions();
        if !art_matches.is_empty() {
            let ci = self
                .completion_index
                .min(art_matches.len().saturating_sub(1));
            let mut dropdown = div()
                .rounded_lg()
                .border_1()
                .border_color(theme.border_subtle)
                .bg(theme.surface_muted)
                .flex()
                .flex_col()
                .overflow_hidden()
                .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows());
            for (i, (id, title)) in art_matches.iter().enumerate() {
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
                .overflow_hidden()
                .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows());
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
        } else if completions::file_path_completion_active(
            self.composer.text(),
            self.composer.cursor(),
        ) {
            composer =
                composer.child(
                    div()
                        .rounded_lg()
                        .border_1()
                        .border_color(theme.border_subtle)
                        .bg(theme.surface_muted)
                        .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows())
                        .child(self.render_empty_state(
                            empty_state(EmptyStateKind::NoSearchResults),
                            theme,
                        )),
                );
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
                        .shadow(
                            elevation_style(Elevation::Medium, theme.color_scheme).box_shadows(),
                        )
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
        let items: Vec<MemoryListItem> = memories
            .iter()
            .map(|m| {
                MemoryListItem::from_fields(
                    m.id.0.clone(),
                    m.title.clone(),
                    m.content.clone(),
                    m.enabled,
                    m.is_profile,
                    m.created_at,
                )
            })
            .collect();
        let grouped = group_memory_cards(&items);

        let mut list = div()
            .flex()
            .flex_col()
            .gap_3()
            .id("memories-list")
            .overflow_y_scroll()
            .w_full()
            .h_full();
        if grouped.is_empty() {
            list =
                list.child(self.render_empty_state(empty_state(EmptyStateKind::NoMemories), theme));
        }
        for (group, cards) in grouped {
            list = list.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.text_muted)
                    .child(group.label()),
            );
            for card in cards {
                let id = card.id.clone();
                let enabled = card.enabled;
                let is_profile = card.is_profile;
                let card_el = div()
                    .p_3()
                    .bg(if is_profile {
                        theme.surface_selected
                    } else {
                        theme.surface_hover
                    })
                    .rounded_md()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .border_1()
                    .border_color(if is_profile {
                        theme.accent
                    } else {
                        theme.border_subtle
                    })
                    .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows())
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .font_weight(FontWeight(600.))
                                    .text_color(theme.text_primary)
                                    .truncate()
                                    .child(card.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if enabled {
                                        theme.accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .child(card.status_label),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(card.snippet.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .mt_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(format!("Created {}", card.created_label)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .px_2()
                                    .rounded_md()
                                    .bg(theme.surface_muted)
                                    .text_color(theme.text_muted)
                                    .child(if is_profile {
                                        PROFILE_GROUP_LABEL
                                    } else {
                                        "Regular"
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_2()
                            .mt_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.accent)
                                    .cursor_pointer()
                                    .child(if enabled { "Disable" } else { "Enable" })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener({
                                            let id = id.clone();
                                            move |this, _, _, cx| {
                                                let mem_id = ronin_core::MemoryId(id.clone());
                                                this.shell
                                                    .set_memory_enabled(&mem_id, !enabled)
                                                    .ok();
                                                cx.notify();
                                            }
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.accent)
                                    .cursor_pointer()
                                    .child(if is_profile {
                                        "Remove from profile"
                                    } else {
                                        "Add to profile"
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener({
                                            let id = id.clone();
                                            move |this, _, _, cx| {
                                                let mem_id = ronin_core::MemoryId(id.clone());
                                                this.shell
                                                    .set_memory_profile(&mem_id, !is_profile)
                                                    .ok();
                                                cx.notify();
                                            }
                                        }),
                                    ),
                            )
                            .child(
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
                                                this.shell
                                                    .delete_memory(&ronin_core::MemoryId(
                                                        id.clone(),
                                                    ))
                                                    .ok();
                                                cx.notify();
                                            }
                                        }),
                                    ),
                            ),
                    );
                list = list.child(card_el);
            }
        }

        div()
            .w(px(360.0))
            .h_full()
            .bg(theme.sidebar_background)
            .border_l_1()
            .border_color(theme.border_subtle)
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .shadow(elevation_style(Elevation::High, theme.color_scheme).box_shadows())
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.text_primary)
                            .child("Memory management"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .cursor_pointer()
                            .child("Close")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.memories_panel_open = false;
                                    this.memory_management.close();
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .rounded_md()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .bg(theme.surface_muted)
                            .text_color(theme.text_primary)
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.surface_hover))
                            .child("New memory")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.shell.create_memory("New memory", "Add details…").ok();
                                    cx.notify();
                                }),
                            ),
                    )
                    .child(
                        div()
                            .rounded_md()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .bg(theme.accent)
                            .text_color(theme.accent_text)
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.accent_hover))
                            .child("New profile memory")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.shell
                                        .create_profile_memory(
                                            "Profile",
                                            "Preferences, role, or context…",
                                        )
                                        .ok();
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            .child(list)
    }

    fn begin_artifact_edit(
        &mut self,
        id: String,
        title: String,
        content: String,
        cx: &mut Context<Self>,
    ) {
        self.artifacts_panel
            .begin_edit(id, title.clone(), content.clone());
        let mut title_ed = ComposerEditor::new();
        title_ed.set_font_metrics_from_rem(self.composer_rem);
        title_ed.set_text(title);
        let mut content_ed = ComposerEditor::new();
        content_ed.set_font_metrics_from_rem(self.composer_rem);
        content_ed.set_text(content);
        self.artifact_title_editor = Some(title_ed);
        self.artifact_content_editor = Some(content_ed);
        cx.notify();
    }

    fn cancel_artifact_edit(&mut self, cx: &mut Context<Self>) {
        self.artifacts_panel.cancel_edit();
        self.artifact_title_editor = None;
        self.artifact_content_editor = None;
        cx.notify();
    }

    fn save_artifact_edit(&mut self, cx: &mut Context<Self>) {
        let title = self
            .artifact_title_editor
            .as_ref()
            .map(|e| e.text().to_string())
            .unwrap_or_default();
        let content = self
            .artifact_content_editor
            .as_ref()
            .map(|e| e.text().to_string())
            .unwrap_or_default();
        if let Some(draft) = self.artifacts_panel.commit_edit() {
            let id = ronin_core::ArtifactId(draft.id);
            if let Err(e) = self.shell.update_artifact(&id, &title, &content) {
                tracing::error!(%e, "failed to update artifact");
            }
        }
        self.artifact_title_editor = None;
        self.artifact_content_editor = None;
        cx.notify();
    }

    fn attach_artifact_ref(&mut self, id: &str, cx: &mut Context<Self>) {
        self.composer.insert_str(&format!("@artifact:{id} "));
        cx.notify();
    }

    fn render_artifacts_panel(
        &mut self,
        theme: &M0Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let artifacts = self.shell.list_all_artifacts().unwrap_or_default();
        let threads = self.shell.state().threads.clone();
        let thread_title = |thread_id: &str| -> String {
            threads
                .iter()
                .find(|t| t.id == thread_id)
                .map(|t| t.title.clone())
                .unwrap_or_else(|| "Unknown thread".to_string())
        };

        let cards: Vec<_> = artifacts
            .iter()
            .map(|art| artifact_preview_card(art, &thread_title(&art.thread_id)))
            .collect();
        let empty = artifacts_empty_state(&cards);
        let pending_delete = self.artifacts_panel.pending_delete_id().map(str::to_string);
        let editing_id = self.artifacts_panel.editing().map(|d| d.id.clone());

        let mut list = div()
            .flex()
            .flex_col()
            .gap_2()
            .id("artifacts-list")
            .overflow_y_scroll()
            .w_full()
            .h_full();

        if let Some(_empty_msg) = empty {
            list = list
                .child(self.render_empty_state(empty_state(EmptyStateKind::NoArtifacts), theme));
        }

        for card in &cards {
            let id = card.id.clone();
            let title = card.title.clone();
            let artifact = artifacts.iter().find(|a| a.id.0 == card.id);
            let content = artifact.map(|a| a.content.clone()).unwrap_or_default();
            let is_snippet = artifact.is_some_and(|a| a.is_snippet());
            let language = artifact.and_then(|a| a.language.clone());
            let badge = artifact
                .map(artifact_kind_badge)
                .unwrap_or_else(|| ARTIFACT_KIND_BADGE.to_string());
            let is_pending_delete = pending_delete.as_deref() == Some(card.id.as_str());
            let is_editing = editing_id.as_deref() == Some(card.id.as_str());

            let mut card_el = div()
                .p_3()
                .bg(theme.surface_hover)
                .rounded_md()
                .flex()
                .flex_col()
                .gap_1()
                .shadow(elevation_style(Elevation::Low, theme.color_scheme).box_shadows())
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_between()
                        .items_center()
                        .child(
                            div()
                                .font_weight(FontWeight(600.))
                                .text_color(theme.text_primary)
                                .child(card.title.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .px_2()
                                .py_0p5()
                                .rounded_md()
                                .bg(theme.surface_muted)
                                .text_color(theme.accent)
                                .child(badge),
                        ),
                );

            if is_snippet {
                let code_lines = ronin::markdown_view::render_highlighted_code_lines(
                    language.as_deref(),
                    &content,
                    theme,
                )
                .id(gpui::SharedString::from(format!(
                    "artifact-code-{}",
                    card.id
                )))
                .max_h(px(160.0))
                .overflow_y_scroll();
                card_el = card_el.child(
                    div()
                        .mt_1()
                        .rounded_md()
                        .bg(theme.composer_background)
                        .p_2()
                        .child(code_lines),
                );
            } else {
                card_el = card_el.child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(card.snippet.clone()),
                );
            }

            card_el = card_el.child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(format!("From: {}", card.source_thread_title)),
            );

            if is_editing {
                let title_editor = self.artifact_title_editor.as_mut();
                let content_editor = self.artifact_content_editor.as_mut();
                if let (Some(title_ed), Some(content_ed)) = (title_editor, content_editor) {
                    title_ed.set_container_width(260.0);
                    content_ed.set_container_width(260.0);
                    card_el = card_el
                        .child(
                            div()
                                .mt_2()
                                .p_2()
                                .rounded_md()
                                .border_1()
                                .border_color(theme.border_subtle)
                                .bg(theme.composer_background)
                                .track_focus(&self.artifact_title_focus)
                                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                                    if let Some(ed) = this.artifact_title_editor.as_mut() {
                                        if ed.on_key_down(event) {
                                            cx.notify();
                                        }
                                    }
                                }))
                                .child(title_ed.render_text(
                                    "Title",
                                    theme.text_primary,
                                    theme.text_muted,
                                    theme.accent,
                                )),
                        )
                        .child(
                            div()
                                .mt_1()
                                .p_2()
                                .rounded_md()
                                .border_1()
                                .border_color(theme.border_subtle)
                                .bg(theme.composer_background)
                                .h(px(120.0))
                                .track_focus(&self.artifact_content_focus)
                                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                                    if let Some(ed) = this.artifact_content_editor.as_mut() {
                                        if ed.on_key_down(event) {
                                            cx.notify();
                                        }
                                    }
                                }))
                                .child(content_ed.render_text(
                                    "Content",
                                    theme.text_primary,
                                    theme.text_muted,
                                    theme.accent,
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_2()
                                .mt_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .cursor_pointer()
                                        .child("Save")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.save_artifact_edit(cx);
                                            }),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .cursor_pointer()
                                        .child("Cancel")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.cancel_artifact_edit(cx);
                                            }),
                                        ),
                                ),
                        );
                }
            } else if is_pending_delete {
                card_el = card_el.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .mt_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Delete this artifact?"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child("Confirm")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = id.clone();
                                        move |this, _, _, cx| {
                                            if this.artifacts_panel.confirm_delete().as_deref()
                                                == Some(id.as_str())
                                            {
                                                this.shell
                                                    .delete_artifact(&ronin_core::ArtifactId(
                                                        id.clone(),
                                                    ))
                                                    .ok();
                                            }
                                            cx.notify();
                                        }
                                    }),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .cursor_pointer()
                                .child("Cancel")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        this.artifacts_panel.cancel_delete();
                                        cx.notify();
                                    }),
                                ),
                        ),
                );
            } else {
                card_el = card_el.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .mt_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child("Attach")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = id.clone();
                                        move |this, _, _, cx| {
                                            this.attach_artifact_ref(&id, cx);
                                        }
                                    }),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child("Edit")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = id.clone();
                                        let title = title.clone();
                                        let content = content.clone();
                                        move |this, _, _, cx| {
                                            this.begin_artifact_edit(
                                                id.clone(),
                                                title.clone(),
                                                content.clone(),
                                                cx,
                                            );
                                        }
                                    }),
                                ),
                        )
                        .child(
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
                                            this.artifacts_panel.request_delete(id.clone());
                                            cx.notify();
                                        }
                                    }),
                                ),
                        ),
                );
            }

            list = list.child(card_el);
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
            .shadow(elevation_style(Elevation::High, theme.color_scheme).box_shadows())
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

    fn render_search_panel(&mut self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        self.search_query_editor
            .set_font_metrics_from_rem(self.composer_rem);
        self.search_query_editor.set_container_width(440.0);
        self.sync_search_query_from_editor();

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        // Refresh relative date bounds while the panel is open so "7 days" stays accurate.
        self.search_panel
            .date_preset()
            .apply(self.search_panel.filters_mut(), now_ms);
        let date_preset = self.search_panel.date_preset();
        let filters = self.search_panel.filters().clone();

        let hits = self.current_search_hits();
        let selected = self.search_panel.selected();
        let corpus = self.build_search_corpus();
        let mut providers: Vec<String> = corpus.iter().filter_map(|d| d.provider.clone()).collect();
        providers.sort();
        providers.dedup();
        let mut models: Vec<String> = corpus.iter().filter_map(|d| d.model.clone()).collect();
        models.sort();
        models.dedup();

        let kind_chip = |label: &'static str,
                         kind: SearchContentKind,
                         active: bool,
                         theme: &M0Theme,
                         cx: &mut Context<Self>| {
            div()
                .rounded_md()
                .px_2()
                .py_1()
                .text_xs()
                .bg(if active {
                    theme.accent
                } else {
                    theme.surface_hover
                })
                .text_color(if active {
                    theme.accent_text
                } else {
                    theme.text_primary
                })
                .cursor_pointer()
                .child(label)
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.toggle_search_kind_filter(kind, cx);
                    }),
                )
        };

        let kinds_empty = filters.kinds.is_empty();
        let mut filter_row = div().flex().flex_row().flex_wrap().gap_2().items_center();
        filter_row = filter_row
            .child(div().text_xs().text_color(theme.text_muted).child("Type:"))
            .child(kind_chip(
                "Threads",
                SearchContentKind::Thread,
                kinds_empty || filters.kinds.contains(&SearchContentKind::Thread),
                theme,
                cx,
            ))
            .child(kind_chip(
                "Artifacts",
                SearchContentKind::Artifact,
                kinds_empty || filters.kinds.contains(&SearchContentKind::Artifact),
                theme,
                cx,
            ))
            .child(kind_chip(
                "Memories",
                SearchContentKind::Memory,
                kinds_empty || filters.kinds.contains(&SearchContentKind::Memory),
                theme,
                cx,
            ));

        let mut provider_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .items_center()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child("Provider:"),
            );
        let provider_all_active = filters.provider.is_none();
        provider_row = provider_row.child(
            div()
                .rounded_md()
                .px_2()
                .py_1()
                .text_xs()
                .bg(if provider_all_active {
                    theme.accent
                } else {
                    theme.surface_hover
                })
                .text_color(if provider_all_active {
                    theme.accent_text
                } else {
                    theme.text_primary
                })
                .cursor_pointer()
                .child("Any")
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.set_search_provider_filter(None, cx);
                    }),
                ),
        );
        for provider in &providers {
            let active = filters.provider.as_deref() == Some(provider.as_str());
            let provider = provider.clone();
            provider_row = provider_row.child(
                div()
                    .rounded_md()
                    .px_2()
                    .py_1()
                    .text_xs()
                    .bg(if active {
                        theme.accent
                    } else {
                        theme.surface_hover
                    })
                    .text_color(if active {
                        theme.accent_text
                    } else {
                        theme.text_primary
                    })
                    .cursor_pointer()
                    .child(provider.clone())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.set_search_provider_filter(Some(provider.clone()), cx);
                        }),
                    ),
            );
        }

        let mut model_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .items_center()
            .child(div().text_xs().text_color(theme.text_muted).child("Model:"));
        let model_all_active = filters.model.is_none();
        model_row = model_row.child(
            div()
                .rounded_md()
                .px_2()
                .py_1()
                .text_xs()
                .bg(if model_all_active {
                    theme.accent
                } else {
                    theme.surface_hover
                })
                .text_color(if model_all_active {
                    theme.accent_text
                } else {
                    theme.text_primary
                })
                .cursor_pointer()
                .child("Any")
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.set_search_model_filter(None, cx);
                    }),
                ),
        );
        for model in &models {
            let active = filters.model.as_deref() == Some(model.as_str());
            let model = model.clone();
            model_row = model_row.child(
                div()
                    .rounded_md()
                    .px_2()
                    .py_1()
                    .text_xs()
                    .bg(if active {
                        theme.accent
                    } else {
                        theme.surface_hover
                    })
                    .text_color(if active {
                        theme.accent_text
                    } else {
                        theme.text_primary
                    })
                    .cursor_pointer()
                    .child(model.clone())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.set_search_model_filter(Some(model.clone()), cx);
                        }),
                    ),
            );
        }

        let mut date_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .items_center()
            .child(div().text_xs().text_color(theme.text_muted).child("Date:"));
        for (label, preset) in [
            ("Any time", SearchDatePreset::Any),
            ("7 days", SearchDatePreset::Last7Days),
            ("30 days", SearchDatePreset::Last30Days),
            ("1 year", SearchDatePreset::LastYear),
        ] {
            let active = date_preset == preset;
            date_row = date_row.child(
                div()
                    .rounded_md()
                    .px_2()
                    .py_1()
                    .text_xs()
                    .bg(if active {
                        theme.accent
                    } else {
                        theme.surface_hover
                    })
                    .text_color(if active {
                        theme.accent_text
                    } else {
                        theme.text_primary
                    })
                    .cursor_pointer()
                    .child(label)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.set_search_date_preset(preset, cx);
                        }),
                    ),
            );
        }

        let grouped = group_hits_by_kind(&hits);
        let mut results = div()
            .id("search-results-scroll")
            .flex()
            .flex_col()
            .gap_3()
            .flex_1()
            .min_h(px(0.0))
            .overflow_y_scroll()
            .w_full();

        if hits.is_empty() {
            let empty_label = if self.search_query_editor.text().trim().is_empty() {
                "Type to search threads, artifacts, and memories"
            } else {
                "No matches"
            };
            results = results.child(
                div()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child(empty_label),
            );
        } else {
            let mut flat_index = 0usize;
            for (kind, group) in grouped {
                results = results.child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight(600.))
                        .text_color(theme.text_muted)
                        .child(kind.label()),
                );
                for hit in group {
                    let is_selected = flat_index == selected;
                    let hit_clone = (*hit).clone();
                    let title = hit.document.title.clone();
                    let snippet = hit.snippet.clone();
                    let kind_label = hit.document.kind.label();
                    results = results.child(
                        div()
                            .rounded_md()
                            .px_3()
                            .py_2()
                            .bg(if is_selected {
                                theme.surface_selected
                            } else {
                                theme.surface_muted
                            })
                            .border_1()
                            .border_color(if is_selected {
                                theme.accent
                            } else {
                                theme.border_subtle
                            })
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.surface_hover))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .gap_2()
                                    .child(
                                        div()
                                            .font_weight(FontWeight(600.))
                                            .text_color(theme.text_primary)
                                            .truncate()
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(kind_label),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .truncate()
                                    .child(snippet),
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    this.activate_search_hit(&hit_clone, window, cx);
                                }),
                            ),
                    );
                    flat_index += 1;
                }
            }
        }

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .right(px(0.))
            .bottom(px(0.))
            .flex()
            .items_start()
            .justify_center()
            .pt(px(48.0))
            .bg(hsla(
                theme.app_background.h,
                theme.app_background.s,
                theme.app_background.l,
                0.72,
            ))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.search_panel.close();
                    cx.notify();
                }),
            )
            .child(
                div()
                    .w(px(560.0))
                    .max_h(px(640.0))
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border_strong)
                    .bg(theme.surface_muted)
                    .p_5()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .shadow(elevation_style(Elevation::High, theme.color_scheme).box_shadows())
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight(600.))
                                    .text_color(theme.text_primary)
                                    .child("Search"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .child("Close (Esc)")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.search_panel.close();
                                            cx.notify();
                                        }),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .rounded_md()
                            .border_1()
                            .border_color(theme.accent)
                            .bg(theme.composer_background)
                            .px_3()
                            .py_2()
                            .track_focus(&self.search_focus)
                            .on_key_down(cx.listener(Self::on_search_key_down))
                            .child(self.search_query_editor.render_text(
                                "Search threads, artifacts, memories…",
                                theme.text_primary,
                                theme.text_muted,
                                theme.accent,
                            )),
                    )
                    .child(filter_row)
                    .child(provider_row)
                    .child(model_row)
                    .child(date_row)
                    .child(results),
            )
    }

    fn render_model_picker(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.model_picker.selected();
        let scheme = theme.color_scheme;
        let grouped = group_entries_by_provider(&self.model_picker_entries);
        let mut flat_index = 0usize;
        let mut sections = div()
            .id("model-picker-scroll")
            .flex()
            .flex_col()
            .gap_3()
            .w_full()
            .max_h(px(360.))
            .overflow_y_scroll();

        if grouped.is_empty() {
            sections = sections.child(
                div()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("No models available. Pull an Ollama model or configure OpenAI."),
            );
        }

        for (provider, entries) in grouped {
            let mut rows = div().flex().flex_col().gap_1().w_full();
            rows = rows.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight(600.))
                    .text_color(theme.text_muted)
                    .child(provider.label()),
            );
            for entry in entries {
                let index = flat_index;
                flat_index += 1;
                let highlighted = index == selected;
                let tone = picker_row_tone(highlighted, entry.is_active);
                let colors = picker_row_colors(scheme, tone);
                let caps = format_capability_summary(&entry.capabilities);
                let label = if entry.is_active {
                    format!("{} (active)", entry.model_name)
                } else {
                    entry.model_name.clone()
                };
                rows = rows.child(
                    div()
                        .rounded_md()
                        .px_3()
                        .py_2()
                        .bg(colors.background)
                        .cursor_pointer()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(div().text_sm().text_color(colors.text).child(label))
                                .child(div().text_xs().text_color(colors.text_muted).child(
                                    if caps.is_empty() {
                                        entry.provider_label.to_string()
                                    } else {
                                        format!("{} · {}", entry.provider_label, caps)
                                    },
                                )),
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.model_picker.close();
                                this.apply_model_picker_action(
                                    ModelPickerAction::Select { index },
                                    cx,
                                );
                            }),
                        ),
                );
            }
            sections = sections.child(rows);
        }

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .right(px(0.))
            .bottom(px(0.))
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0., 0., 0., 0.45))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.model_picker.close();
                    cx.notify();
                }),
            )
            .child(
                div()
                    .w(px(420.))
                    .max_h(px(480.))
                    .rounded_xl()
                    .bg(theme.surface_muted)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight(600.))
                                    .child("Select model"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("↑↓ Enter · Esc"),
                            ),
                    )
                    .child(sections),
            )
    }

    fn render_shortcut_help(&self, theme: &M0Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let mut rows = div().flex().flex_col().gap_2().w_full();
        for hint in shortcut_catalog() {
            rows = rows.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight(600.))
                            .text_color(theme.accent)
                            .child(hint.keys),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(hint.action),
                    ),
            );
        }

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .right(px(0.))
            .bottom(px(0.))
            .flex()
            .items_center()
            .justify_center()
            .bg(theme.app_background)
            .child(
                div()
                    .id("shortcut-help-scroll")
                    .w(px(480.0))
                    .max_h(px(520.0))
                    .overflow_y_scroll()
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border_strong)
                    .bg(theme.surface_muted)
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .shadow(elevation_style(Elevation::High, theme.color_scheme).box_shadows())
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight(600.))
                                    .child("Keyboard shortcuts"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .child("Close (Esc)")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            let input = KeyInput {
                                                key: "escape",
                                                control: false,
                                                shift: false,
                                                alt: false,
                                            };
                                            let count = this.thread_count();
                                            let _ = this.keyboard_nav.handle_key(input, count);
                                            cx.notify();
                                        }),
                                    ),
                            ),
                    )
                    .child(rows),
            )
    }
}

impl Render for RoninWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let streaming_active = self.pump_streaming();
        let _ = self.pump_instance_ipc(window, cx);
        if self.needs_initial_focus {
            self.needs_initial_focus = false;
            window.focus(&self.composer_focus);
        }
        let theme_preference = self
            .shell
            .session()
            .load_config()
            .map(|config| config.theme)
            .unwrap_or(ThemePreference::System);
        let theme = resolve_shell_theme(theme_preference, window.appearance());
        let composer_focused = self.composer_focus.is_focused(window);
        let sidebar_focused = self.sidebar_focus.is_focused(window)
            || self.keyboard_nav.focus() == FocusRegion::Sidebar;
        let messages_focused = self.messages_focus.is_focused(window)
            || self.keyboard_nav.focus() == FocusRegion::Messages;

        // Keep nav state aligned when GPUI focus lands on a region
        if composer_focused && self.keyboard_nav.focus() != FocusRegion::Composer {
            self.keyboard_nav
                .set_focus(FocusRegion::Composer, self.shell.state().threads.len());
        } else if self.sidebar_focus.is_focused(window)
            && self.keyboard_nav.focus() != FocusRegion::Sidebar
        {
            self.keyboard_nav
                .set_focus(FocusRegion::Sidebar, self.shell.state().threads.len());
        } else if self.messages_focus.is_focused(window)
            && self.keyboard_nav.focus() != FocusRegion::Messages
        {
            self.keyboard_nav
                .set_focus(FocusRegion::Messages, self.shell.state().threads.len());
        }

        // Blink cursor: smooth duty cycle from streaming motion tokens
        let motion = streaming_motion();
        let blink_elapsed = self.blink_start.elapsed().as_millis() as u64;
        self.composer.cursor_visible = cursor_visible_at(blink_elapsed, &motion);

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
        let sidebar_w = if self.sidebar_collapsed {
            SIDEBAR_COLLAPSED_RAIL
        } else {
            self.sidebar_width
        };
        let outer_pad = rem * 6.0 * 2.0; // p_6 left+right
        let inner_pad = rem * 4.0 * 2.0; // p_4 left+right
        let border_w = 4.0; // border_2
        let send_btn_w = 80.0;
        let gap_w = rem * 0.5; // gap_2
        let text_w = 1120.0 - sidebar_w - outer_pad - inner_pad - border_w - send_btn_w - gap_w;
        self.composer.set_container_width(text_w.max(100.0));

        let sidebar = self.render_sidebar(&theme, sidebar_focused, cx);
        let title = Self::current_thread_title(self.shell.state())
            .map(|t| t.to_string())
            .unwrap_or_else(|| "New Chat".to_string());
        let messages = self.render_messages(&theme, messages_focused, cx);
        let composer = self.render_composer(&theme, composer_focused, cx);

        let mut ui = div()
            .id("ronin-root")
            .relative()
            .size_full()
            .flex()
            .bg(theme.app_background)
            .text_color(theme.text_primary)
            .font_family("Inter")
            .on_key_down(cx.listener(Self::on_global_key_down))
            .on_mouse_move(cx.listener(Self::on_sidebar_resize_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_sidebar_resize_up))
            .can_drop(|value, _, _| value.is::<ExternalPaths>())
            .on_drag_move(cx.listener(Self::on_external_paths_drag_move))
            .on_drop(cx.listener(Self::on_external_paths_drop))
            .child(sidebar)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .can_drop(|value, _, _| value.is::<ExternalPaths>())
                    .on_drop(cx.listener(Self::on_external_paths_drop))
                    .child(
                        div()
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .px_6()
                            .py_4()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .child(div().truncate().child(title))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_3()
                                    .child({
                                        let model_label = self
                                            .active_model_name()
                                            .unwrap_or("Select model")
                                            .to_string();
                                        div()
                                            .text_xs()
                                            .rounded_md()
                                            .px_2()
                                            .py_1()
                                            .bg(theme.surface_muted)
                                            .text_color(theme.text_primary)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.surface_hover))
                                            .child(format!("Model: {model_label}"))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    this.open_model_picker();
                                                    cx.notify();
                                                }),
                                            )
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .cursor_pointer()
                                            .hover(|s| s.text_color(theme.accent))
                                            .child("Shortcuts (Ctrl+/)")
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    let input = KeyInput {
                                                        key: "/",
                                                        control: true,
                                                        shift: false,
                                                        alt: false,
                                                    };
                                                    let count = this.thread_count();
                                                    let (_, action) =
                                                        this.keyboard_nav.handle_key(input, count);
                                                    if matches!(action, NavAction::ToggleHelp) {
                                                        cx.notify();
                                                    }
                                                }),
                                            ),
                                    ),
                            ),
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

        if self.keyboard_nav.help_visible() {
            ui = ui.child(self.render_shortcut_help(&theme, cx));
        }

        if self.search_panel.is_open() {
            ui = ui.child(self.render_search_panel(&theme, cx));
        }

        if self.model_picker.is_open() {
            self.rebuild_model_picker_entries();
            ui = ui.child(self.render_model_picker(&theme, cx));
        }

        if drop_overlay_should_show(self.file_drop_active) {
            ui = ui.child(self.render_drop_overlay(&theme, cx));
        }

        let mut needs_frame = streaming_active
            || composer_focused
            || self.sidebar_drag.is_some()
            || self.file_drop_active
            || self.instance_primary.is_some();
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
