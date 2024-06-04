#!/usr/bin/env bash
set -eu -o pipefail

# Makes a repo carrying a literally named file, which may even contain "/".
# File content is from stdin. Arguments are repo name, file name, and file mode.
function make_repo() (
  local repo="$1" file="$2" mode="$3"
  local blob_hash_escaped tree_hash commit_hash branch

  git init -- "$repo"
  cd -- "$repo" # Temporary, as the function body is a ( ) subshell.

  blob_hash_escaped="$(git hash-object -w --stdin | sed 's/../\\x&/g')"

  tree_hash="$(
    printf "%s %s\\0$blob_hash_escaped" "$mode" "$file" |
    git hash-object -t tree -w --stdin --literally
  )"

  commit_hash="$(git commit-tree -m 'Initial commit' "$tree_hash")"

  branch="$(git symbolic-ref --short HEAD)"
  git branch -f -- "$branch" "$commit_hash"
  test -z "${DEBUG_FIXTURE-}" || git show # TODO: How should verbosity be controlled?
  git rev-parse @^{tree} > head.tree
)

make_repo traverse_dotdot_slashes ../outside 100644 \
  <<<'A file outside the working tree, somehow.'

make_repo traverse_dotgit_slashes .git/hooks/pre-commit 100755 <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF

make_repo traverse_dotdot_backslashes '..\outside' 100644 \
  <<<'A file outside the working tree, somehow.'

make_repo traverse_dotgit_backslashes '.git\hooks\pre-commit' 100755 <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF
