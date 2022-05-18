#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main

git commit --allow-empty -q -m c1

git remote add --fetch remote_repo .
git branch --set-upstream-to remote_repo/main

git config branch.broken.merge not_a_valid_merge_ref
