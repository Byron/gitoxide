#!/bin/bash
set -eu -o pipefail

git init -q

echo -n "foo" > content

git add -A
git commit -m "Commit"

# file size should not be changed by this
echo -n "bar" > content
