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

touch 1
git add 1
git commit -m 1 1
git checkout -b other-branch
echo other-branch > 1
git add 1
git commit -m 1.other 1
git checkout master
echo master > 1
git add 1
git commit -m 1.master 1

# This should fail and leave us in a cherry-pick state
git cherry-pick other-branch || true
