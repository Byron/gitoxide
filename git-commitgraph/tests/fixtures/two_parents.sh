#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q --orphan parent1
git commit -q --allow-empty -m parent1

git checkout -q --orphan parent2
git commit -q --allow-empty -m parent2

git checkout -q -b child parent1
git merge -q --allow-unrelated-histories --no-ff -m child parent2 >/dev/null

git commit-graph write --no-progress --reachable
git repack -adq
