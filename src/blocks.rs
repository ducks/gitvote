// src/blocks.rs

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::process::Command;
use sha2::{Digest, Sha256};
use chrono::{TimeZone, Utc};
use crate::block::Block;

pub fn generate_blocks(branch: &str) -> Result<(), Box<dyn Error>> {
    // Ensure we are on the correct branch
    let current_branch = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?
            .stdout,
    )?.trim().to_string();

    if current_branch != branch {
        return Err(format!(
            "You are on branch '{}', expected '{}'", current_branch, branch
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

        let voter = extract_gpg_fingerprint(&sha)?;

        if seen_voters.contains(&voter) {
            continue;
        }

        seen_voters.insert(voter.clone());

        let timestamp_raw = Command::new("git")
            .args(["show", "-s", "--format=%ct", &sha])
            .output()?
            .stdout;

        let timestamp = Utc.timestamp(
            String::from_utf8(timestamp_raw)?.trim().parse()?,
            0
        );

        let path = &added_votes[0];
        let choice = String::from_utf8(
            Command::new("git")
                .args(["show", &format!("{sha}:{path}")])
                .output()?
                .stdout,
        )?.trim().to_string();

        let block = Block {
            index,
            timestamp,
            choice,
            voter,
            prev_hash: prev_hash.clone(),
        };

        let json = serde_json::to_string(&block)?;
        let hash = format!("{:x}", Sha256::digest(json.as_bytes()));

        prev_hash = Some(hash);

        let filename = format!("blocks/{:06}.json", index);
        fs::write(&filename, serde_json::to_string_pretty(&block)?)?;

        index += 1;
    }

    println!("✔ {} blocks written to 'blocks/'", index);
    Ok(())
}

fn extract_gpg_fingerprint(commit_sha: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["log", "--show-signature", "-1", commit_sha])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    for line in stdout.lines() {
        if let Some(fingerprint) = line.strip_prefix("gpg:                using ") {
            let parts: Vec<_> = fingerprint.split_whitespace().collect();

            if parts.len() >= 3 && parts[0] == "RSA" && parts[1] == "key" {
                return Ok(parts[2].to_string());
            }
        }
    }

    Err(format!("No GPG key found on commit {commit_sha}").into())
}

