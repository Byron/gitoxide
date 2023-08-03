#!/bin/bash
set -eu -o pipefail

driver=${1:?First argument is the driver program supporting both process mode and clean/smudge}

function repo_assertions() {
  echo '* filter=arrow' > .gitattributes
  git add . && git commit -m c1
  echo hi > file
  git add file && git commit -m c2
  rm file
  git checkout file
}

(
  git init no-process && cd no-process

  git config filter.arrow.clean "$driver clean %f"
  git config filter.arrow.smudge "$driver smudge %f"
  git config filter.arrow.requred true

  repo_assertions
)

(
  git init process && cd process

  git config filter.arrow.process "$driver process"
  git config filter.arrow.requred true

  repo_assertions
)

(
  git init process-no-delay && cd process-no-delay

  git config filter.arrow.process "$driver process disallow-delay"
  git config filter.arrow.requred true

  repo_assertions
)
