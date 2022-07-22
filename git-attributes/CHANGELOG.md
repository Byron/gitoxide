# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.2.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 16 calendar days.
 - 25 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - branch start, upgrade to compact_str v0.4 ([`b2f56d5`](https://github.com/Byron/gitoxide/commit/b2f56d5a279dae745d9c2c80ebe599c00e72c0d7))
</details>

## 0.1.0 (2022-05-18)

### Changed (BREAKING)

 - <csr-id-120d9085c35ac72d4b83daee7f2cb59fde91890e/> use git-glob crate for pattern parsing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 79 commits contributed to the release over the course of 61 calendar days.
 - 62 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - adjustments to go along with changes in git-features ([`c55cac6`](https://github.com/Byron/gitoxide/commit/c55cac6a1ada77619bb5723717a5a6d757499fa9))
    - add option to not follow symlinks when reading attribute files ([`5d619e6`](https://github.com/Byron/gitoxide/commit/5d619e66d48cf955958a0e844e832ed59756124f))
    - First primitive ignore pattern test works ([`0424136`](https://github.com/Byron/gitoxide/commit/04241367e8ce99ce6c7583d5dac4955fad3d6542))
    - re-export `git-glob` as its `Case` type is part of the public API ([`4b72045`](https://github.com/Byron/gitoxide/commit/4b7204516a7c61162a2940eb66e8a7c64bf78ce7))
    - Sketch how attribute states can be used ([`d80b321`](https://github.com/Byron/gitoxide/commit/d80b321cf1863ad4436e69c0ee436f628f72531a))
    - also skip negative attribute patterns ([`04ab5d3`](https://github.com/Byron/gitoxide/commit/04ab5d3ed351dc0d0b64226a3d8710ae8b522b70))
    - Allow basename matches to work like before ([`4f6cefc`](https://github.com/Byron/gitoxide/commit/4f6cefc96bea5f116eb26a9de8095271fd0f58e2))
    - adapt to changes in git-glob and add failing test ([`cd58a1c`](https://github.com/Byron/gitoxide/commit/cd58a1c3445f97fd73d68e9f6b0af988806bea0d))
    - refactor ([`fe9fb4c`](https://github.com/Byron/gitoxide/commit/fe9fb4cbab2def0d85fb1d961b911d7f7e62dbcc))
    - Fix crate release size by adding includes ([`f06f666`](https://github.com/Byron/gitoxide/commit/f06f666a8919a1a844a18aed8895a824534f1ae9))
    - try using compatct_str for attribute storage ([`50b8c64`](https://github.com/Byron/gitoxide/commit/50b8c647c85793bd82dab1ac5bf6882884c2d11c))
    - generalize parsing of paths in pattern lists ([`f66c27e`](https://github.com/Byron/gitoxide/commit/f66c27e41670d0702e78739bed5ca8575d22cf1a))
    - support for loading per-directory pattern lists as well ([`457c921`](https://github.com/Byron/gitoxide/commit/457c921ef96c0d276e287009d3f8292ba1bface9))
    - more pendantic baseline parsing ([`99c7b5f`](https://github.com/Byron/gitoxide/commit/99c7b5fee5f95d9840238eb96077f3b4af5df7b8))
    - first succeding tests for global repository excludes ([`4a1e797`](https://github.com/Byron/gitoxide/commit/4a1e79780374726b84be0de44d1e1907c2a6a68e))
    - enforce nicer/unified names so use struct instead of tuple ([`4c9a51e`](https://github.com/Byron/gitoxide/commit/4c9a51ee7206a90a07199d6a36a59f4e16a2d6bc))
    - refactor ([`0852f13`](https://github.com/Byron/gitoxide/commit/0852f132b2d49b674891b85c401a8e4a9463e385))
    - Baseline tests for global excludes and instantiation of pattern lists from files ([`afbb295`](https://github.com/Byron/gitoxide/commit/afbb295b7917c183e0923e018428c7e51e9b6a96))
    - Basic match group pattern matching ([`cc1312d`](https://github.com/Byron/gitoxide/commit/cc1312dc06d1dccfa2e3cf0ae134affa9a3fa947))
    - adapt to changes in git-glob ([`0effef0`](https://github.com/Byron/gitoxide/commit/0effef039b15417bbc225083d427ba1973bf1e0e))
    - push base path handling to the caller ([`e4b57b1`](https://github.com/Byron/gitoxide/commit/e4b57b197884bc981b8e3c9ee8c7b5349afa594b))
    - match group from overrides ([`f4f5a11`](https://github.com/Byron/gitoxide/commit/f4f5a115e9d3cf167eca1e213310c755e53f98e2))
    - A MatchGroup for later matching in stages, and for encapsulating some knoweldge about git repositories ([`0a5b5c4`](https://github.com/Byron/gitoxide/commit/0a5b5c4682bb321747398bb90041b12a2f8bf095))
    - A sketch of something that shouldn't be: a Description to instantiate patterns ([`388a8cd`](https://github.com/Byron/gitoxide/commit/388a8cd55a31d309f5c683645fc18ace6ddf4af3))
    - adapt to changes in git-glob ([`229ac13`](https://github.com/Byron/gitoxide/commit/229ac135235ba96aff651fc865fab0d2cf61aea6))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
    - a way to set a globs base path ([`3d58db8`](https://github.com/Byron/gitoxide/commit/3d58db8a9abfb91600216b8fc6f4109f5289d776))
    - Keep track of absolute patterns, those that have to start with it ([`3956480`](https://github.com/Byron/gitoxide/commit/3956480e6fb5f4766a67ebf2860cae2f48125594))
    - Also parse the position of the first wildcard ([`4178a63`](https://github.com/Byron/gitoxide/commit/4178a6356ad11013ae08b6233de2bfb366bf4278))
    - prepare for upcoming wildcard-length field in glob pattern ([`a11f5d4`](https://github.com/Byron/gitoxide/commit/a11f5d441a22b844caefd31b9cb7783dd6b048ad))
    - use git-glob crate for pattern parsing ([`120d908`](https://github.com/Byron/gitoxide/commit/120d9085c35ac72d4b83daee7f2cb59fde91890e))
    - a more realistic git-attributes file for parser testing ([`42aae32`](https://github.com/Byron/gitoxide/commit/42aae3232694656e5256d9b410e7b326118eac38))
    - differentiate macro and attribute errors ([`a9e2b60`](https://github.com/Byron/gitoxide/commit/a9e2b608964eec7b6e4d7d7614941cc2e0e51ebd))
    - refactor ([`eaab5a5`](https://github.com/Byron/gitoxide/commit/eaab5a5bc97c4cc16ac5b90d9f105b348fc816a2))
    - macro parsing ([`0f677ce`](https://github.com/Byron/gitoxide/commit/0f677ceb7df4ec54ef615e4c4069f549e861f339))
    - prepare for macro support ([`1981f6f`](https://github.com/Byron/gitoxide/commit/1981f6f8e8ab719bf4f67aabff9c72cf0ec1b25b))
    - attribute name validation ([`65c416b`](https://github.com/Byron/gitoxide/commit/65c416bef3323250d0fb82085049ea68adae8001))
    - parse all kinds of attributes, lacking name validation ([`96b0fca`](https://github.com/Byron/gitoxide/commit/96b0fcad1229ad2563e5e628d24289207a165005))
    - very basic parsing of attributes ([`3409a66`](https://github.com/Byron/gitoxide/commit/3409a66a0b8f279d5c10ef4a948824e7809394da))
    - add quote tests ([`93bf118`](https://github.com/Byron/gitoxide/commit/93bf1189902f3a6bff3ea5922bf62006b983e5b5))
    - A first stab at unquoting ansi_c style patterns ([`8ec7b30`](https://github.com/Byron/gitoxide/commit/8ec7b30f6bfaab8273c1007f16a7a1375fe46239))
    - all path-related tests are green ([`81d2bf2`](https://github.com/Byron/gitoxide/commit/81d2bf2ec5f571245d56eb853306d07ede3010a2))
    - part of line handling implemented, but test still fails for good reason ([`311db97`](https://github.com/Byron/gitoxide/commit/311db977049216928bba66201620c3a08d05f07f))
    - API and first test for attributes parsing ([`ccc87de`](https://github.com/Byron/gitoxide/commit/ccc87defb4e739ccc1de8a0deae57233901f674d))
    - refactor ([`3f62795`](https://github.com/Byron/gitoxide/commit/3f627954d2e992dd56eeee82a99f7ad41e619fb2))
    - skip the BOM as well ([`0c256d3`](https://github.com/Byron/gitoxide/commit/0c256d3a60b83ae20575f26ac1a9152fd30c7b29))
    - prepare for git-attribute file parsing ([`939d210`](https://github.com/Byron/gitoxide/commit/939d210de9f490f7e4014b11b7eae51dd801b596))
    - refactor ([`9a9115f`](https://github.com/Byron/gitoxide/commit/9a9115f8db0a84818600f125b1185d1773f10d39))
    - Support for 'ends_with' matching mode ([`e9d222a`](https://github.com/Byron/gitoxide/commit/e9d222a19541e2c75370d3e2feeb24beec093859))
    - iterator actually iterates all lines in a buffer ([`6a37eee`](https://github.com/Byron/gitoxide/commit/6a37eee5292bacdaca8c97608e900872128ae9bf))
    - Make line number accessible ([`0906bed`](https://github.com/Byron/gitoxide/commit/0906bedc7525979eb02192beb007f096cd6ac45f))
    - implement most of the ignore flags ([`d95905f`](https://github.com/Byron/gitoxide/commit/d95905f57e10c90d615243ec692a81404b3571da))
    - Handle trailing whitespaces ([`9a5d089`](https://github.com/Byron/gitoxide/commit/9a5d089010c5f46dc0470a8611b88c532836f841))
    - A sketch of the parser API for ignore files ([`a161e33`](https://github.com/Byron/gitoxide/commit/a161e330fb90f23eb8760cd170316358c34f7359))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - all tests (so far) green ([`67a2050`](https://github.com/Byron/gitoxide/commit/67a2050156cc809767ca026f467f35b552bea043))
    - fix serde support ([`2fb4310`](https://github.com/Byron/gitoxide/commit/2fb43102cf8bbfa9c26877d81d8fd3208fc5e183))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - Merge branch 'main' into worktree-stack ([`8674c11`](https://github.com/Byron/gitoxide/commit/8674c11973e5282d087e35a71c70e418b6cc75be))
    - thanks clippy ([`5992883`](https://github.com/Byron/gitoxide/commit/59928836cb23fdc8bcf0d083ba05deccc0dbf7e0))
    - thanks clippy ([`ac53780`](https://github.com/Byron/gitoxide/commit/ac537802dde00553f9f11908e5c484aa1c7153b6))
    - thanks clippy ([`d6787e4`](https://github.com/Byron/gitoxide/commit/d6787e4e05d24c2b36fcacf2346884fed62f2fec))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - refactor ([`3e78ff5`](https://github.com/Byron/gitoxide/commit/3e78ff53125be2a75142534b6fd6f356b6bc8c5f))
    - thanks clippy ([`365a8f0`](https://github.com/Byron/gitoxide/commit/365a8f08134a023bac7b78f3eee7baff410ba4cb))
    - thanks clippy ([`32b0634`](https://github.com/Byron/gitoxide/commit/32b063477bc12b6b823de3dc390c3dd51012ba20))
    - thanks clippy ([`f5639b6`](https://github.com/Byron/gitoxide/commit/f5639b688df78648479fe1666a7aa2ed65ea6753))
</details>

## 0.0.0 (2022-03-17)

Initial release with no content.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#301](https://github.com/Byron/gitoxide/issues/301)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Name-crates for git-ignore and git-attributes handling ([`2e04a49`](https://github.com/Byron/gitoxide/commit/2e04a4934a42cc2bb90334cf75e4af2ab394cffa))
 * **Uncategorized**
    - Release git-attributes v0.0.0 ([`5da2e98`](https://github.com/Byron/gitoxide/commit/5da2e98001d7602480fbf561355cfbe866bdf820))
    - Release git-ignore v0.0.0, git-attributes v0.0.0 ([`c128f27`](https://github.com/Byron/gitoxide/commit/c128f27df83be2473bd1788cc58118ca4c5ba407))
</details>

