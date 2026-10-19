#!/usr/bin/env bash
set -eu -o pipefail

# A repository with a single file `a.txt` that has the `filter=path` attribute.
# The file content combines its worktree-relative path with a sibling `.clean-map`.
# The filter must receive a relative `%f` and run from the worktree root to reproduce
# the index blob, even when the caller runs from another directory.
# No `filter.path.*` configuration exists, so Git stores the file unfiltered.

git init -q

echo "a.txt filter=path" >.gitattributes
printf '%s' " mapped" >.clean-map
printf '%s' "a.txt mapped" >a.txt

git add -A
git commit -q -m "Commit"

# The new `mtime` makes Git clean and hash the file again, from a subdirectory.
touch -m -t 202001010000 a.txt
mkdir subdir
git -C subdir -c filter.path.clean='cat >/dev/null && printf %s %f && cat "$(dirname %f)/.clean-map"' \
  -c filter.path.required=true diff --exit-code
