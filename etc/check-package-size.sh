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
(enter cargo-smart-release && indent cargo diet -n --package-size-limit 110KB)
(enter gix-actor && indent cargo diet -n --package-size-limit 5KB)
(enter gix-pathspec && indent cargo diet -n --package-size-limit 30KB)
(enter gix-refspec && indent cargo diet -n --package-size-limit 30KB)
(enter gix-path && indent cargo diet -n --package-size-limit 25KB)
(enter gix-attributes && indent cargo diet -n --package-size-limit 25KB)
(enter gix-discover && indent cargo diet -n --package-size-limit 30KB)
(enter gix-index && indent cargo diet -n --package-size-limit 60KB)
(enter gix-worktree && indent cargo diet -n --package-size-limit 40KB)
(enter gix-quote && indent cargo diet -n --package-size-limit 10KB)
(enter gix-revision && indent cargo diet -n --package-size-limit 40KB)
(enter gix-bitmap && indent cargo diet -n --package-size-limit 10KB)
(enter gix-tempfile && indent cargo diet -n --package-size-limit 35KB)
(enter gix-lock && indent cargo diet -n --package-size-limit 20KB)
(enter gix-config && indent cargo diet -n --package-size-limit 140KB)
(enter gix-config-value && indent cargo diet -n --package-size-limit 20KB)
(enter gix-command && indent cargo diet -n --package-size-limit 10KB)
(enter gix-hash && indent cargo diet -n --package-size-limit 30KB)
(enter gix-chunk && indent cargo diet -n --package-size-limit 15KB)
(enter gix-rebase && indent cargo diet -n --package-size-limit 5KB)
(enter gix-sequencer && indent cargo diet -n --package-size-limit 5KB)
(enter gix-features && indent cargo diet -n --package-size-limit 65KB)
(enter gix-ref && indent cargo diet -n --package-size-limit 55KB)
(enter gix-diff && indent cargo diet -n --package-size-limit 10KB)
(enter gix-traverse && indent cargo diet -n --package-size-limit 10KB)
(enter gix-url && indent cargo diet -n --package-size-limit 35KB)
(enter gix-validate && indent cargo diet -n --package-size-limit 5KB)
(enter gix-date && indent cargo diet -n --package-size-limit 25KB)
(enter gix-hashtable && indent cargo diet -n --package-size-limit 5KB)
(enter gix-filter && indent cargo diet -n --package-size-limit 5KB)
(enter gix-lfs && indent cargo diet -n --package-size-limit 5KB)
(enter gix-note && indent cargo diet -n --package-size-limit 5KB)
(enter gix-fetchhead && indent cargo diet -n --package-size-limit 5KB)
(enter gix-sec && indent cargo diet -n --package-size-limit 25KB)
(enter gix-tix && indent cargo diet -n --package-size-limit 5KB)
(enter gix-credentials && indent cargo diet -n --package-size-limit 35KB)
(enter gix-prompt && indent cargo diet -n --package-size-limit 15KB)
(enter gix-object && indent cargo diet -n --package-size-limit 25KB)
(enter gix-commitgraph && indent cargo diet -n --package-size-limit 35KB)
(enter gix-pack && indent cargo diet -n --package-size-limit 135KB)
(enter gix-odb && indent cargo diet -n --package-size-limit 140KB)
(enter gix-protocol && indent cargo diet -n --package-size-limit 80KB)
(enter gix-packetline && indent cargo diet -n --package-size-limit 45KB)
(enter gix && indent cargo diet -n --package-size-limit 280KB)
(enter gix-transport && indent cargo diet -n --package-size-limit 95KB)
(enter gitoxide-core && indent cargo diet -n --package-size-limit 120KB)
