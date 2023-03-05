#!/bin/bash
set -eu -o pipefail

remote="${1:?First argument is the complex repo to clone from}"

git clone --depth 3        file://"$remote" shallow
git clone --depth 3 --bare file://"$remote" shallow.git
