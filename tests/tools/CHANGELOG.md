# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.6.0 (2022-04-04)

<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 210 calendar days.
 - 223 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#266](https://github.com/Byron/gitoxide/issues/266), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - fix windows tests by transforming line endings ([`e276d77`](https://github.com/Byron/gitoxide/commit/e276d777eb7a88dc424badbf88a929b5f567e5de))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - a failing test to show the handle-stability doesn't quite work yet ([`5562e88`](https://github.com/Byron/gitoxide/commit/5562e8888cd8ac8fc3d89a41f8e8cc5cec7b8ca6))
    - refactor ([`c499843`](https://github.com/Byron/gitoxide/commit/c499843485a8af102cb4d3594c4e6014976c5aa0))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - use parking_lot mutex to avoid poison errors ([`d8ca74f`](https://github.com/Byron/gitoxide/commit/d8ca74f358e802916353f545b90127f9a7bb5137))
    - base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade parking_lot and cargo_toml ([`f95c1a0`](https://github.com/Byron/gitoxide/commit/f95c1a0d9c19bcc6feb9b8739a09d86f9970a0e0))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - refactor ([`9ea1e44`](https://github.com/Byron/gitoxide/commit/9ea1e4474a3ce803da7a56e1fc1748f65c11a876))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - add test-tools changelog prior to release ([`1ebc16a`](https://github.com/Byron/gitoxide/commit/1ebc16a6ac9ef188c188a52737820773aa949cee))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - quickfix for unintentionally using 'unicode' feature of bytecode ([`fb5593a`](https://github.com/Byron/gitoxide/commit/fb5593a7272498ae042b6c8c7605faa3d253fa10))
 * **Uncategorized**
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - thanks clippy ([`1038dab`](https://github.com/Byron/gitoxide/commit/1038dab842b32ec1359a53236b241a91427ccb65))
    - add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - Commit to using 'unicode' feature of bstr as git-object wants it too ([`471fa62`](https://github.com/Byron/gitoxide/commit/471fa62b142ba744541d7472464d62826f5c6b93))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - ensure tests use 'merge.ff false' and recreate fixtures on each run ([`1d5ab44`](https://github.com/Byron/gitoxide/commit/1d5ab44145ccbc2064ee8cc7acebb62db82c45aa))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
</details>

## v0.5.0 (2021-08-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 8 calendar days.
 - 12 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-testtools v0.5.0 ([`86e0a92`](https://github.com/Byron/gitoxide/commit/86e0a92c7dc3b69a766aeac1b675b148d61a7ec5))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.4.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 57 calendar days.
 - 64 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - [pack] refactor ([`9ee1e22`](https://github.com/Byron/gitoxide/commit/9ee1e22fa5c5d97ff626f0dfc44706272433bfef))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com/Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [tools] fix create writable fixture ([`bf7783d`](https://github.com/Byron/gitoxide/commit/bf7783dd9ccc9ac433b978b9dded0d38f7351494))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com/Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - [ref] on the way to setup the first transaction test ([`29c0b51`](https://github.com/Byron/gitoxide/commit/29c0b51625e2c7e3a8d60075bb925126a024dc83))
    - Bump once_cell from 1.7.2 to 1.8.0 ([`bd323d9`](https://github.com/Byron/gitoxide/commit/bd323d911b6becf8b379343c6ef56ec46e28fa28))
</details>

## v0.3.0 (2021-06-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 12 calendar days.
 - 38 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - Manually fix crc in tooling ([`48fa9bc`](https://github.com/Byron/gitoxide/commit/48fa9bc80876a0186f43add6c6d3477385241f5e))
    - Bump crc from 1.8.1 to 2.0.0 ([`07f08ac`](https://github.com/Byron/gitoxide/commit/07f08ac1ea04ec278993ad1a5fc1d4f243bf8eb7))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
</details>

## v0.1.0 (2021-04-30)

<csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/>

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 276 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare test utilities for release… ([`d35e654`](https://github.com/Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - [tree-diff] Beginning of more nested test-suite… ([`b8a90e7`](https://github.com/Byron/gitoxide/commit/b8a90e7c9347b0eefdbef6f4c724cc0561cd79c9))
    - fix debug assert, thanks gitpython ([`fe954b9`](https://github.com/Byron/gitoxide/commit/fe954b9f6d26bd8629f24a01bd2a06f9800deed0))
    - Revert "FAIL: try to disable GPG signing with environment variables…" ([`e326352`](https://github.com/Byron/gitoxide/commit/e326352eec7bd1aae13f770328979e5730ffc32b))
    - try to disable GPG signing with environment variables… ([`29bf8ca`](https://github.com/Byron/gitoxide/commit/29bf8ca8399b6d4941aa242b9f08c74e59a179bb))
    - Thanks, cargo audit ([`4f293f5`](https://github.com/Byron/gitoxide/commit/4f293f5036c44a69ccacf102d35202adad83bbe0))
    - thanks clippy ([`002792a`](https://github.com/Byron/gitoxide/commit/002792a8bc2512c92c16fd28662c26c9b3a12572))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com/Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - faster repeated tests if fixtures don't change ([`792277f`](https://github.com/Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - Allow the use of shared test utilities across crates ([`b117626`](https://github.com/Byron/gitoxide/commit/b117626df6da714c24d2b7914301678e89d2d0cb))
    - The first test with the new and nice and cheap journey test tool ([`d3c99e1`](https://github.com/Byron/gitoxide/commit/d3c99e1cf3125ab107e12718b39ac9b7c9a9165c))
</details>

