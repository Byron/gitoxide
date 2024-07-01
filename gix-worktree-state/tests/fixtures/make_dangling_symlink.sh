#!/usr/bin/env bash
set -eu -o pipefail

git init -q

target_oid=$(echo -n "non-existing-target" | git hash-object -w --stdin)

git update-index --index-info <<EOF
120000 $target_oid	dangling
EOF

git commit -m "dangling symlink in index"
