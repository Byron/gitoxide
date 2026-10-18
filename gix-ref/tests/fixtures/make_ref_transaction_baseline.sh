#!/usr/bin/env bash
set -eu -o pipefail

# One small repository per source/destination/blocker/operation/order combination.
# `setup` is copied for gix edits; Git baselines remain repositories so refs and
# reflogs, including dangling HEAD logs, can be observed without storage assumptions.
source_branch=$1
destination_branch=$2
blocker=$3
operation=$4
order=$5

# Keep the source checked out to expose HEAD changes, and give its reflog history
# to preserve. A blocker points to the older commit so accidental updates are visible.
git init -q -b "$source_branch"
git commit -q --allow-empty -m initial
git commit -q --allow-empty -m second
if test "$blocker" != '-'; then
  git branch "$blocker" HEAD~1
fi

status=0
case "$operation" in
  setup) ;;
  update-ref)
    commit_id=$(git rev-parse HEAD)
    delete="delete refs/heads/$source_branch $commit_id"
    create="create refs/heads/$destination_branch $commit_id"
    if test "$order" = update-first; then
      printf 'start\n%s\n%s\nprepare\ncommit\n' "$create" "$delete"
    else
      printf 'start\n%s\n%s\nprepare\ncommit\n' "$delete" "$create"
    fi >edits
    git update-ref --stdin -m rename <edits >stdout 2>stderr || status=$?
    ;;
  branch)
    git branch -m "$source_branch" "$destination_branch" >stdout 2>stderr || status=$?
    ;;
  *) echo "unknown operation: $operation" >&2; exit 1 ;;
esac
printf '%s\n' "$status" >status
