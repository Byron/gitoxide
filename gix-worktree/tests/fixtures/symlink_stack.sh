#!/bin/bash
set -eu -o pipefail

git init base;
(cd base
  touch file
  mkdir dir
  touch dir/file-in-dir
  (cd dir
    ln -s file-in-dir filelink
    mkdir subdir
    ln -s subdir dirlink
  )

  ln -s file root-filelink
  ln -s dir root-dirlink

cat <<EOF > .gitattributes
/file file-attr
/dir/file-in-dir dir-file-attr
EOF
  git add . && git commit -m "init"
)

ln -s base symlink-base

