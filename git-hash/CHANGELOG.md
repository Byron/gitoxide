# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.9.2 (2022-02-01)

A automated maintenance release without impact to the public API.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 7 calendar days.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update changelogs prior to git-pack release ([`b7e3a4a`](https://github.com/Byron/gitoxide/commit/b7e3a4afdd6417a38aadad35c7f584617e7b47fa))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
</details>

## 0.9.1 (2022-01-23)

### New Features

 - <csr-id-bc89fc77354f7d8af6628364be18550c4a45c789/> Implement Display for hash kind
   This helps 'clap' and allows for a little more type-safety during
   declaration.

### Bug Fixes

 - <csr-id-d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e/> don't use panicking const fn just yet to not require rust 1.57

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Implement Display for hash kind ([`bc89fc7`](https://github.com/Byron/gitoxide/commit/bc89fc77354f7d8af6628364be18550c4a45c789))
 * **Uncategorized**
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - don't use panicking const fn just yet to not require rust 1.57 ([`d2e2ea0`](https://github.com/Byron/gitoxide/commit/d2e2ea0a9b9c5f756d8b02b4872e6950faa03b3e))
</details>

## 0.9.0 (2022-01-19)

### New Features

 - <csr-id-84e26a7f3cbae31210e100880a48d3b3e6d04013/> Assign version numbers to `Kind` and implement `TryFrom<u8>`
   This makes reading and writing the hash number easier for newer file
   formats.
 - <csr-id-ce673bfd9afee4a7872c6bcae1c39006b1747be7/> add `Kind::from_len_in_bytes()` const fn
 - <csr-id-9a0d8b810050f2acabca988c5ab24ebe93a5d260/> `Kind::len_in_bytes()` method
   It yields the amount of bytes needed to store the hash.

### Changed (BREAKING)

 - <csr-id-79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded/> rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()`
   This change was done in the name of consistency, as `from_bytes()` is
   used in many other git-* crates
 - <csr-id-1b75541c00b8a18000336a8a7eceae5beba1058d/> Remove `Kind:Efrom_len_in_bytes()` from public API
   It shouldn't be encouraged to assume the hash can be deduced from its
   length, also git doesn't assume this.
   
   If that would happen, we would have other problems though, so let's hope
   it doesn't happen nonetheless.
 - <csr-id-b12ee8a97904e6e90b6c08ad9e6804ee969bff41/> Remove `ObjectId::null_sha1()` from public API
   Use `Kind::Sha1.null()` instead if it's a value where the actual
   repository object hash doesn't matter.
 - <csr-id-eaf48bd75a3b778e31695257aedfbd008769f7bb/> rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()`
   This naming is consistent with typical Rust APIs and the naming used
   throughout the git-* crates thus far.
 - <csr-id-60a4eb5dd7f50949799c558a225146d442dcf936/> remove `Kind::new_sha1()` from public API
 - <csr-id-c079fbe2099bd0ba43e811e987a80ae14e15e131/> Kind::from_len_in_bytes() is infallible
 - <csr-id-2a799e662aa172c243b54d1df0dfc78501cb024f/> remove `ObjectId::from_20_bytes()` from public API
   Use `ObjectId::from()` or `ObjectId::try_from()` instead.
 - <csr-id-53c748d7f438f57e8119cdf04402bfeaa9f2a286/> remove various SHA1 specific hex utilities in favor of unspecific new ones.
   
   removed `to_sha1_hex()`, use `oid::hex_to_buf()` and `oid::hex_to_buf()` instead.
   remove `ObjectId::write_hex_to()` in favor of `oid::write_hex_to()`
 - <csr-id-67652cb5cf01c45291d6e117c31290c585bab9d1/> `oid::null_sha1()` replaced with `Kind::null()`
 - <csr-id-3363f1e61295810964ddb0c255eed87a87fe6539/> remove `ObjectId::from_borrowed_sha1()`
 - <csr-id-75b901eff177dade43a28e770920a2b2206ded69/> remove `ObjectId::to_sha1_hex_string()`
   Use `.to_hex().to_string()` instead.
 - <csr-id-b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c/> SIZE_OF_SHA1_DIGEST is now private
   Replace it with your own constant derived from
 - <csr-id-3373946d27c91169172e62a637a305ef1e5fbb9e/> rename `Kind::to_hex()` to `Kind::to_hex_with_len()`; add `Kind::to_hex()`
   The latter prints the oid in full.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 30 calendar days.
 - 92 days passed between releases.
 - 16 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Basic multi-pack index creation ([`89428b2`](https://github.com/Byron/gitoxide/commit/89428b2936fb0169606a543cf531bddaacb8187c))
    - multi-pack index writing complete with large-offset support ([`f7d5c7f`](https://github.com/Byron/gitoxide/commit/f7d5c7f815dbf52c668444b316ae2e1485463bcb))
    - Assign version numbers to `Kind` and implement `TryFrom<u8>` ([`84e26a7`](https://github.com/Byron/gitoxide/commit/84e26a7f3cbae31210e100880a48d3b3e6d04013))
    - rename `oid::try_from()` to `try_from_bytes()`, add `from_bytes_unchecked()` ([`79dc0d5`](https://github.com/Byron/gitoxide/commit/79dc0d5ba6fa31ddd5c075693ffdc6496c1eaded))
    - Remove `Kind:Efrom_len_in_bytes()` from public API ([`1b75541`](https://github.com/Byron/gitoxide/commit/1b75541c00b8a18000336a8a7eceae5beba1058d))
    - Remove `ObjectId::null_sha1()` from public API ([`b12ee8a`](https://github.com/Byron/gitoxide/commit/b12ee8a97904e6e90b6c08ad9e6804ee969bff41))
    - rename `Kind::null()` to `null_ref()` and `Kind::null_owned()` to `null()` ([`eaf48bd`](https://github.com/Byron/gitoxide/commit/eaf48bd75a3b778e31695257aedfbd008769f7bb))
    - remove `Kind::new_sha1()` from public API ([`60a4eb5`](https://github.com/Byron/gitoxide/commit/60a4eb5dd7f50949799c558a225146d442dcf936))
    - Kind::from_len_in_bytes() is infallible ([`c079fbe`](https://github.com/Byron/gitoxide/commit/c079fbe2099bd0ba43e811e987a80ae14e15e131))
    - refactor ([`7331e99`](https://github.com/Byron/gitoxide/commit/7331e99cb88df19f7b1e04b1468584e9c7c79913))
    - remove `ObjectId::from_20_bytes()` from public API ([`2a799e6`](https://github.com/Byron/gitoxide/commit/2a799e662aa172c243b54d1df0dfc78501cb024f))
    - fix docs ([`cd981e2`](https://github.com/Byron/gitoxide/commit/cd981e222af237c47fcfb74258de8fdfc04dfc1b))
    - remove various SHA1 specific hex utilities in favor of unspecific new ones ([`53c748d`](https://github.com/Byron/gitoxide/commit/53c748d7f438f57e8119cdf04402bfeaa9f2a286))
    - `oid::null_sha1()` replaced with `Kind::null()` ([`67652cb`](https://github.com/Byron/gitoxide/commit/67652cb5cf01c45291d6e117c31290c585bab9d1))
    - remove `ObjectId::from_borrowed_sha1()` ([`3363f1e`](https://github.com/Byron/gitoxide/commit/3363f1e61295810964ddb0c255eed87a87fe6539))
    - remove `ObjectId::to_sha1_hex_string()` ([`75b901e`](https://github.com/Byron/gitoxide/commit/75b901eff177dade43a28e770920a2b2206ded69))
    - SIZE_OF_SHA1_DIGEST is now private ([`b596fa0`](https://github.com/Byron/gitoxide/commit/b596fa0dbbb3cc1d3ac386458ef52e2db9bca55c))
    - rename `Kind::to_hex()` to `Kind::to_hex_with_len()`; add `Kind::to_hex()` ([`3373946`](https://github.com/Byron/gitoxide/commit/3373946d27c91169172e62a637a305ef1e5fbb9e))
    - add `Kind::from_len_in_bytes()` const fn ([`ce673bf`](https://github.com/Byron/gitoxide/commit/ce673bfd9afee4a7872c6bcae1c39006b1747be7))
    - `Kind::len_in_bytes()` method ([`9a0d8b8`](https://github.com/Byron/gitoxide/commit/9a0d8b810050f2acabca988c5ab24ebe93a5d260))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - Very rough version of repository verification ([`80a4a7a`](https://github.com/Byron/gitoxide/commit/80a4a7add688d16376b9bf2ed7f1c7f655b7c912))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
 * **Uncategorized**
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Better not have items within items in changelogs ([`6946125`](https://github.com/Byron/gitoxide/commit/69461254b1bfda5e60911164096e4a061e241296))
    - thanks clippy ([`d8925f5`](https://github.com/Byron/gitoxide/commit/d8925f5bd7ac8ef2c98f0e57a1373e5ffba8ce23))
</details>

## v0.8.0 (2021-10-19)

<csr-id-c5213d2b701ca71af5f3c987647e2a0c5c4d42dd/>

A maintenance release due to reset the entire crate graph to new minor releases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

## v0.7.0 (2021-10-15)

<csr-id-8be4036dce4a857cc14a8b9467aaf2fc0fc2e827/>
<csr-id-ed16bce97c235e7e188444afd7a0d3f7e04a6c72/>

### BREAKING Changes

 - rename `oid::short_hex()` to `oid::to_hex()`
 - `oid::short_hex(len)` for truncated hex representations

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 11 calendar days.
 - 38 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - format links for commit ids ([`9426db5`](https://github.com/Byron/gitoxide/commit/9426db53537162d58a65648f3f3a3a3b65f621dc))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - rename `oid::short_hex()` to `oid::to_hex()` ([`8be4036`](https://github.com/Byron/gitoxide/commit/8be4036dce4a857cc14a8b9467aaf2fc0fc2e827))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - oid::short_hex(len) for truncated hex representations ([`ed16bce`](https://github.com/Byron/gitoxide/commit/ed16bce97c235e7e188444afd7a0d3f7e04a6c72))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
</details>

## v0.6.0 (2021-09-07)

### Breaking

- `ObjectId::empty_tree()` now has a parameter: `Kind`
- `ObjectId::null_sha(…)` -> `ObjectId::null(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 20 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com/Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
</details>

## v0.5.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 6 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.5.1 ([`d826370`](https://github.com/Byron/gitoxide/commit/d826370b88d45fd2a421d3a59c232ed1504c6b0c))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 74 calendar days.
 - 102 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com/Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com/Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com/Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] handle create-or-append when writing valid reflog files… ([`9175085`](https://github.com/Byron/gitoxide/commit/9175085248855a7ffa0d4e006740eafc0f4e1c92))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com/Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.3.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 16 calendar days.
 - 21 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - [traversal] trying to get things done with gitoxide shows some teeth… ([`3fee661`](https://github.com/Byron/gitoxide/commit/3fee661af8d67e277e8657606383a670f17e7825))
    - Nicer debug printing for oids, too ([`b4f94f8`](https://github.com/Byron/gitoxide/commit/b4f94f8af662bf6cdc001ca7b59478c701a40e36))
    - a new failing test ([`86b6c24`](https://github.com/Byron/gitoxide/commit/86b6c2497cfa17bf3f822792e3afe406f7968ee7))
    - fix git-hash docs ([`327a107`](https://github.com/Byron/gitoxide/commit/327a107afd696f7496e04bd6285c217cd8cdc136))
</details>

## v0.2.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 86 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com/Byron/gitoxide/issues/63)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - Revert "Add additional variant for Sha256 in ObjectId" ([`bb24dc4`](https://github.com/Byron/gitoxide/commit/bb24dc44beb6354fe2d96d2318d4d3219f06ae85))
    - Add additional variant for Sha256 in ObjectId ([`3dd7c43`](https://github.com/Byron/gitoxide/commit/3dd7c4350e140b72c21598f95a4557e6115d3124))
    - Make ObjectId into an enum to soon hold more bytes (and type) ([`4bf0c1a`](https://github.com/Byron/gitoxide/commit/4bf0c1a5a5c23bb0c0836ab8cea41eb06a232906))
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com/Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - Remove now unused gith-hash::borrowed::Id ([`59ab1bd`](https://github.com/Byron/gitoxide/commit/59ab1bd9a8ea57e1770caf8841a0af5d38905bec))
    - More general to-hex for ObjectId ([`e2be868`](https://github.com/Byron/gitoxide/commit/e2be868ad4a131682d4aae629ca5b3a5b7ed0d5f))
    - Fix incorrectly implemented display for `oid` ([`c4186b0`](https://github.com/Byron/gitoxide/commit/c4186b0a986b4b49f8aa70308b492063bd33285c))
    - git-commitgraph uses `oid` now ([`0b72966`](https://github.com/Byron/gitoxide/commit/0b72966249523b97fce1bc7b29082ac68fa86a4f))
    - Notes about future proofing `oid` type… ([`658c896`](https://github.com/Byron/gitoxide/commit/658c896690f9a5b63f08484e90837bd1338420a5))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com/Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - oid with even more conversions and better hex-display ([`eecd664`](https://github.com/Byron/gitoxide/commit/eecd6644b10ba1e2e8481287db85c67ea6268674))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com/Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Add quality-of-life parse() support for hex input ([`6f97063`](https://github.com/Byron/gitoxide/commit/6f97063b14eb3b38a36e418657fd50f80db7f905))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - A seemingly complete implementation of a referenced borrowed Id ([`b3fc365`](https://github.com/Byron/gitoxide/commit/b3fc36565157a7f9d2fc9cf1a3c009a20c66e661))
    - Fix doc string naming ([`59c3d45`](https://github.com/Byron/gitoxide/commit/59c3d454b61e6932aee0fce0f709ac214db08633))
    - Move git-hash::owned::Id into git-hash::Id ([`fdbe704`](https://github.com/Byron/gitoxide/commit/fdbe704b6c9ace2b8f629f681a0580b24749a238))
    - Make git-hash Error usage explicit (it's for decoding only) ([`4805cfc`](https://github.com/Byron/gitoxide/commit/4805cfc8d837bb111424b5e32f46d0fb9b12365a))
    - Rename `git_hash::*::Digest` to `Id` ([`188d90a`](https://github.com/Byron/gitoxide/commit/188d90ad463d342d715af701b03f0ed392c977fc))
 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
</details>

## v0.1.2 (2021-01-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 26 calendar days.
 - 27 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.2 ([`d1b4436`](https://github.com/Byron/gitoxide/commit/d1b44369bcca34516c8bf86a540a4591d64ec9ba))
    - update tasks and dependencies ([`96938be`](https://github.com/Byron/gitoxide/commit/96938be512efd6d6ad26238f258865d7488098f4))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
</details>

## v0.1.1 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`4224c5b`](https://github.com/Byron/gitoxide/commit/4224c5b5ceeb6bd1dbe4aac46018be5cc82b77df))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
</details>

## v0.1.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - first incarnation of git-hash to separate concerns and resolve cycle ([`9803041`](https://github.com/Byron/gitoxide/commit/9803041c29c18f2976531c9b487e63cd90fa3e72))
</details>

