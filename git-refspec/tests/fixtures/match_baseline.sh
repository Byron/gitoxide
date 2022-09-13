#!/bin/bash
set -eu -o pipefail

git init;

function baseline() {
  {
    git fetch -v origin "$@" 2>&1
    echo specs: "$@"
  } >> baseline.git
}

mkdir base
(cd base
  git init
  touch file
  git add file
  git commit -m "initial commit"
  git tag -m "message" annotated-v0.0

  git checkout -b f1
  git commit -m "f1" --allow-empty
  git tag v0.0-f1

  git checkout -b f2 main
  git commit -m "f2" --allow-empty
  git tag v0.0-f2
)

git clone --shared ./base clone
(cd clone
  baseline "refs/heads/main"
  baseline "heads/main"
  baseline "main"
)

