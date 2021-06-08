#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}
exe_plumbing=${2:?Second argument must be the plumbing executable to test}
jtt=${3:?Third argument the journey test tool}
kind=${4:?Fourth argument must an indicator of the kind of binary under test}

root="$(cd "${0%/*}" && pwd)"
exe="${root}/../$exe"
exe_plumbing="${root}/../$exe_plumbing"
jtt="${root}/../$jtt"

# shellcheck disable=1090
source "$root/utilities.sh"
source "$root/helpers.sh"
snapshot="$root/snapshots"
fixtures="$root/fixtures"

SUCCESSFULLY=0
WITH_FAILURE=1
WITH_CLAP_FAILURE=2


set-static-git-environment

source "$root/journey/gix.sh"
source "$root/journey/gixp.sh"
