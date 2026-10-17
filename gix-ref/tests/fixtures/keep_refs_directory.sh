#!/usr/bin/env bash
set -eu -o pipefail

# Only one loose reference remains, with no empty sibling directories masking
# cleanup of the refs root. HEAD and objects still make this a valid repository.
git init -q seed
git -C seed commit --allow-empty -q -m initial
git -C seed update-ref refs/topic HEAD
git -C seed update-ref -d refs/heads/main
rmdir seed/.git/refs/heads seed/.git/refs/tags

cp -R seed pack-flat
cp -R seed git-pack-flat
git -C git-pack-flat pack-refs --all --prune
cp -R seed delete-flat
cp -R seed git-delete-flat
git -C git-delete-flat update-ref -d refs/topic

# Nested references must allow cleanup below refs without removing refs itself.
git -C seed update-ref refs/heads/a/b/topic refs/topic
git -C seed update-ref -d refs/topic
cp -R seed pack-nested
cp -R seed git-pack-nested
git -C git-pack-nested pack-refs --all --prune
cp -R seed delete-nested
cp -R seed git-delete-nested
git -C git-delete-nested update-ref -d refs/heads/a/b/topic
