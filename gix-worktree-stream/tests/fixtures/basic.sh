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
echo "subdir/streamed filter=arrow" > dir/.gitattributes
echo "streamed-by-driver" > dir/subdir/streamed
touch dir/subdir/exe
chmod +x dir/subdir/exe
ln -s a symlink-to-a

echo "/dir-ignored/ export-ignore" > .gitattributes
echo "/file-ignored export-ignore" >> .gitattributes

dd if=/dev/zero of=bigfile bs=1024 count=156

git add .
git commit -m "init"

echo "extra" > extra-file
touch extra-exe && chmod +x extra-exe
mkdir extra-dir-empty extra-dir
ln -s ../extra-file extra-dir/symlink-to-extra
dd if=/dev/zero of=extra-bigfile bs=1024 count=156

git rev-parse @^{tree} > head.hex

