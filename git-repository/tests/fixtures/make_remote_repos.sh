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
(cd clone
  git remote add myself .
)

git clone --shared base push-default
(cd push-default

  git remote add myself .
  git remote rename origin new-origin
  git config remote.pushDefault myself
)

git clone --shared base push-url
(cd push-url
  git config remote.origin.pushUrl .
  git config remote.origin.push refs/tags/*:refs/tags/*
)

git clone --shared base many-fetchspecs
(cd many-fetchspecs
  git config --add remote.origin.fetch @
  git config --add remote.origin.fetch refs/tags/*:refs/tags/*
  git config --add remote.origin.fetch HEAD
)

git clone --shared base branch-push-remote
(cd branch-push-remote

  git remote rename origin new-origin
  git remote add myself .
  git config branch.main.pushRemote myself
)

git clone --shared base branch-dot-remote
(cd branch-dot-remote

  git config branch.main.remote .
)

git init --bare url-rewriting
(cd url-rewriting

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
(cd bad-url-rewriting

  git remote add origin https://github.com/foobar/gitoxide
  cat <<EOF >> config

[remote "origin"]
  pushUrl = "file://dev/null"

[url "invalid:://"]
  pushInsteadOf = "file://"

[url "https://github.com/byron/"]
  insteadOf = https://github.com/foobar/
EOF

  {
    git remote get-url origin
    git remote get-url origin --push
  } > baseline.git
)

git clone --shared base protocol_denied
(cd protocol_denied
    git config protocol.allow never
)

git clone --shared base protocol_file_denied
(cd protocol_file_denied
    git config protocol.file.allow never
)

git clone --shared base protocol_file_user
(cd protocol_file_user
    git config protocol.file.allow user
)

git clone --shared base remote-as-url
(cd remote-as-url
  cat <<EOF >> .git/config
[branch "main"]
  remote = https://example.com/fetch-path.git
  pushRemote = https://example.com/push-path.git
EOF
)



git clone --shared base credential-helpers
(cd credential-helpers
    export GIT_TERMINAL_PROMPT=0
    git=$(which git)
    function baseline() {
      local url=${1:?need url}
      {
        echo $url
        echo url=$url | GIT_TRACE=1 $git credential fill 2>&1 | grep -E '^[a-z]+:' || :
      } >> baseline.git
    }

    git config credential.helper ""
    git config --add credential.helper global
    git config 'credential.https://*.helper' 'https://*'
    git config 'credential.http://*.helper' 'http://*'
    git config 'credential.http://*.com.helper' 'http://*.com'
    git config 'credential.http://example.*.helper' 'http://example.*'
    git config 'credential.http://example.?om.helper' 'http://example.?om'
    git config 'credential.http://*.example.com.helper' 'http://*.example.com'
    git config 'credential.http://a.*.example.com.helper' 'http://a.*.example.com'
    git config 'credential.HTTPS://example.com.helper' 'HTTPS://example.com'
    git config credential.http://example.com:80.helper http://example.com:80
    git config credential.https://example.com:443.helper https://example.com:443
    git config credential.http://example.com:8080.helper http://example.com:8080
    git config credential.https://example.com:8080.helper https://example.com:8080
    git config credential.https://example.com:8080/path.helper https://example.com:8080/path
    git config credential.https://example.com:8080/path.usehttppath 1
    git config credential.https://example.com:8080/clear.helper ""
    git config --add credential.https://example.com:8080/clear.helper credential.https://example.com:8080/clear
    git config credential.https://user@example.com/with-user.helper https://user@example.com/with-user
    git config credential.https://user@example.com.helper https://user@example.com
    git config credential.ssh://user@host/with-user.helper ssh://user@host/with-user
    git config credential.ssh://host/with-user.helper ssh://host/with-user
    git config credential.ssh://host:21/path.helper ssh://host:21/path
    git config credential.ssh://host/path.helper ssh://host/path
    git config credential.git://host.org.helper git://host.org

    git config credential.https://dev.azure.com.usehttppath true

    baseline "https://hit-global.helper"
    baseline "http://host"
    baseline "http://example.com:80"
    baseline "http://example.com:80/"
    baseline "http://example.com"
    baseline "http://a.example.com"
    baseline "http://b.example.com/path"
    baseline "http://c.example.com:80/path"
    baseline "http://a.a.example.com:80/path"
    baseline "http://a.b.example.com/path"
    baseline "http://b.a.example.com/path"
    baseline "https://example.com"
    baseline "https://EXAMPLE.com"
    baseline "HTTPS://example.com"
    baseline "https://example.COM"
    baseline "https://example.com/"
    baseline "https://example.com:443"
    baseline "https://example.com:443/"
    baseline "http://example.com:8080/other/path"
    baseline "https://example.com:8080/other/path"
    baseline "https://example.com:8080/path"
    baseline "https://example.com:8080/PATH"
    baseline "https://example.com:8080/path/"
    baseline "https://example.com:8080/clear"
    baseline "https://example.com/with-user"
    baseline "https://user@example.com/with-user"
    baseline "ssh://host/with-user"
    baseline "ssh://user@host/with-user"
    baseline "ssh://host/path"
    baseline "ssh://host/PATH"
    baseline "ssh://host:21/path"
    baseline "ssh://host:21"
    baseline "ssh://host"
    baseline "git://host.org"
)

git clone --shared base detached-head
(cd detached-head
  git checkout @~1
)
