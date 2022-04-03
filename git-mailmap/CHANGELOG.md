# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2022-04-03)

### New Features

 - <csr-id-d2388d8d80f379eccc9ee84ebe07acd67d154630/> `gix repository mailmap entries`
 - <csr-id-77ef2cb819f21ddc5d1ee9e94b5961e3ca5b3139/> `Time::default()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - More aggressive mailmap substitution for better results ([`600eb69`](https://github.com/Byron/gitoxide/commit/600eb69132c24e15fde88ac5724ee5d2105be8df))
    - adjust to changes in git-actor ([`e5c0200`](https://github.com/Byron/gitoxide/commit/e5c02002467a6ad2ab2330cf6f38bcebabf4ba7c))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - `gix repository mailmap entries` ([`d2388d8`](https://github.com/Byron/gitoxide/commit/d2388d8d80f379eccc9ee84ebe07acd67d154630))
    - gix mailmap verify can now detect collisions ([`f89fe2f`](https://github.com/Byron/gitoxide/commit/f89fe2f867fa792db5d9e003ce342a337a6ac973))
    - verify universal line-endings are supported ([`f498dac`](https://github.com/Byron/gitoxide/commit/f498dacfc34c3d0b7c3bdbf100e456ad3ade419e))
    - fix email-only case ([`e31754d`](https://github.com/Byron/gitoxide/commit/e31754d3d9c58f82ff4fad11ed7985dffc99b586))
    - add all docs ([`1c768a5`](https://github.com/Byron/gitoxide/commit/1c768a5511ba4e2f7f860b48429ddd3930a6b501))
    - Another test to validate correct merging and overwriting ([`deaeb7d`](https://github.com/Byron/gitoxide/commit/deaeb7d1730d90d06789c47adad0e25f9b74fd13))
    - Add method to return borrowed values for new name/email ([`87fb932`](https://github.com/Byron/gitoxide/commit/87fb932239e5f5a55046f8edb073373cd4957422))
    - actual lookup and tests, all seems to be working ([`8116664`](https://github.com/Byron/gitoxide/commit/81166646e3ecc9ab7383de62a0a216d0229c90bd))
    - sketch `Snapshot` API to implement map building and signature resolution ([`1890db7`](https://github.com/Byron/gitoxide/commit/1890db73b5fd66467c66efd9ddc365871041c7c3))
    - `Time::default()` ([`77ef2cb`](https://github.com/Byron/gitoxide/commit/77ef2cb819f21ddc5d1ee9e94b5961e3ca5b3139))
    - sketch of mailmap snapshot for lookups ([`d71d067`](https://github.com/Byron/gitoxide/commit/d71d0670cd89a2dac2ea84cbc538f67fef3ee451))
    - Add typical malmap example ([`b67b0f9`](https://github.com/Byron/gitoxide/commit/b67b0f90dd947508ab9ab0b7d68ce0093ae415ae))
    - quickfix for unintentionally using 'unicode' feature of bytecode ([`fb5593a`](https://github.com/Byron/gitoxide/commit/fb5593a7272498ae042b6c8c7605faa3d253fa10))
    - all tests (so far) green ([`67a2050`](https://github.com/Byron/gitoxide/commit/67a2050156cc809767ca026f467f35b552bea043))
    - high-level parsing to deal with allowed mailmap lines ([`f458817`](https://github.com/Byron/gitoxide/commit/f458817005b884e966bcc894a0cf7c9958882ba4))
    - fix serde support ([`2fb4310`](https://github.com/Byron/gitoxide/commit/2fb43102cf8bbfa9c26877d81d8fd3208fc5e183))
    - basic testing of common cases for mailmap ([`903c526`](https://github.com/Byron/gitoxide/commit/903c5263a26d9a57d9fa9dc6649ef4ad0a6e2a94))
    - the first empty test ([`fc47c49`](https://github.com/Byron/gitoxide/commit/fc47c497f992e0f3fbef2f55d0e3b0909cac8290))
 * **Uncategorized**
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - thanks clippy ([`1038dab`](https://github.com/Byron/gitoxide/commit/1038dab842b32ec1359a53236b241a91427ccb65))
    - Commit to using 'unicode' feature of bstr as git-object wants it too ([`471fa62`](https://github.com/Byron/gitoxide/commit/471fa62b142ba744541d7472464d62826f5c6b93))
    - thanks clippy ([`9449bc7`](https://github.com/Byron/gitoxide/commit/9449bc7243c15b3cba88f02a3742784d6fe6b363))
    - refactor ([`3e78ff5`](https://github.com/Byron/gitoxide/commit/3e78ff53125be2a75142534b6fd6f356b6bc8c5f))
</details>

## 0.0.0 (2022-03-26)

An empty crate without any content to reserve the name for the gitoxide project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#366](https://github.com/Byron/gitoxide/issues/366)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - empty git-mailmap crate ([`a30a5da`](https://github.com/Byron/gitoxide/commit/a30a5da60d67a4e52ba69727318edb9832b7cae2))
 * **Uncategorized**
    - Release git-mailmap v0.0.0 ([`c43af35`](https://github.com/Byron/gitoxide/commit/c43af35c92e5093349cdabd89f655b26070e6f84))
</details>

