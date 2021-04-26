#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false

git checkout -q -b main

touch f
git add f
git commit -qm 'f added'

echo m > f
git commit -qam 'f modified'

rm f
git commit -qam 'f deleted'

echo m > f
git add f
git commit -qam 'f re-added same content'

mv f f.tmp
mkdir f
mv f.tmp f/f
git add f/
git commit -qam 'f mode modified to dir f/'

echo m >> f/f
git commit -qam 'f/f modified'

touch a
git add a
git commit -qam 'a added'

touch f/a f/b
git add f/
git commit -qam 'f/a f/b added'

rm f/f
ln -s f/a f/f
git commit -qam 'f/f mode changed to link'

git mv a b
git commit -qam 'a renamed to b'



