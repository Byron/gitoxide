#!/bin/bash
set -eu -o pipefail

git init -q

touch a
echo "Test Vals" > a
touch b
touch c
touch executable.sh
chmod +x executable.sh

mkdir d
touch d/a
echo "Subdir" > d/a
ln -sf d/a sa

git add -A
git commit -m "Commit"
