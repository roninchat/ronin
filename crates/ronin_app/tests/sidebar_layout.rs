//! Shell APIs for sidebar width preference and collapse state.

use ronin_app::RoninShell;
use ronin_core::{
    clamp_sidebar_width, RoninPaths, SIDEBAR_WIDTH_DEFAULT, SIDEBAR_WIDTH_MAX, SIDEBAR_WIDTH_MIN,
};
use tempfile::TempDir;

fn open_shell() -> (TempDir, RoninShell) {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let shell = RoninShell::open(paths).expect("open shell");
    (temp, shell)
}

#[test]
fn shell_should_expose_default_sidebar_layout() {
    let (_temp, shell) = open_shell();
    assert_eq!(shell.sidebar_width(), SIDEBAR_WIDTH_DEFAULT);
    assert!(!shell.sidebar_collapsed());
}

#[test]
fn shell_set_sidebar_width_should_clamp_and_persist() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths.clone()).expect("open shell");

    shell
        .set_sidebar_width(SIDEBAR_WIDTH_MAX + 80.0)
        .expect("set width");
    assert_eq!(shell.sidebar_width(), SIDEBAR_WIDTH_MAX);

    shell
        .set_sidebar_width(SIDEBAR_WIDTH_MIN - 50.0)
        .expect("set width");
    assert_eq!(shell.sidebar_width(), SIDEBAR_WIDTH_MIN);

    shell.set_sidebar_width(340.0).expect("set width");
    assert_eq!(shell.sidebar_width(), 340.0);

    let reopened = RoninShell::open(paths).expect("reopen");
    assert_eq!(reopened.sidebar_width(), clamp_sidebar_width(340.0));
}

#[test]
fn shell_toggle_sidebar_collapsed_should_persist() {
    let temp = TempDir::new().expect("temp dir");
    let paths = RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    };
    let mut shell = RoninShell::open(paths.clone()).expect("open shell");

    assert!(!shell.sidebar_collapsed());
    let collapsed = shell.toggle_sidebar_collapsed().expect("toggle");
    assert!(collapsed);
    assert!(shell.sidebar_collapsed());

    let reopened = RoninShell::open(paths).expect("reopen");
    assert!(reopened.sidebar_collapsed());
    assert_eq!(reopened.sidebar_width(), SIDEBAR_WIDTH_DEFAULT);
}
