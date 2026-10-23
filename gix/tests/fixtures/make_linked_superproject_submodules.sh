#!/usr/bin/env bash
set -eu -o pipefail

# Names are short because on Windows, Git refuses a `$GIT_DIR` longer than
# `PATH_MAX - 40` (220) bytes, even with `core.longpaths`. The longest one here
# is the `deinitialized` linked worktree's private submodule repository,
#
#     superproject/.git/worktrees/deinitialized/modules/m1
#
# and its absolute path also includes the checkout and fixture directories.
git init -q module1
(cd module1
  touch this
  git add . && git commit -q -m c1
)

git init superproject
git -C superproject submodule add ../module1 m1
git -C superproject commit -m record-submodule

# Each linked superproject owns its submodule repository, while the main
# superproject's submodule stays populated. Freeze each lifecycle state so
# read-only tests never initialize or deinitialize a shared fixture.
for state in uninitialized initialized deinitialized; do
  git -C superproject worktree add --detach "../$state" HEAD
  if [[ "$state" != uninitialized ]]; then
    # Private worktree submodule object paths can exceed Windows' traditional path limit.
    git -c core.longpaths=true -C "$state" submodule update --init --recursive
  fi
  if [[ "$state" == deinitialized ]]; then
    git -C "$state" submodule deinit -f m1
  fi
done
