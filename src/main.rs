use clap::{Parser, Subcommand};
use std::error::Error;

mod voting;
mod blocks;
mod validate;
mod tally;
mod doctor;

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

    /// Generate blocks from commits
    GenerateBlocks {
        /// Branch to process
        #[arg(long)]
        branch: String,
    },

    /// Validate the entire chain
    ValidateChain,

    /// Tally votes from existing blocks
    Tally,

    /// Check local GPG and Git environment
    Doctor,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cast { choice } => voting::cast_vote(&choice)?,
        Commands::GenerateBlocks { branch } => blocks::generate_blocks(&branch)?,
        Commands::ValidateChain => validate::validate_chain()?,
        Commands::Tally => tally::tally_votes()?,
        Commands::Doctor => doctor::run_doctor_check()?,
    }

    Ok(())
}
