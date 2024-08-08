#!/usr/bin/env bash
set -eu -o pipefail

# Attempt to create files with the latest and earliest possible dates for ext4. Nanoseconds are
# special there, but not usually on other filesystems. In some touch implementations, the format
# may be rejected. So if a command fails, we try again with a more extreme date that is out of
# range, because some implementations will clip it to the edge of the range (but they may fail).
touch -d '2446-05-10 22:38:55.111111111' future || touch -d '2446-05-11 22:38:56' future
touch -d '1901-12-13 20:45:52.222222222' past || touch -d '1901-12-13 20:45:52' past
