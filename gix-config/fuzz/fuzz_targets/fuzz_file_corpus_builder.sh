#!/usr/bin/env bash

set -eox pipefail

CWD=$(pwd)

ROOT=$1
OUTPUT_CORPUS=$2
FIXTURES_DIR=$(readlink -f $ROOT/gix-config/tests/fixtures)

echo $ROOT
echo $FIXTURES_DIR
find $FIXTURES_DIR -name "*.config" -exec zip -j $OUTPUT_CORPUS {} \;

# Generate configs.
REPO=$(mktemp -d)
cd $REPO
bash $FIXTURES_DIR/make_config_repo.sh
find . -name ".*" -exec zip $OUTPUT_CORPUS {} \;
cd $CWD
rm -r $REPO

