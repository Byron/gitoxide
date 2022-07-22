# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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

 - 1 commit contributed to the release.
 - 39 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#331](https://github.com/Byron/gitoxide/issues/331)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - remove local-time-support feature toggle. ([`89a41bf`](https://github.com/Byron/gitoxide/commit/89a41bf2b37db29b9983b4e5492cfd67ed490b23))
</details>

## 0.10.1 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 25 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Replace `Time` with `git-date::Time`. ([`59b3ff8`](https://github.com/Byron/gitoxide/commit/59b3ff8a7e028962917cf3b2930b5b7e5156c302))
 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
</details>

## 0.10.0 (2022-05-18)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 36 calendar days.
 - 45 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
</details>

## 0.9.0 (2022-04-03)

### New Features

 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-70a259c11f12f55a5f26b02cac21ec000c76fb8b/> `Time::seconds()` shortcut
 - <csr-id-705adfd5a5cbec0498a3d67065f7296c0dab8337/> SignatureRef is now Copy
   It's a type with only copyable types inside, so should be copy itself.
   This makes it less awkward to use as well.
 - <csr-id-13799e200508dc67ea4fe6f3c97c47b50694cada/> Time::new(seconds_since_epoch, offset)
 - <csr-id-77ef2cb819f21ddc5d1ee9e94b5961e3ca5b3139/> `Time::default()`

### Changed (BREAKING)

 - <csr-id-5c8b0a44acfa708ef4ffe28cfde0dfed52b29d7c/> `Time::time` -> `Time::seconds_since_unix_epoch`
   And `Time::offset` to `Time::offset_in_seconds`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 55 calendar days.
 - 55 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - sort parents by most recent to find recent tags first ([`d240740`](https://github.com/Byron/gitoxide/commit/d240740cd24bdd8ded1d9048e2861b88476dbbe1))
    - `Time::time` -> `Time::seconds_since_unix_epoch` ([`5c8b0a4`](https://github.com/Byron/gitoxide/commit/5c8b0a44acfa708ef4ffe28cfde0dfed52b29d7c))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - support for trimming of whitespace around name and email ([`a39bf71`](https://github.com/Byron/gitoxide/commit/a39bf71531ee0a6c8db082758d3212c805ce2bf0))
    - `Time::seconds()` shortcut ([`70a259c`](https://github.com/Byron/gitoxide/commit/70a259c11f12f55a5f26b02cac21ec000c76fb8b))
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
    - SignatureRef is now Copy ([`705adfd`](https://github.com/Byron/gitoxide/commit/705adfd5a5cbec0498a3d67065f7296c0dab8337))
    - Time::new(seconds_since_epoch, offset) ([`13799e2`](https://github.com/Byron/gitoxide/commit/13799e200508dc67ea4fe6f3c97c47b50694cada))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - `Time::default()` ([`77ef2cb`](https://github.com/Byron/gitoxide/commit/77ef2cb819f21ddc5d1ee9e94b5961e3ca5b3139))
 * **Uncategorized**
    - Release git-actor v0.9.0, git-object v0.18.0 ([`ef9242b`](https://github.com/Byron/gitoxide/commit/ef9242bdb35c02afc36af7c59073d78091fbf504))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
</details>

## 0.8.1 (2022-02-06)

### New Features

 - <csr-id-f99851bb272ce2d81704712b9e70edaddc442589/> keep feature documentation inline with manifests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 12 calendar days.
 - 13 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#329](https://github.com/Byron/gitoxide/issues/329)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - keep feature documentation inline with manifests ([`f99851b`](https://github.com/Byron/gitoxide/commit/f99851bb272ce2d81704712b9e70edaddc442589))
 * **Uncategorized**
    - Release git-actor v0.8.1 ([`08fe550`](https://github.com/Byron/gitoxide/commit/08fe5508472f2eb209db8a5fc4e4942a9d7db93d))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
</details>

## 0.8.0 (2022-01-23)

A maintenance release thanks to upgraded dependencies.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 35 calendar days.
 - 55 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - upgrade git-actor dependencies ([`82bb1c0`](https://github.com/Byron/gitoxide/commit/82bb1c0ee622db073805126f9e62cbc91820ccf6))
</details>

## 0.7.0 (2021-11-29)

<csr-id-598698b88c194bc0e6ef69539f9fa7246ebfab70/>

Maintenance release due, which isn't really required but one now has to be careful what's committed at once.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 25 calendar days.
 - 40 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#250](https://github.com/Byron/gitoxide/issues/250)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#250](https://github.com/Byron/gitoxide/issues/250)**
    - move loose header manipulation from git-pack to git-object ([`598698b`](https://github.com/Byron/gitoxide/commit/598698b88c194bc0e6ef69539f9fa7246ebfab70))
 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Move "loose object header" ser/de to git-object ([`3d1565a`](https://github.com/Byron/gitoxide/commit/3d1565acfc336baf6487edccefd72d0226141a08))
</details>

## v0.6.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.5.3 (2021-10-15)

This release contains no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 36 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
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

## v0.5.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.5.2 ([`32a8fde`](https://github.com/Byron/gitoxide/commit/32a8fde43ac3db3486710c3f01df07b664fcb9b0))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com/Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [object #164] Allow referenced objects to be serialized as well ([`a98d298`](https://github.com/Byron/gitoxide/commit/a98d2985dae2259d72bb91a01548906862fee9f7))
</details>

## v0.5.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 10 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.5.1 ([`0758045`](https://github.com/Byron/gitoxide/commit/0758045a43d15238eb6537fb3b60d2c1fdf7674e))
    - [repository #190] produce nice reflog messages ([`e7a8b62`](https://github.com/Byron/gitoxide/commit/e7a8b62eb24f840f639aa436b4e79a4a567d3d05))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com/Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com/Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com/Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [actor #190] methods to get an actor signature at the current time ([`6d0bedd`](https://github.com/Byron/gitoxide/commit/6d0beddb20092a80b113a39c862d6b680d79deb6))
</details>

## v0.5.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com/Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #175] refactor ([`ec88c59`](https://github.com/Byron/gitoxide/commit/ec88c5905194150cc94db4d4c20e9f4e2f6595c3))
</details>

## v0.4.0 (2021-08-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com/Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] refactor ([`08a1849`](https://github.com/Byron/gitoxide/commit/08a18498d62f1d5bdabbb4712b08f3d17d63e16c))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com/Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
</details>

## v0.3.3 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.3.3 ([`3ead949`](https://github.com/Byron/gitoxide/commit/3ead9498db4168fb93f857324224c7dce340bc29))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.3.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.3.2 ([`8f96eca`](https://github.com/Byron/gitoxide/commit/8f96ecae72e6363f1edacde0d3b861836d1c5730))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.3.1 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor-0.3.1 ([`727087d`](https://github.com/Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - [utils #154] commit manifest changes; create tags ([`95dcd9d`](https://github.com/Byron/gitoxide/commit/95dcd9d7d060101596c51116218102cc8049d0dd))
</details>

## v0.3.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com/Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
</details>

## v0.2.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 45 calendar days.
 - 46 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com/Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com/Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com/Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [ref] log line writing ([`3da8fcf`](https://github.com/Byron/gitoxide/commit/3da8fcf0bfb77b80c06a3358416f10d6f393db8b))
    - Merge branch 'negotiate-fallible' ([`27c8abe`](https://github.com/Byron/gitoxide/commit/27c8abe1948bc10c779efa33d4bc0b92741f6444))
    - [actor] refactor ([`bccb738`](https://github.com/Byron/gitoxide/commit/bccb738edfc2e6923643a2e73f93b6acfdd7cf5c))
    - [actor] don't leak btoi errors… ([`e6c7fc1`](https://github.com/Byron/gitoxide/commit/e6c7fc18954a5a5ad12b3da6c290f8cb9a74c19c))
    - [actor] FAIL an attempt to remove btoi errors ([`3f99cf5`](https://github.com/Byron/gitoxide/commit/3f99cf531caacb93a3ce81b16d61be18e5d8a017))
    - [actor] pure nom error handling… ([`78cbe18`](https://github.com/Byron/gitoxide/commit/78cbe18888ec654f3410fc655d9beaaf63f68003))
    - [object] refactor ([`1ddb5c0`](https://github.com/Byron/gitoxide/commit/1ddb5c07b75aa2b9a9536125fbba1fc862b7fe34))
    - [actor] make signature parsing public, exposing nom :/ ([`a627972`](https://github.com/Byron/gitoxide/commit/a627972ecc53d38210c826f851ea9c5fec17b9cb))
    - [actor] cleanup error interaction with nom… ([`2dd7197`](https://github.com/Byron/gitoxide/commit/2dd7197248d58a7a89f5ff368072c511fce127e3))
</details>

## v0.1.1 (2021-06-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`e9cdc95`](https://github.com/Byron/gitoxide/commit/e9cdc958e7ce2290e2d7958cdb5aa9323ef35d37))
    - [actor] fix dependencies ([`3ff918e`](https://github.com/Byron/gitoxide/commit/3ff918efa0b94dd20f781a3d038a0449cd9c7a59))
</details>

## v0.1.0 (2021-06-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - thanks clippy ([`94fb007`](https://github.com/Byron/gitoxide/commit/94fb0071193b0e3428fd1747422ea1675dd5974b))
    - [actor] refactor ([`986d09a`](https://github.com/Byron/gitoxide/commit/986d09a4c894966bc7d918c7aaf7da6e211bfbbd))
    - [actor] refactor ([`591a741`](https://github.com/Byron/gitoxide/commit/591a74153b3fbbe6ffdc2dc06834f581dc632b3e))
    - [refs] git-actor crate to share types between git-ref and git-object ([`13edbf7`](https://github.com/Byron/gitoxide/commit/13edbf7c5d7668991cad6c49b0bbd3e396a267c4))
</details>

