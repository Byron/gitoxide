#!/bin/bash
set -eu -o pipefail

function tick () {
  if test -z "${tick+set}"
  then
    tick=1112911993
  else
    tick=$(($tick + 60))
  fi
  GIT_COMMITTER_DATE="$tick -0700"
  GIT_AUTHOR_DATE="$tick -0700"
  export GIT_COMMITTER_DATE GIT_AUTHOR_DATE
}

tick
function commit() {
  local message=${1:?first argument is the commit message}
  tick
  git commit --allow-empty -m "$message"
}

function optimize() {
  git commit-graph write --no-progress --reachable
  git repack -adq
}

(git init simple && cd simple
  git config merge.ff false

  git checkout -q -b main
  commit c1
  commit c2
  commit c3
  commit c4

  git checkout -q -b branch1
  git checkout -q -b branch2
  commit b2c1
  commit b2c2

  git checkout branch1
  commit b1c1
  commit b1c2

  git checkout -q main
  commit c5
  git merge branch1 branch2 -m merge

  optimize
)

(git init intermixed && cd intermixed
  git config merge.ff false

  git checkout -q -b main
  commit c1
  commit c2

  git checkout -q -b branch1
  git checkout -q -b branch2
  commit b2c1

  git checkout branch1
  commit b1c1

  git checkout branch2
  commit b2c2

  git checkout branch1
  commit b1c2

  git checkout -q main
  commit c3
  git merge branch1 branch2 -m merge

  optimize
)
