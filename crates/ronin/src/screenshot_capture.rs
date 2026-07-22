//! Wayland portal screenshot capture with CLI fallbacks.
//!
//! M3.0 (#76): prefers portal Window target when advertised; otherwise falls
//! back to interactive portal / CLI paths.

use std::path::{Path, PathBuf};
use std::process::Command;

use ronin_core::{
    capabilities_from_available_targets, execute_screenshot_plan, plan_screenshot_capture,
    ScreenshotBackendCapabilities, ScreenshotCaptureMode, ScreenshotCapturer, ScreenshotError,
    ScreenshotModeRunner, ScreenshotTargetPreference,
};

/// Tries XDG desktop portal (ashpd) first, then CLI fallbacks (`grim`, `gnome-screenshot`, `import`).
#[derive(Debug, Default)]
pub struct PortalOrFallbackScreenshotCapturer;

impl ScreenshotCapturer for PortalOrFallbackScreenshotCapturer {
    fn capture(&self, dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
        self.capture_with_preference(dest_dir, ScreenshotTargetPreference::Interactive)
    }

    fn capabilities(&self) -> ScreenshotBackendCapabilities {
        portal_backend_capabilities()
    }

    fn capture_with_preference(
        &self,
        dest_dir: &Path,
        preference: ScreenshotTargetPreference,
    ) -> Result<PathBuf, ScreenshotError> {
        let caps = self.capabilities();
        let plan = plan_screenshot_capture(preference, caps);
        // Prefer portal-backed modes; on total portal failure, CLI interactive fallback.
        match execute_screenshot_plan(self, dest_dir, plan) {
            Ok(path) => Ok(path),
            Err(portal_plan_err) => {
                // Last resort: CLI tools (always interactive / full-output style).
                capture_via_fallback(dest_dir, preference).map_err(|fallback_err| {
                    ScreenshotError::CaptureFailed(format!(
                        "portal plan failed ({portal_plan_err}); CLI fallback failed ({fallback_err})"
                    ))
                })
            }
        }
    }
}

impl ScreenshotModeRunner for PortalOrFallbackScreenshotCapturer {
    fn capture_mode(
        &self,
        dest_dir: &Path,
        mode: ScreenshotCaptureMode,
    ) -> Result<PathBuf, ScreenshotError> {
        match mode {
            ScreenshotCaptureMode::Interactive => capture_via_portal(dest_dir, None),
            ScreenshotCaptureMode::Window => capture_via_portal(
                dest_dir,
                Some(ashpd::desktop::screenshot::AvailableTargets::Window),
            ),
        }
    }
}

fn portal_backend_capabilities() -> ScreenshotBackendCapabilities {
    let available = block_on_available_targets();
    capabilities_from_available_targets(available)
}

fn block_on_available_targets() -> Option<u32> {
    async_std::task::block_on(async {
        let connection = ashpd::zbus::Connection::session().await.ok()?;
        let proxy = ashpd::zbus::Proxy::new(
            &connection,
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Screenshot",
        )
        .await
        .ok()?;
        proxy.get_property::<u32>("AvailableTargets").await.ok()
    })
}

fn capture_via_portal(
    dest_dir: &Path,
    target: Option<ashpd::desktop::screenshot::AvailableTargets>,
) -> Result<PathBuf, ScreenshotError> {
    std::fs::create_dir_all(dest_dir)?;
    let uri = block_on_portal_screenshot(target)?;
    let source = uri_to_path(&uri)?;
    if !source.exists() {
        return Err(ScreenshotError::CaptureFailed(format!(
            "portal returned missing file: {}",
            source.display()
        )));
    }
    let dest = dest_dir.join(
        source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("screenshot.png"),
    );
    if source != dest {
        std::fs::copy(&source, &dest)?;
        Ok(dest)
    } else {
        Ok(source)
    }
}

fn block_on_portal_screenshot(
    target: Option<ashpd::desktop::screenshot::AvailableTargets>,
) -> Result<String, ScreenshotError> {
    async_std::task::block_on(async {
        let mut request = ashpd::desktop::screenshot::Screenshot::request()
            .interactive(true)
            .modal(true);
        if let Some(t) = target {
            request = request.target(t);
        }
        let response = request
            .send()
            .await
            .map_err(|e| ScreenshotError::CaptureFailed(e.to_string()))?
            .response()
            .map_err(|e| ScreenshotError::CaptureFailed(e.to_string()))?;
        Ok(response.uri().as_str().to_string())
    })
}

fn uri_to_path(uri: &str) -> Result<PathBuf, ScreenshotError> {
    let path = uri.strip_prefix("file://").ok_or_else(|| {
        ScreenshotError::CaptureFailed(format!("unexpected screenshot URI: {uri}"))
    })?;
    let decoded = urlencoding_decode(path);
    Ok(PathBuf::from(decoded))
}

fn urlencoding_decode(path: &str) -> String {
    // Minimal %XX decode for portal file URIs.
    let mut out = String::with_capacity(path.len());
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (
                (bytes[i + 1] as char).to_digit(16),
                (bytes[i + 2] as char).to_digit(16),
            ) {
                out.push(char::from((h * 16 + l) as u8));
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn capture_via_fallback(
    dest_dir: &Path,
    preference: ScreenshotTargetPreference,
) -> Result<PathBuf, ScreenshotError> {
    std::fs::create_dir_all(dest_dir)?;
    let dest = dest_dir.join(format!(
        "ronin-screenshot-{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    ));
    let dest_str = dest.to_string_lossy().into_owned();

    // Window preference: try gnome-screenshot -w first when available.
    if preference == ScreenshotTargetPreference::Window {
        match Command::new("gnome-screenshot")
            .args(["-w", "-f", &dest_str])
            .status()
        {
            Ok(status) if status.success() && dest.exists() => return Ok(dest),
            Ok(status) => {
                tracing::debug!(%status, "gnome-screenshot -w did not succeed; continuing fallbacks");
            }
            Err(e) => {
                tracing::debug!(error = %e, "gnome-screenshot unavailable for window fallback");
            }
        }
    }

    let attempts: [(&str, Vec<&str>); 3] = [
        ("grim", vec![&dest_str]),
        ("gnome-screenshot", vec!["-f", &dest_str]),
        ("import", vec!["-window", "root", &dest_str]),
    ];

    let mut errors = Vec::new();
    for (bin, args) in &attempts {
        match Command::new(bin).args(args).status() {
            Ok(status) if status.success() && dest.exists() => return Ok(dest),
            Ok(status) => errors.push(format!("{bin} exited {status}")),
            Err(e) => errors.push(format!("{bin}: {e}")),
        }
    }
    Err(ScreenshotError::CaptureFailed(errors.join("; ")))
}

/// Pure helper used by host tests: resolve preference against reported caps.
pub fn resolve_host_screenshot_plan(
    preference: ScreenshotTargetPreference,
    available_targets: Option<u32>,
) -> ronin_core::ScreenshotCapturePlan {
    let caps = capabilities_from_available_targets(available_targets);
    plan_screenshot_capture(preference, caps)
}

/// Pure helper: whether a capture preference would attempt window mode first.
pub fn host_would_attempt_window_target(
    preference: ScreenshotTargetPreference,
    available_targets: Option<u32>,
) -> bool {
    resolve_host_screenshot_plan(preference, available_targets).primary
        == ScreenshotCaptureMode::Window
}

#[cfg(test)]
mod preference_plan_tests {
    use super::*;

    #[test]
    fn host_plan_window_when_bit_set() {
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(2)
        ));
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(1)
        ));
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Interactive,
            Some(2)
        ));
    }
}
