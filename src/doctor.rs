use std::error::Error;
use std::process::Command;
use std::path::Path;

pub fn run_doctor_check() -> Result<(), Box<dyn Error>> {
    println!("🩺 GitVote Doctor Check");
    println!("------------------------");

    // Check for Git repo
    if !Path::new(".git").exists() {
        return Err("Not in a Git repository.".into());
    }
    println!("✔ Git repository detected.");

    // Check for GPG signing key configured
    let signing_key = Command::new("git")
        .args(["config", "--get", "user.signingkey"])
        .output()?;

    let signing_key = String::from_utf8(signing_key.stdout)?.trim().to_string();

    if signing_key.is_empty() {
        return Err("❌ No GPG signing key configured.".into());
    }

    println!("✔ GPG signing key configured: {}", signing_key);

    // Check if commit signing is enabled
    let gpgsign = Command::new("git")
        .args(["config", "--get", "commit.gpgsign"])
        .output()?;

    let gpgsign = String::from_utf8(gpgsign.stdout)?.trim().to_string();

    if gpgsign != "true" {
        return Err("❌ Git commit signing is not enabled (commit.gpgsign != true).".into());
    }

    println!("✔ Git commit signing enabled.");

    // Optional: try signing a dry-run commit to verify full signing works
    let dry_run = Command::new("git")
        .args(["commit", "--allow-empty", "--dry-run", "-S", "-m", "test"])
        .output()?;

    if !dry_run.status.success() {
        return Err("❌ GPG signing failed during dry-run commit.".into());
    }

    println!("✔ Dry-run commit signing successful.");
    println!();
    println!("✅ GitVote environment looks good. You are ready to vote!");

    Ok(())
}
