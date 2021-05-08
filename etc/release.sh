#!/bin/bash

set -eu -o pipefail

./etc/check-package-size.sh

for crate in git-features git-url git-hash git-ref git-object git-traverse git-diff git-odb git-packetline git-transport git-protocol git-repository gitoxide-core .; do
  (cd $crate && cargo release "$@")
done
