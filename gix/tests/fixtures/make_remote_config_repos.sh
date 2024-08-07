#!/usr/bin/env bash
set -eu -o pipefail

(mkdir fetch && cd fetch
  git init -q

  git checkout -b main

  git commit --allow-empty -q -m c1
  git branch broken

  git remote add --fetch remote_repo .
  git branch --set-upstream-to remote_repo/main

  git checkout broken
  git branch --set-upstream-to remote_repo/broken

  git config branch.broken.merge not_a_valid_merge_ref
  git config push.default simple
)

(mkdir push-mapped && cd push-mapped
  git init -q

  git checkout -b main
  git commit --allow-empty -q -m c1

  cat<<EOF >.git/config
[remote "origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/origin/*
    push = refs/heads/main ; this should be ignored
    push = refs/heads/main:refs/heads/remapped-main
    push = refs/heads/main:refs/heads/skipped ; skipped as it's not the first matching one
    push = refs/heads/feature:refs/heads/remapped-feature ; this is picked up before going to push.default (which would fail)

[branch "main"]
    remote = "origin"
    merge = refs/heads/main

[push]
  default = simple

[branch "feature"]
    remote = "origin"
    merge = refs/heads/main  ; this one is remapped to merge from main, which doesn't affect the push remote.
EOF
)

(mkdir push-missing && cd push-missing
  git init -q

  git checkout -b main
  git commit --allow-empty -q -m c1

  cat<<EOF >.git/config
[remote "origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/origin/*
    push = refs/heads/main ; there is a match, but no destination is available

[push]
  default = current ; this could work, but the default isn't looked at if there are any push specs

[branch "main"]
    remote = "origin"
    merge = refs/heads/main
EOF
)

(mkdir push-default-current && cd push-default-current
  git init -q

  git checkout -b main
  git commit --allow-empty -q -m c1

  cat<<EOF >.git/config
[remote "origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/origin/*

[push]
  default = current ; this would be the one setting that works as it ignores 'branch.main.merge'

[branch "main"]
    remote = "origin"
    merge = refs/heads/other
EOF
)

(mkdir push-remote && cd push-remote
  git init -q

  git checkout -b main
  git commit --allow-empty -q -m c1

  cat<<EOF >.git/config
[remote "origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/origin/*

[remote "push-origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/push-remote/*

[branch "main"]
    remote = "origin"
    pushRemote = push-origin
    merge = refs/heads/other
EOF
)


(mkdir push-remote-default && cd push-remote-default
  git init -q

  git checkout -b main
  git commit --allow-empty -q -m c1

  cat<<EOF >.git/config

[remote "push-origin"]
    url = .
    fetch = +refs/heads/*:refs/remotes/push-remote/*

[branch "main"]
    remote = "origin"
    merge = refs/heads/other

[remote]
    pushDefault = push-origin
EOF
)

git clone fetch multiple-remotes
(cd multiple-remotes
  git remote add other ../fetch && git fetch other
  git remote add with/two/slashes ../fetch && git fetch with/two/slashes
  git remote add with/two ../fetch && git fetch with/two

  git checkout -b main --track origin/main
  git checkout -b other-main --track other/main
)