# GitVote

**GitVote** is a Git-native voting ledger where each vote is a Git commit.
Votes are stored in structured JSON files, tracked in branches that represent elections.

## Features

- Each vote is a verifiable Git commit
- One vote per voter enforced
- Elections live on their own branches
- Votes are tallied by replaying commit history
- Results are mergeable back to `main` for archival

---

## Getting Started

### Clone the Repo

```bash
git clone git@github.com/ducks/gitvote.git
cd gitvote
```

### Build the CLI
```
cargo build --release
./target/release/gitvote --help
```

## Commands

### Create a New Election
`./gitvote create-genesis --branch president`
Creates a `blocks/000000.json` file and commits it to the `president` branch.

### Cast a Vote
`./gitvote cast --voter alice --choice blue --branch president`
Adds a vote block for `alice`, commits it, and links it to the previous commit.

### Tally Votes
`./gitvote tally --branch president`
Prints a summary of vote counts and who voted for what.

## Flow Example

```
# Start a new election
./gitvote create-genesis --branch referendum-a

# Cast votes
./gitvote cast --voter alice --choice yes --branch referendum-a
./gitvote cast --voter bob --choice no --branch referendum-a

# View results
./gitvote tally --branch referendum-a

# Merge completed vote history into main
git checkout main
git merge referendum-a --no-ff
git push
```

## Vote Format
Each vote is recorded in a block file such as `blocks/000001.json`:

```
{
  "index": 1,
  "timestamp": "2025-06-14T22:50:00Z",
  "votes": [
    {
      "voter": "alice",
      "choice": "blue"
    }
  ],
  "prev_hash": "abc123..."
}
```

## License
MIT
