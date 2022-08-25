#!/bin/bash
set -eu -o pipefail

test "$1" = get && \
echo username=user-script && \
echo password=pass-script
