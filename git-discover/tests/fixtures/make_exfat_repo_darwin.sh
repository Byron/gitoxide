#!/bin/bash
set -eu -o pipefail

[[ $(uname) == Darwin ]] || exit 1

dmg_file=exfat_repo.dmg
hdiutil create -size 10m -fs exfat -volname "test" $dmg_file

mount=exfat_mount
mkdir $mount
hdiutil attach $dmg_file -nobrowse -mountpoint $mount

(cd $mount
  git init -q
  git checkout -b main
  touch this
  git add this
  git commit -q -m c1
)

hdiutil detach $mount
rm -R $mount
