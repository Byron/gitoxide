# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-c24ea67f84aa48953949682672114715bee67432/> parse now takes the current time `parse(…, Option<time>)` as parameter.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 24 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **Uncategorized**
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - refactor ([`c5c6bf6`](https://github.com/Byron/gitoxide/commit/c5c6bf6ef3f0c9c12389bb638ab4d32b61839dec))
    - refactor ([`956613f`](https://github.com/Byron/gitoxide/commit/956613fcdb33a845526fa9743aa0e7f80b3badfa))
    - refactor ([`1026b7c`](https://github.com/Byron/gitoxide/commit/1026b7c613a3a8b46a27dd7cd5e3520043b21ab7))
    - WIP. ([`79d82d4`](https://github.com/Byron/gitoxide/commit/79d82d46613c83280d2401ef4d72a35010a70b87))
    - Parse the output while parsing the baseline file. ([`70fe59f`](https://github.com/Byron/gitoxide/commit/70fe59f4a1cad25f687397206ee2cbe50e643181))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - parse now takes the current time `parse(…, Option<time>)` as parameter. ([`c24ea67`](https://github.com/Byron/gitoxide/commit/c24ea67f84aa48953949682672114715bee67432))
    - thanks clippy ([`590fcc9`](https://github.com/Byron/gitoxide/commit/590fcc9f3fc768802fd09f4e73036e225ec5c363))
    - a sample on how to more easily test relative date parsing ([`c585c9b`](https://github.com/Byron/gitoxide/commit/c585c9b2b0e628914169bbfba55aa5130da51a83))
    - add test to check times before unix epoch ([`eb304ea`](https://github.com/Byron/gitoxide/commit/eb304ea91f857070380ef4e4dbdff6b8ab89cee1))
    - refactor ([`0e231eb`](https://github.com/Byron/gitoxide/commit/0e231ebeeb41306b1075bc06c78b45dc65ded5fa))
    - refactor ([`5793465`](https://github.com/Byron/gitoxide/commit/5793465b0f19b284a4615290e2b08203e969e9bb))
    - refactor; add failing test to see invalid date error in action ([`90008aa`](https://github.com/Byron/gitoxide/commit/90008aa4e59f78cfe2ecd5a7ee851bb56f7b0d33))
    - PR comments. ([`1eac4de`](https://github.com/Byron/gitoxide/commit/1eac4de99de250e770b48054fd5bc6b806418e4d))
    - `parse` is pure function. ([`9ad1a5f`](https://github.com/Byron/gitoxide/commit/9ad1a5fa2ce54e978396ff3eaa7061a8edd10d4a))
    - Fallible timestamp cast i64 -> u32. ([`cce7616`](https://github.com/Byron/gitoxide/commit/cce7616835f1dff29b2c74bd96b5238d55167d5f))
    - `parse()` returns Result. ([`206f392`](https://github.com/Byron/gitoxide/commit/206f3923f5da2e9e26677e917550e6e5baa2913a))
    - Add output to baseline. ([`5c3b733`](https://github.com/Byron/gitoxide/commit/5c3b7334289ef71a9dbcc021f1989eb8e021fc7f))
    - `parse` returns Result. ([`67c8c6a`](https://github.com/Byron/gitoxide/commit/67c8c6a9666161a3a0cd01203e0fb1da22336939))
    - Add fixtures. ([`6c40ac1`](https://github.com/Byron/gitoxide/commit/6c40ac1e39fbefba640fc9cb4fd3ef419a149e99))
    - Add git baseline. ([`b747a60`](https://github.com/Byron/gitoxide/commit/b747a60b62c533b3919124e628f31e6bbef9c838))
    - refactor ([`3e6e0f9`](https://github.com/Byron/gitoxide/commit/3e6e0f9ebdf7ab87c689be074ae1eecfea4485f0))
    - Draft. ([`95b4902`](https://github.com/Byron/gitoxide/commit/95b490230ebfbd4c2f04edd7074d5a0f5e2429ec))
    - Draft. ([`43b6c06`](https://github.com/Byron/gitoxide/commit/43b6c064bfa407d1a2cabf09aad1cc8334dec651))
</details>

## 0.1.0 (2022-08-24)

### New Features

 - <csr-id-034c8dc4437c06dc8af702fbe9b86ec903c73a18/> bump version to 1.0 to prevent accidental inclusions downstream
   For some reason, cargo considers different patch releases breaking, so
   creating a new patch can break installation of gitoxide entirely.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - bump version to 1.0 to prevent accidental inclusions downstream ([`034c8dc`](https://github.com/Byron/gitoxide/commit/034c8dc4437c06dc8af702fbe9b86ec903c73a18))
</details>

## 0.0.5 (2022-08-24)

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

 - 7 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Fix doc comment ([`51cd9ce`](https://github.com/Byron/gitoxide/commit/51cd9ceda6a8a0127a18802dc2cc49861013a65d))
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
</details>

## 0.0.4 (2022-08-19)

### New Features

 - <csr-id-8f7f9ce2b06ec884220b8cd5010b3df04b1ff0bc/> Raw and Unix formats.
 - <csr-id-4b0c2198f9d5b28584c717123c7cfb1b27724605/> Add ISO-strict format

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.4, git-actor v0.11.2, git-revision v0.4.1, git-repository v0.21.1 ([`2f9dc84`](https://github.com/Byron/gitoxide/commit/2f9dc847e0d54f4181ce35ddadd9286ba80ca01f))
    - update changelogs prior to release ([`1b5fd86`](https://github.com/Byron/gitoxide/commit/1b5fd86d121634f8567e8442f125377e460032c6))
    - prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Raw and Unix formats. ([`8f7f9ce`](https://github.com/Byron/gitoxide/commit/8f7f9ce2b06ec884220b8cd5010b3df04b1ff0bc))
    - Foundation for custom formats that aren't easily done with `time` formatting ([`b74eaf8`](https://github.com/Byron/gitoxide/commit/b74eaf85d41e1ec67d8c84cc8484702514c3e7cd))
    - Add ISO-strict format ([`4b0c219`](https://github.com/Byron/gitoxide/commit/4b0c2198f9d5b28584c717123c7cfb1b27724605))
    - refinements ([`b1fea0f`](https://github.com/Byron/gitoxide/commit/b1fea0fe76bd94850c7da34ee9504525ad667748))
    - Add common git date formats. ([`090795b`](https://github.com/Byron/gitoxide/commit/090795b4040e2dad995390e502f87c2ced8045f8))
</details>

## 0.0.3 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 25 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - git-style disambiguation errors ([`5717194`](https://github.com/Byron/gitoxide/commit/57171946081c03da98f3d33f5b963c3bc4b2d6d9))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'index-write-refactor' ([`805f432`](https://github.com/Byron/gitoxide/commit/805f432bf8e9d2dd9ede56caf959de386d5d80c7))
    - adjust `git_date::parsea(str)` to use a str ([`0f8680a`](https://github.com/Byron/gitoxide/commit/0f8680a60913556b7fbd7543fda6a409ac05b121))
    - refactor ([`11a5fa2`](https://github.com/Byron/gitoxide/commit/11a5fa29615d47c24f78446a1c3f5d3b8acf2f93))
    - refactor ([`8e6f4a9`](https://github.com/Byron/gitoxide/commit/8e6f4a921b6b45945e711aaf5858b7714371fb41))
    - thanks clipppy ([`b139d70`](https://github.com/Byron/gitoxide/commit/b139d7043fbbbe5b933d96e83544059fe2a7bdd8))
    - refactor ([`bd64387`](https://github.com/Byron/gitoxide/commit/bd64387d8ad3377571755dff14577cc3c53ee9cc))
    - Use time format strings. ([`f84e8f5`](https://github.com/Byron/gitoxide/commit/f84e8f5f16ec2197d1967fb1cc06e9609ea52c16))
    - refactor ([`556dd8c`](https://github.com/Byron/gitoxide/commit/556dd8cb78ea9321031984e2c6b4f9bc415f1be5))
    - refactor ([`5bbcbcd`](https://github.com/Byron/gitoxide/commit/5bbcbcd75d1ab26746da7a927390ff3b6cc19a85))
    - Format `git-date::Time` with `time::format_description`. ([`d4243bc`](https://github.com/Byron/gitoxide/commit/d4243bc4feb994bde99156ba77fff63bc9c875e9))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - Merge branch 'main' into rev-parse-delegate ([`4ae2bed`](https://github.com/Byron/gitoxide/commit/4ae2bedfc25d1881d58ebdc54aca0936c68d4859))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
</details>

## 0.0.2 (2022-07-22)

### New Features

 - <csr-id-c76fde7de278b49ded13b655d5345e4eb8c1b134/> initialize `Time` from `now_utc` and `now_local`
   Localtime support depends on some other factors now, but that
   will only get better over time.
   
   We might have to document `unsound_local_time` at some point.
 - <csr-id-aeda76ed500d2edba62747d667227f2664edd267/> `Time::is_set()` to see if the time is more than just the default.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 39 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#331](https://github.com/Byron/gitoxide/issues/331)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - initialize `Time` from `now_utc` and `now_local` ([`c76fde7`](https://github.com/Byron/gitoxide/commit/c76fde7de278b49ded13b655d5345e4eb8c1b134))
    - `Time::is_set()` to see if the time is more than just the default. ([`aeda76e`](https://github.com/Byron/gitoxide/commit/aeda76ed500d2edba62747d667227f2664edd267))
 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
</details>

## 0.0.1 (2022-06-13)

### New Features

 - <csr-id-cfb6a726ddb763f7c22688f8ef309e719c2dfce4/> Add `Time` type.
   It was originally from the `git-actor` crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 58 calendar days.
 - 59 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - reflog lookup by date is complete ([`b3d009e`](https://github.com/Byron/gitoxide/commit/b3d009e80e3e81afd3d095fa2d7b5fc737d585c7))
    - Add `Time` type. ([`cfb6a72`](https://github.com/Byron/gitoxide/commit/cfb6a726ddb763f7c22688f8ef309e719c2dfce4))
 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
</details>

## 0.0.0 (2022-04-14)

An empty crate without any content to reserve the name for the gitoxide project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#331](https://github.com/Byron/gitoxide/issues/331)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - frame for git-date ([`37e8ef8`](https://github.com/Byron/gitoxide/commit/37e8ef890305db0798059919290a0d27a9a39194))
 * **Uncategorized**
    - Release git-date v0.0.0 ([`2bc2f76`](https://github.com/Byron/gitoxide/commit/2bc2f765dc4f8a4779c132f7729fb782c66c0d99))
</details>

