#!/usr/bin/env bash
set -eu -o pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

git init jj-trackcopy-1
(cd jj-trackcopy-1
  # The following is to be executed in the receiving git repository
  index=.git/index
  git hash-object -w -t blob -- $ROOT/assets/jj-trackcopy-1/*.blob
  rm -f "$index"
  git update-index --index-info < "$ROOT/assets/jj-trackcopy-1/2de73f57fc9599602e001fc6331034749b2eacb0.tree"
  git commit --allow-empty -F "$ROOT/assets/jj-trackcopy-1/2de73f57fc9599602e001fc6331034749b2eacb0.msg"
  rm -f "$index"
  git update-index --index-info < "$ROOT/assets/jj-trackcopy-1/47bd6f4aa4a7eeef8b01ce168c6c771bdfffcbd3.tree"
  git commit --allow-empty -F "$ROOT/assets/jj-trackcopy-1/47bd6f4aa4a7eeef8b01ce168c6c771bdfffcbd3.msg"
)