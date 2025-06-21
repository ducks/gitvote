use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn full_multi_voter_protocol() {
    // Locate gitvote binary
    let binary_path = std::fs::canonicalize("./target/release/gitvote")
        .expect("Build gitvote first: cargo build --release");

    // Create central bare repo
    let central_repo = TempDir::new().unwrap();
    run("git init --bare", central_repo.path());

    // Simulate 3 distinct voters
    let voter1 = TestVoter::new("alice", central_repo.path(), &binary_path);
    let voter2 = TestVoter::new("bob", central_repo.path(), &binary_path);
    let voter3 = TestVoter::new("carol", central_repo.path(), &binary_path);

    voter1.cast_vote("blue");
    run(
        "git symbolic-ref HEAD refs/heads/president",
        central_repo.path(),
    );
    voter2.cast_vote("red");
    voter3.cast_vote("blue");

    // Admin: Clone bare repo into working tree to run admin tools
    let admin_dir = TempDir::new().unwrap();
    run(
        &format!("git clone {} {}", central_repo.path().display(), admin_dir.path().display()),
        Path::new("."),
    );

    // Run full admin flow
    run_in(
        &format!("{} generate-blocks --branch president", binary_path.display()),
        admin_dir.path(),
    );

    run_in(
        &format!("{} validate-chain", binary_path.display()),
        admin_dir.path(),
    );

    run_in(
        &format!("{} tally", binary_path.display()),
        admin_dir.path(),
    );

    // Verify 3 blocks created
    let blocks_path = admin_dir.path().join("blocks");
    let count = fs::read_dir(&blocks_path)
        .unwrap()
        .filter(|e| e.as_ref().unwrap().path().extension().unwrap() == "json")
        .count();

    assert_eq!(count, 3, "expected 3 blocks written");
}

/// Fully isolated simulated voter
struct TestVoter {
    name: String,
    gpg_dir: TempDir,
    git_dir: TempDir,
    key_id: String,
    binary_path: PathBuf,
}

impl TestVoter {
    fn new(name: &str, central_repo: &Path, binary_path: &Path) -> Self {
        let gpg_dir = TempDir::new().unwrap();
        fs::set_permissions(gpg_dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let git_dir = TempDir::new().unwrap();

        let key_id = generate_gpg_key(name, gpg_dir.path());

        // Clone central repo (simulate fork)
        run(
            &format!("git clone {} {}", central_repo.display(), git_dir.path().display()),
            Path::new("."),
        );

        // Create election branch inside clone
        run("git checkout -b president", git_dir.path());
        run("git commit --allow-empty -m init", git_dir.path());
        run("git push -u origin president", git_dir.path());

        // Configure Git for this voter
        run_in(&format!("git config user.name {}", name), git_dir.path());
        run_in(&format!("git config user.signingkey {}", key_id), git_dir.path());
        run_in("git config commit.gpgsign true", git_dir.path());

        // Create GPG wrapper script
        let wrapper_path = git_dir.path().join("gpg-wrapper.sh");
        let wrapper_content = format!(
            "#!/bin/sh\nexec gpg --homedir {} \"$@\"",
            gpg_dir.path().display()
        );
        fs::write(&wrapper_path, wrapper_content).unwrap();
        run_in(&format!("chmod +x {}", wrapper_path.display()), git_dir.path());

        // Point Git to wrapper
        run_in(&format!("git config gpg.program {}", wrapper_path.display()), git_dir.path());

        Self {
            name: name.to_string(),
            gpg_dir,
            git_dir,
            key_id,
            binary_path: binary_path.to_path_buf(),
        }
    }

    fn cast_vote(&self, choice: &str) {
        run_in(
            &format!("{} cast --choice {}", self.binary_path.display(), choice),
            self.git_dir.path(),
        );

        run_in("git pull --rebase", self.git_dir.path());
        run_in("git push --set-upstream origin president", self.git_dir.path());
    }
}

fn generate_gpg_key(name: &str, homedir: &Path) -> String {
    let batch = format!(
        "Key-Type: RSA
Key-Length: 2048
Name-Real: {name}
Name-Email: {name}@example.com
Expire-Date: 0
%no-protection
%commit"
    );

    let batch_file = homedir.join("batch");
    fs::write(&batch_file, batch).unwrap();

    let status = Command::new("gpg")
        .args([
            "--batch", "--generate-key",
            "--homedir", homedir.to_str().unwrap(),
            batch_file.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success(), "GPG key generation failed");

    let output = Command::new("gpg")
        .args([
            "--homedir", homedir.to_str().unwrap(),
            "--list-keys",
            "--with-colons",
        ])
        .output()
        .unwrap();

    let out = String::from_utf8(output.stdout).unwrap();
    for line in out.lines() {
        if line.starts_with("fpr:") {
            let parts: Vec<_> = line.split(':').collect();
            return parts[9].to_string();
        }
    }
    panic!("Failed to extract GPG key ID");
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
        .unwrap();

    assert!(status.success(), "Command failed: {}", cmd);
}

fn run_in(cmd: &str, dir: &Path) {
    run(cmd, dir);
}

