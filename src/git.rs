use std::process::Command;
use std::error::Error;

pub fn get_git_voter() -> Result<String, Box<dyn Error>> {
    let name = String::from_utf8(
        Command::new("git").args(["config", "user.name"]).output()?.stdout
    )?.trim().to_string();

    let email = String::from_utf8(
        Command::new("git").args(["config", "user.email"]).output()?.stdout
    )?.trim().to_string();

    if name.is_empty() || email.is_empty() {
        return Err("Git user.name or user.email not configured.".into());
    }

    Ok(format!("{} <{}>", name, email))
}
