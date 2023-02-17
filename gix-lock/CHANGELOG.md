# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.0.3 (2023-02-17)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

## 3.0.2 (2023-02-17)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Bug Fixes

 - <csr-id-73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0/> Interpret `PermissionDenied` as reason to retry on lock acquisition failure.
   Evidence from CI suggests that on windows 'AlreadyExists' isn't the
   common error code. Instead, maybe due to racyness, it can also emit
   PermissionDenied errors which we now handle specifically.

### New Features

 - <csr-id-f9bb71b984045a069f7caf0c650ef24bd3cc0be1/> add `backoff` for an exponential backoff iterator.
   It's used for computing the backoff on the lock itself, but is certainly
   useful in other occasions as well.
 - <csr-id-a789490fbbba1f44d79d57c8ba74bf40c600faf2/> export git-tempfile in the root
   This allows access to signal handler setup for example, which can be
   useful in some situations.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 148 commits contributed to the release over the course of 610 calendar days.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#384](https://github.com/Byron/gitoxide/issues/384), [#386](https://github.com/Byron/gitoxide/issues/386), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - refactor ([`287f47c`](https://github.com/Byron/gitoxide/commit/287f47c17a52be382f0866abaa0db6b33a09f560))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - export git-tempfile in the root ([`a789490`](https://github.com/Byron/gitoxide/commit/a789490fbbba1f44d79d57c8ba74bf40c600faf2))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - Interpret `PermissionDenied` as reason to retry on lock acquisition failure. ([`73ecb40`](https://github.com/Byron/gitoxide/commit/73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - rename `git-lock` to `gix-lock` ([`0422433`](https://github.com/Byron/gitoxide/commit/04224336b910fbcb8c32ef61edb345c53e945425))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'main' into http-config ([`f4ff821`](https://github.com/Byron/gitoxide/commit/f4ff821fd4233dd1dc1a449af4d4600becf3b4ac))
    - Merge branch 'async-fetch' ([`0c9c48b`](https://github.com/Byron/gitoxide/commit/0c9c48b3b91a1396eb1796f288a2cb10380d1f14))
    - add `backoff` for an exponential backoff iterator. ([`f9bb71b`](https://github.com/Byron/gitoxide/commit/f9bb71b984045a069f7caf0c650ef24bd3cc0be1))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'main' into git-sec ([`2fe70f9`](https://github.com/Byron/gitoxide/commit/2fe70f96cfb68e108637ce78f8edda2eb4e2e61a))
    - Merge branch 'main' into repo-status ([`8122c5c`](https://github.com/Byron/gitoxide/commit/8122c5c8a122e5297832f96c07a6fb1a5df02620))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [stability #171] prepare git-lock and git-tempfile release ([`3a1cf4d`](https://github.com/Byron/gitoxide/commit/3a1cf4d441b53c880b5c887916302a493ad28b41))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - Release git-lock v0.3.2 ([`a5ea2e7`](https://github.com/Byron/gitoxide/commit/a5ea2e7ba4bac484fad0672811ecd303214d8425))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.3.1 ([`168f5a0`](https://github.com/Byron/gitoxide/commit/168f5a0c5e9fdaf56789c884853569c5b850d7fa))
    - [lock #154] add io impls for `File` ([`be62a8b`](https://github.com/Byron/gitoxide/commit/be62a8bbe6c643017816ea23f15f04d38eb98b46))
    - (cargo-release) version 0.3.0 ([`263088b`](https://github.com/Byron/gitoxide/commit/263088b3faaccd9edae8c21dfc7d39b191d76207))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.2.0 ([`20d8e27`](https://github.com/Byron/gitoxide/commit/20d8e27dd4e93ae2234a3fe19b5f1511365eee2e))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - Bump fastrand from 1.4.1 to 1.5.0 ([`b138b43`](https://github.com/Byron/gitoxide/commit/b138b437b9fb57f5df8bd582ac288ffac00df8cd))
    - [ref] fix docs ([`536555d`](https://github.com/Byron/gitoxide/commit/536555d66705175d83ba411392b049c8ed736ee9))
    - [ref] fix build ([`b4dcdfc`](https://github.com/Byron/gitoxide/commit/b4dcdfc9b2f2edcbcf9fb144d1f97e9a841463ad))
    - [lock] support recoverable commits ([`b2217e7`](https://github.com/Byron/gitoxide/commit/b2217e7d25df9801354f702b0625d3168f8d3271))
    - [lock] refactor ([`48861b2`](https://github.com/Byron/gitoxide/commit/48861b23406bbeee33081b32b2a528533997910d))
    - [lock] FAIL: trying to make peristence recoverable… ([`1fcdd1e`](https://github.com/Byron/gitoxide/commit/1fcdd1e5da51fe8f15e9bfab86b22a82ef936bcc))
    - [ref] try fix windows, once again ([`95e74dd`](https://github.com/Byron/gitoxide/commit/95e74dd9f1510fd288f281beea3f560319ad323d))
    - [lock] access to the locked resource path ([`797bafa`](https://github.com/Byron/gitoxide/commit/797bafaa0a83449be2acb647f875a67476548afe))
    - [lock] allow accessing the lock file path more easily ([`b808b00`](https://github.com/Byron/gitoxide/commit/b808b00f62f587679ffbfe9e465b33419cdd5879))
    - [lock] Fix handling of .lock extension on files without extension ([`64ac60d`](https://github.com/Byron/gitoxide/commit/64ac60d82db9bd81ceed2f0f1167c196de340ffe))
    - [lock] close file lock and commit markers ([`f700821`](https://github.com/Byron/gitoxide/commit/f7008214497d3a80ffcd62840854f3db65c563a9))
    - [lock] Marker commit with runtime check for protection ([`b747814`](https://github.com/Byron/gitoxide/commit/b74781403716e5156dad06ecf5cf83ff72262161))
    - (cargo-release) version 0.1.0 ([`60d48b0`](https://github.com/Byron/gitoxide/commit/60d48b0d008ca3b8612fd7b7e70b89b4bdb52b2c))
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com/Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] capture amount of attempts taken when obtaining a lock ([`7fafa3e`](https://github.com/Byron/gitoxide/commit/7fafa3e70a26c040e2f50c477df314eccf90a42f))
    - [lock] validate error message when waiting for some tim ([`34d3c5a`](https://github.com/Byron/gitoxide/commit/34d3c5a357c33e6734714cbbe8f6065b59f1215e))
    - [lock] the first test for lock failure (immediate mode) ([`2d67a0e`](https://github.com/Byron/gitoxide/commit/2d67a0e3948e717df0cc4de58686f5efa3645be9))
    - [lock] add [must_use = "reason"] attribute where it matters ([`813c46b`](https://github.com/Byron/gitoxide/commit/813c46b1ac9ed5454c7832a6bad5a112f145b565))
    - thanks clippy ([`29782e8`](https://github.com/Byron/gitoxide/commit/29782e8c32d0b062e33a77bd4d639438e88b91b4))
    - [lock] lock acquire with backoff, but without test for now ([`bb2ba81`](https://github.com/Byron/gitoxide/commit/bb2ba81b2723cca11b4d98efa176ce5d184ab594))
    - [lock] prevent flakyness due to rounding or something ([`6f8fbcc`](https://github.com/Byron/gitoxide/commit/6f8fbcc197446068b680ea4c41e79bfc6356fcff))
    - [lock] refactor ([`ddc2170`](https://github.com/Byron/gitoxide/commit/ddc2170498d3822ebf6fc35e2e4b336f7af6d9f3))
    - [lock] remaining test for everything proper exponential backoff needs ([`368d994`](https://github.com/Byron/gitoxide/commit/368d9944b2b3379b0d01be93aea02cae25d1c867))
    - [lock] support for randomization ([`220eb99`](https://github.com/Byron/gitoxide/commit/220eb9945b2006f79b82d80eaabd09b5e291953b))
    - [lock] better overshoot test for exponential backoff ([`62c17d8`](https://github.com/Byron/gitoxide/commit/62c17d8721a8d797fe7475ea36a90931e835ebc4))
    - [lock] a sketch of exponential backoff, without rnadomization ([`55670b4`](https://github.com/Byron/gitoxide/commit/55670b458d248f6249f0031ec73c5eb4f72e36f7))
    - [lock] refactor, remaining docs ([`956e69f`](https://github.com/Byron/gitoxide/commit/956e69fcb96085d96124b6c56d829607b36adf9f))
    - [lock] tests green ([`3706b26`](https://github.com/Byron/gitoxide/commit/3706b2669ebee5cd25a75a42d9b0a4a380707ee1))
    - [lock] creation of lockfiles, with immediate failure mode ([`fda7da8`](https://github.com/Byron/gitoxide/commit/fda7da8ec43f7999effdfd76791a6541f8c52338))
    - [lock] first tests and a lot of refactoring ([`3c34194`](https://github.com/Byron/gitoxide/commit/3c34194b6c0fd5ab22eb91081a563ba3bfa19110))
    - [lock] even more sketched out API ([`0dc88c9`](https://github.com/Byron/gitoxide/commit/0dc88c9c42690a3e8320fe5b701c51eed8747118))
    - [lock] refactor; Marker is definitely not necessary… ([`6af84c9`](https://github.com/Byron/gitoxide/commit/6af84c92dbe049068be795ef4870fd830baf5384))
    - [lock] test what happens if multiple tempfiles are created ([`17942c7`](https://github.com/Byron/gitoxide/commit/17942c7960f25ad1f8f7fb2c94f251d39bb03c6e))
    - [lock] sketch API ([`f0e1427`](https://github.com/Byron/gitoxide/commit/f0e142734c1b09e6c4175b3c1b232d886449e280))
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com/Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com/Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - [lock] frame for git-lock crate ([`e6bc87d`](https://github.com/Byron/gitoxide/commit/e6bc87d77f9b397b25694f58d347de2fc38bf71d))
</details>

## 3.0.1 (2022-12-19)

A maintenance release without user-facing changes.

## 3.0.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 2.2.0 (2022-11-17)

### New Features

 - <csr-id-f9bb71b984045a069f7caf0c650ef24bd3cc0be1/> add `backoff` for an exponential backoff iterator.
   It's used for computing the backoff on the lock itself, but is certainly
   useful in other occasions as well.

## 2.1.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 2.1.0 (2022-05-18)

### New Features

 - <csr-id-a789490fbbba1f44d79d57c8ba74bf40c600faf2/> export gix-tempfile in the root
   This allows access to signal handler setup for example, which can be
   useful in some situations.

### Bug Fixes

 - <csr-id-73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0/> Interpret `PermissionDenied` as reason to retry on lock acquisition failure.
   Evidence from CI suggests that on windows 'AlreadyExists' isn't the
   common error code. Instead, maybe due to racyness, it can also emit
   PermissionDenied errors which we now handle specifically.

## 2.0.0 (2022-04-03)

A maintenance release due to mildly breaking changes in `gix-tempfile`.

## v1.0.1 (2021-10-15)

This release contains no functional changes, but a more useful changelog.

## v1.0.0 (2021-08-25)

- initial release

## v0.3.2 (2021-08-17)

## v0.3.1 (2021-08-11)

## v0.3.0 (2021-08-11)

## v0.2.0 (2021-08-10)

## v0.1.0 (2021-06-23)

## v0.0.0 (2021-06-17)

