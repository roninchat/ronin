//! Isolation path corpus.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::isolated_ronin_paths;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn isolation_dense_many_roots_under_temp() {
    for i in 0..200 {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(format!("iso_{i}"));
        let paths = isolated_ronin_paths(&root).unwrap();
        assert!(paths.config_dir.starts_with(&root));
        assert!(paths.data_dir.starts_with(&root));
        assert!(paths.config_dir.exists());
        assert!(paths.data_dir.exists());
        if let Ok(home) = std::env::var("HOME") {
            let forbidden = PathBuf::from(home).join(".local/share/ronin");
            assert!(!paths.data_dir.starts_with(&forbidden));
        }
    }
}
