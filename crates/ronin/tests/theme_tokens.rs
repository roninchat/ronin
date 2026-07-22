//! Semantic theme token resolution for light and dark schemes.

use gpui::{rgb, WindowAppearance};
use ronin::quick_mode::resolve_quick_overlay_theme;
use ronin::theme::{color_scheme_from_appearance, resolve_shell_theme, M0Theme, ThemeTokens};
use ronin_core::{ColorScheme, ThemePreference};

#[test]
fn dark_theme_tokens_should_match_polished_dark_palette() {
    let tokens = ThemeTokens::for_scheme(ColorScheme::Dark);

    assert_eq!(tokens.surface, rgb(0x1e1e2e).into());
    assert_eq!(tokens.surface_elevated, rgb(0x313244).into());
    assert_eq!(tokens.text, rgb(0xcdd6f4).into());
    assert_eq!(tokens.text_muted, rgb(0xa6adc8).into());
    assert_eq!(tokens.border, rgb(0x313244).into());
    assert_eq!(tokens.accent, rgb(0xcba6f7).into());
    assert_eq!(tokens.danger, rgb(0xf38ba8).into());
    assert_eq!(tokens.spacing, 8.0);
    assert_eq!(tokens.radius, 8.0);
}

#[test]
fn light_theme_tokens_should_match_polished_light_palette() {
    let tokens = ThemeTokens::for_scheme(ColorScheme::Light);

    assert_eq!(tokens.surface, rgb(0xeff1f5).into());
    assert_eq!(tokens.surface_elevated, rgb(0xffffff).into());
    assert_eq!(tokens.text, rgb(0x4c4f69).into());
    assert_eq!(tokens.text_muted, rgb(0x6c6f85).into());
    assert_eq!(tokens.border, rgb(0xccd0da).into());
    assert_eq!(tokens.accent, rgb(0x8839ef).into());
    assert_eq!(tokens.danger, rgb(0xd20f39).into());
    assert_eq!(tokens.spacing, 8.0);
    assert_eq!(tokens.radius, 8.0);
}

#[test]
fn light_and_dark_theme_tokens_should_differ_on_surfaces_and_text() {
    let light = ThemeTokens::for_scheme(ColorScheme::Light);
    let dark = ThemeTokens::for_scheme(ColorScheme::Dark);

    assert_ne!(light.surface, dark.surface);
    assert_ne!(light.surface_elevated, dark.surface_elevated);
    assert_ne!(light.text, dark.text);
    assert_ne!(light.text_muted, dark.text_muted);
    assert_ne!(light.border, dark.border);
    assert_ne!(light.accent, dark.accent);
    assert_ne!(light.danger, dark.danger);
}

#[test]
fn color_scheme_from_appearance_should_map_light_variants_to_light() {
    assert_eq!(
        color_scheme_from_appearance(WindowAppearance::Light),
        ColorScheme::Light
    );
    assert_eq!(
        color_scheme_from_appearance(WindowAppearance::VibrantLight),
        ColorScheme::Light
    );
}

#[test]
fn color_scheme_from_appearance_should_map_dark_variants_to_dark() {
    assert_eq!(
        color_scheme_from_appearance(WindowAppearance::Dark),
        ColorScheme::Dark
    );
    assert_eq!(
        color_scheme_from_appearance(WindowAppearance::VibrantDark),
        ColorScheme::Dark
    );
}

#[test]
fn m0_theme_for_scheme_should_force_polished_light_not_dark_fallback() {
    let light = M0Theme::for_scheme(ColorScheme::Light);
    let dark = M0Theme::for_scheme(ColorScheme::Dark);

    assert_eq!(light.app_background, rgb(0xeff1f5).into());
    assert_eq!(light.sidebar_background, rgb(0xe6e9ef).into());
    assert_eq!(light.surface_muted, rgb(0xffffff).into());
    assert_eq!(light.composer_background, rgb(0xffffff).into());
    assert_eq!(light.text_primary, rgb(0x4c4f69).into());
    assert_eq!(light.text_muted, rgb(0x6c6f85).into());
    assert_eq!(light.border_subtle, rgb(0xccd0da).into());
    assert_eq!(light.accent, rgb(0x8839ef).into());
    assert_eq!(light.accent_text, rgb(0xffffff).into());

    assert_ne!(light.app_background, dark.app_background);
    assert_ne!(light.text_primary, dark.text_primary);
}

#[test]
fn m0_theme_for_scheme_should_use_polished_dark_palette() {
    let dark = M0Theme::for_scheme(ColorScheme::Dark);

    assert_eq!(dark.app_background, rgb(0x1e1e2e).into());
    assert_eq!(dark.sidebar_background, rgb(0x181825).into());
    assert_eq!(dark.text_primary, rgb(0xcdd6f4).into());
    assert_eq!(dark.accent, rgb(0xcba6f7).into());
}

#[test]
fn resolve_shell_theme_should_follow_system_when_preference_is_system() {
    let light = resolve_shell_theme(ThemePreference::System, WindowAppearance::Light);
    let dark = resolve_shell_theme(ThemePreference::System, WindowAppearance::Dark);

    assert_eq!(light.app_background, rgb(0xeff1f5).into());
    assert_eq!(dark.app_background, rgb(0x1e1e2e).into());
}

#[test]
fn resolve_shell_theme_should_force_dark_regardless_of_appearance() {
    let theme = resolve_shell_theme(ThemePreference::Dark, WindowAppearance::Light);
    assert_eq!(theme.app_background, rgb(0x1e1e2e).into());
}

#[test]
fn resolve_shell_theme_should_force_light_regardless_of_appearance() {
    let theme = resolve_shell_theme(ThemePreference::Light, WindowAppearance::Dark);
    assert_eq!(theme.app_background, rgb(0xeff1f5).into());
}

#[test]
fn quick_overlay_theme_should_match_shell_theme_for_preference_and_appearance() {
    let light = resolve_quick_overlay_theme(ThemePreference::System, WindowAppearance::Light);
    let dark = resolve_quick_overlay_theme(ThemePreference::System, WindowAppearance::Dark);
    let forced = resolve_quick_overlay_theme(ThemePreference::Dark, WindowAppearance::Light);

    assert_eq!(light.app_background, rgb(0xeff1f5).into());
    assert_eq!(dark.app_background, rgb(0x1e1e2e).into());
    assert_eq!(forced.app_background, rgb(0x1e1e2e).into());
    assert_ne!(light.text_primary, dark.text_primary);
}
