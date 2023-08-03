#!/bin/bash
set -eu -o pipefail

export GIT_INDEX_VERSION=4
git init -q
git config index.threads 2

touch a b c
mkdir d
(cd d && touch a b c && mkdir last && cd last && touch 123 34 6)
touch x

git add .
git commit -m "empty"
