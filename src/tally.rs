// src/tally.rs

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use crate::block::Block;

pub fn tally_votes() -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<_> = fs::read_dir("blocks")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    entries.sort_by_key(|e| e.path().to_path_buf());

    let mut tally: HashMap<String, u64> = HashMap::new();
    let mut voters: HashMap<String, String> = HashMap::new();

    for entry in &entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)?;
        let block: Block = serde_json::from_str(&content)?;

        if voters.contains_key(&block.voter) {
            continue;
        }

        *tally.entry(block.choice.clone()).or_insert(0) += 1;
        voters.insert(block.voter.clone(), block.choice.clone());
    }

    println!("✔ Tally complete:");
    println!();

    for (choice, count) in &tally {
        println!("{} votes: {}", choice, count);
    }

    println!();
    println!("Total unique voters: {}", voters.len());

    Ok(())
}

