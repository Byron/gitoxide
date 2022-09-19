#!/bin/bash
set -eu -o pipefail

git init -q

git checkout -b main
mkdir dir
touch a b dir/c
git add .
git commit -q -m c1

echo a >> a
echo b >> b
echo dir/c >> dir/c
git commit -q -am c2

echo a1 >> a
git commit -q -am c3
