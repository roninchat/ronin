//! Silent-attach negatives for window screenshot capture (#76).

use ronin_core::{
    may_inject_into_chat_request, screenshot_attachment, screenshot_capture_bytes_origin,
    screenshot_capture_may_inject_into_chat_request,
    screenshot_explicit_attach_may_inject_into_chat_request, ContextOrigin,
    RecordingScreenshotCapturer, ScreenshotBackendCapabilities, ScreenshotCapturer,
    ScreenshotTargetPreference,
};

fn png(dir: &std::path::Path, n: &str) -> std::path::PathBuf {
    let p = dir.join(n);
    std::fs::write(
        &p,
        [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE,
            0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ],
    )
    .unwrap();
    p
}

#[test]
fn silent_negative_00() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s0.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_01() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s1.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_02() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s2.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_03() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s3.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_04() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s4.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_05() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s5.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_06() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s6.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_07() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s7.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_08() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s8.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_09() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s9.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_10() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s10.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_11() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s11.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_12() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s12.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_13() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s13.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_14() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s14.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_15() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s15.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_16() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s16.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_17() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s17.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_18() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s18.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_19() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s19.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_20() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s20.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_21() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s21.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_22() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s22.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_23() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s23.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_24() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s24.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_25() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s25.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_26() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s26.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_27() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s27.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_28() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s28.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_29() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s29.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_30() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s30.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_31() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s31.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_32() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s32.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_33() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s33.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_34() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s34.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_35() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s35.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_36() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s36.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_37() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s37.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_38() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s38.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_39() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s39.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_40() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s40.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_41() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s41.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_42() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s42.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_43() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s43.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_44() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s44.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_45() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s45.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_46() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s46.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_47() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s47.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_48() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s48.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_49() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s49.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_50() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s50.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_51() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s51.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_52() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s52.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_53() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s53.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_54() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s54.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_55() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s55.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_56() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s56.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_57() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s57.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_58() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s58.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_59() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s59.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_60() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s60.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_61() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s61.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_62() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s62.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_63() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s63.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_64() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s64.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_65() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s65.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_66() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s66.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_67() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s67.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_68() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s68.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_69() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s69.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_70() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s70.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_71() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s71.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_72() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s72.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_73() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s73.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_74() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s74.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_75() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s75.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_76() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s76.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_77() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s77.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_78() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s78.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}

#[test]
fn silent_negative_79() {
    assert!(!screenshot_capture_may_inject_into_chat_request());
    assert!(!may_inject_into_chat_request(
        screenshot_capture_bytes_origin()
    ));
    assert!(!may_inject_into_chat_request(
        ContextOrigin::AmbientDesktopEvent
    ));
    assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    let dir = tempfile::tempdir().unwrap();
    let path = png(dir.path(), "s79.png");
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: false,
    });
    let captured = c
        .capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .unwrap();
    assert!(!screenshot_capture_may_inject_into_chat_request());
    let _draft = screenshot_attachment(&captured).unwrap();
    assert!(may_inject_into_chat_request(
        ContextOrigin::ExplicitAttachment
    ));
}
