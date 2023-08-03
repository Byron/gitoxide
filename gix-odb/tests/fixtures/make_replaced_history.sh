#!/bin/bash
set -eu -o pipefail

git init -q

echo "#include <stdio.h>" > file.c && git add file.c
git commit -m "Initial commit"
echo "// 2nd line" >> file.c && git commit -am "2nd commit"
echo "// 3rd line" >> file.c && git commit -am "3rd commit"
echo "// 4th line" >> file.c && git commit -am "4th commit"
git branch long_history HEAD^ # Create branch off of 3rd commit
new_base=$(echo 'Short history stops here' | git commit-tree 'HEAD~2^{tree}')
git rebase --onto $new_base HEAD~2
git replace HEAD~1 long_history # Here's the git-replace
