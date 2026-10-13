#!/usr/bin/env bash
set -eu -o pipefail

# Build symlinks directly in the index so their creation order and native Windows
# types cannot influence the input. Git's checkout is the reference output.
git init -q
git config core.symlinks true

mkdir -p target/nested m-padding
printf 'content\n' >target/file
printf 'nested content\n' >target/nested/file
# Cross checkout's chunk boundary to also exercise parallel finalization.
for ((i = 0; i < 210; i++)); do
    printf 'padding\n' >"m-padding/$i"
done
git add target m-padding

link() {
    local target_blob_id
    target_blob_id=$(printf '%s' "$2" | git hash-object -w --stdin)
    git update-index --add --cacheinfo "120000,$target_blob_id,$1"
}

# Every directory chain needs several passes in index order. The first target
# traverses a directory link as a leading component, and one link is nested.
link a-child z-dir/nested
link a-dir b-dir
link b-dir y-dir
link nested/dir-link ../b-dir
link y-dir z-dir
link z-dir target

link a-file b-file
link b-file y-file
link y-file z-file
link z-file target/file

# Missing targets and cycles must terminate and remain file links on Windows.
link a-missing b-missing
link b-missing missing
link cycle-a cycle-b
link cycle-b cycle-a
link self self

git checkout-index --all --prefix=git-checkout/
