#!/bin/bash

set -eu -o pipefail

function enter () {
  local dir="${1:?need directory to enter}"
  echo -n $'  in' $dir $'\t→\t'
  cd $dir
}

function indent () {
  "$@" | grep "package size" | while read -r line; do
    echo "    " $line
  done
}

echo "in root: gitoxide CLI"
(enter cargo-smart-release && indent cargo diet -n --package-size-limit 100KB)
(enter git-actor && indent cargo diet -n --package-size-limit 5KB)
(enter git-pathspec && indent cargo diet -n --package-size-limit 25KB)
(enter git-refspec && indent cargo diet -n --package-size-limit 25KB)
(enter git-path && indent cargo diet -n --package-size-limit 20KB)
(enter git-attributes && indent cargo diet -n --package-size-limit 20KB)
(enter git-discover && indent cargo diet -n --package-size-limit 25KB)
(enter git-index && indent cargo diet -n --package-size-limit 50KB)
(enter git-worktree && indent cargo diet -n --package-size-limit 35KB)
(enter git-quote && indent cargo diet -n --package-size-limit 10KB)
(enter git-revision && indent cargo diet -n --package-size-limit 35KB)
(enter git-bitmap && indent cargo diet -n --package-size-limit 10KB)
(enter git-tempfile && indent cargo diet -n --package-size-limit 30KB)
(enter git-lock && indent cargo diet -n --package-size-limit 20KB)
(enter git-config && indent cargo diet -n --package-size-limit 120KB)
(enter git-config-value && indent cargo diet -n --package-size-limit 20KB)
(enter git-command && indent cargo diet -n --package-size-limit 5KB)
(enter git-hash && indent cargo diet -n --package-size-limit 30KB)
(enter git-chunk && indent cargo diet -n --package-size-limit 10KB)
(enter git-rebase && indent cargo diet -n --package-size-limit 5KB)
(enter git-sequencer && indent cargo diet -n --package-size-limit 5KB)
(enter git-features && indent cargo diet -n --package-size-limit 55KB)
(enter git-ref && indent cargo diet -n --package-size-limit 50KB)
(enter git-diff && indent cargo diet -n --package-size-limit 10KB)
(enter git-traverse && indent cargo diet -n --package-size-limit 10KB)
(enter git-url && indent cargo diet -n --package-size-limit 25KB)
(enter git-validate && indent cargo diet -n --package-size-limit 5KB)
(enter git-date && indent cargo diet -n --package-size-limit 15KB)
(enter git-hashtable && indent cargo diet -n --package-size-limit 5KB)
(enter git-filter && indent cargo diet -n --package-size-limit 5KB)
(enter git-lfs && indent cargo diet -n --package-size-limit 5KB)
(enter git-note && indent cargo diet -n --package-size-limit 5KB)
(enter git-fetchhead && indent cargo diet -n --package-size-limit 5KB)
(enter git-sec && indent cargo diet -n --package-size-limit 15KB)
(enter git-tix && indent cargo diet -n --package-size-limit 5KB)
(enter git-credentials && indent cargo diet -n --package-size-limit 30KB)
(enter git-prompt && indent cargo diet -n --package-size-limit 15KB)
(enter git-object && indent cargo diet -n --package-size-limit 25KB)
(enter git-commitgraph && indent cargo diet -n --package-size-limit 30KB)
(enter git-pack && indent cargo diet -n --package-size-limit 125KB)
(enter git-odb && indent cargo diet -n --package-size-limit 130KB)
(enter git-protocol && indent cargo diet -n --package-size-limit 80KB)
(enter git-packetline && indent cargo diet -n --package-size-limit 35KB)
(enter git-repository && indent cargo diet -n --package-size-limit 240KB)
(enter git-transport && indent cargo diet -n --package-size-limit 75KB)
(enter gitoxide-core && indent cargo diet -n --package-size-limit 100KB)
