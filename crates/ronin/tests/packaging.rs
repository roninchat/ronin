//! Packaging: .desktop validation, install path planning, icon inventory.

use std::path::{Path, PathBuf};

use ronin::packaging::{
    desktop_file_contents, icon_asset_paths, parse_desktop_required_fields, plan_install,
    plan_uninstall, required_icon_sizes, validate_desktop_file, InstallMode, APP_ICON_NAME,
    DESKTOP_CATEGORIES, DESKTOP_COMMENT, DESKTOP_NAME,
};

fn repo_packaging_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packaging")
}

#[test]
fn desktop_file_contents_should_include_freedesktop_required_fields() {
    let content = desktop_file_contents("/usr/local/bin/ronin");
    let fields = parse_desktop_required_fields(&content).expect("parse");

    assert_eq!(fields.name, DESKTOP_NAME);
    assert_eq!(fields.exec, "/usr/local/bin/ronin");
    assert_eq!(fields.icon, APP_ICON_NAME);
    assert_eq!(fields.comment, DESKTOP_COMMENT);
    assert!(fields.categories.iter().any(|c| c == "Utility"));
    assert!(fields.categories.iter().any(|c| DESKTOP_CATEGORIES.contains(&c.as_str())));
    assert_eq!(fields.r#type, "Application");
    assert!(!fields.terminal);

    validate_desktop_file(&content).expect("valid");
}

#[test]
fn packaged_desktop_file_on_disk_should_pass_validation() {
    let path = repo_packaging_dir().join("ronin.desktop");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    validate_desktop_file(&content).expect("packaged desktop file");
    let fields = parse_desktop_required_fields(&content).expect("fields");
    assert_eq!(fields.name, DESKTOP_NAME);
    assert_eq!(fields.icon, APP_ICON_NAME);
    assert!(fields.exec.contains("ronin"));
}

#[test]
fn install_plan_user_prefix_should_use_xdg_data_home_layout() {
    let prefix = Path::new("/home/user/.local");
    let plan = plan_install(prefix, InstallMode::User, Path::new("target/release/ronin"));

    assert_eq!(plan.binary_dest, PathBuf::from("/home/user/.local/bin/ronin"));
    assert_eq!(
        plan.desktop_dest,
        PathBuf::from("/home/user/.local/share/applications/ronin.desktop")
    );
    assert_eq!(
        plan.icon_svg_dest,
        PathBuf::from("/home/user/.local/share/icons/hicolor/scalable/apps/ronin.svg")
    );
    assert_eq!(
        plan.icon_png_dests.get(&48).map(PathBuf::as_path),
        Some(Path::new(
            "/home/user/.local/share/icons/hicolor/48x48/apps/ronin.png"
        ))
    );
    assert_eq!(
        plan.icon_png_dests.get(&256).map(PathBuf::as_path),
        Some(Path::new(
            "/home/user/.local/share/icons/hicolor/256x256/apps/ronin.png"
        ))
    );
}

#[test]
fn install_plan_system_prefix_should_use_usr_local_layout() {
    let plan = plan_install(
        Path::new("/usr/local"),
        InstallMode::System,
        Path::new("./ronin"),
    );
    assert_eq!(plan.binary_dest, PathBuf::from("/usr/local/bin/ronin"));
    assert_eq!(
        plan.desktop_dest,
        PathBuf::from("/usr/local/share/applications/ronin.desktop")
    );
}

#[test]
fn uninstall_plan_should_list_all_installed_paths() {
    let plan = plan_install(
        Path::new("/usr/local"),
        InstallMode::System,
        Path::new("ronin"),
    );
    let remove = plan_uninstall(&plan);
    assert!(remove.contains(&plan.binary_dest));
    assert!(remove.contains(&plan.desktop_dest));
    assert!(remove.contains(&plan.icon_svg_dest));
    for size in required_icon_sizes() {
        assert!(remove.contains(plan.icon_png_dests.get(&size).unwrap()));
    }
}

#[test]
fn icon_assets_should_exist_for_required_sizes_and_svg() {
    let packaging = repo_packaging_dir();
    let assets = icon_asset_paths(&packaging);
    assert!(assets.svg.exists(), "missing {}", assets.svg.display());
    for size in required_icon_sizes() {
        let path = assets.pngs.get(&size).expect("size mapped");
        assert!(path.exists(), "missing icon {}", path.display());
        // Validate pixel dimensions via image crate metadata when available.
        let dyn_img = image::image_dimensions(path).expect("read dimensions");
        assert_eq!(dyn_img, (*size, *size), "icon {size} wrong size");
    }
}

#[test]
fn install_plan_dry_run_should_list_copy_operations_without_side_effects() {
    let plan = plan_install(
        Path::new("/tmp/ronin-prefix"),
        InstallMode::User,
        Path::new("target/release/ronin"),
    );
    let ops = plan.dry_run_operations(&repo_packaging_dir());
    assert!(ops.iter().any(|op| op.contains("bin/ronin")));
    assert!(ops.iter().any(|op| op.contains("ronin.desktop")));
    assert!(ops.iter().any(|op| op.contains("ronin.svg")));
    assert!(!Path::new("/tmp/ronin-prefix/bin/ronin").exists());
}
