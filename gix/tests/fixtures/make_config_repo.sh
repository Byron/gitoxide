#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF >>.git/config
[a]
  bool = on
  bad-bool = zero
  int = 42
  int-overflowing = 9999999999999g
  relative-path = ./something
  absolute-path = /etc/man.conf
  bad-home-path = ~/repo
  bad-user-path = ~noname/repo
  single-string = hello world
  local-override = base
  env-override = base

[include]
  path = ../a.config

[user]
  name = user
  email = user@email

[core]
  autocrlf = true
EOF

# make it a proper repository to allow it to be used for other tests as well.
touch file
git add file
git commit -m "init"


cat <<EOF >>a.config
[a]
  local-override = from-a.config

[committer]
  name = committer
  email = committer@email
EOF
cat <<EOF >>b.config
[a]
  system-override = from-b.config
EOF

cat <<EOF >>c.config
[a]
  env-override = from-c.config
EOF

cat <<EOF >>system.config
[a]
  system = from-system.config
  system-override = base
[include]
  path = ./b.config
EOF
