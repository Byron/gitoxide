#!/usr/bin/env bash
set -eu

test "$1" = get && \
echo username=user-script && \
echo password=pass-script
