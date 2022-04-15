#!/bin/bash
set -eu -o pipefail

git init -q

# Every symlink is dangerous as it might either link to another directory and thus redirect
# all writes in the path, or it might point to a file and opening the symlink actually opens
# the target.
# We handle this by either validating symlinks specifically or create symlinks
empty_oid=$(git hash-object -w --stdin </dev/null)
fake_dir_target=$(echo -n 'A-dir' | git hash-object -w --stdin)
fake_file_target=$(echo -n 'A-file' | git hash-object -w --stdin)

git update-index --index-info <<-EOF
100644 $empty_oid	A-dir/a
100644 $empty_oid	A-file
120000 $fake_dir_target	FAKE-DIR
120000 $fake_file_target	FAKE-FILE
100644 $empty_oid	fake-dir/b
100644 $empty_oid	fake-file
EOF

git commit -m "init"
git checkout -f HEAD;
