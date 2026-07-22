//! Persona / system-prompt customization.

use ronin_core::{effective_system_prompt, PersonaConfig, PersonaMode, RONIN_SYSTEM_PROMPT};

#[test]
fn effective_system_prompt_should_default_to_built_in_ronin_prompt() {
    let prompt = effective_system_prompt(&PersonaConfig::default());
    assert_eq!(prompt, RONIN_SYSTEM_PROMPT);
    assert!(prompt.contains("Ronin"));
}

#[test]
fn effective_system_prompt_should_append_custom_persona_text() {
    let persona = PersonaConfig {
        mode: PersonaMode::Append,
        text: "Speak like a pirate.".into(),
    };
    let prompt = effective_system_prompt(&persona);
    assert!(
        prompt.starts_with(RONIN_SYSTEM_PROMPT),
        "append mode must keep the built-in prompt first"
    );
    assert!(
        prompt.contains("Speak like a pirate."),
        "append mode must include custom text"
    );
    assert!(
        prompt.len() > RONIN_SYSTEM_PROMPT.len(),
        "appended prompt should be longer than built-in alone"
    );
}

#[test]
fn effective_system_prompt_should_replace_built_in_when_mode_is_replace() {
    let persona = PersonaConfig {
        mode: PersonaMode::Replace,
        text: "You are a terse code reviewer.".into(),
    };
    let prompt = effective_system_prompt(&persona);
    assert_eq!(prompt, "You are a terse code reviewer.");
    assert!(
        !prompt.contains("You are Ronin"),
        "replace mode must not include the built-in Ronin prompt"
    );
}

#[test]
fn effective_system_prompt_replace_with_empty_text_should_fall_back_to_built_in() {
    let persona = PersonaConfig {
        mode: PersonaMode::Replace,
        text: "   ".into(),
    };
    assert_eq!(effective_system_prompt(&persona), RONIN_SYSTEM_PROMPT);
}

#[test]
fn persona_mode_should_serialize_as_lowercase_toml() {
    let persona = PersonaConfig {
        mode: PersonaMode::Replace,
        text: "custom".into(),
    };
    let toml = toml::to_string(&persona).expect("serialize");
    assert!(toml.contains("mode = \"replace\""));
    assert!(toml.contains("custom"));
}

#[test]
fn persona_config_should_persist_across_session_reload() {
    use ronin_core::{RoninConfig, RoninPaths, RoninSession};
    use tempfile::TempDir;

    let temp = TempDir::new().expect("temp");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    std::fs::create_dir_all(&paths.config_dir).unwrap();

    let session = RoninSession::open(paths.clone()).expect("open");
    session
        .save_config(&RoninConfig {
            persona: PersonaConfig {
                mode: PersonaMode::Append,
                text: "Prefer British spelling.".into(),
            },
            ..RoninConfig::default()
        })
        .expect("save");

    let reloaded = RoninSession::open(paths).expect("reopen");
    let config = reloaded.load_config().expect("load");
    assert_eq!(config.persona.mode, PersonaMode::Append);
    assert_eq!(config.persona.text, "Prefer British spelling.");
    assert_eq!(
        effective_system_prompt(&config.persona),
        format!("{RONIN_SYSTEM_PROMPT}\n\nPrefer British spelling.")
    );
}
