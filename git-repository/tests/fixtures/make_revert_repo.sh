#!/bin/bash
set -eu -o pipefail

git init -q

touch 1 2 3
git add 1
git commit -m 1 1
git add 2
git commit -m 2 2
git add 3
git commit -m 3 3
git revert --no-commit HEAD~1
