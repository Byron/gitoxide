#!/usr/bin/env bash
set -eu -o pipefail

# Makes a repo carrying a tree structure representing the given path to a blob.
# File content is from stdin. Args are repo name, path, -x or +x, and tr sets.
function make_repo() (
  local repo="$1" path="$2" xbit="$3" set1="$4" set2="$5"
  local dir dir_standin path_standin path_standin_pattern path_replacement

  git init -- "$repo"
  cd -- "$repo" # Temporary, as the function body is a ( ) subshell.

  dir="${path%/*}"
  dir_standin="$(tr "$set1" "$set2" <<<"$dir")"
  path_standin="$(tr "$set1" "$set2" <<<"$path")"
  mkdir -p -- "$dir_standin"
  cat >"$path_standin"
  git add --chmod="$xbit" -- "$path_standin"
  path_standin_pattern="$(sed 's/[|.*^$\]/\\&/g' <<<"$path_standin")"
  path_replacement="$(sed 's/[|&\]/\\&/g' <<<"$path")"
  cp .git/index old_index
  LC_ALL=C sed "s|$path_standin_pattern|$path_replacement|g" old_index >.git/index
  git commit -m 'Initial commit'
)

make_repo traverse_dotdot_trees '../outside' -x '.' '@' \
  <<<'A file outside the working tree, somehow.'

make_repo traverse_dotgit_trees '.git/hooks/pre-commit' +x '.' '@' <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF

make_repo traverse_dotgit_stream '.git::$INDEX_ALLOCATION/hooks/pre-commit' +x ':' ',' <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF
