#!/bin/bash
set -eu -o pipefail

mkdir worktree

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


git clone --shared base relative-workdir
(cd relative-workdir
  git config --local core.worktree ../../worktree
)

git clone --bare --shared base bare-relative-workdir
(cd bare-relative-workdir
  git config --local core.worktree ../worktree
)
