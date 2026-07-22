//! Host screenshot target planning helpers (#76).

use ronin::{host_would_attempt_window_target, resolve_host_screenshot_plan};
use ronin_core::{ScreenshotCaptureMode, ScreenshotTargetPreference, SCREENSHOT_TARGET_WINDOW_BIT};

#[test]
fn host_attempts_window_only_when_bit_advertised() {
    assert!(host_would_attempt_window_target(
        ScreenshotTargetPreference::Window,
        Some(SCREENSHOT_TARGET_WINDOW_BIT)
    ));
    assert!(host_would_attempt_window_target(
        ScreenshotTargetPreference::Window,
        Some(1 | SCREENSHOT_TARGET_WINDOW_BIT | 4)
    ));
    assert!(!host_would_attempt_window_target(
        ScreenshotTargetPreference::Window,
        Some(1)
    ));
    assert!(!host_would_attempt_window_target(
        ScreenshotTargetPreference::Window,
        Some(8)
    ));
    assert!(!host_would_attempt_window_target(
        ScreenshotTargetPreference::Window,
        None
    ));
    assert!(!host_would_attempt_window_target(
        ScreenshotTargetPreference::Interactive,
        Some(SCREENSHOT_TARGET_WINDOW_BIT)
    ));
}

#[test]
fn host_plan_fallback_flags_match_core_policy() {
    let plan = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(2));
    assert_eq!(plan.primary, ScreenshotCaptureMode::Window);
    assert_eq!(plan.fallback, Some(ScreenshotCaptureMode::Interactive));
    assert!(!plan.fell_back_due_to_caps);

    let plan = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, None);
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(plan.fell_back_due_to_caps);

    let plan = resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(2));
    assert_eq!(plan.primary, ScreenshotCaptureMode::Interactive);
    assert!(!plan.fell_back_due_to_caps);
}

#[test]
fn host_plan_matrix_over_common_portal_masks() {
    let masks = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 10, 15, 255];
    for mask in masks {
        for pref in [
            ScreenshotTargetPreference::Interactive,
            ScreenshotTargetPreference::Window,
        ] {
            let attempt = host_would_attempt_window_target(pref, Some(mask));
            let expect = pref == ScreenshotTargetPreference::Window && (mask & 2) != 0;
            assert_eq!(attempt, expect, "mask={mask} pref={pref:?}");
        }
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            None
        ));
    }
}
