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

# NOTE: This relies on GNU sed behavior and will fail on *BSDs (including macOS) without GNU
# sed installed.  
sed=$(which gsed sed | head -1 || true)

# NOTE: Starting with git 2.35.0 --preserve-merges was renamed to --rebase-merges
# TODO: Revisit this after git 3.x is released
IFS=. read git_major git_minor <<< $(git --version | awk '{print $3}')
if [ "${git_major}" -eq 2 ] && [ "$(bc <<< "${git_minor} >= 35.0")" = "1" ]; then
    rebase_command="--rebase-merges"
else
    rebase_command="--preserve-merges"
fi
EDITOR="${sed} -i.bak -z 's/pick/edit/2'" git rebase "${rebase_command}" --interactive HEAD~2

