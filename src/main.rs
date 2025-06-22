use clap::{Parser, Subcommand};
use std::error::Error;


mod blocks;
mod doctor;
mod git;
mod tally;
mod schema;
mod utils;
mod validate;
mod voting;
mod vote;

#[derive(Parser)]
#[command(name = "gitvote")]
#[command(about = "GitVote - Git-native cryptographic voting system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cast a vote
    Cast {
        /// Your vote choice
        #[arg(long)]
        choice: String,
    },

    /// Validate the entire chain
    Validate,

    /// Tally votes from existing blocks
    Tally,

    /// Check local GPG and Git environment
    Doctor,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cast { choice } => voting::cast_vote(&choice)?,
        Commands::Validate => validate::validate_votes()?,
        Commands::Tally => tally::tally_votes()?,
        Commands::Doctor => doctor::run_doctor_check()?,
    }

    Ok(())
}
