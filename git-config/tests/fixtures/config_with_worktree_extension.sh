#!/bin/bash
set -eu -o pipefail

git init -q main-worktree
(cd main-worktree
  git config extensions.worktreeConfig true
  git config --worktree worktree.override "set in the main worktree"
)
