#!/bin/bash
set -eu -o pipefail

git init -q

cat <<EOF >>.git/config
[user]
  email = local@example.com

[committer]
  name = local committer
EOF

cat <<EOF >>global.config
[user]
  name = global name
  email = global@example.com

[committer]
  email = global-committer@example.com
EOF

cat <<EOF >>system.config
[user]
  name = system name
  email = system@example.com
EOF
