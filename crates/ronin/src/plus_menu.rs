//! Composer + attach menu rows.
//!
//! Catalog mirrors `composer_pickers` insert/label strings, with icons. That
//! module is left untouched.

use crate::icons::IconName;

/// One row in the composer + menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlusMenuItem {
    /// Text inserted into the composer (e.g. `"@file:"`).
    pub insert: &'static str,
    /// User-visible label.
    pub label: &'static str,
    /// Leading icon.
    pub icon: IconName,
}

/// Composer + menu: file, folder, clipboard, screenshots, and optional memory/artifact.
pub fn plus_menu_items(memories_enabled: bool, artifacts_enabled: bool) -> Vec<PlusMenuItem> {
    let mut items = vec![
        PlusMenuItem {
            insert: "@file:",
            label: "Attach file",
            icon: IconName::File,
        },
        PlusMenuItem {
            insert: "@folder:",
            label: "Attach folder",
            icon: IconName::Folder,
        },
        PlusMenuItem {
            insert: "@clipboard",
            label: "Attach clipboard",
            icon: IconName::Clipboard,
        },
        PlusMenuItem {
            insert: "@screenshot",
            label: "Capture screenshot",
            icon: IconName::Camera,
        },
        PlusMenuItem {
            insert: "@screenshot:window",
            label: "Capture window",
            icon: IconName::Maximize,
        },
    ];
    if artifacts_enabled {
        items.push(PlusMenuItem {
            insert: "@artifact:",
            label: "Attach artifact",
            icon: IconName::Box,
        });
    }
    if memories_enabled {
        items.push(PlusMenuItem {
            insert: "@memory:",
            label: "Attach memory",
            icon: IconName::Sparkles,
        });
    }
    items
}
