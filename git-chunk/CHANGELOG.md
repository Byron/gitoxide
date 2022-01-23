# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 (2022-01-23)

### New Features

 - <csr-id-389fea2addc94801e73a521b2ac9a8529e4fbb3d/> Facilities to write chunk files
   This includes utilities to plan chunks for writing the table of
   contents, and to get a hand when actually writing the chunks themselves
   while assuring they are written into the correct spot.
 - <csr-id-373a85003e0b293666283ef742762b13e1211fc5/> add `file::Index::size_for_entries()` const fn
   This is useful for min-size checks of files that are to be loaded.
 - <csr-id-e14096e0c19689a1e2480b484537b2a0ffbfc3b9/> add `file::Index::usize_offset_by_id()` and `range::into_usize_or_panic()`

### Changed (BREAKING)

 - <csr-id-11b0f4afc993fe1eb1316839baaa15187483d420/> `file::Index::chunks` is not public anymore
   The internal state is validated, and to assure that it can't be publicly
   accessible.
 - <csr-id-9328015c653cae582882c346965b51d7cf6b9d08/> rename `into_usize_range()` to `range::into_usize()`
 - <csr-id-dda26a4ddd7cc195c10b79a7f428d6298285197b/> rename `Kind` to `Id`
   This is more in line with gits terminology and feels quite natural to
   use as well.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 33 calendar days.
 - 34 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - write progress for multi-pack writing ([`1bea1d4`](https://github.com/Byron/gitoxide/commit/1bea1d47908d3ec44c83b2e39a5b67134ad51ee0))
    - Facilities to write chunk files ([`389fea2`](https://github.com/Byron/gitoxide/commit/389fea2addc94801e73a521b2ac9a8529e4fbb3d))
    - multi-pack index writing complete with large-offset support ([`f7d5c7f`](https://github.com/Byron/gitoxide/commit/f7d5c7f815dbf52c668444b316ae2e1485463bcb))
    - fix docs ([`b61a920`](https://github.com/Byron/gitoxide/commit/b61a9200d267865be76bdd2f36477c3940bc4dcc))
    - Writing of chunk index ([`17a93c3`](https://github.com/Byron/gitoxide/commit/17a93c3f072c4e3a9a28cf8b11e44e065232b293))
    - Sketch all the chunk-write API and use it from multi-index write ([`5457761`](https://github.com/Byron/gitoxide/commit/545776180f75cba87f7119f9bd862d39f081f1bd))
    - `file::Index::chunks` is not public anymore ([`11b0f4a`](https://github.com/Byron/gitoxide/commit/11b0f4afc993fe1eb1316839baaa15187483d420))
    - cargo fmt ([`8b9da35`](https://github.com/Byron/gitoxide/commit/8b9da35b3e0d3458efcac150f7062c9d7382a6c4))
    - fix docs ([`cd981e2`](https://github.com/Byron/gitoxide/commit/cd981e222af237c47fcfb74258de8fdfc04dfc1b))
    - add `file::Index::size_for_entries()` const fn ([`373a850`](https://github.com/Byron/gitoxide/commit/373a85003e0b293666283ef742762b13e1211fc5))
    - refactor ([`8b8b4c5`](https://github.com/Byron/gitoxide/commit/8b8b4c538823fb4d2c37be80340d843080f08d19))
    - add `file::Index::usize_offset_by_id()` and `range::into_usize_or_panic()` ([`e14096e`](https://github.com/Byron/gitoxide/commit/e14096e0c19689a1e2480b484537b2a0ffbfc3b9))
    - rename `into_usize_range()` to `range::into_usize()` ([`9328015`](https://github.com/Byron/gitoxide/commit/9328015c653cae582882c346965b51d7cf6b9d08))
    - rename `Kind` to `Id` ([`dda26a4`](https://github.com/Byron/gitoxide/commit/dda26a4ddd7cc195c10b79a7f428d6298285197b))
 * **Uncategorized**
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
</details>

## 0.2.0 (2021-12-20)

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

 - 6 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - update changelog ([`099f055`](https://github.com/Byron/gitoxide/commit/099f0559f835e5893cfc91d6d07216ed65db0790))
    - Use `[u8;4]` as chunk id ([`9d9f2ee`](https://github.com/Byron/gitoxide/commit/9d9f2ee55202788910cd955cdcc08196d18f2cf5))
    - new file::Index::highest_offset() method ([`d0fab1e`](https://github.com/Byron/gitoxide/commit/d0fab1e7f083088f607365ceec056e6e521cbdcc))
    - refactor ([`7a9e628`](https://github.com/Byron/gitoxide/commit/7a9e628725c927d4fed8ef70e96ca2b802195bff))
    - remove unnecessary test dependencies ([`463afcc`](https://github.com/Byron/gitoxide/commit/463afcc71419ce73719720192424bf5a6d37c69a))
 * **Uncategorized**
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
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

