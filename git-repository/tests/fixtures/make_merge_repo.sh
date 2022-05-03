#!/bin/bash
set -eu -o pipefail

git init -q

echo file.main > file
git add file
git commit -m file.main file

git checkout -b other-branch
echo file.other-branch > file
git add file
git commit -m file.other-branch file

git checkout main
echo file.main changed > file
git commit -m file.main\ changed file

git merge other-branch || true
