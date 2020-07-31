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
indent cargo diet -n --package-size-limit 20KB
(enter git-features && indent cargo diet -n --package-size-limit 6KB)
(enter git-ref && indent cargo diet -n --package-size-limit 4KB)
(enter git-object && indent cargo diet -n --package-size-limit 15KB)
(enter git-odb && indent cargo diet -n --package-size-limit 45KB)
(enter git-repository && indent cargo diet -n --package-size-limit 10KB)
(enter git-transport && indent cargo diet -n --package-size-limit 4KB)
(enter gitoxide-core && indent cargo diet -n --package-size-limit 10KB)
