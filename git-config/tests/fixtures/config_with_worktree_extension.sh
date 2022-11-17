#!/bin/bash
set -eu -o pipefail

git init -q

git config extensions.worktreeConfig true
git config --worktree someSection.someSetting "some value"