mod common;

use common::setup_test_ledger;

use gitvote::voting::{cast_vote, tally_votes};
use gitvote::vote::Vote;
use gitvote::genesis::create_genesis_block;

#[test]
fn simulate_small_election() {
    // Setup isolated temp Git repo
    let (dir, original_dir) = setup_test_ledger("president");

    // Create genesis block
    create_genesis_block("president").unwrap();

    // Simulate votes
    let votes = vec![
        Vote { voter: "alice".into(), choice: "blue".into() },
        Vote { voter: "bob".into(), choice: "red".into() },
        Vote { voter: "carol".into(), choice: "blue".into() },
    ];

    for vote in votes {
        cast_vote(vote, "test-election").unwrap();
    }

    // Tally results
    let (tally, voters) = tally_votes("test-election").unwrap();

    assert_eq!(tally.get("blue"), Some(&2));
    assert_eq!(tally.get("red"), Some(&1));
    assert_eq!(voters.len(), 3);
    assert_eq!(voters.get("alice").unwrap(), "blue");

    std::env::set_current_dir(original_dir).unwrap();
    drop(dir);
}
