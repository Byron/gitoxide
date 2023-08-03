#!/bin/bash
set -eu -o pipefail

bare="${1:-}"

mkdir repo
(
  cd repo
  git init -q

  git checkout -b main
  mkdir dir
  touch a b dir/c
  git add .
  git commit -q -m c1
  echo hello >> a
  git commit -q -am c2
)

if [ "$bare" == "bare" ]; then
  git clone --bare --shared repo repo.git
  cd repo.git
else
  cd repo
fi

git worktree add ../wt-a
git worktree add ../prev/wt-a HEAD~1
git worktree add ../wt-b HEAD~1
git worktree add ../wt-a/nested-wt-b HEAD~1
git worktree add --lock ../wt-c-locked
git worktree add ../wt-deleted && rm -Rf ../wt-deleted

git worktree list --porcelain > ../worktree-list.baseline
