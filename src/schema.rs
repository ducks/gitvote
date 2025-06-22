use std::fs;
use std::path::Path;
use std::error::Error;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub allowed: Vec<String>,
}

pub fn load_schema() -> Result<Schema, Box<dyn Error>> {
    let path = Path::new("schema.json");

    if !path.exists() {
        return Err("schema.json not found in proposal branch.".into());
    }

    let content = fs::read_to_string(path)?;
    let schema: Schema = serde_json::from_str(&content)?;

    Ok(schema)
}
