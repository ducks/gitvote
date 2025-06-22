use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: DateTime<Utc>,
    pub choice: String,
    pub voter: String,
    pub prev_hash: Option<String>,
    pub hash: String,
    pub signature: String,
}
