//! Table-driven cases for screenshot target planning (#76).

use ronin_core::{
    capabilities_from_available_targets, plan_screenshot_capture, portal_supports_window_target,
    ScreenshotBackendCapabilities, ScreenshotCaptureMode, ScreenshotTargetPreference,
    SCREENSHOT_TARGET_WINDOW_BIT,
};

struct PlanCase {
    name: &'static str,
    preference: ScreenshotTargetPreference,
    supports_window: bool,
    expect_primary: ScreenshotCaptureMode,
    expect_fallback: Option<ScreenshotCaptureMode>,
    expect_caps_fallback: bool,
}

struct BitCase {
    name: &'static str,
    bits: u32,
    expect_window: bool,
}

#[test]
fn plan_screenshot_capture_table() {
    let cases = [
        PlanCase {
            name: "interactive_no_caps",
            preference: ScreenshotTargetPreference::Interactive,
            supports_window: false,
            expect_primary: ScreenshotCaptureMode::Interactive,
            expect_fallback: None,
            expect_caps_fallback: false,
        },
        PlanCase {
            name: "interactive_with_window_caps",
            preference: ScreenshotTargetPreference::Interactive,
            supports_window: true,
            expect_primary: ScreenshotCaptureMode::Interactive,
            expect_fallback: None,
            expect_caps_fallback: false,
        },
        PlanCase {
            name: "window_supported",
            preference: ScreenshotTargetPreference::Window,
            supports_window: true,
            expect_primary: ScreenshotCaptureMode::Window,
            expect_fallback: Some(ScreenshotCaptureMode::Interactive),
            expect_caps_fallback: false,
        },
        PlanCase {
            name: "window_unsupported",
            preference: ScreenshotTargetPreference::Window,
            supports_window: false,
            expect_primary: ScreenshotCaptureMode::Interactive,
            expect_fallback: None,
            expect_caps_fallback: true,
        },
    ];
    for case in cases {
        let plan = plan_screenshot_capture(
            case.preference,
            ScreenshotBackendCapabilities {
                supports_window_target: case.supports_window,
            },
        );
        assert_eq!(plan.primary, case.expect_primary, "{}", case.name);
        assert_eq!(plan.fallback, case.expect_fallback, "{}", case.name);
        assert_eq!(
            plan.fell_back_due_to_caps, case.expect_caps_fallback,
            "{}",
            case.name
        );
    }
}

#[test]
fn portal_window_bit_table() {
    let cases = [
        BitCase {
            name: "zero",
            bits: 0,
            expect_window: false,
        },
        BitCase {
            name: "screen_only",
            bits: 1,
            expect_window: false,
        },
        BitCase {
            name: "window_only",
            bits: SCREENSHOT_TARGET_WINDOW_BIT,
            expect_window: true,
        },
        BitCase {
            name: "area_only",
            bits: 4,
            expect_window: false,
        },
        BitCase {
            name: "active_window_only",
            bits: 8,
            expect_window: false,
        },
        BitCase {
            name: "screen_and_window",
            bits: 1 | 2,
            expect_window: true,
        },
        BitCase {
            name: "all_bits",
            bits: 1 | 2 | 4 | 8,
            expect_window: true,
        },
        BitCase {
            name: "area_and_active",
            bits: 4 | 8,
            expect_window: false,
        },
    ];
    for case in cases {
        assert_eq!(
            portal_supports_window_target(case.bits),
            case.expect_window,
            "{}",
            case.name
        );
        let caps = capabilities_from_available_targets(Some(case.bits));
        assert_eq!(
            caps.supports_window_target, case.expect_window,
            "caps {}",
            case.name
        );
    }
}

#[test]
fn capabilities_none_is_unsupported_across_preferences() {
    let caps = capabilities_from_available_targets(None);
    assert!(!caps.supports_window_target);
    for pref in [
        ScreenshotTargetPreference::Interactive,
        ScreenshotTargetPreference::Window,
    ] {
        let plan = plan_screenshot_capture(pref, caps);
        assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
        assert!(plan.fallback.is_none());
        if pref == ScreenshotTargetPreference::Window {
            assert!(plan.fell_back_due_to_caps);
        } else {
            assert!(!plan.fell_back_due_to_caps);
        }
    }
}
