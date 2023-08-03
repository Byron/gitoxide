#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF >.git/info/exclude
# a sample .git/info/exclude
file-anywhere
/file-from-top

dir-anywhere/
/dir-from-top

subdir-anywhere/file
subdir-anywhere/dir/
EOF

git commit --allow-empty -m "init"
