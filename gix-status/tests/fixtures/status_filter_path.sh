#!/usr/bin/env bash
set -eu -o pipefail

# A repository with a single file `a.txt` that has the `filter=path` attribute.
# The file content is its own path relative to the worktree, `a.txt`. A `clean`
# filter that outputs its `%f` argument thus produces the content of the index blob
# if it receives the path relative to the worktree, like Git gives it.
# No `filter.path.*` configuration exists, so Git stores the file unfiltered.

git init -q

echo "a.txt filter=path" >.gitattributes
printf '%s' "a.txt" >a.txt

git add -A
git commit -q -m "Commit"

# Git passes `%f` relative to the worktree: a clean filter printing it reproduces the blob `a.txt`.
# The new `mtime` makes Git clean and hash the file again.
touch -m -t 202001010000 a.txt
git -c filter.path.clean='cat >/dev/null && printf %s %f' -c filter.path.required=true diff --exit-code
