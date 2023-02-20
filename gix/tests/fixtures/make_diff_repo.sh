#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
mkdir dir
touch a b dir/c d
git add .
git commit -q -m "c1 - initial"

echo a >> a
echo b >> b
echo dir/c >> dir/c
echo d >> d
git commit -q -am "c2"

echo a1 >> a
git commit -q -am "c3-modification"

git mv a dir/a-moved
git commit -m "r1-identity"

touch s1 s2 s3
git add s* && git commit -m "c4 - add identical files"

git mv s1 z && git mv s2 b2 && git mv s3 b1
git commit -m "r2-ambiguous"

git mv dir/c dir/c-moved
echo n >> dir/c-moved
echo n >> b
git commit -am "r3" # modified rename and normal modification

touch lt1 lt2
ln -s lt1 link-1
echo lt1 > no-link # a file that has content like a link and a similar name
ln -s ../lt2 dir/link-2
git add . && git commit -m "c5 - add links"

git mv link-1 renamed-link-1
git rm no-link
git rm dir/link-2 && ln -s lt1 z-link-2 && git add .
git commit -m "r4-symlinks" # symlinks are only tracked by identity

seq 10 > f1
seq 11 > f2
git add . && git commit -m "c6 - two files with more content"

echo n >> f1
echo n >> f2
git mv f1 f1-renamed
git mv f2 f2-renamed

git commit -am "r5" # two renames


seq 9 > base
git add base
git commit -m "c7" # base has to be added

echo 10 >> base
cp base c1
cp base c2
cp base dir/c3
git add . && git commit -m "tc1-identity"

echo 11 >> base
cp base c4 # can be located by identity
cp base c5 && echo 12 >> c5
cp base dir/c6 && echo 13 >> dir/c6
git add . && git commit -m "tc2-similarity"

cp base c6 # can be located by identity, but base needs --find-copies-harder
cp base c7 && echo 13 >> c7 # modified copy, similarity and find copies harder
seq 15 > newly-added
echo nn >> b
git add .
git commit -m "tc3-find-harder"
