#!/bin/bash
set -eu -o pipefail

git init -q

touch a
git add --intent-to-add a
