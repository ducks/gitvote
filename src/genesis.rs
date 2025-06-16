use crate::block::Block;
use crate::checkout_branch;
use chrono::Utc;
use std::fs;
use std::path::Path;

pub fn create_genesis_block(branch: &str)-> Result<(), Box<dyn std::error::Error>> {
    if branch == "main" {
        return Err("'main' is a protected branch. Cannot create genesis block on it.".into());
    }

    checkout_branch(&branch)?;

    let genesis_path = Path::new("blocks/000000.json");
    if genesis_path.exists() {
        return Err("Genesis block already exists. Aborting.".into());
    }

    let block = Block {
        index: 0,
        timestamp: Utc::now(),
        votes: Vec::new(),
        prev_hash: None,
    };

    let json = serde_json::to_string_pretty(&block)?;
    fs::create_dir_all("blocks")?;
    fs::write(genesis_path, json)?;

    if !Path::new(".git").exists() {
        std::process::Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to init git");
    }



    std::process::Command::new("git")
        .args(["add", "blocks/000000.json"])
        .status()
        .expect("Failed to git add");

    std::process::Command::new("git")
        .args(["commit", "-m", "Genesis block"])
        .status()
        .expect("Failed to git commit");

    Ok(())
}
