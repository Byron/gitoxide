#!/bin/bash
set -eu -o pipefail

mkdir base
(cd base
  git init -q

  git checkout -b main
  touch a && git add a
  git commit -q -m c1
  echo 1 >> a
  git commit -q -am c2
  echo 1 >> a
  git commit -q -am c3
)

mkdir empty
(cd empty
  git init -q

  git checkout -b main
  touch a && git add a
  git commit -q -m c1
  touch .git/shallow
)

git clone --depth 1 --bare file://$PWD/base shallow.git
git -C  shallow.git commit-graph write --no-progress --reachable

git clone --depth 1 file://$PWD/base shallow
git -C  shallow commit-graph write --no-progress --reachable
