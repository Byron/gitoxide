#!/bin/bash
set -eu -o pipefail

git init -q

touch f1 f2 f3
git add f1
git commit -m f1 f1
git add f2
git commit -m f2 f2
git add f3
git commit -m f3 f3
git revert --no-commit HEAD~1
