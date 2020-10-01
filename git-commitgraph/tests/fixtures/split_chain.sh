#!/bin/bash
set -eu -o pipefail

export GIT_AUTHOR_DATE="2000-01-01 00:00:00 +0000"
export GIT_AUTHOR_EMAIL=author@example.com
export GIT_AUTHOR_NAME=author
export GIT_COMMITTER_DATE="2000-01-02 00:00:00 +0000"
export GIT_COMMITTER_EMAIL=committer@example.com
export GIT_COMMITTER_NAME=committer

mkdir -p "$1"
cd "$1"
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
