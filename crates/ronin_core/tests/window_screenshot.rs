//! Window-targeted screenshot capture policy + silent-attach gates (#76).
//!
//! Public seams:
//! - [`ronin_core::plan_screenshot_capture`]
//! - [`ronin_core::execute_screenshot_plan`]
//! - [`ronin_core::capture_with_preference_plan`]
//! - [`ronin_core::portal_supports_window_target`]
//! - [`ronin_core::screenshot_capture_may_inject_into_chat_request`]
//! - [`ronin_core::RecordingScreenshotCapturer`]

use std::path::Path;

use ronin_core::{
    capabilities_from_available_targets, capture_with_preference_plan, execute_screenshot_plan,
    may_inject_into_chat_request, plan_screenshot_capture, portal_supports_window_target,
    screenshot_attachment, screenshot_capture_bytes_origin,
    screenshot_capture_may_inject_into_chat_request,
    screenshot_explicit_attach_may_inject_into_chat_request, ContextOrigin,
    RecordingScreenshotCapturer, ScreenshotBackendCapabilities, ScreenshotCaptureMode,
    ScreenshotCapturer, ScreenshotTargetPreference, SCREENSHOT_TARGET_WINDOW_BIT,
};

fn fixture_png(dir: &Path) -> std::path::PathBuf {
    let path = dir.join("shot.png");
    // Minimal 1x1 PNG
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    std::fs::write(&path, png).expect("write fixture");
    path
}

#[test]
fn window_bit_matches_portal_spec() {
    assert_eq!(SCREENSHOT_TARGET_WINDOW_BIT, 2);
    assert!(portal_supports_window_target(2));
    assert!(portal_supports_window_target(2 | 1 | 4));
    assert!(!portal_supports_window_target(0));
    assert!(!portal_supports_window_target(1));
    assert!(!portal_supports_window_target(4));
    assert!(!portal_supports_window_target(8)); // ActiveWindow alone ≠ Window
}

#[test]
fn capabilities_from_available_targets_none_means_unsupported() {
    let caps = capabilities_from_available_targets(None);
    assert!(!caps.supports_window_target);
    let caps = capabilities_from_available_targets(Some(2));
    assert!(caps.supports_window_target);
    let caps = capabilities_from_available_targets(Some(1));
    assert!(!caps.supports_window_target);
}

#[test]
fn plan_window_when_supported_uses_window_with_interactive_fallback() {
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert_eq!(plan.fallback, Some(ScreenshotCaptureMode::Interactive));
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn plan_window_when_unsupported_falls_back_to_interactive() {
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert_eq!(plan.fallback, None);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn plan_interactive_never_requests_window() {
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Interactive,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert_eq!(plan.fallback, None);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn execute_falls_back_when_window_mode_fails() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let runner = RecordingScreenshotCapturer::new();
    runner.set_path(&path);
    runner.fail_on(ScreenshotCaptureMode::Window);

    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    let captured = execute_screenshot_plan(&runner, dir.path(), plan).expect("fallback ok");
    assert_eq!(captured, path);
    assert_eq!(
        runner.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
}

#[test]
fn capture_with_preference_selects_window_when_available() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let capturer = RecordingScreenshotCapturer::new();
    capturer.set_path(&path);
    capturer.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });

    let captured = capturer
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .expect("window capture");
    assert_eq!(captured, path);
    assert_eq!(
        capturer.take_preferences(),
        vec![ScreenshotTargetPreference::Window]
    );
    assert_eq!(capturer.take_modes(), vec![ScreenshotCaptureMode::Window]);
}

#[test]
fn capture_with_preference_window_unsupported_uses_interactive_only() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let capturer = RecordingScreenshotCapturer::new();
    capturer.set_path(&path);
    capturer.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });

    capturer
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .expect("interactive fallback");
    assert_eq!(
        capturer.take_modes(),
        vec![ScreenshotCaptureMode::Interactive]
    );
}

#[test]
fn raw_capture_bytes_never_inject_into_chat_without_explicit_attach() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    // Explicit attach path remains the only admission gate for screenshots.
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn capture_alone_does_not_build_chat_attachment_without_screenshot_attachment() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let capturer = RecordingScreenshotCapturer::new();
    capturer.set_path(&path);
    capturer.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = capturer
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .expect("captured");
    // Path exists, but trust still blocks ambient injection — attach is a separate step.
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let draft = screenshot_attachment(&captured).expect("explicit attach");
    assert_eq!(draft.kind, ronin_core::AttachmentKind::Screenshot);
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn preference_plan_helper_matches_capturer_path() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let runner = RecordingScreenshotCapturer::new();
    runner.set_path(&path);
    let caps = ScreenshotBackendCapabilities {
        supports_window_target: true,
    };
    let captured = capture_with_preference_plan(
        &runner,
        dir.path(),
        ScreenshotTargetPreference::Window,
        caps,
    )
    .expect("plan capture");
    assert_eq!(captured, path);
    assert_eq!(runner.take_modes(), vec![ScreenshotCaptureMode::Window]);
}

#[test]
fn mode_runner_window_then_interactive_on_forced_failure() {
    let dir = tempfile::tempdir().expect("temp");
    let path = fixture_png(dir.path());
    let runner = RecordingScreenshotCapturer::new();
    runner.set_path(&path);
    runner.fail_on(ScreenshotCaptureMode::Window);
    let caps = ScreenshotBackendCapabilities {
        supports_window_target: true,
    };
    capture_with_preference_plan(
        &runner,
        dir.path(),
        ScreenshotTargetPreference::Window,
        caps,
    )
    .expect("fallback");
    assert_eq!(
        runner.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
}
