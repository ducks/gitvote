use chrono::Utc;
use std::fs;
use std::process::Command;
use std::error::Error;
use std::path::Path;
use uuid::Uuid;
use crate::vote::Vote;
use crate::git::get_git_voter;
use crate::utils::generate_fake_signature;
use crate::schema::load_schema;

/// Casts a vote by writing a vote intent file and signing the commit.
/// Assumes user has already checked out the correct election branch.
pub fn cast_vote(choice: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(".git").exists() {
        return Err("Not inside a git repo.".into());
    }

    let schema = load_schema()?;
    if !schema.allowed.contains(&choice.to_string()) {
        return Err(format!("Invalid choice '{}'. Allowed: {:?}", choice, schema.allowed).into());
    }

    let voter = get_git_voter()?;
    let signature = generate_fake_signature(&voter, choice);

    fs::create_dir_all("votes")?;
    let filename = format!("votes/vote-{}.json", Uuid::new_v4());

    let timestamp = Utc::now();

    let vote = Vote {
        voter,
        choice: choice.to_string(),
        signature,
        timestamp,
    };
    let json = serde_json::to_string_pretty(&vote)?;
    fs::write(&filename, json)?;

    Command::new("git").args(["add", &filename]).status()?;
    Command::new("git").args(["commit", "-m", &format!("vote: {}", choice)]).status()?;

    println!("✔ Vote recorded as {}", filename);
    Ok(())
}
