#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF>>.git/config
[a]
  bool = on
  bad-bool = zero
  int = 42
  int-overflowing = 9999999999999g
  relative-path = ./something
  absolute-path = /etc/man.conf
  bad-user-path = ~noname/repo
  single-string = hello world
  override = base

[include]
  path = ../a.config
EOF


cat <<EOF>>a.config
[a]
  override = from-a.config
EOF
