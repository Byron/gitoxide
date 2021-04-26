#!/bin/bash
set -eu -o pipefail

# The goal with this repo is to have the smallest commit-graph file possible, in the hopes that an

git init -q
git config commit.gpgsign false

git checkout -q -b commit
git commit -q --allow-empty -m commit

git commit-graph write --no-progress --reachable
git repack -adq
