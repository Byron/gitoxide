#!/usr/bin/env bash
set -eu -o pipefail

git init -q changed-and-untracked
(cd changed-and-untracked
  touch empty
  echo "content" > executable
  chmod +x executable

  mkdir dir
  echo "other content" > dir/content
  echo "different content" > dir/content2

  git add -A
  git commit -m "Commit"
  echo "change" >> executable


  mkdir dir/empty
  >dir/untracked
  >untracked

  git status
)

cp -R changed-and-untracked changed-and-untracked-and-renamed
(cd changed-and-untracked-and-renamed
  # it has a local change compared to the indexed version, hence it's rewritten
  mv executable rewritten-executable

  cp dir/content content-copy
  cp dir/content content-copy-with-rewrite
  echo change >> content-copy-with-rewrite

  mv dir/content plainly-renamed-content

  mv dir/content2 content-with-rewrite
  echo change >> content-with-rewrite

)
