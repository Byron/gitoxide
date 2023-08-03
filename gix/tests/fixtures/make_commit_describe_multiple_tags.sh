#!/bin/bash
set -eu -o pipefail

git init -q
git commit --allow-empty -q -m c1
git commit --allow-empty -q -m c2
git commit --allow-empty -q -m c3

# Tag the first commit (with lightweight tags only)
git tag l0 ":/c1"
GIT_COMMITTER_DATE="2022-01-02 00:00:00 +0000" git tag l1 "HEAD~2"

# Tag the second commit (for tests involving tag priority)
# The date is not checked for lightweight tags, so date the annotated tag to 0
GIT_COMMITTER_DATE="1970-01-01 00:00:00 +0000" git tag v1 -m "tag object 0" :/c2
git tag v0 ":/c2"

# Tag the third (HEAD) commit (testing the combination of priority, date and lexicographical order)
git tag v2 -m "tag object 1"
git tag v2.5
GIT_COMMITTER_DATE="2022-01-02 00:00:00 +0000" git tag v4 -m "tag object 4"
GIT_COMMITTER_DATE="2022-01-02 00:00:00 +0000" git tag v5 -m "tag object 5"
GIT_COMMITTER_DATE="2022-01-03 00:00:00 +0000" git tag v3
