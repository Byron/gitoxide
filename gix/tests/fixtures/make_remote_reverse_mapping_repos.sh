#!/usr/bin/env bash
set -eu -o pipefail

# Reverse mapping works offline from fetch refspecs alone, without commits or
# tracking refs. Each copy fixes one source or destination spelling in config.
git init -q base
git -C base remote add origin .

# Qualified and partially qualified sources must be preserved, not expanded.
cp -R base full-source
git -C full-source config remote.origin.fetch refs/heads/main:refs/remotes/origin/main
cp -R base partial-source
git -C partial-source config remote.origin.fetch heads/main:refs/remotes/origin/main
cp -R base head-source
git -C head-source config remote.origin.fetch HEAD:refs/remotes/origin/main

# Short names and object IDs cannot become full reference names offline.
cp -R base short-source
git -C short-source config remote.origin.fetch main:refs/remotes/origin/main
cp -R base object-id-source
git -C object-id-source config remote.origin.fetch abcdef0123456789abcdef0123456789abcdef01:refs/remotes/origin/main

# A destination abbreviation must not match the full tracking-reference name.
cp -R base abbreviated-destination
git -C abbreviated-destination config remote.origin.fetch refs/heads/main:remotes/origin/main
