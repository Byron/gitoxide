#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main
git commit -q --allow-empty -m c1

mkdir -p .git/refs/remotes/origin
cp .git/refs/heads/main .git/refs/remotes/origin/

git tag t1
