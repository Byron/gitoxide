#!/bin/bash
set -eu -o pipefail

git init -q

touch empty
echo -n "content" > executable
chmod +x executable

mkdir dir
echo "other content" > dir/content
echo "* filter=arrow" > .gitattributes
echo "executable -filter" >> .gitattributes
echo ".gitattributes -filter" >> .gitattributes

mkdir dir/sub-dir
(cd dir/sub-dir && ln -sf ../content symlink)

git add -A
git commit -m "Commit"
