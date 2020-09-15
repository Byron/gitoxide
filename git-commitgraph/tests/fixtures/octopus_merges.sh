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

git checkout -q --orphan root
git commit -q --allow-empty -m root

git checkout -q -b commit1 root
git commit -q --allow-empty -m commit1

git checkout -q -b commit2 root
git commit -q --allow-empty -m commit2

git checkout -q -b commit3 root
git commit -q --allow-empty -m commit3

git checkout -q -b commit4 root
git commit -q --allow-empty -m commit4

git checkout -q -b three_parents commit1
git merge -q -m three_parents --no-ff commit2 commit3 >/dev/null

git checkout -q -b four_parents commit2
git merge -q -m four_parents --no-ff commit1 commit3 commit4 >/dev/null

git commit-graph write --no-progress --reachable
git repack -adq
