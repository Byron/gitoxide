#!/bin/bash
set -eu -o pipefail

git init -q
git config merge.ff false

# Commit in year 2000
git checkout -q -b main
GIT_COMMITTER_DATE="2000-01-02 00:00:00 +0000" git commit -q --allow-empty -m c1 #134385f6d781b7e97062102c6a483440bfda2a03-

# Commit in year 2001
git checkout -q -b branch1
GIT_COMMITTER_DATE="2001-01-02 00:00:00 +0000" git commit -q --allow-empty -m b1c1 #bcb05040a6925f2ff5e10d3ae1f9264f2e8c43ac-

# Commit in year 2000
git checkout -q main
GIT_COMMITTER_DATE="2000-01-02 00:00:00 +0000" git commit -q --allow-empty -m c2 #9902e3c3e8f0c569b4ab295ddf473e6de763e1e7-


git commit-graph write --no-progress --reachable
git repack -adq

# Commit from branch1 made in 2001 merged in 2002
GIT_COMMITTER_DATE="2002-01-02 00:00:00 +0000" git merge branch1 -m m1b1 #288e509293165cb5630d08f4185bdf2445bf6170-
