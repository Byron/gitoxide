#!/bin/bash
set -eu -o pipefail

GIT_INDEX_VERSION=2 git init -q
git config commit.gpgsign false
git config index.threads 2

touch a
git add a
git commit -m "empty"
