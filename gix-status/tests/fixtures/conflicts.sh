#!/bin/bash
set -eu -o pipefail

(git init both-deleted && cd both-deleted
  echo test > file
  git add file && git commit -m file &&
  git branch alt && git mv file added-by-them
  git commit -m "file renamed in added-by-them" && git checkout alt
  git mv file added-by-us
  git commit -m "file renamed in added-by-us"
  git reset --hard alt
  git merge main || :
)

(git init deleted-by-us && cd deleted-by-us
		git init
		>file && git add file && git commit -m "initial"
		echo change >> file && git commit -am "modify"
		git checkout -b side HEAD^
		git rm file
		git commit -m delete
		git merge main || :
)

(git init deleted-by-them && cd deleted-by-them
  echo "This is some content." > file
  git add file
  git commit -m "Initial commit"
  git checkout -b conflict
  git rm file
  git commit -m "Delete file in feature branch"
  git checkout main
  echo "Modified by main branch." >> file
  git add file
  git commit -m "Modified file in main branch"
  git merge conflict || :
)

(git init both-modified && cd both-modified
  git init
  > file && git add file && git commit -m "init"

  git checkout -b conflict
  echo conflicting >> file && git commit -am "alt-change"

  git checkout main
  echo other >> file && git commit -am "change"

  git merge conflict || :
)

(git init both-added && cd both-added
  git init
  set -x
	echo init >> deleted-by-them && git add . && git commit -m "init"

	git checkout -b second_branch
	git rm deleted-by-them
	git commit -m "deleted-by-them deleted on second_branch"
	echo second > both-added && git add . && git commit -m second

	git checkout main
	echo on_second > deleted-by-them && git commit -am "on second"
	echo main > both-added && git add . && git commit -m main

	git merge second_branch || :
)
