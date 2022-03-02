#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

empty_oid=$(git hash-object -w --stdin </dev/null)

git update-index --index-info <<-EOF
100644 $empty_oid	FILE_X
100644 $empty_oid	FILE_x
100644 $empty_oid	file_X
100644 $empty_oid	file_x
100644 $empty_oid	D/B
100644 $empty_oid	D/C
100644 $empty_oid	d
EOF

git commit -m "init"
git checkout -f HEAD;
