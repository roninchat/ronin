//! M0 color theme for the native Ronin shell.

use gpui::rgb;

/// Color palette used by the M0 native shell.
#[derive(Clone, Copy)]
pub struct M0Theme {
    /// Main window background.
    pub app_background: gpui::Hsla,
    /// Sidebar and side panel background.
    pub sidebar_background: gpui::Hsla,
    /// Muted surface for cards and rows.
    pub surface_muted: gpui::Hsla,
    /// Hovered surface.
    pub surface_hover: gpui::Hsla,
    /// Selected surface.
    pub surface_selected: gpui::Hsla,
    /// Composer input background.
    pub composer_background: gpui::Hsla,
    /// Subtle border color.
    pub border_subtle: gpui::Hsla,
    /// Strong border color.
    pub border_strong: gpui::Hsla,
    /// Primary text color.
    pub text_primary: gpui::Hsla,
    /// Muted text color.
    pub text_muted: gpui::Hsla,
    /// Accent color for actions and highlights.
    pub accent: gpui::Hsla,
    /// Accent hover color.
    pub accent_hover: gpui::Hsla,
    /// Text color rendered on accent surfaces.
    pub accent_text: gpui::Hsla,
}

impl M0Theme {
    /// The default dark theme.
    pub fn dark() -> Self {
        Self {
            app_background: rgb(0x1e1e2e).into(),
            sidebar_background: rgb(0x181825).into(),
            surface_muted: rgb(0x313244).into(),
            surface_hover: rgb(0x45475a).into(),
            surface_selected: rgb(0x585b70).into(),
            composer_background: rgb(0x11111b).into(),
            border_subtle: rgb(0x313244).into(),
            border_strong: rgb(0x45475a).into(),
            text_primary: rgb(0xcdd6f4).into(),
            text_muted: rgb(0xa6adc8).into(),
            accent: rgb(0xcba6f7).into(),
            accent_hover: rgb(0xb4befe).into(),
            accent_text: rgb(0x11111b).into(),
        }
    }
}
