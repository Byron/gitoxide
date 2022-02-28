#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

touch empty
echo "content" > executable
chmod +x executable

mkdir dir
echo "other content" > dir/content
mkdir dir/sub-dir
(cd dir/sub-dir && ln -sf ../content symlink)

git add -A
git commit -m "Commit"
