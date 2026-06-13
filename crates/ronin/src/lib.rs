#![deny(missing_docs)]

//! Native Ronin launcher support.

/// Markdown parsing and AST for GPUI rendering.
pub mod markdown;

use std::path::PathBuf;

use ronin_core::RoninPaths;

/// User-requested launch behavior parsed from CLI arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchIntent {
    /// Open Ronin with the default persisted shell state.
    OpenPersisted,
    /// Open Ronin with a newly created empty chat selected.
    NewThread,
    /// Open Ronin with Ollama selected as the local provider.
    OpenWithOllama,
}

/// Errors returned by Ronin launcher setup.
#[derive(Debug, thiserror::Error)]
pub enum LauncherError {
    /// No home directory was available for fallback XDG paths.
    #[error("HOME is required when XDG directories are not set")]
    MissingHome,

    /// CLI argument is not supported by the M0 launcher.
    #[error("unsupported launch flag '{flag}'. supported flags: --new, --provider ollama")]
    UnsupportedFlag {
        /// Unsupported flag supplied by the user.
        flag: String,
    },
}

/// Parses CLI launch intent from arguments after the binary name.
pub fn parse_launch_intent(
    args: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<LaunchIntent, LauncherError> {
    let mut intent = LaunchIntent::OpenPersisted;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let arg = arg.as_ref();
        match arg {
            "--new" => intent = LaunchIntent::NewThread,
            "--provider" => match args.next().as_ref().map(AsRef::as_ref) {
                Some("ollama") => intent = LaunchIntent::OpenWithOllama,
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
