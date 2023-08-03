#!/bin/bash
set -eu -o pipefail

git init -q

touch 1 2 3
git add 1
git commit -m 1 1
git add 2
git commit -m 2 2
git add 3
git commit -m 3 3

# NOTE: Starting around git 2.35.0 --preserve-merges was renamed to --rebase-merges
# however --preserve-merges first appeared in git 2.18.  That should cover most use cases.
GIT_EDITOR="sed -i.bak 's/pick/edit/g'" git rebase --rebase-merges --interactive HEAD~2
