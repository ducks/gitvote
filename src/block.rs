use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::vote::Vote;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub votes: Vec<Vote>,
    pub prev_hash: Option<String>,
}
