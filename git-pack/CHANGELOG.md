# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.28.0 (2022-12-19)

### New Features

 - <csr-id-a85dcddba29a453bbea87ac865b8aedc97f41aed/> add `data::File::resolve_header()` to obtain object information without decompressing it.
 - <csr-id-c8835c6edae784c9ffcb69a674c0a6545dbb2af3/> upgrade to `prodash 21.1` and add `Ids` to all progress instances.
   That way callers can identify progress they are interested in, say, for
   selective visualizations.

### Changed (BREAKING)

 - <csr-id-37f3a675d7fa931f5d3f38e91df74ec0b517422b/> move `data::ResolveBase` into `data::decode_entry::`; unify `decode_entry|decode_header::Error` into `decode::Error`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'odb-iteration' ([`693a469`](https://github.com/Byron/gitoxide/commit/693a46977e2b57b93ee921320e008c8ad1beb81b))
    - Assure deltas are counted correctly, even if the base is out of pack. ([`ddaf47f`](https://github.com/Byron/gitoxide/commit/ddaf47f023970e8acfb98e8874da22f2604a92d9))
    - Merge branch 'read-header' ([`3d01252`](https://github.com/Byron/gitoxide/commit/3d0125271ec7bd606734bd74757a7e31a18c7ce5))
    - tweaks to immprove header decode performance ([`04df898`](https://github.com/Byron/gitoxide/commit/04df89835395d43e92ce3f0fee922f1799687aa4))
    - move `data::ResolveBase` into `data::decode_entry::`; unify `decode_entry|decode_header::Error` into `decode::Error`. ([`37f3a67`](https://github.com/Byron/gitoxide/commit/37f3a675d7fa931f5d3f38e91df74ec0b517422b))
    - add `data::File::resolve_header()` to obtain object information without decompressing it. ([`a85dcdd`](https://github.com/Byron/gitoxide/commit/a85dcddba29a453bbea87ac865b8aedc97f41aed))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - upgrade clru, remove it from git-repository dependencies (unused) ([`7e7547d`](https://github.com/Byron/gitoxide/commit/7e7547d995afc16192a1ee08add5a87560197fc9))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'optimize_hashtables' ([`95ad56c`](https://github.com/Byron/gitoxide/commit/95ad56c11489bc46d6eb2b2f48cf0bf01e954c58))
    - use newly added git-hashtable ([`50cb436`](https://github.com/Byron/gitoxide/commit/50cb4362010e1a5799fe782df36ac5fcdb48dd8a))
    - switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - upgrade to `prodash 21.1` and add `Ids` to all progress instances. ([`c8835c6`](https://github.com/Byron/gitoxide/commit/c8835c6edae784c9ffcb69a674c0a6545dbb2af3))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.27.0 (2022-11-21)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'main' into http-config ([`7c5b37d`](https://github.com/Byron/gitoxide/commit/7c5b37d28e98f59a6847368a0d0166d2dbb4acc1))
</details>

## 0.26.0 (2022-11-08)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.22.0, git-index v0.7.1, git-pack v0.26.0, git-odb v0.36.0, git-transport v0.21.2, git-repository v0.27.0, safety bump 6 crates ([`f0cab31`](https://github.com/Byron/gitoxide/commit/f0cab317bb0c2799fa80d16f3ae1b89d6aee4284))
    - prepare changelogs prior to release ([`f5f3a9e`](https://github.com/Byron/gitoxide/commit/f5f3a9edd038a89c8c6c4da02054e5439bcc0071))
    - Fix spelling where -> were ([`57355c7`](https://github.com/Byron/gitoxide/commit/57355c7b345d864aaf80fdbd9deec50ca78d94fd))
</details>

## 0.25.0 (2022-11-06)

### New Features

 - <csr-id-450257eb596465352fb363a5c8c514f544e7f9ac/> `Bundle::write…` also produces a `.keep` file
   These files are placed before moving the coresponding pack and index
   files into place to prevent them from being picked up for collection
   while their refs are still being placed.
   
   The caller is responsible for deleting them once refs integrate the
   contained objects into the commit graph.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 20 calendar days.
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
    - less noisy way of writing trait bounds ([`b593806`](https://github.com/Byron/gitoxide/commit/b593806ca3571d680801130ad528f266d3eab83e))
    - upgrade to `prodash` v21 ([`a0655dc`](https://github.com/Byron/gitoxide/commit/a0655dc7bc5dff388bc69a648e7f16b44fd1abd9))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Merge branch 'main' into gix-clone ([`3b48317`](https://github.com/Byron/gitoxide/commit/3b48317d6a9f41765d4f2a9f0a49c31afcdb68b6))
    - thanks clippy ([`93e7691`](https://github.com/Byron/gitoxide/commit/93e7691be421e40cc72e3e2e0506584a2fbd4857))
    - `Bundle::write…` also produces a `.keep` file ([`450257e`](https://github.com/Byron/gitoxide/commit/450257eb596465352fb363a5c8c514f544e7f9ac))
</details>

## 0.24.0 (2022-10-10)

<csr-id-b46347fd3d50886eeca500e31e1e12b354711309/>
<csr-id-591afd56d9862a6348ef8b3af61798004b36aa19/>

### New Features

 - <csr-id-6836cb148cbadf919bdac2e0e77bdec34caa9307/> more efficient distribution of tasks to threads during pack resolution.
   This clearly is a peak-performance optimization as it will now
   efficiently distribut tree-root nodes to threads one at a time by
   means of shared memory, which is accessed mutably yet safely.
   
   This change can also further reduce peak memory usage as it will not
   keep buffers of more resolved deltas then necessary, as it only handles
   one at a time per thread.

### Bug Fixes

 - <csr-id-ec58bef84d620360dc52e34e173ace1310f74c85/> reduce memory usage for pack resolution by 18%…
   …compared to where it started out before.
   
   This is the result of using u32 intead of usize where possible,
   leveraging the object limit of u32 in packs.
   
   This change makes the biggest difference by not keeping the decompressed
   memory of leaf nodes alive for longer than needed, at the cost of
   some code duplication which could be extracted into a function if one
   was inclined to deal with the boilerplate and lots of generics.
 - <csr-id-49d168a8859c6b5a9e7ef58cd836093212c2c7ad/> Allow verification of empty packs and indices.
   Empty packs are fine, even though the implementation should probably
   assure to not write them, or remove them after having been written.
 - <csr-id-a745512185fb0a46e35daaa6d28829aec05edb55/> increase pack-receive performance using a BufWriter
   Previously the NamedTempFile would receive every little write request
   for millions of objects, consuming considerable amounts of time.
   
   Now a buf writer alleviates this issue entirely.
 - <csr-id-0b6ed60f842f0a36f61f187651080540a358758e/> `bundle::write::Error` is now publicly available

### Bug Fixes (BREAKING)

 - <csr-id-5a75afe0467e4a84323ea10172eed835cc7fae4c/> Reading and writing empty packs is explicitly allowed.
   This can happen when sending packs that don't actually contain changes,
   but need to be sent to conform to the protocol.

### Other (BREAKING)

 - <csr-id-b46347fd3d50886eeca500e31e1e12b354711309/> `index::write::Outcome::index_kind` -> `::index_version`.
 - <csr-id-591afd56d9862a6348ef8b3af61798004b36aa19/> `bundle::write::Options::index_kind` -> `::index_version`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - prefix created pack files with `pack-` ([`e489b10`](https://github.com/Byron/gitoxide/commit/e489b10878c85289e0a9869804abee2418de6989))
    - increase pack-receive performance using a BufWriter ([`a745512`](https://github.com/Byron/gitoxide/commit/a745512185fb0a46e35daaa6d28829aec05edb55))
    - `index::write::Outcome::index_kind` -> `::index_version`. ([`b46347f`](https://github.com/Byron/gitoxide/commit/b46347fd3d50886eeca500e31e1e12b354711309))
    - add test to show that empty packs won't be written as expected behaviour. ([`72ce7fd`](https://github.com/Byron/gitoxide/commit/72ce7fdced10b8359e74daea3bb35ab73b29e7c0))
    - `bundle::write::Error` is now publicly available ([`0b6ed60`](https://github.com/Byron/gitoxide/commit/0b6ed60f842f0a36f61f187651080540a358758e))
    - `bundle::write::Options::index_kind` -> `::index_version`. ([`591afd5`](https://github.com/Byron/gitoxide/commit/591afd56d9862a6348ef8b3af61798004b36aa19))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-smart-release' ([`aa80b60`](https://github.com/Byron/gitoxide/commit/aa80b606e5570f327660cca42ea81581a6e9d5e3))
    - make fmt ([`7b9c065`](https://github.com/Byron/gitoxide/commit/7b9c06547b75929e3e5bf4240f43c7e9bc7d54e0))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'main' into clone ([`acb0738`](https://github.com/Byron/gitoxide/commit/acb07382a9306d6962bea60e8977d83d021743f4))
    - Merge branch 'delta-tree-parallelization' ([`cca2ad5`](https://github.com/Byron/gitoxide/commit/cca2ad5ee9483d7da968658e0a4d610dbc4ad4d6))
    - Don't enforce Send bounds in serial version of `in_parallel_with_slice()` ([`dda661e`](https://github.com/Byron/gitoxide/commit/dda661e1b7cc0ace6cd9504233f20980e1e52387))
    - thanks clippy ([`583f2ca`](https://github.com/Byron/gitoxide/commit/583f2cacbdaa79149b3741c1f08cabcc73a1a03f))
    - more efficient distribution of tasks to threads during pack resolution. ([`6836cb1`](https://github.com/Byron/gitoxide/commit/6836cb148cbadf919bdac2e0e77bdec34caa9307))
    - refactor ([`0e2ca47`](https://github.com/Byron/gitoxide/commit/0e2ca47fab99fec72cf8c8a327f5c3627a916add))
    - Try to use counters directly in delta-tree resolution ([`c6ded26`](https://github.com/Byron/gitoxide/commit/c6ded26aa2513fefda0c19784a70ce5b046910f1))
    - reduce memory usage for pack resolution by 18%… ([`ec58bef`](https://github.com/Byron/gitoxide/commit/ec58bef84d620360dc52e34e173ace1310f74c85))
    - Merge branch 'reduce-pack-resolve-mem-consumption' ([`a8ced57`](https://github.com/Byron/gitoxide/commit/a8ced575ef935b9a2078d6f2b15aff746a862317))
    - minor refactor after trying more memory foot-print related changes ([`5852334`](https://github.com/Byron/gitoxide/commit/58523341d1203ff55144a981d7dd3b276bd47b31))
    - simplify size-related tests ([`fb2b2a8`](https://github.com/Byron/gitoxide/commit/fb2b2a88193247d55f74c3e39cd8a12f3862b6d6))
    - cut the size of bytes needed to store children in half for a lower footprint ([`38c96f2`](https://github.com/Byron/gitoxide/commit/38c96f2ab1fc2c3a91958e2b4ab213574ec9e8f0))
    - Merge branch 'crates-index-diff-fixes' ([`b09bad7`](https://github.com/Byron/gitoxide/commit/b09bad7b5702838c1119d052b322d90c9b5968bb))
    - thanks clippy ([`851bc79`](https://github.com/Byron/gitoxide/commit/851bc798a76160701387d25119b101aa8014c93f))
    - Allow verification of empty packs and indices. ([`49d168a`](https://github.com/Byron/gitoxide/commit/49d168a8859c6b5a9e7ef58cd836093212c2c7ad))
    - refactor ([`38a7452`](https://github.com/Byron/gitoxide/commit/38a74522b5c1ca5c255b8b132c25b90fc7fa85f9))
    - Reading and writing empty packs is explicitly allowed. ([`5a75afe`](https://github.com/Byron/gitoxide/commit/5a75afe0467e4a84323ea10172eed835cc7fae4c))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.23.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 22 calendar days.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.22.0 (2022-08-28)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - adjust to changes in `git-diff` ([`54954ee`](https://github.com/Byron/gitoxide/commit/54954ee5022a900f9f97baec63e9a073eca514e9))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.21.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.21.1 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.21.0 (2022-07-22)

This is a maintenance release with no functional changes.

### New Features (BREAKING)

 - <csr-id-95210cb2ba85f75148b4ef48ccea9d9f8a0a0114/> Provide optional `candidates` for ambigious entries during `lookup_prefix()`
   The candidate entries are all entries matching a given prefix.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - refactor ([`c28404b`](https://github.com/Byron/gitoxide/commit/c28404b3656ba6714db6898ce8151c0bcf4448e0))
    - Assure index ambiguous object range can represent 'none found' ([`5ffe54f`](https://github.com/Byron/gitoxide/commit/5ffe54ff88f026139474658fb470742751126119))
    - Avoid allocating index entries in case of ambiguity by using a range ([`4db4754`](https://github.com/Byron/gitoxide/commit/4db47547fa405542efd38b475e3e430548b9d160))
    - fix build ([`8a63076`](https://github.com/Byron/gitoxide/commit/8a6307617580f72457d3f008e4fe40ae7dcfd360))
    - more efficient and clear addition of entries to candidates ([`a76a0e1`](https://github.com/Byron/gitoxide/commit/a76a0e161ec289d289e5683e541cff45d393756d))
    - properly order ambiguous entries, ascending ([`0340896`](https://github.com/Byron/gitoxide/commit/0340896b49fe4f40a9dba1c947ec9a2050d8f9b8))
    - Provide optional `candidates` for ambigious entries during `lookup_prefix()` ([`95210cb`](https://github.com/Byron/gitoxide/commit/95210cb2ba85f75148b4ef48ccea9d9f8a0a0114))
    - refactor ([`a2f9fa4`](https://github.com/Byron/gitoxide/commit/a2f9fa431a820093cb34158e71cb30dc21db1151))
 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.20.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 18 calendar days.
 - 22 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
</details>

## 0.19.1 (2022-05-21)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#422](https://github.com/Byron/gitoxide/issues/422)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#422](https://github.com/Byron/gitoxide/issues/422)**
    - prepare changelog ([`de2d587`](https://github.com/Byron/gitoxide/commit/de2d5874b8d75c53165a9fc3ed35e2b37142bf52))
 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - thanks clippy ([`409a95b`](https://github.com/Byron/gitoxide/commit/409a95b6505db8568bfea24bc62c92640a5c3cbf))
    - Merge branch 'davidkna-discover-ceiling' ([`66944ba`](https://github.com/Byron/gitoxide/commit/66944ba986114ece2d3b31440c721d0e84b4f267))
    - thanks clippy ([`4979d20`](https://github.com/Byron/gitoxide/commit/4979d2071cbd1a98f3d81aacd60dd99f07d3f746))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.19.0 (2022-05-18)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 40 calendar days.
 - 43 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#384](https://github.com/Byron/gitoxide/issues/384)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - prevent line-ending conversions for shell scripts on windows ([`96bb4d4`](https://github.com/Byron/gitoxide/commit/96bb4d460db420e18dfd0f925109c740e971820d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - cleanup ([`199583a`](https://github.com/Byron/gitoxide/commit/199583abbec1ff58204c3cf90dd8be88730fc112))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - fix clippy - many false positives this time ([`045e6fa`](https://github.com/Byron/gitoxide/commit/045e6fae17077555c3e115992905c8046f2c5d0b))
    - fix clippy - many false positives this time ([`099bd5b`](https://github.com/Byron/gitoxide/commit/099bd5b86fb80b26a73863b80ad60a0394458b6d))
</details>

## 0.18.0 (2022-04-05)

### Changed (BREAKING)

 - <csr-id-8c5ae77f06a64c57df9a9ad1190266896a223dbe/> Remove deprecated compound and linked object databases
   The dynamic/general store is the only maintained can-do-it-all
   DB now.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - restrict signature changes to 'Ancestores::sorting()` ([`d71bd9d`](https://github.com/Byron/gitoxide/commit/d71bd9ded1e5e5a61a27be3d55f4b85ee4049bcf))
    - Adjust to changes in git-traverse ([`8240622`](https://github.com/Byron/gitoxide/commit/824062215865e6ec12afeb2d51b3c63f15291244))
 * **Uncategorized**
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - Remove deprecated compound and linked object databases ([`8c5ae77`](https://github.com/Byron/gitoxide/commit/8c5ae77f06a64c57df9a9ad1190266896a223dbe))
</details>

## 0.17.0 (2022-04-03)

<csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/>
<csr-id-9b9f10ad862b5e097c836c51df1eb98607df5ae1/>

### Chore

 - <csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/> upgrade dashmap to 5.1.0 (with security fix)

### New Features

 - <csr-id-503b1a1f8d4f39b44c166209d7a8ba8d74137859/> `index::File::lookup_prefix(…)`
 - <csr-id-cb83beedd1aa389f6774e2296f79273e8c8f14f4/> git-hash::Prefix::from_id()
   A way to obtain a prefix of an object id, with all non-prefix
   bytes set to zero.
 - <csr-id-16208306ab49ade30d8ffd6b067ebd8eefd84cd4/> in-manifest and in-lib documentation of feature toggles

### Bug Fixes

 - <csr-id-42e0487286c1f745837c0ce337ed7c9d86b14516/> support Rust 1.52

### Refactor

 - <csr-id-9b9f10ad862b5e097c836c51df1eb98607df5ae1/> remove unnecessary unsafe by using `chunks_mut()`
   This was probably a left-over from times where there was a static
   requirement on the chunks processing. Maybe… .

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 36 commits contributed to the release over the course of 55 calendar days.
 - 60 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364), [#67](https://github.com/Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - `index::File::lookup_prefix(…)` ([`503b1a1`](https://github.com/Byron/gitoxide/commit/503b1a1f8d4f39b44c166209d7a8ba8d74137859))
    - support MSRV ([`d09fd9b`](https://github.com/Byron/gitoxide/commit/d09fd9b37557f2dc199e8a4651c56b3b63423136))
    - add documentation for lookup_prefix along with missing test ([`927b2ac`](https://github.com/Byron/gitoxide/commit/927b2ace875cdda63ce312eb7ad5329f2159608d))
    - lookup_prefix() seems to work now ([`b558f11`](https://github.com/Byron/gitoxide/commit/b558f111520381e25a9500d3b2401fdd337db6f6))
    - A stab at implementing lookup_prefix - to no avail ([`69cb6d1`](https://github.com/Byron/gitoxide/commit/69cb6d1dd6b8df74fee1ead1ce15bcf0b51d7232))
    - refactor ([`cff6f9f`](https://github.com/Byron/gitoxide/commit/cff6f9fc90e58c409e367912d0b38860fae9a205))
    - refactor ([`5bc548e`](https://github.com/Byron/gitoxide/commit/5bc548ed500045491012ab0a93bcbe13e78b0dc8))
    - Prefix now validates all constraints and errors on violation ([`75efa79`](https://github.com/Byron/gitoxide/commit/75efa79f62efc29b343d2d2f53eaf001eef176df))
    - git-hash::Prefix::from_id() ([`cb83bee`](https://github.com/Byron/gitoxide/commit/cb83beedd1aa389f6774e2296f79273e8c8f14f4))
    - Sketch for abbreviated method lookup ([`467453a`](https://github.com/Byron/gitoxide/commit/467453a7e625a3bc8e3e381ce50f24f1be8ba605))
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
    - upgrade parking_lot and cargo_toml ([`f95c1a0`](https://github.com/Byron/gitoxide/commit/f95c1a0d9c19bcc6feb9b8739a09d86f9970a0e0))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Salvage an alternative parallelization approach which might be good for index-creation ([`7e76796`](https://github.com/Byron/gitoxide/commit/7e76796d5c2956961bd998286bec05fca1ba8fc4))
    - refactor ([`f86eacc`](https://github.com/Byron/gitoxide/commit/f86eacc5cfaf6d88ead4f8dbd65989d32674c213))
    - switch index checkout to chunk-based operation ([`e5f6943`](https://github.com/Byron/gitoxide/commit/e5f69433e4a6cc7866b666e0baccfa32efb92a7f))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-lib documentation of feature toggles ([`1620830`](https://github.com/Byron/gitoxide/commit/16208306ab49ade30d8ffd6b067ebd8eefd84cd4))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Adapt to changes in git_features::path to deal with Result ([`bba4c68`](https://github.com/Byron/gitoxide/commit/bba4c680c627a418efbd25f14bd168df19b8dedd))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
    - gitoxide-core without os-str-bytes ([`909aa14`](https://github.com/Byron/gitoxide/commit/909aa1402c82c3128052023613a297b213716e3d))
    - Remove os_str_bytes from git-pack ([`86f6e50`](https://github.com/Byron/gitoxide/commit/86f6e5054ea11b7aeb9c85321913de090f71e3a1))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - add some precaution to avoid strange interactions with packs ([`b052a9a`](https://github.com/Byron/gitoxide/commit/b052a9a3e9127fd9a4029594ea9de6e436db03c6))
    - fix build ([`9c8e449`](https://github.com/Byron/gitoxide/commit/9c8e449e928b3190e5845606f79b12c529dede55))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - Use an even faster way of counting ([`3877920`](https://github.com/Byron/gitoxide/commit/387792085542ebc8277ac0dcaf9e3dc3b522a69a))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - remove unnecessary unsafe by using `chunks_mut()` ([`9b9f10a`](https://github.com/Byron/gitoxide/commit/9b9f10ad862b5e097c836c51df1eb98607df5ae1))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - thanks clippy ([`48be1ee`](https://github.com/Byron/gitoxide/commit/48be1ee666a88f1416896c5e8073d4d86dae7b8c))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - upgrade dashmap to 5.1.0 (with security fix) ([`2520945`](https://github.com/Byron/gitoxide/commit/25209454d3f7e27e12e8ddca92e43b1ff01d58aa))
</details>

## 0.16.1 (2022-02-01)

### Bug Fixes

 - <csr-id-d9451e8d7fc39c252042f9d2447061262c16ae7a/> downgrade dashmap to 4.0 to avoid unsoundness.
   See https://github.com/xacrimon/dashmap/issues/167 for tracking
   progress on resolving the issue.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.16.1 ([`d4a8f9f`](https://github.com/Byron/gitoxide/commit/d4a8f9f73bb829bcc83fa68b6b5a7495fcba6b19))
    - Release git-object v0.17.1, git-pack v0.16.1 ([`e959af8`](https://github.com/Byron/gitoxide/commit/e959af83fa92e8ed87edae6e2d1c6a797964c056))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - update changelogs prior to git-pack release ([`b7e3a4a`](https://github.com/Byron/gitoxide/commit/b7e3a4afdd6417a38aadad35c7f584617e7b47fa))
    - downgrade dashmap to 4.0 to avoid unsoundness. ([`d9451e8`](https://github.com/Byron/gitoxide/commit/d9451e8d7fc39c252042f9d2447061262c16ae7a))
</details>

## 0.16.0 (2022-01-23)

<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-e0b8636f96e4bfe1bc72b5aa6ad4c4c8538ff92c/>
<csr-id-c800fdd331e6d7a0b8d756ba822915259f26e9e8/>
<csr-id-e6ff1a885889cf88f6b34b1193aa03d8bce16af5/>
<csr-id-f48630ba8f745c2ec61a1e3c51fa63a1789a088c/>

### Refactor

 - <csr-id-e0b8636f96e4bfe1bc72b5aa6ad4c4c8538ff92c/> replace bare u32 `data::Id` typedef

### Other

 - <csr-id-e6ff1a885889cf88f6b34b1193aa03d8bce16af5/> :File uses its hash_len parameter
 - <csr-id-f48630ba8f745c2ec61a1e3c51fa63a1789a088c/> :Find implementation for Rc

### Chore

 - <csr-id-c800fdd331e6d7a0b8d756ba822915259f26e9e8/> remove unused dependencies

### New Features

 - <csr-id-b80dec2323b81fb2172df76c7d897a4b5e6bdfea/> zero-objects check for index and multi-index integrity validation
 - <csr-id-56fc99fb9c1cab61abd03c10e1b4af0d6e491bbf/> support for fan-checking in index and multi-index integrity verification
 - <csr-id-28e3ea8612112f6a04cfaff591565eca5a1ffba2/> introduce type for entry indices within an index or multi-index
   That way it's a littl emore descriptive than a bare u32.
 - <csr-id-58c2edb76755ab71e10eef4cd9a51533825c291f/> git_pack::Find::try_find_cached(…, pack_cache)
   With this method it's easier to bypass local caches and control
   the cache oneself entirely.
 - <csr-id-e25f4eadec679406aad6df10026e27e4832c2482/> A simplified version of the `Find` trait
   It's meant for the next generation of object db handles which keep a
   local cache of all the details of the actual object database.

### Bug Fixes

 - <csr-id-42e0487286c1f745837c0ce337ed7c9d86b14516/> support Rust 1.52
 - <csr-id-84ade1d23060f10bf6c8529f8f693d06660b4f4e/> Allow resolution of in-pack ref-deltas
   This finally allows delta tree caches to be used on typical small packs
   returned by GitHub.
 - <csr-id-ba92cc09ba41fe4c9a9097bfeb8d18016408fcdf/> don't try to short-cut detection of large offsets when writing index files
   The code incorrectly assumed that the input is sorted by offsets, with
   the largest offset being last, even though by all means that's not the
   case.
 - <csr-id-6d3f52dc13d7243a6bce6dab89a985114a75d94b/> Avoid the dashmap being cloned for each thread
   Intead, share it by reference, it's sync after all.
   
   This issue was introduced when switching to a `Send + Clone` model,
   instead of `Send + Sync`, to allow thread-local caches in database
   handles of all kinds.
 - <csr-id-b605c1fa0494b10872d3c2e6ecce0e39f1a90a9e/> linked::Store now assures unique IDs across compound stores

### Changed (BREAKING)

 - <csr-id-a79a7fb638b45df88af0d0d5fc9ada6d824bc328/> Improve method signatures of `cache::Tree::*`
 - <csr-id-91d047658b114f372735116c9d8e6962a3873137/> cleanup and unify `verify_integrity()` method signature
   Previously they used many different ways of handling their parameters
   despite all boiling down to calling the same 'index::File::traverse()`
   method.
   
   This allows for more reuse of `Options` structs and generally makes
   clearer how these optinos are used.
 - <csr-id-2cf7727228e1d8094ffd2eec6746006348c39eab/> `index::File::traverse()` now returns an `Outcome` struct instead of tuple of 3 fields
 - <csr-id-bf04644ab75ed1969507f957dc8d4868790d462d/> remove `Option<impl Progress>` in favor of `impl Progress`
 - <csr-id-6829e5e5d6aed1e6c87647144e2dd76a1e4b9f1f/> multi-index integrity check; use `integrity::Outcome` for various integrity checks
 - <csr-id-d851bede97801096d188ff6af06c98a79fe276db/> remove unnecessary `Arc` around `should_interrupt` flag
 - <csr-id-c2679a03358b9c19d63ed1af1cd57324c6381447/> remove Sha1 mentions in `index::verify::Mode::*` variants
   The hash is repository defined and not hard-coded
 - <csr-id-80b120d3278e46429f848df7af3db13413c36649/> introduce `index::File::verify_integrity(…, pack: Option<PackContext>, …)`, replacing tuple
   This allows for more documentation on what input is required there and
   generally makes for an easier to use API.
 - <csr-id-79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded/> rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()`
   This change was done in the name of consistency, as `from_bytes()` is
   used in many other git-* crates
 - <csr-id-2ef9a8424af51310db8c1e6df31dde9953ed3d21/> Change accessors named `hash_kind()` to `object_hash()` for consistency
 - <csr-id-b76f6be6c5baa6cf613a174241f007e92bf5ba36/> consistently use `object_hash` instead of `hash_kind`
 - <csr-id-629412b4cb192614b7eff08dbf203e3448c902c1/> data::Entry::from_read() now takes a hash lengths as parameter
   That way ref-deltas can be interepreted without hard-coding SHA1
 - <csr-id-851dc2c52fa8e204ba2d5ced8fb0959a889869d8/> data::Entry::from_bytes(…, hash_len) takes new parameter
   The hash-len tells it how to interpret ref-delta objects, which
   store the complete hash of the base object.
   
   This is now entirely configurable.
 - <csr-id-db8c8c41b4ced0fc296d3877883d801e77d550ae/> `index::File::at()` with git_hash::Kind parameter
   It will allow to assume different hashes even in the index file format
   which isn't yet capable of storing this information.
 - <csr-id-e6a3c9f72332b524b143bc94ee9df0a6db11e864/> `data::File::at()` and `Bundle::at()` now have `hash_kind` parameter
   It's used to configure the kind of hash to assume when reading packs and
   indices.
 - <csr-id-3f05fea55dc8acce1ed62ecbe4e0a1394f2720b7/> remove `make_object_cache` parameter from `git_pack::data::output::count::objects()`
   It now is an implementation detail of the Find trait.
 - <csr-id-82b9b33bd5f4c3c1721a5093de2cedc62cb10565/> move `bundle::Location` to `data::entry::Location`
   The latter place best describes its purpose.
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 148 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 33 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#260](https://github.com/Byron/gitoxide/issues/260), [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293), [#67](https://github.com/Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#260](https://github.com/Byron/gitoxide/issues/260)**
    - linked::Store now assures unique IDs across compound stores ([`b605c1f`](https://github.com/Byron/gitoxide/commit/b605c1fa0494b10872d3c2e6ecce0e39f1a90a9e))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - remove unused dependencies ([`c800fdd`](https://github.com/Byron/gitoxide/commit/c800fdd331e6d7a0b8d756ba822915259f26e9e8))
    - upgrade dashmap to latest version ([`52d4fe5`](https://github.com/Byron/gitoxide/commit/52d4fe55b6dd88f72479abd4015cab063ddaaf97))
    - refactor ([`b88f253`](https://github.com/Byron/gitoxide/commit/b88f253e46e7ad0a50b670b96c1bfa09eaaecaef))
    - refactor ([`52a4dcd`](https://github.com/Byron/gitoxide/commit/52a4dcd3a6969fa8f423ab39c875f98f9d210e95))
    - Make single-threaded programs possible to use with git-repository ([`dde5c6b`](https://github.com/Byron/gitoxide/commit/dde5c6ba76ff849f69f742c985b4bc65ca830883))
    - Use new odb in place of the old one and it works ([`8ad25c5`](https://github.com/Byron/gitoxide/commit/8ad25c581bc79041545a72baf57b0a469d99cc30))
    - Make find::Entry self-contained ([`ad36fb9`](https://github.com/Byron/gitoxide/commit/ad36fb9b800c17931ce358ac262bef40d43dcfb3))
    - Remove iterator access in favor of fully owned data ([`62d3f10`](https://github.com/Byron/gitoxide/commit/62d3f106437e597a41aae592da28f48e8736b143))
    - Remove CRC32 check entirely as it doesn't seem to be important in the big picture ([`22d35bd`](https://github.com/Byron/gitoxide/commit/22d35bdbc271ccada8d68a1450d9a2533fc739ee))
    - Notes about multi-pack indices in the current data::entry::location ([`7eff6bf`](https://github.com/Byron/gitoxide/commit/7eff6bf525ea48fa913149911ea4c8fe742a25a3))
    - move `bundle::Location` to `data::entry::Location` ([`82b9b33`](https://github.com/Byron/gitoxide/commit/82b9b33bd5f4c3c1721a5093de2cedc62cb10565))
    - Use existing git_features facilities ([`ed0c266`](https://github.com/Byron/gitoxide/commit/ed0c2662d95b74b4abc09b42fc24cb56219dd511))
    - Adjust pack-create to changes in git-pack ([`12db899`](https://github.com/Byron/gitoxide/commit/12db899a72da6decccd82931637d074059b578f5))
    - remove `make_object_cache` parameter from `git_pack::data::output::count::objects()` ([`3f05fea`](https://github.com/Byron/gitoxide/commit/3f05fea55dc8acce1ed62ecbe4e0a1394f2720b7))
    - :Find implementation for Rc ([`f48630b`](https://github.com/Byron/gitoxide/commit/f48630ba8f745c2ec61a1e3c51fa63a1789a088c))
    - MultiPackIndex compatible pack::Find trait definition ([`5fa1a9d`](https://github.com/Byron/gitoxide/commit/5fa1a9dce59c2654374a532d024c8de5959d4d0f))
    - git_pack::Find::try_find_cached(…, pack_cache) ([`58c2edb`](https://github.com/Byron/gitoxide/commit/58c2edb76755ab71e10eef4cd9a51533825c291f))
    - refactor ([`3310d8f`](https://github.com/Byron/gitoxide/commit/3310d8f271f74fc6084e33dd9bd4c5f01b54e432))
    - remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
    - fix docs ([`1bb4253`](https://github.com/Byron/gitoxide/commit/1bb425347e4b502e1c048908cd5f3641d2b16896))
    - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
    - A simplified version of the `Find` trait ([`e25f4ea`](https://github.com/Byron/gitoxide/commit/e25f4eadec679406aad6df10026e27e4832c2482))
    - Add 'contains()' method to Find ([`dfdd6fb`](https://github.com/Byron/gitoxide/commit/dfdd6fb2c83e5d09c3a56936723bc6749ac4b99a))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Add a less thorough and faster way of verifying multi-indices ([`7517482`](https://github.com/Byron/gitoxide/commit/75174825e1012cfb4c34c18391c681b49c2f0d29))
    - refactor ([`91e6d38`](https://github.com/Byron/gitoxide/commit/91e6d382bb2e2430d5d3325a390b7d9bdc0034d6))
    - Allow interrupting multi-index creation more often ([`f223ecb`](https://github.com/Byron/gitoxide/commit/f223ecb6c69358ed8e38d796aca9bef21173cc92))
    - also test pack-creation with multi-index repo ([`235a27a`](https://github.com/Byron/gitoxide/commit/235a27a925e9b5f6729056ac44e8107dcba55cfd))
    - better multi-pack verification progress ([`2e16f13`](https://github.com/Byron/gitoxide/commit/2e16f1321bdccc2cef688d27efd9cc9be1360c31))
    - Handle large multi-pack indices correctly ([`4f6b030`](https://github.com/Byron/gitoxide/commit/4f6b0308f06b7705163ff624a98694e1d928fee1))
    - Fix progress and handling of large of multi-pack index offsets ([`5dc1f81`](https://github.com/Byron/gitoxide/commit/5dc1f813ead64ad13edb2b5ed9bd660d198c7ddb))
    - add missing docs ([`4137327`](https://github.com/Byron/gitoxide/commit/41373274fc7f23e3fed17dc52e3e3e94c2e9e41a))
    - write progress for multi-pack writing ([`1bea1d4`](https://github.com/Byron/gitoxide/commit/1bea1d47908d3ec44c83b2e39a5b67134ad51ee0))
    - adapt to changes in git-features ([`542c0df`](https://github.com/Byron/gitoxide/commit/542c0df9f7498a53a4561e4286b8fdb888565cd3))
    - progress for chunk writing ([`50fde01`](https://github.com/Byron/gitoxide/commit/50fde01b44a0a720ccb874bc23a818334238c6e0))
    - multi-pack index writing complete with large-offset support ([`f7d5c7f`](https://github.com/Byron/gitoxide/commit/f7d5c7f815dbf52c668444b316ae2e1485463bcb))
    - write pack-ids and offsets ([`bfc8069`](https://github.com/Byron/gitoxide/commit/bfc8069e6da2ec6d87fa40bbaaca247c1e247d5f))
    - Add chunk for oids ([`565a7ae`](https://github.com/Byron/gitoxide/commit/565a7aea9341a0f0005a41bc6687fbaacb0c0b97))
    - Write the fanout table ([`6a68ed7`](https://github.com/Byron/gitoxide/commit/6a68ed7708bdbb29c40bcea0dc7cf681c0aff75b))
    - refactor ([`93dc660`](https://github.com/Byron/gitoxide/commit/93dc660aa34c18b5186c57c6a3fad547a63d5eec))
    - Write multi-index header along with path-names chunk ([`2fc6751`](https://github.com/Byron/gitoxide/commit/2fc67512f8be2860ab06dc5a282f4f6550c3fddb))
    - Sketch all the chunk-write API and use it from multi-index write ([`5457761`](https://github.com/Byron/gitoxide/commit/545776180f75cba87f7119f9bd862d39f081f1bd))
    - Add frame for writing a multi-pack index ([`9ce1e7f`](https://github.com/Byron/gitoxide/commit/9ce1e7f2d8c7133590f571919850eaa763f789e3))
    - `index::File::traverse()` now returns an `Outcome` struct instead of tuple of 3 fields ([`2cf7727`](https://github.com/Byron/gitoxide/commit/2cf7727228e1d8094ffd2eec6746006348c39eab))
    - refactor ([`c361ee3`](https://github.com/Byron/gitoxide/commit/c361ee399e9c435b087387c1542b3838c21fad03))
    - multi-index verification now matches that of git itself ([`3a76a28`](https://github.com/Byron/gitoxide/commit/3a76a28e6af11950e8a808d09c36c2ee8b655944))
    - zero-objects check for index and multi-index integrity validation ([`b80dec2`](https://github.com/Byron/gitoxide/commit/b80dec2323b81fb2172df76c7d897a4b5e6bdfea))
    - support for fan-checking in index and multi-index integrity verification ([`56fc99f`](https://github.com/Byron/gitoxide/commit/56fc99fb9c1cab61abd03c10e1b4af0d6e491bbf))
    - More detailed multi-index verification ([`8f9a55b`](https://github.com/Byron/gitoxide/commit/8f9a55bb31af32b266d7c53426bc925361a627b2))
    - even nicer printing ([`d2bea27`](https://github.com/Byron/gitoxide/commit/d2bea270787597d6aef48ffe023ff49969c33bd9))
    - nicer printing of index verification results ([`e3dfa12`](https://github.com/Byron/gitoxide/commit/e3dfa123b368e66f39567bd2a8f5d7d9c09d4fe6))
    - very first experimental support for multi-pack index verification ([`bb35c69`](https://github.com/Byron/gitoxide/commit/bb35c6994765ec3bbbcfde247911d1ffe711a23d))
    - remove `Option<impl Progress>` in favor of `impl Progress` ([`bf04644`](https://github.com/Byron/gitoxide/commit/bf04644ab75ed1969507f957dc8d4868790d462d))
    - multi-index integrity check; use `integrity::Outcome` for various integrity checks ([`6829e5e`](https://github.com/Byron/gitoxide/commit/6829e5e5d6aed1e6c87647144e2dd76a1e4b9f1f))
    - remove unnecessary `Arc` around `should_interrupt` flag ([`d851bed`](https://github.com/Byron/gitoxide/commit/d851bede97801096d188ff6af06c98a79fe276db))
    - remove Sha1 mentions in `index::verify::Mode::*` variants ([`c2679a0`](https://github.com/Byron/gitoxide/commit/c2679a03358b9c19d63ed1af1cd57324c6381447))
    - introduce `index::File::verify_integrity(…, pack: Option<PackContext>, …)`, replacing tuple ([`80b120d`](https://github.com/Byron/gitoxide/commit/80b120d3278e46429f848df7af3db13413c36649))
    - multi-index verify checksum ([`853d468`](https://github.com/Byron/gitoxide/commit/853d4683aae5f4dd4667b452932bd57f99f6afab))
    - fix docs ([`ce044ef`](https://github.com/Byron/gitoxide/commit/ce044ef146e3d67483bed382f5dd5c484699534e))
    - introduce type for entry indices within an index or multi-index ([`28e3ea8`](https://github.com/Byron/gitoxide/commit/28e3ea8612112f6a04cfaff591565eca5a1ffba2))
    - replace bare u32 `data::Id` typedef ([`e0b8636`](https://github.com/Byron/gitoxide/commit/e0b8636f96e4bfe1bc72b5aa6ad4c4c8538ff92c))
    - Adjust to changes in git-odb ([`710780c`](https://github.com/Byron/gitoxide/commit/710780cd355793ea638767213f250e026997a530))
    - Add remaining docs for multi-index ([`10a24c1`](https://github.com/Byron/gitoxide/commit/10a24c1860e63935b435e985900797b2d4c707a8))
    - docs for multi_index::chunk ([`73fbc91`](https://github.com/Byron/gitoxide/commit/73fbc915847b7c458a17bdfbb7fa1de3f31ab437))
    - refactor ([`eafdff4`](https://github.com/Byron/gitoxide/commit/eafdff405b3f408aa5203f40c7f0a570ce20655d))
    - multi-index iteration ([`1c99903`](https://github.com/Byron/gitoxide/commit/1c999035cc3649ab9db02bd82644fb54c408f6d2))
    - Access pack-indices and pack-offsets of multi-pack indices ([`c2a6918`](https://github.com/Byron/gitoxide/commit/c2a69189f88c53ab555158245ce647fcd33fca6a))
    - oid lookup for multi-pack indices ([`254f618`](https://github.com/Byron/gitoxide/commit/254f618ee410be4a2787f599529a6cca1284a0fb))
    - Add basic oid by multi-index file index ([`a54f552`](https://github.com/Byron/gitoxide/commit/a54f552741aed315b21112576d6e5b704a9439d4))
    - rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()` ([`79dc0d5`](https://github.com/Byron/gitoxide/commit/79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded))
    - Change accessors named `hash_kind()` to `object_hash()` for consistency ([`2ef9a84`](https://github.com/Byron/gitoxide/commit/2ef9a8424af51310db8c1e6df31dde9953ed3d21))
    - Adapt to changes in git-hash ([`754a663`](https://github.com/Byron/gitoxide/commit/754a66344ff2cfcfc4a7a3d72f1240e939c48055))
    - Remove unnecessary `Default` implementation for user of Tree::traverse ([`9da20e9`](https://github.com/Byron/gitoxide/commit/9da20e92c96e4ce8dd75e141c24143e4ea1141a7))
    - remove unnecessary Default bound for data in Tree nodes ([`d548f72`](https://github.com/Byron/gitoxide/commit/d548f726013df409b0e1a5fb0e39c15ff445228d))
    - adjust to changes in git-hash ([`9bf25cc`](https://github.com/Byron/gitoxide/commit/9bf25cc4f2e44821f93e85997677bc4e86a67bd4))
    - consistently use `object_hash` instead of `hash_kind` ([`b76f6be`](https://github.com/Byron/gitoxide/commit/b76f6be6c5baa6cf613a174241f007e92bf5ba36))
    - adjust to changes in git-hash ([`ca35246`](https://github.com/Byron/gitoxide/commit/ca35246a91888ae41805d71082055c98d2ff7f0b))
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
    - data::Entry::from_read() now takes a hash lengths as parameter ([`629412b`](https://github.com/Byron/gitoxide/commit/629412b4cb192614b7eff08dbf203e3448c902c1))
    - data::Entry::from_bytes(…, hash_len) takes new parameter ([`851dc2c`](https://github.com/Byron/gitoxide/commit/851dc2c52fa8e204ba2d5ced8fb0959a889869d8))
    - refactor ([`7331e99`](https://github.com/Byron/gitoxide/commit/7331e99cb88df19f7b1e04b1468584e9c7c79913))
    - adjust to changes in git-hash ([`07aa1bc`](https://github.com/Byron/gitoxide/commit/07aa1bca225c30b168a597f920bda392b2cb2713))
    - :File uses its hash_len parameter ([`e6ff1a8`](https://github.com/Byron/gitoxide/commit/e6ff1a885889cf88f6b34b1193aa03d8bce16af5))
    - `index::File::at()` with git_hash::Kind parameter ([`db8c8c4`](https://github.com/Byron/gitoxide/commit/db8c8c41b4ced0fc296d3877883d801e77d550ae))
    - `data::File::at()` and `Bundle::at()` now have `hash_kind` parameter ([`e6a3c9f`](https://github.com/Byron/gitoxide/commit/e6a3c9f72332b524b143bc94ee9df0a6db11e864))
    - remove unnecessary dev-depednency ([`b71ea6a`](https://github.com/Byron/gitoxide/commit/b71ea6a89d11d6cac01b7d9e9b1101f4d637617c))
    - adapt to changes in git-hash ([`82fec95`](https://github.com/Byron/gitoxide/commit/82fec95e9ed4b924849bfcc84b5b2691a925a5b3))
    - Calculate trailer offset instead of storing it ([`bf62067`](https://github.com/Byron/gitoxide/commit/bf62067c690e407e2ace66220337359542e1846a))
    - Make pessimistic size-estimation instead of an optimistic one ([`69f1d2a`](https://github.com/Byron/gitoxide/commit/69f1d2a2063cfebae3ea70979d950f8ab7751eac))
    - refactor ([`8b8b4c5`](https://github.com/Byron/gitoxide/commit/8b8b4c538823fb4d2c37be80340d843080f08d19))
    - refactor ([`8c9c7fc`](https://github.com/Byron/gitoxide/commit/8c9c7fc3bc46afa9c8567a8bc8079cac12ed8422))
    - Adapt to changes in git-chunk ([`44ea5c3`](https://github.com/Byron/gitoxide/commit/44ea5c3c334399bc03d92fa20171d2c0c3afdf49))
    - refactor ([`ac46765`](https://github.com/Byron/gitoxide/commit/ac4676534573e3ccfa219765e645526797c6d71b))
    - Adapt to latest changes to git-chunk ([`743d696`](https://github.com/Byron/gitoxide/commit/743d6967d6236a4bb6a9c8817f957e7604bc9264))
    - Provide multi-index checksum ([`a363de9`](https://github.com/Byron/gitoxide/commit/a363de9b8271986385b1d57e61a6c103c20a4055))
    - update changelog prior to release ([`6ae49e3`](https://github.com/Byron/gitoxide/commit/6ae49e39b2251ad70b72a8f3b3840ebb9334ffd9))
    - completely validate and parse multi-index file ([`e7e40c3`](https://github.com/Byron/gitoxide/commit/e7e40c30dea082d004e8781ef7d36bde0afdd8a7))
    - read and validate index names contained in the multi-pack index ([`24a9790`](https://github.com/Byron/gitoxide/commit/24a979036df515f0616738825e669ec9c8dab1f1))
    - read and validate fanout chunk ([`3ca04e3`](https://github.com/Byron/gitoxide/commit/3ca04e355a413975e55adf8b204d6962a9341d32))
    - Read all mandatory and optional chunks ([`99023bb`](https://github.com/Byron/gitoxide/commit/99023bbde027be82e9217868df7f73ecd09bf705))
    - Load chunk index of midx file ([`fac8efa`](https://github.com/Byron/gitoxide/commit/fac8efacb31935c2143717ebe82003a0916f233f))
    - frame for git-chunk crate to share among git-pack and git-commitgraph ([`b2d2ae2`](https://github.com/Byron/gitoxide/commit/b2d2ae221d43cc14aa169ada3c471e2bd2adadf4))
    - basic midx header parsing ([`edf02ae`](https://github.com/Byron/gitoxide/commit/edf02ae46ce6f3f981acd99310878e1d4a00d23b))
    - First pieces of header parsing; allow to respect multi-index desired hash kind in git-odb ([`1a2a049`](https://github.com/Byron/gitoxide/commit/1a2a04930ab56ba778091e10b15cecf415f5058d))
    - frame for instantiation of multi-pack-index ([`5e085ec`](https://github.com/Byron/gitoxide/commit/5e085ecbea913e0b0191d8267e548fe859bdd5d9))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - way nicer progress messages for repo verification ([`4b4f9f8`](https://github.com/Byron/gitoxide/commit/4b4f9f81879ad181744022eb0d7dc02392a5e91e))
    - upgrade to prodash 17 ([`47860b7`](https://github.com/Byron/gitoxide/commit/47860b7e2769260cfb8522ae455c491605093423))
    - refactor ([`831397c`](https://github.com/Byron/gitoxide/commit/831397c99fee9f2d6758124d993386cca5534f7b))
    - Allow resolution of in-pack ref-deltas ([`84ade1d`](https://github.com/Byron/gitoxide/commit/84ade1d23060f10bf6c8529f8f693d06660b4f4e))
    - refactor ([`38426a1`](https://github.com/Byron/gitoxide/commit/38426a171844014201282a441ebfc7d1f4cfff94))
    - Test to reproduce ref-delta forward references and the issue it poses for index traversal ([`7db7195`](https://github.com/Byron/gitoxide/commit/7db7195953954ded32a410e8d11f07f4c5b61687))
    - Very rough version of repository verification ([`80a4a7a`](https://github.com/Byron/gitoxide/commit/80a4a7add688d16376b9bf2ed7f1c7f655b7c912))
    - refactor ([`6c06659`](https://github.com/Byron/gitoxide/commit/6c066597f310b1bd5eb5611c1147b48846bc0ac0))
    - Improve method signatures of `cache::Tree::*` ([`a79a7fb`](https://github.com/Byron/gitoxide/commit/a79a7fb638b45df88af0d0d5fc9ada6d824bc328))
    - cleanup and unify `verify_integrity()` method signature ([`91d0476`](https://github.com/Byron/gitoxide/commit/91d047658b114f372735116c9d8e6962a3873137))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - fix docs, again ([`7b2ab26`](https://github.com/Byron/gitoxide/commit/7b2ab263b9dbb2ad33a4dddfe82f4cd7f3187271))
    - fix build ([`e3977fe`](https://github.com/Byron/gitoxide/commit/e3977fe033550bfd3297cdd674934e40476aa38b))
    - Use InOrderIter from git-features ([`7721b5f`](https://github.com/Byron/gitoxide/commit/7721b5fc7cba86d785e0936fdfab2ea41163219f))
    - Basic IEOT parsing ([`35bdee4`](https://github.com/Byron/gitoxide/commit/35bdee4bf77787bcbe6c3dd715a677e2e46a8ad1))
    - Assure we are right about the leb64 buffer needed for a 64 bit int ([`7558844`](https://github.com/Byron/gitoxide/commit/7558844b40b6c9af5038fea6b8a4e81583c46bde))
    - Adapt to changes in git-features: use var-int decoding from there ([`52e3c6f`](https://github.com/Byron/gitoxide/commit/52e3c6f6f4cd1bf677c9189fb59db16173954669))
    - remove byteorder from git-pack ([`4122306`](https://github.com/Byron/gitoxide/commit/41223061a2b919fd190066315b419ea17cabfde3))
    - git-pack uses `memmap2` instead of `filebuffer` ([`d9011c7`](https://github.com/Byron/gitoxide/commit/d9011c71048ff34201917b0693586290c23b3ddf))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - Avoid the dashmap being cloned for each thread ([`6d3f52d`](https://github.com/Byron/gitoxide/commit/6d3f52dc13d7243a6bce6dab89a985114a75d94b))
    - Properly count total objects during pack creation ([`bcb3d37`](https://github.com/Byron/gitoxide/commit/bcb3d37a900a40fd70b7be7bad8b2d5db292d2af))
 * **Uncategorized**
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Fix git-pack changelog to be stable ([`fd5b616`](https://github.com/Byron/gitoxide/commit/fd5b616d6ce8f353bd96b2c4994af9ba9c878b3e))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - support Rust 1.52 ([`42e0487`](https://github.com/Byron/gitoxide/commit/42e0487286c1f745837c0ce337ed7c9d86b14516))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - thanks clippy ([`d8925f5`](https://github.com/Byron/gitoxide/commit/d8925f5bd7ac8ef2c98f0e57a1373e5ffba8ce23))
    - thanks clippy ([`5a68d2f`](https://github.com/Byron/gitoxide/commit/5a68d2feffc551ad5f07e90efb2307e966d2636b))
    - thanks clippy ([`1e051dc`](https://github.com/Byron/gitoxide/commit/1e051dc23fb298b0bfe3e9ffb85a95ecb9c0f93f))
    - don't try to short-cut detection of large offsets when writing index files ([`ba92cc0`](https://github.com/Byron/gitoxide/commit/ba92cc09ba41fe4c9a9097bfeb8d18016408fcdf))
    - refactor ([`e7fbd9f`](https://github.com/Byron/gitoxide/commit/e7fbd9f3700496ad7bb7e71226c4d25429f0ccd5))
    - Merge branch 'use-midx-in-store' ([`338521b`](https://github.com/Byron/gitoxide/commit/338521b0443b9dc1007581de42ef6a950f6e0bbf))
    - thanks clippy ([`533a532`](https://github.com/Byron/gitoxide/commit/533a532c86bcf0dae27558e66b1a5cd2e52983df))
    - thanks clippy ([`7bd3ad3`](https://github.com/Byron/gitoxide/commit/7bd3ad3ab9f17eaf94490bea04a9b1297fa5fe64))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
    - thanks clippy ([`35cf46f`](https://github.com/Byron/gitoxide/commit/35cf46f87ecc42cf033ca001acf1b5918b3fea1b))
    - refactor ([`0032223`](https://github.com/Byron/gitoxide/commit/003222365bb2f8ce7d915240db6ff84ccbca6db4))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`4ca9e07`](https://github.com/Byron/gitoxide/commit/4ca9e07c7ac062d48d64ad7b516274e32dbc51c6))
    - make fmt ([`066f3ff`](https://github.com/Byron/gitoxide/commit/066f3ffb8740f242c1b03e680c3c5c1a0e4c36c3))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.15.0 (2021-11-29)

<csr-id-598698b88c194bc0e6ef69539f9fa7246ebfab70/>

### Changed (BREAKING)

 - <csr-id-e7526b2a7b51cbac4018e1ab3b623a85987fadc2/> parallel utilities now use `Send + Clone` instead of `Send + Sync`.
   
   This helps to assure that thread-local computations always work with the
   kind of types we provide. The ones that are carrying out actions are
   notably not `Sync` anymore.
   
   We cater to that by defining our bounds accordingly, but for those
   who want to use other utilities that need Sync, using types like
   `Repository` and `thread_local!()` is the only way to make this
   work.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#250](https://github.com/Byron/gitoxide/issues/250), [#259](https://github.com/Byron/gitoxide/issues/259), [#263](https://github.com/Byron/gitoxide/issues/263)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#250](https://github.com/Byron/gitoxide/issues/250)**
    - Address FIXME related to git_pack::data::Object ([`96386fd`](https://github.com/Byron/gitoxide/commit/96386fd1379b32ce2333baf34f81133cb9817364))
    - move loose header manipulation from git-pack to git-object ([`598698b`](https://github.com/Byron/gitoxide/commit/598698b88c194bc0e6ef69539f9fa7246ebfab70))
 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - sketch a little more how packs could be accessed ([`3fce8f2`](https://github.com/Byron/gitoxide/commit/3fce8f2b35ec6c2076f66fdde16a5f99a68326ac))
    - unify trait bounds for parallel code: prefer Clone over Sync ([`c805d0b`](https://github.com/Byron/gitoxide/commit/c805d0b231cf4d2f51dae7705bfbbc6562f86c32))
    - remove trait bounds to allow single-threaded applications to exist ([`3c790e0`](https://github.com/Byron/gitoxide/commit/3c790e01de0dbd3ffa2683d5cf060723d11d64a5))
    - Turns out the new `PolicyStore` can co-exist with existing one… ([`5e9250f`](https://github.com/Byron/gitoxide/commit/5e9250f5027e4b2c701ceae72a6038ac2a4a2093))
 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - fmt ([`fbeddeb`](https://github.com/Byron/gitoxide/commit/fbeddebcab999f4898f768a3184906091f8ce0b8))
    - parallel utilities now use `Send + Clone` insted of `Send + Sync` ([`e7526b2`](https://github.com/Byron/gitoxide/commit/e7526b2a7b51cbac4018e1ab3b623a85987fadc2))
    - A mad attempt to use thread-local everywhere and avoid Sync… ([`0af5077`](https://github.com/Byron/gitoxide/commit/0af5077e1f028c1c69bbdc098bb567e486282c37))
 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Merge branch 'pack-consistency' ([`5982406`](https://github.com/Byron/gitoxide/commit/5982406b4e1b26fd383d9ec21a3cf652ec8ab25f))
    - Merge branch 'git-loose-objects' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-git-loose-objects ([`ee737cd`](https://github.com/Byron/gitoxide/commit/ee737cd237ad70bf9f2c5e0d3e4557909e495bca))
</details>

## 0.14.0 (2021-11-16)

<csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/>

An important bugfix to prevent assertion failures when writing thin packs.

### Bug Fixes

 - <csr-id-20b3994206aa5bc5e35cbbc9c8f8f99187077f79/> Adjust size-hints of resolving entries iterator and use the upper bound in delta tree.
   
   The delta-tree is a data structure that actually heavily relies on
   favorable allocation and a known amount of objects in order to
   provide front and back buffers. However, this is an implementation
   detail and they don't have to stay consistent at all especially
   after growing the buffer by pushing to it.
   
   Interestingly, the VecDeque internally over-allocates as well which
   definitely helps the example of `as_mut_slices()`, otherwise
   it could also suffer from the assertions that trigger here.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 18 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#247](https://github.com/Byron/gitoxide/issues/247), [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#247](https://github.com/Byron/gitoxide/issues/247)**
    - Rename gix->ein and gixp->gix ([`e8b0919`](https://github.com/Byron/gitoxide/commit/e8b091943f0c9a26317da0003f7fcdf5a56ef21a))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
    - minor refactor ([`227c8b1`](https://github.com/Byron/gitoxide/commit/227c8b1859a6cbf96d48fd8564e575ef7e201db1))
    - Adjust size-hints of resolving entries iterator and use the upper bound in delta tree ([`20b3994`](https://github.com/Byron/gitoxide/commit/20b3994206aa5bc5e35cbbc9c8f8f99187077f79))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Adjust changelogs prior to git-pack release ([`ac8015d`](https://github.com/Byron/gitoxide/commit/ac8015de710142c2bedd0e4188e872e0cf1ceccc))
    - Move "loose object header" ser/de to git-object ([`3d1565a`](https://github.com/Byron/gitoxide/commit/3d1565acfc336baf6487edccefd72d0226141a08))
</details>

## v0.13.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

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

## v0.12.0 (2021-10-15)

<csr-id-d8fe8141e80a9e9a433b5e1a072b850325c806c8/>
<csr-id-faf6f813927720c5adf62102f9ce46606ff2617c/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-71c628d46088ab455b54eb2330d24dcff96c911d/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>

This release contains bugfixes and features, but is considered breaking as `git-traverse`
signalled a breaking change which is one of our dependencies.

### Refactor

 - <csr-id-71c628d46088ab455b54eb2330d24dcff96c911d/> Use 'cache::Object' trait where it matters
 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files

### New Features

 - <csr-id-d6c44e6ab8f436020d4fb235e423b018fd1e7a9f/> dynamically sized full-object speeds up diff-based object counting
   which is what happens when counting objects for fetches where only changed objects should be sent.
 - <csr-id-50cf610e8939812c3d2268c48835e2dac67d0c31/> `cache::Object` trait for caching and retrieving whole objects
 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes
 - <csr-id-5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc/> object cache size is configurable

### Bug Fixes

 - <csr-id-d8fe8141e80a9e9a433b5e1a072b850325c806c8/> don't put more objects into the pack cache than needed.
   
   Previously when accessing a packed object, it would store the base
   object into the pack cache (if it wasn't retrieved from there)
   which is great if that operation is free.
   
   Since it isn't, it's better not to stress the cache with puts
   and trash more objects than necessary.
   
   Now only the last decompressed object will be put into the LRU cache.
 - <csr-id-faf6f813927720c5adf62102f9ce46606ff2617c/> don't include submodules in count,
   which avoids dealing with missing objects entirely. Those ominous missing objects where just git submodules after all.
   
   It's still a good idea to handle these gracefully though, git itself
   seems to ignore them, too, and so do we at least for now.

### Performance

 - <csr-id-f9232acf8e52f8cd95520d122469e136eb07b39f/> ObjectID specific hashers, using the fact that object ids are hashes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 35 commits contributed to the release over the course of 32 calendar days.
 - 36 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#198](https://github.com/Byron/gitoxide/issues/198), [#67](https://github.com/Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/Byron/gitoxide/issues/164)**
    - Avoid duplicate module paths in 'tree' and 'commit' ([`2f2d856`](https://github.com/Byron/gitoxide/commit/2f2d856efe733d3cf81110c0e0607d2e7c40d968))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - don't put more objects into the pack cache than needed ([`d8fe814`](https://github.com/Byron/gitoxide/commit/d8fe8141e80a9e9a433b5e1a072b850325c806c8))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com/Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - ObjectID specific hashers, using the fact that object ids are hashes ([`f9232ac`](https://github.com/Byron/gitoxide/commit/f9232acf8e52f8cd95520d122469e136eb07b39f))
    - Use a custom hasher for 'seen' objects hashset… ([`70179e2`](https://github.com/Byron/gitoxide/commit/70179e2cf8d15ba4e1cf8e94a9915bf5b02cf755))
    - don't include submodules in count… ([`faf6f81`](https://github.com/Byron/gitoxide/commit/faf6f813927720c5adf62102f9ce46606ff2617c))
    - control pack and object cache size in megabytes ([`60c9fad`](https://github.com/Byron/gitoxide/commit/60c9fad8002b4e3f6b9607bba6361871752f4d3d))
    - Use 'cache::Object' trait where it matters ([`71c628d`](https://github.com/Byron/gitoxide/commit/71c628d46088ab455b54eb2330d24dcff96c911d))
    - split data::output::count::objects into files ([`8fe4612`](https://github.com/Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - cache::Object trait for caching and retrieving whole objects ([`50cf610`](https://github.com/Byron/gitoxide/commit/50cf610e8939812c3d2268c48835e2dac67d0c31))
    - object cache size is configurable ([`5a8c2da`](https://github.com/Byron/gitoxide/commit/5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc))
    - dynamically sized full-object speeds up diff-based object counting… ([`d6c44e6`](https://github.com/Byron/gitoxide/commit/d6c44e6ab8f436020d4fb235e423b018fd1e7a9f))
    - Count ref-deltas in thin packs as well ([`80c6994`](https://github.com/Byron/gitoxide/commit/80c6994149d19917c25e36e1bdf0dc8c9678365e))
    - Add '--thin' flag to pack-create and pass it on ([`2664d73`](https://github.com/Byron/gitoxide/commit/2664d73f531a4b1f4bc784c1fe3a991711c86475))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 ([`5ae6ff5`](https://github.com/Byron/gitoxide/commit/5ae6ff52cd2cd1ccd1e26bb987c154eb19603696))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] generic write_object() ([`c569f83`](https://github.com/Byron/gitoxide/commit/c569f83363489dde03c8b9cd01e75d35f5e04dbc))
</details>

## v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 8 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com/Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com/Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #185] refactor ([`7604935`](https://github.com/Byron/gitoxide/commit/7604935b12eacb26a98bedc5f77636b5583629a5))
</details>

## v0.9.0 (2021-08-27)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
   - `data::object::Object` -> `data::Object`
   - `data::entry::Entry` -> `data::Entry`

* **new methods**
   - `Find::find_tag_iter()`
 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 5 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com/Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - [pack #179] refactor ([`76e66d1`](https://github.com/Byron/gitoxide/commit/76e66d1b9d24bb25a9f681d9612e52c8ccd60e2c))
    - [pack #179] move Tree traversal cache private ([`34e45d7`](https://github.com/Byron/gitoxide/commit/34e45d745cb8756831c56dc441695a25cd0069a9))
    - [pack #179] refactor ([`5a3677d`](https://github.com/Byron/gitoxide/commit/5a3677dd3f3dcab26a3d9270b6184fd0fe18c54e))
    - [pack #179] refactor bundle ([`420dca2`](https://github.com/Byron/gitoxide/commit/420dca29bccca6e7d759880d8342f23b33eead0d))
    - [pack #179] fix docs ([`7ad7a44`](https://github.com/Byron/gitoxide/commit/7ad7a4428d0e38f2ff776f7efab6996505d2bba2))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [pack #179] refactor ([`620d8a5`](https://github.com/Byron/gitoxide/commit/620d8a54db5cd8367ec85c8b837cab710c509e3e))
    - [pack #179] add changelog ([`2102569`](https://github.com/Byron/gitoxide/commit/210256932a338038adb55c5475d8f90560aa4c12))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com/Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com/Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] dissolve 'immutable' module ([`70e11c2`](https://github.com/Byron/gitoxide/commit/70e11c21b0637cd250f54381d5490e9976880ad9))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com/Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] into_mutable() -> into_owned() ([`7e701ce`](https://github.com/Byron/gitoxide/commit/7e701ce49efe5d40327770a988aae88692d88219))
    - [object #177] fix docs ([`25d8e7b`](https://github.com/Byron/gitoxide/commit/25d8e7b1862bd05489359b162a32c6ad45ecdf9a))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com/Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] fix docs ([`07be661`](https://github.com/Byron/gitoxide/commit/07be6611d1742633815566443f71eef8b85ad5c0))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - [pack #172] A note about empty packs in Bundle writer ([`09a777f`](https://github.com/Byron/gitoxide/commit/09a777f1da5e792c5eb4c8ff9e83504ad8d19c5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [actor #173] fix docs ([`2d7956a`](https://github.com/Byron/gitoxide/commit/2d7956a22511d73b767e443dac21b60e93f286dd))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com/Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - cleanup imports ([`e669303`](https://github.com/Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - Allow creation of empty indices ([`d122fc7`](https://github.com/Byron/gitoxide/commit/d122fc79cc9b9a52a2817bdd46d3215c10e61129))
    - [pack #170] there can only be one ([`dce4f97`](https://github.com/Byron/gitoxide/commit/dce4f97a84aa6a73e31e7397501cfce27241c5b8))
    - [pack #170] clru allows for free lists, reducing allocation pressure... ([`4d820d2`](https://github.com/Byron/gitoxide/commit/4d820d2f94dc3afc062bbd25e969c87410212c3a))
    - [pack #170] basic progress for resolution ([`ada0b96`](https://github.com/Byron/gitoxide/commit/ada0b96e3707c06d7d6f7e4002907e12b45f7419))
    - [pack #170] Basic entry resolution without progress ([`7461f31`](https://github.com/Byron/gitoxide/commit/7461f31f03d67ecc9fdf398adf3cb6d4eb365412))
    - [pack #170] first step towards resolving in multi-threaded mode… ([`f3c21f9`](https://github.com/Byron/gitoxide/commit/f3c21f99594ab4080b8aa1ffed9ea8a33e18fabd))
    - [pack #170] Don't double-lookup trees during traversal… ([`7b06829`](https://github.com/Byron/gitoxide/commit/7b068296fe5ca10af212d8fe2662940188b7359c))
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" ([`811bb54`](https://github.com/Byron/gitoxide/commit/811bb54991636f7e517087b62cf0c8c8cc2ad9e6))
    - [pack #67] Don't pre-fetch packed objects during counting ([`d08b673`](https://github.com/Byron/gitoxide/commit/d08b6739d8e9294b795aba75e9c7f9f20645af2b))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com/Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - [pack #67] refactor ([`14717f6`](https://github.com/Byron/gitoxide/commit/14717f6132672a5d271832a68de0b323b73abb2a))
    - [pack #67] Add cache debugging capabilities to git-features ([`8776c98`](https://github.com/Byron/gitoxide/commit/8776c9834ac4622b3057f5db464a9817ed9acdb0))
    - [pack #167] Use custom uluru version to avoid a lot of allocations… ([`7bd3671`](https://github.com/Byron/gitoxide/commit/7bd3671ad949d62f84147ef7ff3fde59937fee54))
    - [pack #164] fix docs ([`08ee674`](https://github.com/Byron/gitoxide/commit/08ee674c55cef6ab76520de2f836b246c907888c))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" ([`4c2ea21`](https://github.com/Byron/gitoxide/commit/4c2ea212bbffb0ba3c21ba388dfc79cc7a1c4734))
    - [pack #167] Use custom uluru version to avoid a lot of allocations… ([`8d49976`](https://github.com/Byron/gitoxide/commit/8d499762b74c08437d901bb98806e0a1fc6f93bb))
    - [pack #167] a single-threaded special case for counting… ([`65e29de`](https://github.com/Byron/gitoxide/commit/65e29de45a92c82cebd832634ab194db19a1b590))
    - [pack #167] generalize over immutable insertions… ([`169f000`](https://github.com/Byron/gitoxide/commit/169f000087aab18f0257fb0c61dc3b3901e97505))
    - [pack #167] refactor ([`6bf0f7e`](https://github.com/Byron/gitoxide/commit/6bf0f7e86312b2a4d262c80979c61c94519bd4b0))
    - [pack #167] progress is handled by reducer… ([`a22f8e1`](https://github.com/Byron/gitoxide/commit/a22f8e171e705bc42fcf290789e8e05423bd72d1))
    - [pack #167] Error handling for object input ([`0aac40c`](https://github.com/Byron/gitoxide/commit/0aac40c88a5c26f7c295db8433b510b168f15ca3))
    - thanks clippy ([`d689599`](https://github.com/Byron/gitoxide/commit/d689599d1b819c18a3be60075170dbe00462e216))
    - [pack #167] remove iterator based count objects impl… ([`7ec2f2b`](https://github.com/Byron/gitoxide/commit/7ec2f2b40e83aaa218360a8b5989792cd67de2ed))
    - [pack] A non-iterator version of parallel object counting… ([`04fe855`](https://github.com/Byron/gitoxide/commit/04fe855a37577d3da5bbd619807b44e449947893))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.2 ([`39a3f71`](https://github.com/Byron/gitoxide/commit/39a3f71ba5997ac26d9994cdc7c2145af3220f64))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.1 ([`045eb09`](https://github.com/Byron/gitoxide/commit/045eb094691324a398120f6039bbfa34b4fda1af))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.8.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com/Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
    - (cargo-release) version 0.6.0 ([`4b71e15`](https://github.com/Byron/gitoxide/commit/4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com/Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com/Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com/Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.5.0 ([`1687e59`](https://github.com/Byron/gitoxide/commit/1687e599be98d97925fbab594f31cf5558e9d2b1))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com/Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com/Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
    - Revert "break more dev-depedency cycles up to git-odb" ([`22337ce`](https://github.com/Byron/gitoxide/commit/22337ce23995eee474e7dfb2e37fb56814522942))
</details>

## v0.3.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 ([`8b24197`](https://github.com/Byron/gitoxide/commit/8b241977b31720e7f08809bca0b277267b29102e))
    - break more dev-depedency cycles up to git-odb ([`7ee278b`](https://github.com/Byron/gitoxide/commit/7ee278bf5b04adc5e4ab82cb83a3519f93587176))
</details>

## v0.3.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 135 commits contributed to the release over the course of 76 calendar days.
 - 77 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com/Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - [pack #153] finish transitioning to git-tempfile ([`38173fc`](https://github.com/Byron/gitoxide/commit/38173fcf62c04b485c4b309bdf7e6b7afacfcd58))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref #139] add missing docs ([`5422ec8`](https://github.com/Byron/gitoxide/commit/5422ec8923a5f3c284f7094894a952a392812e63))
    - [pack] refactor ([`581fb51`](https://github.com/Byron/gitoxide/commit/581fb51a84567e341d315e6bacee8e681718f7a7))
    - [pack] refactor ([`b19f6b9`](https://github.com/Byron/gitoxide/commit/b19f6b9b1fcd5ebbc5b1f2a4bef0543b1c693bd1))
    - [pack] fix docs ([`e7b9d96`](https://github.com/Byron/gitoxide/commit/e7b9d9613874cd1ebaf740dc08db467c461a4751))
    - [pack] fix build ([`98dd557`](https://github.com/Byron/gitoxide/commit/98dd557b963acfe1c4e717451d222c187c46a5da))
    - [pack] update CRC values when changing entries to satisfy all consistency checks ([`990ea48`](https://github.com/Byron/gitoxide/commit/990ea4866be2d22ae2043da2dcd9577b748de255))
    - [pack] fix trailer of last entry to match expected recomputed pack hash… ([`8d0ec7d`](https://github.com/Byron/gitoxide/commit/8d0ec7d7c0afb6112e66518a2987907d2e4d29e3))
    - [pack] refactor ([`1852e3e`](https://github.com/Byron/gitoxide/commit/1852e3ea98a462958862ab05f110649e3b06e2b5))
    - [pack] all tests running for now, but… ([`aec8439`](https://github.com/Byron/gitoxide/commit/aec8439683c639f7b6e344cb76bf1dd9fc769d17))
    - [pack] hacky proof of concept that this actually works… ([`6085a92`](https://github.com/Byron/gitoxide/commit/6085a9201ecbd9285547c1d17c9834f09e22fef9))
    - [pack] on the way to 'quickly' get a proof of concept ([`cdc7582`](https://github.com/Byron/gitoxide/commit/cdc7582ab7e35ec1daac44401bf7cb62e0b592a2))
    - [pack] refactor ([`685cce6`](https://github.com/Byron/gitoxide/commit/685cce612eec99ed9f15d86d5ce2a7e6c270ae0d))
    - [pack] refactor ([`f822ebb`](https://github.com/Byron/gitoxide/commit/f822ebb9e899bd52d5baec8179a843c47d073e44))
    - thanks clippy ([`96ef0b0`](https://github.com/Byron/gitoxide/commit/96ef0b036c3c94a45f3ab882a8b32bfcc1250653))
    - [pack] a quickly made iterator that writes input::Entries ([`116bdc4`](https://github.com/Byron/gitoxide/commit/116bdc4ba879da9785877ebca56ab3c57b9cfd98))
    - [pack] prepare a custom writing iterator for input::Entries… ([`a4d2764`](https://github.com/Byron/gitoxide/commit/a4d27648b4021bcf65c95dc5bcfa2b3d11f538fd))
    - thanks clippy ([`bd517d6`](https://github.com/Byron/gitoxide/commit/bd517d6374f20670086eedce2776a8ecf7d0d22b))
    - [pack] prepare bundle writer for yet another iterator wrapper… ([`33be1a1`](https://github.com/Byron/gitoxide/commit/33be1a1ffba34a64eeb04b4479790fec2f50bcba))
    - [pack] refactor ([`50861e6`](https://github.com/Byron/gitoxide/commit/50861e6266a6e1800607eb19288e040846325c06))
    - [pack] refactor ([`dc07225`](https://github.com/Byron/gitoxide/commit/dc07225d7eea04e0cfe61c87b56009e06491726c))
    - [pack] another todo down, the last one ([`3fc8c8f`](https://github.com/Byron/gitoxide/commit/3fc8c8ff5ab1c49b55e3b9e1af3fa2f0aee68b94))
    - [pack] one more todo down, it should work now, right?… ([`69a9ff1`](https://github.com/Byron/gitoxide/commit/69a9ff17b3fe16de782ffabb76b87510e8a5b74e))
    - [pack] fix thin pack support test… ([`4bdebdd`](https://github.com/Byron/gitoxide/commit/4bdebddd3791ba71f3f6b4182229a1c48c5a4a95))
    - [pack] definitely not working yet ([`690d9b7`](https://github.com/Byron/gitoxide/commit/690d9b7fbc34b7d2393649d39290071f81cb8bb1))
    - [pack] a step closer, new cases show up ([`75eaba3`](https://github.com/Byron/gitoxide/commit/75eaba36072cf29e76a97fbbd425f0861eb657e2))
    - [pack] refactor ([`a8512f8`](https://github.com/Byron/gitoxide/commit/a8512f89a4e0dd7492fa208c1da41eed9d6a208f))
    - [pack] improved test to validate a fix ([`e3eeeb1`](https://github.com/Byron/gitoxide/commit/e3eeeb146a0ba3dbe701b2e4da560309ff181753))
    - [pack] attempt to get a more realistic test, but… ([`2890737`](https://github.com/Byron/gitoxide/commit/2890737c7e074d31f3bb55acb63664a2da93faaa))
    - [pack] refactor ([`cabc1e5`](https://github.com/Byron/gitoxide/commit/cabc1e5858d52806542ee8d9266bac36e5d39c96))
    - [pack] first succeeding test ([`f5da439`](https://github.com/Byron/gitoxide/commit/f5da439dce93cc203dacb4a5e9d0ae68a87b9be4))
    - [pack] first reasonably failing test showing that offset computation is indeed wrong ([`df1bc2f`](https://github.com/Byron/gitoxide/commit/df1bc2f66ff9e7046898b6937c5ad239313a70dc))
    - [pack] the first test for the lookup ref deltas iter ([`b162f9e`](https://github.com/Byron/gitoxide/commit/b162f9eb37f09f49e363376dc3f0c6c126442bbf))
    - [pack] Make use of thin-pack resolver when writing bundles… ([`9f43bf0`](https://github.com/Byron/gitoxide/commit/9f43bf029624f7c94346646465e366609b89e2e1))
    - [pack] handle the same ref-base correctly ([`2f94854`](https://github.com/Byron/gitoxide/commit/2f948545a935d2cb7c5a252ec74764440a9ff595))
    - [pack] thin pack resolver which might actually work ([`54f055a`](https://github.com/Byron/gitoxide/commit/54f055a53e888156459340e8ab160650a198ab13))
    - [pack] first sketch of resolver for thin pack entries ([`ee428e0`](https://github.com/Byron/gitoxide/commit/ee428e07bcc3df9bc795d06068a444beed71f2d0))
    - [pack] refactor ([`a8fd70f`](https://github.com/Byron/gitoxide/commit/a8fd70fdbff871779ad5a9ba491162ae49605c9f))
    - [pack] thanks clippy ([`7c2fc89`](https://github.com/Byron/gitoxide/commit/7c2fc89c70aa6de9cb0707799918e623267326a8))
    - [pack] actually, this is how it works, so this code should be unreachable ([`8f359e1`](https://github.com/Byron/gitoxide/commit/8f359e1fc8cb99fcf0003eaab1d97cdeaac20876))
    - [pack] first step towards fixing bad-objects properly ([`3c96507`](https://github.com/Byron/gitoxide/commit/3c965070a7c799f0507f9e7faae2896346bc9e65))
    - [pack] discard bad-object tracking in favor of delayed handling ([`31ce008`](https://github.com/Byron/gitoxide/commit/31ce008208cdd3bc4f093abab6fabf4c8074c130))
    - Revert "[pack] fix race to finally make pack-gen missing objects…" ([`ad0d2a8`](https://github.com/Byron/gitoxide/commit/ad0d2a8e4e92d11351225db0115de0ed1210f9e3))
    - [pack] fix race to finally make pack-gen missing objects… ([`73394db`](https://github.com/Byron/gitoxide/commit/73394db1b048d3dc87b8b4934737f27b6a8a0d3c))
    - [pack] it seems git is just skipping bad objects during pack-gen ([`0f29b82`](https://github.com/Byron/gitoxide/commit/0f29b82b48f45f509016eb16ea92af7f6dbf65a6))
    - Revert "[pack] FAIL: See if not looking up the pack location speeds up counting…" ([`d03fe97`](https://github.com/Byron/gitoxide/commit/d03fe9732b69c6ca3b7a6df96097233661e53a05))
    - [pack] FAIL: See if not looking up the pack location speeds up counting… ([`48c4930`](https://github.com/Byron/gitoxide/commit/48c49300a55e6443d5e4d94632979b6d07f2bc5a))
    - Revert "[pack] FAIL: speedup with Mutex<HashSet>" ([`df98edf`](https://github.com/Byron/gitoxide/commit/df98edf48c49717136a6e8e5d9b1f64aeda17db2))
    - [pack] FAIL: speedup with Mutex<HashSet> ([`f8aca03`](https://github.com/Byron/gitoxide/commit/f8aca03c2d126574541c136019df4e51b52a5b10))
    - [pack] In single-threaded mode, use a huge cache for some speedup ([`aec8a9b`](https://github.com/Byron/gitoxide/commit/aec8a9b4b9deb102b06390a19727eab7660621f9))
    - [pack] fix offset index properly by using chunk-absolute offsets ([`461c1ee`](https://github.com/Byron/gitoxide/commit/461c1eefe9214b07cd80a37292b23744846383d3))
    - [pack] forcefully fix issue with incorrect partition point ([`290bd65`](https://github.com/Byron/gitoxide/commit/290bd65f10f5a64de6735b09119b7bbffc44254b))
    - [pack] test for parital pack without thin pack allowance… ([`1f48d3b`](https://github.com/Byron/gitoxide/commit/1f48d3b58a1151a1fefce9bf4af5649837309a37))
    - [pack] pack-create with immediate counting and traversing… ([`b74a98f`](https://github.com/Byron/gitoxide/commit/b74a98fc87a92a8ccbaec59aeea5284731e2fe49))
    - [pack] entry writer now supports deltas and it seems to work even ([`fcda6f0`](https://github.com/Byron/gitoxide/commit/fcda6f096f95a6322122229ac364a2dd5ea0ce6b))
    - thanks clippy ([`cc61f82`](https://github.com/Byron/gitoxide/commit/cc61f82f597d9a0ab43efaaccc2cb568b9aa746f))
    - [pack] on-demand cache for pack-offset to id lookup ([`0bfdea8`](https://github.com/Byron/gitoxide/commit/0bfdea843606673005ecab6a482a9fce89a4cb69))
    - [pack] refactor ([`4bb3ce4`](https://github.com/Byron/gitoxide/commit/4bb3ce4f2e89dd817c284ed8ae9e2559ed60f9a2))
    - [pack] thin pack offset to index lookup ([`121aca4`](https://github.com/Byron/gitoxide/commit/121aca45ecb1acce3496b1b2ac003aa95851f247))
    - [pack] refactor ([`372b9ce`](https://github.com/Byron/gitoxide/commit/372b9cee78a6b49eb7ebb5cf452a324e07775d98))
    - [pack] a way to obtain whole bundles for offset-to-index lookup ([`15fcbe2`](https://github.com/Byron/gitoxide/commit/15fcbe254b75e8f74652711cc339ae5ade74d24c))
    - [pack] refactor ([`64b1dcd`](https://github.com/Byron/gitoxide/commit/64b1dcdb0fb53749ce73017d0dc1e053689d17d4))
    - [pack] refactor ([`1d713b4`](https://github.com/Byron/gitoxide/commit/1d713b482264ddb0aba6a98e3918f8236ce12c80))
    - [pack] refactor ([`cdf020a`](https://github.com/Byron/gitoxide/commit/cdf020a3b29bc59062d3ccf56672e9c18201c67c))
    - [pack] refactor ([`2ccefb2`](https://github.com/Byron/gitoxide/commit/2ccefb2832b326966a24d0cbcfd79ca5309f91aa))
    - [pack] refactor; entry-iterator now produces delta-objects ([`5dc370b`](https://github.com/Byron/gitoxide/commit/5dc370ba01d25a6e8b7f4bfa03259c83e6b1d758))
    - [pack] rough version of obtaining object indices for deltas ([`a58e270`](https://github.com/Byron/gitoxide/commit/a58e270ef96011ffd2434539e3099cbe27aed3f3))
    - [pack] refactor ([`8cfa414`](https://github.com/Byron/gitoxide/commit/8cfa414482a4318ed385f42582ec885fb73134e3))
    - [pack] pass all data to where it belongs to… ([`af5cb1f`](https://github.com/Byron/gitoxide/commit/af5cb1f4b809ac268ca3d878896854c966dcea97))
    - [pack] add the notion of thin-packs to the pack generator ([`a289bba`](https://github.com/Byron/gitoxide/commit/a289bbaa36546109d3371a8fcd7a6dc3c363861f))
    - [pack] build an index of pack ranges as well ([`4d6ab7b`](https://github.com/Byron/gitoxide/commit/4d6ab7b74c325820a3760361faace380f958572f))
    - [pack] bundle::Location with pack offset; order counts by that… ([`f92f285`](https://github.com/Byron/gitoxide/commit/f92f285167c6b5bc4d86f255e360c4534e38bb29))
    - [pack] better identify the currently implemented pack generation mode. ([`f9e3b3c`](https://github.com/Byron/gitoxide/commit/f9e3b3ca3bbf063e8d71c62fe607b812c745a969))
    - [pack] refactor ([`f3dc3da`](https://github.com/Byron/gitoxide/commit/f3dc3da492e1dda5dd9e43fddc57da6a118081b3))
    - [pack] refactor ([`9ee1e22`](https://github.com/Byron/gitoxide/commit/9ee1e22fa5c5d97ff626f0dfc44706272433bfef))
    - [pack] refactor ([`78d46c1`](https://github.com/Byron/gitoxide/commit/78d46c13d0510ee3e2e2f33cd60d624d63e85900))
    - [pack] refactor ([`69af352`](https://github.com/Byron/gitoxide/commit/69af3526b0fcfa8a270238f3e2cf59d332bd187e))
    - change wording ([`6c82a16`](https://github.com/Byron/gitoxide/commit/6c82a16d340acb9b11c5cf56c917c9fe6f2cdf0e))
    - Bump uluru from 2.1.1 to 2.2.0 ([`52e274f`](https://github.com/Byron/gitoxide/commit/52e274fe985948b6b742ff7066fcb9831e427ba3))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com/Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com/Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary unsafe code ([`83e207a`](https://github.com/Byron/gitoxide/commit/83e207a44aece0ff4870e57990bd5aaf43f38e22))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com/Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - fix build ([`dbfa49a`](https://github.com/Byron/gitoxide/commit/dbfa49acf58b2c0763c5e98e5276860b43dfb27b))
    - Fix everything up so that… ([`5930563`](https://github.com/Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com/Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - fix pack tests ([`7968467`](https://github.com/Byron/gitoxide/commit/7968467cc0d392e3d223811ed36ae777531a5a36))
    - The last occurrence of the global git-features::interrupt usage gone ([`6820724`](https://github.com/Byron/gitoxide/commit/6820724be83ebf48c7ccf6a65a3d6383f766c9de))
    - another one ([`0a8ed0e`](https://github.com/Byron/gitoxide/commit/0a8ed0ecc078d76dc3a5fe13518cf43bfbb121f0))
    - And another one down ([`abce75e`](https://github.com/Byron/gitoxide/commit/abce75eefff44b9538c112b60ad5e0596482e89c))
    - refactor ([`7f9be36`](https://github.com/Byron/gitoxide/commit/7f9be36ea909ee67555591287bcb140fdc54c801))
    - And one less usage of the global interrupt handler… ([`5da57a3`](https://github.com/Byron/gitoxide/commit/5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23))
    - thanks clippy ([`3b2e765`](https://github.com/Byron/gitoxide/commit/3b2e7650d8afe2c0e246e005ab1c321a157cbd44))
    - Make most interrupts local to the method or function ([`4588993`](https://github.com/Byron/gitoxide/commit/458899306a3f3c8578f185d7ecbf1ade2a7142dd))
    - [features] sketch of iterator to auto-check for interruptions ([`61d3a15`](https://github.com/Byron/gitoxide/commit/61d3a15c66b4c1be1d98715b8a60705a3a314455))
    - [pack] refactor ([`25f04ba`](https://github.com/Byron/gitoxide/commit/25f04baa100bd1996f48fbeb4c87e40ff1b27d90))
    - [pack] refactor ([`18cabb8`](https://github.com/Byron/gitoxide/commit/18cabb8618ffc324412302bfda208948abffb61f))
    - [pack] also put counts in order for stable packs ([`f299160`](https://github.com/Byron/gitoxide/commit/f299160cafd00f0fea00a2402901570f5ddf27d5))
    - [pack] fix run of 'cargo test --all' ([`e7ecdc1`](https://github.com/Byron/gitoxide/commit/e7ecdc195d03fa9a29ad1e44464b42e3ca6fb6a4))
    - [pack] a working in-order iterator ([`5fea926`](https://github.com/Byron/gitoxide/commit/5fea926803bcc7b2ef7d8f156e3d31a503831091))
    - [pack] tests for error handling of in-order iterator ([`44892cc`](https://github.com/Byron/gitoxide/commit/44892cca9309c4cca0eaa30dbedc65422a2699d1))
    - [pack] ground work for ordering in produced chunks ([`9680649`](https://github.com/Byron/gitoxide/commit/96806494d32243bd1798a89c094e220dbe050d68))
    - [pack] also run multi-threaded tests as part of unit-tests ([`5d3006a`](https://github.com/Byron/gitoxide/commit/5d3006a5d075bce9011b20920a84404952624c45))
    - Bump uluru from 2.0.0 to 2.1.1 ([`b6ac506`](https://github.com/Byron/gitoxide/commit/b6ac506ba2df0f82eaae64eaf023cc0c0376ddff))
    - [pack] hopefully fix tests on CI; verify determinism of pack ([`51dec8b`](https://github.com/Byron/gitoxide/commit/51dec8b3c661ba9071306ab89796aa93d9a25b65))
    - [pack] deterministic single-threaded pack generation ([`ddb6442`](https://github.com/Byron/gitoxide/commit/ddb6442fd6681a2dd3890a8a415003ec770c7d64))
    - [pack] refactor ([`cfdf802`](https://github.com/Byron/gitoxide/commit/cfdf8021ea1448ac4844b1f3bf252fefde2572fa))
    - [pack] basic statistics for entries ([`37229a6`](https://github.com/Byron/gitoxide/commit/37229a650ceb0155aa7ca87b499fe188ac4bb565))
    - thanks clippy ([`18b2113`](https://github.com/Byron/gitoxide/commit/18b2113b1e3c372145bc9037ee6a9de7efe4e506))
    - [pack] write packs to a directory with the proper name ([`3fbca7d`](https://github.com/Byron/gitoxide/commit/3fbca7dd62752a7dd752b83a39ec8dfd7b2f2ea8))
    - [pack] refactor ([`f10adea`](https://github.com/Byron/gitoxide/commit/f10adea76d92eada3ca204fe69e7b5f81a06d8cc))
    - [pack] fix docs ([`6ba471d`](https://github.com/Byron/gitoxide/commit/6ba471d228c45a3821b4984905a4b4ecaff5b0b0))
    - [pack] fix build ([`81ee633`](https://github.com/Byron/gitoxide/commit/81ee633c7f482746bc28a2a43d74ebbaded7af5f))
    - [pack] statistics for counting objects seemingly work… ([`4e3deb1`](https://github.com/Byron/gitoxide/commit/4e3deb1364dd1bef0af79d6aa97086a95b4983bc))
    - [pack] actual counts statistics ([`3a9f6d8`](https://github.com/Byron/gitoxide/commit/3a9f6d8a53da3235bde4a3f32859381d4843cb7e))
    - [pack] aggregate the count outcome ([`c7ac0e6`](https://github.com/Byron/gitoxide/commit/c7ac0e60a5d69f3a948d47c3acc3060cddbafb98))
    - [pack] use statistics reducer ([`0974ab1`](https://github.com/Byron/gitoxide/commit/0974ab176777bfa02ac0ea32915f6d9c46e3ddeb))
    - [pack] count object reducer sketch ([`ea45692`](https://github.com/Byron/gitoxide/commit/ea4569282e2f63042869dd47205874c161bfecfe))
    - [pack] refactor ([`fdf485a`](https://github.com/Byron/gitoxide/commit/fdf485afa66af20abca586b04f588a33c167310f))
    - [pack] refactor ([`0514f1d`](https://github.com/Byron/gitoxide/commit/0514f1df113c5f6bf1c934b15741ca8ea47316ae))
    - [pack] refactor ([`37922d1`](https://github.com/Byron/gitoxide/commit/37922d12765c221e747fad4ca813597490525279))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - refactor ([`a25a774`](https://github.com/Byron/gitoxide/commit/a25a774675e2e9db1c891351077d3af2fd5c72ed))
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports ([`de2ba3c`](https://github.com/Byron/gitoxide/commit/de2ba3c4919d454894911c54fd4bb0e0a4665723))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

## v0.2.0 (2021-05-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`b213628`](https://github.com/Byron/gitoxide/commit/b213628feeb8dfa87dab489c7d3155a60e6a236d))
    - [git-odb] prep release ([`4984ce3`](https://github.com/Byron/gitoxide/commit/4984ce3e19b60b89a4337f90ac4b9c44c42558a0))
    - [git-odb] refactor ([`2958145`](https://github.com/Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - [git-pack] fix docs ([`efd20d4`](https://github.com/Byron/gitoxide/commit/efd20d4e1afbfbe573d620dea4761c06f948a296))
    - [git-pack] refactor ([`ea2b3de`](https://github.com/Byron/gitoxide/commit/ea2b3deab78882943e11270e4166ca7c340b03e1))
    - [git-pack] refactor ([`bc4b7b1`](https://github.com/Byron/gitoxide/commit/bc4b7b18a04506a3d08d66d1222d706b82a2f6e7))
    - [git-pack] refactor ([`157b6ff`](https://github.com/Byron/gitoxide/commit/157b6ff7b55ba2b7f8f90f66864212906426f8d7))
    - [git-pack] refactor ([`49c1c3e`](https://github.com/Byron/gitoxide/commit/49c1c3ea67379c5a122a8c3921d8ff713e14d371))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com/Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] refactor ([`be6ddaa`](https://github.com/Byron/gitoxide/commit/be6ddaa98fc1dcaf77dc0fd9c9d67754e74927e4))
    - [git-pack] used by git-odb ([`5d6ee07`](https://github.com/Byron/gitoxide/commit/5d6ee07a8dec64fe5f68c14c418d922077fad3df))
    - [git-pack] refactor ([`1b2a245`](https://github.com/Byron/gitoxide/commit/1b2a245aa494c0f9cacc2ad6b8ca02e9891fdb4c))
    - [git-pack] move hash-writer to git-features as it's quite general purpose ([`80e5640`](https://github.com/Byron/gitoxide/commit/80e5640169363910b4189fda58bb495c6677eaaa))
</details>

## v0.1.0 (2021-05-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-pack] the very first version… ([`8c06cdb`](https://github.com/Byron/gitoxide/commit/8c06cdb14269e798b7ff771ea3864f85fa673ed7))
</details>

