#!/bin/bash
set -eu -o pipefail

[[ $(uname) == Darwin ]] || exit 1

hdiutil create -size 10m -fs exfat -volname "test" exfat_repo.dmg
mkdir exfat_mount
hdiutil attach exfat_repo.dmg -nobrowse -mountpoint exfat_mount

(
  cd exfat_mount
  git init -q
  git checkout -b main
  touch this
  git add this
  git commit -q -m c1
)

hdiutil detach exfat_mount
rm -R exfat_mount
