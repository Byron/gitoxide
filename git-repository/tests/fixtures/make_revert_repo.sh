#!/bin/bash
set -eu -o pipefail

git init -q

git config commit.gpgsign false

git config advice.statusHints false
git config advice.resolveConflict false
git config advice.commitBeforeMerge false
git config advice.skippedCherryPicks false

git config init.defaultBranch master

unset GIT_AUTHOR_DATE
unset GIT_COMMITTER_DATE

touch 1 2 3
git add 1
git commit -m 1 1
git add 2
git commit -m 2 2
git add 3
git commit -m 3 3
git revert --no-commit HEAD~1
