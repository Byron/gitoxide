#!/usr/bin/env bash
set -eu -o pipefail

gix_testtools_require_symlinks

# Tracked status needs attributes, but must never read ignore files.
# Keep the staged and worktree contents equally sized to require a content check.
git init -q tracked
(cd tracked
  echo hi >file
  git add file && git commit -q -m "init"
  echo ho >file && git add file
  echo ha >file

  # A symlink loop makes reads fail
  ln -s .gitignore .gitignore
  echo "file visible" >.gitattributes
)

# Untracked status needs ignores, but must not read attributes without an attribute pathspec.
git init -q untracked
(cd untracked
  touch untracked
  ln -s .gitattributes .gitattributes
  cat >.gitignore <<EOF
.gitignore
.gitattributes
ignored
EOF
  echo ignored >ignored
)
