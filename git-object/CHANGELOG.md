# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 11 calendar days.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - Rebuild all changelogs to assure properly ordered headlines (cfcaa66)
    - Sort all commits by time, descending… (7c37a3d)
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Generate changelogs with details (fd0f3bd)
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - feat: `BodyRef::without_trailer()` for more obvious access than `*body` or `body.as_ref()` (f0ea526)
    - refactor (ef3fc6d)
    - feat: `CommitRef::message_trailers()` as shortcut… (5324391)
    - more tests for trailers iterator (c3b0161)
    - feat: `BodyRef::trailers()` allows iterating trailer tokens and values (175e1cb)
    - Some tests and sketch for BodyRef parsing (3953c24)
    - feat: CommitRef::summary() and `MessageRef::body()` methods (1714d05)
    - refactor (7055dc8)
    - Another test for footer separation, simple version (b439186)
    - Return to safety (35313b9)
    - omg nom parsing works… (cd11704)
    - FAIL: not really successful to continue down the 'fold' road (d9afc22)
    - three tests failing with nom (13646e8)
    - Revert " FAIL: try to use nom-way of the previous body parsing…" (d1e6f62)
    - FAIL: try to use nom-way of the previous body parsing… (909f668)
    - sketch nom version of the message parser… (1ec47de)
    - Fix build (d0a956f)
    - refactor!: Use git_object::commit::MessageRef::summary()… (13e7c3a)
    - feat(commit): A summary for commit messages suitable for logs (cd3fc99)
    - More message parsing tests with windows line separators (001e8c2)
    - A manual message parse impl and more tests (f4b8a0d)
    - More message parsing tests, now with legit failure… (625be8d)
    - feat(commit): Add `message()` method and `MessageRef` type… (6150b2d)
 * **#67**
    - describe variants (899c579)
    - parse entry mode into number instead of comparing it to byte strings (83d591d)
    - ObjectID specific hashers, using the fact that object ids are hashes (f9232ac)
    - Tree parsing now probably is twice as fast… (d1e2b89)
 * **Uncategorized**
    - Merge branch 'main' into changelog-generation (c956f33)
    - thanks clippy (d78d382)
    - thanks clippy (4ea1126)
    - thanks clippy (e56af5a)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.14.0 (2021-09-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [object #164] refactor (883343b)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7)
    - [repository #164] generic write_object() (c569f83)
    - thanks clippy (33a8fb3)
    - [object #164] Allow referenced objects to be serialized as well (a98d298)
</details>

## v0.13.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.13.1 (2c55ea7)
    - Bump git-hash v0.6.0 (6efd90d)
    - [object #190] consistent method naming (c5de433)
    - [object #190] More conversion methods for Object (78bacf9)
    - [repository #190] A way to write objects and the empty tree specifically (7c559d6)
</details>

## v0.13.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [object #177] cleanup CommitRefIter imports and git_object::Error (058f68a)
    - [object #177] dissolve 'immutable' module (70e11c2)
    - [object #177] fix docs (2fd23ed)
    - [object #177] resolve 'mutable' module (b201b32)
    - [object #177] refactor (216dd0f)
    - [object #177] refactor (472e13b)
    - [object #177] Commit::write_to migration (60b9365)
    - [object #177]  commit::RefIter -> CommitRefIter (e603306)
    - [object #177] migrate immutable::commit into crate::commit (45d3934)
    - [object #177] refactor tag write_to (7f19559)
    - [object #177] tag::RefIter -> TagRefIter (28587c6)
    - [object #177] into_mutable() -> into_owned() (7e701ce)
    - [object #177] fix docs (25d8e7b)
    - [object #177] move mutable objects to crate::* (c551c02)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] fix docs (07be661)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - Release git-actor v0.5.0 (a684b0f)
    - [actor #175] refactor (ec88c59)
    - Release git-actor v0.4.0 (16358c9)
    - [actor #173] fix docs (2d7956a)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - Upgrade to nom-7 (f0aa3e1)
    - [smart-release #162] use TreeRef capabilities to lookup path (51d1943)
    - [repository #162] what could be a correct implementation of a tree path lookup (1f638ee)
</details>

## v0.12.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.2 (6e58edd)
    - [object] argh, remove these tests for now no time for this (13d627d)
    - [object] simply exclude the feature from testing for now… (adba3b9)
    - [object] fix magically smaller object size expectation (bf4d2d7)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.12.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.1 (086baa2)
    - remove dev-dependency cycles by removing their version (c40faca)
</details>

## v0.12.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
</details>

## v0.11.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.5.0 (bf15c2a)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
</details>

## v0.10.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 82 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.4.0 (0d5c8b9)
    - (cargo-release) version 0.2.0 (8ff5115)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref] fix build (bad find&replace) (467395f)
    - [ref] refactor (e26c72f)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - [ref] refactor (207a799)
    - [ref] flexible and simple support for different hash lengths (9c2edd5)
    - thanks clippy (c437304)
    - [object] Add feature toggle for verbose errors… (4b63d8a)
    - [object] support for verbose errors for object parsing (8156f10)
    - [object] refactor (6f63983)
    - Merge branch 'negotiate-fallible' (27c8abe)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97)
    - [object] remove unused dependencies (2f01e46)
    - [object] cleanup parsing error handling by removing NomDetail (e91cb40)
    - [object] refactor (1ddb5c0)
    - [object] replace custom context impl with the one by nom (9a6692d)
    - [object] refactor (8205429)
    - [actor] git-object uses git-actor (d01dd2f)
    - [actor] make signature parsing public, exposing nom :/ (a627972)
    - [refs] try to get structure in place for reflog parsing (727c66a)
    - thanks clippy (6200ed9)
    - (cargo-release) version 0.3.0 (87db688)
    - (cargo-release) version 0.3.0 (6b33678)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - (cargo-release) version 0.2.0 (3286e42)
    - (cargo-release) version 0.4.0 (866f86f)
    - (cargo-release) version 0.2.0 (1327894)
    - [git-object] use git-validate crate (4ba98e8)
    - [git-object] refactor (d64d326)
    - [git-ref] the first failing test (7e802a0)
    - Switch to latest nom (859e57e)
    - [git-ref] clear it out and move existing functionality to git-object (fa548ce)
    - (cargo-release) version 0.5.0 (b6b5856)
    - [pack-gen] refactor (61554e2)
    - [pack-gen] tag support for tree traversal (28ed260)
    - (cargo-release) version 0.10.0 (5d7ee6a)
    - [pack-gen] more tests for Tag iterator (b69d6d6)
    - [pack-gen] the first green test for Tag iterators (df5ef8a)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.9.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#79**
    - refactor; add test for empty tree iteration (6340296)
 * **Uncategorized**
    - (cargo-release) version 0.9.0 (84897fd)
    - Merge branch 'patch-2' (f01dc54)
    - Merge branch 'patch-1' (5edc076)
    - Allow empty trees when parsing them at once, fixes #79 (d34fd19)
    - refactor (9870923)
    - [hours-demo] computation seems to work better now (26ecca2)
    - refactor (2d00c4e)
    - [hours-demo] Maybe the pinnacle of performance… (f70c61a)
    - remove debug-assert which doesn't hold - it's OK to have empty commit messages (13abc2d)
    - And it's a wrap for git-diff docs for now (9e09dd5)
    - [traversal] first impl based on git-odb::traver (76a3017)
    - a new crate: git-traverse (1a9af50)
</details>

## v0.8.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (a1ce210)
    - (cargo-release) version 0.3.0 (e9665c7)
    - [traversal] add CommitIter::tree_id() convenience method (6affd9d)
    - [traversal] trying to get things done with gitoxide shows some teeth… (3fee661)
    - refactor; better iter error handling tests (9fe139b)
    - [tree-diff] more tests for the tree iterator (91b5a02)
    - test error handling of commit iteration (fcec4b4)
    - thanks clippy (41418ed)
    - fix serde support for commit iter token (3bfcb49)
    - [tree-diff] all the tests for commit iter (7ebea87)
    - [tree-diff] more tests (4f81450)
    - [tree-diff] And there is a working commit iterator, needs more tests (d991847)
    - [tree-diff] A complete nearly working impl of a Commit iterator (4711821)
    - Frame for Commit iterator (796b74a)
    - first failing test for commit iterator; store two parents without alloc (8337514)
    - [tree-diff] one more test green + refactor (bc5549d)
    - [tree-diff] refactor into iterator based model (29b527a)
    - [tree-diff] The least intrusive way to allow dealing with tree iterators (d41dd3c)
    - [tree-diff] prototype an immutable tree iterator to avoid entry allocs (f38e5cd)
    - [tree-diff] A step closer to handling additions in a directory (a11f210)
    - refactor (a4d5f99)
    - refactor (633cba7)
    - First sketch of diff API (fc3f2b7)
    - Better ergonomics for accessing decoded objects (ae3eab6)
    - thanks clippy (8295548)
    - refactor (9d03843)
    - fix debug assert, thanks gitpython (fe954b9)
    - More explicit expectations towards entries in mutable Trees (d94f84c)
    - refactor (f19ea33)
    - An even better name for decode errors (f270850)
    - Make clear it's a decode error we are using there (f45cb4b)
    - rename git-object::(owned->mutable)|(borrowed|immutable) #(67) (91ee558)
    - The first basic traversal utility #(67) (ea6610b)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.7.0 (2021-04-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 112 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - Use new `oid` where possible in git-odb (68a709e)
    - refactor; better errors for invalid hash sizes (be84b36)
    - Make ObjectId/oid happen! (ca78d15)
    - Remove all public exports of git-hash types in git-object (accf89d)
    - Remove re-export of git_object::borrowed::Id (a3f2816)
    - Move git-hash::owned::Id into git-hash::Id (fdbe704)
    - Rename `git_hash::*::Digest` to `Id` (188d90a)
 * **Uncategorized**
    - (cargo-release) version 0.7.0 (b900914)
    - (cargo-release) version 0.2.0 (4ec09f4)
    - thanks clippy (cefbf3e)
    - upgrade depdendencies (e4a7711)
    - improved high-level docs for git-object (60036f2)
    - Add missing '.' at end of doc comments (7136854)
</details>

## v0.6.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - first round of git-object doc proof reading (524ce51)
</details>

## v0.5.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (fc7d600)
    - `deny(missing_docs)` for git-object (8525684)
    - more docs for owned git-object (b79101d)
    - a few more comments in git-object (171d269)
    - thanks clippy (ba9b3c2)
    - refactor (d5d7cf9)
    - more git-object docs (ba595f6)
    - more docs of git-object::owned (0620dce)
    - docs for git-object::borrowed (68e524d)
    - docs for git-object::borrowed::commit (c5c1df0)
    - Add and use borrowed::Id::null_sha1() (c717492)
    - Updated `expect` message (e8d8d93)
    - Update error message for type name (92cbb13)
    - Document borrowed odb objects (7626f7f)
    - remove dash in all repository links (98c1360)
    - Finish removal of rust 2018 idioms (0d1699e)
    - refactor (e4bcfe6)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (0d7b60e)
    - (cargo-release) version 0.4.0 (f9dd225)
    - [clone] proper parsing of V1 refs (d262307)
    - [clone] Don't expose hex-error in public interfaces anymore (92dab30)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (a0bebd1)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 71 commits contributed to the release over the course of 31 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
    - thanks clippy (62d2ff3)
    - organize object type comparisons by probability… (19a5d94)
    - don't cause re-allocs of the compression buffer (2bb6fd2)
    - Reduce memory consumption (6d1a7a1)
    - Also read the pack trailer during iteration (98a8e17)
    - refactor; better tests (12d14bf)
    - first step towards putting the index file into position (d994c74)
    - Improve looks of documentation (11a32eb)
    - Finish Sink implementation (84f7908)
    - Introduce hash kind, as this should be specified when writing an object (f5d0acf)
    - (cargo-release) version 0.2.0 (76fe0ab)
    - (cargo-release) version 0.2.0 (d350a13)
    - beautifully implement shared extra-header access (920d1ac)
    - roundtrip Rust repo in stress test; accept more diverse trees when parsing (0347cdb)
    - Make sure we write out trailing newlines properly in multi-line headers! (7f044c3)
    - Consume PGP signature in tags fully (ffd6c31)
    - Support for very special tree entry mode… (2be2c9d)
    - make tagger signature optional (3358f9a)
    - remove now unused pgp_signature field - it's in extra-headers (c8c937c)
    - proper support for extra-headers (d0feb2b)
    - Abiility to read mergetags (for now only these) as extra-headers (bd3a2db)
    - Switch to latest quick-error (9760856)
    - Fully implement --encode and --re-encode flags (a7cfac8)
    - empty trees are allowed, and they are special, too (6bed200)
    - refactor (56b66ac)
    - Basic top-level object round-tripping (e851cbe)
    - refactor (ec5e50f)
    - implement blob (f30caf4)
    - refactor (335e98a)
    - tree roundtrip (8b26a0e)
    - prepare for writing out owned trees (2b6eced)
    - manual deserialize implementation, for now (9f46efd)
    - Use borrowed::Id in trees for full type safety (5d57c1f)
    - refactor (f7b8826)
    - commit round-tripping works with multi-line signatures (b692b0a)
    - first attempts to roundtrip signatures shows I parse it wrongly :D (1b48367)
    - Prepare for allowing an owned, processed version of multi-line headers (f966e7f)
    - first attempt to round-trip multi-line headers (645ef94)
    - single-line header support (478c09e)
    - The first basic version of commit serialization (5319f64)
    - make reusing round-trip code easier (3b9d66c)
    - refactor (987787e)
    - Fix tests on windows, by ignoring them (512ed6c)
    - Use borrowed::Id everywhere (9f876f0)
    - move git_object::Id into git_object::owned::Id - much better already! (50c7136)
    - basic integration of borrowed Id; translate between owned and borrowed (84ff638)
    - prepare to allow Id be owned and borrwed; abstract over hash type (d883c31)
    - introduce the notion of IdRef (7007361)
    - Use statically known borrowed arrays for perfect type safety! (3ead048)
    - refactor (766f3e4)
    - tags can write signatures (a48275e)
    - tags can write a message properly (b590b77)
    - green tests as basic tags can now be serialied (62a02b4)
    - more tests for signature serialization (5000f30)
    - time serialization (1eb1e36)
    - prepare writing of time as part of signature (f560bc5)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e)
    - refactor (b4392e8)
    - some more boilerplate to actually implement complete ref name checking (087857a)
    - very basic first steps of validated serialization (d3fd5ff)
    - it's probably OK to consume the borrowed objects when converting them to owned (101ddd5)
    - try basics of roundtrip without consuming the source object (581794e)
    - refactor (bca1f16)
    - first sketch of owned Tag in preparation for round-tripping (fa2745a)
    - refactor (90ae25d)
    - refactor (256581b)
    - 'data -> 'a as it's shorter and also more idiomatic (71821e9)
    - refactor (dedd4dc)
    - apply cargo-diet (better late than never :D) (295fc81)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release over the course of 26 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable (5688a34)
    - Handle windows newlines in test suite for packs as well. (ebd5176)
    - Fixup text file tests on windows (2288088)
    - Add metadata to allow docs.rs build all featueres (10f9386)
    - git-odb with serde support (0da930c)
    - cut back on onnecessary annnotations: serde(borrow) (759915c)
    - serde support for all git-object types, incl. test (1ae8f9c)
    - learn from the best: with-serde -> serde1 (d651c21)
    - commit to using bstr whenever something is not data bytes; remove miniserde (3183d1b)
    - Prepare centralization of bstr as optional component (aa857d9)
    - add support for miniserde (f806647)
    - first gentle test of adding serde support selectively. (78d9bc0)
    - Allow for more screen space when formatting (6794300)
    - Pack offset by index (69e35b1)
    - test V1 lookup (e9c7127)
    - validate sha1 of pack objects, some work, some don't for some reason… (aa8799a)
    - Capability to write loose object headers, fast (de0aeff)
    - simplify folder names (36fde1f)
    - fix clippy (a9c5da7)
    - more convenient access to our four object types (ecda6d2)
    - even better trait derives (e78f9f6)
    - Better trait support for basic types (6617386)
    - Memory size checks for objects (ab51616)
    - Make single-field objects blob and tree more explicit (1aef68f)
    - add Blob type to parsed objects (d3e8e4b)
    - fix imports (10f2967)
    - try pub use with rename. Not bad in the docs, but maybe a bit confusing (526f3f8)
    - refactor (2ffd7fa)
    - refacto (ffc0089)
    - refactor (b9a1647)
    - test for parsing trees from loose dbs (4f48249)
    - refactor (9f9ccad)
    - Move git-object tests to top-level for separation and cleanness (df42a01)
    - Prefer integration level tests, but use unit-tests where appropriate (ec3be19)
    - run previously unused method of Tree (0d159c2)
    - Actually use the Tree object (635e735)
    - handle commits without newlines; make tag newlines optional (c0b54be)
    - Handle tags without newline; document fixture processing step (344a562)
    - Don't assume newlines in trees anymore (45d7c36)
    - Found huge issue with newlines polluting fixtures. (f182d22)
    - first tree implementation, which seems to work well (9694fcb)
    - boilerplate for tree parsing (48c4c07)
    - refactor (d48cafa)
    - Add conversion traits for Object<->Tag|Commit (7dcbd5d)
    - Make Commit available in borrowed object (b2d1b5d)
    - Use smallvec to save memory in the common case (single parent) (263835b)
    - more tests (56248fe)
    - Now gpg-signature parsing works correctly - thanks to peek(…) (7078dac)
    - first somewhat working version of single/multi-line signature parsing (dab5c65)
    - support single-line gpg signatures (71330b5)
    - support for commit encoding field (40bffe9)
    - more commit tests, next up: encoding (ca4d3aa)
    - first successful parsing of commit (b44765a)
    - parse BStr versions of hex-shas directly (e3a2b77)
    - parse parents (696e0a3)
    - Use BStr instead of Id to avoid parsing into something we might not use/need (7c97471)
    - factor out hex sha parsing (d650dd2)
    - refactor (0104f4c)
    - first stab at factoring header parsing into sub-parser (6f6ee8f)
    - first fixtures for commit parsing (551f2d1)
    - avoid unnecessary allocation when creating SHA1 paths in loose ODB (09d8d3a)
    - document existing use of unsafe, deny everywhere else (41f4bce)
    - cleanup integer parsing in loose object database (ecdce1a)
    - Add remaining tag tests, along with some fixes (06e22fb)
    - use bstr were possible (01dd4e2)
    - the defining property is actually that the object is borrowing data (e0125fd)
    - refactor (683360a)
    - move all tests into the top-level for nicer names basically :D (598901a)
    - refactor (0f01e9f)
    - refactor (87bbea4)
    - refactor; add more signature parsing tests (ba9c7de)
    - cleanup; all tests work! (7c96603)
    - fix whitespace (ebaaa00)
    - first version of tag message parsing - it's actually changed now (74b2328)
    - implement parse_signature with nom, starting to like it (ebdf205)
    - First part of parsing tagger signatures (5b43270)
    - generalize with Boxed error cause (824cd2c)
    - first seemingly awkward way of not discarding too much error information… (6f9a636)
    - refactor (fb287af)
    - the first sketch of parsing a tag with Nom and half-decent errors (4498dff)
    - Use git-object in git-odb (07f7c31)
    - Move all object related code into own crate… (605ef20)
</details>

