#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2

mkdir subdir
mkdir -p some/very/deeply/nested/subdir

git clone --bare --shared . bare.git

git worktree add worktrees/a
git worktree add worktrees/b-private-dir-deleted
rm -R .git/worktrees/b-private-dir-deleted
git worktree add worktrees/c-worktree-deleted
rm -R worktrees/c-worktree-deleted

(cd bare.git
  git worktree add ../worktrees/from-bare/c
  git worktree add ../worktrees/from-bare/d-private-dir-deleted
  rm -R -v ./worktrees/d-private-dir-deleted
)

git clone --bare --shared . bare-no-config.git
(cd bare-no-config.git
  rm config
)

git init --bare bare-no-config-after-init.git
(cd bare-no-config-after-init.git
  rm config
)

git clone --shared . worktree-no-config
(cd worktree-no-config
  rm .git/config
)

git init worktree-no-config-after-init
(cd worktree-no-config-after-init
  rm .git/config
)

