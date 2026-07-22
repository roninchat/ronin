//! Wayland portal screenshot capture with CLI fallbacks.

use std::path::{Path, PathBuf};
use std::process::Command;

use ronin_core::{ScreenshotCapturer, ScreenshotError};

/// Tries XDG desktop portal (ashpd) first, then CLI fallbacks (`grim`, `gnome-screenshot`, `import`).
#[derive(Debug, Default)]
pub struct PortalOrFallbackScreenshotCapturer;

impl ScreenshotCapturer for PortalOrFallbackScreenshotCapturer {
    fn capture(&self, dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
        match capture_via_portal(dest_dir) {
            Ok(path) => Ok(path),
            Err(portal_err) => capture_via_fallback(dest_dir).map_err(|fallback_err| {
                ScreenshotError::CaptureFailed(format!(
                    "portal failed ({portal_err}); fallback failed ({fallback_err})"
                ))
            }),
        }
    }
}

fn capture_via_portal(dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
    std::fs::create_dir_all(dest_dir)?;
    let uri = block_on_portal_screenshot()?;
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

fn block_on_portal_screenshot() -> Result<String, ScreenshotError> {
    async_std::task::block_on(async {
        let response = ashpd::desktop::screenshot::Screenshot::request()
            .interactive(true)
            .modal(true)
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

fn capture_via_fallback(dest_dir: &Path) -> Result<PathBuf, ScreenshotError> {
    std::fs::create_dir_all(dest_dir)?;
    let dest = dest_dir.join(format!(
        "ronin-screenshot-{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    ));
    let dest_str = dest.to_string_lossy().into_owned();

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
