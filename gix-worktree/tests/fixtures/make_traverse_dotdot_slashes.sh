#!/bin/bash
set -eu -o pipefail

readonly filename='../outside'
readonly filemode=100644

emit_payload() {
    echo 'A file outside the working tree, somehow.'
}

repo="$1"
git init -- "$repo"
cd -- "$repo"

blob_hash_escaped="$(
    emit_payload |
    git hash-object -w --stdin |
    sed 's/../\\x&/g'
)"

tree_hash="$(
    printf "%s %s\\0$blob_hash_escaped" "$filemode" "$filename" |
    git hash-object -t tree -w --stdin --literally
)"

commit_hash="$(git commit-tree -m 'Initial commit' "$tree_hash")"
branch="$(git symbolic-ref --short HEAD)"
git branch -f -- "$branch" "$commit_hash"
git show
