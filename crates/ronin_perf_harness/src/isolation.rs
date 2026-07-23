//! Harness Isolation helpers (never touch user XDG by default).

use std::path::{Path, PathBuf};

use ronin_core::RoninPaths;

use crate::error::HarnessError;

/// Creates isolated RoninPaths under `root` (config + data). Does not use user XDG.
pub fn isolated_ronin_paths(root: &Path) -> Result<RoninPaths, HarnessError> {
    let config_dir = root.join("config");
    let data_dir = root.join("data");
    std::fs::create_dir_all(&config_dir)?;
    std::fs::create_dir_all(&data_dir)?;
    assert_not_user_xdg(&config_dir)?;
    assert_not_user_xdg(&data_dir)?;
    Ok(RoninPaths {
        config_dir,
        data_dir,
    })
}

fn assert_not_user_xdg(path: &Path) -> Result<(), HarnessError> {
    let forbidden = user_xdg_roots();
    for root in forbidden {
        if path.starts_with(&root) {
            return Err(HarnessError::Isolation(format!(
                "refusing path under user XDG {}: {}",
                root.display(),
                path.display()
            )));
        }
    }
    Ok(())
}

fn user_xdg_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        roots.push(home.join(".config/ronin"));
        roots.push(home.join(".local/share/ronin"));
    }
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        roots.push(PathBuf::from(xdg_config).join("ronin"));
    }
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        roots.push(PathBuf::from(xdg_data).join("ronin"));
    }
    roots
}
