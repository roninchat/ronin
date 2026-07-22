//! Dense host screenshot plan coverage (#76).

use ronin::{host_would_attempt_window_target, resolve_host_screenshot_plan};
use ronin_core::{ScreenshotCaptureMode, ScreenshotTargetPreference};

#[test]
fn host_dense_mask_000() {
    let mask = 0u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_001() {
    let mask = 3u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_002() {
    let mask = 6u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_003() {
    let mask = 9u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_004() {
    let mask = 12u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_005() {
    let mask = 15u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_006() {
    let mask = 18u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_007() {
    let mask = 21u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_008() {
    let mask = 24u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_009() {
    let mask = 27u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_010() {
    let mask = 30u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_011() {
    let mask = 1u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_012() {
    let mask = 4u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_013() {
    let mask = 7u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_014() {
    let mask = 10u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_015() {
    let mask = 13u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_016() {
    let mask = 16u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_017() {
    let mask = 19u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_018() {
    let mask = 22u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_019() {
    let mask = 25u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_020() {
    let mask = 28u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_021() {
    let mask = 31u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_022() {
    let mask = 2u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_023() {
    let mask = 5u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_024() {
    let mask = 8u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_025() {
    let mask = 11u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_026() {
    let mask = 14u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_027() {
    let mask = 17u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_028() {
    let mask = 20u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_029() {
    let mask = 23u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_030() {
    let mask = 26u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_031() {
    let mask = 29u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_032() {
    let mask = 0u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_033() {
    let mask = 3u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_034() {
    let mask = 6u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_035() {
    let mask = 9u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_036() {
    let mask = 12u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_037() {
    let mask = 15u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_038() {
    let mask = 18u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_039() {
    let mask = 21u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_040() {
    let mask = 24u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_041() {
    let mask = 27u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_042() {
    let mask = 30u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_043() {
    let mask = 1u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_044() {
    let mask = 4u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_045() {
    let mask = 7u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_046() {
    let mask = 10u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_047() {
    let mask = 13u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_048() {
    let mask = 16u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_049() {
    let mask = 19u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_050() {
    let mask = 22u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_051() {
    let mask = 25u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_052() {
    let mask = 28u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_053() {
    let mask = 31u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_054() {
    let mask = 2u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_055() {
    let mask = 5u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_056() {
    let mask = 8u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_057() {
    let mask = 11u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_058() {
    let mask = 14u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_059() {
    let mask = 17u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_060() {
    let mask = 20u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_061() {
    let mask = 23u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_062() {
    let mask = 26u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_063() {
    let mask = 29u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_064() {
    let mask = 0u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_065() {
    let mask = 3u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_066() {
    let mask = 6u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_067() {
    let mask = 9u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_068() {
    let mask = 12u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_069() {
    let mask = 15u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_070() {
    let mask = 18u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_071() {
    let mask = 21u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_072() {
    let mask = 24u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_073() {
    let mask = 27u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_074() {
    let mask = 30u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_075() {
    let mask = 1u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_076() {
    let mask = 4u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_077() {
    let mask = 7u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_078() {
    let mask = 10u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_079() {
    let mask = 13u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_080() {
    let mask = 16u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_081() {
    let mask = 19u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_082() {
    let mask = 22u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_083() {
    let mask = 25u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_084() {
    let mask = 28u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_085() {
    let mask = 31u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_086() {
    let mask = 2u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_087() {
    let mask = 5u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_088() {
    let mask = 8u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_089() {
    let mask = 11u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_090() {
    let mask = 14u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_091() {
    let mask = 17u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_092() {
    let mask = 20u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_093() {
    let mask = 23u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_094() {
    let mask = 26u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_095() {
    let mask = 29u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_096() {
    let mask = 0u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_097() {
    let mask = 3u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_098() {
    let mask = 6u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_099() {
    let mask = 9u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_100() {
    let mask = 12u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_101() {
    let mask = 15u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_102() {
    let mask = 18u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_103() {
    let mask = 21u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_104() {
    let mask = 24u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_105() {
    let mask = 27u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_106() {
    let mask = 30u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_107() {
    let mask = 1u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_108() {
    let mask = 4u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_109() {
    let mask = 7u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_110() {
    let mask = 10u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_111() {
    let mask = 13u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_112() {
    let mask = 16u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_113() {
    let mask = 19u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_114() {
    let mask = 22u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_115() {
    let mask = 25u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_116() {
    let mask = 28u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_117() {
    let mask = 31u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_118() {
    let mask = 2u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}

#[test]
fn host_dense_mask_119() {
    let mask = 5u32;
    let window = resolve_host_screenshot_plan(ScreenshotTargetPreference::Window, Some(mask));
    let interactive =
        resolve_host_screenshot_plan(ScreenshotTargetPreference::Interactive, Some(mask));
    assert_eq!(interactive.primary, ScreenshotCaptureMode::Interactive);
    assert!(!interactive.fell_back_due_to_caps);
    if mask & 2 != 0 {
        assert_eq!(window.primary, ScreenshotCaptureMode::Window);
        assert!(host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    } else {
        assert_eq!(window.primary, ScreenshotCaptureMode::Interactive);
        assert!(window.fell_back_due_to_caps);
        assert!(!host_would_attempt_window_target(
            ScreenshotTargetPreference::Window,
            Some(mask)
        ));
    }
}
