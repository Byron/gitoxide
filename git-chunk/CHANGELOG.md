# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.1 (2023-02-09)

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 79 calendar days.
 - 79 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#691](https://github.com/Byron/gitoxide/issues/691)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.4.0 (2022-11-21)

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

 - 5 commits contributed to the release over the course of 62 calendar days.
 - 62 days passed between releases.
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
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.3.2 (2022-09-20)

Maintenance release without observable changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - replace `quick-error` with `thiserror` ([`bc45906`](https://github.com/Byron/gitoxide/commit/bc45906ea38adb82a7179cb6b92f7bc34b7e0371))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.3.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 212 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
</details>

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

 - 19 commits contributed to the release over the course of 33 calendar days.
 - 34 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
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
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
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

