use chrono::Utc;
use std::fs;
use std::process::Command;
use std::error::Error;
use std::path::Path;

/// Casts a vote by writing a vote intent file and signing the commit.
/// Assumes user has already checked out the correct election branch.
pub fn cast_vote(choice: &str) -> Result<(), Box<dyn Error>> {
    // Confirm we are in a Git repo
    if !Path::new(".git").exists() {
        return Err("Not in a Git repository.".into());
    }

    // Check current branch
    let current_branch = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?
            .stdout,
    )?.trim().to_string();

    if current_branch == "HEAD" {
        return Err("Not on a branch (detached HEAD state).".into());
    }

    if current_branch == "main" {
        return Err("Voting on 'main' branch is not allowed.".into());
    }

    println!("✔ Voting on branch: {}", current_branch);

    // Prepare vote file
    let timestamp = Utc::now().timestamp_millis();
    let filename = format!("votes/vote-{}.txt", timestamp);
    fs::create_dir_all("votes")?;
    fs::write(&filename, choice)?;

    // Add file to Git
    let status = Command::new("git")
        .args(["add", &filename])
        .status()?;

    if !status.success() {
        return Err("Failed to add vote file to Git.".into());
    }

    // Commit with signed commit
    let commit_msg = format!("vote: {}", choice);
    let status = Command::new("git")
        .args(["commit", "-S", "-m", &commit_msg])
        .status()?;

    if !status.success() {
        return Err("Failed to create signed commit.".into());
    }

    println!("✔ Vote for '{}' recorded.", choice);
    println!("Don't forget to push your branch and open a PR.");

    Ok(())
}
