#![deny(missing_docs)]

//! Native Ronin launcher support.

/// Markdown parsing and AST for GPUI rendering.
pub mod markdown;

/// Markdown AST rendering helpers for the GPUI shell.
pub mod markdown_view;

/// Composer completion logic for context commands, memories, and file paths.
pub mod completions;

/// Filterable `@` attachment and `/` action picker presentation.
pub mod composer_pickers;

/// Drag-and-drop path ingest and clipboard image paste → attachments.
pub mod composer_ingest;

/// Composer context/token size estimation and indicator presentation.
pub mod context_indicator;

/// Folder attach: bounded listing and file selection.
pub mod folder_attach;

/// Attachment size warnings before send.
pub mod attachment_size;

/// Presentation model for the dedicated Artifacts panel.
pub mod artifacts_panel;

/// Attachment preview models for composer and sent messages.
pub mod attachment_preview;

/// Wayland portal screenshot capture with CLI fallbacks.
pub mod screenshot_capture;

/// Opt-in clipboard watch host reader (arboard) for confirm-to-attach proposals.
pub mod clipboard_watch;

/// Syntax highlighting for fenced Markdown code blocks.
pub mod syntax_highlight;

/// Composer text editor state and input handling.
pub mod composer;

/// Color themes and semantic UI tokens for the native shell.
pub mod theme;

/// Elevation, empty/error presentations, and streaming motion tokens.
pub mod visual_polish;

/// Keyboard-first navigation state machine and shortcut catalog.
pub mod keyboard_nav;

/// Inline thread rename presentation and title-generation status copy.
pub mod thread_titles;

/// Post-send message edit drafts and branch navigation labels.
pub mod message_branches;

/// Global search across threads, artifacts, and memories.
pub mod global_search;

/// Memory management: enable/disable, profile group, context indicator.
pub mod memory_management;

/// Model picker: provider grouping, capabilities, keyboard navigation.
pub mod model_picker;

/// Provider settings: Test Connection action and result presentation.
pub mod provider_settings;

/// Quick mode compact overlay state and action labels.
pub mod quick_mode;

/// Single-instance lock and Unix-socket IPC for CLI intent routing.
pub mod instance;

/// Linux packaging: .desktop validation, install path planning, icon inventory.
pub mod packaging;

pub use instance::{
    acquire_instance, instance_runtime_dir, plan_incoming_launch, AppliedIntent, IncomingLaunch,
    InstanceAcquire, InstanceError, InstancePrimary,
};

use std::path::PathBuf;

use ronin_core::RoninPaths;

/// User-requested launch behavior parsed from CLI arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchIntent {
    /// Open Ronin with the default persisted shell state.
    OpenPersisted {
        /// File paths to attach when the app opens.
        attach_paths: Vec<PathBuf>,
    },
    /// Open Ronin with a newly created empty chat selected.
    NewThread {
        /// File paths to attach when the app opens.
        attach_paths: Vec<PathBuf>,
    },
    /// Open Ronin with Ollama selected as the local provider.
    OpenWithOllama {
        /// File paths to attach when the app opens.
        attach_paths: Vec<PathBuf>,
    },
    /// Quick-launch mode: open the compact overlay for a one-shot question.
    Quick {
        /// File paths to attach when the app opens.
        attach_paths: Vec<PathBuf>,
    },
}

/// Errors returned by Ronin launcher setup.
#[derive(Debug, thiserror::Error)]
pub enum LauncherError {
    /// No home directory was available for fallback XDG paths.
    #[error("HOME is required when XDG directories are not set")]
    MissingHome,

    /// CLI argument is not supported by the launcher.
    #[error("unsupported launch flag '{flag}'. supported flags: --new, --quick, --provider ollama, --attach <path>")]
    UnsupportedFlag {
        /// Unsupported flag supplied by the user.
        flag: String,
    },
}

/// Parses CLI launch intent from arguments after the binary name.
pub fn parse_launch_intent(
    args: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<LaunchIntent, LauncherError> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LaunchMode {
        Persisted,
        NewThread,
        Ollama,
        Quick,
    }

    let mut mode = LaunchMode::Persisted;
    let mut attach_paths = Vec::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let arg = arg.as_ref();
        match arg {
            "--new" => mode = LaunchMode::NewThread,
            "--quick" => mode = LaunchMode::Quick,
            "--attach" => match args.next().as_ref().map(AsRef::as_ref) {
                Some(path) => attach_paths.push(PathBuf::from(path)),
                None => {
                    return Err(LauncherError::UnsupportedFlag {
                        flag: "--attach".to_string(),
                    });
                }
            },
            "--provider" => match args.next().as_ref().map(AsRef::as_ref) {
                Some("ollama") => mode = LaunchMode::Ollama,
                Some(provider) => {
                    return Err(LauncherError::UnsupportedFlag {
                        flag: format!("--provider {provider}"),
                    });
                }
                None => {
                    return Err(LauncherError::UnsupportedFlag {
                        flag: "--provider".to_string(),
                    });
                }
            },
            flag => {
                return Err(LauncherError::UnsupportedFlag {
                    flag: flag.to_string(),
                });
            }
        }
    }

    let intent = match mode {
        LaunchMode::Persisted => LaunchIntent::OpenPersisted { attach_paths },
        LaunchMode::NewThread => LaunchIntent::NewThread { attach_paths },
        LaunchMode::Ollama => LaunchIntent::OpenWithOllama { attach_paths },
        LaunchMode::Quick => LaunchIntent::Quick { attach_paths },
    };

    tracing::info!(intent = ?intent, "ronin launch intent parsed");
    Ok(intent)
}

/// Builds Ronin config/data paths from XDG environment values.
pub fn ronin_paths_from_env(
    xdg_config_home: Option<&str>,
    xdg_data_home: Option<&str>,
    home: Option<&str>,
) -> Result<RoninPaths, LauncherError> {
    let config_base = xdg_config_home
        .map(PathBuf::from)
        .or_else(|| home.map(|home| PathBuf::from(home).join(".config")))
        .ok_or(LauncherError::MissingHome)?;
    let data_base = xdg_data_home
        .map(PathBuf::from)
        .or_else(|| home.map(|home| PathBuf::from(home).join(".local/share")))
        .ok_or(LauncherError::MissingHome)?;

    Ok(RoninPaths {
        config_dir: config_base.join("ronin"),
        data_dir: data_base.join("ronin"),
    })
}

/// Builds Ronin paths from the current process environment.
pub fn ronin_paths() -> Result<RoninPaths, LauncherError> {
    let xdg_config_home = std::env::var("XDG_CONFIG_HOME").ok();
    let xdg_data_home = std::env::var("XDG_DATA_HOME").ok();
    let home = std::env::var("HOME").ok();

    ronin_paths_from_env(
        xdg_config_home.as_deref(),
        xdg_data_home.as_deref(),
        home.as_deref(),
    )
}
