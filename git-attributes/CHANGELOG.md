# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 (2022-12-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.6.0 (2022-11-21)

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

## 0.5.0 (2022-10-10)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.4.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 22 calendar days.
 - 24 days passed between releases.
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
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - upgrade all dependencies, except for `windows` ([`2968181`](https://github.com/Byron/gitoxide/commit/29681819ffe53d3926d631dc482f71d6200cb549))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.3.3 (2022-08-27)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.3.2 (2022-08-24)

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

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.3.1 (2022-08-17)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 24 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'pathspec' ([`7db59a4`](https://github.com/Byron/gitoxide/commit/7db59a4074111086adfc2f79fd0d26bb30303ca9))
    - improve docs and use 'new-style' in error messages. ([`e36d83e`](https://github.com/Byron/gitoxide/commit/e36d83e62eb7969726e7c8b3d25dbb743a508f8a))
    - Add docs for `git-attributes` ([`0eabea9`](https://github.com/Byron/gitoxide/commit/0eabea9772ce67f70442bc8ded02a7e82f5c17cc))
    - refactor ([`1cbc142`](https://github.com/Byron/gitoxide/commit/1cbc142d37599f4d7bfaf9cb07de41ee4b3f4c24))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
</details>

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - refactor ([`63baa75`](https://github.com/Byron/gitoxide/commit/63baa752901388a46a4211c70f3b3a64aa36d4ec))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - refactor of `Name` and `Assignment` ([`6449e77`](https://github.com/Byron/gitoxide/commit/6449e77e11ef0d25c2990f1c29e9fbea3c97fb0a))
    - refactor ([`e83879c`](https://github.com/Byron/gitoxide/commit/e83879cee1666ba927a95c05c714d132a109eeef))
    - impl '.as_ref()' for State ([`82074d5`](https://github.com/Byron/gitoxide/commit/82074d5e026c31b382cf97eade22eeca1bce3390))
    - use "to_owned" instead of "into" ([`35c6d38`](https://github.com/Byron/gitoxide/commit/35c6d38088e09d88b30e12538e175a4a286980cd))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - refactor ([`3be7a2d`](https://github.com/Byron/gitoxide/commit/3be7a2dc3cb8f476184555a1c62e230b7703db54))
    - fix build ([`1838f3d`](https://github.com/Byron/gitoxide/commit/1838f3db13eae6d278264dcdbc48d202de992349))
    - refactor ([`957356b`](https://github.com/Byron/gitoxide/commit/957356b4e5ea30ff5fa4390859f9e91093df9feb))
    - fix rust fmt issue ([`a9cb68b`](https://github.com/Byron/gitoxide/commit/a9cb68b5e04e11dc1bd7a4dc152001f26a87a445))
    - implement name::error for git-attributes ([`0849ebf`](https://github.com/Byron/gitoxide/commit/0849ebf4bc2052d7886f9425800a547bf530e967))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - refactor ([`9945ceb`](https://github.com/Byron/gitoxide/commit/9945ceb0a99c1343cb6e652e44900b36d3786e22))
    - refactor ([`3b2bab8`](https://github.com/Byron/gitoxide/commit/3b2bab89172b86068bda9704bc9d69690bcfb2ba))
    - quickerror to thiserror ([`da84b67`](https://github.com/Byron/gitoxide/commit/da84b675d3e825d2f815957fbed9928a0480ea4a))
    - protected attribute name via "AttributeName" type ([`7bb408e`](https://github.com/Byron/gitoxide/commit/7bb408e631138854a6dff85ce356da96f61367de))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.2.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - branch start, upgrade to compact_str v0.4 ([`b2f56d5`](https://github.com/Byron/gitoxide/commit/b2f56d5a279dae745d9c2c80ebe599c00e72c0d7))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.1.0 (2022-05-18)

### Changed (BREAKING)

 - <csr-id-120d9085c35ac72d4b83daee7f2cb59fde91890e/> use git-glob crate for pattern parsing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 84 commits contributed to the release over the course of 61 calendar days.
 - 62 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`5992883`](https://github.com/Byron/gitoxide/commit/59928836cb23fdc8bcf0d083ba05deccc0dbf7e0))
    - thanks clippy ([`ac53780`](https://github.com/Byron/gitoxide/commit/ac537802dde00553f9f11908e5c484aa1c7153b6))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - thanks clippy ([`d6787e4`](https://github.com/Byron/gitoxide/commit/d6787e4e05d24c2b36fcacf2346884fed62f2fec))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - refactor ([`3e78ff5`](https://github.com/Byron/gitoxide/commit/3e78ff53125be2a75142534b6fd6f356b6bc8c5f))
    - thanks clippy ([`365a8f0`](https://github.com/Byron/gitoxide/commit/365a8f08134a023bac7b78f3eee7baff410ba4cb))
    - thanks clippy ([`32b0634`](https://github.com/Byron/gitoxide/commit/32b063477bc12b6b823de3dc390c3dd51012ba20))
    - Merge branch 'parse-git-ignore' ([`8ab19a6`](https://github.com/Byron/gitoxide/commit/8ab19a639b25b70872e89a933245abeea2b10ded))
    - thanks clippy ([`f5639b6`](https://github.com/Byron/gitoxide/commit/f5639b688df78648479fe1666a7aa2ed65ea6753))
</details>

## 0.0.0 (2022-03-17)

Initial release with no content.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
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

