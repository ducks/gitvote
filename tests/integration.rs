use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn full_end_to_end_flow_with_real_cli() {
    // Create temp directory
    let tmp_dir = TempDir::new().expect("failed to create temp dir");
    let repo_path = tmp_dir.path();

    // Find the gitvote binary (always absolute path to avoid path issues)
    let binary_path = std::fs::canonicalize("./target/release/gitvote")
        .expect("You must build the binary first: cargo build --release");

    // Init Git repo
    run("git init", repo_path);
    run("git checkout -b president", repo_path);

    run("git commit --allow-empty -m init", repo_path);

    // Cast 3 votes using the real gitvote binary
    cast_vote(repo_path, &binary_path, "blue");
    cast_vote(repo_path, &binary_path, "red");
    cast_vote(repo_path, &binary_path, "blue");

    // Generate blocks
    run(
        &format!("{} generate-blocks --branch president", binary_path.display()),
        repo_path,
    );

    // Validate chain
    run(
        &format!("{} validate-chain", binary_path.display()),
        repo_path,
    );

    // Tally votes
    run(
        &format!("{} tally", binary_path.display()),
        repo_path,
    );

    // Assert blocks written
    let blocks_path = repo_path.join("blocks");
    assert!(blocks_path.exists());

    let block_count = fs::read_dir(blocks_path)
        .unwrap()
        .filter(|e| e.as_ref().unwrap().path().extension().unwrap() == "json")
        .count();

    assert_eq!(block_count, 3);
}

fn cast_vote(repo_path: &Path, binary_path: &Path, choice: &str) {
    run(
        &format!("{} cast --choice {}", binary_path.display(), choice),
        repo_path,
    );
}

fn run(cmd: &str, dir: &Path) {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    let status = Command::new(bin)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("failed to run command");

    assert!(status.success(), "command failed: {}", cmd);
}
