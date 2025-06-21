use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn full_end_to_end_flow_with_real_cli() {

    let binary_path = std::fs::canonicalize("./target/release/gitvote")
        .expect("You must build the binary first (cargo build --release)");

    // Create temp directory
    let tmp_dir = TempDir::new().expect("failed to create temp dir");
    let repo_path = tmp_dir.path();


    // Init Git repo
    run("git init", repo_path);
    run("git checkout -b president", repo_path);

    // Cast 3 votes using the real gitvote binary
    cast_vote(repo_path, "president", "blue", &binary_path);
    cast_vote(repo_path, "president", "red", &binary_path);
    cast_vote(repo_path, "president", "blue", &binary_path);

    // Generate blocks
    run(
        "./target/release/gitvote generate-blocks --branch president",
        repo_path,
    );

    // Validate chain
    run(
        "./target/release/gitvote validate-chain",
        repo_path,
    );

    // Tally votes
    run(
        "./target/release/gitvote tally",
        repo_path,
    );

    // Assert blocks written
    let blocks_path = repo_path.join("blocks");
    assert!(blocks_path.exists());

    let block_count = fs::read_dir(blocks_path)
        .unwrap()
        .filter(|e| e.as_ref().unwrap().path().extension().unwrap() == "json")
        .count();

    // We expect 3 blocks since 3 votes were cast
    assert_eq!(block_count, 3);
}

fn cast_vote(repo_path: &Path, race: &str, choice: &str, binary_path: &Path) {
    run(
        &format!("{} --race {} --choice {}", binary_path.display(), race, choice),
        repo_path,
    );
}

fn run(cmd: &str, dir: &Path) {
    let mut parts = cmd.split_whitespace();
    let bin = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();
    println!("{}", bin);

    let status = Command::new(bin)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("failed to run command");

    assert!(status.success(), "command failed: {}", cmd);
}
