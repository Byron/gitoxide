#!/bin/bash
set -eu -o pipefail

export GIT_INDEX_VERSION=2
git init -q
git config index.threads 2

touch a
git add a
git commit -m "empty"
