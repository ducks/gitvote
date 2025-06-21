use chrono::Utc;
use std::fs;
use std::process::Command;
use std::error::Error;

/// Casts a vote by writing a vote intent file and signing the commit.
pub fn cast_vote(race: &str, choice: &str) -> Result<(), Box<dyn Error>> {
    // Confirm we are in a Git repo
    if !std::path::Path::new(".git").exists() {
        return Err("Not in a Git repository.".into());
    }

    // Checkout the race branch
    let status = Command::new("git")
        .args(["checkout", race])
        .status()?;
    if !status.success() {
        return Err(format!("Failed to checkout branch '{}'", race).into());
    }

    // Prepare vote file
    let timestamp = Utc::now().timestamp();
    let filename = format!("votes/vote-{}.txt", timestamp);

    fs::create_dir_all("votes")?;
    fs::write(&filename, choice)?;

    // Add vote file to Git
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

    println!("Vote cast successfully for '{}'", choice);
    println!("Don't forget to push your branch!");

    Ok(())
}
