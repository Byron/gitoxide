#!/bin/bash
set -eu -o pipefail

git init -q

touch file
git add file
git commit -m first file

git checkout -b other-branch
echo other-branch > file
git commit -m file.other file

git checkout main
echo main > file
git add file
git commit -m file.main file

# This should fail and leave us in a cherry-pick state
git cherry-pick other-branch || true
