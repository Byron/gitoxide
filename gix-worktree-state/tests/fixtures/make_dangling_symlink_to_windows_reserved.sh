#!/usr/bin/env bash
set -eu -o pipefail

git init -q

# On Windows, the target is a reserved legacy DOS device name.
con_oid=$(echo -n "CON" | git hash-object -w --stdin)

git update-index --index-info <<EOF
120000 $con_oid	dangling-con-symlink
EOF

git commit -m "dangling symlinks with Widnows reserved target in index"
