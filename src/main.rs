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
            genesis::create_genesis_block(&branch)?;
        }
        Cli::Vote { voter, choice, branch } => {
            let vote = Vote { voter, choice };

            cast_vote(vote, &branch)?;
        }
        Cli::Tally { branch } => {
            let (tally, voters) = tally_votes(&branch)?;

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
