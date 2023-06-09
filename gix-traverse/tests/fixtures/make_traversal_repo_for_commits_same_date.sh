#!/bin/bash
set -eu -o pipefail

# all commits have the same date as it's set by `gix-testtools` to a fixed value.

git init -q
git config merge.ff false

git checkout -q -b main
git commit -q --allow-empty -m c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3
git commit -q --allow-empty -m c4

git checkout -q -b branch1
git commit -q --allow-empty -m b1c1
git commit -q --allow-empty -m b1c2

git checkout -q main
git commit -q --allow-empty -m c5
git merge branch1 -m m1b1

git commit-graph write --no-progress --reachable
git repack -adq
