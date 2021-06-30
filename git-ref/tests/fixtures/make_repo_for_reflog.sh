#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main
git commit -q --allow-empty -m c1

touch this
git add this
git commit -q -m "add this"

git revert HEAD
git revert HEAD

git commit --amend -m "add this, for sure"
