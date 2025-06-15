mod block;
mod genesis;
mod vote;
mod voting;

use std::error::Error;
use std::fs;

use clap::Parser;

use crate::genesis::create_genesis_block;
use crate::vote::Vote;
use crate::voting::{ cast_vote, tally_votes, checkout_branch };

#[derive(Parser)]
#[command(name = "gitvote")]
#[command(about = "A toy blockchain built on Git")]
enum Cli {
    CreateGenesis {
        #[arg(long, default_value = "main")]
        branch: String,
    },
    Vote {
        #[arg(long)]
        voter: String,
        #[arg(long)]
        choice: String,
        #[arg(long, default_value = "main")]
        branch: String,
    },
    Tally {
        #[arg(long, default_value = "main")]
        branch: String,
    },
    Sim,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli {
        Cli::CreateGenesis { branch } => {
            checkout_branch(&branch)?;
            genesis::create_genesis_block()?;
        }
        Cli::Vote { voter, choice, branch } => {
            checkout_branch(&branch)?;

            let vote = Vote { voter, choice };

            cast_vote(vote)?;
        }
        Cli::Tally { branch } => {
            checkout_branch(&branch)?;

            let (tally, voters) = tally_votes()?;

            println!("Vote Tally:");
            for (choice, count) in &tally {
                println!("  {choice}: {count}");
            }

            println!("\nWho voted:");
            for (voter, choice) in &voters {
                println!("  {voter} → {choice}");
            }
        }
        Cli::Sim => { run_sim().unwrap() }
    }

    Ok(())
}

pub fn run_sim() -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("blocks").ok(); // clean old chain
    fs::remove_dir_all(".git").ok();

    create_genesis_block()?;

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
        if let Err(e) = cast_vote(vote) {
            println!("✗ Error casting vote: {e}");
        }
    }

    let (tally, voters) = tally_votes()?;

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
