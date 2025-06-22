#!/bin/bash

set -e

REPO_URL="git@github.com:ducks/gitvote-test.git"
PROPOSAL_BRANCH="proposal/001-color-vote"

VOTER_NAME="$1"
VOTER_EMAIL="$2"
VOTE_CHOICE="$3"

if [ -z "$VOTER_NAME" ] || [ -z "$VOTER_EMAIL" ] || [ -z "$VOTE_CHOICE" ]; then
  echo "Usage: $0 'Voter Name' 'voter@example.com' 'choice'"
  exit 1
fi

# Create isolated working directory
CLONE_DIR="sim-$VOTER_NAME"
rm -rf "$CLONE_DIR"
git clone "$REPO_URL" "$CLONE_DIR"
cd "$CLONE_DIR"

# Checkout proposal branch
git checkout "$PROPOSAL_BRANCH"

# Set Git identity just inside this repo clone
git config user.name "$VOTER_NAME"
git config user.email "$VOTER_EMAIL"

# Cast the vote
~/dev/gitvote/target/release/gitvote cast --choice "$VOTE_CHOICE"

# Create branch for PR
BRANCH_NAME="${PROPOSAL_BRANCH}-${VOTER_EMAIL}"
git checkout -b "$BRANCH_NAME"
git push origin "$BRANCH_NAME"

echo "✅ Vote cast as $VOTER_NAME for $VOTE_CHOICE and pushed to $BRANCH_NAME"
echo "👉 Now open a PR targeting $PROPOSAL_BRANCH"
