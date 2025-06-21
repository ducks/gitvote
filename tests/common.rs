use std::process::Command;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};
use std::env;

/// Creates a temporary Git repo and switches to the given branch.
///
/// Returns the `TempDir` (so it lives long enough) and original working directory.
pub fn setup_test_ledger(branch: &str) -> (TempDir, PathBuf) {
    let dir = tempdir().expect("failed to create tempdir");
    let repo_path = dir.path().to_path_buf();
    let original_dir = env::current_dir().expect("failed to get current dir");

    env::set_current_dir(&repo_path).expect("failed to enter tempdir");

    Command::new("git")
        .arg("init")
        .status()
        .expect("failed to init git repo");

    Command::new("git")
        .args(["checkout", "-b", branch])
        .status()
        .expect("failed to create branch");

    (dir, original_dir)
}

