# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-d0fab1e7f083088f607365ceec056e6e521cbdcc/> new `file::Index::highest_offset()` method
   With it it's simpler to figure out from where to read trailing
   checksums.

### New Features (BREAKING)

 - <csr-id-9d9f2ee55202788910cd955cdcc08196d18f2cf5/> Use `[u8;4]` as chunk id
   This allows to remove the additional string to describe the ids, which
   are usually ascii anyway.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Use `[u8;4]` as chunk id ([`9d9f2ee`](https://github.com/Byron/gitoxide/commit/9d9f2ee55202788910cd955cdcc08196d18f2cf5))
    - new file::Index::highest_offset() method ([`d0fab1e`](https://github.com/Byron/gitoxide/commit/d0fab1e7f083088f607365ceec056e6e521cbdcc))
    - refactor ([`7a9e628`](https://github.com/Byron/gitoxide/commit/7a9e628725c927d4fed8ef70e96ca2b802195bff))
    - remove unnecessary test dependencies ([`463afcc`](https://github.com/Byron/gitoxide/commit/463afcc71419ce73719720192424bf5a6d37c69a))
</details>

## 0.1.0 (2021-12-20)

Initial release with enough functionality to handle multi-pack indices and commitgraph files.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - update changelog prior to release ([`6ae49e3`](https://github.com/Byron/gitoxide/commit/6ae49e39b2251ad70b72a8f3b3840ebb9334ffd9))
    - remove empty tests ([`e30dcea`](https://github.com/Byron/gitoxide/commit/e30dcea6ca56b7bea175be11868e924317ab9974))
    - read and validate fanout chunk ([`3ca04e3`](https://github.com/Byron/gitoxide/commit/3ca04e355a413975e55adf8b204d6962a9341d32))
    - Read all mandatory and optional chunks ([`99023bb`](https://github.com/Byron/gitoxide/commit/99023bbde027be82e9217868df7f73ecd09bf705))
    - Load chunk index of midx file ([`fac8efa`](https://github.com/Byron/gitoxide/commit/fac8efacb31935c2143717ebe82003a0916f233f))
    - frame for git-chunk crate to share among git-pack and git-commitgraph ([`b2d2ae2`](https://github.com/Byron/gitoxide/commit/b2d2ae221d43cc14aa169ada3c471e2bd2adadf4))
 * **Uncategorized**
    - Release git-chunk v0.1.0 ([`544f4a9`](https://github.com/Byron/gitoxide/commit/544f4a9c694e96236a4c7fe8b68fdfc229d76f25))
    - thanks clippy ([`35cf46f`](https://github.com/Byron/gitoxide/commit/35cf46f87ecc42cf033ca001acf1b5918b3fea1b))
</details>

