#!/bin/sh
set -eu

readonly filename='../outside'
readonly filemode=100644

emit_payload() {
    printf 'A file outside the working tree, somehow.\n'
}

repo="$1"
git init -- "$repo"
cd -- "$repo"

blob_hash="$(emit_payload | git hash-object -w --stdin)"
printf '%s' "$blob_hash" | xxd -r -p >blob-hash-bytes

tree_hash="$(
    printf '%s %s\0' "$filemode" "$filename" |
    cat - blob-hash-bytes |
    git hash-object -t tree -w --stdin --literally
)"

rm blob-hash-bytes

commit_hash="$(git commit-tree -m 'Initial commit' "$tree_hash")"
branch="$(git symbolic-ref --short HEAD)"
git branch -f -- "$branch" "$commit_hash"
git show
