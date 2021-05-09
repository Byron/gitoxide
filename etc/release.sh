#!/bin/bash

set -eu -o pipefail

#./etc/check-package-size.sh

utils="${1?The first argument is the 'utils' tool}"
shift

for crate in git-features git-url git-hash git-ref git-object git-traverse git-diff git-odb git-packetline git-transport git-protocol git-repository gitoxide-core .; do
  version_info=$($utils crate-path "$crate")
  version_path="etc/crates/$version_info"
  version_dir="${version_path%/*}"
  if [[ -f "$version_path" ]]; then
    echo "Skipping '$version_info' as it is published already"
    continue
  fi
  echo "Publishing '${version_info}'…"
  (cd $crate && cargo release "$@")
  mkdir -p "$version_dir"
  touch "$version_path"
  # Need to keep the working dir clean or cargo release refuses to work
  git add "$version_dir" && printf "[track publish] %s\n\n[skip ci]" "$version_info" | git commit -F -
done

git push --tags origin main
