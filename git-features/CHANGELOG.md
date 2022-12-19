# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.25.0 (2022-12-19)

### New Features (BREAKING)

 - <csr-id-0f27c67c92fc0bc23a6712b5c4c730ad6a0156bf/> add support for explicit non-parallel iteration.
   That way we can allow the implementation to choose whether they
   need greatest speed at some cost or not.
   
   This also allows us to create a new thread-pool on each iteration
   as those who expect high cost or many files will likely chose to do
   that instead of single-threaded iteration, which nicely contains the
   threads needed and avoids keeping them alive as part of some global pool.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 19 calendar days.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - add support for explicit non-parallel iteration. ([`0f27c67`](https://github.com/Byron/gitoxide/commit/0f27c67c92fc0bc23a6712b5c4c730ad6a0156bf))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - upgrade to prodash v22 for API improvements ([`77ab98d`](https://github.com/Byron/gitoxide/commit/77ab98dd41c3849b674d8b3794ef29219ca1447d))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
</details>

## 0.24.1 (2022-11-27)

### New Features

 - <csr-id-6d530a1dc77f0f4ac00622a2fd47c7bdb731a77a/> name spawned threads
   That way it's a bit more obvious what's happening when the CPU goes
   up in flames.
 - <csr-id-c8835c6edae784c9ffcb69a674c0a6545dbb2af3/> upgrade to `prodash 21.1` and add `Ids` to all progress instances.
   That way callers can identify progress they are interested in, say, for
   selective visualizations.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'named-threads' ([`726dd87`](https://github.com/Byron/gitoxide/commit/726dd87b5db45c333ccad898338a1cacea9e3269))
    - name spawned threads ([`6d530a1`](https://github.com/Byron/gitoxide/commit/6d530a1dc77f0f4ac00622a2fd47c7bdb731a77a))
    - upgrade to `prodash 21.1` and add `Ids` to all progress instances. ([`c8835c6`](https://github.com/Byron/gitoxide/commit/c8835c6edae784c9ffcb69a674c0a6545dbb2af3))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.24.0 (2022-11-21)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 14 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
</details>

## 0.23.1 (2022-11-06)

### New Features

 - <csr-id-9076ce33ec167e425a0163d3e40a81a3fd0db6cd/> `fs::Snapshot` can `Clone` if `T` can `Clone`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 23 calendar days.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade to `prodash` v21 ([`a0655dc`](https://github.com/Byron/gitoxide/commit/a0655dc7bc5dff388bc69a648e7f16b44fd1abd9))
    - `fs::Snapshot` can `Clone` if `T` can `Clone`. ([`9076ce3`](https://github.com/Byron/gitoxide/commit/9076ce33ec167e425a0163d3e40a81a3fd0db6cd))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - thanks clippy ([`ad96233`](https://github.com/Byron/gitoxide/commit/ad96233e1aa77fb7d9185f653f3e9519128cf20f))
</details>

## 0.23.0 (2022-10-10)

### New Features

 - <csr-id-a7c11d2cb5f88a4ff322d9a9848459062790d8b3/> perfect granularity for threads processing with `in_parallel_with_slice()`
 - <csr-id-ff1db66f2dad3afc8bc77610006bca9fea5947d2/> add `progress::Step|StepShared` as types of `prodash`
   This may help to use the `Progress::counter()` method as part of method
   signatures, being an `Option<progress::StepShared>`.

### Changed (BREAKING)

 - <csr-id-38446dc8824afef30ef121598de3451d13b9262c/> remove `fs-jwalk-single-threaded` in favor of `fs-walkdir-parallel`.
   This way, `jwalk` and the dependencies (and troubles) it brings have to
   be opted in, but also allow other users to actually opt out while
   allowing the `parallel` feature to be in effect.
   
   In other words, previously the `parallel` feature conflated `jwalk`
   dependencies into the tree, which isn't the case anymore.

### New Features (BREAKING)

 - <csr-id-3b29fc18672c0176684c797a0f16f85d09369bf8/> make jwalk fully optional

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - remove `fs-jwalk-single-threaded` in favor of `fs-walkdir-parallel`. ([`38446dc`](https://github.com/Byron/gitoxide/commit/38446dc8824afef30ef121598de3451d13b9262c))
    - make jwalk fully optional ([`3b29fc1`](https://github.com/Byron/gitoxide/commit/3b29fc18672c0176684c797a0f16f85d09369bf8))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'main' into clone ([`acb0738`](https://github.com/Byron/gitoxide/commit/acb07382a9306d6962bea60e8977d83d021743f4))
    - Merge branch 'delta-tree-parallelization' ([`cca2ad5`](https://github.com/Byron/gitoxide/commit/cca2ad5ee9483d7da968658e0a4d610dbc4ad4d6))
    - Don't enforce Send bounds in serial version of `in_parallel_with_slice()` ([`dda661e`](https://github.com/Byron/gitoxide/commit/dda661e1b7cc0ace6cd9504233f20980e1e52387))
    - Allow discarding the state which could otherwise be used for aggregation. ([`56792fb`](https://github.com/Byron/gitoxide/commit/56792fb53299d52073555c8646360ec0ae88c86d))
    - allow input for `in_parallel_with_slice` to be mutable. ([`e928bf7`](https://github.com/Byron/gitoxide/commit/e928bf7c699a7e48ad283c2cf7fd7479c37c70fc))
    - perfect granularity for threads processing with `in_parallel_with_slice()` ([`a7c11d2`](https://github.com/Byron/gitoxide/commit/a7c11d2cb5f88a4ff322d9a9848459062790d8b3))
    - add `progress::Step|StepShared` as types of `prodash` ([`ff1db66`](https://github.com/Byron/gitoxide/commit/ff1db66f2dad3afc8bc77610006bca9fea5947d2))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.22.6 (2022-09-16)

Fix docs.rs rendering.

### New Features

 - <csr-id-cfe46b502afc3ecb312849ddbd7748007d432cd1/> add zlib-ng feature to allow linking against system libz-ng
   Allow to use zlib-ng (zlib-ng-sys) with native API (no compat mode)
   that can co-exist with system libz (loaded by e.g. libcurl).
   This is used in gitoxide package on Alpine Linux.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - update changelog prior to release ([`ff80042`](https://github.com/Byron/gitoxide/commit/ff80042c9691e5dba5834c674174fdf6d3bdfe7d))
    - fix git-features docs build ([`e5963fe`](https://github.com/Byron/gitoxide/commit/e5963fea183d81db1fe502121b494146a58bd86e))
    - upgrade all dependencies, except for `windows` ([`2968181`](https://github.com/Byron/gitoxide/commit/29681819ffe53d3926d631dc482f71d6200cb549))
    - Merge branch 'dep-upgrade' ([`59767b1`](https://github.com/Byron/gitoxide/commit/59767b1fc1d07b8a7a9333a719e3716746611bc4))
    - upgrade prodash and crosstermion to latest versions ([`ab7ee5b`](https://github.com/Byron/gitoxide/commit/ab7ee5b5d5c15771f431ada9c3b4f53e4be2afdd))
    - add zlib-ng feature to allow linking against system libz-ng ([`cfe46b5`](https://github.com/Byron/gitoxide/commit/cfe46b502afc3ecb312849ddbd7748007d432cd1))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
</details>

## 0.22.5 (2022-09-20)

### New Features

 - <csr-id-cfe46b502afc3ecb312849ddbd7748007d432cd1/> add zlib-ng feature to allow linking against system libz-ng
   Allow to use zlib-ng (zlib-ng-sys) with native API (no compat mode)
   that can co-exist with system libz (loaded by e.g. libcurl).
   This is used in gitoxide package on Alpine Linux.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
    - working progress printing ([`67ec2c7`](https://github.com/Byron/gitoxide/commit/67ec2c7f9a4a6cefdf7148f5c7e48a79f201c4d2))
    - First attempt to get progress information from stat worker. ([`0947c70`](https://github.com/Byron/gitoxide/commit/0947c703f9cecc31ceba101565e6ecafb00adb08))
    - upgrade to prodash 20.1 for `Progress::counter()` feature ([`0ac4a2c`](https://github.com/Byron/gitoxide/commit/0ac4a2c514aeb94d8e90ce28ae7a0e0350c21ab2))
 * **Uncategorized**
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
</details>

## 0.22.4 (2022-09-04)

A maintenance release without breaking changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 6 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#524](https://github.com/Byron/gitoxide/issues/524)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#524](https://github.com/Byron/gitoxide/issues/524)**
    - prepare changelogs prior to release ([`6446b39`](https://github.com/Byron/gitoxide/commit/6446b395d5926565ef899b0c923f35468ccf1921))
 * **Uncategorized**
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.22.3 (2022-08-27)

### Fix

- restrict `sha1` `asm` to supported archs ([`b383fab`](https://github.com/Byron/gitoxide/commit/b383fabbe10868317be51b99cfdd9b0981816042))

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#503](https://github.com/Byron/gitoxide/issues/503)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#503](https://github.com/Byron/gitoxide/issues/503)**
    - prepare changelog ([`3c99e7f`](https://github.com/Byron/gitoxide/commit/3c99e7f02ada72a171856ffc5b870da83fffc703))
 * **Uncategorized**
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`dbfa328`](https://github.com/Byron/gitoxide/commit/dbfa3282cf876596b250b2040c1ec0b761741796))
    - Merge branch 'zlib-sys' ([`7b48297`](https://github.com/Byron/gitoxide/commit/7b482977a556d28f5d9759e1be33cdf3d85c8665))
    - restrict `sha1` `asm` to supported archs ([`b383fab`](https://github.com/Byron/gitoxide/commit/b383fabbe10868317be51b99cfdd9b0981816042))
    - Add feature to link to traditional zlib for dynamic linking support ([`c954bbf`](https://github.com/Byron/gitoxide/commit/c954bbff60ba70a0f1680e3978dd1f8fc1e3a0e7))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.22.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 5 calendar days.
 - 8 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.22.1 (2022-08-15)

### New Features

 - <csr-id-f498d35baba52e40ecd47381e87c1ce49cf13285/> add `fs-jwalk-single-threaded` feature to specifically decouple `jwalk` from rayon
   It has been an issue in https://github.com/starship/starship/issues/4251
   apparently and rayon interactions can be difficult.
 - <csr-id-7f199f0e5246809efde9880110093fbd11a4f8fe/> `fs::Snapshot` to on-demand reload shared resources.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 23 calendar days.
 - 24 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - optimize some portions of the Snapshot code for speed. ([`711fd5c`](https://github.com/Byron/gitoxide/commit/711fd5c6c221440917fa68248e45d5278c780a9e))
    - More convenient API for fs::Snapshots ([`561d2e7`](https://github.com/Byron/gitoxide/commit/561d2e746b1b82ac20f6f14b9c4e3910240075b4))
    - `fs::Snapshot` to on-demand reload shared resources. ([`7f199f0`](https://github.com/Byron/gitoxide/commit/7f199f0e5246809efde9880110093fbd11a4f8fe))
    - Use generalized reload-on-demand in `git-ref` ([`8d0cce7`](https://github.com/Byron/gitoxide/commit/8d0cce7d1521374d5199552fc69a417a957519bc))
    - Now it's possible to update packed refs using the shared code ([`78222c2`](https://github.com/Byron/gitoxide/commit/78222c2e39aa24c84852e999448c042f2fd37db4))
    - The first step towards using the generalized `ReloadIfChanged` in git-ref ([`e8de0ef`](https://github.com/Byron/gitoxide/commit/e8de0ef38db2f2d83cb277ed101464f23c0e98e4))
    - generalized port of packed-refs update logic for use in index ([`e3aff0c`](https://github.com/Byron/gitoxide/commit/e3aff0c2b83720e5745f3d7a8d0f571421a26d99))
 * **Uncategorized**
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - add `fs-jwalk-single-threaded` feature to specifically decouple `jwalk` from rayon ([`f498d35`](https://github.com/Byron/gitoxide/commit/f498d35baba52e40ecd47381e87c1ce49cf13285))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - thanks clippy! ([`c072dbb`](https://github.com/Byron/gitoxide/commit/c072dbb3e203e4a42843895b7d99404d900fdccd))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.22.0 (2022-07-22)

### New Features

 - <csr-id-c76fde7de278b49ded13b655d5345e4eb8c1b134/> initialize `Time` from `now_utc` and `now_local`
   Localtime support depends on some other factors now, but that
   will only get better over time.
   
   We might have to document `unsound_local_time` at some point.

### Changed (BREAKING)

 - <csr-id-89a41bf2b37db29b9983b4e5492cfd67ed490b23/> remove local-time-support feature toggle.
   We treat local time as default feature without a lot of fuzz, and
   will eventually document that definitive support needs a compile
   time switch in the compiler (`--cfg unsound_local_offset` or something).
   
   One day it will perish. Failure is possible anyway and we will write
   code to deal with it while minimizing the amount of system time
   fetches when asking for the current local time.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 36 calendar days.
 - 39 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - remove local-time-support feature toggle. ([`89a41bf`](https://github.com/Byron/gitoxide/commit/89a41bf2b37db29b9983b4e5492cfd67ed490b23))
    - initialize `Time` from `now_utc` and `now_local` ([`c76fde7`](https://github.com/Byron/gitoxide/commit/c76fde7de278b49ded13b655d5345e4eb8c1b134))
    - a first sketch on how identity management could look like. ([`780f14f`](https://github.com/Byron/gitoxide/commit/780f14f5c270802e51cf039639c2fbdb5ac5a85e))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Turn on performance mode for sha-1 computation ([`44371a1`](https://github.com/Byron/gitoxide/commit/44371a10f464f32db346aa6b8309e983cfa20933))
 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - git-features' walkdir: 2.3.1 -> 2.3.2 ([`41dd754`](https://github.com/Byron/gitoxide/commit/41dd7545234e6d2637d2bca5bb6d4f6d8bfc8f57))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
</details>

## 0.21.1 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Assure we used most recent version of crossbeam-utils ([`033f0d3`](https://github.com/Byron/gitoxide/commit/033f0d3e0015b7eead6408c775d2101eb413ffbf))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.21.0 (2022-05-18)

### Changed (BREAKING)

 - <csr-id-90611ce1527618bcc738440bfc1ccc7a45319974/> remove `path` module in favor of `git-path` crate

### New Features (BREAKING)

 - <csr-id-d078d6ee76a80d1dfaf71608c12d8a402bd670d4/> mild refactor of paths module to waste less on unix
   Previously it might have performed find-and-replace on unix paths even
   though they wouldn't have changed afterwards, yet costing an allocation.
   
   There is also the realization that it should go into its own crate to have
   neater import paths and more convenience.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 42 calendar days.
 - 46 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Enforce path conversion on windows gnu, it doesn't seem to like slashes ([`4d55a8f`](https://github.com/Byron/gitoxide/commit/4d55a8f99f2a0b7c0c4ed70a615b7e58b5bee04b))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - remove `path` module in favor of `git-path` crate ([`90611ce`](https://github.com/Byron/gitoxide/commit/90611ce1527618bcc738440bfc1ccc7a45319974))
    - mild refactor of paths module to waste less on unix ([`d078d6e`](https://github.com/Byron/gitoxide/commit/d078d6ee76a80d1dfaf71608c12d8a402bd670d4))
    - refactor ([`8345b7c`](https://github.com/Byron/gitoxide/commit/8345b7caa0cc1cd8489e41822eea89da4c539e6d))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`380174f`](https://github.com/Byron/gitoxide/commit/380174f0ad9e60ccafcd4cfb24e244f106137964))
</details>

## 0.20.0 (2022-04-02)

### New Features

 - <csr-id-e4d6685064ad2b433f8acd3a74b320bf0169a994/> Add `git_config::values::Path` for a typesafe git path
   Add a `Path` type to the `git_config::values` which
   can be interpolated according to gits own path interpolation
   rules.
 - <csr-id-3c8581fc294c65c9eb42698969fe3263135a864e/> add new 'path' module for all path-related conversions
   It's meant to unify all path and byte related handling to help assuring
   encoding is handled correctly or at least similarly everywhere.
 - <csr-id-15ff212b17087de93f259e366f4e4b821cfbc28e/> in-manifest and in-lib documentation of feature toggles

### Bug Fixes

 - <csr-id-234cd10ca55482ce1a840ce3244308d249895bcc/> Assure std::io::copy() doesn't hang when we cause an interrupt
   The user can ask for interruptions which previously used the
   error kind Interrupted. This however has special meaning and
   usually means to retry.

### New Features (BREAKING)

 - <csr-id-8945d95f7fa88562d37ff67ac6e38bead73dd2df/> `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 37 commits contributed to the release over the course of 54 calendar days.
 - 68 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
    - upgrade parking_lot and cargo_toml ([`f95c1a0`](https://github.com/Byron/gitoxide/commit/f95c1a0d9c19bcc6feb9b8739a09d86f9970a0e0))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - more stable testing of perviously racy test for new parallelization mechanism ([`0b4b90f`](https://github.com/Byron/gitoxide/commit/0b4b90fa498d9e07a55b72af2f799da4cd2da81f))
    - Salvage an alternative parallelization approach which might be good for index-creation ([`7e76796`](https://github.com/Byron/gitoxide/commit/7e76796d5c2956961bd998286bec05fca1ba8fc4))
    - refactor ([`f86eacc`](https://github.com/Byron/gitoxide/commit/f86eacc5cfaf6d88ead4f8dbd65989d32674c213))
    - Use hopefully faster crossbeam channel to receive parallelized results ([`3b324b8`](https://github.com/Byron/gitoxide/commit/3b324b868d9d172038797f911eeebfcba8107865))
    - switch index checkout to chunk-based operation ([`e5f6943`](https://github.com/Byron/gitoxide/commit/e5f69433e4a6cc7866b666e0baccfa32efb92a7f))
    - add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError` ([`8945d95`](https://github.com/Byron/gitoxide/commit/8945d95f7fa88562d37ff67ac6e38bead73dd2df))
    - fix `interrupt::Iter` ([`0f0d390`](https://github.com/Byron/gitoxide/commit/0f0d390c475044a75e5db4dcd831d755e74aa3e9))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-lib documentation of feature toggles ([`15ff212`](https://github.com/Byron/gitoxide/commit/15ff212b17087de93f259e366f4e4b821cfbc28e))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Update changelog prior to release ([`1d07934`](https://github.com/Byron/gitoxide/commit/1d079346e789b0acc9a4bdf7577b21c1c37b6106))
    - Remove Option return values in favor of Result ([`493dbae`](https://github.com/Byron/gitoxide/commit/493dbae434e8e4a939e90d03ec3f500744c0725a))
    - Add `git_config::values::Path` for a typesafe git path ([`e4d6685`](https://github.com/Byron/gitoxide/commit/e4d6685064ad2b433f8acd3a74b320bf0169a994))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
    - Make real clear panics are only possible on windows ([`6b283dc`](https://github.com/Byron/gitoxide/commit/6b283dc7b9339fd65ea35f56eb29f121f571caf7))
    - one usage of os_str_bytes down, along with some custom conversion code ([`1cc95ce`](https://github.com/Byron/gitoxide/commit/1cc95cefbd132a4277ec52c2147f7c81fea92d48))
    - gitoxide-core without os-str-bytes ([`909aa14`](https://github.com/Byron/gitoxide/commit/909aa1402c82c3128052023613a297b213716e3d))
    - Remove os_str_bytes from git-pack ([`86f6e50`](https://github.com/Byron/gitoxide/commit/86f6e5054ea11b7aeb9c85321913de090f71e3a1))
    - Don't use os_str_ext in git-features; adapt git-ref ([`9258b7b`](https://github.com/Byron/gitoxide/commit/9258b7baf0895593c13a152ff9e6f52e036cebe1))
    - add new 'path' module for all path-related conversions ([`3c8581f`](https://github.com/Byron/gitoxide/commit/3c8581fc294c65c9eb42698969fe3263135a864e))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - the first possibly working version of loading a mailmap with multiple sources ([`98d745e`](https://github.com/Byron/gitoxide/commit/98d745e8080975a91cff1ce75e187258c851d3f4))
    - cleanup bstr usage to not accidentally pull in unicode ([`8ff53af`](https://github.com/Byron/gitoxide/commit/8ff53af9876a5e35bcfd076124ad776e1b6ff331))
 * **Uncategorized**
    - Release git-features v0.20.0, git-config v0.2.0 ([`a6460db`](https://github.com/Byron/gitoxide/commit/a6460db80ba3c49ea37c712465c7cbdefa5c32b6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - remove 'unused_mut' warning on windows ([`4733e6c`](https://github.com/Byron/gitoxide/commit/4733e6c6f5ea7d5afa4dd171bbba066cf5120ddc))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Commit to using 'unicode' feature of bstr as git-object wants it too ([`471fa62`](https://github.com/Byron/gitoxide/commit/471fa62b142ba744541d7472464d62826f5c6b93))
    - Assure std::io::copy() doesn't hang when we cause an interrupt ([`234cd10`](https://github.com/Byron/gitoxide/commit/234cd10ca55482ce1a840ce3244308d249895bcc))
    - Upgrade to prodash 19 ([`90c6c5a`](https://github.com/Byron/gitoxide/commit/90c6c5aec4015ff969d6e2514fa4d49873ee80f5))
    - thanks clippy ([`07a4094`](https://github.com/Byron/gitoxide/commit/07a4094965ac1b4eb223da8e5ca5cc4a86c5f596))
    - properly document optional features ([`572e57d`](https://github.com/Byron/gitoxide/commit/572e57d5796692764c5c9633969aad25a6f9221a))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Small refactoring and documentation. ([`fefb01b`](https://github.com/Byron/gitoxide/commit/fefb01b84f954700b19e010282c4b413de8e03d2))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - thanks clippy ([`a8e9497`](https://github.com/Byron/gitoxide/commit/a8e9497caebf1c0e9faac537717cd86378f1acf6))
</details>

## 0.19.1 (2022-01-23)

<csr-id-361892ca15aa648802f6701ab6a5a30aedde3449/>

A maintenance release thanks to upgrade to `prodash` 18.

### Changed (BREAKING)

 - <csr-id-61e5cfece4d8f405e35fc1957b00ce1da7526c26/> renamed `progress::Read::reader` -> `progress::Read::inner`

### New Features

 - <csr-id-cb7e4e784d615f9fa3d6fb9c36240f0592403358/> Add InOrderIter to 'parallel' module
   This iterator makes possible identifies results using a sequence id
   and returns only consecutive items.
   
   Use it to collect unordered results produced by threads.
   It's advantage to collecting yourself and sorting is the potential
   for a smaller memory footprint of in-flight results, one doesn't
   have to collect them all for ordering, necessarily.
 - <csr-id-ca095ed881db2a8f06a6b067dbaac17e923b0945/> Make a scope-like abstraction available
   This allows more delicate threading control like is required for the
   index.
 - <csr-id-b8400ed80543d67a5895c975ba9b1fc28427411c/> decoding of variable int numbers.
   It's here only so that we can share the code across crates, for now
   without any feature toggles.
 - <csr-id-0a749a22057b5513a8cefa0e26b0a9a268c769d3/> Add `progress::Write` to automatically pass bytes written to a progress instance

### Chore

 - <csr-id-361892ca15aa648802f6701ab6a5a30aedde3449/> update sha-1 dependency to 0.10

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - upgrade to tui 0.17 and prodash 18 ([`eba101a`](https://github.com/Byron/gitoxide/commit/eba101a576ecb7bc0f63357d0dd81eb817b94be4))
</details>

## 0.19.0 (2022-01-19)

<csr-id-361892ca15aa648802f6701ab6a5a30aedde3449/>

### Chore

 - <csr-id-361892ca15aa648802f6701ab6a5a30aedde3449/> update sha-1 dependency to 0.10

### New Features

 - <csr-id-cb7e4e784d615f9fa3d6fb9c36240f0592403358/> Add InOrderIter to 'parallel' module
   This iterator makes possible identifies results using a sequence id
   and returns only consecutive items.
   
   Use it to collect unordered results produced by threads.
   It's advantage to collecting yourself and sorting is the potential
   for a smaller memory footprint of in-flight results, one doesn't
   have to collect them all for ordering, necessarily.
 - <csr-id-ca095ed881db2a8f06a6b067dbaac17e923b0945/> Make a scope-like abstraction available
   This allows more delicate threading control like is required for the
   index.
 - <csr-id-b8400ed80543d67a5895c975ba9b1fc28427411c/> decoding of variable int numbers.
   It's here only so that we can share the code across crates, for now
   without any feature toggles.
 - <csr-id-0a749a22057b5513a8cefa0e26b0a9a268c769d3/> Add `progress::Write` to automatically pass bytes written to a progress instance

### Changed (BREAKING)

 - <csr-id-61e5cfece4d8f405e35fc1957b00ce1da7526c26/> renamed `progress::Read::reader` -> `progress::Read::inner`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 47 calendar days.
 - 51 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - update sha-1 dependency to 0.10 ([`361892c`](https://github.com/Byron/gitoxide/commit/361892ca15aa648802f6701ab6a5a30aedde3449))
    - remove slow/unnecessary threading utilities ([`269b7ef`](https://github.com/Byron/gitoxide/commit/269b7efc47bb1d6380b2059f63bd0c53fcd285de))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Add `progress::Write` to automatically pass bytes written to a progress instance ([`0a749a2`](https://github.com/Byron/gitoxide/commit/0a749a22057b5513a8cefa0e26b0a9a268c769d3))
    - renamed `progress::Read::reader` -> `progress::Read::inner` ([`61e5cfe`](https://github.com/Byron/gitoxide/commit/61e5cfece4d8f405e35fc1957b00ce1da7526c26))
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - upgrade to prodash 17 ([`47860b7`](https://github.com/Byron/gitoxide/commit/47860b7e2769260cfb8522ae455c491605093423))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
    - fix docs ([`3f89b63`](https://github.com/Byron/gitoxide/commit/3f89b6336e79bc12bc31d40b74221e79a72d2b36))
    - fix build ([`e3977fe`](https://github.com/Byron/gitoxide/commit/e3977fe033550bfd3297cdd674934e40476aa38b))
    - Use InOrderIter from git-features ([`7721b5f`](https://github.com/Byron/gitoxide/commit/7721b5fc7cba86d785e0936fdfab2ea41163219f))
    - Add InOrderIter to 'parallel' module ([`cb7e4e7`](https://github.com/Byron/gitoxide/commit/cb7e4e784d615f9fa3d6fb9c36240f0592403358))
    - Make a scope-like abstraction available ([`ca095ed`](https://github.com/Byron/gitoxide/commit/ca095ed881db2a8f06a6b067dbaac17e923b0945))
    - single and multi-threaded index tests ([`a22cb0f`](https://github.com/Byron/gitoxide/commit/a22cb0f1ead9a2f32e43eb2fb378281e592a4ed3))
    - decoding of variable int numbers. ([`b8400ed`](https://github.com/Byron/gitoxide/commit/b8400ed80543d67a5895c975ba9b1fc28427411c))
 * **Uncategorized**
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - upgrade dependencies ([`968df47`](https://github.com/Byron/gitoxide/commit/968df4746729556dcf4f5039b1d1ed1a1da2705a))
    - refactor ([`e7fbd9f`](https://github.com/Byron/gitoxide/commit/e7fbd9f3700496ad7bb7e71226c4d25429f0ccd5))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.18.0 (2021-11-29)

### New Features

 - <csr-id-7e95d8ab29051ffc892f2dcbaf5369e8c7e7b294/> add threading primitives with feature toggle
   If the `threading` feature is set, the `threading` module will contain thread-safe primitives
   for shared ownership and mutation, otherwise these will be their single threaded counterparts.
   
   This way, single-threaded applications don't have to pay for threaded primitives.

### Changed (BREAKING)

 - <csr-id-e7526b2a7b51cbac4018e1ab3b623a85987fadc2/> parallel utilities now use `Send + Clone` insted of `Send + Sync`
   This helps to assure that thread-local computations always work with the
   kind of types we provide. The ones that are carrying out actions are
   notably not `Sync` anymore.
   
   We cater to that by defining our bounds accordingly, but for those
   who want to use other utilities that need Sync, using types like
   `Repository` and `thread_local!()` is the only way to make this
   work.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 5 calendar days.
 - 40 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#259](https://github.com/Byron/gitoxide/issues/259), [#263](https://github.com/Byron/gitoxide/issues/263)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - unify trait bounds for parallel code: prefer Clone over Sync ([`c805d0b`](https://github.com/Byron/gitoxide/commit/c805d0b231cf4d2f51dae7705bfbbc6562f86c32))
    - remove trait bounds to allow single-threaded applications to exist ([`3c790e0`](https://github.com/Byron/gitoxide/commit/3c790e01de0dbd3ffa2683d5cf060723d11d64a5))
 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - Make it possible to return read guards with packed buffers ([`f5c3c8f`](https://github.com/Byron/gitoxide/commit/f5c3c8f7309bf53b9e53f786e75931d701a8585c))
    - parallel utilities now use `Send + Clone` insted of `Send + Sync` ([`e7526b2`](https://github.com/Byron/gitoxide/commit/e7526b2a7b51cbac4018e1ab3b623a85987fadc2))
    - A mad attempt to use thread-local everywhere and avoid Sync… ([`0af5077`](https://github.com/Byron/gitoxide/commit/0af5077e1f028c1c69bbdc098bb567e486282c37))
    - add threading primitives with feature toggle ([`7e95d8a`](https://github.com/Byron/gitoxide/commit/7e95d8ab29051ffc892f2dcbaf5369e8c7e7b294))
 * **Uncategorized**
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - thanks clippy ([`db1bb99`](https://github.com/Byron/gitoxide/commit/db1bb99101a9248b464b0df9f526067b8f2a184e))
    - Merge branch 'pack-consistency' ([`5982406`](https://github.com/Byron/gitoxide/commit/5982406b4e1b26fd383d9ec21a3cf652ec8ab25f))
</details>

## v0.17.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.16.5 (2021-10-15)

This release contains no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Handle changelogs with upcoming version section if they were left for editing ([`0f5f47d`](https://github.com/Byron/gitoxide/commit/0f5f47da4662b596cbbbd9c0d83e135e2cc52c11))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - new changelogs for actor and features crates ([`e0d437c`](https://github.com/Byron/gitoxide/commit/e0d437c4cfa06e0792609f41ed5876c390634921))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
</details>

## v0.16.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 6 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.16.4 ([`fd189c7`](https://github.com/Byron/gitoxide/commit/fd189c7d973ad2a639d319f3761f37aa90712ef6))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [features #190] be more explicit about why sha1-asm is disabled ([`507d710`](https://github.com/Byron/gitoxide/commit/507d710d837c3911a9329c1c132eee912a37e1a8))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com/Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [actor #190] methods to get an actor signature at the current time ([`6d0bedd`](https://github.com/Byron/gitoxide/commit/6d0beddb20092a80b113a39c862d6b680d79deb6))
    - [features #189] simple UTC-offset support for git-features ([`b58134b`](https://github.com/Byron/gitoxide/commit/b58134bbd132f9e685d1adf7859ec5219c16dd25))
    - [features #???] WIP local time ([`1388ebf`](https://github.com/Byron/gitoxide/commit/1388ebf0925eb326ec3045d7f83bd5beda22a6fe))
    - [#189] Upgrade to prodash 16… ([`8e98418`](https://github.com/Byron/gitoxide/commit/8e98418652926860f58906a6f21a3210e2f0183f))
</details>

## v0.16.3 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 5 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [pack #67] Optimize caches based on cache debugging ([`1271c01`](https://github.com/Byron/gitoxide/commit/1271c01d2635ab49474add61a9feb78b98bd6180))
    - [pack #67] Add cache debugging capabilities to git-features ([`8776c98`](https://github.com/Byron/gitoxide/commit/8776c9834ac4622b3057f5db464a9817ed9acdb0))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - thanks clippy ([`d689599`](https://github.com/Byron/gitoxide/commit/d689599d1b819c18a3be60075170dbe00462e216))
    - [features] refactor ([`0958fc8`](https://github.com/Byron/gitoxide/commit/0958fc8dbaa72dda0c1e2d40a88d74b4e18bfe39))
    - [features] refactor ([`d4605cd`](https://github.com/Byron/gitoxide/commit/d4605cde6d825c0bfaf4282c4cbd85d9f07dc43f))
</details>

## v0.16.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.16.2 ([`42861ca`](https://github.com/Byron/gitoxide/commit/42861ca4f0cc9b741d033d4ffa50147b08513b58))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.16.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 ([`e10e55c`](https://github.com/Byron/gitoxide/commit/e10e55c1bf1b40965da9b8b6c87953e6eafda09a))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
</details>

## v0.16.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 55 commits contributed to the release over the course of 78 calendar days.
 - 93 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - upgrade prodash/crosstermion ([`f109409`](https://github.com/Byron/gitoxide/commit/f1094099de028deabbee3587a70291a7e625e328))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - [pack] fix build ([`98dd557`](https://github.com/Byron/gitoxide/commit/98dd557b963acfe1c4e717451d222c187c46a5da))
    - [pack] all tests running for now, but… ([`aec8439`](https://github.com/Byron/gitoxide/commit/aec8439683c639f7b6e344cb76bf1dd9fc769d17))
    - refactor sha-1 specification to avoid duplication ([`e23d19c`](https://github.com/Byron/gitoxide/commit/e23d19cd339f0ca5420c82e8125d2c9c7dfcb0da))
    - resolver = 2: works! ([`6dc8779`](https://github.com/Byron/gitoxide/commit/6dc877993135ce86649b239821e5b374251743d0))
    - try windows one more time: resolver = "2" ([`69d52b8`](https://github.com/Byron/gitoxide/commit/69d52b8ed7a733fe7f31826e576ba8b19619b148))
    - Fix windows, leave todo, move on ([`2de9e78`](https://github.com/Byron/gitoxide/commit/2de9e78dba35de31456eb553ae703120de23cba6))
    - See if turning off "asm" support entirely fixes windows ([`b804ef2`](https://github.com/Byron/gitoxide/commit/b804ef2ea6da1ebffaab4d09d0b91eae98ff70c9))
    - Try to fix build, again ([`c616627`](https://github.com/Byron/gitoxide/commit/c616627cc9984e40798120a801387fc179d6640b))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com/Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - [features] enable ASM for everyone… ([`7a1128f`](https://github.com/Byron/gitoxide/commit/7a1128f594c5395a22e5e2b23772bce1d4de7a37))
    - [ref] reproducible loose ref iteration with built-in sorting ([`e138748`](https://github.com/Byron/gitoxide/commit/e13874807ccc3cbc2b4aacccf63ac5c3dd21c445))
    - [features] fix docs in the absence of sha1 related features ([`6ca02ac`](https://github.com/Byron/gitoxide/commit/6ca02ace7552c1ffaead81929bc751d96afa713a))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] first rough implementation of loose ref iteration ([`918af42`](https://github.com/Byron/gitoxide/commit/918af425298a1fdbb8e7dd6328daefe9eaa10cef))
    - refactor ([`2174513`](https://github.com/Byron/gitoxide/commit/21745135ced62411be535ebbc827b3638726318b))
    - fix docs ([`e68d460`](https://github.com/Byron/gitoxide/commit/e68d460716dc51c7f4757c11f3c8af6c3881e2cf))
    - Remove mentions of interrupt handling feature toggles ([`833ac04`](https://github.com/Byron/gitoxide/commit/833ac0464b42bd3ecc76c6263b4b06e8ab4ff182))
    - Fix everything up so that… ([`5930563`](https://github.com/Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com/Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - First step towards moving git-features::interrupt… ([`8a741d0`](https://github.com/Byron/gitoxide/commit/8a741d0c5423ed7c35d9382307c760a6b9460ccd))
    - fix build ([`ea2bfac`](https://github.com/Byron/gitoxide/commit/ea2bfac65f742ca7617bc77a50376c29156c4ea5))
    - refactor ([`7f9be36`](https://github.com/Byron/gitoxide/commit/7f9be36ea909ee67555591287bcb140fdc54c801))
    - And one less usage of the global interrupt handler… ([`5da57a3`](https://github.com/Byron/gitoxide/commit/5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23))
    - Make most interrupts local to the method or function ([`4588993`](https://github.com/Byron/gitoxide/commit/458899306a3f3c8578f185d7ecbf1ade2a7142dd))
    - fix build ([`04d919f`](https://github.com/Byron/gitoxide/commit/04d919f9228d287912554275194487870500d18c))
    - refactor ([`e0b7f69`](https://github.com/Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [features] sketch of iterator to auto-check for interruptions ([`61d3a15`](https://github.com/Byron/gitoxide/commit/61d3a15c66b4c1be1d98715b8a60705a3a314455))
    - [tempfile] integrate with git-features to have a single top-level interrupt… ([`6e9400d`](https://github.com/Byron/gitoxide/commit/6e9400d9cb8e370d870c3aa635718b134c82268f))
    - [features] protect interrupt handler from multi-initialization ([`592404c`](https://github.com/Byron/gitoxide/commit/592404c2b24dc9d24465ff5f73216213436a277a))
    - [interrupt] remove any user mesasages as it can't be done in a handler. ([`8a10af7`](https://github.com/Byron/gitoxide/commit/8a10af77db654ebce940bb05f8eefd171036ef40))
    - [tempfile] a first somewhat working version of signal-hooks for interrupt handling ([`07b3242`](https://github.com/Byron/gitoxide/commit/07b3242e446cb4520dbc54308632ab6221fc19c8))
    - Update to latest prodash to get rid of ctrlc ([`c070d6f`](https://github.com/Byron/gitoxide/commit/c070d6f5273d7ef9049ddd02fd26332623dc0ae6))
    - refactor ([`2e86723`](https://github.com/Byron/gitoxide/commit/2e8672312a4b1e2638e3ffe82a97cc2f87b496cf))
    - Bump crossbeam-utils from 0.8.4 to 0.8.5 ([`fce4d10`](https://github.com/Byron/gitoxide/commit/fce4d107c7abc778bbdfcd37349c3075e54fd756))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-pack] fix docs ([`efd20d4`](https://github.com/Byron/gitoxide/commit/efd20d4e1afbfbe573d620dea4761c06f948a296))
    - [git-features] fix compilation ([`38c7961`](https://github.com/Byron/gitoxide/commit/38c796142dc5823e1cb14906d9cd4040a8c3be3a))
    - [git-pack] move hash-writer to git-features as it's quite general purpose ([`80e5640`](https://github.com/Byron/gitoxide/commit/80e5640169363910b4189fda58bb495c6677eaaa))
    - [git-features] Remove feature that would break licensing agreements ([`cd6ce67`](https://github.com/Byron/gitoxide/commit/cd6ce673308e7e5b1e86fb682ee3ace2ca9ae18c))
    - [git-features] fix typo ([`c6f342f`](https://github.com/Byron/gitoxide/commit/c6f342f3d29a969a08d037f01eb24555bc03e85e))
    - [git-features] Finally zlib with feature toggles is working… ([`057016e`](https://github.com/Byron/gitoxide/commit/057016e2df3138992c4857f9b65bf19dc2c9a097))
    - [git-features] And now zlib finally works! ([`6d887d5`](https://github.com/Byron/gitoxide/commit/6d887d589a57e159986f049c8a9e19c52ce7b85b))
    - [git-features] simplify even more ([`ca54d97`](https://github.com/Byron/gitoxide/commit/ca54d97d579dd4f16025a2325d5e39431f6e8a36))
    - [git-features] refactor to help understand a zlib-related logic bug ([`ae826e8`](https://github.com/Byron/gitoxide/commit/ae826e8c3240efd14939beedd33a06695a6c112b))
    - [git-features] a first step towards supporting a pure rust zlib backend ([`040cab7`](https://github.com/Byron/gitoxide/commit/040cab7f27de83b283957189244d523d71ca1457))
    - [git-features] Add zlib module to allow changing implementation on the fly ([`4bdf783`](https://github.com/Byron/gitoxide/commit/4bdf7833d99d3c2884b9747614f9c14a06c1e945))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com/Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Put prodash behind a feature toggle, too ([`966058d`](https://github.com/Byron/gitoxide/commit/966058d611c548e90c050462de52e36f1925e775))
    - Put 'walkdir' behind a feature flag/make it optional. ([`1a3cc5b`](https://github.com/Byron/gitoxide/commit/1a3cc5bea1868ed3ae015403fbe0cdec788be749))
    - Put 'sha1' behind a feature toggle ([`4f326bc`](https://github.com/Byron/gitoxide/commit/4f326bc261c4e7f0d5510df74ad4215da3580696))
    - Use crc32fast instead of `crc` ([`11955f9`](https://github.com/Byron/gitoxide/commit/11955f95e200ef75e752a833952d288fbd0fc389))
    - Put crc functionality behind a feature toggle ([`458fa6e`](https://github.com/Byron/gitoxide/commit/458fa6ec726ec7901c1f6d970cbb1c1ea975dded))
</details>

## v0.14.0 (2021-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 5 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com/Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - upgrade to prodash 13/tui 0.15 ([`1c99f51`](https://github.com/Byron/gitoxide/commit/1c99f51b35b4ba85792a3b32dbb7e48052facc5e))
</details>

## v0.13.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 12 calendar days.
 - 21 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - Allow calling 'finalize()' on the entries iterator ([`3c617bc`](https://github.com/Byron/gitoxide/commit/3c617bc2ae59adbb12c254308269e745149d462b))
    - git-odb without cargo warnings due to using the same test twice ([`8945f95`](https://github.com/Byron/gitoxide/commit/8945f95364b489e7a639d74dd0f28b17e82e70f3))
    - Fix compile warning for git-features ([`d457faa`](https://github.com/Byron/gitoxide/commit/d457faac6bb56a229b74147c8a4cf2484026bb1a))
    - fix doc links ([`870af2a`](https://github.com/Byron/gitoxide/commit/870af2a6949bcb1f7f45bc0ff98d9e9a07014b22))
    - run git-odb tests in parallel, too; improved threaded error handling ([`40802fd`](https://github.com/Byron/gitoxide/commit/40802fd8bbb15b8a61249522d67f3a5b28da64b3))
    - refactor ([`82c2f42`](https://github.com/Byron/gitoxide/commit/82c2f428e22c3cda79913c9ca2f092c377d692aa))
    - refactor ([`7a6b514`](https://github.com/Byron/gitoxide/commit/7a6b514a5b9b93bf574cd3a114f27ad5967e89ac))
    - refactor ([`5ef1f22`](https://github.com/Byron/gitoxide/commit/5ef1f22c1e12ff8d607663d4dfbbbfe426a29e0f))
    - fix docs #(67) ([`01db10a`](https://github.com/Byron/gitoxide/commit/01db10a27431ad89a68ed3e4eabae810748a6f29))
    - refactor ([`3e908bd`](https://github.com/Byron/gitoxide/commit/3e908bd4b4077c4a5d113cefc113f9d71f249133))
    - refactor ([`409d763`](https://github.com/Byron/gitoxide/commit/409d763d2fca974a647487c72d15f568a9b62ccb))
    - refactor ([`896ab94`](https://github.com/Byron/gitoxide/commit/896ab940bcd475d026e4009b3aa2fa6a025c14bc))
    - Remove unused dependency ([`26beb2a`](https://github.com/Byron/gitoxide/commit/26beb2a5ad87e173fd3d13d17b0e9676a650cac9))
    - Don't finish the computation on drop of SteppedReduce ([`6453633`](https://github.com/Byron/gitoxide/commit/6453633f1420327aee07dca2ad27abd8f96108c0))
    - thanks clippy ([`c320761`](https://github.com/Byron/gitoxide/commit/c320761821b08946a2b37e219400ded853a86408))
    - Remove unsafe interface for stepped computation #(67) ([`c856613`](https://github.com/Byron/gitoxide/commit/c856613a35aea7dea1d093bfcfe1ddbde93fdf26))
    - A first working version of a static parallel iterator #(67) ([`d7d5c68`](https://github.com/Byron/gitoxide/commit/d7d5c6855a038a8b01571a6a16a61fe0d8036d30))
    - A way iteration won't work with 'static #(67) ([`6fda1f2`](https://github.com/Byron/gitoxide/commit/6fda1f20a57b9dcc1a5818d8d0b656218b383230))
    - Sketch of machinery for producing pack entries #(67) ([`ac8e7fb`](https://github.com/Byron/gitoxide/commit/ac8e7fb6c8ae4ac42f56482d9d7744aa66132702))
    - Less restrictive requirements: Clone instead of Copy #(67) ([`410e7d6`](https://github.com/Byron/gitoxide/commit/410e7d64049b5a749113126f5412a61ae4b79887))
    - Improve Safety docs #(67) ([`15e4748`](https://github.com/Byron/gitoxide/commit/15e47480054d9a517c28f47db3b5fa87968a307e))
    - A test to assure referenced input and references in 'consume' work #(67) ([`4526d82`](https://github.com/Byron/gitoxide/commit/4526d82fab4d6e8f2ab05497aa5893d5a8f8b253))
    - Make iterator creation unsafe and document why #(67) ([`593d5df`](https://github.com/Byron/gitoxide/commit/593d5df478e67e28f9b3d48b201ff6830208726f))
    - First seemingly working version of an iterator which allows controlling threaded work #(67) ([`4a7ef7d`](https://github.com/Byron/gitoxide/commit/4a7ef7d6398c2ff5dd6aac41f8224cd2d61ee189))
    - Make the parallel SteppedReduce compile #(67) ([`017fdf4`](https://github.com/Byron/gitoxide/commit/017fdf48972a6a09e5155bd76bd437d8e195dae3))
    - More docs to differentiate SteppedReduce from in_parallel() #(67) ([`153c083`](https://github.com/Byron/gitoxide/commit/153c0837bbf1df3b5cb386e08265f9b06eaee2a9))
    - serial version of SteppedReduce seems to be working #(67) ([`779542e`](https://github.com/Byron/gitoxide/commit/779542e4f4c951e9b16d2310146020da9ce36859))
    - Only store thread state #(67) ([`0bf8a9b`](https://github.com/Byron/gitoxide/commit/0bf8a9b3c4a086732ee04f81c6a214296d49eab9))
    - sketch instantiation of iterator adapter #(67) ([`a3083ad`](https://github.com/Byron/gitoxide/commit/a3083ad3aad7984afc6b6d343ca7453f79897062))
    - A reducer test in preparation for allow it to be used as iterator #(67) ([`1c2adf4`](https://github.com/Byron/gitoxide/commit/1c2adf4a546273489bf8224eb7982dbdf3fb6aca))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com/Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
    - Allow parallel reducers to produce something during 'feed()' #(67) ([`6c04fcd`](https://github.com/Byron/gitoxide/commit/6c04fcd643083d9db633edd3bb838b4f5de8f0db))
</details>

## v0.12.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 10 calendar days.
 - 74 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com/Byron/gitoxide/issues/63)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - git-protocol uses `oid` type ([`3930a6f`](https://github.com/Byron/gitoxide/commit/3930a6ff508f5bb2249fb2c2f21e00b74fecda22))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Move git-hash::owned::Id into git-hash::Id ([`fdbe704`](https://github.com/Byron/gitoxide/commit/fdbe704b6c9ace2b8f629f681a0580b24749a238))
    - Rename `git_hash::*::Digest` to `Id` ([`188d90a`](https://github.com/Byron/gitoxide/commit/188d90ad463d342d715af701b03f0ed392c977fc))
 * **Uncategorized**
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com/Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - refactor ([`dee8c66`](https://github.com/Byron/gitoxide/commit/dee8c66e300dc2a2b6e1a6d6c3674a7ce6aac687))
</details>

## v0.11.0 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com/Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
</details>

## v0.10.1 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 38 calendar days.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.10.1 ([`0dcdfd7`](https://github.com/Byron/gitoxide/commit/0dcdfd754649240f43fe0f4b6e1245e8c7b89635))
    - Remove usage of gitfeatures::fs in organize subcommand ([`b567d37`](https://github.com/Byron/gitoxide/commit/b567d3709a74e9fdafef54b0fe58ca82721cd773))
    - Assure basic 'organize' operation is working as expected ([`deb6073`](https://github.com/Byron/gitoxide/commit/deb6073671ae95de674aaef7ca01e03f95b41ca8))
    - A first stab at finding git repositories ([`e4dc964`](https://github.com/Byron/gitoxide/commit/e4dc96403894f1fe509335905679347ecdf535c7))
    - upgrade 'jwalk' ([`cba048f`](https://github.com/Byron/gitoxide/commit/cba048f094858388f4242e37a2409fe0822f8c07))
    - upgrade 'bytes' ([`3934392`](https://github.com/Byron/gitoxide/commit/39343922b4a1129394aa788a9591920aee077569))
    - upgrade prodash and friends ([`50755bc`](https://github.com/Byron/gitoxide/commit/50755bc83f73072dc629301bf69c5c065d5c2aa4))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
</details>

## v0.10.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - use git-hash in git-features ([`5b307e0`](https://github.com/Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.9.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 18 calendar days.
 - 19 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com/Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com/Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - more docs for owned git-object ([`b79101d`](https://github.com/Byron/gitoxide/commit/b79101d714f59a42a30eb47776486a212ec0f738))
    - fix io::pipe tests ([`9604154`](https://github.com/Byron/gitoxide/commit/9604154e687813a11f0eee469e408561a6a74a4e))
    - uograde everything else ([`0cd79d0`](https://github.com/Byron/gitoxide/commit/0cd79d00bce3f042b5cc849cf48739e29f95fcb0))
    - upgrade prodash and tui ([`b5eadca`](https://github.com/Byron/gitoxide/commit/b5eadca343bbaa1af86722b5f1bcd33f4e3939a6))
    - add remaining docs to git-features using the missing_docs directive ([`f8aafd6`](https://github.com/Byron/gitoxide/commit/f8aafd6c78687899a2ca3a3e6147d93fc45b8cb9))
</details>

## v0.8.0 (2020-11-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com/Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - finish git-features documentation ([`934a26c`](https://github.com/Byron/gitoxide/commit/934a26c5e254baf2be9178096b6dead0e4c1ed1d))
    - refactor ([`b3a8bb5`](https://github.com/Byron/gitoxide/commit/b3a8bb5f7f0c6e80259922546928c2739c24f7b5))
    - refactor ([`f9e8d29`](https://github.com/Byron/gitoxide/commit/f9e8d2932c02c22bf57acd39fb0a9e6d521070bd))
    - docs for the git-features::pipe module ([`67a950a`](https://github.com/Byron/gitoxide/commit/67a950a2e0fd56b29565668ed0a0f399d5aa989d))
    - Document git-features::parallel ([`b899227`](https://github.com/Byron/gitoxide/commit/b8992275cd4310b05494be41c059e9b6049d06b1))
    - dependency update ([`fb077f9`](https://github.com/Byron/gitoxide/commit/fb077f9fecb89ed8a60d57b45726401883e838bf))
    - finish git_features::interrupt docs ([`471a1bf`](https://github.com/Byron/gitoxide/commit/471a1bf24efee70f21b15839cdc9f8ebe319f917))
    - dependency update ([`b3b4aba`](https://github.com/Byron/gitoxide/commit/b3b4aba5e05596befecd17e225067be9315b74fd))
    - docs for git-features::hash ([`a3fdecc`](https://github.com/Byron/gitoxide/commit/a3fdecc9a3587b20c01e3b3a2d51190138131c3d))
    - first sketch of filesystem docs for git-features ([`1a8141c`](https://github.com/Byron/gitoxide/commit/1a8141c2c4a8bcc79d68049a35bd8aba5ab822a3))
</details>

## v0.7.0 (2020-11-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 63 calendar days.
 - 65 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com/Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - Merge branch 'commit-graph' into main ([`9cb09b2`](https://github.com/Byron/gitoxide/commit/9cb09b248796f0ff5c9d3f3e857de4731324cfd5))
    - specify the hash to create with 'hash::bytes_of_file' ([`c000294`](https://github.com/Byron/gitoxide/commit/c000294423ae0759b978399db3b69ac07c20578d))
    - move 'git_odb::hash::bytes_of_file' into git_features::hash ([`c5f6b45`](https://github.com/Byron/gitoxide/commit/c5f6b4587ee4042a080c0505613b0c72fdfe5273))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge branch 'main' into commit-graph ([`ca5b801`](https://github.com/Byron/gitoxide/commit/ca5b80174b73cc9ac162b3f33b5d3721ef936cb1))
    - Use parallel walkdir (via jwalk) when parallel feature is enabled ([`f444c85`](https://github.com/Byron/gitoxide/commit/f444c859f5b215ea70a46d5493a2babbf7a98235))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
</details>

## v0.6.0 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com/Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
    - Switch to prodash 10 and safe a lot of trait bounds in the process ([`e2fb1d9`](https://github.com/Byron/gitoxide/commit/e2fb1d944b4d803a11c91f868b831d406fb5e35f))
</details>

## v0.5.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 21 calendar days.
 - 24 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com/Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com/Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [clone] test (and fix) for piped line reading ([`afe2996`](https://github.com/Byron/gitoxide/commit/afe2996689b5bea915ac5f142d320056faf49899))
    - [clone] Send headers with BufReaders ([`6a95aaa`](https://github.com/Byron/gitoxide/commit/6a95aaab582941c6d1697dde6982c0aa8896c73d))
    - [clone] pipe allows to send errors as well ([`69286ec`](https://github.com/Byron/gitoxide/commit/69286ecb3680b5071693ef0d9fb2e9345b2722d4))
    - [clone] BufRead for Reader… ([`bf1d40f`](https://github.com/Byron/gitoxide/commit/bf1d40f2d44a9b04ffe2134ddcd3779985cdafc4))
    - [clone] a piped iterator ([`5148c85`](https://github.com/Byron/gitoxide/commit/5148c85efc70c0ec06be3ebce267ce727c8ee4e1))
    - [clone] pipe probably shouldn't abort on empty writes ([`9cfa9b7`](https://github.com/Byron/gitoxide/commit/9cfa9b79841187167f0f96abfd1c17a37b4c365d))
    - thanks clippy ([`c4f570f`](https://github.com/Byron/gitoxide/commit/c4f570fcae7e21745a37a4265b05d21e6149157b))
    - [clone] more pipe tests ([`1652a74`](https://github.com/Byron/gitoxide/commit/1652a74761631cadfc6feab366adc0808d83063d))
    - [clone] first working pipe implementation ([`490a9b9`](https://github.com/Byron/gitoxide/commit/490a9b96915a760e339e576d9f49737b43a8739f))
    - [clone] frame for implementing 'pipe' support ([`c555681`](https://github.com/Byron/gitoxide/commit/c55568127ff943cc6749dba5054d7b3e93c049eb))
    - Fix git-features hash tests ([`35e8809`](https://github.com/Byron/gitoxide/commit/35e8809f6bc7d19ed9e0bac8e3af85f433978901))
</details>

## v0.4.0 (2020-08-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com/Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - [protocol] properly implement remote progress reporting ([`a81954a`](https://github.com/Byron/gitoxide/commit/a81954a6a37afacd51add6661a656b8fb663ca54))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - add 'disable-interrupts' feature flag ([`ccd9c3e`](https://github.com/Byron/gitoxide/commit/ccd9c3e2d37aa6898dc17f47a82c187baa810b03))
    - refactor ([`b4a6e16`](https://github.com/Byron/gitoxide/commit/b4a6e16364822c0dccb56f98dbfb0ca4c8007069))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 16 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com/Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - thanks clippy ([`6725104`](https://github.com/Byron/gitoxide/commit/6725104d2841e6518db641d06e3e107cf4f40f96))
    - first step towards parallelizing file hashes and traversal! ([`9573836`](https://github.com/Byron/gitoxide/commit/95738369e0d3accf7f6239c8cd966a7f5c36825a))
    - better usability for units ([`b226253`](https://github.com/Byron/gitoxide/commit/b226253636d8146a084a7bcd7c0c320e37f9d2fb))
    - update dependencie ([`ade06b4`](https://github.com/Byron/gitoxide/commit/ade06b46bb3c16ac1e26dbbb4a7045f0c09f2d8e))
    - Make obvious that interrupt request was received ([`34b2373`](https://github.com/Byron/gitoxide/commit/34b23737f560fe52d4f98fb886eba754652f9a5e))
    - Remove once_cell dependency as it is really not required anymore ([`5ac9538`](https://github.com/Byron/gitoxide/commit/5ac95385cc8d1c50c16da6e5fb0c66ac138f9966))
    - make interrupt handler work reliably ([`e71da0f`](https://github.com/Byron/gitoxide/commit/e71da0fce6d6eab68f7b81b13cdc78ce8e9b7ee3))
    - Conditionally use an eager iterator… ([`e9b5511`](https://github.com/Byron/gitoxide/commit/e9b5511568f4e64968596994855783f19672d678))
    - refactor ([`d14f0f6`](https://github.com/Byron/gitoxide/commit/d14f0f6c3b5f303df75b33aadbf16653075d2272))
    - Allow eager iterator to behave properly when used with index writing ([`66ebc5f`](https://github.com/Byron/gitoxide/commit/66ebc5f1ad5f262eb464dc7ca0892ec952d34382))
    - first successful test of moving the streaming iterator into its own thread ([`c9fcb68`](https://github.com/Byron/gitoxide/commit/c9fcb68c644c96a15cb9956a754bec7b65bb5fbd))
    - now it's order preserving ([`4c8711e`](https://github.com/Byron/gitoxide/commit/4c8711e51efd88e0f159ad02de2692c4cb72ce27))
    - first sketch of order-destroying eager iterator ([`20fca45`](https://github.com/Byron/gitoxide/commit/20fca4515f6e9ea320d0bf21c15cd6d2c3cff742))
    - Print read throughput automatically ([`0a71b48`](https://github.com/Byron/gitoxide/commit/0a71b482310a129aa8757475290b3b24a200b702))
    - Make sure interrupt logic works even without an interrupt handler… ([`66b1644`](https://github.com/Byron/gitoxide/commit/66b164472f5893f9e634ac1f9147a41dc742296d))
    - Add percentage and throughput to tasks that matter ([`763d7ca`](https://github.com/Byron/gitoxide/commit/763d7caa4c70111b7cb3ef5733d2c3c697758c28))
    - Upgrade to latest iteration of prodash ([`3a4faec`](https://github.com/Byron/gitoxide/commit/3a4faecab56e37670c553e6563f11a46d740c333))
    - First part of migration to prodash 8.0, but… ([`6901a09`](https://github.com/Byron/gitoxide/commit/6901a098641820c8d974ce56a24d6cdca779730d))
    - thanks clippy ([`ed5882d`](https://github.com/Byron/gitoxide/commit/ed5882d75e0a9fceb0628e84302eb49a66277fa6))
    - Write about user interfaces and the use/non-use of async ([`91ba045`](https://github.com/Byron/gitoxide/commit/91ba0457745f860b7a68cb38b13e69754747e8d9))
    - interrupt support for pretty plumbing ([`bca7ce2`](https://github.com/Byron/gitoxide/commit/bca7ce2e668a4be2600d2d04d00f46b21c82eee2))
    - support for interruptible operations ([`a025593`](https://github.com/Byron/gitoxide/commit/a02559378f9165df97a217f24834a851be719b08))
    - refactor ([`413968d`](https://github.com/Byron/gitoxide/commit/413968dfee5e5a66ed9e63823f6bda5a5a22753e))
    - receive progress information when reading packs in bundle ([`759091d`](https://github.com/Byron/gitoxide/commit/759091d3c6696b427d7b5aab1b6da05a0d268c04))
    - initial batch of progress usage for index creation… ([`b10e5c6`](https://github.com/Byron/gitoxide/commit/b10e5c664be9bd1bdb2b72b858ebaf35c1ed4cb4))
    - first stab at streaming pack header encoding ([`3c6e78b`](https://github.com/Byron/gitoxide/commit/3c6e78bec9cbd4df842919cc8dc3c575414ed002))
    - We can now restore (possibly half-written) packs ([`b1daa46`](https://github.com/Byron/gitoxide/commit/b1daa465c40ea8c7c9de69a18e467d69459d911e))
    - See how big a Sha1 hasher really is ([`26b271d`](https://github.com/Byron/gitoxide/commit/26b271d44863fb184b0a947c3a9da2b3252f9a78))
    - First sketch of new verify expressed in terms of traversal ([`4cb570f`](https://github.com/Byron/gitoxide/commit/4cb570f96ddd7ee2faa62e54927afd78ba7822af))
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 10 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`0bb8314`](https://github.com/Byron/gitoxide/commit/0bb831480d8657e1bb29ee7009aeac673471403e))
    - incorporate dynamic chunking into 'less-time' algorithm ([`295aa2f`](https://github.com/Byron/gitoxide/commit/295aa2f01dc596a8880cd2f68a8d83bc6913ce48))
    - integrate new chunk size code into lookup code ([`a8422cf`](https://github.com/Byron/gitoxide/commit/a8422cf0b0c9ff4d3275cc17a68a74811b5bd01f))
    - first round of number tuning done ([`a647b2d`](https://github.com/Byron/gitoxide/commit/a647b2da2905c4079e646ea44cbec778f3f7c71f))
    - Somehow handle chunk size in absence of known chunk amount ([`acfccad`](https://github.com/Byron/gitoxide/commit/acfccadef40ebcc67f8dea4e58c02392b7e2e7de))
    - Chunk computation seems alright, what about realistic values ([`973e6bb`](https://github.com/Byron/gitoxide/commit/973e6bb3d67d89eec2faf2467a129d992b90ed72))
    - getting there… ([`a1b5d56`](https://github.com/Byron/gitoxide/commit/a1b5d565f305f0f2666fd59272d9bf9c62ae2962))
    - first step towards computing better chunk sizes and thread limits ([`1cdde7d`](https://github.com/Byron/gitoxide/commit/1cdde7d339a6ed3650c54f9b48154089d7da9919))
    - Add 'inc()' convenience methods to progress ([`2e46c9b`](https://github.com/Byron/gitoxide/commit/2e46c9b72a2a5b90bcdac249de07ffbc124cfb04))
    - (more) graceful shutdown of failing parallel tasks ([`163f50f`](https://github.com/Byron/gitoxide/commit/163f50fab81b425e6e306ec54fb1eb60a7c02cf8))
    - Respect thread limit in 'in_parallel' ([`babfd84`](https://github.com/Byron/gitoxide/commit/babfd84cba77ef7a0f541ba921b31ebd3f3c50e3))
</details>

## v0.1.0 (2020-07-12)

<csr-id-ab6f98b905f13ed2a7c0c483f34fab63141fbc5b/>

### Other

 - <csr-id-ab6f98b905f13ed2a7c0c483f34fab63141fbc5b/> try-join with static typing works, but…
   …seems like a lot of effort. Probably not worth continuing here

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 12 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable ([`5688a34`](https://github.com/Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Flume isn't actually needed for that… ([`c750022`](https://github.com/Byron/gitoxide/commit/c750022394928aa37a8400611f6fdf4ee77c0f69))
    - Don't just ignore send errors - we should panic for now ([`f128117`](https://github.com/Byron/gitoxide/commit/f128117138b24de780a00bb96e7c1c9f987e8aa0))
    - Proper implementation of line renderer into 'lean' CLI ([`e98e7c2`](https://github.com/Byron/gitoxide/commit/e98e7c280d73e9d9ebd13202afb93a56cb2f7c9c))
    - upgrade to prodash version 7 ([`af02b46`](https://github.com/Byron/gitoxide/commit/af02b46cc1eff5ba1da7da20d3f524a79fad686f))
    - update prodash to verion 6.0 ([`a4731a3`](https://github.com/Byron/gitoxide/commit/a4731a3aca159f8916b29d9ce5a71856089c5a6b))
    - Add metadata to allow docs.rs build all featueres ([`10f9386`](https://github.com/Byron/gitoxide/commit/10f9386a12decc1f13999aee72be484c8f6d48ce))
    - Switch to prodash 5.0 for windows support ([`88542e1`](https://github.com/Byron/gitoxide/commit/88542e117dd1c2e7606fcbe88b30c51b4c115989))
    - Allow to limit the logging depth for less cluttered output ([`fce7035`](https://github.com/Byron/gitoxide/commit/fce703531d7006f7d961d6ffa66f51f6c9bc0efc))
    - finally speed up logging progress properly - needs input throttling ([`1a550c6`](https://github.com/Byron/gitoxide/commit/1a550c6458b10fad2e42b641899216c5517c6e26))
    - Avoid calling system time too often in logs, it reduced performance ([`b17bd76`](https://github.com/Byron/gitoxide/commit/b17bd76d35822b3af174c74af3d6fac887889fe2))
    - Revert "ABORT: try-join with static typing works, but…" ([`b8b979b`](https://github.com/Byron/gitoxide/commit/b8b979b99b5f3848e0a6884c58594ba2b481a147))
    - try-join with static typing works, but… ([`ab6f98b`](https://github.com/Byron/gitoxide/commit/ab6f98b905f13ed2a7c0c483f34fab63141fbc5b))
    - Remove dependency to git-object from git-features - it better remains free ([`67c3a6a`](https://github.com/Byron/gitoxide/commit/67c3a6ab4cc32358a1406c2f863e26a4c2929867))
    - \#[forbid(unsafe)] for all crates ([`afda803`](https://github.com/Byron/gitoxide/commit/afda8039259b7a30cfed5dbcdd9caf4773b4c234))
    - Allow for more screen space when formatting ([`6794300`](https://github.com/Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - refactor ([`7add82c`](https://github.com/Byron/gitoxide/commit/7add82c39169e3c2fff76c48cdd318fe6040d7bc))
    - Automatically close the TUI when there is no progress anymore. ([`c416152`](https://github.com/Byron/gitoxide/commit/c416152b04051958de7bd161a8a2ee42ca163275))
    - pretty progress in a generalized form ([`caa883b`](https://github.com/Byron/gitoxide/commit/caa883b96827deb63b5c8787ed820d22f2c85249))
    - express DoOrDiscard in terms of Either (progress) ([`cb29a45`](https://github.com/Byron/gitoxide/commit/cb29a45f4e73bfaa25cbf623b1cda2435673028b))
    - Provide 'either' type with implementation for Progress ([`237bb5e`](https://github.com/Byron/gitoxide/commit/237bb5ee1c2b677f5bfd9ca7fdea9d9d2db865b3))
    - better trait bounds of `in_parallel_if`… ([`6264f2f`](https://github.com/Byron/gitoxide/commit/6264f2f99929ffaa4d50cdcae7bc296e1b4762f4))
    - First implementation of logging per thread ([`477dd90`](https://github.com/Byron/gitoxide/commit/477dd90ce5e102875b19489bf8ae9877522ef9c8))
    - Support for providing progress to threads ([`2815858`](https://github.com/Byron/gitoxide/commit/2815858adf7ac0f7b4cbc88cf05df0ea6aef4116))
    - first very basic progress implementation ([`b820717`](https://github.com/Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com/Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - Implement `Progress` trait for prodash::tree::Item ([`0eeb6d7`](https://github.com/Byron/gitoxide/commit/0eeb6d770d58621427bc88107a20860b89b86a24))
    - implement progress trait for logs with throttling ([`287eca9`](https://github.com/Byron/gitoxide/commit/287eca91b244ccbc703cb275b1ae032bfeb02532))
    - Add 'fast-sha1' to git-features ([`b22541f`](https://github.com/Byron/gitoxide/commit/b22541f0c39af470877119b136e4eb1b82dff2db))
    - A new crate to represent features that can toggle from the top-level ([`23c420c`](https://github.com/Byron/gitoxide/commit/23c420cc95219dc7c04d3905aaa03281cb51724e))
</details>

