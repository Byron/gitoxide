#!/bin/bash
set -eu -o pipefail

set -x
git init
touch empty && git add empty
git commit -m upstream
git clone . super 
git clone super multiple
(cd multiple
  git submodule add ../multiple submodule
  git submodule add ../multiple a/b
  git submodule add --name .a/..c ../multiple a\\c
  git submodule add --name a/d\\ ../multiple a/d\\
  git submodule add --name a\\e ../multiple a/e/
  git commit -m "subsubmodule-a"
)

(cd super
  git submodule add ../multiple submodule
  git commit -m "submodule"
) 
git clone super super-clone 
(cd super-clone
  git submodule update --init --recursive
) 
git clone super empty-clone 
(cd empty-clone
  git submodule init
) 
git clone super top-only-clone 
git clone super relative-clone 
(cd relative-clone
  git submodule update --init --recursive
) 
git clone super recursive-clone 
(cd recursive-clone
  git submodule update --init --recursive
)

git clone super not-a-submodule
(cd not-a-submodule
  cp .gitmodules modules.bak
  git rm submodule
  echo fake > submodule
  mv modules.bak .gitmodules
  git add submodule && git commit -m "no submodule in index and commit, but in configuration"
)
