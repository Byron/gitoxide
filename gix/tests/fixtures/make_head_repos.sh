#!/bin/bash
set -eu -o pipefail

(git init symbolic && cd symbolic
  git commit -m "init" --allow-empty
)

git clone symbolic detached
(cd detached
  git remote rm origin
  git checkout @
)

git clone symbolic tag-symbolic
(cd tag-symbolic
  git tag -a -m make-tag-object point-at-commit HEAD
  git tag point-at-tag point-at-commit
  git tag -a -m make-tag-object point-at-tag-start point-at-tag
  git remote rm origin
  echo "ref: refs/tags/point-at-tag-start" > .git/HEAD
)

git clone tag-symbolic tag-detached
(cd tag-detached
  git remote rm origin
  git fetch --tags
  git rev-parse point-at-tag-start > .git/HEAD.tmp
  mv .git/HEAD.tmp .git/HEAD
)

