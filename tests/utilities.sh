#!/bin/bash

WHITE="$(tput setaf 9 2>/dev/null || echo -n '')"
YELLOW="$(tput setaf 3 2>/dev/null || echo -n '')"
GREEN="$(tput setaf 2 2>/dev/null || echo -n '')"
RED="$(tput setaf 1 2>/dev/null || echo -n '')"
OFFSET=( )
STEP="  "

function title () {
  echo "$WHITE-----------------------------------------------------"
  echo "${GREEN}$*"
  echo "$WHITE-----------------------------------------------------"
}

function _context () {
  local name="${1:?}"
  shift
  echo 1>&2 "${YELLOW}${OFFSET[*]:-}[$name] $*"
  OFFSET+=("$STEP")
}

function with () {
  _context with "$*"
}

function when () {
  _context when "$*"
}

function _note () {
  local name="${1:?}"
  local color="${2:-}"
  shift 2
  echo 1>&2 -n "${OFFSET[*]:-}${color}[$name] ${*//  /}"
}

function it () {
  _note it "${GREEN}" "$*"
}

function precondition () {
  _note precondition "${WHITE}" "$*"
}

function shortcoming () {
  _note shortcoming "${RED}" "$*"
}

function fail () {
  echo 1>&2 "${RED} $*"
  exit 1
}

function sandbox () {
  sandbox_tempdir="$(mktemp -t sandbox.XXXXXX -d)"
  # shellcheck disable=2064
  trap "popd >/dev/null" EXIT
  pushd "$sandbox_tempdir" >/dev/null \
   || fail "Could not change directory into temporary directory."

  local custom_init="${1:-}"
  if [ -n "$custom_init" ]; then
    eval "$custom_init"
  fi
}

function expect_equals () {
  expect_run 0 test "${1:?}" = "${2:?}"
}

function expect_exists () {
  expect_run 0 test -e "${1:?}"
}

function expect_run_sh () {
  expect_run "${1:?}" bash -c "${2:?}"
}

function expect_snapshot () {
  local expected=${1:?}
  local actual=${2:?}
  if ! [ -e "$expected" ]; then
    mkdir -p "${expected%/*}"
    cp -R "$actual" "$expected"
  fi
  expect_run 0 diff -r -N "$expected" "$actual"
}

function expect_run () {
  local expected_exit_code=$1
  shift
  local output=
  set +e
  output="$("$@" 2>&1)"

  local actual_exit_code=$?
  if [[ "$actual_exit_code" == "$expected_exit_code" ]]; then
    if [[ -n "${WITH_SNAPSHOT-}" ]]; then
      local expected="$WITH_SNAPSHOT"
      if ! [ -f "$expected" ]; then
        mkdir -p "${expected%/*}"
        echo -n "$output" > "$expected" || exit 1
      fi
      if ! diff "$expected" <(echo -n "$output"); then
        echo 1>&2 "${RED} - FAIL"
        echo 1>&2 "${WHITE}\$ $*"
        echo 1>&2 "Output snapshot did not match snapshot at '$expected'"
        echo 1>&2 "$output"
        exit 1
      fi
    fi
    echo 1>&2
  else
    echo 1>&2 "${RED} - FAIL"
    echo 1>&2 "${WHITE}\$ $*"
    echo 1>&2 "${RED}Expected actual status $actual_exit_code to be $expected_exit_code"
    echo 1>&2 "$output"
    exit 1
  fi
  set -e
}
