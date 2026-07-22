//! Dense screenshot target selection / fallback scenarios (#76).

use ronin_core::{
    execute_screenshot_plan, may_inject_into_chat_request, plan_screenshot_capture,
    screenshot_capture_bytes_origin, screenshot_capture_may_inject_into_chat_request,
    ContextOrigin, RecordingScreenshotCapturer, ScreenshotBackendCapabilities,
    ScreenshotCaptureMode, ScreenshotTargetPreference,
};

fn png(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("d.png");
    std::fs::write(
        &path,
        [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE,
            0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ],
    )
    .unwrap();
    path
}

#[test]
fn dense_window_supported_path_00() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_01() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_02() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_03() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_04() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_05() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_06() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_07() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_08() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_09() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_10() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_11() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_12() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_13() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_14() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_15() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_16() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_17() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_18() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_19() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_20() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_21() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_22() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_23() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_24() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_25() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_26() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_27() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_28() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_29() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_30() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_31() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_32() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_33() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_34() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_35() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_36() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_37() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_38() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    r.fail_on(ScreenshotCaptureMode::Window);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(
        r.take_modes(),
        vec![
            ScreenshotCaptureMode::Window,
            ScreenshotCaptureMode::Interactive
        ]
    );
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_supported_path_39() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Window]);
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert_eq!(
        screenshot_capture_bytes_origin(),
        ContextOrigin::AmbientDesktopEvent
    );
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
}

#[test]
fn dense_window_unsupported_path_00() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_01() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_02() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_03() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_04() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_05() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_06() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_07() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_08() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_09() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_10() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_11() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_12() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_13() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_14() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_15() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_16() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_17() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_18() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_19() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_20() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_21() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_22() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_23() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_24() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_25() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_26() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_27() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_28() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_29() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_30() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_31() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_32() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_33() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_34() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_35() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_36() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_37() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_38() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}

#[test]
fn dense_window_unsupported_path_39() {
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path());
    let r = RecordingScreenshotCapturer::new();
    r.set_path(&path);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: false,
        },
    );
    assert!(plan.fell_back_due_to_caps);
    execute_screenshot_plan(&r, dir.path(), plan).unwrap();
    assert_eq!(r.take_modes(), vec![ScreenshotCaptureMode::Interactive]);
}
