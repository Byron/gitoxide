#!/bin/bash
set -eu -o pipefail

GIT_INDEX_VERSION=2 git init -q
git config commit.gpgsign false
git config index.threads 1

touch a b c
mkdir d
(cd d && touch a b c)

git add .
git commit -m "empty"
