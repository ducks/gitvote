# GitVote: Git-Native Cryptographic Voting Protocol

## Overview

**GitVote** is a decentralized, cryptographically-verifiable voting protocol
built entirely on top of Git version control. It leverages Git’s distributed,
append-only nature and native support for cryptographic signatures to enable
tamper-resistant, auditable elections without requiring bespoke blockchain
infrastructure.

## Core Principles

- Tamper-evident: All votes are immutable Git commits
- Identity via cryptographic keys: Voters are identified by GPG signing keys
- Decentralized submission: Voters submit votes via Git pull requests
- Centralized finalization: Admin-controlled canonical chain assembly
- Publicly auditable: Entire vote chain is openly cloneable and replayable
- No off-chain data required: Entire election lives in the Git repo

## Protocol Flow

### Election Initialization

- Admin creates a dedicated Git repo for elections.
- For each ballot/race, a separate branch is created (e.g. `president`, `mayor`).

### Voter Distribution

- Voters receive:
  - Repo URL
  - Assigned ballot branch names
  - Voting window timeline
  - Setup instructions (including GPG configuration)

### Voter Preparation

- Voter clones the election repo.
- Voter generates GPG keypair if necessary.
- Voter uploads public key to Git host (e.g. GitHub/Gitea) for commit verification.
- Voter enables signed commits (`commit.gpgsign=true`).
- Voter verifies setup via `gitvote doctor` command.

### Casting a Vote

- Voter checks out assigned ballot branch.
- Voter runs:

`gitvote --race <branch> --choice <vote>`

- The `gitvote` tool:
- Writes a timestamped `votes/vote-<timestamp>.txt` file
- Commits the file with a signed commit
- Voter pushes commit to remote

### Submitting Vote for Validation

- Voter opens a pull request targeting the ballot branch.
- CI system automatically runs validation pipeline.

## Validation Pipeline (CI)

Upon PR creation or push:

1. Validate vote intent file structure:
  - Only one `votes/` file added per PR
  - Valid vote contents (allowed choices)
2. Validate commit signature:
  - Commit must be GPG signed
  - Extract public key fingerprint
3. Check for duplicate votes:
  - Voter has not previously voted on this ballot (fingerprint not seen)
4. Canonical Block Generation:
  - Run `gitvote generate-blocks` on ballot branch to construct
    `blocks/000000.json`, `blocks/000001.json`, etc.
5. Chain Validation:
  - Run `gitvote validate-chain` to verify correct hash chaining

PR fails if any validation fails.
PR is eligible for merge if validation passes.

## Chain Generation Logic

- Admin or CI tool constructs blocks from merged commits.
- Each block includes:
- `index` — sequential block number
- `timestamp` — commit time
- `voter` — GPG fingerprint
- `choice` — vote content
- `prev_hash` — hash of previous block
- Block hashes are computed as SHA-256 over full block content (excluding
`prev_hash` field to avoid circularity).
- Duplicate votes by same voter fingerprint are ignored.

## Election Finalization

At voting close:

- Admin reviews and merges all passing PRs into ballot branch.
- Admin runs final `generate-blocks` and `validate-chain`.
- Admin tallies votes via `gitvote tally`.

## Archiving Election History

Once the ballot branch is finalized:

- Admin merges ballot branch into `main` (canonical audit ledger):

```
git checkout main
git merge --no-ff <branch>
git push
```

- `main` contains complete historical election record.
- Entire repo can be cloned and replayed by any auditor to verify correctness.

## Failure Modes and Safeguards

| Failure Mode        | Mitigation |
|----------------------|------------|
| Unsigned commits     | CI rejects PR |
| Invalid vote format  | CI rejects PR |
| Duplicate votes      | `generate-blocks` ignores duplicates |
| Tampered chain       | `validate-chain` detects hash breaks |
| Unauthorized voters  | Admin controls public key registration |

## Key Advantages

- Fully offline-verifiable
- Distributed participation
- No server infrastructure required beyond Git hosting
- No need for voter accounts, logins, or sensitive identity information
- Cryptographic integrity with auditability baked in
- Leverages existing developer tools (GitHub, Gitea, GitLab)

## Open Extensions

- Public key registration / allowlist
- Anonymous ballots via zero-knowledge proofs
- Web frontend for voters (simplify UX)
- Air-gapped voting stations for in-person elections
- Multi-party ledger mirroring for redundancy

---

GitVote Protocol v20250621 — Draft Specification
