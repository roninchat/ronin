//! Linux packaging helpers: .desktop entries, install path planning, icon inventory.
//!
//! These seams are testable without performing a real system install.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Application display name for launchers.
pub const DESKTOP_NAME: &str = "Ronin";

/// Freedesktop icon name (without extension / size).
pub const APP_ICON_NAME: &str = "ronin";

/// Short launcher description.
pub const DESKTOP_COMMENT: &str = "Native intelligence for Linux — local-first AI workbench";

/// Desktop entry categories (freedesktop).
pub const DESKTOP_CATEGORIES: &[&str] = &["Utility", "Development", "Office"];

/// PNG icon sizes shipped with packaging.
pub fn required_icon_sizes() -> &'static [u32] {
    &[48, 128, 256]
}

/// User vs system install layout under a prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMode {
    /// `$prefix/bin`, `$prefix/share/...` (e.g. `~/.local`).
    User,
    /// Same layout under `/usr/local` (or similar).
    System,
}

/// Parsed required fields from a `.desktop` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopRequiredFields {
    /// `Type=`
    pub r#type: String,
    /// `Name=`
    pub name: String,
    /// `Exec=`
    pub exec: String,
    /// `Icon=`
    pub icon: String,
    /// `Comment=`
    pub comment: String,
    /// `Categories=` split on `;` (empty trailing ignored).
    pub categories: Vec<String>,
    /// `Terminal=`
    pub terminal: bool,
}

/// Planned install destinations for binary, desktop entry, and icons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    /// Install prefix (`~/.local`, `/usr/local`, …).
    pub prefix: PathBuf,
    /// Source binary path used at install time.
    pub binary_source: PathBuf,
    /// Destination for the `ronin` binary.
    pub binary_dest: PathBuf,
    /// Destination `.desktop` path.
    pub desktop_dest: PathBuf,
    /// Destination scalable SVG icon.
    pub icon_svg_dest: PathBuf,
    /// Destination PNG icons keyed by size.
    pub icon_png_dests: BTreeMap<u32, PathBuf>,
}

/// Source icon paths under the packaging directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconAssetPaths {
    /// Scalable SVG.
    pub svg: PathBuf,
    /// Raster icons by size.
    pub pngs: BTreeMap<u32, PathBuf>,
}

impl InstallPlan {
    /// Human-readable dry-run copy operations (no filesystem writes).
    pub fn dry_run_operations(&self, packaging_dir: &Path) -> Vec<String> {
        let mut ops = Vec::new();
        ops.push(format!(
            "install -D {} {}",
            self.binary_source.display(),
            self.binary_dest.display()
        ));
        ops.push(format!(
            "install -D {}/ronin.desktop {}",
            packaging_dir.display(),
            self.desktop_dest.display()
        ));
        let assets = icon_asset_paths(packaging_dir);
        ops.push(format!(
            "install -D {} {}",
            assets.svg.display(),
            self.icon_svg_dest.display()
        ));
        for size in required_icon_sizes() {
            if let (Some(src), Some(dest)) =
                (assets.pngs.get(size), self.icon_png_dests.get(size))
            {
                ops.push(format!("install -D {} {}", src.display(), dest.display()));
            }
        }
        ops
    }
}

/// Builds the canonical Ronin `.desktop` file body for `exec_path`.
pub fn desktop_file_contents(exec_path: &str) -> String {
    let categories = DESKTOP_CATEGORIES.join(";");
    format!(
        "\
[Desktop Entry]
Type=Application
Version=1.0
Name={DESKTOP_NAME}
GenericName=AI Workbench
Comment={DESKTOP_COMMENT}
Exec={exec_path}
Icon={APP_ICON_NAME}
Terminal=false
Categories={categories};
Keywords=AI;chat;LLM;ollama;assistant;
StartupNotify=true
"
    )
}

/// Parses required freedesktop fields from desktop file text.
pub fn parse_desktop_required_fields(
    content: &str,
) -> Result<DesktopRequiredFields, String> {
    let mut r#type = None;
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut comment = None;
    let mut categories = None;
    let mut terminal = None;

    let mut in_desktop_entry = false;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_desktop_entry {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key {
            "Type" => r#type = Some(value.to_string()),
            "Name" => name = Some(value.to_string()),
            "Exec" => exec = Some(value.to_string()),
            "Icon" => icon = Some(value.to_string()),
            "Comment" => comment = Some(value.to_string()),
            "Categories" => {
                categories = Some(
                    value
                        .split(';')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect(),
                );
            }
            "Terminal" => {
                terminal = Some(matches!(value.to_ascii_lowercase().as_str(), "true" | "1"));
            }
            _ => {}
        }
    }

    Ok(DesktopRequiredFields {
        r#type: r#type.ok_or("missing Type")?,
        name: name.ok_or("missing Name")?,
        exec: exec.ok_or("missing Exec")?,
        icon: icon.ok_or("missing Icon")?,
        comment: comment.ok_or("missing Comment")?,
        categories: categories.ok_or("missing Categories")?,
        terminal: terminal.unwrap_or(false),
    })
}

/// Validates that a desktop file has the fields Ronin packaging requires.
pub fn validate_desktop_file(content: &str) -> Result<(), String> {
    if !content.contains("[Desktop Entry]") {
        return Err("missing [Desktop Entry] group".into());
    }
    let fields = parse_desktop_required_fields(content)?;
    if fields.r#type != "Application" {
        return Err(format!("Type must be Application, got {}", fields.r#type));
    }
    if fields.name != DESKTOP_NAME {
        return Err(format!("Name must be {DESKTOP_NAME}"));
    }
    if fields.icon != APP_ICON_NAME {
        return Err(format!("Icon must be {APP_ICON_NAME}"));
    }
    if fields.exec.trim().is_empty() {
        return Err("Exec must not be empty".into());
    }
    if fields.comment.trim().is_empty() {
        return Err("Comment must not be empty".into());
    }
    if fields.categories.is_empty() {
        return Err("Categories must not be empty".into());
    }
    for required in DESKTOP_CATEGORIES {
        if !fields.categories.iter().any(|c| c == required) {
            return Err(format!("Categories must include {required}"));
        }
    }
    if fields.terminal {
        return Err("Terminal must be false for a GUI app".into());
    }
    Ok(())
}

/// Plans install destinations under `prefix`.
pub fn plan_install(prefix: &Path, _mode: InstallMode, binary_source: &Path) -> InstallPlan {
    let share = prefix.join("share");
    let mut icon_png_dests = BTreeMap::new();
    for &size in required_icon_sizes() {
        icon_png_dests.insert(
            size,
            share
                .join("icons/hicolor")
                .join(format!("{size}x{size}"))
                .join("apps")
                .join(format!("{APP_ICON_NAME}.png")),
        );
    }
    InstallPlan {
        prefix: prefix.to_path_buf(),
        binary_source: binary_source.to_path_buf(),
        binary_dest: prefix.join("bin").join("ronin"),
        desktop_dest: share.join("applications").join("ronin.desktop"),
        icon_svg_dest: share
            .join("icons/hicolor/scalable/apps")
            .join(format!("{APP_ICON_NAME}.svg")),
        icon_png_dests,
    }
}

/// Lists every path an uninstall should remove for `plan`.
pub fn plan_uninstall(plan: &InstallPlan) -> Vec<PathBuf> {
    let mut paths = vec![
        plan.binary_dest.clone(),
        plan.desktop_dest.clone(),
        plan.icon_svg_dest.clone(),
    ];
    paths.extend(plan.icon_png_dests.values().cloned());
    paths
}

/// Resolves packaging-dir icon asset paths.
pub fn icon_asset_paths(packaging_dir: &Path) -> IconAssetPaths {
    let icons = packaging_dir.join("icons");
    let mut pngs = BTreeMap::new();
    for &size in required_icon_sizes() {
        pngs.insert(
            size,
            icons
                .join("hicolor")
                .join(format!("{size}x{size}"))
                .join("apps")
                .join(format!("{APP_ICON_NAME}.png")),
        );
    }
    IconAssetPaths {
        svg: icons
            .join("hicolor/scalable/apps")
            .join(format!("{APP_ICON_NAME}.svg")),
        pngs,
    }
}
