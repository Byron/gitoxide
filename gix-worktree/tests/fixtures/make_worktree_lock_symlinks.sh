#!/usr/bin/env bash
set -eu -o pipefail

gix_testtools_require_symlinks

# Lock markers link to live and missing targets outside their administrative directories.
# Both targets stay inside the disposable fixture so lock/unlock cannot affect user files.
mkdir -p live/git-dir dangling/git-dir
printf keep >live/target
ln -s ../target live/git-dir/locked
ln -s ../target dangling/git-dir/locked
