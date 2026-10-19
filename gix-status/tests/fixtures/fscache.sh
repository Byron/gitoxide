#!/usr/bin/env bash
set -eu -o pipefail

# Plain entries for comparing directory-enumeration metadata with live metadata.
# Windows-specific names, timestamps, and reparse points are prepared in Rust.
printf 'hello' > file
mkdir directory
printf 'nested' > directory/file
