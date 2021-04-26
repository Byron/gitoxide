#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q -b parent
git commit -q --allow-empty -m parent

git checkout -q -b child parent
git commit -q --allow-empty -m child

git commit-graph write --no-progress --reachable
git repack -adq
