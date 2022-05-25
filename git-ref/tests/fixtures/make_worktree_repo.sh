#!/bin/bash
set -eu -o pipefail

mkdir repo && cd repo
git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3
git commit -q --allow-empty -m c4

git tag t1
git tag -m "tag object" dt1

git bisect start && git bisect bad HEAD

git worktree add ../w-detached HEAD~1
(
  cd ../w-detached
  git bisect start
  git bisect bad HEAD
)

git worktree add ../w1
(
  cd ../w1
  git reset --hard HEAD~2

  git bisect start
  git bisect bad HEAD
)

if [ "${1:-}"  = "packed" ]; then
  git pack-refs --all --prune
fi
