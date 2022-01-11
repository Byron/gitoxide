#!/bin/bash
set -eu -o pipefail

GIT_INDEX_VERSION=4 git init -q
git config commit.gpgsign false
git config index.threads 2

touch a b c
mkdir d
(cd d && touch a b c && mkdir last && cd last && touch 123 35 6)

git add .
git commit -m "empty"
