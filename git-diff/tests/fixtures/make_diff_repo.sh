#!/bin/bash
set -eu -o pipefail

git init -q
git config commit.gpgsign false
git config core.autocrlf false

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
ln -s a f/f
git commit -qam 'f/f mode changed to link'

git mv a b
git commit -qam 'a renamed to b'

git rm -r f
touch f
git add f
git commit -qam 'f/ changed into file f'

mkdir d
touch d/f
git add d
git commit -qam 'add d/f'

rm -r d/
git commit -qam 'delete d/'

touch c d e
git add .
git commit -qam 'add /c /d /e'

mkdir g
touch g/a
git add g
git commit -qam 'add g/a'

git rm c d e
git commit -qam 'remove /c /d /e'

git rm f
touch ff
git add ff
git commit -qam 'rm /f, add /ff'

touch g/aa
git rm g/a
git add g/aa
git commit -qam 'rm g/a, add g/aa'

git rm ff
touch f
git add f
git commit -qam 'rm /ff, add /f'

rm g/aa
touch g/a
git add g/a
git commit -qam 'rm g/aa, add g/a'
