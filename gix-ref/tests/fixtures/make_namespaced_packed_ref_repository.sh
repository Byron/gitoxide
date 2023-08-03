#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -q -b main
git commit -q --allow-empty -m c1
git branch dt1
git branch d1

namespace_1=refs/namespaces/foo/refs
mkdir -p .git/$namespace_1/remotes/origin

cp .git/refs/heads/main .git/$namespace_1/remotes/origin/
cp .git/refs/heads/main .git/$namespace_1/d1

echo "ref: $namespace_1/remotes/origin/main" > .git/$namespace_1/remotes/origin/HEAD

namespace_2=refs/namespaces/bar/refs
mkdir -p .git/$namespace_2/{tags,heads} .git/$namespace_2/remotes/origin

echo "ref: $namespace_2/heads/multi-link-target1" > .git/$namespace_2/multi-link
echo "ref: $namespace_2/tags/multi-link-target2" > .git/$namespace_2/heads/multi-link-target1
echo "ref: $namespace_2/remotes/origin/multi-link-target3" > .git/$namespace_2/tags/multi-link-target2
git rev-parse HEAD > .git/$namespace_2/remotes/origin/multi-link-target3

git tag t1
git tag -m "tag object" dt1

git pack-refs --all --prune
