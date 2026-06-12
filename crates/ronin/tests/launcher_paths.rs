use ronin::ronin_paths_from_env;

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
