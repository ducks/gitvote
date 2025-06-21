use std::{
    collections::{ HashMap, HashSet },
    error::Error,
    fs::{self},
    process::Command,
};

use sha2::{Digest, Sha256};

use chrono::Utc;
use chrono::TimeZone;

use serde_json;
use serde_json::from_str;
use crate::{block::Block};

pub fn read_ordered_blocks() -> Result<Vec<Block>, Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = std::fs::read_dir("blocks")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut blocks = Vec::new();
    for entry in entries {
        let content = std::fs::read_to_string(entry.path())?;
        let block: Block = serde_json::from_str(&content)?;
        blocks.push(block);
    }

    Ok(blocks)
}

/// Get current Git HEAD commit hash
pub fn get_git_commit_hash() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err("Failed to get commit hash".into());
    }

    let hash = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(hash)
}

/// Casts a vote by committing a file with the vote intent.
///
/// The admin will later convert this into a block.
pub fn cast_vote(choice: &str, branch: &str) -> Result<(), Box<dyn Error>> {
    // Safety: only allow voting on election branches, not main
    if branch == "main" {
        return Err("Cannot cast votes directly to main branch.".into());
    }

    // Ensure we are on the correct branch
    let current_branch = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?
            .stdout,
    )?.trim().to_string();

    if current_branch != branch {
        return Err(format!(
            "You are on branch '{}', but voting should happen on '{}'",
            current_branch, branch
        ).into());
    }

    let timestamp = Utc::now().timestamp();
    fs::write(format!("votes/vote-{}.txt", timestamp), choice)?;

    // Add + commit
    Command::new("git")
        .args(["add", "vote.txt"])
        .status()?;

    Command::new("git")
        .args(["commit", "-m", &format!("vote: {}", choice)])
        // If you want GPG signing enforced:
        // .args(["-S"])
        .status()?;

    Ok(())
}

/// Returns (tally: HashMap<choice, count>, voters: HashMap<voter, choice>)
pub fn tally_votes() -> Result<(HashMap<String, u64>, HashMap<String, String>), Box<dyn Error>> {
    let mut tally = HashMap::new();
    let mut voters = HashMap::new();

    // Read all blocks
    let mut entries: Vec<_> = fs::read_dir("blocks")?
        .map(|r| r.unwrap().path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    entries.sort();

    for path in entries {
        let data = fs::read_to_string(&path)?;
        let block: Block = from_str(&data)?;

        // Count only the first vote per voter
        if !voters.contains_key(&block.voter) {
            *tally.entry(block.choice.clone()).or_insert(0) += 1;
            voters.insert(block.voter.clone(), block.choice.clone());
        }
    }

    Ok((tally, voters))
}

pub fn checkout_branch(branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .args(["checkout", branch])
        .status()?;

    if !status.success() {
        // Try to create it
        let status = Command::new("git")
            .args(["checkout", "-b", branch])
            .status()?;
        if !status.success() {
            return Err(format!("Failed to checkout or create branch '{branch}'").into());
        }
    }

    Ok(())
}

pub fn generate_blocks(branch: &str) -> Result<(), Box<dyn Error>> {
    // Confirm we're on the expected branch
    let current_branch = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?
            .stdout,
    )?.trim().to_string();

    if current_branch != branch {
        return Err(format!(
            "You are on branch '{}', not '{}'", current_branch, branch
        ).into());
    }

    fs::create_dir_all("blocks")?;

    let revs = Command::new("git")
        .args(["rev-list", "--reverse", branch])
        .output()?
        .stdout;

    let commits: Vec<String> = String::from_utf8(revs)?
        .lines()
        .map(str::to_string)
        .collect();

    let mut seen_voters = HashSet::new();
    let mut prev_hash = None;
    let mut index = 0;

    for sha in commits {
        // Detect added vote files in this commit
        let diff = Command::new("git")
            .args(["diff-tree", "--no-commit-id", "--name-status", "-r", &sha])
            .output()?
            .stdout;
        let diff_str = String::from_utf8(diff)?;

        let added_votes: Vec<String> = diff_str
            .lines()
            .filter_map(|line| {
                if let Some(path) = line.strip_prefix("A\tvotes/") {
                    Some(format!("votes/{}", path))
                } else {
                    None
                }
            })
            .collect();

        if added_votes.is_empty() {
            continue;
        }

        // Get voter identity
        let voter = String::from_utf8(
            Command::new("git")
                .args(["show", "-s", "--format=%ae", &sha])
                .output()?
                .stdout,
        )?.trim().to_string();

        if seen_voters.contains(&voter) {
            continue;
        }

        // Mark voter as seen
        seen_voters.insert(voter.clone());

        // Get commit timestamp
        let timestamp_raw = Command::new("git")
            .args(["show", "-s", "--format=%ct", &sha])
            .output()?
            .stdout;
        let timestamp = Utc.timestamp(String::from_utf8(timestamp_raw)?.trim().parse()?, 0);

        // Extract first vote file in commit (could support multiple later)
        let path = &added_votes[0];
        let choice = String::from_utf8(
            Command::new("git")
                .args(["show", &format!("{sha}:{path}")])
                .output()?
                .stdout,
        )?.trim().to_string();

        // Build block
        let block = Block {
            index,
            timestamp,
            choice,
            voter,
            prev_hash: prev_hash.clone(),
        };

        // Hash the block
        let json = serde_json::to_string(&block)?;
        let hash = format!("{:x}", Sha256::digest(json.as_bytes()));
        prev_hash = Some(hash);

        // Write block
        let filename = format!("blocks/{:06}.json", index);
        fs::write(&filename, serde_json::to_string_pretty(&block)?)?;
        index += 1;
    }

    println!("✔️ Wrote {index} blocks to blocks/");
    Ok(())
}

pub fn validate_chain() -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<_> = fs::read_dir("blocks")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    entries.sort_by_key(|e| e.path().to_path_buf());

    let mut prev_hash: Option<String> = None;

    for entry in &entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)?;
        let block: Block = serde_json::from_str(&content)?;

        // Hash the block (excluding prev_hash for fairness)
        let raw_json = serde_json::to_string(&block)?;
        let computed_hash = format!("{:x}", Sha256::digest(raw_json.as_bytes()));

        // Check that block.prev_hash matches expected
        if block.index > 0 {
            if block.prev_hash.as_deref() != prev_hash.as_deref() {
                return Err(format!(
                    "Invalid chain at block {} (file: {}): expected prev_hash {:?}, got {:?}",
                    block.index,
                    path.display(),
                    prev_hash,
                    block.prev_hash
                ).into());
            }
        }

        prev_hash = Some(computed_hash);
    }

    println!("✔️ Chain is valid: {} blocks verified.", entries.len());
    Ok(())
}
