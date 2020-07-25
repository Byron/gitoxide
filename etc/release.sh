#!/bin/bash

set -eu -o pipefail

./etc/check-package-size.sh

for crate in git-features git-object git-odb git-repository gitoxide-core .; do
  (cd $crate && cargo release "$@")
done
