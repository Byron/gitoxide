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

# A graph that is purposefully using times that can't be trusted, i.e. the root E
# has a higher time than its future commits, so that it would be preferred
# unless if there was an additional pruning step to deal with this case.
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

commits=$(git log --all --format=%s)
commit_array=($commits)
num_commits=${#commit_array[@]}

for ((i=0; i<num_commits; i++)); do
    for ((j=0; j<num_commits; j++)); do
        baseline ${commit_array[$i]} ${commit_array[$j]}
    done
done > 3_permutations.baseline

git commit-graph write --no-progress --reachable
git repack -adq
