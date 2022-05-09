#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2

mkdir -p some/very/deeply/nested/subdir

git clone --bare --shared . bare.git

git worktree add worktrees/a
git worktree add worktrees/b-deleted
rm -Rf worktrees/b-deleted

cd bare.git
git worktree add ../worktrees/from-bare/c
git worktree add ../worktrees/from-bare/d-deleted
rm -Rf -v ../worktrees/from-bare/d-deleted