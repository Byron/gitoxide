#!/bin/bash
set -eu -o pipefail

cat <<EOF >user.exclude
# a custom exclude configured per user
user-file-anywhere
/user-file-from-top

user-dir-anywhere/
/user-dir-from-top

user-subdir/file
**/user-subdir-anywhere/file
EOF

mkdir repo;
(cd repo
  git init -q
  git config core.excludesFile ../user.exclude

  cat <<EOF >.git/info/exclude
# a sample .git/info/exclude
file-anywhere
/file-from-top

dir-anywhere/
/dir-from-top

subdir/file
**/subdir-anywhere/file
EOF

  git commit --allow-empty -m "init"

  mkdir user-dir-anywhere user-dir-from-top dir-anywhere dir-from-top
  mkdir -p dir/user-dir-anywhere dir/dir-anywhere

  git check-ignore -vn --stdin 2>&1 <<EOF >git-check-ignore.baseline || :
user-file-anywhere
dir/user-file-anywhere
user-file-from-top
no-match/user-file-from-top
user-dir-anywhere
dir/user-dir-anywhere
user-dir-from-top
no-match/user-dir-from-top
user-subdir/file
subdir/user-subdir-anywhere/file
file-anywhere
dir/file-anywhere
file-from-top
no-match/file-from-top
dir-anywhere
dir/dir-anywhere
dir-from-top
no-match/dir-from-top
subdir/file
subdir/subdir-anywhere/file
EOF

)
