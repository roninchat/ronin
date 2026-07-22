//! Screenshot capture abstraction for portal-based and fallback providers.
//!
//! M3.0 (#76): capture requests may prefer a **window** target when the backend
//! advertises it; otherwise resolve to the existing interactive path.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::trust::{may_inject_into_chat_request, ContextOrigin};

/// Portal `AvailableTargets` bit for user-selected window capture.
pub const SCREENSHOT_TARGET_WINDOW_BIT: u32 = 2;

/// Errors produced while capturing a screenshot.
#[derive(Debug, thiserror::Error)]
pub enum ScreenshotError {
    /// Capture provider failed.
    #[error("screenshot capture failed: {0}")]
    CaptureFailed(String),
    /// Captured file could not be written or located.
    #[error("screenshot file error: {0}")]
    Io(#[from] std::io::Error),
}

/// User / caller preference for how a screenshot should be targeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenshotTargetPreference {
    /// Existing interactive portal / CLI path (user picks region, window, or screen).
    #[default]
    Interactive,
    /// Prefer a window-scoped capture when the backend supports it.
    Window,
}

/// Concrete capture mode the host should attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotCaptureMode {
    /// Interactive / default screenshot path.
    Interactive,
    /// Window-targeted screenshot path.
    Window,
}

/// Capabilities advertised by a screenshot backend (portal property / probe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScreenshotBackendCapabilities {
    /// Backend advertises portal Window target (`AvailableTargets` bit 2).
    pub supports_window_target: bool,
}

/// Resolved capture plan: primary mode, optional fallback, and whether caps forced a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenshotCapturePlan {
    /// Mode to try first.
    pub primary: ScreenshotCaptureMode,
    /// Mode to try if primary fails (window → interactive).
    pub fallback: Option<ScreenshotCaptureMode>,
    /// True when Window was requested but caps lacked window support (immediate interactive).
    pub fell_back_due_to_caps: bool,
}

/// Whether `available_targets` bitmask includes the Window target.
pub fn portal_supports_window_target(available_targets: u32) -> bool {
    available_targets & SCREENSHOT_TARGET_WINDOW_BIT != 0
}

/// Build backend caps from a portal `AvailableTargets` bitmask (or `None` if unknown).
pub fn capabilities_from_available_targets(
    available_targets: Option<u32>,
) -> ScreenshotBackendCapabilities {
    ScreenshotBackendCapabilities {
        supports_window_target: available_targets.is_some_and(portal_supports_window_target),
    }
}

/// Resolve preference + capabilities into a capture plan (no I/O).
pub fn plan_screenshot_capture(
    preference: ScreenshotTargetPreference,
    caps: ScreenshotBackendCapabilities,
) -> ScreenshotCapturePlan {
    match preference {
        ScreenshotTargetPreference::Interactive => ScreenshotCapturePlan {
            primary: ScreenshotCaptureMode::Interactive,
            fallback: None,
            fell_back_due_to_caps: false,
        },
        ScreenshotTargetPreference::Window if caps.supports_window_target => {
            ScreenshotCapturePlan {
                primary: ScreenshotCaptureMode::Window,
                fallback: Some(ScreenshotCaptureMode::Interactive),
                fell_back_due_to_caps: false,
            }
        }
        ScreenshotTargetPreference::Window => ScreenshotCapturePlan {
            primary: ScreenshotCaptureMode::Interactive,
            fallback: None,
            fell_back_due_to_caps: true,
        },
    }
}

/// Execute a capture plan against a [`ScreenshotModeRunner`] (primary, then optional fallback).
pub fn execute_screenshot_plan<R: ScreenshotModeRunner + ?Sized>(
    runner: &R,
    dest_dir: &Path,
    plan: ScreenshotCapturePlan,
) -> Result<PathBuf, ScreenshotError> {
    match runner.capture_mode(dest_dir, plan.primary) {
        Ok(path) => Ok(path),
        Err(primary_err) => match plan.fallback {
            Some(fallback_mode) => runner.capture_mode(dest_dir, fallback_mode).map_err(
                |fallback_err| {
                    ScreenshotError::CaptureFailed(format!(
                        "primary {:?} failed ({primary_err}); fallback {:?} failed ({fallback_err})",
                        plan.primary, fallback_mode
                    ))
                },
            ),
            None => Err(primary_err),
        },
    }
}

/// Plan + execute capture for a preference given live backend capabilities.
pub fn capture_with_preference_plan<R: ScreenshotModeRunner + ?Sized>(
    runner: &R,
    dest_dir: &Path,
    preference: ScreenshotTargetPreference,
    caps: ScreenshotBackendCapabilities,
) -> Result<PathBuf, ScreenshotError> {
    let plan = plan_screenshot_capture(preference, caps);
    execute_screenshot_plan(runner, dest_dir, plan)
}

/// Host/test seam that can run a single concrete capture mode.
pub trait ScreenshotModeRunner {
    /// Capture using exactly `mode` (no preference resolution).
    fn capture_mode(
        &self,
        dest_dir: &Path,
        mode: ScreenshotCaptureMode,
    ) -> Result<PathBuf, ScreenshotError>;
}

/// Captures a screenshot to a filesystem path.
///
/// Implementations may use the XDG desktop portal (Wayland) or a fallback
/// mechanism. Tests inject a [`FakeScreenshotCapturer`] /
/// [`RecordingScreenshotCapturer`].
pub trait ScreenshotCapturer {
    /// Captures a screenshot, writing under `dest_dir` when the provider creates
    /// a new file. Returns the path of the captured image.
    ///
    /// Equivalent to [`ScreenshotCapturer::capture_with_preference`] with
    /// [`ScreenshotTargetPreference::Interactive`].
    fn capture(&self, dest_dir: &Path) -> Result<PathBuf, ScreenshotError>;

    /// Backend capability probe (default: no window target).
    fn capabilities(&self) -> ScreenshotBackendCapabilities {
        ScreenshotBackendCapabilities::default()
    }

    /// Capture using a target preference (window when available, else interactive).
    fn capture_with_preference(
        &self,
        dest_dir: &Path,
        preference: ScreenshotTargetPreference,
    ) -> Result<PathBuf, ScreenshotError> {
        let _ = preference;
        self.capture(dest_dir)
    }
}

/// Context origin for a raw capture path before the user attaches it — never model context.
pub fn screenshot_capture_bytes_origin() -> ContextOrigin {
    ContextOrigin::AmbientDesktopEvent
}

/// Whether a raw screenshot capture (pre-attach) may merge into a provider chat request.
pub fn screenshot_capture_may_inject_into_chat_request() -> bool {
    may_inject_into_chat_request(screenshot_capture_bytes_origin())
}

/// Whether an explicit `@screenshot` / capture-action attachment may enter chat.
pub fn screenshot_explicit_attach_may_inject_into_chat_request() -> bool {
    may_inject_into_chat_request(ContextOrigin::ExplicitAttachment)
}

/// Test double that returns a preconfigured path without talking to a portal.
#[derive(Debug, Clone)]
pub struct FakeScreenshotCapturer {
    path: PathBuf,
}

impl FakeScreenshotCapturer {
    /// Creates a fake capturer that always returns `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl ScreenshotCapturer for FakeScreenshotCapturer {
    fn capture(&self, _dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
        if !self.path.exists() {
            return Err(ScreenshotError::CaptureFailed(format!(
                "fake screenshot missing at {}",
                self.path.display()
            )));
        }
        Ok(self.path.clone())
    }

    fn capture_with_preference(
        &self,
        dest_dir: &Path,
        preference: ScreenshotTargetPreference,
    ) -> Result<PathBuf, ScreenshotError> {
        let _ = preference;
        self.capture(dest_dir)
    }
}

/// Recording capturer / mode runner for public-seam tests.
#[derive(Debug, Default)]
pub struct RecordingScreenshotCapturer {
    calls: Mutex<Vec<ScreenshotCaptureMode>>,
    preferences: Mutex<Vec<ScreenshotTargetPreference>>,
    /// Modes that should fail when invoked.
    fail_modes: Mutex<Vec<ScreenshotCaptureMode>>,
    path: Mutex<Option<PathBuf>>,
    caps: Mutex<ScreenshotBackendCapabilities>,
}

impl RecordingScreenshotCapturer {
    /// Empty recorder (set path / caps before capture).
    pub fn new() -> Self {
        Self::default()
    }

    /// Path returned on successful capture.
    pub fn set_path(&self, path: impl Into<PathBuf>) {
        *self.path.lock().unwrap_or_else(|p| p.into_inner()) = Some(path.into());
    }

    /// Backend capabilities reported by [`ScreenshotCapturer::capabilities`].
    pub fn set_capabilities(&self, caps: ScreenshotBackendCapabilities) {
        *self.caps.lock().unwrap_or_else(|p| p.into_inner()) = caps;
    }

    /// Mark a mode so the next `capture_mode` for it fails.
    pub fn fail_on(&self, mode: ScreenshotCaptureMode) {
        self.fail_modes
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .push(mode);
    }

    /// Snapshot of modes attempted (oldest first).
    pub fn take_modes(&self) -> Vec<ScreenshotCaptureMode> {
        std::mem::take(&mut *self.calls.lock().unwrap_or_else(|p| p.into_inner()))
    }

    /// Snapshot of preferences passed to [`ScreenshotCapturer::capture_with_preference`].
    pub fn take_preferences(&self) -> Vec<ScreenshotTargetPreference> {
        std::mem::take(&mut *self.preferences.lock().unwrap_or_else(|p| p.into_inner()))
    }
}

impl ScreenshotModeRunner for RecordingScreenshotCapturer {
    fn capture_mode(
        &self,
        _dest_dir: &Path,
        mode: ScreenshotCaptureMode,
    ) -> Result<PathBuf, ScreenshotError> {
        self.calls
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .push(mode);
        let fail = {
            let mut fails = self.fail_modes.lock().unwrap_or_else(|p| p.into_inner());
            if let Some(idx) = fails.iter().position(|m| *m == mode) {
                fails.remove(idx);
                true
            } else {
                false
            }
        };
        if fail {
            return Err(ScreenshotError::CaptureFailed(format!(
                "recording capturer forced failure for {mode:?}"
            )));
        }
        let path = self
            .path
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clone()
            .ok_or_else(|| ScreenshotError::CaptureFailed("recording path unset".into()))?;
        if !path.exists() {
            return Err(ScreenshotError::CaptureFailed(format!(
                "recording screenshot missing at {}",
                path.display()
            )));
        }
        Ok(path)
    }
}

impl ScreenshotCapturer for RecordingScreenshotCapturer {
    fn capture(&self, dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
        self.capture_with_preference(dest_dir, ScreenshotTargetPreference::Interactive)
    }

    fn capabilities(&self) -> ScreenshotBackendCapabilities {
        *self.caps.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn capture_with_preference(
        &self,
        dest_dir: &Path,
        preference: ScreenshotTargetPreference,
    ) -> Result<PathBuf, ScreenshotError> {
        self.preferences
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .push(preference);
        let caps = self.capabilities();
        capture_with_preference_plan(self, dest_dir, preference, caps)
    }
}
