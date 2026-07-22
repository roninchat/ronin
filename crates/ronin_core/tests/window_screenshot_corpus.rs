//! Corpus coverage for screenshot target planning (#76).

use std::path::PathBuf;

use ronin_core::{
    capture_with_preference_plan, execute_screenshot_plan, plan_screenshot_capture,
    portal_supports_window_target, screenshot_capture_may_inject_into_chat_request,
    screenshot_explicit_attach_may_inject_into_chat_request, RecordingScreenshotCapturer,
    ScreenshotBackendCapabilities, ScreenshotCaptureMode, ScreenshotTargetPreference,
    SCREENSHOT_TARGET_WINDOW_BIT,
};

fn write_png(path: &std::path::Path) {
    let png: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8,
        0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00,
        0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    std::fs::write(path, png).expect("png");
}

#[test]
fn portal_bit_corpus_0_through_31() {
    for bits in 0u32..32 {
        let expect = bits & SCREENSHOT_TARGET_WINDOW_BIT != 0;
        assert_eq!(portal_supports_window_target(bits), expect, "bits={bits}");
    }
}

#[test]
fn plan_corpus_for_bitmasks_and_preferences() {
    let prefs = [
        ScreenshotTargetPreference::Interactive,
        ScreenshotTargetPreference::Window,
    ];
    for bits in [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 10, 15, 255, 1024] {
        let supports = bits & SCREENSHOT_TARGET_WINDOW_BIT != 0;
        let caps = ScreenshotBackendCapabilities {
            supports_window_target: supports,
        };
        for pref in prefs {
            let plan = plan_screenshot_capture(pref, caps);
            match pref {
                ScreenshotTargetPreference::Interactive => {
                    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
                    assert!(plan.fallback.is_none());
                    assert!(!plan.fell_back_due_to_caps);
                }
                ScreenshotTargetPreference::Window if supports => {
                    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
                    assert_eq!(plan.fallback, Some(ScreenshotCaptureMode::Interactive));
                    assert!(!plan.fell_back_due_to_caps);
                }
                ScreenshotTargetPreference::Window => {
                    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
                    assert!(plan.fallback.is_none());
                    assert!(plan.fell_back_due_to_caps);
                }
            }
        }
    }
}

#[test]
fn execute_corpus_labeled_window_fallback_paths() {
    let labels: &[&str] = &[
        "case-000", "case-001", "case-002", "case-003", "case-004", "case-005", "case-006",
        "case-007", "case-008", "case-009", "case-010", "case-011", "case-012", "case-013",
        "case-014", "case-015", "case-016", "case-017", "case-018", "case-019", "case-020",
        "case-021", "case-022", "case-023", "case-024", "case-025", "case-026", "case-027",
        "case-028", "case-029", "case-030", "case-031", "case-032", "case-033", "case-034",
        "case-035", "case-036", "case-037", "case-038", "case-039", "case-040", "case-041",
        "case-042", "case-043", "case-044", "case-045", "case-046", "case-047", "case-048",
        "case-049", "case-050", "case-051", "case-052", "case-053", "case-054", "case-055",
        "case-056", "case-057", "case-058", "case-059", "case-060", "case-061", "case-062",
        "case-063", "case-064", "case-065", "case-066", "case-067", "case-068", "case-069",
        "case-070", "case-071", "case-072", "case-073", "case-074", "case-075", "case-076",
        "case-077", "case-078", "case-079",
    ];
    for (i, label) in labels.iter().enumerate() {
        let dir = tempfile::tempdir().expect("temp");
        let path = dir.path().join(format!("{label}.png"));
        write_png(&path);
        let runner = RecordingScreenshotCapturer::new();
        runner.set_path(&path);
        let supports = i % 2 == 0;
        let fail_window = i % 3 == 0 && supports;
        if fail_window {
            runner.fail_on(ScreenshotCaptureMode::Window);
        }
        let caps = ScreenshotBackendCapabilities {
            supports_window_target: supports,
        };
        let pref = if i % 5 == 0 {
            ScreenshotTargetPreference::Interactive
        } else {
            ScreenshotTargetPreference::Window
        };
        let got = capture_with_preference_plan(&runner, dir.path(), pref, caps)
            .unwrap_or_else(|e| panic!("{label}: {e}"));
        assert_eq!(got, path, "{label}");
        let modes = runner.take_modes();
        match pref {
            ScreenshotTargetPreference::Interactive => {
                assert_eq!(modes, vec![ScreenshotCaptureMode::Interactive], "{label}");
            }
            ScreenshotTargetPreference::Window if supports && fail_window => {
                assert_eq!(
                    modes,
                    vec![
                        ScreenshotCaptureMode::Window,
                        ScreenshotCaptureMode::Interactive
                    ],
                    "{label}"
                );
            }
            ScreenshotTargetPreference::Window if supports => {
                assert_eq!(modes, vec![ScreenshotCaptureMode::Window], "{label}");
            }
            ScreenshotTargetPreference::Window => {
                assert_eq!(modes, vec![ScreenshotCaptureMode::Interactive], "{label}");
            }
        }
        assert!(!screenshot_capture_may_inject_into_chat_request());
        assert!(screenshot_explicit_attach_may_inject_into_chat_request());
    }
}

#[test]
fn silent_attach_invariant_holds_for_each_plan_shape() {
    for supports in [false, true] {
        for pref in [
            ScreenshotTargetPreference::Interactive,
            ScreenshotTargetPreference::Window,
        ] {
            let plan = plan_screenshot_capture(
                pref,
                ScreenshotBackendCapabilities {
                    supports_window_target: supports,
                },
            );
            let _ = plan;
            assert!(!screenshot_capture_may_inject_into_chat_request());
        }
    }
}

#[test]
fn execute_reports_combined_error_when_both_modes_fail() {
    let dir = tempfile::tempdir().expect("temp");
    let path = dir.path().join("x.png");
    write_png(&path);
    let runner = RecordingScreenshotCapturer::new();
    runner.set_path(&path);
    runner.fail_on(ScreenshotCaptureMode::Window);
    runner.fail_on(ScreenshotCaptureMode::Interactive);
    let plan = plan_screenshot_capture(
        ScreenshotTargetPreference::Window,
        ScreenshotBackendCapabilities {
            supports_window_target: true,
        },
    );
    let err = execute_screenshot_plan(&runner, dir.path(), plan).expect_err("both fail");
    let msg = err.to_string();
    assert!(msg.contains("primary"));
    assert!(msg.contains("fallback"));
}

#[test]
fn recording_preferences_are_tracked() {
    let dir = tempfile::tempdir().expect("temp");
    let mut path = PathBuf::from(dir.path());
    path.push("pref.png");
    write_png(&path);
    let c = RecordingScreenshotCapturer::new();
    c.set_path(&path);
    c.set_capabilities(ScreenshotBackendCapabilities {
        supports_window_target: true,
    });
    use ronin_core::ScreenshotCapturer;
    c.capture_with_preference(dir.path(), ScreenshotTargetPreference::Window)
        .expect("ok");
    assert_eq!(
        c.take_preferences(),
        vec![ScreenshotTargetPreference::Window]
    );
}
