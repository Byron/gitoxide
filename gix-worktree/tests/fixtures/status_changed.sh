#!/bin/bash
set -eu -o pipefail

git init -q

touch empty
echo -n "content" > executable
chmod +x executable

mkdir dir
echo -n "other content" > dir/content
echo -n "other content" > dir/content2
mkdir dir/sub-dir
(cd dir/sub-dir && ln -sf ../content symlink)

git add -A
git commit -m "Commit"

chmod +x dir/content
chmod +x dir/content2
echo "new content" > dir/content2
rm empty
ln -sf dir/content empty
git reset 