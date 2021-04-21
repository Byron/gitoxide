#!/bin/bash
set -eu -o pipefail

parent_dir="$1"
mkdir "$parent_dir"

script_dir=$(dirname "$(realpath -e "$0")")

run() {
    local target_dir=$parent_dir/$1
    local script=$script_dir/$1.sh

    local temp_dir=$(mktemp -d create_fixtures-XXXXXXXXXX)
    trap "rm -rf $temp_dir" EXIT
    (cd "$temp_dir" && "$script")
    cp -dR "$temp_dir/.git/objects/" "$target_dir/"
    rm -rf "$temp_dir"
    trap - EXIT
}

#run bloom
#run bloom_too_large
run octopus_merges
run single_commit
run single_parent
run split_chain
run two_parents
