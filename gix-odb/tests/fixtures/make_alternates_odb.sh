#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2

(git init object_source && cd object_source
  git checkout -b main
  touch this
  git add this
  git commit -q -m alternate-c1
)

echo $PWD/object_source/.git/objects > .git/objects/info/alternates

