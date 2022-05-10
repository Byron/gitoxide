#!/bin/bash
set -eu -o pipefail

git init -q

mkdir -p tld tld/sd
cat <<EOF >.gitignore
# directory exclude
tld/

!tld/file
EOF

cat <<EOF >tld/.gitignore
sd/
!sd/

!file
EOF

git check-ignore -vn --stdin 2>&1 <<EOF >git-check-ignore.baseline || :
tld
tld/
tld/file
tld/sd
tld/sd/
EOF

