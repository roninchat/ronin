//! Keyboard-first navigation state machine and discoverable shortcut catalog.
//!
//! Testable without GPUI: focus regions, thread highlight, scroll intents, help overlay.

/// Which primary chrome region currently owns keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRegion {
    /// Thread list / sidebar chrome.
    Sidebar,
    /// Message transcript scroll area.
    Messages,
    /// Composer text input.
    Composer,
}

/// Vertical scroll intent for the message list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Page toward older / top content.
    Up,
    /// Page toward newer / bottom content.
    Down,
}

/// Result of handling a key through the navigation state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavAction {
    /// Key was ignored by navigation (may belong to composer/editor).
    None,
    /// Primary focus region changed.
    FocusChanged(FocusRegion),
    /// Sidebar thread highlight moved.
    ThreadHighlightChanged {
        /// New highlighted index.
        index: usize,
    },
    /// Enter should open/select the highlighted thread.
    SelectThread {
        /// Thread index to select.
        index: usize,
    },
    /// Page the message list.
    ScrollMessages(ScrollDirection),
    /// Help overlay visibility flipped.
    ToggleHelp,
    /// Global search overlay visibility flipped.
    ToggleSearch,
}

/// Normalized key event for the navigation state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInput<'a> {
    /// GPUI-style key name (`"tab"`, `"pageup"`, `"1"`, …).
    pub key: &'a str,
    /// Control / Ctrl modifier.
    pub control: bool,
    /// Shift modifier.
    pub shift: bool,
    /// Alt / Option modifier.
    pub alt: bool,
}

/// Mutable keyboard navigation state for the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardNavState {
    focus: FocusRegion,
    thread_highlight: Option<usize>,
    help_visible: bool,
}

impl Default for KeyboardNavState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardNavState {
    /// Creates navigation state with composer focus (M0 default).
    pub fn new() -> Self {
        Self {
            focus: FocusRegion::Composer,
            thread_highlight: None,
            help_visible: false,
        }
    }

    /// Current focus region.
    pub fn focus(&self) -> FocusRegion {
        self.focus
    }

    /// Highlighted thread index in the sidebar, if any.
    pub fn thread_highlight(&self) -> Option<usize> {
        self.thread_highlight
    }

    /// Whether the keyboard shortcut help overlay is open.
    pub fn help_visible(&self) -> bool {
        self.help_visible
    }

    /// Sets focus and ensures a valid thread highlight when entering the sidebar.
    pub fn set_focus(&mut self, region: FocusRegion, thread_count: usize) {
        self.focus = region;
        if region == FocusRegion::Sidebar {
            self.ensure_thread_highlight(thread_count);
        }
    }

    /// Cycles focus forward (Tab) or backward (Shift+Tab).
    pub fn cycle_focus(&mut self, reverse: bool, thread_count: usize) -> FocusRegion {
        self.focus = match (self.focus, reverse) {
            (FocusRegion::Composer, false) => FocusRegion::Sidebar,
            (FocusRegion::Sidebar, false) => FocusRegion::Messages,
            (FocusRegion::Messages, false) => FocusRegion::Composer,
            (FocusRegion::Composer, true) => FocusRegion::Messages,
            (FocusRegion::Messages, true) => FocusRegion::Sidebar,
            (FocusRegion::Sidebar, true) => FocusRegion::Composer,
        };
        if self.focus == FocusRegion::Sidebar {
            self.ensure_thread_highlight(thread_count);
        }
        self.focus
    }

    fn ensure_thread_highlight(&mut self, thread_count: usize) {
        if thread_count == 0 {
            self.thread_highlight = None;
            return;
        }
        let idx = self
            .thread_highlight
            .unwrap_or(0)
            .min(thread_count.saturating_sub(1));
        self.thread_highlight = Some(idx);
    }

    /// Moves the sidebar highlight by `delta`, clamping to `[0, thread_count)`.
    pub fn move_thread(&mut self, delta: i32, thread_count: usize) -> Option<usize> {
        if thread_count == 0 {
            self.thread_highlight = None;
            return None;
        }
        let current = self.thread_highlight.unwrap_or(0) as i32;
        let next = (current + delta).clamp(0, thread_count as i32 - 1) as usize;
        self.thread_highlight = Some(next);
        Some(next)
    }

    /// Handles a key for navigation. Returns `(consumed, action)`.
    ///
    /// When `consumed` is true, the shell should not forward the key to the composer.
    pub fn handle_key(&mut self, key: KeyInput<'_>, thread_count: usize) -> (bool, NavAction) {
        if key.alt {
            return (false, NavAction::None);
        }

        if key.control {
            match key.key {
                "/" | "?" => {
                    self.help_visible = !self.help_visible;
                    return (true, NavAction::ToggleHelp);
                }
                "f" if key.shift => {
                    return (true, NavAction::ToggleSearch);
                }
                "f" => {
                    return (true, NavAction::ToggleSearch);
                }
                "1" => {
                    self.set_focus(FocusRegion::Sidebar, thread_count);
                    return (true, NavAction::FocusChanged(FocusRegion::Sidebar));
                }
                "2" => {
                    self.set_focus(FocusRegion::Messages, thread_count);
                    return (true, NavAction::FocusChanged(FocusRegion::Messages));
                }
                "3" => {
                    self.set_focus(FocusRegion::Composer, thread_count);
                    return (true, NavAction::FocusChanged(FocusRegion::Composer));
                }
                _ => {}
            }
        }

        if key.key == "escape" && self.help_visible {
            self.help_visible = false;
            return (true, NavAction::ToggleHelp);
        }

        match self.focus {
            FocusRegion::Sidebar => self.handle_sidebar_key(key, thread_count),
            FocusRegion::Messages => self.handle_messages_key(key, thread_count),
            FocusRegion::Composer => self.handle_composer_key(key, thread_count),
        }
    }

    fn handle_sidebar_key(
        &mut self,
        key: KeyInput<'_>,
        thread_count: usize,
    ) -> (bool, NavAction) {
        if key.control {
            return (false, NavAction::None);
        }
        match key.key {
            "up" => {
                if let Some(index) = self.move_thread(-1, thread_count) {
                    (true, NavAction::ThreadHighlightChanged { index })
                } else {
                    (true, NavAction::None)
                }
            }
            "down" => {
                if let Some(index) = self.move_thread(1, thread_count) {
                    (true, NavAction::ThreadHighlightChanged { index })
                } else {
                    (true, NavAction::None)
                }
            }
            "enter" if !key.shift => {
                if let Some(index) = self.thread_highlight {
                    (true, NavAction::SelectThread { index })
                } else {
                    (true, NavAction::None)
                }
            }
            "tab" => {
                let focus = self.cycle_focus(key.shift, thread_count);
                (true, NavAction::FocusChanged(focus))
            }
            _ => (false, NavAction::None),
        }
    }

    fn handle_messages_key(
        &mut self,
        key: KeyInput<'_>,
        thread_count: usize,
    ) -> (bool, NavAction) {
        if key.control {
            return (false, NavAction::None);
        }
        match key.key {
            "pageup" => (true, NavAction::ScrollMessages(ScrollDirection::Up)),
            "pagedown" => (true, NavAction::ScrollMessages(ScrollDirection::Down)),
            "tab" => {
                let focus = self.cycle_focus(key.shift, thread_count);
                (true, NavAction::FocusChanged(focus))
            }
            _ => (false, NavAction::None),
        }
    }

    fn handle_composer_key(
        &mut self,
        key: KeyInput<'_>,
        thread_count: usize,
    ) -> (bool, NavAction) {
        if key.control {
            return (false, NavAction::None);
        }
        // Tab cycles focus when the shell decides there is no completion to accept.
        if key.key == "tab" {
            let focus = self.cycle_focus(key.shift, thread_count);
            return (true, NavAction::FocusChanged(focus));
        }
        (false, NavAction::None)
    }
}

/// One discoverable keyboard shortcut row for tooltips / help overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutHint {
    /// Key chord label (e.g. `"Ctrl+N"`).
    pub keys: &'static str,
    /// Short action description.
    pub action: &'static str,
}

/// Catalog of M0 + navigation shortcuts for discoverability.
pub fn shortcut_catalog() -> &'static [ShortcutHint] {
    &[
        ShortcutHint {
            keys: "Enter",
            action: "Send message",
        },
        ShortcutHint {
            keys: "Shift+Enter",
            action: "Insert newline",
        },
        ShortcutHint {
            keys: "Esc",
            action: "Cancel generation / close help",
        },
        ShortcutHint {
            keys: "Ctrl+N",
            action: "New thread",
        },
        ShortcutHint {
            keys: "Ctrl+L / Ctrl+K",
            action: "Focus composer",
        },
        ShortcutHint {
            keys: "Ctrl+B",
            action: "Collapse / expand sidebar",
        },
        ShortcutHint {
            keys: "Ctrl+1",
            action: "Focus sidebar",
        },
        ShortcutHint {
            keys: "Ctrl+2",
            action: "Focus message list",
        },
        ShortcutHint {
            keys: "Ctrl+3",
            action: "Focus composer",
        },
        ShortcutHint {
            keys: "Tab",
            action: "Cycle focus (sidebar → messages → composer)",
        },
        ShortcutHint {
            keys: "↑ / ↓",
            action: "Move thread highlight (sidebar focused)",
        },
        ShortcutHint {
            keys: "Enter",
            action: "Open highlighted thread (sidebar focused)",
        },
        ShortcutHint {
            keys: "Page Up / Page Down",
            action: "Scroll message list",
        },
        ShortcutHint {
            keys: "Ctrl+F / Ctrl+Shift+F",
            action: "Open global search",
        },
        ShortcutHint {
            keys: "Ctrl+/",
            action: "Toggle keyboard shortcut help",
        },
    ]
}
