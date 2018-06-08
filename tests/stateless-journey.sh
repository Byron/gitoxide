#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
exe="${root}/../$exe"

# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots/cli"
fixture="$root/fixtures"

SUCCESSFULLY=0
WITH_FAILURE=1

title "CLI"
(when "initializing a repository"
  (sandbox
    precondition "an initialized baseline repository" && {
      git init &>/dev/null
      expect_snapshot "$snapshot/baseline-init" .git
    }
  )
  (sandbox
    it "succeeds" && {
      WITH_SNAPSHOT="$snapshot/init-success" \
      expect_run $SUCCESSFULLY "$exe" init
    }

    it "matches the output of baseline git init" && {
      expect_snapshot "$snapshot/baseline-init" .git
    }
  )
)

