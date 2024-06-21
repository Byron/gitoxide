#!/usr/bin/env bash

set -euC -o pipefail

readonly input_dir='gix-packetline/src'
readonly output_parent_dir='gix-packetline-blocking'
readonly output_dir="$output_parent_dir/src"

function fail () {
  printf '%s: error: %s\n' "$0" "$1" >&2
  exit 1
}

function chdir_toplevel () {
  local root_padded root

  # Find the working tree's root. (Padding covers the trailing-newline case.)
  root_padded="$(git rev-parse --show-toplevel && echo -n .)" ||
    fail 'git-rev-parse failed to find top-level dir'
  root="${root_padded%$'\n.'}"

  cd -- "$root"
}

function merging () {
  local git_dir_padded git_dir

  # Find the .git directory. (Padding covers the trailing-newline case.)
  git_dir_padded="$(git rev-parse --git-dir && echo -n .)" ||
    fail 'git-rev-parse failed to find git dir'
  git_dir="${git_dir_padded%$'\n.'}"

  test -e "$git_dir/MERGE_HEAD"
}

function output_dir_status () {
  git status --porcelain --ignored=traditional -- "$output_dir" ||
    fail 'git-status failed'
}

function check_output_dir () {
  if ! test -e "$output_dir"; then
    # The destination does not exist on disk, so nothing will be lost. Proceed.
    return
  fi

  if merging; then
    # In a merge, it would be confusing to replace anything at the destination.
    if output_dir_status | grep -q '^'; then
      fail 'output location exists, and a merge is in progress'
    fi
  else
    # We can lose data if anything of value at the destination is not in the
    # index. (This includes unstaged deletions, for two reasons. We could lose
    # track of which files had been deleted. More importantly, replacing a
    # staged symlink or regular file with an unstaged directory is shown by
    # git-status as only a deletion, even if the directory is non-empty.)
    if output_dir_status | grep -q '^.[^ ]'; then
      fail 'output location exists, with unstaged changes or ignored files'
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
  local input_file endline

  input_file="$1"
  endline="$2"

  # shellcheck disable=SC2016  # The backticks are intentionally literal.
  printf '// DO NOT EDIT - this is a copy of %s. Run `just copy-packetline` to update it.%s%s' \
    "$input_file" "$endline" "$endline"
}

function copy_with_header () {
  local input_file output_file endline

  input_file="$1"
  output_file="$2"

  if first_line_ends_crlf "$input_file"; then
    endline=$'\r\n'
  else
    endline=$'\n'
  fi

  make_header "$input_file" "$endline" | cat -- - "$input_file" >"$output_file"
}

function generate_one () {
  local input_file output_file

  input_file="$1"
  output_file="$output_dir${input_file#"$input_dir"}"

  if test -d "$input_file"; then
    mkdir -p -- "$output_file"
  elif test -L "$input_file"; then
    # Cover this case separately, for more useful error messages.
    fail "input file is symbolic link: $input_file"
  elif ! test -f "$input_file"; then
    # This covers less common kinds of files we can't or shouldn't process.
    fail "input file neither regular file nor directory: $input_file"
  elif [[ "$input_file" =~ \.rs$ ]]; then
    copy_with_header "$input_file" "$output_file"
  else
    fail "input file not named as Rust source code: $input_file"
  fi
}

function generate_all () {
  local input_file

  if ! test -d "$input_dir"; then
    fail "no input directory: $input_dir"
  fi
  if ! test -d "$output_parent_dir"; then
    fail "no output parent directory: $output_parent_dir"
  fi
  check_output_dir

  rm -rf -- "$output_dir"  # It may be a directory, symlink, or regular file.
  if test -e "$output_dir"; then
    fail 'unable to remove output location'
  fi

  find "$input_dir" -print0 | while IFS= read -r -d '' input_file; do
    generate_one "$input_file"
  done
}

chdir_toplevel
generate_all
