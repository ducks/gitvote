# Admin Guide

## Starting a new proposal

1. Create a new proposal branch:

`git checkout -b proposal/002-new-topic`

2. Define allowed choices in `schema.json`:

```json
{
"allowed": ["yes", "no", "abstain"]
}
```

3. Push the new branch to the governance repo.
4. Copy `docs/governance-workflow.yml` into `.github/workflows/` to enable CI.

## Finalizing the election

1. Once voting is complete and all valid PRs are merged:

`gitvote tally`

2. Optionally archive the proposal branch into `main` for permanent
   recordkeeping.
