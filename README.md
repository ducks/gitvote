# GitVote

**GitVote** is a Git-native cryptographic voting protocol. Voters submit
signed commits containing vote intent files on designated election branches.
Administrators generate tamper-evident, hash-linked blocks from commit history,
producing a fully auditable and offline-verifiable vote ledger.

## Features

- Each vote is a signed Git commit tied to a cryptographic identity (GPG key)
- Elections live on dedicated branches
- Voters submit votes via simple `gitvote` CLI
- Admin generates canonical blockchain-style ledger from commit history
- Duplicate voting is automatically prevented
- Results are auditable, replayable, and archived into `main` branch

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

## Voter Workflow

1. Clone the election repo
2. Cast a vote
`./gitvote --race president --choice blue`

This:
- Create a new branch
- Writes a vote intent file (e.g. votes/vote-<timestamp>.txt)
- Creates a signed commit
- Prepares the branch for push

3. Push your branch
4. Open a pull request

## Admin Workflow

### Generate Blocks

`./gitvote generate-blocks --branch president`

This scans merged commits, extracts vote intent, and produces canonical
`blocks/000000.json`, `blocks/000001.json`, etc.

### Validate the Chain

`./gitvote validate-chain`

Verifies the integrity of the hash-linked blocks.

### Tally Votes

`./gitvote tally`

Prints vote totals from the finalized blocks.

### Archive Election

Once complete, merge the election branch into `main`:

```
git checkout main
git merge --no-ff president
git push
```

## Vote Intent Format

Voter commits contain files like:

`votes/vote-<timestamp>.txt`

The file content contains the vote choice, e.g.:

`blue`

Voter identity is extracted from the GPG signature on the commit itself.

## Block Format

Generated blocks contain:

```
{
  "index": 1,
  "timestamp": "2025-06-14T22:50:00Z",
  "choice": "blue",
  "voter": "ABC123456789DEF1234567890ABCDEF123456789",
  "prev_hash": "abc123..."
}
```

- `voter` is the GPG key fingerprint extracted from commit signature.
- `prev_hash` links each block to the previous via SHA-256.


## License
MIT
