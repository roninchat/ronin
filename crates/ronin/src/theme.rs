//! Color themes and semantic UI tokens for the native Ronin shell.

use gpui::{rgb, WindowAppearance};
use ronin_core::ColorScheme;

/// Maps the desktop window appearance to a Ronin color scheme.
pub fn color_scheme_from_appearance(appearance: WindowAppearance) -> ColorScheme {
    match appearance {
        WindowAppearance::Light | WindowAppearance::VibrantLight => ColorScheme::Light,
        WindowAppearance::Dark | WindowAppearance::VibrantDark => ColorScheme::Dark,
    }
}

/// Resolves the shell palette from config preference and desktop appearance.
pub fn resolve_shell_theme(
    preference: ronin_core::ThemePreference,
    appearance: WindowAppearance,
) -> M0Theme {
    let system = color_scheme_from_appearance(appearance);
    let scheme = ronin_core::resolve_color_scheme(preference, system);
    M0Theme::for_scheme(scheme)
}

/// Semantic UI tokens that resolve per color scheme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeTokens {
    /// Primary surface background.
    pub surface: gpui::Hsla,
    /// Elevated surface (cards, panels).
    pub surface_elevated: gpui::Hsla,
    /// Primary text color.
    pub text: gpui::Hsla,
    /// Muted/secondary text color.
    pub text_muted: gpui::Hsla,
    /// Default border color.
    pub border: gpui::Hsla,
    /// Accent color for actions and highlights.
    pub accent: gpui::Hsla,
    /// Danger/error color.
    pub danger: gpui::Hsla,
    /// Base spacing unit in pixels.
    pub spacing: f32,
    /// Default corner radius in pixels.
    pub radius: f32,
}

impl ThemeTokens {
    /// Resolves semantic tokens for the given color scheme.
    pub fn for_scheme(scheme: ColorScheme) -> Self {
        match scheme {
            ColorScheme::Dark => Self::dark(),
            ColorScheme::Light => Self::light(),
        }
    }

    fn dark() -> Self {
        Self {
            surface: rgb(0x1e1e2e).into(),
            surface_elevated: rgb(0x313244).into(),
            text: rgb(0xcdd6f4).into(),
            text_muted: rgb(0xa6adc8).into(),
            border: rgb(0x313244).into(),
            accent: rgb(0xcba6f7).into(),
            danger: rgb(0xf38ba8).into(),
            spacing: 8.0,
            radius: 8.0,
        }
    }

    fn light() -> Self {
        Self {
            surface: rgb(0xeff1f5).into(),
            surface_elevated: rgb(0xffffff).into(),
            text: rgb(0x4c4f69).into(),
            text_muted: rgb(0x6c6f85).into(),
            border: rgb(0xccd0da).into(),
            accent: rgb(0x8839ef).into(),
            danger: rgb(0xd20f39).into(),
            spacing: 8.0,
            radius: 8.0,
        }
    }
}

/// Color palette used by the M0 native shell.
#[derive(Clone, Copy)]
pub struct M0Theme {
    /// Resolved light/dark scheme (drives syntax highlighting themes).
    pub color_scheme: ColorScheme,
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
    /// Resolves the shell palette for the given color scheme.
    pub fn for_scheme(scheme: ColorScheme) -> Self {
        match scheme {
            ColorScheme::Dark => Self::dark(),
            ColorScheme::Light => Self::light(),
        }
    }

    /// The polished dark theme.
    pub fn dark() -> Self {
        Self {
            color_scheme: ColorScheme::Dark,
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

    /// The polished light theme (Catppuccin Latte–aligned, not a dark fallback).
    pub fn light() -> Self {
        Self {
            color_scheme: ColorScheme::Light,
            app_background: rgb(0xeff1f5).into(),
            sidebar_background: rgb(0xe6e9ef).into(),
            surface_muted: rgb(0xffffff).into(),
            surface_hover: rgb(0xdce0e8).into(),
            surface_selected: rgb(0xccd0da).into(),
            composer_background: rgb(0xffffff).into(),
            border_subtle: rgb(0xccd0da).into(),
            border_strong: rgb(0xbcc0cc).into(),
            text_primary: rgb(0x4c4f69).into(),
            text_muted: rgb(0x6c6f85).into(),
            accent: rgb(0x8839ef).into(),
            accent_hover: rgb(0x7287fd).into(),
            accent_text: rgb(0xffffff).into(),
        }
    }
}
