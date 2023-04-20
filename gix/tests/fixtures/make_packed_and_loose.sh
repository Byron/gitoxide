#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
touch this
git add this
git commit -q -m c1
echo hello >> this
git commit -q -am c2

git gc

echo hello >> this
echo hello >> that
git add . && git commit -m "loose"
