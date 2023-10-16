#!/bin/bash
set -eu -o pipefail

bare="${1:-}"

mkdir repo
(
  cd repo
  git init -q

  git checkout -b main
  mkdir dir
  touch a b dir/c
  git add .
  git commit -q -m c1
  echo hello >> a
  git commit -q -am c2
)

(if [ "$bare" == "bare" ]; then
  git clone --bare --shared repo repo.git
  cd repo.git
else
  cd repo
fi

  git worktree add ../wt-a
  git worktree add ../prev/wt-a HEAD~1
  git worktree add ../wt-b HEAD~1
  git worktree add ../wt-a/nested-wt-b HEAD~1
  git worktree add --lock ../wt-c-locked
  git worktree add ../wt-deleted && rm -Rf ../wt-deleted

  git worktree list --porcelain > ../worktree-list.baseline
)


git --git-dir=repo-with-worktree-in-config-unborn-no-worktreedir --work-tree=does-not-exist-yet init
worktree=repo-with-worktree-in-config-unborn-worktree
git --git-dir=repo-with-worktree-in-config-unborn --work-tree=$worktree init && mkdir $worktree

repo=repo-with-worktree-in-config-unborn-empty-worktreedir
git --git-dir=$repo --work-tree="." init
git -C $repo config core.worktree ''

repo=repo-with-worktree-in-config-unborn-worktreedir-missing-value
git --git-dir=$repo init
touch $repo/index
git -C $repo config core.bare false
echo "    worktree" >> $repo/config

worktree=repo-with-worktree-in-config-worktree
git --git-dir=repo-with-worktree-in-config --work-tree=$worktree init
mkdir $worktree && touch $worktree/file
(cd repo-with-worktree-in-config
  git add file
  git commit -m "make sure na index exists"
)
