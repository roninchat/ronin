//! First-run shortcut coach steps.

use ronin::shortcut_coach::{first_run_steps, should_show_coach};

#[test]
fn first_run_steps_should_cover_send_new_chat_palette_and_sidebar() {
    let steps = first_run_steps();
    assert!(steps.len() <= 4);
    assert_eq!(steps.len(), 4);
    let ids: Vec<_> = steps.iter().map(|step| step.id).collect();
    assert_eq!(ids, ["send", "new-chat", "palette", "sidebar"]);
    assert!(steps[0].body.contains("Enter"));
    assert!(steps[1].body.contains("Ctrl+N"));
    assert!(steps[2].body.contains("Ctrl+P"));
    assert!(steps[3].body.contains("Ctrl+B") || steps[3].body.to_lowercase().contains("menu"));
}

#[test]
fn should_show_coach_only_when_unseen_and_hints_enabled() {
    assert!(should_show_coach(false, true));
    assert!(!should_show_coach(true, true));
    assert!(!should_show_coach(false, false));
    assert!(!should_show_coach(true, false));
}
