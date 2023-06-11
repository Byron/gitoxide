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
  local date=${2:-}
  local file="$message.t"
  echo "$1" > "$file"
  git add -- "$file"
  if [ -n "$date" ]; then
    export GIT_COMMITTER_DATE="$date"
  else
    tick
  fi
  git commit -m "$message"
  git tag "$message"
}

# adapted from git/t/t5318 'lower layers have overflow chunk'
UNIX_EPOCH_ZERO="@0 +0000"
FUTURE_DATE="@4147483646 +0000"

git init
git config commitGraph.generationVersion 2

commit future-1 "$FUTURE_DATE"
commit old-1 "$UNIX_EPOCH_ZERO"
git commit-graph write --reachable
commit future-2 "$FUTURE_DATE"
commit old-2 "$UNIX_EPOCH_ZERO"
git commit-graph write --reachable --split=no-merge
commit extra
# this makes sure it's actually in chain format.
git commit-graph write --reachable --split=no-merge
