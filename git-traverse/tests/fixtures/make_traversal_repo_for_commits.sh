#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main
git commit -q --allow-empty -m c1
git commit -q --allow-empty -m c2
git commit -q --allow-empty -m c3
git commit -q --allow-empty -m c4

git checkout -q -b branch1
git commit -q --allow-empty -m b1c1
git commit -q --allow-empty -m b1c2

git checkout -q main
git commit -q --allow-empty -m c5
git merge branch1 -m m1b1
