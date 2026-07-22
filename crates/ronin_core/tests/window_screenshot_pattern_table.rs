//! Pattern table for screenshot preference × capability (#76).

use ronin_core::{
    plan_screenshot_capture, ScreenshotBackendCapabilities, ScreenshotCaptureMode,
    ScreenshotTargetPreference,
};

#[test]
fn preference_capability_cartesian_product() {
    let prefs = [
        ScreenshotTargetPreference::Interactive,
        ScreenshotTargetPreference::Window,
    ];
    let caps_flags = [false, true];
    let mut seen = 0usize;
    for pref in prefs {
        for supports in caps_flags {
            for _repeat in 0..50 {
                let plan = plan_screenshot_capture(
                    pref,
                    ScreenshotBackendCapabilities {
                        supports_window_target: supports,
                    },
                );
                match (pref, supports) {
                    (ScreenshotTargetPreference::Interactive, _) => {
                        assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
                        assert!(plan.fallback.is_none());
                        assert!(!plan.fell_back_due_to_caps);
                    }
                    (ScreenshotTargetPreference::Window, true) => {
                        assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
                        assert_eq!(plan.fallback, Some(ScreenshotCaptureMode::Interactive));
                        assert!(!plan.fell_back_due_to_caps);
                    }
                    (ScreenshotTargetPreference::Window, false) => {
                        assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
                        assert!(plan.fallback.is_none());
                        assert!(plan.fell_back_due_to_caps);
                    }
                }
                seen += 1;
            }
        }
    }
    assert_eq!(seen, 200);
}

#[test]
fn pattern_row_000() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_001() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_002() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_003() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_004() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_005() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_006() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_007() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_008() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_009() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_010() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_011() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_012() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_013() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_014() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_015() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_016() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_017() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_018() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_019() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_020() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_021() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_022() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_023() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_024() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_025() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_026() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_027() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_028() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_029() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_030() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_031() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_032() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_033() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_034() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_035() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_036() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_037() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_038() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_039() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_040() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_041() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_042() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_043() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_044() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_045() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_046() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_047() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_048() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_049() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_050() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_051() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_052() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_053() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_054() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_055() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_056() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_057() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_058() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_059() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_060() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_061() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_062() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_063() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_064() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_065() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_066() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_067() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_068() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_069() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_070() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_071() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_072() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_073() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_074() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_075() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_076() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_077() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_078() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_079() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_080() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_081() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_082() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_083() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_084() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_085() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_086() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_087() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_088() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_089() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_090() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_091() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_092() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_093() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_094() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_095() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_096() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_097() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_098() {
    let pref = ScreenshotTargetPreference::Window;
    let supports = true;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn pattern_row_099() {
    let pref = ScreenshotTargetPreference::Interactive;
    let supports = false;
    let plan = plan_screenshot_capture(
        pref,
        ScreenshotBackendCapabilities {
            supports_window_target: supports,
        },
    );
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}
