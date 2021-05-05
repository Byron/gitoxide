#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main
touch a b c
mkdir d e f
touch d/a e/b f/c f/z
mkdir f/d
touch f/d/x

git add .
git commit -q -m c1
