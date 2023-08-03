#!/bin/bash
set -eu -o pipefail

git init

mkdir dir-ignored
touch dir-ignored/file-ignored-transitively
touch file-ignored

echo "hi" > a
mkdir dir
echo "ho" > dir/b
mkdir dir/subdir

touch dir/subdir/exe
chmod +x dir/subdir/exe
ln -s a symlink-to-a

echo "/dir-ignored/ export-ignore" > .gitattributes
echo "/file-ignored export-ignore" >> .gitattributes

git add .
git commit -m "init"

echo "extra to be streamed" > extra-file
touch extra-exe && chmod +x extra-exe
mkdir extra-dir-empty extra-dir
ln -s ../extra-file extra-dir/symlink-to-extra

git rev-parse @^{tree} > head.hex

