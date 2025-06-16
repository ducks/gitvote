use std::{
    collections::{ HashMap, HashSet },
    error::Error,
    fs::{self, File},
    io::BufReader,
    process::Command,
};

use chrono::Utc;
use serde_json;
use crate::{block::Block, vote::Vote};

/// Load all blocks sorted by index
pub fn load_chain() -> Result<Vec<Block>, Box<dyn Error>> {
    let mut blocks = vec![];

    for entry in fs::read_dir("blocks")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let block: Block = serde_json::from_reader(reader)?;
        blocks.push(block);
    }

    blocks.sort_by_key(|b| b.index);
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

/// Cast a new vote (fails if voter has already voted)
pub fn cast_vote(vote: Vote, branch: &str) -> Result<(), Box<dyn Error>> {
    if branch == "main" {
        return Err("'main' is a protected branch. Cannot create genesis block on it.".into());
    }

    checkout_branch(&branch)?;

    let chain = load_chain()?;
    let latest = chain.last().ok_or("No genesis block found")?;

    // Check for duplicate vote
    let mut seen = HashSet::new();
    for block in &chain {
        for v in &block.votes {
            if !seen.insert(&v.voter) && v.voter == vote.voter {
                return Err(format!("Voter '{}' has already voted", vote.voter).into());
            }
        }
    }

    // Build new block
    let index = latest.index + 1;
    let prev_hash = get_git_commit_hash()?;
    let new_block = Block {
        index,
        timestamp: Utc::now(),
        votes: vec![vote],
        prev_hash: Some(prev_hash),
    };

    // Write block to file
    let path = format!("blocks/{:06}.json", index);
    let json = serde_json::to_string_pretty(&new_block)?;
    fs::write(&path, json)?;

    // Git commit
    Command::new("git").args(["add", &path]).status()?.success()
        .then_some(())
        .ok_or("Failed to git add")?;

    Command::new("git")
        .args(["commit", "-m", &format!("Vote block {index}")])
        .status()?
        .success()
        .then_some(())
        .ok_or("Failed to git commit")?;

    Ok(())
}



/// Returns (tally: HashMap<choice, count>, voters: HashMap<voter, choice>)
pub fn tally_votes(branch: &str) -> Result<(HashMap<String, u64>, HashMap<String, String>), Box<dyn Error>> {
    checkout_branch(&branch)?;

    let chain = load_chain()?;
    let mut tally = HashMap::new();
    let mut voters = HashMap::new();
    let mut seen = HashSet::new();

    for block in &chain {
        for vote in &block.votes {
            if seen.insert(&vote.voter) {
                *tally.entry(vote.choice.clone()).or_insert(0) += 1;
                voters.insert(vote.voter.clone(), vote.choice.clone());
            }
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
