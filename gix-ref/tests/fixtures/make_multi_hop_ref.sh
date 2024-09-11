#!/usr/bin/env bash
set -eu -o pipefail

git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1

git tag t1
git tag -m "tag object" dt1
git tag -m "tag object indirect" dt2 dt1

echo "ref: refs/tags/dt2" > .git/refs/multi-hop
echo "ref: refs/multi-hop" > .git/refs/multi-hop2

if [ "${1:-}" = "packed" ]; then
  git pack-refs --all --prune
fi
