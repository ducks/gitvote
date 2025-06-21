use std::fs;
use std::error::Error;
use sha2::{ Digest, Sha256 };
use gitvote::block::Block;

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

        let raw_json = serde_json::to_string(&block)?;
        let computed_hash = format!("{:x}", Sha256::digest(raw_json.as_bytes()));

        if block.index > 0 {
            if block.prev_hash.as_deref() != prev_hash.as_deref() {
                return Err(format!(
                    "Chain broken at block {}: expected prev_hash {:?}, got {:?}",
                    block.index,
                    prev_hash,
                    block.prev_hash
                ).into());
            }
        }

        prev_hash = Some(computed_hash);
    }

    println!("✔ Chain is valid: {} blocks verified.", entries.len());
    Ok(())
}
