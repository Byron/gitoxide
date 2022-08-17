# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-ac820d8a81221726ecea82a3054c364966758e26/> `Clone` for `ewah::Vec`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 25 calendar days.
 - 136 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - `Clone` for `ewah::Vec` ([`ac820d8`](https://github.com/Byron/gitoxide/commit/ac820d8a81221726ecea82a3054c364966758e26))
 * **Uncategorized**
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.1.0 (2022-04-03)

### Bug Fixes (BREAKING)

 - <csr-id-0b1543d481337ed51dcfdeb907af21f0bc98dcb9/> lower rust edition to 2018

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 73 calendar days.
 - 73 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#355](https://github.com/Byron/gitoxide/issues/355)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - lower rust edition to 2018 ([`0b1543d`](https://github.com/Byron/gitoxide/commit/0b1543d481337ed51dcfdeb907af21f0bc98dcb9))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - frame for printing index information ([`9ea98fd`](https://github.com/Byron/gitoxide/commit/9ea98fda75fbef339647a0ca03776060356d1206))
 * **[#355](https://github.com/Byron/gitoxide/issues/355)**
    - See if git-bitmap now compiles on 32 bit systems ([`0c77816`](https://github.com/Byron/gitoxide/commit/0c778166de19491f75c0a74e49da64ffb73c83be))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
</details>

## 0.0.1 (2022-01-19)

EWAH bitmaps can be decoded from disk and enabled bits can be acted upon by calling
a closure each time an index is set to true.

Please note that the current feature set is minimal and driven by the need of the
`git-index` crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
    - basic itreation of set bits ([`4796ad9`](https://github.com/Byron/gitoxide/commit/4796ad9e5d2bcf05a5cb37aec64c441aefcf49fe))
    - refactor ([`a1dc8de`](https://github.com/Byron/gitoxide/commit/a1dc8dedc5d9e1712295131d2332c21f3df4ac45))
    - Support for 'sdir' extension ([`a38c3b8`](https://github.com/Byron/gitoxide/commit/a38c3b889cfbf1447c87d489d3eb9902e757aa4b))
    - rlw field is actually an offset into `bits` when used at runtime ([`7d1aba1`](https://github.com/Byron/gitoxide/commit/7d1aba15fb8e0f927730b76ab62cedc9ef3bca1a))
    - Turn git-bitmap Array into Vec, as it will be able to adjust its size ([`9e99e01`](https://github.com/Byron/gitoxide/commit/9e99e016c17f0d5bcd2ab645261dfac58cb48be5))
    - EWAH decoding works ([`bad7e19`](https://github.com/Byron/gitoxide/commit/bad7e1938344f3918c28083c2f45936fd20de8f3))
    - first stab at decoding ewah bitmaps ([`353a53c`](https://github.com/Byron/gitoxide/commit/353a53ccab5af990e7c384b74b38e5429417d449))
 * **Uncategorized**
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - thanks clippy ([`93c3d23`](https://github.com/Byron/gitoxide/commit/93c3d23d255a02d65b5404c2f62f96d94e36f33d))
    - Fix index without extension test & thanks clippy ([`066464d`](https://github.com/Byron/gitoxide/commit/066464d2ad2833012fa196fe41e93a54ab05457f))
</details>

## 0.0.0 (2022-01-12)

Initial release, entirely empty.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#293](https://github.com/Byron/gitoxide/issues/293)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - git-bitmap - changelog ([`339318c`](https://github.com/Byron/gitoxide/commit/339318c072928b0d683a3746ea9e5c18e485dbbd))
    - Add git-bitmap crate for use in git-index ([`a517f26`](https://github.com/Byron/gitoxide/commit/a517f2697678f31e29ec9982d02ccfec6a777bbf))
 * **Uncategorized**
    - Release git-bitmap v0.0.0 ([`0c98247`](https://github.com/Byron/gitoxide/commit/0c98247a2e069e2ee5061f1d19532481758cd9fb))
</details>

