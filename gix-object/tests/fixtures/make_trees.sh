#!/bin/bash
set -eu -o pipefail

function baseline() {
    local revspec=${1:?First argument is the revspec of the object to create a baseline for}
    local basename=${2:?Second argument is the name of the baseline file}
    local baseline_file="$basename.baseline"
    git rev-parse "$revspec" | git cat-file --batch | tail -n +2 > "$baseline_file"
    truncate -s "$(($(stat -c '%s' "$baseline_file")-1))" "$baseline_file"
}

git init -q
mkdir file
touch bin bin.d file.to file.toml file.toml.bin file0 file/a

git add .
git commit -m "c1"

baseline @^{tree} tree
