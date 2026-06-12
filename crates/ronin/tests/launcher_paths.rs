use ronin::{parse_launch_intent, ronin_paths_from_env, LaunchIntent};

#[test]
fn parse_launch_intent_should_return_new_thread_when_new_flag_is_present() {
    let intent = parse_launch_intent(["--new"]).expect("parse --new");

    assert_eq!(intent, LaunchIntent::NewThread);
}

#[test]
fn parse_launch_intent_should_reject_unsupported_flags() {
    let error = parse_launch_intent(["--unknown"]).expect_err("unsupported flag");

    assert_eq!(
        error.to_string(),
        "unsupported launch flag '--unknown'. supported flags: --new"
    );
}

#[test]
fn ronin_should_exit_nonzero_when_unsupported_flag_is_passed() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ronin"))
        .arg("--unknown")
        .output()
        .expect("run ronin with unsupported flag");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("unsupported launch flag '--unknown'. supported flags: --new"));
}

#[test]
fn ronin_paths_should_use_xdg_dirs_when_present() {
    let paths = ronin_paths_from_env(
        Some("/tmp/ronin-config"),
        Some("/tmp/ronin-data"),
        Some("/home/ronin"),
    )
    .expect("paths from xdg env");

    assert_eq!(
        paths.config_dir.to_string_lossy(),
        "/tmp/ronin-config/ronin"
    );
    assert_eq!(paths.data_dir.to_string_lossy(), "/tmp/ronin-data/ronin");
}
