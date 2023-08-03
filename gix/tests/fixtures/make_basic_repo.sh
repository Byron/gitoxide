#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2

mkdir -p some/very/deeply/nested/subdir

git init --bare bare.git

git init --bare bare-repo-with-index.git
(cd bare-repo-with-index.git
  touch index
)

git init non-bare-repo-without-index
(cd non-bare-repo-without-index
  touch this
  git add this && git commit -m "init"
  rm .git/index
)
