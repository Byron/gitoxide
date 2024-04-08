#!/bin/bash

set -euC -o pipefail

function fail () {
  printf '%s: error: %s\n' "$0" "$1" >&2
  exit 1
}

function chdir_toplevel() {
  local root_padded root

  # Find the working tree's root. (Padding is for the trailing-newline case.)
  root_padded="$(git rev-parse --show-toplevel && echo -n .)" ||
    fail 'git-rev-parse failed to find top-level dir'
  root="${root_padded%$'\n.'}"

  cd -- "$root"
}

function merging () {
  local git_dir_padded git_dir

  # Find the .git directory. (Padding is for the trailing-newline case.)
  git_dir_padded="$(git rev-parse --git-dir && echo -n .)" ||
    fail 'git-rev-parse failed to find git dir'
  git_dir="${git_dir_padded%$'\n.'}"

  test -e "$git_dir/MERGE_HEAD"
}

function target_status () {
  git status --short --ignored=traditional -- gix-packetline-blocking/src ||
    fail 'git-status failed'
}

function check_target () {
  if ! test -e "gix-packetline-blocking/src"; then
    # The target does not exist on disk, so nothing will be lost. Proceed.
    return
  fi

  if merging; then
    # In a merge, it would be confusing to replace anything at the target.
    if target_status | grep -q '^'; then
      fail 'target exists, and a merge is in progress'
    fi
  else
    # We can lose data if anything of value at the target is not in the index.
    if target_status | grep -q '^.[^ ]'; then
      fail 'target exists, with unstaged changes or ignored files'
    fi
  fi
}

function first_line_ends_crlf () {
  # This is tricky to check portably. In Cygwin-like environments including
  # MSYS2 and Git Bash, most text processing tools, including awk, sed, and
  # grep, automatically ignore \r before \n. Some ignore \r everywhere. Some
  # can be told to keep \r, but in non-portable ways that may affect other
  # implementations. Bash ignores \r in some places even without "-o igncr",
  # and ignores \r even more with it, including in all text from command
  # substitution. Simple checks may be non-portable to other OSes. Fortunately,
  # tools that treat input as binary data are exempt (even cat, but "-v" is
  # non-portable, and unreliable in general because lines can end in "^M").
  # This may be doable without od, by using tr more heavily, but it could be
  # hard to avoid false positives with unexpected characters or \r without \n.

  head -n 1 -- "$1" |  # Get the longest prefix with no non-trailing \n byte.
    od -An -ta |       # Represent all bytes symbolically, without addresses.
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
  local source target

  source="$1"
  target="gix-packetline-blocking/src/${source#gix-packetline/src/}"

  if test -d "$source"; then
    mkdir -p -- "$target"
  elif test -L "$source"; then
    # Cover this case separately, for more useful error messages.
    fail "source file is symbolic link: $source"
  elif ! test -f "$source"; then
    # This covers less common kinds of files we can't or shouldn't process.
    fail "source file neither regular file nor directory: $source"
  elif [[ "$source" =~ \.rs$ ]]; then
    copy_with_header "$source" "$target"
  else
    fail "source file not named as Rust source code: $source"
  fi
}

function generate_all () {
  local source

  chdir_toplevel

  if ! test -d gix-packetline/src; then
    fail 'no source directory: gix-packetline/src'
  fi
  if ! test -d gix-packetline-blocking; then
    fail 'no target parent directory: gix-packetline-blocking'
  fi

  check_target

  rm -rf gix-packetline-blocking/src  # No trailing "/" as it may be a symlink.
  if test -e gix-packetline-blocking/src; then
    fail 'unable to remove target'
  fi

  find gix-packetline/src/ -print0 | while IFS= read -r -d '' source; do
    generate_one "$source"
  done
}

generate_all
