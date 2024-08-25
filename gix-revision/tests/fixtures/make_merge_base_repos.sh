#!/usr/bin/env bash
set -eu -o pipefail

git init

EMPTY_TREE=$(git mktree </dev/null)
function mkcommit () {
  local OFFSET_SECONDS=$1
  local COMMIT_NAME=$2
  shift 2

  PARENTS= 
  for P; do
    PARENTS="${PARENTS}-p $P "
  done

  GIT_COMMITTER_DATE="$((400403349 + OFFSET_SECONDS)) +0000"
  GIT_AUTHOR_DATE=$GIT_COMMITTER_DATE
  export GIT_COMMITTER_DATE GIT_AUTHOR_DATE

  commit=$(echo $COMMIT_NAME | git commit-tree $EMPTY_TREE ${PARENTS:-})

  git update-ref "refs/tags/$COMMIT_NAME" "$commit"
  echo $commit
}

function baseline() {
  echo "$@"
  echo $(git rev-parse "$@")
  git merge-base --all "$@" || :
  echo
}

# Merge-bases adapted from Git test suite
# No merge base
mkcommit 0 DA
mkcommit 100 DB
{
  echo "just-one-returns-one-in-code"
  echo $(git rev-parse DA)
  echo $(git rev-parse DA)
  echo
  baseline DA DB
  baseline DA DA DB
} > 1_disjoint.baseline

# E---D---C---B---A
# \"-_         \   \
#  \  `---------G   \
#   \                \
#    F----------------H
E=$(mkcommit 5 E)
D=$(mkcommit 4 D $E)
F=$(mkcommit 6 F $E)
C=$(mkcommit 3 C $D)
B=$(mkcommit 2 B $C)
A=$(mkcommit 1 A $B)
G=$(mkcommit 7 G $B $E)
H=$(mkcommit 8 H $A $F)

{
  baseline G H
} > 2_a.baseline

git commit-graph write --no-progress --reachable
git repack -adq
