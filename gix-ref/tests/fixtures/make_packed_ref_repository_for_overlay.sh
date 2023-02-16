#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1
git branch newer-as-loose
git tag -m "tag object" tag-object

mkdir -p .git/refs/remotes/origin

cp .git/refs/heads/main .git/refs/remotes/origin/

echo "ref: refs/remotes/origin/main" > .git/refs/remotes/origin/HEAD

git pack-refs --all --prune

git checkout -q newer-as-loose
git commit -q --allow-empty -m c2
