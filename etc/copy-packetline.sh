#!/bin/bash

set -euC -o pipefail

function usage() {
  local name

  name="$(basename -- "$0")"
  printf '%s [--all]         regenerate gix-packetline-blocking source\n' "$name"
  printf '%s --file {path}   regenerate a single file (avoid; prefer --all)\n' "$name"
  printf '%s --help          print this message\n' "$name"
}

function fail () {
  printf '%s: error: %s\n' "$0" "$1" >&2
  exit 1
}

function status () {
  git status --short --ignored=traditional -- "$@"
}

function avoids_pattern () (
  set +e  # Temporary, since the function body is in ( ).
  grep -q "$@"
  test "$?" -eq 1  # Distinguish no-match from error.
)

function indent () {
  sed 's/^/    /'
}

function generate_all () {
  local root failures

  root="$(git rev-parse --show-toplevel)/."  # /. in case name ends in newline.
  cd -- "$root"

  if ! test -d gix-packetline/src; then
    fail 'no source directory: gix-packetline/src'
  fi
  if ! test -d gix-packetline-blocking; then
    fail 'no target parent directory: gix-packetline-blocking'
  fi

  # FIXME: This misinterprets the status when in an unresolved merge conflict!
  if ! status gix-packetline-blocking/src | avoids_pattern '^.[^ ]'; then
    fail 'target has unstaged changes or contains ignored files'
  fi

  rm -rf gix-packetline-blocking/src  # No trailing /, as it may be a symlink.
  if test -e gix-packetline-blocking/src; then
    fail 'unable to remove target'
  fi

  failures="$(find gix-packetline/src/ -exec "$0" --file {} \; -o -print)"

  # If we get here, traversal succeeded, but perhaps some generations failed.
  if test -n "$failures"; then
    fail $'failed to generate from:\n'"$(indent <<<"$failures")"
  fi
}

function first_line_ends_crlf () {
  # This is tricky to check portably. On Windows in Cygwin-like environments
  # including MSYS2 and Git Bash, most text processing tools, including awk,
  # sed, and grep, automatically substitute \n for \r\n. Some can be told not
  # to, but in non-portable ways that may affect other implementations. Bash
  # does so on command substitution and other input, and optionally more often.
  # Easy ways to check are often non-portable to other OSes. Fortunately, tools
  # that treat input as binary data are exempt (including cat, but "-v" is not
  # portable, and it's unreliable in general as lines can end in "^M"). This
  # may be doable without od, by using tr more heavily, but it could be hard to
  # avoid false positives with unexpected characters, or with \r not before \n.

  head -n 1 -- "$1" |  # Get the longest prefix with no non-trailing \n byte.
    od -An -ta |       # Show all bytes symbolically, without addresses.
    tr -sd '\n' ' ' |  # Scrunch into one line, so "cr nl" appears as such.
    grep -q 'cr nl$'   # Check if the result signifies a \r\n line ending.
}

function make_header () {
  local source endline
  source="$1"
  endline="$2"

  # shellcheck disable=SC2016  # The backticks are intentionally literal.
  printf '//! DO NOT EDIT - this is a copy of %s. Run `just copy-packetline` to update it.%s%s' \
    "$source" "$endline" "$endline"
}

function copy_with_header () {
  local source target endline

  source="$1"
  target="$2"

  if first_line_ends_crlf "$source"; then
    endline=$'\r\n'
  else
    endline=$'\n'
  fi

  make_header "$source" "$endline" | cat - -- "$source" >"$target"
}

function generate_one () {
  local source shared target

  source="$1"
  shared="${source#gix-packetline/src/}"
  if test "$source" = "$shared"; then
    fail "source path seems to be outside gix-packetline/src/: $source"
  fi
  target="gix-packetline-blocking/src/$shared"

  if test -d "$source"; then
    mkdir -p -- "$target"
  elif test -L "$source"; then
    # Cover this case separately, for more useful error messages.
    fail "source file is symbolic link: $source"
  elif ! test -f "$source"; then
    # This covers less common kinds of files we can't/shouldn't process.
    fail "source file neither regular file nor directory: $source"
  elif [[ "$source" =~ \.rs$ ]]; then
    copy_with_header "$source" "$target"
  else
    fail "source file not named as Rust source code: $source"
  fi
}

case "$0" in
*{}*)
  # Some "find" implementations expand "{}" even inside a larger argument.
  fail "can't operate portably with literal {} in command name"
  ;;
esac

if { test "$#" -eq 1 && test "$1" = '--all'; } || test "$#" -eq 0; then
  generate_all
elif test "$#" -eq 2 && test "$1" = '--file'; then
  generate_one "$2"
elif test "$#" -eq 1 && test "$1" = '--help'; then
  usage
else
  fail 'unrecognized syntax, try passing only --help for usage'
fi
