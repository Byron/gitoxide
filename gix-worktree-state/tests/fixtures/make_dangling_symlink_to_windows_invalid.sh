#!/usr/bin/env bash
set -eu -o pipefail

git init -q

# On Windows, the target is an invalid file name.
qmarks_oid=$(echo -n "???" | git hash-object -w --stdin)

git update-index --index-info <<EOF
120000 $qmarks_oid	dangling-qmarks-symlink
EOF

git commit -m "dangling symlinks with Windows invalid target in index"
