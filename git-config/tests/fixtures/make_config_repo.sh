#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF >>.git/config
[a]
  local = value

[include]
  path = ../a.config
EOF


cat <<EOF >>a.config
[a]
  local-include = from-a.config
EOF

cat <<EOF >>system.config
[a]
  system = from-system.config
  system-override = base
[include]
  path = ./b.config
EOF

cat <<EOF >>.gitconfig
[a]
  user = from-user.config
EOF

cat <<EOF >>b.config
[a]
  system-override = from-b.config
EOF

cat <<EOF >>c.config
[env]
  override = from-c.config
EOF

mkdir -p .config/git
(
  cd .config/git
  cat <<EOF >>config
  [a]
    git = git-application
EOF
)
