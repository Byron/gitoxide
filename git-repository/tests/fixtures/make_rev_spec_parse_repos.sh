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

# based on https://github.com/git/git/blob/8168d5e9c23ed44ae3d604f392320d66556453c9/t/t1512-rev-parse-disambiguation.sh#L38
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
  echo xyzfaowcoh | git hash-object -t bad -w --stdin --literally
  echo xyzhjpyvwl | git hash-object -t bad -w --stdin --literally
  baseline "bad0"

  echo 1bbfctrkc | git hash-object -t bad -w --stdin --literally
  baseline "e328"
)



