#!/usr/bin/bash

set -eox pipefail

ROOT=$1
OUTPUT_CORPUS=$2
FIXTURES_DIR=$(readlink -f $ROOT/gix-object/tests/fixtures/tag)

echo $ROOT
echo $FIXTURES_DIR

zip -j $OUTPUT_CORPUS $FIXTURES_DIR/*
