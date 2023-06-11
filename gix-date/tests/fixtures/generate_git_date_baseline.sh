#!/bin/bash
set -eu -o pipefail

git init;

function baseline() {
    local test_date=$1 # first argument is the date to test
    local test_name=$2 # second argument is the format name for re-formatting

    git -c section.key="$test_date" config --type=expiry-date section.key && status=0 || status=$?
    {
        echo "$test_date"
        echo "$test_name"
        echo "$status"
        if [ $status == 0 ]
        then
          git -c section.key="$test_date" config --type=expiry-date section.key
        else
          echo "-1"
        fi
    } >> baseline.git
}

# success

# date formats following to https://git-scm.com/docs/git-log#Documentation/git-log.txt---dateltformatgt

# short
# ODO
#baseline '2022-08-22'
# rfc2822
baseline 'Thu, 18 Aug 2022 12:45:06 +0800' 'RFC2822'
# iso8601
baseline '2022-08-17 22:04:58 +0200' 'ISO8601'
# iso8601_strict
baseline '2022-08-17T21:43:13+08:00' 'ISO8601_STRICT'
# default
baseline 'Thu Sep 04 2022 10:45:06 -0400' '' # cannot round-trip, incorrect day-of-week
baseline 'Sun Sep 04 2022 10:45:06 -0400' 'GITOXIDE'
# unix
baseline '1234567890' 'UNIX'
# raw
baseline '1660874655 +0800' 'RAW'

# Note that we can't necessarily put 64bit dates here yet as `git` on the system might not yet support it.

