#!/bin/bash
set -eu -o pipefail

git init -q

echo file.main > file
git add file
git commit -m file.main file

git checkout -b other-branch
echo file.other-branch > file
git commit -m file.other-branch file
# Create an mbox formatted patch and save the path
patch_path=$(git format-patch main)

git checkout main
# Create a conflict
echo file.main.update > file
git commit -m file.main.update file

# This will fail due to the merge conflict and leave us in a 'apply mbox in progress' state
git am "$patch_path" || true
