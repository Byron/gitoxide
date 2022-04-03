# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 (2022-04-03)

A maintenance release, triggered by putting too many adjustments into a single commit.

### Changed (BREAKING)

 - <csr-id-2ef9a8424af51310db8c1e6df31dde9953ed3d21/> Change accessors named `hash_kind()` to `object_hash()` for consistency

### New Features

 - <csr-id-265b8ec07fd5357df629f0d29fb2412d0186a287/> Add support for hashes of different size
   Even though right now, there is only Sha1, in future it's easy to
   support other hash sizes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 125 calendar days.
 - 165 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#279](https://github.com/Byron/gitoxide/issues/279), [#293](https://github.com/Byron/gitoxide/issues/293), [#329](https://github.com/Byron/gitoxide/issues/329)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Also consider the size of the fanout table as part of the min size ([`8190708`](https://github.com/Byron/gitoxide/commit/8190708bc2b6ac9900d5f98b6c7db8cb3f38a632))
    - use latest capabilities of `git-hash` ([`a489ac2`](https://github.com/Byron/gitoxide/commit/a489ac2ca19a9fbf64f590c0d36c02b55c1a0536))
    - cargo fmt ([`8b9da35`](https://github.com/Byron/gitoxide/commit/8b9da35b3e0d3458efcac150f7062c9d7382a6c4))
    - Access pack-indices and pack-offsets of multi-pack indices ([`c2a6918`](https://github.com/Byron/gitoxide/commit/c2a69189f88c53ab555158245ce647fcd33fca6a))
    - adapt to changes in git-hash ([`5eb0230`](https://github.com/Byron/gitoxide/commit/5eb0230b58c25c0aa744eee0bd878dd91410dbe1))
    - Change accessors named `hash_kind()` to `object_hash()` for consistency ([`2ef9a84`](https://github.com/Byron/gitoxide/commit/2ef9a8424af51310db8c1e6df31dde9953ed3d21))
    - adjust to changes in git-hash ([`9bf25cc`](https://github.com/Byron/gitoxide/commit/9bf25cc4f2e44821f93e85997677bc4e86a67bd4))
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
    - Add support for hashes of different size ([`265b8ec`](https://github.com/Byron/gitoxide/commit/265b8ec07fd5357df629f0d29fb2412d0186a287))
    - refactor ([`501b85b`](https://github.com/Byron/gitoxide/commit/501b85b0ba57873f13e3086983d3b7a8b20defdd))
    - refactor ([`8c9c7fc`](https://github.com/Byron/gitoxide/commit/8c9c7fc3bc46afa9c8567a8bc8079cac12ed8422))
    - Use `git-chunk` crate for all chunk-related operations ([`0cd7f3b`](https://github.com/Byron/gitoxide/commit/0cd7f3b796fec9fe3eac953b6e56bd78b0ea18f9))
    - First round of introducing git-chunk ([`51b991b`](https://github.com/Byron/gitoxide/commit/51b991b2ca5727deb3447a51b14088dfdad8e7fe))
    - Adapt to latest changes to git-chunk ([`743d696`](https://github.com/Byron/gitoxide/commit/743d6967d6236a4bb6a9c8817f957e7604bc9264))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - remove byteorder dependency from git-commitgraph ([`c526811`](https://github.com/Byron/gitoxide/commit/c5268115d9193ba2e309a943ee1d3c9e5825562c))
    - use memmap2 in git-commitgraph ([`0c946f5`](https://github.com/Byron/gitoxide/commit/0c946f5cb9d6eb13615b6c3d1a7b479ab5874441))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **Uncategorized**
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - thanks clippy ([`53bd30f`](https://github.com/Byron/gitoxide/commit/53bd30fd56c971b2be5a5d22045b97dc5f216303))
    - thanks clippy ([`6cc1bd1`](https://github.com/Byron/gitoxide/commit/6cc1bd15a49d9ec67a4a381ee3f64d557850733c))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
</details>

## v0.6.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.5.0 (2021-10-15)

This is a maintenance release without functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 38 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Maintenance release note to avoid being fully generated ([`56ef363`](https://github.com/Byron/gitoxide/commit/56ef363f0e7a8b9106765d96d0636e38b2bed550))
    - changelog for git-commitgraph ([`d981f1f`](https://github.com/Byron/gitoxide/commit/d981f1f18ecbc943202702ab25a31a371a4b294d))
 * **Uncategorized**
    - Release git-commitgraph v0.5.0, gitoxide-core v0.11.0, gitoxide v0.9.0 ([`960eb0e`](https://github.com/Byron/gitoxide/commit/960eb0e5e5a7df117ed2ae2a8e2ec167b074c332))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
</details>

## v0.4.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-commitgraph v0.4.4 ([`dec935c`](https://github.com/Byron/gitoxide/commit/dec935cd6ef9a70afd247e5fcf44983c97c1b10b))
</details>

## v0.4.3 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 10 calendar days.
 - 12 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-commitgraph v0.4.3 ([`7dfe16b`](https://github.com/Byron/gitoxide/commit/7dfe16bdebaf971b7101331ad037d1ca8ab491d2))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [repository #165] refactor ([`1547d0b`](https://github.com/Byron/gitoxide/commit/1547d0b062e35bad2229dac532e6f30bf105db73))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
</details>

## v0.4.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-commitgraph v0.4.2 ([`847c456`](https://github.com/Byron/gitoxide/commit/847c4564d9b64c071db790979654d0883d7a38d0))
    - Promote file-format constants to `git_commitgraph::file` module. ([`0afd354`](https://github.com/Byron/gitoxide/commit/0afd354f94fb1829d4c097b49cba503bac3d1c38))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.4.1 (2021-08-15)

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 119 calendar days.
 - 128 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-commitgraph v0.4.1 ([`1776a0d`](https://github.com/Byron/gitoxide/commit/1776a0d7168f1f15a18e0f873a9918a6db33b94a))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - change wording ([`6c82a16`](https://github.com/Byron/gitoxide/commit/6c82a16d340acb9b11c5cf56c917c9fe6f2cdf0e))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com/Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com/Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com/Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - fix git-commigraph build (broke after git-hash changed its ways) ([`08fd7a0`](https://github.com/Byron/gitoxide/commit/08fd7a08800d926bcfeb1cfe6faa1f02c0b8904e))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com/Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Put 'sha1' behind a feature toggle ([`4f326bc`](https://github.com/Byron/gitoxide/commit/4f326bc261c4e7f0d5510df74ad4215da3580696))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com/Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - prepare test utilities for release… ([`d35e654`](https://github.com/Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - Revert "FAIL: try to disable GPG signing with environment variables…" ([`e326352`](https://github.com/Byron/gitoxide/commit/e326352eec7bd1aae13f770328979e5730ffc32b))
    - try to disable GPG signing with environment variables… ([`29bf8ca`](https://github.com/Byron/gitoxide/commit/29bf8ca8399b6d4941aa242b9f08c74e59a179bb))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com/Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - faster repeated tests if fixtures don't change ([`792277f`](https://github.com/Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - git-commitgraph uses test-tools ([`5d30e5a`](https://github.com/Byron/gitoxide/commit/5d30e5a3474aabd67cb5d1afc826aa68957d2b7a))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com/Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
</details>

## v0.4.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 56 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com/Byron/gitoxide/issues/63)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com/Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - git-commitgraph with a more convenient public interface with AsRef ([`ba04e4e`](https://github.com/Byron/gitoxide/commit/ba04e4ed673c654a8968532228571a93a3ebc8e2))
    - git-commitgraph uses `oid` now ([`0b72966`](https://github.com/Byron/gitoxide/commit/0b72966249523b97fce1bc7b29082ac68fa86a4f))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com/Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com/Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
    - Remove re-export of git_object::borrowed::Id ([`a3f2816`](https://github.com/Byron/gitoxide/commit/a3f28169c1268c1129852f279631d5a7f7540cdf))
 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`06612eb`](https://github.com/Byron/gitoxide/commit/06612eb12d4679bec7dae08a511dd87d80087151))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com/Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
</details>

## v0.3.2 (2021-02-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 18 calendar days.
 - 32 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.2 ([`d91dd9d`](https://github.com/Byron/gitoxide/commit/d91dd9d8c57688dc9c420460ef5800cd07b3c9b4))
    - [commitgraph] Tweak and expand documentation. ([`ac52867`](https://github.com/Byron/gitoxide/commit/ac5286772c0eefd994b3d85ab185e0d4960cdd0a))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com/Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
</details>

## v0.3.1 (2021-01-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 23 calendar days.
 - 24 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 ([`89db50c`](https://github.com/Byron/gitoxide/commit/89db50ce01ea7cc83b7f90484e2f8736dba7ccde))
    - remaining docs for git-commitgraph crate ([`9146176`](https://github.com/Byron/gitoxide/commit/91461760884979218617fcfdc56efd8be73b9d6f))
    - more commitgraph docs ([`a81ea67`](https://github.com/Byron/gitoxide/commit/a81ea6730f11f769caed9a70cad123cace96b625))
    - all docs for git-commitgraph::file ([`8b26201`](https://github.com/Byron/gitoxide/commit/8b262011ceffaff74bea9f4ffc730682884fff64))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
</details>

## v0.3.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com/Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.2.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 64 calendar days.
 - 74 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`d61ad88`](https://github.com/Byron/gitoxide/commit/d61ad884021d3c0a61a14ba1df4daadfa1a0b561))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com/Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com/Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - (cargo-release) version 0.1.3 ([`a833fd1`](https://github.com/Byron/gitoxide/commit/a833fd18e1bc3a501e4f1ed66506f48673f79590))
    - thanks clippy ([`ba9b3c2`](https://github.com/Byron/gitoxide/commit/ba9b3c2345887353e02fc081be80733f1c5e22d9))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com/Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com/Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - Note about why git_features::hash::bytes_of_file() is not yet used ([`ca48fc4`](https://github.com/Byron/gitoxide/commit/ca48fc4f7c00215acf95370fe894a6e585c18c13))
    - Add and use borrowed::Id::null_sha1() ([`c717492`](https://github.com/Byron/gitoxide/commit/c717492d0038f55a6f21b48937b56a756890d214))
    - refactor ([`e4935e0`](https://github.com/Byron/gitoxide/commit/e4935e03040e1f4ded652ed43a1e0177eefb44f4))
    - replace 'ImpossibleVariantError' with 'std::convert::Infallible'` ([`c53638c`](https://github.com/Byron/gitoxide/commit/c53638ccd9e392af839b7eb03826fa6aab94faff))
    - [commitgraph] Clean up `{file,graph}::verify::Error` types. ([`fa22cab`](https://github.com/Byron/gitoxide/commit/fa22cab259338dc140dd660f4f4b9bbc9d6cc3d0))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com/Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Loosen lifetime restrictions on return values. ([`701f33c`](https://github.com/Byron/gitoxide/commit/701f33c06b80deaabe7625b01d36e2a1b1af3a78))
    - [commitgraph] Replace `T as U` with `U::from(T)` or `t.try_into()`. ([`28f94b4`](https://github.com/Byron/gitoxide/commit/28f94b4bccdf317c9f4ccb62e0e3f3314f3995c9))
    - [commitgraph] Tweak `File::iter_base_graph_ids` implementation. ([`5b06780`](https://github.com/Byron/gitoxide/commit/5b067808a793e3515c0c12cf95c11b57beaa8d09))
    - [commitgraph] Add `Graph::at` constructor. ([`a783052`](https://github.com/Byron/gitoxide/commit/a783052d0cc2d3c9fa1dda3ea77286a79690d2c1))
    - [commitgraph] Validate trailer section when parsing files. ([`1b738ac`](https://github.com/Byron/gitoxide/commit/1b738ac0719ec20b24982d148a386d63ec4dc2d6))
    - [commitgraph] Use `thiserror` instead of `quick_error`. ([`c8b1f74`](https://github.com/Byron/gitoxide/commit/c8b1f74328965708e38a689b865660ad36f22ecb))
</details>

## v0.1.2 (2020-10-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.2 ([`b401468`](https://github.com/Byron/gitoxide/commit/b40146828771d9837350e07250fb21851f700fcc))
    - Merge remote-tracking branch 'origin/main' into main ([`f3d90d7`](https://github.com/Byron/gitoxide/commit/f3d90d7f65cdbcfed4281c0382f8c6766809afaa))
</details>

## v0.1.1 (2020-10-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`04c7cdf`](https://github.com/Byron/gitoxide/commit/04c7cdf1418f43052390f5d67bd4e7e43ae8b2e6))
    - Fix repository URL ([`d721f47`](https://github.com/Byron/gitoxide/commit/d721f478ab441db30585af747d9f47717443d7e1))
    - update commitgraph information ([`275cfde`](https://github.com/Byron/gitoxide/commit/275cfde06192c8b3a3d633b21e970b54ddc1a53f))
</details>

## v0.1.0 (2020-10-01)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 15 calendar days.
 - 42 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [commitgraph] add size limit and prep for release ([`4eabf55`](https://github.com/Byron/gitoxide/commit/4eabf554dc7cc08416d1078fa29db606455dc031))
    - [commitgraph] bump minor version for first release ([`76bb4d3`](https://github.com/Byron/gitoxide/commit/76bb4d355dd1570340fe7d05d2a3378e15a36d4e))
    - [commitgraph] refactor file::init ([`8b003a0`](https://github.com/Byron/gitoxide/commit/8b003a01729e4bfcb433e34f32b8e450cbe75fea))
    - [commitgraph] refactor ([`c4b14c1`](https://github.com/Byron/gitoxide/commit/c4b14c1eae8dfcdcb3637d64e3c81dc424e26607))
    - [commitgraph] Rename LexPosition to 'file::Position' ([`6f90bee`](https://github.com/Byron/gitoxide/commit/6f90beeb418480f9cd8bb7ae3b5db678b24103cb))
    - [commitgraph] refactor graph::init module ([`d2eec1d`](https://github.com/Byron/gitoxide/commit/d2eec1dbedac6e87cc281cdd84423d9c7cfba323))
    - [commitgraph] refactor Graph, Position, and access module ([`3c8640e`](https://github.com/Byron/gitoxide/commit/3c8640e5baf4729f4394c569dc0aed9865121e7a))
    - [commitgraph] refactor ([`2ed0037`](https://github.com/Byron/gitoxide/commit/2ed0037c87fa17fbdb560cab46f72bf64805623b))
    - [commitgraph] refactor ([`7026961`](https://github.com/Byron/gitoxide/commit/7026961ab7de4ee66ae84bdfdeef359ae960d231))
    - [commitgraph] Assure git doesn't try to sign commits when fixtures are created ([`9ae1f4b`](https://github.com/Byron/gitoxide/commit/9ae1f4b9bb05a19ba279a1242f3c84d439421f18))
    - Merge branch 'main' into commit-graph ([`ca5b801`](https://github.com/Byron/gitoxide/commit/ca5b80174b73cc9ac162b3f33b5d3721ef936cb1))
    - [commitgraph] Attempt to fix bash script execution on Windows. ([`5e78213`](https://github.com/Byron/gitoxide/commit/5e78213b1cd53986b8a39accf17da3456e496016))
    - [commitgraph] Use crate::graph::Graph instead of crate::Graph. ([`21e4527`](https://github.com/Byron/gitoxide/commit/21e45275221505b30f466a3b0223534d5a2281e5))
    - [commitgraph] Rearrange some `use` statements. ([`185d14b`](https://github.com/Byron/gitoxide/commit/185d14b25b8fc85308b1ba62391595dda51ce58a))
    - [commitgraph] Don't export Commit symbol at crate level. ([`be0e845`](https://github.com/Byron/gitoxide/commit/be0e845649b87acd3197ea212c78af8e0f9e22bf))
    - [commitgraph] Include Conor in crate manifest. ([`000748c`](https://github.com/Byron/gitoxide/commit/000748ccffc222729a7a1c1ce19c4fa1ba50fbed))
    - [commitgraph] Add some doc comments. ([`6cf5cd8`](https://github.com/Byron/gitoxide/commit/6cf5cd8da54e9d5670e3a44de95253df1091b110))
    - [commitgraph] Remove unused error variant. ([`66588f2`](https://github.com/Byron/gitoxide/commit/66588f227de8fd883a5f429821509e968c59b4fc))
    - [commitgraph] Rename GraphFile -> File. ([`f451822`](https://github.com/Byron/gitoxide/commit/f451822ec912253b2e5a5b0a63e1abd76939f58d))
    - [commitgraph] Rename CommitData -> Commit. ([`d8c2007`](https://github.com/Byron/gitoxide/commit/d8c20072fdce7cba249f4d9b5a0cba6136beb06f))
    - [commitgraph] Don't re-export graph_file symbols at crate level. ([`7c405ab`](https://github.com/Byron/gitoxide/commit/7c405aba660537999a24b6824198b3afb6dde529))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - [commitgraph] Ditch pre-generated test repos. ([`1ce8468`](https://github.com/Byron/gitoxide/commit/1ce84689ee89eb0f9e4f57cdba3a5ccac4a1a12d))
    - [commitgraph] Remove `Kind` enum. ([`3c92761`](https://github.com/Byron/gitoxide/commit/3c927610eb717645e7f83a257184e44f76918571))
    - [commitgraph] Take `info` dir as arg, not `objects` dir. ([`36953e0`](https://github.com/Byron/gitoxide/commit/36953e0ec6119e1a01ae9b8e46e40bbd083e732c))
    - [commitgraph] implement basic, low-level read API ([`d1f0e9c`](https://github.com/Byron/gitoxide/commit/d1f0e9cbd259b460a7d12ae068fb95ede0000cb2))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
</details>

## v0.0.0 (2020-08-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Reorganize git-commitgraph goals; add crate ([`21c9b75`](https://github.com/Byron/gitoxide/commit/21c9b7500cb144b3169a6537961ec2b9e865be81))
</details>

