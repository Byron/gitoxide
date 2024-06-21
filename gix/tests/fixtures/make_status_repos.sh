#!/usr/bin/env bash
set -eu -o pipefail

git init -q untracked-only
(cd untracked-only
  touch this
  mkdir subdir
  >subdir/that

  git add .
  git commit -q -m init

  mkdir new
  touch new/untracked subdir/untracked
)
