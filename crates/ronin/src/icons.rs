//! Named Lucide icons and SVG element helpers.

use gpui::{prelude::*, px, svg, Hsla, Svg};

/// Bundled Lucide icon used by chrome, menus, and the palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconName {
    /// Add / new.
    Plus,
    /// Search.
    Search,
    /// Hamburger menu.
    Menu,
    /// Horizontal overflow.
    Ellipsis,
    /// Vertical overflow.
    EllipsisVertical,
    /// Copy to clipboard.
    Copy,
    /// Refresh / retry (`refresh-cw`).
    Refresh,
    /// Attach file.
    Paperclip,
    /// Screenshot.
    Camera,
    /// Settings.
    Settings,
    /// Send message.
    Send,
    /// Expand / model picker.
    ChevronsUpDown,
    /// New chat.
    MessageSquarePlus,
    /// Account / user.
    User,
    /// Close / dismiss.
    X,
    /// Minimize / subtract.
    Minus,
    /// Maximize / window (`maximize-2`).
    Maximize,
    /// Keyboard shortcuts.
    Keyboard,
    /// Folder.
    Folder,
    /// Clipboard.
    Clipboard,
    /// File.
    File,
    /// Memory / sparkle.
    Sparkles,
    /// Artifact.
    Box,
    /// Regenerate (`rotate-ccw`).
    RotateCcw,
    /// Edit / rename.
    Pencil,
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Help.
    HelpCircle,
    /// Sign out.
    LogOut,
    /// Confirm / done.
    Check,
    /// Navigate back.
    ChevronLeft,
    /// Navigate forward.
    ChevronRight,
    /// Image attachment.
    Image,
    /// Sidebar.
    PanelLeft,
    /// Collapse sidebar.
    PanelLeftClose,
    /// Light theme.
    Sun,
    /// Dark theme.
    Moon,
    /// Stop / stop generation.
    Square,
}

/// Asset path passed to [`gpui::Svg::path`] / [`crate::assets::RoninAssets::load`].
pub fn icon_path(name: IconName) -> &'static str {
    match name {
        IconName::Plus => "icons/plus.svg",
        IconName::Search => "icons/search.svg",
        IconName::Menu => "icons/menu.svg",
        IconName::Ellipsis => "icons/ellipsis.svg",
        IconName::EllipsisVertical => "icons/ellipsis-vertical.svg",
        IconName::Copy => "icons/copy.svg",
        IconName::Refresh => "icons/refresh-cw.svg",
        IconName::Paperclip => "icons/paperclip.svg",
        IconName::Camera => "icons/camera.svg",
        IconName::Settings => "icons/settings.svg",
        IconName::Send => "icons/send.svg",
        IconName::ChevronsUpDown => "icons/chevrons-up-down.svg",
        IconName::MessageSquarePlus => "icons/message-square-plus.svg",
        IconName::User => "icons/user.svg",
        IconName::X => "icons/x.svg",
        IconName::Minus => "icons/minus.svg",
        IconName::Maximize => "icons/maximize-2.svg",
        IconName::Keyboard => "icons/keyboard.svg",
        IconName::Folder => "icons/folder.svg",
        IconName::Clipboard => "icons/clipboard.svg",
        IconName::File => "icons/file.svg",
        IconName::Sparkles => "icons/sparkles.svg",
        IconName::Box => "icons/box.svg",
        IconName::RotateCcw => "icons/rotate-ccw.svg",
        IconName::Pencil => "icons/pencil.svg",
        IconName::ZoomIn => "icons/zoom-in.svg",
        IconName::ZoomOut => "icons/zoom-out.svg",
        IconName::HelpCircle => "icons/help-circle.svg",
        IconName::LogOut => "icons/log-out.svg",
        IconName::Check => "icons/check.svg",
        IconName::ChevronLeft => "icons/chevron-left.svg",
        IconName::ChevronRight => "icons/chevron-right.svg",
        IconName::Image => "icons/image.svg",
        IconName::PanelLeft => "icons/panel-left.svg",
        IconName::PanelLeftClose => "icons/panel-left-close.svg",
        IconName::Sun => "icons/sun.svg",
        IconName::Moon => "icons/moon.svg",
        IconName::Square => "icons/square.svg",
    }
}

/// Short tooltip noun for the icon.
pub fn icon_label(name: IconName) -> &'static str {
    match name {
        IconName::Plus => "Add",
        IconName::Search => "Search",
        IconName::Menu => "Menu",
        IconName::Ellipsis | IconName::EllipsisVertical => "More",
        IconName::Copy => "Copy",
        IconName::Refresh => "Refresh",
        IconName::Paperclip => "Attach",
        IconName::Camera => "Screenshot",
        IconName::Settings => "Settings",
        IconName::Send => "Send",
        IconName::ChevronsUpDown => "Expand",
        IconName::MessageSquarePlus => "New chat",
        IconName::User => "Account",
        IconName::X => "Close",
        IconName::Minus => "Minimize",
        IconName::Maximize => "Maximize",
        IconName::Keyboard => "Shortcuts",
        IconName::Folder => "Folder",
        IconName::Clipboard => "Clipboard",
        IconName::File => "File",
        IconName::Sparkles => "Memory",
        IconName::Box => "Artifact",
        IconName::RotateCcw => "Regenerate",
        IconName::Pencil => "Edit",
        IconName::ZoomIn => "Zoom in",
        IconName::ZoomOut => "Zoom out",
        IconName::HelpCircle => "Help",
        IconName::LogOut => "Sign out",
        IconName::Check => "Confirm",
        IconName::ChevronLeft => "Back",
        IconName::ChevronRight => "Forward",
        IconName::Image => "Image",
        IconName::PanelLeft => "Sidebar",
        IconName::PanelLeftClose => "Close sidebar",
        IconName::Sun => "Light",
        IconName::Moon => "Dark",
        IconName::Square => "Stop",
    }
}

/// Colored SVG element sized in logical pixels.
///
/// GPUI 0.2.2 [`Svg`] implements [`Styled`], so `text_color` tints the stroke.
pub fn icon(name: IconName, color: Hsla, size_px: f32) -> Svg {
    svg()
        .path(icon_path(name))
        .text_color(color)
        .size(px(size_px))
}
