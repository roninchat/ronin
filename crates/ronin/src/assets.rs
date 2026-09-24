//! Bundled Lucide icons and Inter fonts for the GPUI shell.
//!
//! Register with `Application::new().with_assets(RoninAssets)` and load fonts via
//! `cx.text_system().add_fonts(bundled_font_bytes())`.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

macro_rules! icon_asset {
    ($file:literal) => {
        (
            concat!("icons/", $file),
            include_bytes!(concat!("../../../assets/icons/", $file)) as &[u8],
        )
    };
}

const ICON_ASSETS: &[(&str, &[u8])] = &[
    icon_asset!("plus.svg"),
    icon_asset!("search.svg"),
    icon_asset!("menu.svg"),
    icon_asset!("ellipsis.svg"),
    icon_asset!("ellipsis-vertical.svg"),
    icon_asset!("copy.svg"),
    icon_asset!("refresh-cw.svg"),
    icon_asset!("paperclip.svg"),
    icon_asset!("camera.svg"),
    icon_asset!("settings.svg"),
    icon_asset!("send.svg"),
    icon_asset!("chevrons-up-down.svg"),
    icon_asset!("message-square-plus.svg"),
    icon_asset!("user.svg"),
    icon_asset!("x.svg"),
    icon_asset!("minus.svg"),
    icon_asset!("maximize-2.svg"),
    icon_asset!("keyboard.svg"),
    icon_asset!("folder.svg"),
    icon_asset!("clipboard.svg"),
    icon_asset!("file.svg"),
    icon_asset!("sparkles.svg"),
    icon_asset!("box.svg"),
    icon_asset!("rotate-ccw.svg"),
    icon_asset!("pencil.svg"),
    icon_asset!("zoom-in.svg"),
    icon_asset!("zoom-out.svg"),
    icon_asset!("help-circle.svg"),
    icon_asset!("log-out.svg"),
    icon_asset!("check.svg"),
    icon_asset!("chevron-left.svg"),
    icon_asset!("chevron-right.svg"),
    icon_asset!("image.svg"),
    icon_asset!("panel-left.svg"),
    icon_asset!("panel-left-close.svg"),
    icon_asset!("sun.svg"),
    icon_asset!("moon.svg"),
    icon_asset!("square.svg"),
];

/// Compile-time bundled SVG icons, keyed by `icons/<name>.svg`.
#[derive(Clone, Copy, Debug, Default)]
pub struct RoninAssets;

impl AssetSource for RoninAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let bytes = ICON_ASSETS
            .iter()
            .find(|(asset_path, _)| *asset_path == path)
            .map(|(_, data)| Cow::Borrowed(*data));
        Ok(bytes)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let prefix = path.trim_matches('/');
        Ok(ICON_ASSETS
            .iter()
            .map(|(asset_path, _)| *asset_path)
            .filter(|asset_path| icon_is_under_prefix(asset_path, prefix))
            .map(SharedString::from)
            .collect())
    }
}

fn icon_is_under_prefix(asset_path: &str, prefix: &str) -> bool {
    if prefix.is_empty() || prefix == "icons" {
        return asset_path.starts_with("icons/");
    }
    asset_path == prefix
        || asset_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// Inter Regular and SemiBold bytes for `TextSystem::add_fonts`.
pub fn bundled_font_bytes() -> Vec<Cow<'static, [u8]>> {
    vec![
        Cow::Borrowed(include_bytes!("../../../assets/fonts/Inter-Regular.ttf") as &[u8]),
        Cow::Borrowed(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf") as &[u8]),
    ]
}
