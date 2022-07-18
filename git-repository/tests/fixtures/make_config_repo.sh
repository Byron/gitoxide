#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF>>.git/config
[a]
  bool = on
  bad-bool = zero
  relative-path = ./something
  absolute-path = /etc/man.conf
  bad-user-path = ~noname/repo
EOF
