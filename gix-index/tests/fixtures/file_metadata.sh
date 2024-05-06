#!/usr/bin/env bash
set -eu -o pipefail

# The largest-possible date for Ext4, nanos are special there, but ont usually on other filesystems
touch -d "2446-05-10 22:38:55.111111111" future
# The smallest-possible date for Ext4, nanos are special there, but ont usually on other filesystems
touch -d "1901-12-13 20:45:52.222222222" past
