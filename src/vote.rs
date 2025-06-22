use chrono::{ DateTime, Utc };

use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    /// The voter's ID (e.g. username or public key)
    pub voter: String,

    /// Their selected choice (e.g. a candidate or option)
    pub choice: String,

    pub signature: String,
    pub timestamp: DateTime<Utc>,
}
