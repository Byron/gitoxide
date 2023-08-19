#!/bin/bash
set -eu -o pipefail

git init -q

empty_oid=$(git hash-object -w --stdin </dev/null)
content_oid=$(echo "content" | git hash-object -w --stdin)
symlink_target=$(echo -n 'X' | git hash-object -w --stdin)

echo "FILE_? filter=arrow" > .gitattributes
git add -A

git update-index --index-info <<-EOF
100644 $content_oid	FILE_X
100644 $content_oid	FILE_x
100644 $content_oid	file_X
100644 $content_oid	file_x
100644 $empty_oid	D/B
100644 $empty_oid	D/C
100644 $empty_oid	d
100644 $empty_oid	X
120000 $symlink_target	x
120000 $symlink_target	link-to-X
EOF

git commit -m "init"
git checkout -f HEAD;
