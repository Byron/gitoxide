#!/bin/bash
set -eu -o pipefail

# Note that symlink creation fails on Windows for some reason,
# so these tests shouldn't be run there.

git init breakout-symlink
(cd breakout-symlink
  mkdir hide
  ln -s ../.. hide/breakout
  touch file
)

ln -s breakout-symlink symlink-to-breakout-symlink

git init immediate-breakout-symlink
(cd immediate-breakout-symlink
  ln -s .. breakout
)
