#!/usr/bin/env bash
set -eu -o pipefail

git init -q

target_oid=$(echo -n "." | git hash-object -w --stdin)

git update-index --index-info <<EOF
120000 $target_oid	symlink
EOF

git commit -m "symlink in index, points to directory"
