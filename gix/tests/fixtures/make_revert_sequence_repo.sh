#!/bin/bash
set -eu -o pipefail

git init -q

echo 1.0 > 1
git add 1
git commit -m 1.0 1

echo 1.1 > 1
git commit -m 1.1 1

echo 1.2 > 1
git commit -m 1.2 1
touch 2
git add 2
git commit -m 2 2

# This should fail and leave us in a revert + sequencer state
git revert --no-commit HEAD HEAD~2 || true
