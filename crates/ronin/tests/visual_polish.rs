//! Premium visual polish: elevation, empty states, errors, streaming motion.

use ronin::visual_polish::{
    cursor_visible_at, elevation_style, empty_state, error_presentation, streaming_motion,
    Elevation, EmptyStateKind, ErrorKind,
};
use ronin_core::ColorScheme;

#[test]
fn elevation_tokens_should_be_theme_aware() {
    let dark_low = elevation_style(Elevation::Low, ColorScheme::Dark);
    let light_low = elevation_style(Elevation::Low, ColorScheme::Light);

    assert!(dark_low.blur_radius > 0.0);
    assert!(light_low.blur_radius > 0.0);
    assert_ne!(
        dark_low.shadow_alpha, light_low.shadow_alpha,
        "light and dark elevations use different shadow strength"
    );
}

#[test]
fn elevation_levels_should_increase_blur() {
    let low = elevation_style(Elevation::Low, ColorScheme::Dark);
    let medium = elevation_style(Elevation::Medium, ColorScheme::Dark);
    let high = elevation_style(Elevation::High, ColorScheme::Dark);

    assert!(medium.blur_radius > low.blur_radius);
    assert!(high.blur_radius > medium.blur_radius);
    assert!(high.offset_y >= medium.offset_y);
}

#[test]
fn empty_states_should_cover_all_major_surfaces() {
    let kinds = [
        EmptyStateKind::NoThreads,
        EmptyStateKind::EmptyThread,
        EmptyStateKind::NoArtifacts,
        EmptyStateKind::NoMemories,
        EmptyStateKind::NoSearchResults,
        EmptyStateKind::OllamaOffline,
        EmptyStateKind::NoModelsInstalled,
    ];

    for kind in kinds {
        let content = empty_state(kind);
        assert!(
            !content.title.is_empty(),
            "{kind:?} missing title"
        );
        assert!(
            !content.body.is_empty(),
            "{kind:?} missing body"
        );
        assert!(
            !content.icon.is_empty(),
            "{kind:?} missing icon glyph"
        );
        assert!(
            content.action_hint.is_some(),
            "{kind:?} should suggest a next action"
        );
    }
}

#[test]
fn empty_thread_state_should_invite_first_message() {
    let content = empty_state(EmptyStateKind::EmptyThread);
    assert!(content.title.to_lowercase().contains("conversation") || content.title.to_lowercase().contains("chat"));
    assert!(content.body.to_lowercase().contains("message") || content.body.to_lowercase().contains("ask"));
}

#[test]
fn ollama_offline_empty_state_should_be_actionable() {
    let content = empty_state(EmptyStateKind::OllamaOffline);
    let blob = format!(
        "{} {} {}",
        content.title,
        content.body,
        content.action_hint.unwrap_or("")
    )
    .to_lowercase();
    assert!(blob.contains("ollama"));
    assert!(blob.contains("running") || blob.contains("start") || blob.contains("reachable"));
}

#[test]
fn error_presentation_should_be_consistent_and_actionable() {
    for kind in [
        ErrorKind::Provider,
        ErrorKind::StreamFailure,
        ErrorKind::MigrationFailure,
        ErrorKind::Attachment,
    ] {
        let err = error_presentation(kind, "detail about failure");
        assert!(!err.icon.is_empty());
        assert!(!err.title.is_empty());
        assert!(err.message.contains("detail about failure"));
        assert!(err.action_hint.is_some());
    }
}

#[test]
fn stream_failure_error_should_mention_retry() {
    let err = error_presentation(ErrorKind::StreamFailure, "connection reset");
    let hint = err.action_hint.unwrap_or("").to_lowercase();
    assert!(hint.contains("retry") || err.body_mentions_retry());
}

#[test]
fn streaming_motion_should_use_smooth_cursor_duty_cycle() {
    let motion = streaming_motion();
    assert!(motion.cursor_cycle_ms >= 700, "longer cycle reduces flicker");
    assert!(motion.cursor_visible_ms < motion.cursor_cycle_ms);
    let duty = motion.cursor_visible_ms as f32 / motion.cursor_cycle_ms as f32;
    assert!(
        (0.55..=0.70).contains(&duty),
        "visible duty cycle should feel steady, got {duty}"
    );

    assert!(cursor_visible_at(0, &motion));
    assert!(cursor_visible_at(motion.cursor_visible_ms - 1, &motion));
    assert!(!cursor_visible_at(motion.cursor_visible_ms, &motion));
    assert!(!cursor_visible_at(motion.cursor_cycle_ms - 1, &motion));
    assert!(cursor_visible_at(motion.cursor_cycle_ms, &motion));
}
