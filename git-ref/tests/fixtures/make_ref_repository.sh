#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main
git commit -q --allow-empty -m c1
git branch dt1
git branch d1

mkdir -p .git/refs/remotes/origin

cp .git/refs/heads/main .git/refs/remotes/origin/
cp .git/refs/heads/main .git/refs/d1

echo "ref: refs/remotes/origin/main" > .git/refs/remotes/origin/HEAD
echo "notahexsha" > .git/refs/broken

git tag t1
git tag dt1
