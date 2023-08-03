#!/bin/bash
set -eu -o pipefail

git init -q

touch a b c

git add .
git commit -m "init"

git config extensions.worktreeConfig true

git config --worktree core.sparseCheckout true
git config --worktree core.sparseCheckoutCone true
git config --worktree index.sparse true

echo "/*" > .git/info/sparse-checkout &&
echo "!/*/" >> .git/info/sparse-checkout

git checkout main