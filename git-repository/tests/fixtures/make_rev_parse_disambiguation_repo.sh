#!/bin/bash
set -eu -o pipefail

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
)



