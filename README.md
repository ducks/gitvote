# GitVote

**GitVote** is a Git-native cryptographic voting protocol.

Voters submit votes via pull requests on dedicated proposal branches. The
system produces an immutable, hash-linked, fully auditable vote ledger, backed
entirely by Git and simple files.

## Features

- Voters submit votes via simple gitvote CLI
- Each voter is uniquely identified via their Git identity (user.name + user.email)
- Duplicate voting is automatically prevented
- Votes are submitted as pull requests
- CI system automatically validates votes before merging
- After merges, CI builds tamper-evident hash-linked blocks
- Full auditability: results can be independently verified offline

---

## Getting Started

### Clone the Repo for the CLI tool

```bash
git clone git@github.com/ducks/gitvote.git
cd gitvote
cargo build --release
```

The CLI will be available at:
```
./target/release/gitvote --help
```

## Voter Workflow

1. Fork the voting repo
- Visit repo (e.g. https://github.com/ducks/gitvote-gov)
- Click **Fork** (top right corner in Github UI)

2. Clone your fork locally
```
git clone git@github.com:YOU/gitvote-gov.git
cd gitvote-gov
git checkout proposal/001-color-vote
```

3. Cast your vote

Run CLI from your built `gitvote` binary:

`/path/to/gitvote/target/release/gitvote cast --choice purple`

This will:
- Write your vote file into votes/
- Commit your vote using your Git identity
- (Optionally) Sign the commit if GPG signing is enabled

4. Push your vote to your fork

`git push origin proposal/001-color-vote`

5. Open a pull request

- On GitHub, open a PR from your fork’s proposal/001-color-vote branch
into the upstream proposal/001-color-vote branch.
- CI will automatically validate your vote.

## Admin Workflow

### Validate Votes (runs automatically via CI)

Each PR triggers:

`gitvote validate`

Which verifies:
- Vote file format
- Schema compliance (`schema.json`)
- Duplication prevention (unique voters only)

### Build Immutable Blocks

After valid PRs are merged, CI automatically runs:

`gitvote build-chain`

This creates immutable hash-linked blocks:

```
blocks/block-0000.json
blocks/block-0001.json
```

### Tally Results

At any time, you can generate vote tallies from the proposal branch:

`gitvote tally`

Example results:

```
Vote Tally:
  purple votes: 3
  red votes: 2
Total unique voters: 5
```

## Vote Intent Format

Vote files are written to:

`votes/vote-<uuid>.json`

Example file contents:

```
{
  "voter": "Alice Voter <alice@example.com>",
  "choice": "blue",
  "timestamp": "2025-06-22T23:55:41Z",
  "signature": "a7490554...fake-sig"
}
```

## Block Format

Generated blocks contain:

```
{
  "index": 1,
  "timestamp": "2025-06-22T23:55:41Z",
  "choice": "blue",
  "voter": "Alice Voter <alice@example.com>",
  "prev_hash": "abc123...",
  "hash": "def456...",
  "signature": "a7490554..."
}
```

- Each block includes the vote, timestamp, voter identity, and hash chain linkage.
- The full chain is immutable and verifiable offline.


## License
MIT
