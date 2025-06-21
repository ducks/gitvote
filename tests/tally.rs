// tests/tally.rs

use gitvote::block::Block;
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_tally_votes() {
    let blocks = vec![
        Block {
            index: 0,
            timestamp: Utc::now(),
            choice: "blue".to_string(),
            voter: "voter1".to_string(),
            prev_hash: None,
        },
        Block {
            index: 1,
            timestamp: Utc::now(),
            choice: "red".to_string(),
            voter: "voter2".to_string(),
            prev_hash: Some("dummy".to_string()),
        },
        Block {
            index: 2,
            timestamp: Utc::now(),
            choice: "blue".to_string(),
            voter: "voter3".to_string(),
            prev_hash: Some("dummy".to_string()),
        },
    ];

    let mut tally: HashMap<String, u64> = HashMap::new();

    for block in &blocks {
        *tally.entry(block.choice.clone()).or_insert(0) += 1;
    }

    assert_eq!(*tally.get("blue").unwrap(), 2);
    assert_eq!(*tally.get("red").unwrap(), 1);
}
