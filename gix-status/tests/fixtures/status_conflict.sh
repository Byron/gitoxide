#!/bin/bash
set -eu -o pipefail

git init -q

echo base > content
git add -A
git commit -m "base"

git checkout -b feat
echo feat > content
git commit -am "feat"

git checkout main
echo base-change > content
git commit -am "new base"

git merge feat || :
