#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
exe="${root}/../$exe"

# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixtures="$root/fixtures"

SUCCESSFULLY=0
# WITH_FAILURE=1

title "smart-release"
(sandbox
  snapshot="$snapshot/triple-depth-workspace"
  cp -R $fixtures/tri-depth-workspace/* .
  { echo 'target/' > .gitignore && git init . && git add . && git commit -q -m "initial"; } &>/dev/null

  (with "'c' as argument and minor-bumping dependencies"
    it "succeeds" && {
      WITH_SNAPSHOT="$snapshot/no-args-success" \
      expect_run $SUCCESSFULLY "$exe" smart-release c -d minor
    }
  )
)

