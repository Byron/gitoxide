#!/bin/bash
set -eu -o pipefail

function setup_repo() {
  local version=${1:?need generation version}
  local time=${2:?timestamp seconds since unix epoch}
  git init -q

  # one past the max 32bit date git can represent
  export GIT_COMMITTER_DATE="@${time} +0000"
  git config commitGraph.generationVersion ${version}

  git commit -q --allow-empty -m c1

  git commit-graph write --no-progress --reachable
}

(mkdir v1 && cd v1 && setup_repo 1 68719476737) # the year 4000 something (overflows in graph)
(mkdir v2 && cd v2 && setup_repo 2 68719476737)
(mkdir max-date && cd max-date && setup_repo 1 17147483646) # the year 2500ish
