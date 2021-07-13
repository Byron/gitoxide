#!/bin/bash
root="$(cd "${0%/*}" && pwd)"
source "$root/make_ref_repository.sh"

git pack-refs --all --prune
