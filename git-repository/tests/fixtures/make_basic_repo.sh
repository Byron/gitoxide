#!/bin/bash
set -eu -o pipefail

git init -q
mkdir -p some/very/deeply/nested/subdir

git init --bare bare.git
