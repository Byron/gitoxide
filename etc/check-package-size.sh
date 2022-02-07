#!/bin/bash

set -eu -o pipefail

function enter () {
  local dir="${1:?need directory to enter}"
  echo $'  in' $dir
  cd $dir
}

function indent () {
  "$@" | grep "package size" | while read -r line; do
    echo "    " $line
  done
}

echo "in root: gitoxide CLI"
#indent cargo diet -n --package-size-limit 25KB - fails right now because of dotted profile.dev.package
(enter cargo-smart-release && indent cargo diet -n --package-size-limit 85KB)
(enter git-actor && indent cargo diet -n --package-size-limit 5KB)
(enter git-index && indent cargo diet -n --package-size-limit 30KB)
(enter git-worktree && indent cargo diet -n --package-size-limit 10KB)
(enter git-revision && indent cargo diet -n --package-size-limit 10KB)
(enter git-bitmap && indent cargo diet -n --package-size-limit 5KB)
(enter git-tempfile && indent cargo diet -n --package-size-limit 25KB)
(enter git-lock && indent cargo diet -n --package-size-limit 15KB)
(enter git-config && indent cargo diet -n --package-size-limit 65KB)
(enter git-hash && indent cargo diet -n --package-size-limit 15KB)
(enter git-chunk && indent cargo diet -n --package-size-limit 10KB)
(enter git-features && indent cargo diet -n --package-size-limit 40KB)
(enter git-ref && indent cargo diet -n --package-size-limit 50KB)
(enter git-diff && indent cargo diet -n --package-size-limit 10KB)
(enter git-traverse && indent cargo diet -n --package-size-limit 10KB)
(enter git-url && indent cargo diet -n --package-size-limit 10KB)
(enter git-validate && indent cargo diet -n --package-size-limit 5KB)
(enter git-object && indent cargo diet -n --package-size-limit 25KB)
(enter git-commitgraph && indent cargo diet -n --package-size-limit 25KB)
(enter git-pack && indent cargo diet -n --package-size-limit 115KB)
(enter git-odb && indent cargo diet -n --package-size-limit 115KB)
(enter git-protocol && indent cargo diet -n --package-size-limit 50KB)
(enter git-packetline && indent cargo diet -n --package-size-limit 35KB)
(enter git-repository && indent cargo diet -n --package-size-limit 70KB)
(enter git-transport && indent cargo diet -n --package-size-limit 50KB)
(enter gitoxide-core && indent cargo diet -n --package-size-limit 50KB)
