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
echo "new content" > dir/content2
chmod -x executable
echo -n "foo" > executable

rm empty
ln -sf dir/content empty
git reset 