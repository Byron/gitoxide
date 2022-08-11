set -eu -o pipefail

function tick () {
  if test -z "${tick+set}"
  then
    tick=1112911993
  else
    tick=$(($tick + 60))
  fi
  GIT_COMMITTER_DATE="$tick -0700"
  GIT_AUTHOR_DATE="$tick -0700"
  export GIT_COMMITTER_DATE GIT_AUTHOR_DATE
}

GIT_AUTHOR_EMAIL=author@example.com
GIT_AUTHOR_NAME='A U Thor'
GIT_AUTHOR_DATE='1112354055 +0200'
TEST_COMMITTER_LOCALNAME=committer
TEST_COMMITTER_DOMAIN=example.com
GIT_COMMITTER_EMAIL=committer@example.com
GIT_COMMITTER_NAME='C O Mitter'
GIT_COMMITTER_DATE='1112354055 +0200'

# runup to the correct count for ambigous commits
tick; tick; tick; tick; tick

git init base
(
  cd base
  tick

  echo g > file
  git add file && git commit -m $'G\n\n initial message'
  git branch g

  tick
  git checkout --orphan=h
  echo h > file
  git add file && git commit -m H

  tick
  git checkout main
  git merge h --allow-unrelated-histories || :
  { echo g && echo h && echo d; } > file
  git add file
  git commit -m D
  git branch d

  tick
  git checkout --orphan=i
  echo i > file
  git add file && git commit -m I
  git tag -m I-tag i-tag

  tick
  git checkout --orphan=j
  echo j > file
  git add file && git commit -m J

  tick
  git checkout i
  git merge j --allow-unrelated-histories || :
  { echo i && echo j && echo f; } > file
  git add file
  git commit -m F
  git branch f

  tick
  git checkout --orphan=e
  echo e > file
  git add file && git commit -m E

  tick
  git checkout main
  git merge e i --allow-unrelated-histories || :
  { echo g && echo h && echo i && echo j && echo d && echo e && echo f && echo b; } > file
  git add file && git commit -m B
  git tag -m b-tag b-tag && git branch b

  tick
  git checkout i
  echo c >> file
  git add file && git commit -m $'C\n\n message recent'
  git branch c
  git reset --hard i-tag

  tick
  git checkout main
  git merge c || :
  { echo g && echo h && echo i && echo j && echo d && echo e && echo f && echo b && echo c && echo a; } > file
  git add file && git commit -m A
  git branch a
)

git clone --shared base clone
(
  cd clone
  git remote add myself .
)

git clone --shared base push-default
(
  cd push-default

  git remote add myself .
  git remote rename origin new-origin
  git config remote.pushDefault myself
)

git clone --shared base push-url
(
  cd push-url
  git config remote.origin.pushUrl .
  git config remote.origin.push refs/tags/*:refs/tags/*
)

git clone --shared base many-fetchspecs
(
  cd many-fetchspecs
  git config --add remote.origin.fetch @
  git config --add remote.origin.fetch refs/tags/*:refs/tags/*
  git config --add remote.origin.fetch HEAD
)

git clone --shared base branch-push-remote
(
  cd branch-push-remote

  git remote rename origin new-origin
  git remote add myself .
  git config branch.main.pushRemote myself
)

git init --bare url-rewriting
(
  cd url-rewriting

  git remote add origin https://github.com/foobar/gitoxide
  cat <<EOF >> config

[remote "origin"]
  pushUrl = "file://dev/null"

[url "ssh://"]
  insteadOf = "https://"
  pushInsteadOf = "file://"

[url "https://github.com/byron/"]
  insteadOf = https://github.com/foobar/
  pushInsteadOf = ssh://example.com/
EOF

  {
    git remote get-url origin
    git remote get-url origin --push
  } > baseline.git
)

git init --bare bad-url-rewriting
(
  cd bad-url-rewriting

  git remote add origin https://github.com/foobar/gitoxide
  cat <<EOF >> config

[remote "origin"]
  pushUrl = "file://dev/null"

[url "foo://"]
  pushInsteadOf = "file://"

[url "https://github.com/byron/"]
  insteadOf = https://github.com/foobar/
EOF

  {
    git remote get-url origin
    git remote get-url origin --push
  } > baseline.git
)
