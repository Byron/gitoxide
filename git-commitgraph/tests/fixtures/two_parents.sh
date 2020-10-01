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

git checkout -q --orphan parent1
git commit -q --allow-empty -m parent1

git checkout -q --orphan parent2
git commit -q --allow-empty -m parent2

git checkout -q -b child parent1
git merge -q --allow-unrelated-histories --no-ff -m child parent2 >/dev/null

git commit-graph write --no-progress --reachable
git repack -adq
