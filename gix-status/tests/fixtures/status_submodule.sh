#!/bin/bash
set -eu -o pipefail

git init -q module1
(cd module1
  touch this
  git add .
  git commit -q -m c1
  echo hello >> this
  git commit -q -am c2
)

git init no-change
(cd no-change
  git submodule add ../module1 m1
  git commit -m "add module 1"
)

cp -R no-change deleted-dir
(cd deleted-dir
  rm -Rf m1
)

cp -R no-change type-change
(cd type-change
  rm -Rf m1
  touch m1
)

cp -R no-change empty-dir-no-change
(cd empty-dir-no-change
  rm -Rf m1
  mkdir m1
)

cp -R no-change conflict
(cd conflict
  (cd m1
    git checkout @~1
  )

  git commit -am "change submodule head"
  git checkout -b other @~1
  git rm -rf m1
  git commit -m "removed submodule"

  git merge main || :
)
