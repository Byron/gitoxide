#!/bin/bash
set -eu -o pipefail

git init -q
git config merge.ff false

git checkout -q -b main
git commit -q --allow-empty -m c1
git tag at-c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3
git commit -q --allow-empty -m c4

git checkout -q -b branch1
git commit -q --allow-empty -m b1c1
git tag at-b1c1
git commit -q --allow-empty -m b1c2

git checkout -q main
git commit -q --allow-empty -m c5
git tag at-c5
git merge branch1 -m m1b1

git commit-graph write --no-progress --reachable
git repack -adq

git clone --depth 1 file://$PWD shallow-1-clone
git clone --depth 2 file://$PWD shallow-2-clone
