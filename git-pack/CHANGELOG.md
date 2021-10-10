# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### other (BREAKING)

 - <csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/> Avoid duplicate module paths in 'tree' and 'commit'

### New Features

 - <csr-id-d6c44e6ab8f436020d4fb235e423b018fd1e7a9f/> dynamically sized full-object speeds up diff-based object counting
   which is what happens when counting objects for fetches where only changed objects should be sent.
 - <csr-id-50cf610e8939812c3d2268c48835e2dac67d0c31/> `cache::Object` trait for caching and retrieving whole objects
 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes
 - <csr-id-5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc/> object cache size is configurable

### Fixed

 - <csr-id-d8fe8141e80a9e9a433b5e1a072b850325c806c8/> don't put more objects into the pack cache than needed.
   
   Previously when accessing a packed object, it would store the base
   object into the pack cache (if it wasn't retrieved from there)
   which is great if that operation is free.
   
   Since it isn't, it's better not to stress the cache with puts
   and trash more objects than necessary.
   
   Now only the last decompressed object will be put into the LRU cache.
 - <csr-id-faf6f813927720c5adf62102f9ce46606ff2617c/> don't include submodules in count,
   which avoids dealing with missing objects entirely. Those ominous missing objects where just git submodules after all.
   
   It's still a good idea to handle these gracefully though, git itself
   seems to ignore them, too, and so do we at least for now.

### Performance

 - <csr-id-f9232acf8e52f8cd95520d122469e136eb07b39f/> ObjectID specific hashers, using the fact that object ids are hashes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 27 calendar days.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: #164, #198, #67

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - Avoid duplicate module paths in 'tree' and 'commit' (2f2d856)
 * **#198**
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes (4eebaac)
    - Rebuild all changelogs to assure properly ordered headlines (4a9a05f)
    - Sort all commits by time, descending… (f536bad)
    - greatly reduce changelog size now that the traversal fix is applied (a0bc98c)
    - don't put more objects into the pack cache than needed (d8fe814)
    - Fixup remaining changelogs… (2f75db2)
    - Generate changelogs with details (e1861ca)
    - Update all changelogs with details (58ab2ae)
    - Update changelogs (c857d61)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
 * **#67**
    - ObjectID specific hashers, using the fact that object ids are hashes (f9232ac)
    - Use a custom hasher for 'seen' objects hashset… (70179e2)
    - don't include submodules in count… (faf6f81)
    - control pack and object cache size in megabytes (60c9fad)
    - Use 'cache::Object' trait where it matters (71c628d)
    - split data::output::count::objects into files (8fe4612)
    - cache::Object trait for caching and retrieving whole objects (50cf610)
    - object cache size is configurable (5a8c2da)
    - dynamically sized full-object speeds up diff-based object counting… (d6c44e6)
    - Count ref-deltas in thin packs as well (80c6994)
    - Add '--thin' flag to pack-create and pass it on (2664d73)
 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… (01f7b72)
    - Update changelogs just for fun (21541b3)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 (5ae6ff5)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] generic write_object() (c569f83)
</details>

## v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.10.0 (e5e3c80)
    - [repository #190] first shot at ancestor iteration… (85f1a48)
    - Bump git-hash v0.6.0 (6efd90d)
    - [repository #185] refactor (7604935)
</details>

## v0.9.0 (2021-08-27)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
   - `data::object::Object` -> `data::Object`
   - `data::entry::Entry` -> `data::Entry`

* **new methods**
   - `Find::find_tag_iter()`
 
### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs (081faf5)
    - [pack #179] refactor (76e66d1)
    - [pack #179] move Tree traversal cache private (34e45d7)
    - [pack #179] refactor (5a3677d)
    - [pack #179] refactor bundle (420dca2)
    - [pack #179] fix docs (7ad7a44)
    - [pack #179] refactor (ab6554b)
    - [pack #179] refactor (620d8a5)
    - [pack #179] add changelog (2102569)
    - Bump git-traverse v0.8.0 (54f3541)
    - Bump git-diff v0.9.0 (2e2e798)
    - [object #177] cleanup CommitRefIter imports and git_object::Error (058f68a)
    - [object #177] dissolve 'immutable' module (70e11c2)
    - [object #177]  commit::RefIter -> CommitRefIter (e603306)
    - [object #177] migrate immutable::commit into crate::commit (45d3934)
    - [object #177] tag::RefIter -> TagRefIter (28587c6)
    - [object #177] into_mutable() -> into_owned() (7e701ce)
    - [object #177] fix docs (25d8e7b)
    - [object #177] move mutable objects to crate::* (c551c02)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] fix docs (07be661)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - Merge branch 'git-ref-refactor' (5dbf753)
    - [pack #172] A note about empty packs in Bundle writer (09a777f)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [actor #173] fix docs (2d7956a)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - Release git-tempfile v1.0.0 (1238535)
    - cleanup imports (e669303)
    - Merge branch 'Byron:main' into main (dc58eca)
    - Allow creation of empty indices (d122fc7)
    - [pack #170] there can only be one (dce4f97)
    - [pack #170] clru allows for free lists, reducing allocation pressure... (4d820d2)
    - [pack #170] basic progress for resolution (ada0b96)
    - [pack #170] Basic entry resolution without progress (7461f31)
    - [pack #170] first step towards resolving in multi-threaded mode… (f3c21f9)
    - [pack #170] Don't double-lookup trees during traversal… (7b06829)
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" (811bb54)
    - [pack #67] Don't pre-fetch packed objects during counting (d08b673)
    - Release git-pack v0.9.0 (7fbc961)
    - [pack #67] refactor (14717f6)
    - [pack #67] Add cache debugging capabilities to git-features (8776c98)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671)
    - [pack #164] fix docs (08ee674)
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" (4c2ea21)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (8d49976)
    - [pack #167] a single-threaded special case for counting… (65e29de)
    - [pack #167] generalize over immutable insertions… (169f000)
    - [pack #167] refactor (6bf0f7e)
    - [pack #167] progress is handled by reducer… (a22f8e1)
    - [pack #167] Error handling for object input (0aac40c)
    - thanks clippy (d689599)
    - [pack #167] remove iterator based count objects impl… (7ec2f2b)
    - [pack] A non-iterator version of parallel object counting… (04fe855)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.2 (39a3f71)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.1 (045eb09)
    - remove dev-dependency cycles by removing their version (c40faca)
</details>

## v0.8.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - (cargo-release) version 0.18.0 (b327590)
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (d704bca)
    - (cargo-release) version 0.6.0 (4b71e15)
    - (cargo-release) version 0.5.0 (e21142b)
    - (cargo-release) version 0.17.0 (c52a491)
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (c2f94a5)
    - (cargo-release) version 0.4.0 (d69d0ac)
    - (cargo-release) version 0.6.0 (d58f37e)
    - (cargo-release) version 0.5.0 (1687e59)
    - (cargo-release) version 0.4.0 (28e58f6)
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.4.0 (70ef344)
    - [utils #154] refactor: bool.then(||this) - neat (1dec1c4)
    - Revert "break more dev-depedency cycles up to git-odb" (22337ce)
</details>

## v0.3.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 (8b24197)
    - break more dev-depedency cycles up to git-odb (7ee278b)
</details>

## v0.3.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 136 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - (cargo-release) version 0.5.0 (0e11e98)
    - [pack #153] finish transitioning to git-tempfile (38173fc)
    - thanks clippy (e1964e4)
    - [ref #139] add missing docs (5422ec8)
    - [pack] refactor (581fb51)
    - [pack] refactor (b19f6b9)
    - [pack] fix docs (e7b9d96)
    - [pack] fix build (98dd557)
    - [pack] update CRC values when changing entries to satisfy all consistency checks (990ea48)
    - [pack] fix trailer of last entry to match expected recomputed pack hash… (8d0ec7d)
    - [pack] refactor (1852e3e)
    - [pack] all tests running for now, but… (aec8439)
    - [pack] hacky proof of concept that this actually works… (6085a92)
    - [pack] on the way to 'quickly' get a proof of concept (cdc7582)
    - [pack] refactor (685cce6)
    - [pack] refactor (f822ebb)
    - thanks clippy (96ef0b0)
    - [pack] a quickly made iterator that writes input::Entries (116bdc4)
    - [pack] prepare a custom writing iterator for input::Entries… (a4d2764)
    - thanks clippy (bd517d6)
    - [pack] prepare bundle writer for yet another iterator wrapper… (33be1a1)
    - [pack] refactor (50861e6)
    - [pack] refactor (dc07225)
    - [pack] another todo down, the last one (3fc8c8f)
    - [pack] one more todo down, it should work now, right?… (69a9ff1)
    - [pack] fix thin pack support test… (4bdebdd)
    - [pack] definitely not working yet (690d9b7)
    - [pack] a step closer, new cases show up (75eaba3)
    - [pack] refactor (a8512f8)
    - [pack] improved test to validate a fix (e3eeeb1)
    - [pack] attempt to get a more realistic test, but… (2890737)
    - [pack] refactor (cabc1e5)
    - [pack] first succeeding test (f5da439)
    - [pack] first reasonably failing test showing that offset computation is indeed wrong (df1bc2f)
    - [pack] the first test for the lookup ref deltas iter (b162f9e)
    - [pack] Make use of thin-pack resolver when writing bundles… (9f43bf0)
    - [pack] handle the same ref-base correctly (2f94854)
    - [pack] thin pack resolver which might actually work (54f055a)
    - [pack] first sketch of resolver for thin pack entries (ee428e0)
    - [pack] refactor (a8fd70f)
    - [pack] thanks clippy (7c2fc89)
    - [pack] actually, this is how it works, so this code should be unreachable (8f359e1)
    - [pack] first step towards fixing bad-objects properly (3c96507)
    - [pack] discard bad-object tracking in favor of delayed handling (31ce008)
    - Revert "[pack] fix race to finally make pack-gen missing objects…" (ad0d2a8)
    - [pack] fix race to finally make pack-gen missing objects… (73394db)
    - [pack] it seems git is just skipping bad objects during pack-gen (0f29b82)
    - Revert "[pack] FAIL: See if not looking up the pack location speeds up counting…" (d03fe97)
    - [pack] FAIL: See if not looking up the pack location speeds up counting… (48c4930)
    - Revert "[pack] FAIL: speedup with Mutex<HashSet>" (df98edf)
    - [pack] FAIL: speedup with Mutex<HashSet> (f8aca03)
    - [pack] In single-threaded mode, use a huge cache for some speedup (aec8a9b)
    - [pack] fix offset index properly by using chunk-absolute offsets (461c1ee)
    - [pack] forcefully fix issue with incorrect partition point (290bd65)
    - [pack] test for parital pack without thin pack allowance… (1f48d3b)
    - [pack] pack-create with immediate counting and traversing… (b74a98f)
    - [pack] entry writer now supports deltas and it seems to work even (fcda6f0)
    - thanks clippy (cc61f82)
    - [pack] on-demand cache for pack-offset to id lookup (0bfdea8)
    - [pack] refactor (4bb3ce4)
    - [pack] thin pack offset to index lookup (121aca4)
    - [pack] refactor (372b9ce)
    - [pack] a way to obtain whole bundles for offset-to-index lookup (15fcbe2)
    - [pack] refactor (64b1dcd)
    - [pack] refactor (1d713b4)
    - [pack] refactor (cdf020a)
    - [pack] refactor (2ccefb2)
    - [pack] refactor; entry-iterator now produces delta-objects (5dc370b)
    - [pack] rough version of obtaining object indices for deltas (a58e270)
    - [pack] refactor (8cfa414)
    - [pack] pass all data to where it belongs to… (af5cb1f)
    - [pack] add the notion of thin-packs to the pack generator (a289bba)
    - [pack] build an index of pack ranges as well (4d6ab7b)
    - [pack] bundle::Location with pack offset; order counts by that… (f92f285)
    - [pack] better identify the currently implemented pack generation mode. (f9e3b3c)
    - [pack] refactor (f3dc3da)
    - [pack] refactor (9ee1e22)
    - [pack] refactor (78d46c1)
    - [pack] refactor (69af352)
    - change wording (6c82a16)
    - Bump uluru from 2.1.1 to 2.2.0 (52e274f)
    - Don't use ASM on windows for Sha1 as it fails to build there. (ba1fb7a)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - Remove unnecessary unsafe code (83e207a)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - Bump thiserror from 1.0.25 to 1.0.26 (9682590)
    - thanks clippy (6200ed9)
    - fix build (dbfa49a)
    - Fix everything up so that… (5930563)
    - A first attempt to make intrerupt tools work, but… (8fb8d37)
    - fix pack tests (7968467)
    - The last occurrence of the global git-features::interrupt usage gone (6820724)
    - another one (0a8ed0e)
    - And another one down (abce75e)
    - refactor (7f9be36)
    - And one less usage of the global interrupt handler… (5da57a3)
    - thanks clippy (3b2e765)
    - Make most interrupts local to the method or function (4588993)
    - [features] sketch of iterator to auto-check for interruptions (61d3a15)
    - [pack] refactor (25f04ba)
    - [pack] refactor (18cabb8)
    - [pack] also put counts in order for stable packs (f299160)
    - [pack] fix run of 'cargo test --all' (e7ecdc1)
    - [pack] a working in-order iterator (5fea926)
    - [pack] tests for error handling of in-order iterator (44892cc)
    - [pack] ground work for ordering in produced chunks (9680649)
    - [pack] also run multi-threaded tests as part of unit-tests (5d3006a)
    - Bump uluru from 2.0.0 to 2.1.1 (b6ac506)
    - [pack] hopefully fix tests on CI; verify determinism of pack (51dec8b)
    - [pack] deterministic single-threaded pack generation (ddb6442)
    - [pack] refactor (cfdf802)
    - [pack] basic statistics for entries (37229a6)
    - thanks clippy (18b2113)
    - [pack] write packs to a directory with the proper name (3fbca7d)
    - [pack] refactor (f10adea)
    - [pack] fix docs (6ba471d)
    - [pack] fix build (81ee633)
    - [pack] statistics for counting objects seemingly work… (4e3deb1)
    - [pack] actual counts statistics (3a9f6d8)
    - [pack] aggregate the count outcome (c7ac0e6)
    - [pack] use statistics reducer (0974ab1)
    - [pack] count object reducer sketch (ea45692)
    - [pack] refactor (fdf485a)
    - [pack] refactor (0514f1d)
    - [pack] refactor (37922d1)
    - (cargo-release) version 0.3.0 (6b33678)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - (cargo-release) version 0.2.0 (3286e42)
    - refactor (a25a774)
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports (de2ba3c)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
    - [git-ref] the first failing test (7e802a0)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

## v0.2.0 (2021-05-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (b213628)
    - [git-odb] prep release (4984ce3)
    - [git-odb] refactor (2958145)
    - [git-pack] fix docs (efd20d4)
    - [git-pack] refactor (ea2b3de)
    - [git-pack] refactor (bc4b7b1)
    - [git-pack] refactor (157b6ff)
    - [git-pack] refactor (49c1c3e)
    - (cargo-release) version 0.16.0 (769c649)
    - [git-pack] refactor (be6ddaa)
    - [git-pack] used by git-odb (5d6ee07)
    - [git-pack] refactor (1b2a245)
    - [git-pack] move hash-writer to git-features as it's quite general purpose (80e5640)
</details>

## v0.1.0 (2021-05-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-pack] the very first version… (8c06cdb)
</details>

