#!/bin/bash

set -eu
echo -n "$@"
read password
echo "$password"
