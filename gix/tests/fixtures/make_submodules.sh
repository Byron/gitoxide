#!/bin/bash
set -eu -o pipefail

git init -q module1
(cd module1
  touch this
  mkdir subdir
  touch subdir/that
  git add .
  git commit -q -m c1
  echo hello >> this
  git commit -q -am c2
)

git init with-submodules
(cd with-submodules
  mkdir dir
  touch dir/file
  git add dir
  git commit -m "init"

  git submodule add ../module1 m1
  git commit -m "add module 1"

  git submodule add ../module1 dir/m1
)

cp -R with-submodules with-submodules-in-index
(cd with-submodules-in-index
  git add .
  rm .gitmodules
)

cp -R with-submodules with-submodules-in-tree
(cd with-submodules-in-tree
  rm .gitmodules
  rm .git/index
)

git init old-form-invalid-worktree-path
(cd old-form-invalid-worktree-path
  mkdir dir
  git submodule add --name old ../module1 dir/old-form
  rm dir/old-form/.git
  # the config file contains a worktree path that points to the wrong place now
  mv .git/modules/old dir/old-form/.git
)

cp -R old-form-invalid-worktree-path old-form
(cd old-form
  cd dir/old-form
  git config --unset core.worktree
)

git clone with-submodules with-submodules-after-clone
(cd with-submodules-after-clone
  git submodule init m1
)
git clone --bare with-submodules with-submodules-after-clone.git
(cd with-submodules-after-clone.git
  # manually create a clone and see if we can handle it despite being bare
  git clone --bare ../module1 modules/m1
)

git clone with-submodules not-a-submodule
(cd not-a-submodule
  git submodule update --init
  cp .gitmodules modules.bak
  git rm m1
  echo fake > m1
  mv modules.bak .gitmodules
  git add m1 && git commit -m "no submodule in index and commit, but in configuration"
)
