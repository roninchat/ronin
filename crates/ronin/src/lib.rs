#![deny(missing_docs)]

//! Native Ronin launcher support.

use std::path::PathBuf;

use ronin_core::RoninPaths;

/// Errors returned by Ronin launcher setup.
#[derive(Debug, thiserror::Error)]
pub enum LauncherError {
    /// No home directory was available for fallback XDG paths.
    #[error("HOME is required when XDG directories are not set")]
    MissingHome,
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
