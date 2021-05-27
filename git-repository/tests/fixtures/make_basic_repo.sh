#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -b main
touch this
git add this
git commit -q -m c1

mkdir -p some/very/deeply/nested/subdir

git init --bare bare.git
