#!/usr/bin/env bash
set -eu -o pipefail


git init -q
git config merge.ff false

git checkout -q -b main

echo "line 1" >> file.txt
git add file.txt
git commit -q -m c1

echo "line 2" >> file.txt
git add file.txt
git commit -q -m c2

echo "line 3" >> file.txt
git add file.txt
git commit -q -m c3

echo "line 4" >> file.txt
git add file.txt
git commit -q -m c4
