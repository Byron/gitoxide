#!/usr/bin/env bash
set -eu -o pipefail

# Two commits let revision walks verify that falling back from a corrupt graph
# retains both the current tip and its parent. Rust post-processing in
# gix/tests/gix/repository/revision.rs corrupts only the tip's first-parent
# position before this fixture is cached, leaving the graph readable.
git init -q
git checkout -q -b main
git commit -q --allow-empty -m parent
git commit -q --allow-empty -m child
git commit-graph write --no-progress --reachable
