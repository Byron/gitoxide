#!/usr/bin/env bash
set -eu -o pipefail

git init -q

# Shouldn't be necessary, as a repo starts with some config vars, but this removes any doubt.
git config --local foo.bar baz
