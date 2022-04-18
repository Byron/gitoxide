#!/bin/bash
set -eu -o pipefail

git init -q

touch 1 2 3

git add 1
git commit -m 1 1

git checkout -b other-branch
echo 2.other-branch > 2
git add 2
git commit -m 2.other-branch 2
git add 3
git commit -m 3 3

git checkout main
echo 2.main > 2
git add 2
git commit -m 2.main 2

# This should fail and leave us in a cherry-pick + sequencer state
git cherry-pick other-branch~2..other-branch || true
