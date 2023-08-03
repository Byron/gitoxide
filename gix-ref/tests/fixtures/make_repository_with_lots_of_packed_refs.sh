#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1

current_commit="$(git rev-parse HEAD)"

for level in $(seq -w 1000); do
  mkdir -p .git/refs/heads/"$level"
  for refname in $(seq 150); do
    echo "$current_commit" > .git/refs/heads/"$level"/$refname
  done
done

git tag t1
git tag dt1

git pack-refs --all
