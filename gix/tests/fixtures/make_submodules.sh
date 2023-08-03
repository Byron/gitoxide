#!/bin/bash
set -eu -o pipefail

git init -q module1
(cd module1
  touch this
  mkdir subdir
  touch subdir/that
  git add .
  git commit -q -m c1
  echo hello >> this
  git commit -q -am c2
)

git init with-submodules
(cd with-submodules
  mkdir dir
  touch dir/file
  git add dir
  git commit -m "init"

  git submodule add ../module1 m1
  git commit -m "add module 1"

  git submodule add ../module1 dir/m1
)
