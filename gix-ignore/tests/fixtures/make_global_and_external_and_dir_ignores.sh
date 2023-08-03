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
a/b/*
z/x
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

  cat <<EOF >.gitignore
# a sample .gitignore
top-level-local-file-anywhere
d/e/*
e/f
EOF

  mkdir dir-with-ignore
  cat <<EOF >dir-with-ignore/.gitignore
# a sample .gitignore
sub-level-local-file-anywhere
EOF

  git add .gitignore dir-with-ignore
  git commit --allow-empty -m "init"

  mkdir user-dir-anywhere user-dir-from-top dir-anywhere dir-from-top
  mkdir -p dir/user-dir-anywhere dir/dir-anywhere

git check-ignore -vn --stdin 2>&1 <<EOF >git-check-ignore.baseline || :
user-file-anywhere
dir/user-file-anywhere
user-file-from-top
no-match/user-file-from-top
user-dir-anywhere
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
top-level-local-file-anywhere
dir/top-level-local-file-anywhere
no-match/sub-level-local-file-anywhere
dir-with-ignore/sub-level-local-file-anywhere
dir-with-ignore/sub-dir/sub-level-local-file-anywhere
a/b/C
a/B/c
A/B/C
z/x
Z/x
z/X
Z/X
d/e/F
d/e/f
D/e/F
D/E/F
e/f
e/F
E/f
E/F
EOF
)
