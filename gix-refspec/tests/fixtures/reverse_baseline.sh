#!/usr/bin/env bash
set -eu -o pipefail

# Reverse lookup is textual: `git branch --track` records the parsed fetch source
# in branch.topic.merge without fetching it or resolving its namespace/object ID.
# Each case retains its repository, inputs, and Git output so the Rust tests only
# read cached fixture data. An empty baseline.git means Git rejected tracking.
git init base
(cd base && git commit --allow-empty -m initial)
commit_id=$(git -C base rev-parse HEAD)
printf '%s\n' "$commit_id" > commit-id
case_id=0

function baseline() {
  local group=$1 spec=$2 tracking_ref=$3 expected_source=$4
  case_id=$((case_id + 1))
  mkdir -p "$group"
  git clone --no-local --no-checkout base "$group/$case_id"
  (
    cd "$group/$case_id"
    git remote set-url origin .
    git update-ref "$tracking_ref" "$commit_id"
    git config remote.origin.fetch "$spec"
    printf '%s\n' "$spec" > refspec
    printf '%s\n' "$tracking_ref" > tracking-ref
    if git branch --track topic "$tracking_ref" > branch.out 2> branch.err; then
      test "$(git config --get branch.topic.remote)" = origin
      git config --get branch.topic.merge > baseline.git
      test "$(cat baseline.git)" = "$expected_source"
    else
      test "$expected_source" = '-'
      : > baseline.git
    fi
  )
}

# Exact sources aren't expanded; only `@` is normalized to `HEAD` by parsing.
tracking_ref=refs/remotes/origin/main
for source in refs/heads/main main heads/main HEAD; do
  baseline exact-sources "$source:$tracking_ref" "$tracking_ref" "$source"
done
baseline exact-sources "@:$tracking_ref" "$tracking_ref" HEAD

# Both hash formats and hex case must survive unchanged. This deliberately isn't
# the commit the local tracking ref points to, and need not exist at all.
case ${GIX_TEST_FIXTURE_HASH:-sha1} in
  sha1) source=abcdef0123456789abcdef0123456789abcdef01 ;;
  sha256) source=abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789 ;;
  *) exit 1 ;;
esac
test "$source" != "$commit_id"
baseline hexadecimal-sources "$source:$tracking_ref" "$tracking_ref" "$source"
source=$(printf '%s' "$source" | tr 'a-f' 'A-F')
baseline hexadecimal-sources "$source:$tracking_ref" "$tracking_ref" "$source"

# Globs substitute bytes literally, including slashes and empty captures. Prefix
# and suffix must not overlap, even when each individually matches the name.
baseline glob-sources 'refs/heads/*:refs/remotes/origin/*' "$tracking_ref" refs/heads/main
baseline glob-sources 'heads/*:refs/remotes/origin/*' "$tracking_ref" heads/main
baseline glob-sources 'refs/*:refs/remotes/origin/*' refs/remotes/origin/heads/main refs/heads/main
baseline glob-sources 'refs/heads/pre*post:refs/remotes/origin/pre*post' refs/remotes/origin/prea/bpost refs/heads/prea/bpost
baseline glob-sources 'refs/heads/pre*post:refs/remotes/origin/pre*post' refs/remotes/origin/prepost refs/heads/prepost
baseline glob-sources 'refs/heads/*heads/main:refs/remotes/origin/*origin/main' "$tracking_ref" '-'

# Exact destinations are literal names, not abbreviated refs or object IDs.
# Hash-looking destinations equal the local target to catch accidental ID matching.
for destination in main heads/main origin/main remotes/origin/main "$commit_id"; do
  baseline exact-destinations "refs/heads/main:$destination" "$tracking_ref" '-'
done
destination=$(printf '%s' "$commit_id" | tr 'a-f' 'A-F')
baseline exact-destinations "refs/heads/main:$destination" "$tracking_ref" '-'
tracking_ref=refs/remotes/origin/$commit_id
baseline exact-destinations "refs/heads/main:$tracking_ref" "$tracking_ref" refs/heads/main
