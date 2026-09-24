//! Icon asset paths are non-empty and match bundled filenames.

use ronin::icons::{icon_label, icon_path, IconName};

const ALL: &[IconName] = &[
    IconName::Plus,
    IconName::Search,
    IconName::Menu,
    IconName::Ellipsis,
    IconName::EllipsisVertical,
    IconName::Copy,
    IconName::Refresh,
    IconName::Paperclip,
    IconName::Camera,
    IconName::Settings,
    IconName::Send,
    IconName::ChevronsUpDown,
    IconName::MessageSquarePlus,
    IconName::User,
    IconName::X,
    IconName::Minus,
    IconName::Maximize,
    IconName::Keyboard,
    IconName::Folder,
    IconName::Clipboard,
    IconName::File,
    IconName::Sparkles,
    IconName::Box,
    IconName::RotateCcw,
    IconName::Pencil,
    IconName::ZoomIn,
    IconName::ZoomOut,
    IconName::HelpCircle,
    IconName::LogOut,
    IconName::Check,
    IconName::ChevronLeft,
    IconName::ChevronRight,
    IconName::Image,
    IconName::PanelLeft,
    IconName::PanelLeftClose,
    IconName::Sun,
    IconName::Moon,
    IconName::Square,
];

#[test]
fn icon_paths_should_be_non_empty_svg_assets() {
    for name in ALL {
        let path = icon_path(*name);
        assert!(!path.is_empty(), "{name:?} path empty");
        assert!(path.starts_with("icons/"), "{name:?} {path}");
        assert!(path.ends_with(".svg"), "{name:?} {path}");
        assert!(!icon_label(*name).is_empty(), "{name:?} label empty");
    }
}

#[test]
fn icon_paths_should_be_unique() {
    let mut paths: Vec<_> = ALL.iter().map(|name| icon_path(*name)).collect();
    paths.sort_unstable();
    let before = paths.len();
    paths.dedup();
    assert_eq!(paths.len(), before);
}
