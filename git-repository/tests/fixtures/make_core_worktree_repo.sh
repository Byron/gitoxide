#!/bin/bash
set -eu -o pipefail

mkdir worktree

mkdir repo
cd repo
git init -q

git checkout -b main
mkdir dir
touch a b dir/c
git add .
git commit -q -m c1
echo hello >> a
git commit -q -am c2

git config --local core.worktree ../../worktree
