#!/usr/bin/env bash
set -eu -o pipefail

source "$(dirname "${BASH_SOURCE[0]}")/make_submodule_with_worktree.sh"
git -C submodule-with-extra-worktree-host commit -m record-submodule

# Each linked superproject owns its submodule repository, while the main
# superproject's submodule stays populated. Freeze each lifecycle state so
# read-only tests never initialize or deinitialize a shared fixture.
for state in uninitialized initialized deinitialized; do
  linked="linked-superproject-$state"
  git -C submodule-with-extra-worktree-host worktree add --detach "../$linked" HEAD
  if [[ "$state" != uninitialized ]]; then
    # Private worktree submodule object paths exceed Windows' traditional path limit.
    git -c core.longpaths=true -C "$linked" submodule update --init --recursive
  fi
  if [[ "$state" == deinitialized ]]; then
    git -C "$linked" submodule deinit -f m1
  fi
done
