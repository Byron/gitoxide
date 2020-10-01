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

git checkout -q -b parent
git commit -q --allow-empty -m parent

git checkout -q -b child parent
git commit -q --allow-empty -m child

git commit-graph write --no-progress --reachable
git repack -adq
