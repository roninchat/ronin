//! Screenshot capture abstraction for portal-based and fallback providers.

use std::path::{Path, PathBuf};

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

/// Captures a screenshot to a filesystem path.
///
/// Implementations may use the XDG desktop portal (Wayland) or a fallback
/// mechanism. Tests inject a [`FakeScreenshotCapturer`].
pub trait ScreenshotCapturer {
    /// Captures a screenshot, writing under `dest_dir` when the provider creates
    /// a new file. Returns the path of the captured image.
    fn capture(&self, dest_dir: &Path) -> Result<PathBuf, ScreenshotError>;
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
}
