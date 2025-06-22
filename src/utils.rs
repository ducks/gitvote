use sha2::{Sha256, Digest};

pub fn generate_fake_signature(voter: &str, choice: &str) -> String {
    let raw = format!("{}:{}", voter, choice);
    format!("{:x}", Sha256::digest(raw.as_bytes()))
}
