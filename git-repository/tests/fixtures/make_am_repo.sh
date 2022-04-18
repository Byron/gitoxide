#!/bin/bash
set -eu -o pipefail

git init -q

echo 1.main > 1
git add 1
git commit -m 1.main 1

git checkout -b other-branch
echo 1.other-branch > 1
git commit -m 1.other-branch 1
# Create an mbox formatted patch and save the path
patch_path=$(git format-patch main)

git checkout main
# Create a conflict
echo 1.main.update > 1
git commit -m 1.main.update 1

# This will fail due to the merge conflict and leave us in a 'apply mbox in progress' state
git am 0001-1.other-branch.patch || true
