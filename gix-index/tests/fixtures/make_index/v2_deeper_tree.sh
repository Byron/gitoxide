#!/usr/bin/env bash
set -eu -o pipefail

export GIT_INDEX_VERSION=2;

mkdir sub
(cd sub
  mkdir a b c
  mkdir c/d
  touch a/1 b/2 c/3 c/d/3
)

git init -q
git config index.threads 1

touch a b
chmod +x b
ln -s a c
mkdir d
(cd d && touch a b c
  mkdir nested
  (cd nested
    touch 1
  )
)

git add .
git commit -m "init"
