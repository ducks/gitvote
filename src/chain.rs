use std::fs;
use chrono::Utc;
use sha2::{Sha256, Digest};
use std::error::Error;
use crate::block::Block;
use crate::vote::Vote;

pub fn build() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("blocks")?;
    fs::create_dir_all("votes")?;

    let mut entries: Vec<_> = fs::read_dir("votes")?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    entries.sort_by_key(|e| e.path().to_path_buf());
    let mut prev_hash = "GENESIS".to_string();

    for (index, entry) in entries.iter().enumerate() {
        let content = fs::read_to_string(entry.path())?;
        let vote: Vote = serde_json::from_str(&content)?;

        let block = Block {
            index,
            voter: vote.voter,
            choice: vote.choice,
            signature: vote.signature,
            prev_hash: Some(prev_hash.clone()),
            hash: String::new(),
            timestamp: Utc::now(),
        };

        let raw = serde_json::to_string(&block)?;
        let hash = format!("{:x}", Sha256::digest(raw.as_bytes()));

        let finalized = Block { hash: hash.clone(), ..block };
        let file = format!("blocks/block-{:04}.json", index);
        fs::write(&file, serde_json::to_string_pretty(&finalized)?)?;

        prev_hash = hash;
    }

    println!("✔ Chain built with {} blocks.", entries.len());
    Ok(())
}
