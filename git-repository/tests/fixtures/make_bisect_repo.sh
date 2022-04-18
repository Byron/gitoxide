#!/bin/bash
set -eu -o pipefail

git init -q

touch 1 2

git add 1
git commit -m 1 1

git add 2
git commit -m 2 2

git bisect start
