#!/bin/bash
set -eu -o pipefail

# The contents and structure of this loop are a direct copy
# from git's own test suite (t/t5500-fetch-pack.sh).
# Please do not change this loop and instead add additional
# test cases at the bottom of this file.
for r in repo re:po re/po
do
  for p in "ssh+git" "git+ssh" git ssh
  do
    for h in host user@host user@[::1] user@::1
    do
      for c in "" :
      do
        test_expect_success "fetch-pack --diag-url $p://$h$c/$r" '
          check_prot_host_port_path $p://$h/$r $p "$h" NONE "/$r"
        '
        test_expect_success "fetch-pack --diag-url $p://$h$c/~$r" '
          check_prot_host_port_path $p://$h/~$r $p "$h" NONE "~$r"
        '
      done
    done
    for h in host User@host User@[::1]
    do
      test_expect_success "fetch-pack --diag-url $p://$h:22/$r" '
        check_prot_host_port_path $p://$h:22/$r $p "$h" 22 "/$r"
      '
    done
  done
  for p in file
  do
    test_expect_success !MINGW "fetch-pack --diag-url $p://$h/$r" '
      check_prot_path $p://$h/$r $p "/$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $p://$h/$r" '
      check_prot_path $p://$h/$r $p "//$h/$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $p:///$r" '
      check_prot_path $p:///$r $p "/$r"
    '
    test_expect_success !MINGW "fetch-pack --diag-url $p://$h/~$r" '
      check_prot_path $p://$h/~$r $p "/~$r"
    '
    test_expect_success MINGW "fetch-pack --diag-url $p://$h/~$r" '
      check_prot_path $p://$h/~$r $p "//$h/~$r"
    '
  done
  for h in nohost nohost:12 [::1] [::1]:23 [ [:aa
  do
    test_expect_success "fetch-pack --diag-url ./$h:$r" '
      check_prot_path ./$h:$r $p "./$h:$r"
    '
    test_expect_success "fetch-pack --diag-url ./$p:$h/~$r" '
    check_prot_path ./$p:$h/~$r $p "./$p:$h/~$r"
    '
  done
  p=ssh
  for h in host [::1]
  do
    test_expect_success "fetch-pack --diag-url $h:$r" '
      check_prot_host_port_path $h:$r $p "$h" NONE "$r"
    '
    test_expect_success "fetch-pack --diag-url $h:/~$r" '
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

