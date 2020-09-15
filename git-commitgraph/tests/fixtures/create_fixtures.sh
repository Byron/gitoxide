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
    "$script" "$temp_dir"
    cp -dR "$temp_dir/.git/objects/" "$target_dir/"
    rm -rf "$temp_dir"
    trap - EXIT
}

run bloom
run bloom_too_large
run octopus_merges
run single_commit
run single_parent
run split_chain
run two_parents

#"$script_dir"/bloom.sh "$parent_dir/bloom"
#"$script_dir"/bloom_too_large.sh "$parent_dir/bloom_to_large"
#"$script_dir"/octopus_merges.sh "$parent_dir/octopus_merges"
#"$script_dir"/single_commit.sh "$parent_dir/single_commit"
#"$script_dir"/single_parent.sh "$parent_dir/single_parent"
#"$script_dir"/two_parents.sh "$parent_dir/two_parents"
