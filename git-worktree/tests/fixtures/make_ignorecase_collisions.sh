#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

touch a A
git add a
echo A | git update-index --add --stdin

git commit -m "init"
