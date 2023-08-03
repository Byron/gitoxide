#!/bin/bash
set -eu -o pipefail

git init -q

touch content
echo -n "content" > content

git add --intent-to-add -A
