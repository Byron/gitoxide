# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.8.0 (2021-10-19)

<csr-id-c5213d2b701ca71af5f3c987647e2a0c5c4d42dd/>

A maintenance release due to reset the entire crate graph to new minor releases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com//Byron/gitoxide/issues/222)

## v0.7.0 (2021-10-15)

<csr-id-8be4036dce4a857cc14a8b9467aaf2fc0fc2e827/>
<csr-id-ed16bce97c235e7e188444afd7a0d3f7e04a6c72/>

### BREAKING Changes

 - rename `oid::short_hex()` to `oid::to_hex()`
 - `oid::short_hex(len)` for truncated hex representations

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 11 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com//Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com//Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com//Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com//Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - format links for commit ids ([`9426db5`](https://github.com//Byron/gitoxide/commit/9426db53537162d58a65648f3f3a3a3b65f621dc))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com//Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - rename `oid::short_hex()` to `oid::to_hex()` ([`8be4036`](https://github.com//Byron/gitoxide/commit/8be4036dce4a857cc14a8b9467aaf2fc0fc2e827))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - oid::short_hex(len) for truncated hex representations ([`ed16bce`](https://github.com//Byron/gitoxide/commit/ed16bce97c235e7e188444afd7a0d3f7e04a6c72))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com//Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
</details>

## v0.6.0 (2021-09-07)

### Breaking

- `ObjectId::empty_tree()` now has a parameter: `Kind`
- `ObjectId::null_sha(…)` -> `ObjectId::null(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com//Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
</details>

## v0.5.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.5.1 ([`d826370`](https://github.com//Byron/gitoxide/commit/d826370b88d45fd2a421d3a59c232ed1504c6b0c))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 74 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com//Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com//Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com//Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] handle create-or-append when writing valid reflog files… ([`9175085`](https://github.com//Byron/gitoxide/commit/9175085248855a7ffa0d4e006740eafc0f4e1c92))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com//Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - thanks clippy ([`6200ed9`](https://github.com//Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com//Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.3.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 16 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com//Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - [traversal] trying to get things done with gitoxide shows some teeth… ([`3fee661`](https://github.com//Byron/gitoxide/commit/3fee661af8d67e277e8657606383a670f17e7825))
    - Nicer debug printing for oids, too ([`b4f94f8`](https://github.com//Byron/gitoxide/commit/b4f94f8af662bf6cdc001ca7b59478c701a40e36))
    - a new failing test ([`86b6c24`](https://github.com//Byron/gitoxide/commit/86b6c2497cfa17bf3f822792e3afe406f7968ee7))
    - fix git-hash docs ([`327a107`](https://github.com//Byron/gitoxide/commit/327a107afd696f7496e04bd6285c217cd8cdc136))
</details>

## v0.2.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com//Byron/gitoxide/issues/63)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com//Byron/gitoxide/issues/63)**
    - Revert "Add additional variant for Sha256 in ObjectId" ([`bb24dc4`](https://github.com//Byron/gitoxide/commit/bb24dc44beb6354fe2d96d2318d4d3219f06ae85))
    - Add additional variant for Sha256 in ObjectId ([`3dd7c43`](https://github.com//Byron/gitoxide/commit/3dd7c4350e140b72c21598f95a4557e6115d3124))
    - Make ObjectId into an enum to soon hold more bytes (and type) ([`4bf0c1a`](https://github.com//Byron/gitoxide/commit/4bf0c1a5a5c23bb0c0836ab8cea41eb06a232906))
    - Impl == and != for common combinations of ObjectId/oid ([`2455178`](https://github.com//Byron/gitoxide/commit/24551781cee4fcf312567ca9270d54a95bc4d7ae))
    - Remove now unused gith-hash::borrowed::Id ([`59ab1bd`](https://github.com//Byron/gitoxide/commit/59ab1bd9a8ea57e1770caf8841a0af5d38905bec))
    - More general to-hex for ObjectId ([`e2be868`](https://github.com//Byron/gitoxide/commit/e2be868ad4a131682d4aae629ca5b3a5b7ed0d5f))
    - Fix incorrectly implemented display for `oid` ([`c4186b0`](https://github.com//Byron/gitoxide/commit/c4186b0a986b4b49f8aa70308b492063bd33285c))
    - git-commitgraph uses `oid` now ([`0b72966`](https://github.com//Byron/gitoxide/commit/0b72966249523b97fce1bc7b29082ac68fa86a4f))
    - Notes about future proofing `oid` type… ([`658c896`](https://github.com//Byron/gitoxide/commit/658c896690f9a5b63f08484e90837bd1338420a5))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com//Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - oid with even more conversions and better hex-display ([`eecd664`](https://github.com//Byron/gitoxide/commit/eecd6644b10ba1e2e8481287db85c67ea6268674))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com//Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Add quality-of-life parse() support for hex input ([`6f97063`](https://github.com//Byron/gitoxide/commit/6f97063b14eb3b38a36e418657fd50f80db7f905))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com//Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - A seemingly complete implementation of a referenced borrowed Id ([`b3fc365`](https://github.com//Byron/gitoxide/commit/b3fc36565157a7f9d2fc9cf1a3c009a20c66e661))
    - Fix doc string naming ([`59c3d45`](https://github.com//Byron/gitoxide/commit/59c3d454b61e6932aee0fce0f709ac214db08633))
    - Move git-hash::owned::Id into git-hash::Id ([`fdbe704`](https://github.com//Byron/gitoxide/commit/fdbe704b6c9ace2b8f629f681a0580b24749a238))
    - Make git-hash Error usage explicit (it's for decoding only) ([`4805cfc`](https://github.com//Byron/gitoxide/commit/4805cfc8d837bb111424b5e32f46d0fb9b12365a))
    - Rename `git_hash::*::Digest` to `Id` ([`188d90a`](https://github.com//Byron/gitoxide/commit/188d90ad463d342d715af701b03f0ed392c977fc))
 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com//Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
</details>

## v0.1.2 (2021-01-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.2 ([`d1b4436`](https://github.com//Byron/gitoxide/commit/d1b44369bcca34516c8bf86a540a4591d64ec9ba))
    - update tasks and dependencies ([`96938be`](https://github.com//Byron/gitoxide/commit/96938be512efd6d6ad26238f258865d7488098f4))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com//Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
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
    - (cargo-release) version 0.1.1 ([`4224c5b`](https://github.com//Byron/gitoxide/commit/4224c5b5ceeb6bd1dbe4aac46018be5cc82b77df))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com//Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
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
    - first incarnation of git-hash to separate concerns and resolve cycle ([`9803041`](https://github.com//Byron/gitoxide/commit/9803041c29c18f2976531c9b487e63cd90fa3e72))
</details>

