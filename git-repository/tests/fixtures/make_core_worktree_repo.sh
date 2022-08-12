#!/bin/bash
set -eu -o pipefail

mkdir worktree
touch worktree-file
WORKTREE_ABS="$PWD/worktree"

mkdir base
(cd base
  git init -q
  git checkout -b main

  mkdir dir
  touch a b dir/c
  git add .
  git commit -q -m c1
  echo hello >> a
  git commit -q -am c2
)


git clone --shared base relative-worktree
(cd relative-worktree
  git config --local core.worktree ../../worktree
  git worktree list --porcelain > .git/worktree-list.baseline
  git status --porcelain > .git/status.baseline
)

git clone --shared base absolute-worktree
(cd absolute-worktree
  git config --local core.worktree "$WORKTREE_ABS"
  git worktree list --porcelain > .git/worktree-list.baseline
  git status --porcelain > .git/status.baseline
)

git clone --shared base relative-nonexisting-worktree
(cd relative-nonexisting-worktree
  git config --local core.worktree ../worktree
  git status --porcelain || : > .git/status.baseline
)

git clone --shared base relative-worktree-file
(cd relative-worktree-file
  git config --local core.worktree ../../worktree-file
  git status --porcelain || : > .git/status.baseline
)

git clone --bare --shared base bare-relative-worktree
(cd bare-relative-worktree
  git config --local core.worktree ../worktree
  git status --porcelain || : > status.baseline
)
