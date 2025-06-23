# GitVote: Git-Native Cryptographic Voting Protocol

## Overview

**GitVote** is a decentralized, verifiable voting protocol built entirely on top of Git.
It leverages Git’s distributed, append-only nature and hash-linked file structure to enable tamper-resistant, auditable elections without requiring blockchains, servers, or databases.

## Core Principles

- Tamper-evident: All votes are immutable Git commits
- Identity via Git username and email (Git config)
- Decentralized submission: Voters submit votes via Git pull requests
- Centralized finalization: Admin-controlled canonical chain assembly
- Publicly auditable: Entire vote chain is openly cloneable and replayable
- No off-chain data required: Entire election lives entirely in the Git repo

## Protocol Flow

### Election Initialization

- Admin creates a dedicated Git repo for elections.
- For each ballot/race, a separate branch is created (e.g. `proposal/001-color-vote`).

### Voter Distribution

- Voters receive:
  - Repo URL
  - Assigned proposal branch names
  - Voting window timeline
  - Setup instructions (GitVote CLI tool)

### Voter Preparation

- Voter forks the election repo on GitHub.
- Voter clones their fork locally.
- Voter checks out assigned proposal branch.
- Voter ensures Git config is correctly set (`user.name`, `user.email`).
- Voter verifies setup via `gitvote doctor` command.

### Casting a Vote

- Voter runs:

    gitvote cast --choice <vote>

- The `gitvote` tool:
  - Writes a `votes/vote-<uuid>.json` file containing:
    - voter identity (Git config)
    - choice
    - timestamp
    - simulated signature (for future crypto extensibility)
  - Commits the file (optionally signed via GPG if configured)
  - Prepares the branch for push

### Submitting Vote for Validation

- Voter pushes branch to their fork.
- Voter opens a pull request targeting the proposal branch in the upstream repo.
- CI system automatically runs validation pipeline.

## Validation Pipeline (CI)

Upon PR creation or push:

1. Validate vote intent file structure:
    - Only one `votes/` file added per PR
    - Valid vote contents (allowed choices from `schema.json`)
2. Validate voter identity:
    - Extracted from vote file's `voter` field (Git username/email)
3. Check for duplicate votes:
    - Voter has not previously voted on this proposal
4. Chain Building:
    - Run `gitvote build-chain` to construct `blocks/block-0000.json`, `block-0001.json`, etc.
5. Chain Validation:
    - Run `gitvote validate` to verify correct hash chaining

PR fails if any validation fails.
PR is eligible for merge if validation passes.

## Chain Generation Logic

- CI tool constructs blocks from merged vote files.
- Each block includes:
  - `index` — sequential block number
  - `timestamp` — vote timestamp
  - `voter` — Git username and email
  - `choice` — vote content
  - `prev_hash` — hash of previous block
  - `hash` — current block hash
- Block hashes are computed as SHA-256 over the full block content.

## Election Finalization

At voting close:

- Admin reviews and merges all passing PRs into the proposal branch.
- Admin (or CI) runs final `gitvote build-chain` and `gitvote validate`.
- Admin tallies votes via `gitvote tally`.

## Archiving Election History

Once the proposal branch is finalized:

- Admin merges proposal branch into `main` for permanent record:

    git checkout main
    git merge --no-ff <proposal-branch>
    git push

- `main` contains full immutable election history.
- Entire repo can be cloned and audited independently.

## Failure Modes and Safeguards

| Failure Mode        | Mitigation |
|----------------------|------------|
| Invalid vote format  | CI rejects PR |
| Duplicate votes      | CI detects duplicates |
| Invalid choices      | CI rejects invalid choices |
| Tampered chain       | `gitvote validate` detects hash breaks |
| Unauthorized voters  | PR permissions / repo forking model control access |

## Key Advantages

- Fully offline-verifiable
- Distributed participation via forks and pull requests
- No server infrastructure required beyond Git hosting
- No need for voter accounts, logins, or centralized identity
- Fully transparent and auditable
- Uses existing developer workflows (GitHub, GitLab, Gitea)

## Open Extensions

- Enforce GPG signature validation
- Anonymous ballots via zero-knowledge proofs
- Ranked-choice or weighted voting
- Admin-controlled voter allowlists
- Web frontends for non-technical voters

---

GitVote Protocol v20250622 — Updated Specification
