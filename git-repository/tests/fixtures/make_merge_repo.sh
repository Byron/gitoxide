#!/bin/bash
set -eu -o pipefail

git init -q

echo 1.main > 1
git add 1
git commit -m 1.main 1

git checkout -b other-branch
echo 1.other-branch > 1
git add 1
git commit -m 1.other-branch 1

git checkout main
echo 1.main changed > 1
git commit -m 1.main\ changed 1

git merge other-branch || true
