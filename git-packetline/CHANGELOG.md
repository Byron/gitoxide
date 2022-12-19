# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.14.1 (2022-12-19)

### New Features

 - <csr-id-41fdb84717b825399bfaefb58e98a84a8b373cb5/> `WithSidebands` now offers a `read_data_line(byte_buf)` method.
   That way one won't have to assume UTF8 encoding in the returned buffer.
   Note that the reason for it not returning a reference to its internal
   buffer is due to the async implementation requiring it. Its future-based
   architecture can't really express the lifetimes associated with it (yet).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
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
    - Merge branch 'remove-lines-parsing' ([`9d8e32d`](https://github.com/Byron/gitoxide/commit/9d8e32d3c276fec34e3fce0feb29de0d24a8d1d2))
    - `WithSidebands` now offers a `read_data_line(byte_buf)` method. ([`41fdb84`](https://github.com/Byron/gitoxide/commit/41fdb84717b825399bfaefb58e98a84a8b373cb5))
    - refactor ([`8fb21d7`](https://github.com/Byron/gitoxide/commit/8fb21d7dbac06b518943813d99388c8dbf58830e))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.14.0 (2022-11-21)

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

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 42 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
</details>

## 0.13.1 (2022-10-10)

### Bug Fixes

 - <csr-id-d7f62b441700c6d3526517c8c4f369cb9a72c102/> support keepalive packets.
   Keepalive packets are side-band only empty datalines that should
   just be ignored. This is now happening, allowing longer git
   operations to work as they will send keepalive packets every 5 seconds,
   and previously we would choke on it.
   
   Note that empty datalines are never send otherwise, making it a
   previously unused marker that can safely be skipped.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - support keepalive packets. ([`d7f62b4`](https://github.com/Byron/gitoxide/commit/d7f62b441700c6d3526517c8c4f369cb9a72c102))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.13.0 (2022-09-20)

<csr-id-5a74999f853215feb33140997c4a0dc62e49df66/>

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Chore (BREAKING)

 - <csr-id-5a74999f853215feb33140997c4a0dc62e49df66/> replace quick-error with thiserror
   Many of the error definitions changed from tuple types to structs.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - fix docs ([`dad9cbe`](https://github.com/Byron/gitoxide/commit/dad9cbeb853c0cc5128360b05c04b5a3da7ec75e))
    - replace quick-error with thiserror ([`5a74999`](https://github.com/Byron/gitoxide/commit/5a74999f853215feb33140997c4a0dc62e49df66))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - thanks clippy ([`1d85564`](https://github.com/Byron/gitoxide/commit/1d8556461a585188616683d3b9998ec07936d5fc))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.12.7 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.12.6 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 90 calendar days.
 - 90 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.12.5 (2022-05-18)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 40 calendar days.
 - 45 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - fix clippy - many false positives this time ([`045e6fa`](https://github.com/Byron/gitoxide/commit/045e6fae17077555c3e115992905c8046f2c5d0b))
    - fix clippy - many false positives this time ([`099bd5b`](https://github.com/Byron/gitoxide/commit/099bd5b86fb80b26a73863b80ad60a0394458b6d))
</details>

## 0.12.4 (2022-04-03)

### New Features

 - <csr-id-b90eb9b272200beb5edeaa5c56bb132faf69b28c/> in-manifest and in-lib documentation of features

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 55 calendar days.
 - 69 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#329](https://github.com/Byron/gitoxide/issues/329)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - git-packetline provides more documentation on docs.rs ([`2001b94`](https://github.com/Byron/gitoxide/commit/2001b9437d0b2f3e6f360b8e09650dfcf27862ef))
    - in-manifest and in-lib documentation of features ([`b90eb9b`](https://github.com/Byron/gitoxide/commit/b90eb9b272200beb5edeaa5c56bb132faf69b28c))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
</details>

## 0.12.3 (2022-01-23)

A maintenance release thanks to changed dependencies.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
    - adapt to changes in git-hash ([`82fec95`](https://github.com/Byron/gitoxide/commit/82fec95e9ed4b924849bfcc84b5b2691a925a5b3))
 * **Uncategorized**
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`42ebb53`](https://github.com/Byron/gitoxide/commit/42ebb536cd6086f096b8422291776c9720fa0948))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`d1a956d`](https://github.com/Byron/gitoxide/commit/d1a956d8ccc9d0e27d81ce1c7973e339e6c9b66b))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.12.2 (2021-11-29)

A maintenance release, triggered by putting too many adjustments into a single commit.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 12 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Fix build warnings related to pin-project lite ([`126aeec`](https://github.com/Byron/gitoxide/commit/126aeec1f4cb358c7d24fec4fb0a92e7ff9319e8))
</details>

## 0.12.1 (2021-11-16)

A maintenance release triggered by changes to git-pack and changelog rewrites.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
</details>

## v0.12.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.11.0 (2021-10-15)

### Dependency Upgrade (BREAKING)

* `git-traverse` saw a breaking change moving to v0.9, which triggered this crate to signal a breaking change, too.

### Type Change (BREAKING)

* `read_line(…)` now strongly types `ERR` packet lines using the new `read::Error`
   type instead of transforming it into a string-error.
   This makes it easier to retrieve the exact error message from the
   returned `std::io::Error` which is useful for presentation to the user.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 31 calendar days.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200)

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
    - refactor and improve path filtering to find relevant commits… ([`01b2466`](https://github.com/Byron/gitoxide/commit/01b246644c76d842892a8dfcf8392026baf288d8))
    - The first headline level controls all the other ones ([`715ea54`](https://github.com/Byron/gitoxide/commit/715ea54624a2651a4828ccd8cd035889495212b8))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com/Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#200](https://github.com/Byron/gitoxide/issues/200)**
    - parse issue numbers from description and clean it up ([`95c0a51`](https://github.com/Byron/gitoxide/commit/95c0a510f875e8fd889b87caee356a4c1e099ea8))
    - feat: add git_packetline::read::Error to represent ERR lines ([`454c840`](https://github.com/Byron/gitoxide/commit/454c840c652ecb5774d2e3d37be14db42749ea63))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.10.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.1 ([`4f9da02`](https://github.com/Byron/gitoxide/commit/4f9da02ae0f0ce8e62b20852319f46ab26b88d89))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - [ref #190] more conversion trait impls ([`1795a33`](https://github.com/Byron/gitoxide/commit/1795a333c05c60a1a2f3164d5c4c78289eb7050c))
</details>

## v0.10.0 (2021-08-27)

#### Breaking

* **renames / moves**
    - `immutable::PacketLine` -> `PacketLineRef`
    - `immutable::Error` -> `ErrorRef`
    - `immutable::Text` -> `TextRef`
    - `immutable::Band` -> `BandRef`
    - `immutable::DecodeBandError` -> `decode::band::Error`
    - `pub immutable::` -> `line::`
    - `pub write::` -> `write::`

* **removals**
   - `write::Writer` (is now only `Writer`)
   - `read::StreamingPeekableIter` (is now only `StreamingPeekableIter`)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com/Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - Bump git-packetline v0.10.0 ([`b09f391`](https://github.com/Byron/gitoxide/commit/b09f3912e0addd7b4b0ef22bc3a24869d5011646))
    - [packetline #178] fix docs ([`878d8e8`](https://github.com/Byron/gitoxide/commit/878d8e8d9f88a31dd9db30e381e65c1031919474))
    - [packetline #178] refactor ([`0c7c599`](https://github.com/Byron/gitoxide/commit/0c7c5990fc71c0ee192e5ed42a6b8d268ea764fd))
    - [packetline #178] fix docs ([`b3fd65d`](https://github.com/Byron/gitoxide/commit/b3fd65d4130010d48afabe70b76880abcd6c8fb8))
    - [packetline #178] refactor ([`23438fd`](https://github.com/Byron/gitoxide/commit/23438fd4a807376c1d4699732ea6c83c0bde574f))
    - [packetline #178] rename PacketLine to PacketLineRef… ([`d4c16a9`](https://github.com/Byron/gitoxide/commit/d4c16a93946244177606b58cc702b81a16424ad4))
    - [packetline #178] add changelog in preparation for breaking changes ([`ffd96f9`](https://github.com/Byron/gitoxide/commit/ffd96f9fd747a99f0250444cf4b6f5a161646129))
</details>

## v0.9.1 (2021-08-17)

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
    - Release git-packetline v0.9.1 ([`2276e2a`](https://github.com/Byron/gitoxide/commit/2276e2aefb8a4e51024644826249b3f97da2ccdb))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.9.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.0 ([`7ffbd60`](https://github.com/Byron/gitoxide/commit/7ffbd602c08605026b0bb97ab85216907badaf09))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
</details>

## v0.8.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 ([`ad6d7f9`](https://github.com/Byron/gitoxide/commit/ad6d7f9c2b4f8879d466e758fc9b51ece6879e96))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
</details>

## v0.7.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`2ef3106`](https://github.com/Byron/gitoxide/commit/2ef3106eb84981e2dabd84f81362b4e44f938ea6))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
</details>

## v0.6.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 101 commits contributed to the release over the course of 89 calendar days.
 - 93 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#77](https://github.com/Byron/gitoxide/issues/77)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#77](https://github.com/Byron/gitoxide/issues/77)**
    - [git-packetline] refactor ([`aa61993`](https://github.com/Byron/gitoxide/commit/aa61993066b0bcb29e53edbb6eb1525781827426))
 * **Uncategorized**
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] refactor ([`bd94ea5`](https://github.com/Byron/gitoxide/commit/bd94ea55c1b598e507b5717ee5a5d6f14830c3bb))
    - [pack] fix docs ([`e7b9d96`](https://github.com/Byron/gitoxide/commit/e7b9d9613874cd1ebaf740dc08db467c461a4751))
    - Bump futures-io from 0.3.15 to 0.3.16 ([`3c23820`](https://github.com/Byron/gitoxide/commit/3c23820d3f0d3567f44215cdb0ad13ab675a201f))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - fix docs ([`2698dae`](https://github.com/Byron/gitoxide/commit/2698daec29ac68f928a06f2bc9f4df44fcc8222c))
    - fix build ([`22bda81`](https://github.com/Byron/gitoxide/commit/22bda81712b1379869abf764d47c05e03f697a50))
    - thanks clippy ([`3f7e27b`](https://github.com/Byron/gitoxide/commit/3f7e27b91e2c7d66959f5f4c1a667f3315111cd6))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - [async-client] unblock the async delegate in the cheapest possible way… ([`a3b5d75`](https://github.com/Byron/gitoxide/commit/a3b5d75d387dc5d6c44f695f63df8803613637a2))
    - Revert "[async-client] Try to bring 'Send' back but…" ([`52eb953`](https://github.com/Byron/gitoxide/commit/52eb953fcc44cce19604b1df6a600237b8c81392))
    - [async-client] Try to bring 'Send' back but… ([`3a06adb`](https://github.com/Byron/gitoxide/commit/3a06adb41f6b2946f78044e4ab1385e6441fc40f))
    - Prevent selecting mutually exclusive features ([`7f5da18`](https://github.com/Byron/gitoxide/commit/7f5da18c39b84af788ea1366ccca2c8b9d09f755))
    - [git-protocol] fetch in sync and async… ([`4776039`](https://github.com/Byron/gitoxide/commit/47760399bffd030c848e0ef6df52a4765d8fb566))
    - Bump maybe-async from 0.2.4 to 0.2.6 ([`d99a1a8`](https://github.com/Byron/gitoxide/commit/d99a1a815809d22c7384c6ecb1275e39fb911d91))
    - [git-protocol] fix build ([`4cce648`](https://github.com/Byron/gitoxide/commit/4cce6487d6d514541afee1a9aa92043f186136d3))
    - refactor ([`14c9093`](https://github.com/Byron/gitoxide/commit/14c909341d243ca3dcc42d343aeee65d28045b65))
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support ([`ee01c79`](https://github.com/Byron/gitoxide/commit/ee01c79887a892e001787bbefa93f75d9c4f1cfc))
    - [git-transport] ExtendedBufRead for Async… ([`d4e56c8`](https://github.com/Byron/gitoxide/commit/d4e56c8efd586b571445e0085ce518c5efb8f5e6))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com/Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-packetline] refactor ([`7e513f1`](https://github.com/Byron/gitoxide/commit/7e513f1fa3ba143bb1ae5f9052c195043a53943c))
    - [git-packetline] Switch back to pin-project-lite ([`63cb0fc`](https://github.com/Byron/gitoxide/commit/63cb0fcb6248e5b9489156d602235d0300858cbc))
    - [git-packetline] all tests green ([`fed6c69`](https://github.com/Byron/gitoxide/commit/fed6c69fd8b2877a66fe9d87916f3d54a3fc342b))
    - [git-packetline] Nearly there - one failing test and its known why it does that ([`51c63c0`](https://github.com/Byron/gitoxide/commit/51c63c081df4bd26adef7b8336034aee74237a86))
    - [git-packetline] another green test ([`e67d77d`](https://github.com/Byron/gitoxide/commit/e67d77d545530ddce18846b0a5e3d732f071a11b))
    - [git-packetline] Custom implementation of read_line future to avoid extra work… ([`91c2895`](https://github.com/Byron/gitoxide/commit/91c28954babfd863340a165721d3dab186b668a1))
    - [git-packetline] read_line test green, but… ([`8007c65`](https://github.com/Byron/gitoxide/commit/8007c653d9e2065db913f683a1aa39bd2e016ee5))
    - [git-packetline] fix compile errors if no features are specified ([`a2b44c8`](https://github.com/Byron/gitoxide/commit/a2b44c81a993b08d7786ca8139796f586229c90b))
    - [git-packetline] YES, finally, the first green test ([`f16b012`](https://github.com/Byron/gitoxide/commit/f16b0124e778b5b8d2272228cf1644f9706df85c))
    - Revert "Revert "[git-packetline] It compiles with parent as option, even with state machine"" ([`e300f9f`](https://github.com/Byron/gitoxide/commit/e300f9fbbf1dda914b3d53bfac584eaa59ffe03f))
    - Revert "[git-packetline] An Option really does the trick" ([`8eb78f5`](https://github.com/Byron/gitoxide/commit/8eb78f51f753680d1ad7123ed07c9d4fc2562632))
    - [git-packetline] An Option really does the trick ([`c05bd79`](https://github.com/Byron/gitoxide/commit/c05bd795156d7c3ca72ab39a01b57684c87d32c0))
    - Revert "[git-packetline] It compiles with parent as option, even with state machine" ([`890cc50`](https://github.com/Byron/gitoxide/commit/890cc5018b8816ce369e09e3fbe8041f7421d602))
    - [git-packetline] It compiles with parent as option, even with state machine ([`a97bbfd`](https://github.com/Byron/gitoxide/commit/a97bbfd6a4fafaf672186af72a53ed75fd817948))
    - [git-packetline] Even without pin projection lifetimes don't add up ([`7e834f5`](https://github.com/Byron/gitoxide/commit/7e834f584da1be7d00a0671df33d52171f79595f))
    - [git-packetline] [FAIL] For some reason the is a lifetime mismatch again… ([`b4ff4e7`](https://github.com/Byron/gitoxide/commit/b4ff4e7fae38dda4d281f41fb20abbd57c02993f))
    - [git-packetline] first step towards state based impl ([`22740c5`](https://github.com/Byron/gitoxide/commit/22740c5bd2cc0805cc795038b997ca189e1df6ec))
    - [git-packetline] Use what's learned previously to make it compile without added buffer ([`88511f7`](https://github.com/Byron/gitoxide/commit/88511f7f68f19db2e60ea4801e26243f39ad654e))
    - Revert "[git-packetline] get it to compile by resorting to another buffer" ([`3866517`](https://github.com/Byron/gitoxide/commit/38665173722ec57d72a3eb43f619e586ece81138))
    - [git-packetline] get it to compile by resorting to another buffer ([`01e15c8`](https://github.com/Byron/gitoxide/commit/01e15c8b6e4e582d75069f6e38f22ce37e5fb29c))
    - [git-packetline] [HACKY-SUCCESS] It's possible to do it, but how to do it without unsafe? ([`96d0ecf`](https://github.com/Byron/gitoxide/commit/96d0ecf535753068c440b8c9909f7e72bba6b5b9))
    - [git-packetline] [FAIL] No, cannot poll a dynamically created future ([`194c991`](https://github.com/Byron/gitoxide/commit/194c991d64fdf8fb6cffe12d5a8b6a2ba761e36e))
    - [git-packetline] [FAIL] try to brute-force keeping futures for polling… ([`42a7d00`](https://github.com/Byron/gitoxide/commit/42a7d00252434e6f0b200fbb4a0155415e2e8537))
    - [git-packetline] [FAIL] try to impl fill_buf - can't return parent buffer ([`1e8b006`](https://github.com/Byron/gitoxide/commit/1e8b006d3f8bed554ff247613b05a851849b574e))
    - [git-packetline] Upgrade to pin_project as drop impl is needed ([`3d53404`](https://github.com/Byron/gitoxide/commit/3d5340424020a95b39e8c7ee747bdfdae934bdd0))
    - [git-packetline] A step towards implementing poll_fill_buf ([`3c487de`](https://github.com/Byron/gitoxide/commit/3c487de86b9b7a7647372d7caf940617c571b9a1))
    - [git-packetline] Frame for async sideband ([`adc365e`](https://github.com/Byron/gitoxide/commit/adc365e019b2fead79e1a4ad5657a9d6b49305fd))
    - [git-packetline] Use underlying StreamPeekIter buffer instead of copying into own ([`88b8bc3`](https://github.com/Byron/gitoxide/commit/88b8bc33eda0c41af24575998a65232e5ce57e23))
    - [git-packetline] [FAIL] try to get rid of second buffer in sideband reader ([`4d8f4b5`](https://github.com/Byron/gitoxide/commit/4d8f4b5ba5ffb7044b0525d4f63778688f72d12e))
    - [git-packetline] streaming peek iter with async support ([`60164fd`](https://github.com/Byron/gitoxide/commit/60164fdaad02b538f1238232852bb231ec894269))
    - [git-packetline] fix docs ([`4a47c9e`](https://github.com/Byron/gitoxide/commit/4a47c9ea79bc908bbba81d1ffa021c53a9246101))
    - [git-packetline] refactor ([`e8b2dd1`](https://github.com/Byron/gitoxide/commit/e8b2dd118859222d87eacaa194a118225d450c00))
    - [git-packetline] Async IO for packetline serialization. ([`3bb9cf1`](https://github.com/Byron/gitoxide/commit/3bb9cf15a4703a88fab98223923f1acf50e57a46))
    - [git-packetline] refactor ([`2a84b78`](https://github.com/Byron/gitoxide/commit/2a84b787df693e8ce95bcde01663f6cdef8494cd))
    - [git-packetline] encode module now available as async edition ([`119fcc3`](https://github.com/Byron/gitoxide/commit/119fcc328aa1778f64d6b7342d1e439a8ac081a4))
    - [git-packetline] Use io::(Result|Error) everywhere ([`374f129`](https://github.com/Byron/gitoxide/commit/374f129e0d1473db9a2107c408f655da032efe89))
    - [git-packetline] Deduplicate 'encode' module tests ([`34f48c3`](https://github.com/Byron/gitoxide/commit/34f48c310643d5246799ad7d2ac968c36289893e))
    - [git-packetline] refactor ([`f038ca1`](https://github.com/Byron/gitoxide/commit/f038ca1e1c6d99bfcedb0387abc4151b188750c6))
    - [git-packetline] remove now unnecessary duplicate tests ([`c8178d7`](https://github.com/Byron/gitoxide/commit/c8178d7fe03e3dc6b24edc68f29a32dbf43b6d3c))
    - [git-packetline] Use maybe_async to deduplicate tests - neat ([`439a7b7`](https://github.com/Byron/gitoxide/commit/439a7b76c3d306a979890aedd0d857527830c1dc))
    - [git-packetline] refactor ([`d698d7b`](https://github.com/Byron/gitoxide/commit/d698d7bc4cfd49c6f752dab17f669bce27aa437a))
    - [git-packetline] All tests for high-level writer pass ([`eef8c9f`](https://github.com/Byron/gitoxide/commit/eef8c9f0b320cea89e900cfd7b5eed54d3bc7a8f))
    - [git-packetline] OMG it's green! ([`fbffd89`](https://github.com/Byron/gitoxide/commit/fbffd898eedc3a16369aeb65a496f6460fd5238e))
    - [git-packetline] An owning inplementation of the LineWriter ([`70ce3c9`](https://github.com/Byron/gitoxide/commit/70ce3c96f189e51a0d4d8b5f1f572372f64bcb0a))
    - [git-packetline] An owning LineWriter ([`445fac6`](https://github.com/Byron/gitoxide/commit/445fac6b079a8728a5b17f1a5cb70178fafe2c8a))
    - Revert "[git-packetline] Use no pin projections" - let's own the writer ([`6c5750a`](https://github.com/Byron/gitoxide/commit/6c5750a810fd8a13c67e947b72ec4dcdb717552b))
    - [git-packetline] Use no pin projections ([`dc4e0e5`](https://github.com/Byron/gitoxide/commit/dc4e0e5946dd24e92b52c592863e28736fcb636e))
    - [git-packetline] Allow different lifetimes for writer and buffers ([`3b3c53d`](https://github.com/Byron/gitoxide/commit/3b3c53dc85d70cce7a58aa5eb21e3b97249f6e45))
    - [git-packetline] A complete LineWriter implementation by hand, OMG ([`3299548`](https://github.com/Byron/gitoxide/commit/32995484a83756fd522f4b7ba45150254809ebfe))
    - [git-packetline] write prefix properly ([`432b214`](https://github.com/Byron/gitoxide/commit/432b2145e3618a0989ed0a99eb80b1827afe79c8))
    - [git-packetline] write hex_len properly ([`acdcfb7`](https://github.com/Byron/gitoxide/commit/acdcfb7b8b26adb4c77e5e1e6d550ab2cfe9b7dd))
    - [git-packetline] it compiles, but write_all needs to be implemented by hand ([`2c44350`](https://github.com/Byron/gitoxide/commit/2c44350d6906d5a01e985e6b5d1e690fd1ee35af))
    - [git-packetline] First draft of LineWriter - and it shows some teeth ([`13127ee`](https://github.com/Byron/gitoxide/commit/13127ee2dc93a993b952fb4e94d0736836496067))
    - [git-packetline] Make failing test pass officially for now ([`cbd6291`](https://github.com/Byron/gitoxide/commit/cbd6291a75565a8a15f38f7ffd6bc4918aa46a3a))
    - [git-packetline] it turns out that a simple write trait isn't simple ([`7933698`](https://github.com/Byron/gitoxide/commit/793369807fed9f4ddab5db012d84b2b83c2d9613))
    - [git-packetline] Calling auto-generated futures isn't easy :D ([`8361238`](https://github.com/Byron/gitoxide/commit/836123890d2604e9398589a98cd11feeb9810c7a))
    - [git-packetline] All encode capabilities that Write needs ([`88a971d`](https://github.com/Byron/gitoxide/commit/88a971d01f80bedeb180198585d0d6ba2f63bfc0))
    - [git-packetline] the first green encode test ([`ebc4703`](https://github.com/Byron/gitoxide/commit/ebc4703a26fc2d8a6d88a336489c1b8400d6c387))
    - [git-packetline] Now maybe_async would be useful ([`ab4b30e`](https://github.com/Byron/gitoxide/commit/ab4b30e4cebe52b5b3a6c9c19ce1f1d51f570cc4))
    - [git-packetline] refactor ([`7d79288`](https://github.com/Byron/gitoxide/commit/7d792887d743cc649ae20010a3686a14f65cd3ad))
    - [git-packetline] fix tests ([`b26c43b`](https://github.com/Byron/gitoxide/commit/b26c43bf5bd50e7dd0aaa9587e2e45c035ddcad8))
    - [git-packetline] prepare 'packetline' and 'encode' for async ([`1a986fb`](https://github.com/Byron/gitoxide/commit/1a986fb45e5286ddebf974e3498509876ff0ee08))
    - [git-packetline] One tiny step closer, and it's obvious there is more IO :D ([`0bef59c`](https://github.com/Byron/gitoxide/commit/0bef59cc930187f2ac9b760d127fcb38c4fcc341))
    - [git-packetline] the first green test ([`916c862`](https://github.com/Byron/gitoxide/commit/916c862f218bb0ae936e701500df7158fbdc6815))
    - [git-packetline] the first very failing test… ([`0220bca`](https://github.com/Byron/gitoxide/commit/0220bca6515f0cc46e649a696400ff458407a681))
    - [git-packetline] add async-io feature toggle ([`727ad97`](https://github.com/Byron/gitoxide/commit/727ad9700803d105f1a72c7cd7c7e8fe1a383c52))
    - refactor ([`c8ba842`](https://github.com/Byron/gitoxide/commit/c8ba842ca30a41eedc900526e9081a9e79b7a344))
    - [git-packetline] 'blocking-io' feature toggle and tests'blocking-io' feature toggle and tests ([`380e8b2`](https://github.com/Byron/gitoxide/commit/380e8b21bb34da5974ac661de0537a762bfceeb2))
    - [git-packetline] fix doc links ([`cf50f28`](https://github.com/Byron/gitoxide/commit/cf50f28f9237ef246d523e6ed7e574948da1df7b))
    - [git-packetline] refactor ([`1328c5b`](https://github.com/Byron/gitoxide/commit/1328c5b4001f380936beff73e1f822f14e41e98b))
    - thanks clippy ([`334e129`](https://github.com/Byron/gitoxide/commit/334e129e956a62400fc240effc7f527f10abc3d5))
    - [git-packetline] Fix performance regression ([`513e7ad`](https://github.com/Byron/gitoxide/commit/513e7ad2c1a38c27fd9715f37e33e6cdec79f1fa))
    - [git-packetline] Deduplicate read-line logic as well, with perf regression ([`1c13706`](https://github.com/Byron/gitoxide/commit/1c13706c812f5a14559fcf0b983cdf4420bb1ef5))
    - [git-packetline] refactor ([`17ab380`](https://github.com/Byron/gitoxide/commit/17ab380e552c5da56b06a8addd0d43c1b7f310fa))
    - [git-packetline] Step one towards less code duplication ([`d863de0`](https://github.com/Byron/gitoxide/commit/d863de0085ae73248f96fb8fcc4fce0a7941a7b4))
    - [git-packetline] more docs ([`4591e46`](https://github.com/Byron/gitoxide/commit/4591e4601c4fee3cb7cc37dafd02bef83441e69a))
    - (cargo-release) version 0.6.0 ([`ec5a54e`](https://github.com/Byron/gitoxide/commit/ec5a54e9f3543afddc9f972f16135edc6ef6ff5b))
    - [git-packetline] refactor ([`e5769d1`](https://github.com/Byron/gitoxide/commit/e5769d1e7668ae54c667d2593c0c22e7723710c0))
    - [git-packetline] refactor ([`fef3c9f`](https://github.com/Byron/gitoxide/commit/fef3c9f0aed3f6a509a71e8ff20050c6ea660f56))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

## v0.5.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 133 calendar days.
 - 133 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`8c4cc3f`](https://github.com/Byron/gitoxide/commit/8c4cc3fb5922d1a761463bbbad65e59f91cce4cb))
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com/Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com/Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com/Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - refactor ([`77764f3`](https://github.com/Byron/gitoxide/commit/77764f3b9c3e8202119bb9327e150089c3ecbb9b))
    - refactor ([`edf7d38`](https://github.com/Byron/gitoxide/commit/edf7d382148aa139485c8279c2a50dc6c86d481d))
    - refactor ([`ca98221`](https://github.com/Byron/gitoxide/commit/ca98221d5a512dabf683cc1da56d40a17285f2fb))
    - bump git-odb minor version ([`5c833ce`](https://github.com/Byron/gitoxide/commit/5c833ce64babd00b7ced3e3a1c9ed3dbd260c9f4))
    - (cargo-release) version 0.11.0 ([`fd698e3`](https://github.com/Byron/gitoxide/commit/fd698e334e44d5c478c162f98d09afd9ce7a6895))
    - (cargo-release) version 0.10.0 ([`3161777`](https://github.com/Byron/gitoxide/commit/316177729e42f8d000a40ab01b9b97621e7179e8))
    - (cargo-release) version 0.9.0 ([`efc8983`](https://github.com/Byron/gitoxide/commit/efc898381d830e44487c62e35a665d3ccd0a2d39))
    - (cargo-release) version 0.8.0 ([`1ccfdcd`](https://github.com/Byron/gitoxide/commit/1ccfdcdb96b59c6415e7fbc800371d594b2ef7a1))
    - thanks clippy ([`343ab9a`](https://github.com/Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
    - deny missing docs for git-packetline ([`3a78840`](https://github.com/Byron/gitoxide/commit/3a78840481c60dd122dedda090f1a235c9a21088))
</details>

## v0.4.1 (2020-12-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 9 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`7c623de`](https://github.com/Byron/gitoxide/commit/7c623dec0f62f123cdf46ae8c36d7b18cb55b00b))
    - Finish git-packetline docs ([`7ae3e73`](https://github.com/Byron/gitoxide/commit/7ae3e7391042dddb6ac33c541a020f23eee294a1))
    - last remaining docs prior to refactoring ([`da966fc`](https://github.com/Byron/gitoxide/commit/da966fcdbca656c87e34a16dcbd6e69d9488e93b))
    - docs for encode ([`213924d`](https://github.com/Byron/gitoxide/commit/213924de746871bf3152c5b8612c6b3515da1dbb))
    - docs for ReadWithSidebands ([`e277cce`](https://github.com/Byron/gitoxide/commit/e277cce4d72c4d44122019a26e45c67c682d25b5))
    - Finish `Provider` docs ([`832f7f3`](https://github.com/Byron/gitoxide/commit/832f7f3d09d7cd2e7a7e7ac2526690d2d05acdc4))
    - more docs for git-packetline ([`3c7e727`](https://github.com/Byron/gitoxide/commit/3c7e727c4d7881deb1afa0f5596935993e477ec1))
    - Some more docs for git-packetline ([`77edb62`](https://github.com/Byron/gitoxide/commit/77edb623610cc4c03b75e6f5da3af63b2604829d))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
</details>

## v0.4.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`72eaece`](https://github.com/Byron/gitoxide/commit/72eaeceed135e4cc5c943685f4c902d03597c4d2))
    - (cargo-release) version 0.6.0 ([`27f5955`](https://github.com/Byron/gitoxide/commit/27f5955e047f35e21a86789eb46bfd89e1c99b44))
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 90 calendar days.
 - 92 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`eade7d1`](https://github.com/Byron/gitoxide/commit/eade7d101e071153055b07d9c6ae3c1452493a21))
    - (cargo-release) version 0.5.0 ([`c767e07`](https://github.com/Byron/gitoxide/commit/c767e07ccfc58a28e3e8ec22b590afdf0d92b9f2))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`8930610`](https://github.com/Byron/gitoxide/commit/8930610c3ad73d2c1294880c3081f0662525f339))
</details>

## v0.2.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.1 ([`abc218c`](https://github.com/Byron/gitoxide/commit/abc218c442cea95884d8b987faf0f29fc25384b1))
    - Assure peek behaves exactly as we want it to with ERR lines ([`bbdaee5`](https://github.com/Byron/gitoxide/commit/bbdaee5ff7abe364e4eb1bcbfce7fe7068935166))
    - V1 parsing of shallow and unshallow lines… ([`8bcf535`](https://github.com/Byron/gitoxide/commit/8bcf535a8b07d9b1d53fb84c73ba55c76a318daf))
    - (cargo-release) version 0.4.0 ([`2272fa4`](https://github.com/Byron/gitoxide/commit/2272fa4bcacdaf1898e4cd8b791232fc1321227f))
</details>

## v0.2.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release over the course of 23 calendar days.
 - 24 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`da830de`](https://github.com/Byron/gitoxide/commit/da830defc9cfa81ce159f6d908da828227760845))
    - [clone] Support for reading multi-step negoritaions, but… ([`507d342`](https://github.com/Byron/gitoxide/commit/507d342dfe2a714a4dd0bc100d96ed9e64a58243))
    - thanks clippy ([`6aeb68c`](https://github.com/Byron/gitoxide/commit/6aeb68c587916610352644e0e7c4fe812957debd))
    - [clone] support for stopped_at() in provider and reader ([`6bd8c87`](https://github.com/Byron/gitoxide/commit/6bd8c8723617e70c3e9daaddf284884aacefc483))
    - [ref-ls] A way to abort on multiple delimiters; first tests work ([`8d44912`](https://github.com/Byron/gitoxide/commit/8d44912e7215b85c6931b7b829bd73ac38584424))
    - refactor ([`feec5be`](https://github.com/Byron/gitoxide/commit/feec5be335a99a4c47ba98f93803863044575838))
    - [ref-ls] Allow multiple delimiters at the same time ([`cfae63a`](https://github.com/Byron/gitoxide/commit/cfae63a5f7d2d99560dd857f7220980d70c4c4d8))
    - [ref-ls] It would be practical to simply have access to the line provider… ([`5fba787`](https://github.com/Byron/gitoxide/commit/5fba78796d3bcc16f812dc3202d521ee057e86f9))
    - [ref-ls] support for line peeking in packet line readers ([`0c0c575`](https://github.com/Byron/gitoxide/commit/0c0c57522972f2a49ed5261474114da062e6ab15))
    - [ref-ls] don't do anything on drop ([`9f18d9b`](https://github.com/Byron/gitoxide/commit/9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4))
    - fix packet-line tests ([`0939e6c`](https://github.com/Byron/gitoxide/commit/0939e6c7cf19395a8cfe09c76630dcb3614fa9d9))
    - [clone] Don't expose hex-error in public interfaces anymore ([`92dab30`](https://github.com/Byron/gitoxide/commit/92dab3033890fe26fe2b901d87abe16abd065cce))
    - refactor ([`c138059`](https://github.com/Byron/gitoxide/commit/c138059434885536984996cd8fec002aba3d5fe1))
    - refactor ([`f2ff90d`](https://github.com/Byron/gitoxide/commit/f2ff90d65edd91c4f6dc6baaf1242df31ef0ef2e))
    - [clone] a way to change progress handling on the fly ([`c1bcc0a`](https://github.com/Byron/gitoxide/commit/c1bcc0adf04a32e9332fae047fba066d4cff6538))
    - refactor ([`aceaaed`](https://github.com/Byron/gitoxide/commit/aceaaed45be5d523c9b4c1f98444b84619cbc13f))
    - refactor ([`2cdda7a`](https://github.com/Byron/gitoxide/commit/2cdda7af8ae884b5efde8861f13d85b07d643b94))
    - [clone] Sketch 'request()' implementation for git protocol ([`fd0e0e9`](https://github.com/Byron/gitoxide/commit/fd0e0e9e49f5481c14e17a462f9e507663fd5e6a))
    - [clone] the problem actually was rooted in trying to read binary data ([`b7af002`](https://github.com/Byron/gitoxide/commit/b7af002a445143e5437fe497a2d9fb1653adadae))
    - [clone] first impl of custom read-line (still fails) ([`7f2bdfa`](https://github.com/Byron/gitoxide/commit/7f2bdfa6276692557768ec7a9e969277d7f7db43))
    - [clone] Add test which probably indicates the need for a custom read_line(…) ([`2360a70`](https://github.com/Byron/gitoxide/commit/2360a7003c07baf88ad3cd46d75bc31a06341301))
    - refactor ([`359765a`](https://github.com/Byron/gitoxide/commit/359765a89042f52d41281a31a4ad854215e99c33))
    - [clone] more tests for progress line handling ([`66c2958`](https://github.com/Byron/gitoxide/commit/66c2958769797610ba415d39a050e0ffd0fb7c75))
    - [clone] decouple packet line from git-features and progress ([`13bf25e`](https://github.com/Byron/gitoxide/commit/13bf25edb64b8fd3ec77e24cce8911c020e91b11))
    - refactor ([`fb7dd26`](https://github.com/Byron/gitoxide/commit/fb7dd267f12bb23ce5c2ba275e487b90f5117208))
    - thanks clippy (what would I do without you <3) ([`631af04`](https://github.com/Byron/gitoxide/commit/631af04c87f0b6b22c3ac1ef0d300a02bbdcd217))
    - refactor ([`94f0d8a`](https://github.com/Byron/gitoxide/commit/94f0d8ab911625218728d9ba582eeed776f060ed))
    - [clone] Keep line reader around in http transport ([`feb2596`](https://github.com/Byron/gitoxide/commit/feb259645651309b31df91b18ab247d6955f9a7f))
    - [clone] packet line readers now reset the parent automatically… ([`8250448`](https://github.com/Byron/gitoxide/commit/8250448e5c441cd14dfe77bfbbdb21b5f87ebf8c))
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' ([`abf9c3b`](https://github.com/Byron/gitoxide/commit/abf9c3b3c9fe757a7418626cd985960f58718357))
    - [clone] Finally it all works exactly as desired… ([`c5bbb57`](https://github.com/Byron/gitoxide/commit/c5bbb57ad7069c839757f72432d23c43de0b61da))
    - [clone] FAIL: can't pass line reader as box ([`633341d`](https://github.com/Byron/gitoxide/commit/633341dd5f3fbd7b910c545e203e0bd734b5f989))
    - [clone] sketching how to possibly return Line readers while keeping it sane… ([`4ba123b`](https://github.com/Byron/gitoxide/commit/4ba123b8e543a2ef3ba07aaf467b208047db0e1d))
    - [clone] Add Peek support for packet line reader ([`10f1ef7`](https://github.com/Byron/gitoxide/commit/10f1ef7b9c59ec549a7c1e72cfce3dc42617b620))
    - [clone] a simpler peek version that will soon work ([`c35051b`](https://github.com/Byron/gitoxide/commit/c35051bbafe3278d6cc17e9b29cd42092fcdf03f))
    - [clone] FAIL: try to have peek_line() borrowcheck ([`dea5672`](https://github.com/Byron/gitoxide/commit/dea5672c374f95d13cf9b9629da09c51d4ff0375))
    - refactor ([`f3c5c05`](https://github.com/Byron/gitoxide/commit/f3c5c059169e9cc998ec0c80baf637142eb200ef))
    - packet line writer deals with long lines and definitely isn't smart ([`549e6e6`](https://github.com/Byron/gitoxide/commit/549e6e69e58d93efb685efa4036c8999270b8182))
    - First rough implementation of packet line writer ([`721c215`](https://github.com/Byron/gitoxide/commit/721c215ec57ca55a22ddbbfa1e4e63a7f44c6cfd))
    - Don't try to find 'ERR ' in every packet line we parse… ([`922fcb6`](https://github.com/Byron/gitoxide/commit/922fcb6d718622bdd6e157edfb47d60cf2a5d4f5))
    - thanks clippy ([`25cdbec`](https://github.com/Byron/gitoxide/commit/25cdbecb791993ffe8a3fdf59ae826fa6c63039a))
    - no panics in packet line to let caller handle invariants; read… ([`a89a443`](https://github.com/Byron/gitoxide/commit/a89a44388a353e7324bbed145ac4996bd677a15b))
    - [clone] as_read() support for packet lines ([`e214df5`](https://github.com/Byron/gitoxide/commit/e214df5c3a63c26e046cf24cfe8ec5147946b042))
    - [clone] first stab at making packet liner reader more 'practical' ([`7178543`](https://github.com/Byron/gitoxide/commit/7178543804575040a3685a31dde5515f634d21a9))
    - [clone] prepare for making progress in packet line reader optional ([`ffe84c0`](https://github.com/Byron/gitoxide/commit/ffe84c046129a12c384678c56e72f3fdfb04f550))
</details>

## v0.1.0 (2020-08-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com/Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - [clone] move packet-line code into own crate ([`879af67`](https://github.com/Byron/gitoxide/commit/879af671fcde405d3d08ddbc07ea70d0bee23ef1))
</details>

