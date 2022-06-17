#!/bin/bash
set -eu -o pipefail

ROOT="$PWD"

function baseline() {
  local spec=${1:?first argument is the spec to test}
  {
    echo "$spec"
    git rev-parse -q --verify "$spec" 2>/dev/null || echo $?
  }>> "$ROOT/baseline.git"
}

# The contents of this file is based on https://github.com/git/git/blob/8168d5e9c23ed44ae3d604f392320d66556453c9/t/t1512-rev-parse-disambiguation.sh#L38
git init --bare blob.prefix
(
  cd blob.prefix
  # Both start with "dead..", under both SHA-1 and SHA-256
  echo brocdnra | git hash-object -w --stdin
  echo brigddsv | git hash-object -w --stdin
  # Both start with "beef.."
  echo 1agllotbh | git hash-object -w --stdin
  echo 1bbfctrkc | git hash-object -w --stdin

  baseline "dead"
  baseline "beef"
)


git init --bare blob.bad
(
  cd blob.bad
  # Both have the prefix "bad0"
  # Maybe one day we have a test to see how disambiguation reporting deals with this.
  echo xyzfaowcoh | git hash-object -t bad -w --stdin --literally
  echo xyzhjpyvwl | git hash-object -t bad -w --stdin --literally
  baseline "bad0"

  echo 1bbfctrkc | git hash-object -t bad -w --stdin --literally
  baseline "e328"
  baseline "e328^{object}"
)

function oid_to_path() {
  local basename=${1#??}
  echo "${1%$basename}/$basename"
}

git init --bare blob.corrupt
(
  cd blob.corrupt
  # Both have the prefix "cafe".
  # Maybe one day we have a test to see how disambiguation reporting deals with this.
  echo bnkxmdwz | git hash-object -w --stdin
  oid=$(echo bmwsjxzi | git hash-object -w --stdin)
  oidf=objects/$(oid_to_path "$oid")
  chmod 755 $oidf
  echo broken >$oidf

  baseline "cafea"
  baseline "cafea^{object}"
)

# This function writes out its parameters, one per line
function write_lines () {
  	printf "%s\n" "$@"
}

function tick () {
  if test -z "${test_tick+set}"
  then
    test_tick=1112911993
  else
    test_tick=$(($test_tick + 60))
  fi
  GIT_COMMITTER_DATE="$test_tick -0700"
  GIT_AUTHOR_DATE="$test_tick -0700"
  export GIT_COMMITTER_DATE GIT_AUTHOR_DATE
}

git init ambiguous_blob_tree_commit
(
  GIT_AUTHOR_EMAIL=author@example.com
  GIT_AUTHOR_NAME='A U Thor'
  GIT_AUTHOR_DATE='1112354055 +0200'
  TEST_COMMITTER_LOCALNAME=committer
  TEST_COMMITTER_DOMAIN=example.com
  GIT_COMMITTER_EMAIL=committer@example.com
  GIT_COMMITTER_NAME='C O Mitter'
  GIT_COMMITTER_DATE='1112354055 +0200'

  tick
  cd ambiguous_blob_tree_commit
  (
    write_lines 0 1 2 3 4 5 6 7 8 9
    echo
    echo b1rwzyc3
  ) >a0blgqsjc
  # create one blob 0000000000b36
  git add a0blgqsjc
  # create one tree 0000000000cdc
  git write-tree

  sed -e "s/|$//" >patch <<-EOF
diff --git a/frotz b/frotz
index 000000000..ffffff 100644
--- a/frotz
+++ b/frotz
@@ -10,3 +10,4 @@
 9
 |
 b1rwzyc3
+irwry
EOF

  (
    GIT_INDEX_FILE=frotz
    export GIT_INDEX_FILE
    git apply --build-fake-ancestor frotz patch
  )

  # create one commit 0000000000e4f
  git commit -m a2onsxbvj

  # this commit is ffffffd8 and not ambiguous with the 00000* objects.
  # This is not relevant to gitoxide but we need the complete history.
  echo "hoaxj" | git commit-tree 0000000000cdc -p 000000000

  baseline "0000000000"
  baseline "0000000000cdc:a0blgqsjc" # unambiguous by nature
  baseline "0000000000:a0blgqsjc"    # would be ambiguous, but only trees can have this syntax
  baseline "0000000000cdc^{tree}"    # unambiguous with tree assertion
  baseline "0000000000^{tree}"       # ambiguous with tree assertion, but git can't do it
  baseline "0000000000e4f"           # unambiguous
  baseline "0000000000e4f^{commit}"  # and with commit assertion
  baseline "0000000000^{commit}"     # ambiguous name, but there is only one commit, but git can't do it
  baseline "0000000000^0"            # another transformation that only works for commits
  baseline "0000000000f8f"           # disambiguated tag
  baseline "0000000000^{tag}"        # disambiguated by type, but git can't do it

  baseline "000000000..000000000"    # only one commit is present with this prefix and we prefer these in ranges
  baseline "..000000000"
  baseline "000000000.."

  baseline "000000000...000000000"    # only one commit is present with this prefix and we prefer these in ranges
  baseline "...000000000"
  baseline "000000000..."

  baseline "v1.0.0-0-g0000000000e4f"  # unambiguous commit
  baseline "v1.0.0-0-g0000000000"     # ambiguous commit, but we know we need a commit here. The tag doesn't matter.

  # create one tag 0000000000f8f (making the previous baseline tests ambiguous, but it could be unambiguous since they point to the same commit)
  git tag -a -m j7cp83um v1.0.0
)
