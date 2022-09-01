# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Other

 - <csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/> test `is_git` on macOS exfat
 - <csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/> allow opening repos on macos exfat

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 8 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor ([`2cd1c00`](https://github.com/Byron/gitoxide/commit/2cd1c007330749f69fc1f154ba1a6e7a0f6e23d0))
    - test `is_git` on macOS exfat ([`7057ad2`](https://github.com/Byron/gitoxide/commit/7057ad2251f76756a5b35b6bbba10ab9b6601226))
    - allow opening repos on macos exfat ([`08d0c27`](https://github.com/Byron/gitoxide/commit/08d0c27b0641526aa8b70ee5aadd3658862bef1b))
</details>

## 0.4.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
</details>

## 0.4.0 (2022-08-17)

### New Features

 - <csr-id-2e015a3874ab4db40449ec4b8ffb126d8e1a22a4/> add `is_submodule_git_dir()`
 - <csr-id-4a3e1cfbc537963b851c66754e7724875b49b1f4/> `is_git()` now detects submodule worktrees correctly.
 - <csr-id-aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f/> `git_discover::is_git()` can detect submodule dirs correctly enough.
   We currently detect them as possibly bare, which could be improved if we
   allow ourselves to see `.git/modules` as `submodule` always.

### Changed (BREAKING)

 - <csr-id-1b0ef1856f542e5dc23310be93c2b39a9d84cb80/> Provide more details when classifying submodule directories.
   That way we can avoid feeding `.git` files to `git-repository::open()`
   and avoid work duplication, which ultimately allows to open submodules
   directories of all kinds.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 25 calendar days.
 - 26 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#427](https://github.com/Byron/gitoxide/issues/427), [#482](https://github.com/Byron/gitoxide/issues/482)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **[#482](https://github.com/Byron/gitoxide/issues/482)**
    - add `is_submodule_git_dir()` ([`2e015a3`](https://github.com/Byron/gitoxide/commit/2e015a3874ab4db40449ec4b8ffb126d8e1a22a4))
    - Query the `cwd` only once instead of potentially multiple times, allocating a Vec each time. ([`6be38f2`](https://github.com/Byron/gitoxide/commit/6be38f294087f8c5e9bb8522bbf928a761c5cdf0))
    - Provide more details when classifying submodule directories. ([`1b0ef18`](https://github.com/Byron/gitoxide/commit/1b0ef1856f542e5dc23310be93c2b39a9d84cb80))
    - fix windows build ([`ff488fa`](https://github.com/Byron/gitoxide/commit/ff488fac628b95ec8f64b136a125913b7b96d3a6))
    - fix docs ([`3724f31`](https://github.com/Byron/gitoxide/commit/3724f31c0563d79a0cad88d773fa60ea21f504b8))
    - `is_git()` now detects submodule worktrees correctly. ([`4a3e1cf`](https://github.com/Byron/gitoxide/commit/4a3e1cfbc537963b851c66754e7724875b49b1f4))
    - refactor ([`1ee9918`](https://github.com/Byron/gitoxide/commit/1ee991847a5adeaaeb6e80ae626c28b0ba89e0af))
    - `git_discover::is_git()` can detect submodule dirs correctly enough. ([`aa6fd97`](https://github.com/Byron/gitoxide/commit/aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f))
    - another test indicating git-discover misclassifies `.git/modules` as worktree ([`9133141`](https://github.com/Byron/gitoxide/commit/91331416146afb9e868e0868b47097865d2d7117))
    - Failing test to show discovery skips submodules entirely ([`b56b57c`](https://github.com/Byron/gitoxide/commit/b56b57c4111512853e42397ee1fdfd4c11cadf75))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - thanks clippy ([`2135fb8`](https://github.com/Byron/gitoxide/commit/2135fb80a126afc6d95b9eaa9f1cd32acb46615b))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - Merge branch 'main' into rev-parse-delegate ([`4ae2bed`](https://github.com/Byron/gitoxide/commit/4ae2bedfc25d1881d58ebdc54aca0936c68d4859))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
</details>

## 0.3.0 (2022-07-22)

### New Features

 - <csr-id-010350180459aec41132c960ddafc7b81dd9c04d/> add `DOT_GIT_DIR` constant, containing the name ".git".

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 36 calendar days.
 - 39 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - add `DOT_GIT_DIR` constant, containing the name ".git". ([`0103501`](https://github.com/Byron/gitoxide/commit/010350180459aec41132c960ddafc7b81dd9c04d))
    - adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Turn on performance mode for sha-1 computation ([`44371a1`](https://github.com/Byron/gitoxide/commit/44371a10f464f32db346aa6b8309e983cfa20933))
 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Remove another special case on windows due to canonicalize() ([`61abb0b`](https://github.com/Byron/gitoxide/commit/61abb0b006292d2122784b032e198cc716fb7b92))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.2.0 (2022-06-13)

<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>

### Other

 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 16 calendar days.
 - 16 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - refactor ([`ec37cb8`](https://github.com/Byron/gitoxide/commit/ec37cb8005fa272aed2e23e65adc291875b1fd68))
    - refactor ([`b27a8c2`](https://github.com/Byron/gitoxide/commit/b27a8c243cdc14730478c2a94cafdc8ccf5c60d3))
    - refactor ([`06e96a4`](https://github.com/Byron/gitoxide/commit/06e96a435d820a1ef1e567bf93e7b9ca5fa74829))
    - refactor ([`b555bda`](https://github.com/Byron/gitoxide/commit/b555bdae9964628b6d43e00dd7d7ee8fe674d309))
    - Unify the way `dir_made_absolute` is computed ([`48417fc`](https://github.com/Byron/gitoxide/commit/48417fc8d8d3ec0005ef7b6ef35ced09d92282c2))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - refactor ([`7b307f5`](https://github.com/Byron/gitoxide/commit/7b307f5acb7214f4ff674e00167933cdcccc353a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - avoid shortening absolute paths ([`6106521`](https://github.com/Byron/gitoxide/commit/6106521581029c5c24b23a47bb91c1921edfa0af))
    - Add discovery opt env-overrides & env discovery helpers ([`e521d39`](https://github.com/Byron/gitoxide/commit/e521d39e1b0f4849280bae1527bf28977eec5093))
</details>

## 0.1.3 (2022-05-27)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#426](https://github.com/Byron/gitoxide/issues/426), [#429](https://github.com/Byron/gitoxide/issues/429)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#426](https://github.com/Byron/gitoxide/issues/426)**
    - allow tests to deal with shared drives by adjusting trust expectation ([`85ab096`](https://github.com/Byron/gitoxide/commit/85ab0964a1a35556a5efe392576c9b028e601c6c))
    - cleanup ([`e029eb4`](https://github.com/Byron/gitoxide/commit/e029eb4589942e06a53bc0f6ed472d1d33c4a7e6))
 * **[#429](https://github.com/Byron/gitoxide/issues/429)**
    - Adjust changelogs prior to release ([`7397805`](https://github.com/Byron/gitoxide/commit/7397805fd032a752d6c2f2c2c28ac11ddecc7193))
 * **Uncategorized**
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
</details>

## 0.1.2 (2022-05-23)

<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>

### Features

- `discover` now avoid crossing file-system boundaries by default on unix.

### Other

 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - adopt git-for-windows exception rules ([`136eb37`](https://github.com/Byron/gitoxide/commit/136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - refactor ([`aac5169`](https://github.com/Byron/gitoxide/commit/aac5169aa7c753eb3510e61bc01b47a4b7b01c6f))
    - use `defer` to make dmg unmounting more reliable ([`dbc5caa`](https://github.com/Byron/gitoxide/commit/dbc5caa99b050ac7a96cd5e7a73786072c695530))
    - refactor ([`886e26e`](https://github.com/Byron/gitoxide/commit/886e26ee686776ad4caf1864f24cb358432d5c49))
    - Assure cross-fs check doesn't break if cursor is currently empty ([`4546889`](https://github.com/Byron/gitoxide/commit/4546889adb87c0d9d1c5d22785240033dc3b0dc2))
</details>

## 0.1.1 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### Other

 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - add cross_fs option to upwards discovery ([`277c41f`](https://github.com/Byron/gitoxide/commit/277c41f1185398e14f92247aada8422f6b08afd1))
    - Re-enable discovery test on windows thanks to use of `realpath()` ([`1f4ae9e`](https://github.com/Byron/gitoxide/commit/1f4ae9e74843d108b261cbfe10e158f74286088a))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - Fix windows test failure due to //? by ignoring it there. ([`c5fd322`](https://github.com/Byron/gitoxide/commit/c5fd3223db73ee61844477aff0e8a2438d9b2e39))
    - fix windows tests (broke thanks to \\?\), maybe ([`e458b59`](https://github.com/Byron/gitoxide/commit/e458b5946ebf01d9f901769b8547df85ef14afcb))
    - Assure ceiling dirs are comparable after absolutize ([`32a157b`](https://github.com/Byron/gitoxide/commit/32a157b513191c60795924765825dc7dfb0b38c1))
    - thanks clippy ([`4979d20`](https://github.com/Byron/gitoxide/commit/4979d2071cbd1a98f3d81aacd60dd99f07d3f746))
    - refactor ([`895b772`](https://github.com/Byron/gitoxide/commit/895b772b5855818ad2227cac8dda0be00f9d5189))
    - Control if at least one ceiling dir has to match using an option ([`ca1f3eb`](https://github.com/Byron/gitoxide/commit/ca1f3ebb1306075767597c75bb288a3a0b4ebb41))
    - Revert "remove implicit canonicalization and improve tests" ([`821f3f3`](https://github.com/Byron/gitoxide/commit/821f3f383a21ddfd274cb6bdcd2228717cd21942))
    - refactor ([`6bbc53b`](https://github.com/Byron/gitoxide/commit/6bbc53be867e5292a98c422526185c6a4736a6a5))
    - fix `special_relative_base` test ([`ae226ba`](https://github.com/Byron/gitoxide/commit/ae226ba9c08b04621ce3b42e6972e102d8af2b9c))
    - remove implicit canonicalization and improve tests ([`cdbb4c9`](https://github.com/Byron/gitoxide/commit/cdbb4c9db56ef6eb258a9a76691614d4b45c8d7a))
    - refactor ([`429446c`](https://github.com/Byron/gitoxide/commit/429446c87473dca98bf802fa3de020cb58625f63))
    - refactor ([`9673aae`](https://github.com/Byron/gitoxide/commit/9673aae3da57bc12d45389de3034ab0c2c1c27ab))
    - some more assertions and ceiling dirs ([`a30bcb8`](https://github.com/Byron/gitoxide/commit/a30bcb82413103c69827151f25be0b14b1f04f37))
    - add ceiling_dirs option to upwards discovery ([`e63e722`](https://github.com/Byron/gitoxide/commit/e63e722791a7795cd99048bed834459595c60abc))
    - declare `git-discover` usable as it's fully documented ([`e439015`](https://github.com/Byron/gitoxide/commit/e439015288dad806494e0a2ed8d44fb2247de372))
</details>

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release over the course of 12 calendar days.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - a test to assure non-existing worktree directories don't hinder discovery ([`131481c`](https://github.com/Byron/gitoxide/commit/131481cac26959d46f62d40adcfc895faa4a1698))
    - allow discovery of linked worktree git dirs ([`050f795`](https://github.com/Byron/gitoxide/commit/050f795bfb0fe11655cd7e45c10d87c89ba82625))
    - path shortening now calculates the actual path length, not just component count ([`c0d24e0`](https://github.com/Byron/gitoxide/commit/c0d24e0fb422f990213606d781ba056c238bd70b))
    - adjust to changes in git-path ([`a627ddc`](https://github.com/Byron/gitoxide/commit/a627ddc48be034f8a42d86edf2a9295299297d59))
    - adapt to changes in git-path ([`c258d77`](https://github.com/Byron/gitoxide/commit/c258d77660755559eaa5389bfabe0cebdf478a80))
    - refactor ([`81a542f`](https://github.com/Byron/gitoxide/commit/81a542fbb680a19322877cf3740ae2ae66b95011))
    - rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Another test to assure invalid worktree dirs aren't git dirs ([`e6e3608`](https://github.com/Byron/gitoxide/commit/e6e3608490bda2577c23f93d905a51c33fa4f777))
    - adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - fix tests on windows ([`8080ad2`](https://github.com/Byron/gitoxide/commit/8080ad2baedf83237a0d9a01550582fc8592e568))
    - better tests for worktree discovery ([`9d0f134`](https://github.com/Byron/gitoxide/commit/9d0f134b3186635cbc253464fc7cec2ff522a4e2))
    - The first working version of worktree detection ([`dc73e96`](https://github.com/Byron/gitoxide/commit/dc73e96fdd21713083d9fe0464d1a5689405c116))
    - fix windows tests ([`b025008`](https://github.com/Byron/gitoxide/commit/b0250080c5a1e8ba3fbdb3f70038521b75634306))
    - refactor ([`6b73e0d`](https://github.com/Byron/gitoxide/commit/6b73e0dc60488fcadf21366322e75f711bada6f4))
    - support for gitdir file parsing with all of gits safety ([`1223fc7`](https://github.com/Byron/gitoxide/commit/1223fc7777f57d73c3693407f6a1e41393919a99))
    - basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - fix test on windows maybe ([`9583048`](https://github.com/Byron/gitoxide/commit/95830480928421de1e38c99a7e9a65629ec1cc41))
    - Try to fix winodws build ([`f2c5ed3`](https://github.com/Byron/gitoxide/commit/f2c5ed3326a3076cc25afbaeec9a540d823e0606))
    - refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - refactor ([`eead214`](https://github.com/Byron/gitoxide/commit/eead2144e10b96e997cd702cb651f4c92693df95))
    - avoid running failing test on _any_ windows ([`9351a3d`](https://github.com/Byron/gitoxide/commit/9351a3db2a7e9da13be3071c1d2d5aa0cb7a35b6))
    - Fix tests on windows maybe ([`8084581`](https://github.com/Byron/gitoxide/commit/8084581619c7261f779ae42856be756eac7fbfa4))
    - don't archive repositories with worktrees ([`389ef66`](https://github.com/Byron/gitoxide/commit/389ef663ce0417f3840f90dedfc91a57eae6fd0c))
    - prepare test to provide worktrees of different kinds as well ([`a4cec4a`](https://github.com/Byron/gitoxide/commit/a4cec4ab8ba80a8f439b283d369d48cdafbe7b74))
    - Add required support for reading path files ([`bc08511`](https://github.com/Byron/gitoxide/commit/bc085116241e8824c623efc9893ded220fe9a5a2))
    - Remove handling of environment variables ([`4e914f8`](https://github.com/Byron/gitoxide/commit/4e914f89bbf04dbe55eae01f7be09899296ad8b0))
    - Add git-discover to check-size ([`85b29a6`](https://github.com/Byron/gitoxide/commit/85b29a6ba6b3ee4b925fd10c688b2aabcf4273f2))
    - fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - Migrate all relevant code from git-repository to git-discover ([`631e70e`](https://github.com/Byron/gitoxide/commit/631e70e6f210df40eb789023970ec17095ec3556))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - thanks clippy ([`7617da0`](https://github.com/Byron/gitoxide/commit/7617da002ef5906f858c0cc2c349f442bafd6239))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - thanks clippy ([`3c9da80`](https://github.com/Byron/gitoxide/commit/3c9da80a39bc6638a38868f71d80e2a8ad337a4a))
</details>

## 0.0.0 (2022-05-06)

The first release, an empty crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Add empty git-discover crate ([`6565f16`](https://github.com/Byron/gitoxide/commit/6565f163ba464f5adf63d214143a4e12f48b00fc))
 * **Uncategorized**
    - Release git-discover v0.0.0 ([`2b5cf2b`](https://github.com/Byron/gitoxide/commit/2b5cf2b95a723d885584ac967cf956c28ea35f47))
</details>

