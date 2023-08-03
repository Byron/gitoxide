#!/bin/bash
set -eu -o pipefail

# The contents and structure of this loop are a direct copy
# from git's own test suite (t/t5500-fetch-pack.sh).
# Please do not change this loop and instead add additional
# test cases at the bottom of this file.
for path in "repo" "re:po" "re/po"
do
  for protocol in "ssh+git" "git+ssh" "git" "ssh"
  do
    for host in "host" "user@host" "user@[::1]" "user@::1"
    do
      for port_separator in "" ":"
      do
        test_expect_success "fetch-pack --diag-url $protocol://$host$port_separator/$path" '
          check_prot_host_port_path $p://$h/$r $p "$h" NONE "/$r"
        '
        test_expect_success "fetch-pack --diag-url $protocol://$host$port_separator/~$path" '
          check_prot_host_port_path $p://$h/~$r $p "$h" NONE "~$r"
        '
      done
    done
    for host in "host" "User@host" "User@[::1]"
    do
      test_expect_success "fetch-pack --diag-url $protocol://$host:22/$path" '
        check_prot_host_port_path $p://$h:22/$r $p "$h" 22 "/$r"
      '
    done
  done
  for protocol in "file"
  do
    test_expect_success !MINGW "fetch-pack --diag-url $protocol://$host/$path" '
      check_prot_path $p://$h/$r $p "/$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $protocol://$host/$path" '
      check_prot_path $p://$h/$r $p "//$h/$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $protocol:///$path" '
      check_prot_path $p:///$r $p "/$r"
    '
    test_expect_success !MINGW "fetch-pack --diag-url $protocol://$host/~$path" '
      check_prot_path $p://$h/~$r $p "/~$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $protocol://$host/~$path" '
      check_prot_path $p://$h/~$r $p "//$h/~$r"
    '
  done
  for host in "nohost" "nohost:12" "[::1]" "[::1]:23" "[" "[:aa"
  do
    test_expect_success "fetch-pack --diag-url ./$host:$path" '
      check_prot_path ./$h:$r $p "./$h:$r"
    '
    test_expect_success "fetch-pack --diag-url ./$protocol:$host/~$path" '
    check_prot_path ./$p:$h/~$r $p "./$p:$h/~$r"
    '
  done
  protocol="ssh"
  for host in "host" "[::1]"
  do
    test_expect_success "fetch-pack --diag-url $host:$path" '
      check_prot_host_port_path $h:$r $p "$h" NONE "$r"
    '
    test_expect_success "fetch-pack --diag-url $host:/~$path" '
      check_prot_host_port_path $h:/~$r $p "$h" NONE "~$r"
    '
  done
done

# These two test cases are from git's test suite as well.
test_expect_success MINGW 'fetch-pack --diag-url file://c:/repo' '
  check_prot_path file://c:/repo file c:/repo
'
test_expect_success MINGW 'fetch-pack --diag-url c:repo' '
  check_prot_path c:repo file c:repo
'

