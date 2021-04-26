#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b commit1
git commit -q --allow-empty -m commit1
git checkout -q -b commit2 commit1
git commit -q --allow-empty -m commit2
git checkout -q -b commit3 commit2
git commit -q --allow-empty -m commit3

git show-ref -s commit1 | git commit-graph write --no-progress --split=no-merge --stdin-commits
git show-ref -s commit2 | git commit-graph write --no-progress --split=no-merge --stdin-commits
git show-ref -s commit3 | git commit-graph write --no-progress --split=no-merge --stdin-commits
git repack -adq
