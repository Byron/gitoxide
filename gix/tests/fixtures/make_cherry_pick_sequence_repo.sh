#!/bin/bash
set -eu -o pipefail

git init -q

touch f1 f2 f3

git add f1
git commit -m f1 f1

git checkout -b other-branch
echo f2.other-branch > f2
git add f2
git commit -m f2.other-branch f2
git add f3
git commit -m f3 f3

git checkout main
echo f2.main > f2
git add f2
git commit -m f2.main f2

# This should fail and leave us in a cherry-pick + sequencer state
git cherry-pick other-branch~2..other-branch || true
