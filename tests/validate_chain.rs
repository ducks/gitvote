use gitvote::block::Block;
use chrono::Utc;
use sha2::{Digest, Sha256};

#[test]
fn test_valid_chain() {
    let mut blocks = Vec::new();
    let mut prev_hash = None;

    for i in 0..3 {
        let block = Block {
            index: i,
            timestamp: Utc::now(),
            choice: if i % 2 == 0 { "blue".to_string() } else { "red".to_string() },
            voter: format!("voter-{}", i),
            prev_hash: prev_hash.clone(),
        };

        let raw_json = serde_json::to_string(&block).unwrap();
        let hash = format!("{:x}", Sha256::digest(raw_json.as_bytes()));

        prev_hash = Some(hash);
        blocks.push(block);
    }

    // Now validate
    let mut prev = None;
    for block in &blocks {
        let raw_json = serde_json::to_string(&block).unwrap();
        let hash = format!("{:x}", Sha256::digest(raw_json.as_bytes()));

        if block.index > 0 {
            assert_eq!(block.prev_hash, prev);
        }
        prev = Some(hash);
    }
}
