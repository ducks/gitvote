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
        /// The election race (Git branch)
        #[arg(long)]
        race: String,
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
        Commands::Cast { race, choice } => cast::cast_vote(&race, &choice)?,
        Commands::GenerateBlocks { branch } => blocks::generate_blocks(&branch)?,
        Commands::ValidateChain => validate::validate_chain()?,
        Commands::Tally => tally::tally_votes()?,
        Commands::Doctor => doctor::run_doctor_check()?,
    }

    Ok(())
}

pub fn run_sim() -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("blocks").ok(); // clean old chain
    fs::remove_dir_all(".git").ok();

    create_genesis_block("sim")?;

    for (voter, choice) in [
        ("alice", "blue"),
        ("bob", "red"),
        ("carol", "blue"),
        ("dave", "green"),
        ("alice", "red"), // duplicate, will be rejected
    ] {
        let vote = Vote {
            voter: voter.to_string(),
            choice: choice.to_string(),
        };
        if let Err(e) = cast_vote(vote, "sim") {
            println!("✗ Error casting vote: {e}");
        }
    }

    let (tally, voters) = tally_votes("sim")?;

    println!("\nVote tally:");
    for (choice, count) in tally {
        println!("{choice}: {count}");
    }

    println!("\nVoters:");
    for (voter, choice) in voters {
        println!("{voter} voted for {choice}");
    }

    Ok(())
}
