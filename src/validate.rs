use std::fs;
use std::error::Error;
use crate::schema::load_schema;
use std::path::Path;
use crate::vote::Vote;
use crate::utils::generate_fake_signature;


pub fn validate_votes() -> Result<(), Box<dyn Error>> {
    let schema = load_schema()?;
    let votes_path = Path::new("votes");

    if !votes_path.exists() {
        println!("No votes to validate.");
        return Ok(());
    }

    let mut voters = vec![];

    for entry in fs::read_dir(votes_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        let vote: Vote = serde_json::from_str(&content)?;

        if !schema.allowed.contains(&vote.choice) {
            return Err(format!("Invalid choice '{}' in {:?}", vote.choice, path).into());
        }

        if voters.contains(&vote.voter) {
            return Err(format!("Duplicate vote by voter: {}", vote.voter).into());
        }

        // Validate signature
        let expected_sig = generate_fake_signature(&vote.voter, &vote.choice);

        if vote.signature != expected_sig {
            return Err(format!("Signature mismatch for voter {}", vote.voter).into());
        }

        voters.push(vote.voter);
    }

    println!("✔ All votes are valid.");
    Ok(())
}
