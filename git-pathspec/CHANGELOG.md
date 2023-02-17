# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2023-02-17)

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 116 commits contributed to the release over the course of 280 calendar days.
 - 337 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#415](https://github.com/Byron/gitoxide/issues/415), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#415](https://github.com/Byron/gitoxide/issues/415)**
    - changed quickerror to thiserror ([`49fcab7`](https://github.com/Byron/gitoxide/commit/49fcab749ea3260f3333976567fcc4ab8a072fe3))
    - added alternative parsing module ([`eb2dec0`](https://github.com/Byron/gitoxide/commit/eb2dec0777bfe894f4d02bdc03a3e507302b41b4))
    - attribute parsing functional ([`4b2ed7e`](https://github.com/Byron/gitoxide/commit/4b2ed7e0f033fd01a7364fc41057e113adf2fbdc))
    - updated documentation ([`8b9570f`](https://github.com/Byron/gitoxide/commit/8b9570fdcdb426cbde7edb9fcca4b7340944550e))
    - attribute parsing WIP ([`7c84fb8`](https://github.com/Byron/gitoxide/commit/7c84fb81506cc41fbf68575583129eafbd89139d))
    - added some documentation ([`c04d4be`](https://github.com/Byron/gitoxide/commit/c04d4be29c7a0cf72500d30432215513f6066338))
    - tests now check if pathspec is valid in git ([`334659e`](https://github.com/Byron/gitoxide/commit/334659e34b1f99f3bc662fa599dcd8f0d94ad206))
    - remove WhitespaceError ([`4d20cd9`](https://github.com/Byron/gitoxide/commit/4d20cd91be403d48fd5443202048f4a5bb867a62))
    - error handling: parser can return a result now ([`31aba11`](https://github.com/Byron/gitoxide/commit/31aba11c953b5b7dd70f14ee904026d12db69d10))
    - pathspec parser is functional ([`7d95f16`](https://github.com/Byron/gitoxide/commit/7d95f162a3edb7b2714dfadb9b5cf8311f3da061))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - note that crates have been renamed from `git-*` to `gix-*`. ([`e14dc7d`](https://github.com/Byron/gitoxide/commit/e14dc7d475373d2c266e84ff8f1826c68a34ab92))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - refactor ([`b2835cc`](https://github.com/Byron/gitoxide/commit/b2835cc28e10907eb375b2beb400cf408fa5a3e0))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'pathspec' ([`7db59a4`](https://github.com/Byron/gitoxide/commit/7db59a4074111086adfc2f79fd0d26bb30303ca9))
    - avoid another vec allocation by inlining code via closure. ([`d88952a`](https://github.com/Byron/gitoxide/commit/d88952a533cf2fa2ebf0f015b10f5983a1c8f144))
    - avoid temporary vec in favor of a `&'static [u8]`. ([`5a55dbf`](https://github.com/Byron/gitoxide/commit/5a55dbf5fd2ae8c28d95d72d55a30e4e7e2ef9cf))
    - improve docs and use 'new-style' in error messages. ([`e36d83e`](https://github.com/Byron/gitoxide/commit/e36d83e62eb7969726e7c8b3d25dbb743a508f8a))
    - assure all baseline samples are validated ([`4899722`](https://github.com/Byron/gitoxide/commit/489972209ee2ead2871ca2410bd10e51019dc9ad))
    - add documentation; rename `SearchMode` to `MatchMode`; add test ([`4f6fa59`](https://github.com/Byron/gitoxide/commit/4f6fa59d20b4e03663f3d7c3819a7d02b79ab982))
    - add basic docs for how to run the fuzzer ([`0c9bef4`](https://github.com/Byron/gitoxide/commit/0c9bef47a70a0787b63f9bf8e9b52f2ab9f72738))
    - rename `parser` fuzz target to `parse` ([`ef03823`](https://github.com/Byron/gitoxide/commit/ef03823b407bcdbac16c42e941809dfe7cde850b))
    - refactor ([`1cbc142`](https://github.com/Byron/gitoxide/commit/1cbc142d37599f4d7bfaf9cb07de41ee4b3f4c24))
    - update crate status and READMe ([`07352ce`](https://github.com/Byron/gitoxide/commit/07352ceb942dac5041fbf8584128404880166593))
    - refactor ([`63baa75`](https://github.com/Byron/gitoxide/commit/63baa752901388a46a4211c70f3b3a64aa36d4ec))
    - add readme file ([`913d94c`](https://github.com/Byron/gitoxide/commit/913d94c3c226aac321de12eb7ae8bfa3ee3458e9))
    - remove prefix stuff ([`b7baaa5`](https://github.com/Byron/gitoxide/commit/b7baaa5353aa52ca97488993b50cef72248387de))
    - refactor of `Name` and `Assignment` ([`6449e77`](https://github.com/Byron/gitoxide/commit/6449e77e11ef0d25c2990f1c29e9fbea3c97fb0a))
    - refactor ([`bffbcee`](https://github.com/Byron/gitoxide/commit/bffbcee68affce2a829c67bfe71e7df861c15ee5))
    - fix unescaping logic - thanks fuzzer ([`9c6281f`](https://github.com/Byron/gitoxide/commit/9c6281f903451011407a352e1fca877d79c10466))
    - add fuzzer ([`2a775c6`](https://github.com/Byron/gitoxide/commit/2a775c602da3edf20d84599839ca166bff2457a5))
    - check attr values regarless of it being escaped ([`6e93144`](https://github.com/Byron/gitoxide/commit/6e93144545277cb5efe1dd3fba2ecaf90ea9c726))
    - use "to_owned" instead of "into" ([`35c6d38`](https://github.com/Byron/gitoxide/commit/35c6d38088e09d88b30e12538e175a4a286980cd))
    - help type inference ([`4d6befd`](https://github.com/Byron/gitoxide/commit/4d6befdbad72d1d143edd344cc7e9e0cba1d1e8a))
    - fix build ([`1838f3d`](https://github.com/Byron/gitoxide/commit/1838f3db13eae6d278264dcdbc48d202de992349))
    - refactor ([`1bdf2e1`](https://github.com/Byron/gitoxide/commit/1bdf2e1d1b8f58ed2d6fe21c62edb93fc5473c14))
    - refactor ([`850bcc3`](https://github.com/Byron/gitoxide/commit/850bcc35f0e99ef1d65c6bd888638b9a67cab25b))
    - refactor ([`65c8349`](https://github.com/Byron/gitoxide/commit/65c83491281184161ad6d9831adabb8475722e42))
    - refactor attribute value unnescaping ([`24592f7`](https://github.com/Byron/gitoxide/commit/24592f7a4f6188d3bb0042f9e91dffc5fc01e382))
    - implement name::error for git-attributes ([`0849ebf`](https://github.com/Byron/gitoxide/commit/0849ebf4bc2052d7886f9425800a547bf530e967))
    - improved attribute value unescaping ([`1f89646`](https://github.com/Byron/gitoxide/commit/1f89646fc359f94ee001f5ed01623e5af7934a93))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - refactor ([`9945ceb`](https://github.com/Byron/gitoxide/commit/9945ceb0a99c1343cb6e652e44900b36d3786e22))
    - refactor ([`3b2bab8`](https://github.com/Byron/gitoxide/commit/3b2bab89172b86068bda9704bc9d69690bcfb2ba))
    - refactor ([`852bcc3`](https://github.com/Byron/gitoxide/commit/852bcc316382fce5f5749942ecb43a73738ffe8f))
    - protected attribute name via "AttributeName" type ([`7bb408e`](https://github.com/Byron/gitoxide/commit/7bb408e631138854a6dff85ce356da96f61367de))
    - refactor ([`1ad98e8`](https://github.com/Byron/gitoxide/commit/1ad98e82f66c5f5eacb15f8ae38d8ccb1bc94e9e))
    - escape attribute values in pathspec crate... ([`c22e57f`](https://github.com/Byron/gitoxide/commit/c22e57f8e4131655595da2b81662e23258bb85c8))
    - refactor ([`7f00b50`](https://github.com/Byron/gitoxide/commit/7f00b50070c7b976a030ec836d2660ff5b7b5f72))
    - refactor ([`2523f96`](https://github.com/Byron/gitoxide/commit/2523f9606f0adedb20ac93cf4853298bcd996118))
    - refactor ([`699de03`](https://github.com/Byron/gitoxide/commit/699de03f0c981a9f8b5239c66dc425e504de1ec2))
    - refactor ([`7f93231`](https://github.com/Byron/gitoxide/commit/7f93231b4983f9ce596cea84ad4525feb3778dd6))
    - refactor ([`02fba2c`](https://github.com/Byron/gitoxide/commit/02fba2c124f3665112102469d41d476b6cf48dcd))
    - improved testing against the baseline ([`44991d3`](https://github.com/Byron/gitoxide/commit/44991d373bd2e2f71ccf27eabe9f074cb5fe7c18))
    - refactor ([`020bc24`](https://github.com/Byron/gitoxide/commit/020bc24973233edc261e05fd9935c5e598bf2922))
    - refactor ([`b490b4a`](https://github.com/Byron/gitoxide/commit/b490b4a7be579941c2664fbefb25dd341ed7d1e7))
    - more testcases related to escape chars - still todo ([`f606515`](https://github.com/Byron/gitoxide/commit/f606515dfd89e28c78eaead3cf5023d5064618f5))
    - no splitting on escaped commas in attribute values ([`c0196fa`](https://github.com/Byron/gitoxide/commit/c0196fa363088426de031e55b982f70573ab738d))
    - thanks clippy ([`f80eb85`](https://github.com/Byron/gitoxide/commit/f80eb851ab56b4580eb28935978487b9f37fe819))
    - remove attr from signature bitflag ([`998415d`](https://github.com/Byron/gitoxide/commit/998415d9a3a234787c017bd448410a24ad4965f0))
    - refactor ([`149d1b3`](https://github.com/Byron/gitoxide/commit/149d1b36f93d98002175cda362d50bac584c691a))
    - whitespace test added ([`eecd388`](https://github.com/Byron/gitoxide/commit/eecd388708017414fee9077066f24124d83b70ba))
    - refactor ([`476fa56`](https://github.com/Byron/gitoxide/commit/476fa56993391410fc0bafeccfcb8d4da8168bfc))
    - add more test cases ([`9ceea27`](https://github.com/Byron/gitoxide/commit/9ceea2718a63bdd55ea8b99c2f1656cc09850145))
    - thanks clippy ([`f7a3b69`](https://github.com/Byron/gitoxide/commit/f7a3b69e43e82471047091008355d180e646773d))
    - added more tests ([`476f31c`](https://github.com/Byron/gitoxide/commit/476f31c0f0fc7b29d02110e3a9b9a542defce63e))
    - refactor ([`d3ec61a`](https://github.com/Byron/gitoxide/commit/d3ec61a2fcc6d8269cb952def86a198a7ac9492e))
    - refactor ([`5a3c0fe`](https://github.com/Byron/gitoxide/commit/5a3c0fe8d56e9bc28eda77d3d64ef5338365622c))
    - refactor ([`162f9a0`](https://github.com/Byron/gitoxide/commit/162f9a06860fac69e6db6d76dc5051ec2e6ed2db))
    - pattern now has searchmode... ([`0bed938`](https://github.com/Byron/gitoxide/commit/0bed9382930486af144876d97b97479e03e0f1c1))
    - test refactor and bug fixes ([`57d8d90`](https://github.com/Byron/gitoxide/commit/57d8d90d246226fc9119612d10d31808f8fa3053))
    - Pattern uses MagigSignature without Option ([`f1f4ab3`](https://github.com/Byron/gitoxide/commit/f1f4ab3e3f50d00db4756ea724e5fd1b8ee75a04))
    - error tests now use matches! ([`6a569a7`](https://github.com/Byron/gitoxide/commit/6a569a70d15c416f91ab20083747e25f867f7446))
    - refactor ([`13b7db5`](https://github.com/Byron/gitoxide/commit/13b7db526c0a56360998a731ba10a7b4990d9529))
    - refactor ([`2690b8a`](https://github.com/Byron/gitoxide/commit/2690b8a73c39175ccfddd098c1f72f2cdee048cf))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - hint for how to make a functional version bearable… ([`e8da186`](https://github.com/Byron/gitoxide/commit/e8da18663d5110c87200287a1bc0d1b6f86cf0f9))
    - refactor ([`a0477e9`](https://github.com/Byron/gitoxide/commit/a0477e9b1fdf6ef289208a77f0539ea090c84e79))
    - refactor ([`d109cfe`](https://github.com/Byron/gitoxide/commit/d109cfead637b0b2c2866fb411eeccbf6a5bff2c))
    - refactor ([`fbed980`](https://github.com/Byron/gitoxide/commit/fbed980797057efb22140e8ff371989d49cc2a73))
    - initial setup of pathspec module ([`fece972`](https://github.com/Byron/gitoxide/commit/fece9725d60201b16b67073c185195b88fa1ad20))
</details>

## 0.0.0 (2022-03-17)

An empty crate without any content to reserve the name for the gitoxide project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - add future crate for good measure ([`625eb1d`](https://github.com/Byron/gitoxide/commit/625eb1d7d266036c1f30caab7bd3897af9fdbef7))
 * **Uncategorized**
    - Release git-pathspec v0.0.0 ([`d6bee3f`](https://github.com/Byron/gitoxide/commit/d6bee3f931741906126a800aec9d43bc6bf8690f))
</details>

