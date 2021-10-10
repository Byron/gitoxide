# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: #198, #67

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes (4eebaac)
    - Rebuild all changelogs to assure properly ordered headlines (4a9a05f)
    - Sort all commits by time, descending… (f536bad)
    - greatly reduce changelog size now that the traversal fix is applied (a0bc98c)
    - Fixup remaining changelogs… (2f75db2)
    - Generate changelogs with details (e1861ca)
    - Update all changelogs with details (58ab2ae)
    - Update changelogs (c857d61)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
 * **#67**
    - Assure pack-ids are actually unique, the simple way… (0509b4f)
 * **Uncategorized**
    - Update changelogs just for fun (21541b3)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.21.3 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.3 (223f930)
    - [smart-release #195] fix docs (8d7e132)
</details>

## v0.21.2 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.2 (d443644)
    - Bump git-pack v0.11.0 (5ae6ff5)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7)
    - [repository #164] generic write_object() (c569f83)
    - [repository #164] Support for refreshing the object database (46e10f8)
    - [odb #164] Add refresh() functionality (ee16d04)
</details>

## v0.21.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.21.1 (cb33c2f)
    - [odb #190] Read all eligble packed refs, no "pack-" prefix needed (ab250f7)
    - Bump git-pack v0.10.0 (e5e3c80)
    - Bump git-hash v0.6.0 (6efd90d)
</details>

## v0.21.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs (081faf5)
    - [odb #180] fix docs (bd50752)
    - [odb #180] refactor (eff21da)
    - Bump git-odb v0.21.0 (7b9854f)
    - [odb #180] add changelog (acf1193)
    - [pack #179] refactor bundle (420dca2)
    - [pack #179] refactor (ab6554b)
    - [object #177] move mutable objects to crate::* (c551c02)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - Release git-pack v0.9.0 (7fbc961)
    - [pack #67] refactor (14717f6)
</details>

## v0.20.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.20.2 (6fb8bbb)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.20.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-odb v0.20.1 (ca3f736)
    - remove dev-dependency cycles by removing their version (c40faca)
</details>

## v0.20.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
</details>

## v0.18.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.18.0 (b327590)
    - (cargo-release) version 0.6.0 (d704bca)
</details>

## v0.17.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.17.0 (c52a491)
    - (cargo-release) version 0.5.0 (c2f94a5)
    - (cargo-release) version 0.4.0 (d69d0ac)
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
    - [utils #154] refactor: bool.then(||this) - neat (1dec1c4)
</details>

## v0.16.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 (8cd173b)
</details>

## v0.16.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 113 commits contributed to the release over the course of 83 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - (cargo-release) version 0.2.0 (8ff5115)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [pack] a way to obtain whole bundles for offset-to-index lookup (15fcbe2)
    - [pack] refactor (64b1dcd)
    - [pack] bundle::Location with pack offset; order counts by that… (f92f285)
    - Don't use ASM on windows for Sha1 as it fails to build there. (ba1fb7a)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - Bump thiserror from 1.0.25 to 1.0.26 (9682590)
    - [actor] fix gix hours (b4e95fd)
    - [actor] git-object uses git-actor (d01dd2f)
    - thanks clippy (3f7e27b)
    - (cargo-release) version 0.3.0 (6b33678)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - (cargo-release) version 0.2.0 (3286e42)
    - thanks clippy (c5b4de8)
    - [git-odb] linked::Store can now check if an object exists (bb95c79)
    - refactor (a25a774)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-ref] the first failing test (7e802a0)
    - [git-odb] fix test compiilation (639bc10)
    - [git-odb] much better docs; cleanup exposed API (3d5b229)
    - (cargo-release) version 0.2.0 (b213628)
    - [git-odb] refactor (2958145)
    - [git-odb] refactor (1eab15d)
    - [git-odb] refactor (4967c22)
    - [git-odb] refactor (2e68e0c)
    - [git-odb] fix docs (936cfd3)
    - [git-pack] compilation (b392a55)
    - [git-pack] refactor (ea2b3de)
    - [git-pack] refactor (5ca2547)
    - [git-pack] refactor (157b6ff)
    - (cargo-release) version 0.16.0 (769c649)
    - [git-pack] refactor (be6ddaa)
    - [git-pack] refactor (1fab6af)
    - [git-pack] refactor (e5b00ee)
    - [git-pack] the world compiles again (f0c0e36)
    - [git-pack] used by git-odb (5d6ee07)
    - [git-features] refactor to help understand a zlib-related logic bug (ae826e8)
    - [git-features] a first step towards supporting a pure rust zlib backend (040cab7)
    - [git-odb] refactor (e07478c)
    - [git-odb] fix docs (05347d4)
    - [git-odb] refactor (721303d)
    - [git-odb] refactor (ea224e9)
    - [git-odb] refactor (6a1b16a)
    - [git-odb] refactor (bae3980)
    - [git-odb] refactor (6b7400b)
    - [git-odb] refactor (19ab0cb)
    - [git-odb] refactor (47c4042)
    - [pack-gen] refactor (b5618ca)
    - (cargo-release) version 0.15.0 (d69d9fb)
    - Put prodash behind a feature toggle, too (966058d)
    - Put 'walkdir' behind a feature flag/make it optional. (1a3cc5b)
    - Put 'sha1' behind a feature toggle (4f326bc)
    - Put crc functionality behind a feature toggle (458fa6e)
    - Revert "[pack-gen] quick hack for obtaining the entry size more quickly" (4c36f92)
    - [pack-gen] quick hack for obtaining the entry size more quickly (ad6d007)
    - Revert "[pack-gen] remove tree-diff as traversal option." (2907a5f)
    - [pack-gen] remove tree-diff as traversal option. (8373671)
    - [pack-gen] fix docs (2548b48)
    - [pack-gen] basic progress for entry generation (953190d)
    - [pack-gen] the first barely working progress (5b89a0e)
    - [pack-gen] tests are green (34b6a2e)
    - [pack-gen] thanks clippy (3f948a4)
    - [pack-gen] the basics to get the program going (03b67b0)
    - [pack-gen] Use more light-weight lookups for all blobs (80ce34d)
    - [pack-gen] refactor (e0caf8d)
    - [pack-gen] a way to get the pack location by ID right away (5619efb)
    - [pack-gen] refactor (fcb9c5f)
    - [pack-gen] refactor (11ce2d8)
    - [pack-gen] And the fix - all green (202c704)
    - [pack-gen] with the addition of write-oack checks it actually fails (a9e46a6)
    - [pack-gen] it compiles and all tests are green, with tests for all parts (b3a0344)
    - [pack-gen] minor but relevant differences between 'existing' and 'existing_object' (5f18124)
    - [pack-gen] very close to a basic impl of count + entries-gen… (c927429)
    - [pack-gen] Fill the relevant information for later (932b439)
    - [pack-gen] the first test for object counts (67b1512)
    - [pack-gen] first sketch of how counting could look like (6ef0072)
    - [pack-gen] prep for counting stage (93fd425)
    - [pack-gen] tag handling for diff based traversal (e55906c)
    - [pack-gen] tag support for tree traversal (28ed260)
    - (cargo-release) version 0.10.0 (5d7ee6a)
    - [pack-gen] the first green test for Tag iterators (df5ef8a)
    - [pack-gen] A test to see we can handle tag objects (1898319)
    - refactor (9f0a8cc)
    - [pack-gen] Finally traversal based pack gen works too (086422b)
    - [pack-gen] diff-based traversal now finds all reachable objects (e819c92)
    - thanks clippy (760febf)
    - [pack-gen] add more objects during diff traversal (bc2ef19)
    - [pack-gen] pickup more trees (2da57bd)
    - [pack-gen] Specific tests show that something is off in the changes case… (b131c9e)
    - [pack-gen] central object synchronization for diff based algo as well (6de3558)
    - [pack-gen] have to keep track of all seen objects (dc645c6)
    - [pack-gen] updating tests to verify something shows that objects are duplicated (cef1a58)
    - [pack-gen] tree diff based pack generation passes the trivial test (ad0e345)
    - [pack-gen] refactor (cac002a)
    - [git-traverse] accept tree iterators instead of object ids (f343dad)
    - [pack-gen] Most of changes based pack gen (9ade04c)
    - [pack-gen] prepare diff based pack-gen (fa2ae2c)
    - [git-diff] refactor (087e853)
    - [git-traverse] refactor (85de287)
    - (cargo-release) version 0.3.0 (684de4b)
    - [pack-gen] Speed up tree-traversal :D (90b4093)
    - thanks clippy (009a342)
    - [pack-gen] Probably a valid impl of tree traversal (4c72a17)
    - [pack-gen] quite a bit closer to tree-traversal for pack gen (ecd37ee)
    - [pack-gen] refactor (325d63e)
    - [pack-gen] a test for upcoming traversal modes (8d1e9ac)
    - [pack-gen] refactor (08b136f)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.15.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.15.0 (d91b241)
    - (cargo-release) version 0.9.0 (84897fd)
    - Merge branch 'patch-2' (f01dc54)
    - refactor (a9e4feb)
    - Fix formatting (a341995)
    - Use Seek to skip large objects during indexing. (95e2af7)
    - Remove almost all unsafe code from Tree. (42b6033)
    - thanks clippy (17258cc)
    - thanks clippy (09decde)
    - Convenience methods for iterators (aa6c9e6)
    - refactor (d9783b9)
    - A sketch of convenient finding of commits (db21062)
    - refactor (3af7b7b)
    - sketch of convenience function for `Find` trait. (2bf4958)
    - refactor (bd4d21e)
    - refactor (8b10434)
    - Fix order of operations in git-odb::hash::Write (a31d8c7)
    - (cargo-release) version 0.14.0 (a760f8c)
</details>

## v0.14.0 (2021-05-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.14.0 (d9514ee)
    - rename 'Locate' to 'Find' - shorter and just as good (60f72f5)
    - (cargo-release) version 0.13.0 (5c791af)
    - [traversal] remove git-odb::traversal (now git-traverse::iter) (35b74d2)
</details>

## v0.12.0 (2021-04-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 153 commits contributed to the release over the course of 18 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: #67

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#67**
    - The very first version of complete pack writing (4d76d53)
    - A sketch of the pack::generation function signature (21b0aab)
 * **Uncategorized**
    - prepare test utilities for release… (d35e654)
    - (cargo-release) version 0.8.0 (a1ce210)
    - (cargo-release) version 0.3.0 (e9665c7)
    - [traversal] all the caching (0890403)
    - [tree-diff] first prototype of traversal experiment (ece43b4)
    - thanks clippy (2d5e205)
    - [tree-diff] more tests for the tree iterator (91b5a02)
    - [tree-diff] Now the commit graph traversal works with zero-allocations (8078910)
    - make it easy to get a commit iterator (33213f3)
    - [tree-diff] refactor into iterator based model (29b527a)
    - [tree-diff] The least intrusive way to allow dealing with tree iterators (d41dd3c)
    - refactor (a4d5f99)
    - refactor (03ee510)
    - Better ergonomics for accessing decoded objects (ae3eab6)
    - refactor (c1013dd)
    - refactor (f37c31f)
    - refactor (ac70651)
    - refactor (77764f3)
    - refactor (3185cc9)
    - Thanks, cargo audit (4f293f5)
    - refactor (edf7d38)
    - refactor (ca98221)
    - refactor (b4027e3)
    - refacto (6e328da)
    - fix docs (a54bab4)
    - refactor (3f2ee4c)
    - refactor (d6ab581)
    - refactor (d490b65)
    - Pack V2 writing (base objects only) seems to work now #(67) (e68dd84)
    - The first more thorough and indirect validation of the newly written pack… (d43687e)
    - refactor (08fafaa)
    - test newly written pack data alone (01fdd70)
    - Write pack data entries #(67) (722202e)
    - refactor (eed1e3c)
    - Write pack data header #(67) (717726b)
    - refactor (28cbeb3)
    - refactor (4261c7d)
    - All logic needed to write a valid pack within an iterator (775ab29)
    - sketch of pack data write API (dfeda87)
    - refactor (f33fa10)
    - [experiment/object-access] allow bare repositories (401690d)
    - thanks clippy (c86823a)
    - refactor zlib (4587b82)
    - refactor zlib (496e6bb)
    - refactor (3a4469c)
    - First basic pack entry generation (base only) works… (75cb32b)
    - refactor (d4bf8ae)
    - refactor (2d89222)
    - refactor (eb3a8da)
    - Allow calling 'finalize()' on the entries iterator (3c617bc)
    - refactor (b7d0323)
    - Reduce size of data::Object (7aa783a)
    - First pack-to-pack copy and crc32 verification (37619f0)
    - It's possible to get entry data within pack generation (a2a5927)
    - git-odb without cargo warnings due to using the same test twice (8945f95)
    - A way to obtain entry information using previous lookup information (a55d113)
    - refactor (95ab11b)
    - A probably more secure way of accessing pack data (7a01bd8)
    - sketch of pack-entry retrieval api (d1e9248)
    - refactor (08cf90a)
    - Let's be a bit more conservative with this information for now (efef417)
    - Objects know their pack location (73f1c66)
    - Chunks based iteration for pack generation (23c2694)
    - Some notes about how to treat defaults of file versions (cfa5cf6)
    - run git-odb tests in parallel, too; improved threaded error handling (40802fd)
    - the first test for pack generation (2a2fdde)
    - refactor (385f52d)
    - refactor  Please enter the commit message for your changes. Lines starting (f65c68c)
    - fix doc links (ec35743)
    - thanks clippy (563e445)
    - The first seemingly working iteration over all objects in an odb #(67) (6b34f62)
    - refactor (01d9d91)
    - impl size_hint for linked db iter (ada259b)
    - refactor (82c2f42)
    - refactor (7a6b514)
    - First sketch of object iterator for linked::Db (a316eed)
    - Set environment in testtools to freeze repositories generation scripts (eaad3ab)
    - faster repeated tests if fixtures don't change (792277f)
    - refactor (e1a92ad)
    - Allow the use of shared test utilities across crates (b117626)
    - refactor (40b86a7)
    - refactor (8b00094)
    - fix doc links (7479071)
    - thanks clippy (6f901f5)
    - ancestor iterator is now generic over Locate trait (bbfd616)
    - [fail] try to abstract ancestor::Iter over Locate trait (f8c0375)
    - refactor (5ef1f22)
    - Improve interface for building packs based on Locate trait #(67) (5b66b6e)
    - A version of the Locate trait we can do today #(67) (d752be2)
    - [git-odb] Associated types with lifetimes also don't seem to work (0e68a9d)
    - [git-odb] Trying to use offical traits won't work with our kind of object (29a5054)
    - git-odb::borrowed::Object => git-odb::data::Object (747a13e)
    - An even better name for decode errors (f270850)
    - Make clear it's a decode error we are using there (f45cb4b)
    - rename git-object::(owned->mutable)|(borrowed|immutable) #(67) (91ee558)
    - bump git-odb minor version (5c833ce)
    - thanks clippy (547af6e)
    - Fix test breakage for loose object reading (222c7a2)
    - fix docs #(67) (01db10a)
    - thanks clippy (60a7689)
    - refactor (ef674ff)
    - Remove loose::Object entirely #(67) (5cf4840)
    - Start using loose::Db::locate2 - definitely still bugs in there (d6f07b7)
    - An alternative version of loose::Db::locate() for use with borrowed::Object (5b40a32)
    - refactor (bad3ce4)
    - replace loose::Object::stream() with *::data() #(67) (040b347)
    - sketch loose::Object::data() to start refactoring #(67) (ee1701f)
    - Sketch of trait for locating objects #(67) (31445d7)
    - refactor (2754dd6)
    - refactor (3e908bd)
    - refactor (409d763)
    - refactor (896ab94)
    - Remove unsafe interface for stepped computation #(67) (c856613)
    - Show that with custom iterators, Arc's are natively supported #(67) (0c49007)
    - thanks clippy (405dd9d)
    - multi-tip support #(67) (2254ecc)
    - cache support for traversal #(67) (1e9300a)
    - cycle and duplicate check #(67) (334a72d)
    - a new failing test (86b6c24)
    - refactor (5408b62)
    - The first basic traversal utility #(67) (ea6610b)
    - Fix iteration signature due to shadowed naming (fe8b459)
    - thanks clippy (a463a43)
    - Sketch of machinery for producing pack entries #(67) (ac8e7fb)
    - A step towards using SteppedReduce #(67) (0d5595a)
    - (cargo-release) version 0.13.0 (ac2eddb)
    - Allow parallel reducers to produce something during 'feed()' #(67) (6c04fcd)
    - Allow more fine-grained stepping over the pack generator #(67) (22eb892)
    - Allow to obtain object information without fetching the data #(67) (6553850)
    - sketch a version to abstract object data retrieval #(67) (ad90446)
    - Implement `Write` trait for linked::Db (21362c3)
    - Docs for `linked::Db` (9d936de)
    - Support for caches within linked::Db (3635a3e)
    - `locate()` for `linked::Db` without cache for now (014bc3c)
    - refactor (7b443d1)
    - refactor (d077ead)
    - linked::Db::init() with a few tests (4c77e4c)
    - Frame for linked::Db (e64d148)
    - Make cycles in alternate object chains fatal (67e679a)
    - Resolve alternates as paths, not as repositories (73352c3)
    - Remove support for Polonius in preparation for a new repo type (871c803)
    - (cargo-release) version 0.11.0 (fd698e3)
    - Introduce pack_id for use in pack cache, preventing (most collisions) (ad04ad3)
    - Fix benchmark to get valid test results (20abb3a)
    - First use of memory-cap based LRU cache for object access (b057494)
    - Add hash-map based LRU to allow bigger/more effective object caches (5affdd5)
    - Feature toggle for uluru based Lru cache (98eec48)
    - refactor (d624d09)
    - Improve docs to prevent people to 'misuse' the Lru cache. (fff62ed)
    - LruCache with const-generics (93618d1)
    - [experiment] cached version of compound::locate() (ec988dc)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

## v0.10.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 4 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: #63

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - Impl == and != for common combinations of ObjectId/oid (2455178)
    - git-protocol uses `oid` type (3930a6f)
    - Use new `oid` where possible in git-odb (68a709e)
    - refactor; better errors for invalid hash sizes (be84b36)
    - Make ObjectId/oid happen! (ca78d15)
    - Remove all public exports of git-hash types in git-object (accf89d)
    - Remove re-export of git_object::borrowed::Id (a3f2816)
 * **Uncategorized**
    - (cargo-release) version 0.10.0 (3161777)
    - (cargo-release) version 0.7.0 (b900914)
    - (cargo-release) version 0.12.0 (3b71e7e)
    - (cargo-release) version 0.2.0 (4ec09f4)
    - Greatly reduce compound::Object size (afa8156)
    - The git-odb compound Object clearly is too large (8f0e813)
    - git-odb: add link to simplified/polonius version in the docs (d53c4b0)
    - git-odb: Only check alternates for objects not found in packs or loose (b317200)
    - git-odb: Avoid double-lookup in packs without polonius (eaae9c1)
    - thanks clippy (0c5f404)
    - Remove locate(…) -> Option<Result<…>> in favor of Result<Option<…>> (40ee743)
    - A locate returning Result<Option<Object>> for compound DB (a1dafa6)
    - Use Result<Option<Object>> in Bundle::locate() (2dfef8f)
    - A trial for Result<Option<Object>>  for loose object databases (3842859)
    - Assure loose objects are actually not found when opening (7a4f2bf)
    - Add feature toggle for polonius and make it part of the test suite (c825c11)
</details>

## v0.9.1 (2021-04-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: #59

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#59**
    - Fix initializing pack bundles in compound db (5a48e08)
    - Add failing test (d629339)
    - Move pack fixtures into place which resembles an actual object db (fb5cea4)
 * **Uncategorized**
    - (cargo-release) version 0.9.1 (e0feb28)
</details>

## v0.9.0 (2021-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 14 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 (efc8983)
    - thanks clippy (0fc239c)
    - refactor (f2e9add)
    - upgrade depdendencies (e4a7711)
    - Fix compile warnings produced by +nightly build (e387d2c)
    - Conform imports (fd73731)
    - Fix error type argument order and spell fields out (819568e)
    - [git-odb] Return error on invalid packs (88de64d)
    - [git-odb] Fix Inflate::once (36f6bbd)
    - [git-odb] Remove unnecessary tests (ebe41ca)
    - [gix] Use flate2 by default (f1158a1)
    - [gix] Add optional zlib feature (f1f9665)
    - [git-odb] Add feature flag for zlib-ng (96b3810)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.8.0 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (1ccfdcd)
</details>

## v0.7.1 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 38 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.1 (2c38ff9)
    - (cargo-release) version 0.11.0 (1aa1f5e)
    - Require latest version of git-features in git-odb (e664e93)
    - Remove usage of gitfeatures::fs in organize subcommand (b567d37)
    - refactor; planning (5df492c)
    - thanks clippy (343ab9a)
    - refactor (5b1328f)
    - Add missing '.' at end of doc comments (7136854)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix git-odb tests (35c1209)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - use git-hash in git-features (5b307e0)
</details>

## v0.6.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (27f5955)
    - (cargo-release) version 0.9.0 (a89fdb9)
    - (cargo-release) version 0.5.0 (fc7d600)
</details>

## v0.5.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 27 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (c767e07)
    - more docs for owned git-object (b79101d)
    - thanks clippy (ba9b3c2)
    - refactor (d5d7cf9)
    - more docs of git-object::owned (0620dce)
    - (cargo-release) version 0.8.0 (47c00c2)
    - cargo clippy Rust 1.48 (475a68c)
    - finish refactoring git-odb (ec282ae)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.4.2 (2020-11-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (7fa7bae)
    - (cargo-release) version 0.4.2 (173c957)
    - Minor fixes to git-odb docs (3788512)
    - complete docs for git-odb (0cf8496)
    - prefer empty doc strings for modules over [allow missing docs] (9b3f04f)
    - add remaining doc strings for git-odb (428f0ad)
    - Some more docs (2d87124)
    - try to document all the bits an pieces of git-odb (1b353fa)
    - Finish docs on top-level traversal method (2ef1c99)
    - start describing how pack traversal works (5e990f2)
    - refactor (a681335)
    - document pack::index::write (f5edc60)
    - dependency update (bc336d9)
    - refactor (6b909a2)
    - refactor (b511a2b)
    - document index integrity checking (9336ab9)
    - docs for index access (996acbf)
    - docs for top level pack index module (d2dd72f)
    - document pack data verification (27962ca)
    - document pack entry iteration (c869ee9)
    - docs for pack header (9505b40)
    - some more pack data file documentation (05e05f4)
    - docs for Bundle::write_* (ac41253)
    - remove special Error with just one variant… (d05a416)
    - Docs for Bundle::locate (066787c)
    - some more docs for 'pack' module (c32850d)
    - some more documentation (201f67c)
    - Merge branch 'commit-graph' into main (9cb09b2)
    - specify the hash to create with 'hash::bytes_of_file' (c000294)
    - move 'git_odb::hash::bytes_of_file' into git_features::hash (c5f6b45)
    - the daily commit (single handedly) (b528c2e)
    - document `loose::Object` entirely (d5eef9c)
    - thanks clippy (b9e0a87)
    - docs for Sink (e7a09f0)
    - docs for compound object databases (813df71)
    - Document borrowed odb objects (7626f7f)
    - Document alternates implementation (60666e8)
    - docs for git-odb crate (top-level) (71af366)
    - remove dash in all repository links (98c1360)
    - thanks clippy (e355b4a)
    - refactor (5a1cbf2)
    - And octal values unquoting works too (5effc7b)
    - All explicit escapes (1841544)
    - First bunch of simple unescapes (a45c594)
    - prepare for actual unescaping (284da44)
    - basic infrastructure for unquoting c-style strings (f81bb03)
    - fix incorrect cycle detection, which worked on MacOS by accident (a6e7765)
    - Also use alternates for looking up objects… (ade929d)
    - prepare for unquoting c-strings (47e2fa0)
    - Read multiple alternates from single file; ignore comments (1f8d367)
    - support for relateive alternates (b20e9ee)
    - Ignore all cycles and be happy if we have found at least one actual odb (1effdfd)
    - prepare for multi-line parsing and all the bells and whistles (08f9ec4)
    - Make compound DB initialization less lazy… (6dc57b3)
    - Use parallel walkdir (via jwalk) when parallel feature is enabled (f444c85)
    - alternate now handles cycles (71167e4)
    - first simple alternate tests (7372118)
    - test for circular alternates (fc92709)
    - thanks clippy (4ddc64f)
    - Actually resolve alternates when creating a compound DB (9be7aed)
    - refactor (c1eff58)
    - first sketch of alternate resolution (6cc8a94)
    - remove quickerror dependency from git-odb (7e27495)
    - refactor (7874c35)
    - refactor (3ec99dc)
    - refactor (519dd12)
    - refacator (7ac2153)
    - refactor (d4f288c)
    - refactor (3a8fb61)
    - refactor (98b3f4a)
    - refactor (127b8b2)
    - refactor (669b726)
    - refactor (7bc321e)
    - refactor (0752b45)
</details>

## v0.4.1 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 5 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (60ac8b0)
    - refactor (ad17bfd)
    - refactor (91d9f78)
    - refactor (6ebb5d1)
    - refactor (8877b77)
    - refactor (4a0d034)
    - refactor (485aa91)
    - refactor (c1d2f41)
    - refactor (07aff14)
    - refactor (57d463f)
    - refactor (c6be43d)
    - refactor (524d0fe)
    - refactor (a8f4cd7)
    - Checksum verification for compound object (3be08b0)
    - refactor (59d989a)
    - More methods for compound object (84d2b0e)
    - refactor (e5a9343)
    - refactor (6a84f13)
    - refactor (4e89c3b)
    - Document why we won't use nightly for fixing NLL issue (ca29368)
    - Revert "Fix NLL issue by using nightly" (6864a55)
    - Fix NLL issue by using nightly (8c5bd09)
    - Update tasks, prepare for NLL fix (52af8d1)
    - Thanks clippy (6c4d1ec)
    - This works, but locates twice… (4e709f6)
    - Also the imperative version doesn't borrowcheck… (c5720f1)
    - Looks like the functional approach to locate(…) doesn't borrowcheck (5df6867)
    - refactor (9e68c6b)
    - refactor (f219d5a)
    - sketch compound::Db::locate; sort packs by size (6609a53)
    - refactor (4a09754)
    - Implement Write in terms of writing to the loose object DB (02b88c2)
    - First sketch of compound Db (9bf2279)
    - refactor (203ba99)
    - (cargo-release) version 0.6.0 (9ef184e)
    - Revert "FAIL: try to get rid of tree-traversal Boxed error…" (1b42b31)
    - try to get rid of tree-traversal Boxed error… (13159eb)
    - Parameterize traversal error with Processor error (1513a13)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d9)
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 30 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (2272fa4)
    - (cargo-release) version 0.4.0 (0d7b60e)
    - (cargo-release) version 0.5.0 (82b7313)
    - [clone] All it took was a an intermediary to call 'read' as expected (7c8ecb7)
    - [clone] minor refactor; it's definitely the read() that doesn't work… (406829b)
    - [clone] This actually works: first MVP of retrieving packs via clone (c06d819)
    - [clone] support for progress that can handle writing pack files (46e0055)
    - Use fast-reset for miniz oxide to gain about 4s when resolving the kernel pack (e5b6ce4)
    - fix build (6178133)
    - refactor (174baa7)
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b879)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (b4a6e16)
    - remove tree compaction code (dfc6c7d)
    - See if tree compaction saves considerable amounts of memory (0092c25)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 413 commits contributed to the release over the course of 31 calendar days.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 (4351e28)
    - thanks clippy (6725104)
    - Also run file hashing in indexed mode in parallel (like with lookup) (8f8d14f)
    - first step towards parallelizing file hashes and traversal! (9573836)
    - allow hashing to be interrupted (df4dfd7)
    - unify file based file verification of data and index (e1b4105)
    - update to quick-error 2.0 (4b1b784)
    - Haha, funny, silly me… (a4a1244)
    - better usability for units (b226253)
    - better progress for Sha1 of pack and index (310a59e)
    - Make obvious that interrupt request was received (34b2373)
    - Conditionally use an eager iterator… (e9b5511)
    - Reduce progress information for threads (e9a1b31)
    - Revert "A test to see how much time can be saved by not zeroing zlib buffers" (3d51d59)
    - A test to see how much time can be saved by not zeroing zlib buffers (fd41a51)
    - Implement optionally keeping the compressed bytes (fc26914)
    - first step towards more control over allocation in iterator (cacf76c)
    - never keep decompressed bytes while streaming… (65c3856)
    - Only keep base objects, not the deltas (like originally intended) (fc8334b)
    - reduce footprint of sha1 when writing the index (12aa454)
    - first successful test of moving the streaming iterator into its own thread (c9fcb68)
    - first sketch of order-destroying eager iterator (20fca45)
    - add object size progress when resolving with index (b2f8c9e)
    - add decompression progress (0e5c534)
    - Print read throughput automatically (0a71b48)
    - Allow 'read' progress to go out of scope while keeping it accessible! (d7a7828)
    - Fix throughput display of otherwise stepped progress indicators (399f81d)
    - unify used ranges for line renderer amond pretty and lean interface (f59f66e)
    - Add percentage and throughput to tasks that matter (763d7ca)
    - Upgrade to latest iteration of prodash (3a4faec)
    - First part of migration to prodash 8.0, but… (6901a09)
    - Fix various issues related to 64bit offset support when writing indices… (da31694)
    - fix unit tests: actually sort the directory entries :D (b69717a)
    - Add convenience method to get a new bundle for the index/data just written (a6d74ad)
    - bundle write with a given directory (7f29c73)
    - first unit test for bundle writing (74bda39)
    - journey tests for restore functionality (1aa63e4)
    - refactor (fc42567)
    - refactor (cf3ebe0)
    - refactor (72ca435)
    - more flexible error types for processors - anything goes (be3a947)
    - refactor (c7dd581)
    - refactor (aae8e79)
    - refactor (0e27763)
    - Make lookup based algorithm gracefully interruptible (8d2e649)
    - Write about user interfaces and the use/non-use of async (91ba045)
    - Use pack hash for index file as well :D (2106c64)
    - support for interruptible operations (a025593)
    - thanks clippy (62d2ff3)
    - organize object type comparisons by probability… (19a5d94)
    - count object types as well (e04a8d1)
    - Revert "Less memory for look up mode, faster start" - too slow (584350a)
    - Less memory for look up mode, faster start (395c7e7)
    - remove petgraph entirely (70ba33a)
    - compute statistics for indexed pack verify (3d31c23)
    - prepare for computing indexed statistics (082c246)
    - refactor (bfbae90)
    - keep all metadata per object needed to compute the usual statistics (961b85e)
    - make 'level' available to support statistics (f7ba51c)
    - refactor (6277318)
    - support for error handling in traversal callbacks (c1d5bf6)
    - indexed traversal now works, in theory, but needs error handling (86f8400)
    - support for progress (62108fd)
    - support for thread local storage in callbacks (1dad088)
    - support for learning about the objects slice in the pack (faec782)
    - And even more caapbilities are required to make tree traversal work natively (90523bb)
    - refactor (2bbfd82)
    - refactor (efa7cd8)
    - first steps towards actually using the new tree traversal during verification (785b0ff)
    - thanks clippy (44b20de)
    - refactor (afe5e44)
    - refactor (fcc660d)
    - reduce memory usage for index considerably (aa802be)
    - and now it works! (f14e10e)
    - use new traversal in index writing, but it doesn't work yet (0dd5570)
    - refactor (4ff69c6)
    - refactor (6cbb7cc)
    - generalized tree traversal can theoretically work (64158e0)
    - make traversal part of the tree for greater ease of use (6629e30)
    - prepare flexible traversal on decompressed objects (7707ea6)
    - refactor (deea36c)
    - refactor (83a0102)
    - refactor (b77d148)
    - generalize tree iteration (fdc06de)
    - Also index using the new tree impl during verify (prepare replacement) (92039b0)
    - refactor (e3ff6af)
    - support for building a tree from offsets (95858bc)
    - refactor (8cfe025)
    - refactor (bb9e518)
    - count sorting into the progress, 7.5 mio entries takes a moment (2fc4cd8)
    - Use bigger buffers when reading from disk. (e76e4eb)
    - Only keep decompressed bytes of base objects… (b39ad89)
    - remove memory mode entirely (and some complexity with it) (8812e91)
    - turns out you never want to keep deltas in memory (657aa2c)
    - Remove support for keeping compressed memory to reduce the index size (1e2ec7e)
    - don't cause re-allocs of the compression buffer (2bb6fd2)
    - Revert "FAIL: try to use a customized version of just pieces of Miniz-oxide" (ea0fdb3)
    - try to use a customized version of just pieces of Miniz-oxide (9945eba)
    - dependency upgrade + update (c6692c6)
    - refactor (133e3ba)
    - Let go of another handbreak - decompression is much faster now (ae9dc16)
    - thanks clippy (393067d)
    - Use call to produce the resolver, allowing to delay opening a file mapping… (dd30e8d)
    - Fix actual memory violation (thanks to unsafe code) (c44c5e1)
    - thanks clippy (1083a0b)
    - Reduce memory consumption (6d1a7a1)
    - Unfortunately working with an optional for data is unwieldy, let's use default (12bbca0)
    - Tree can now be used as sole data structure, collecting all results (3e52d6f)
    - preparation for allowing reuse of the tree data structure (f565512)
    - refactor (9c4bc0a)
    - And it works! The new algorithm is sleeker, and really wants to be backported… (8e025b1)
    - thanks, clippy… (079ce9c)
    - Basis for re-implementing core algorithm using new Tree data structure (be6caf4)
    - refactor (290c29a)
    - incorporate proper filtering of bases (0880998)
    - overhauled iterator logic, still missing 'is_root' filter (2bfbae1)
    - First impl of the Iterator shows it's 'unknown' what a root node is (3f32938)
    - sketch on how children access could look like (16a35df)
    - How a referenced version would look like… (e36021d)
    - refactor (62a01fe)
    - more experimentation towards a safe tree data structure… (d907ce8)
    - first stab at new Tree datastructure… (85d7579)
    - Safety for handling base pack offsets (17d8375)
    - …but there seem to be issues with the kernel pack… (cc147bc)
    - quick and dirty impl of gitoxide layer for bundle writing, aka index-pack (e78386b)
    - cargo clippy (586ba7a)
    - implement in-memory mode; refactor (0c195b9)
    - refactor (c9d9298)
    - Use monomorphic calls only at the expense of code siz (40b28d1)
    - refactor (150d0bc)
    - Also implement the 'no output directory' branch (5a3240f)
    - refactor (68e52f8)
    - For the first time, writing an index could work with persistence (16e045c)
    - Don't write pack to file if everything is kept in memory (f3ddda6)
    - Allow data file to be optional in preparation for in-memory operation (95af105)
    - refactor (413968d)
    - refactor (5d27cdb)
    - optional pack lookup depending on the settings (2b509de)
    - Write-through the pack file as we receive it and move it into place (6180e39)
    - receive progress information when reading packs in bundle (759091d)
    - start supporting writing packs to disk right away (f2203e0)
    - refactor (75c333c)
    - prepare for implementing the bundle with various write modes (de420e4)
    - bundle thread progress underneath reducer progress (76b1b2b)
    - prevent deadlock, interestingly (ca02901)
    - refactor (ea254c0)
    - rough progress for writing the index (f1a7f9b)
    - initial batch of progress usage for index creation… (b10e5c6)
    - refactor (77b3c21)
    - refactor (fb23d15)
    - refactor (7da7e08)
    - refactor (5a3ad3a)
    - refactor (785a23d)
    - header encoding works now! As well as index writing :)! (024b854)
    - initial version of a complete header encoding impl, but… (ce6b46b)
    - looks like CRCs are not correct (3c4e4a0)
    - cargo clippy (a5596fb)
    - Fanout writing works now… (93a7ba9)
    - It's a good idea to remove old code from time to time… (9e47f1b)
    - fanout table, but slowly I get it :D (cfd8a25)
    - Fix decompression; fanout table is still wrong though (77fac1a)
    - Despite writing the CRC32 now, it doesn't work yet (ecd12b9)
    - first stab at streaming pack header encoding (3c6e78b)
    - refactor (5925d46)
    - Simplify offset handling in favor of allocating less (ce4ec62)
    - Only allocate memory for offsets if needed (72e0642)
    - First complete implementation of index writing… (826f996)
    - Reduce contention by using the shared cache only once (c370e13)
    - Optimize CRC handling - no need to assign it after the fact (ffcc03d)
    - Assure we can deltas store theyr resolved buffer (d2a81d9)
    - And it does seem to work! Awesome! (71cd982)
    - Delta-application could work if we handle our buffer better (ac6100b)
    - refactor (400a2a9)
    - One step before applying deltas (a074193)
    - prepare for delta application (9a9fb7a)
    - cargo clippy (d69c973)
    - parse pack header before trying to decompress :D (9d1b44a)
    - refactor (772e9ce)
    - consumer can resolve entries (13adce6)
    - refactor (c87f770)
    - refactor (d9d406d)
    - first version of resolver to copy from a memory map (506b8fd)
    - rethink resolver into something even simpler (4388c6c)
    - use parking_lot where possible (367874e)
    - Consumers can fail gracefully (9082080)
    - refactor (1b4cad0)
    - refactor (4ce13bb)
    - support for decompression in case compressed bytes are stored (c1fcf28)
    - computing hashes for bases from decompressed in-memory store works (7c19fe6)
    - show that all data can be passed for processing in threads (a95ce9c)
    - A cache usable from threads (1d4879a)
    - re-associate CRC32 with the correctly sorted ID output (037e1e5)
    - refactor (b3a365d)
    - refactor (97eb524)
    - Use chunked input and calculate 'optimal' chunk and thread sizes (0cc74d7)
    - generalize chunk iterator (905e85e)
    - first rough cut of in_parallel invocation (8f16081)
    - prepare for parallelization (cb36596)
    - Simplify indexing step (070899c)
    - resolver look ups may now turn out empty… (a991923)
    - Allow us to stop searching for bases early when resolving (e7874da)
    - This should be the interface for building indices from packs directly (f5295d0)
    - Got a good idea on how this will work! (7bb229f)
    - keep track of the pack trailer (cdba61e)
    - Now I understand why there is a separate resolution phase… (1c2bcbd)
    - Fix tests (b9866b6)
    - prepare a way to gradually implement V2 index writing (92a4986)
    - refactor (feba75b)
    - We can now restore (possibly half-written) packs (b1daa46)
    - prepare ability to restore pack files (76583e5)
    - Support for pack trailer verification when iterating (f37f131)
    - Also read the pack trailer during iteration (98a8e17)
    - Only take as many objects as we are allowed (without 'take(…)') (86f5853)
    - refactor (e15bde4)
    - Shift thin pack resolution to another work bucket; test for index writing (2592361)
    - refactor; better tests (12d14bf)
    - refactor (bd66a85)
    - Now keeping track of read bytes works (d32d921)
    - An attempt to intercept bytes read from bufread - FAIL (8db04f6)
    - refactor (2d817d7)
    - refactor (893f65b)
    - refactor (12816bc)
    - refactor (56f763a)
    - Associate HashKind with the kind of pack (d66d139)
    - Move all pack-related file handling to bundle; big refactor (f8b6e75)
    - first step towards putting the index file into position (d994c74)
    - initial interface trial for writing pack index files (936bdcc)
    - refactor; more thorough tests (82d87ce)
    - cargo clippy (b768b56)
    - At least make it configurable if to keep decompressed bytes or not (28ebcae)
    - And streaming iteration works, even though we are forced to allocate… (27d624d)
    - Yes, this really cannot work: StreamingIterator (b4df430)
    - In the moment we tried to actually return Entry<'a>, it didn't let me :D (8367955)
    - First steps towards making the InflateReader reusable (83a97d4)
    - Better error handling in iterator, fuse yourself (5ebacc4)
    - The next() impl shows that we should be less lenient (4521cab)
    - Provide entries which borrow from iterator (86eea13)
    - Provide a lifetime for iterator (and possibly its entries) (7852bd1)
    - first version of expected iterated data types (d5e7d31)
    - improved iterator constructors (fb71f04)
    - better handling of pack headers (0030bdb)
    - frame for a pack iterator (07d1096)
    - some more tests (9095728)
    - verification for pack objects (17bd95e)
    - refactor (3ee947e)
    - 'stream()' now assures all data is decompressed (32e994c)
    - it looks like something is wrong with the object stream implementation (d187b5a)
    - Loose object verifycation - but it doesn't seem to work as expected (9dd5676)
    - refactor (37cfd9b)
    - refactor (8e3b9fc)
    - prepare full 'verify' implementation (ee45c7f)
    - refactor (0a33b24)
    - Always compress values when using a sink when exploding packs (70562fa)
    - support for compression even when using sink (105c845)
    - Another stab at fixing stress tests :) (7db6a33)
    - fix stress test; improve progress messages (37ccd92)
    - Ignore decode errors (if configured) at the right spot (e53141d)
    - tests for relaxed error handling (93c0e26)
    - Nice error message on failure (adbc82c)
    - inform about deleted files using progress (a3ee516)
    - Fix error display - certainly something to watch out for (38eff2c)
    - The first 'explode' implementation… (0d31ad1)
    - Support for skipping various safety checks during traversal (0416666)
    - prepare for configuration of safety checks (06638d0)
    - cargo clippy (95e02c9)
    - Restore original verification functionality (0e3c1b9)
    - nearly there! Interesting that anyhow errors must be sync! (eaee77e)
    - finally it compiles with returning Boxed errors! Ouch… (1fc8252)
    - First sketch of new verify expressed in terms of traversal (4cb570f)
    - refactor (f2832a8)
    - Finally a progress can be passed to the delegate… (a9f4de0)
    - refactor (bbb3e1e)
    - Pass all arguments (but progress) to processor (1e87922)
    - Call a bare version of the traversal processor (95a5cea)
    - preparation for abstracting the 'process object (stateful)' function (fe400f5)
    - discard idea of making traversal even more generic (1525f36)
    - Initial step towards separating verification from traversal (d14b4fc)
    - refactor (bae7781)
    - rename verify-pack to pack-verify (keeping it more formal) (ec8c48a)
    - refactor (f580441)
    - Fast implementation for buffered input (c50b150)
    - Respect object size to be 64 bit where applicable… (61c8aba)
    - better errors for writing disk objects (f7bc137)
    - Try to use HashKind where possible (b32e01d)
    - refactor (a3777ed)
    - clippy happy (a938c70)
    - And writing of loose objects works (bbfe7bf)
    - This seems to be a working deflate write implementation (0acce38)
    - The first succesful inflate run for small input (94e1c5a)
    - what seems to be a reasonable write implementation for deflate (45a28d2)
    - Another test to understand the deflate streamer better (4256038)
    - refactor (dd463df)
    - refactor (0b42237)
    - refactor (5b0bb84)
    - Put down a few tests to understand how deflate wants to be fed (178a018)
    - refactor (0d8d7fe)
    - Improve looks of documentation (11a32eb)
    - Fix tests for now… (79ab945)
    - refactor (0cd7bb7)
    - Complete and unoptimized disk writer for objects, but… (9d0c3f1)
    - refactor (62e75bc)
    - Make use of HashKind in Write trait (0304dd0)
    - Make our Sink API similar to std::io::sink() (a03ae0f)
    - Finish Sink implementation (84f7908)
    - first steps towards serialization tests for sink (e8d52c6)
    - Introduce hash kind, as this should be specified when writing an object (f5d0acf)
    - A simple trait for writing owned objects and streams (68b7d7d)
    - (cargo-release) version 0.2.0 (76fe0ab)
    - (cargo-release) version 0.2.0 (0bb8314)
    - Use 'optimized' chunk size for 'less-time' algorithm (c8c23c0)
    - incorporate dynamic chunking into 'less-time' algorithm (295aa2f)
    - integrate new chunk size code into lookup code (a8422cf)
    - Simplify progress code using `inc()` (9e8df59)
    - Add 'inc()' convenience methods to progress (2e46c9b)
    - Run clippy first; pacify clippy (0a5b883)
    - use faster algorithm by default (bb45c3d)
    - Properly compute delta chain length by default (a93b894)
    - remove hits_to_live (3a3fae9)
    - attempt to auto-remove unusable deltas… (5dd8243)
    - Now with cache (and due to that, incorrect statistics for now) (efd28d2)
    - Make chunk statistics independent of traversal method (6225f36)
    - First working version of alternate object traversal, without cache (51b5eb6)
    - initial state for indexed lookup (acbcd79)
    - refactor; tests now fail with more than just not-implemented (310a2f7)
    - speedup entry sorting a little; use less memory (b4df372)
    - better index entries sorting progress (b4d7038)
    - prepare sharing even more code (61c76cf)
    - Make use of shared reducer in upcoming indexed verify implementation (290eae1)
    - Use shared reduce implementation in lookup based algorithm (10fc88d)
    - prepare for integration of general reducer (c37832e)
    - refactor; enable testing of reverse-delta lookup (512daf9)
    - Revert "Move deallocation off into own thread" - not worth it! (051da15)
    - Move deallocation off into own thread (90230f1)
    - Implement more cache-friendly pack offset v2 retrieval (00cf84b)
    - refactor (3c25c67)
    - initial refactor of DeltaTree, but… (6384649)
    - measuring performance of sorting index offsets is quite revealing (4b16336)
    - Properly handle the BufReader to make indexing work; FAST (57e95cf)
    - Avoid seek in favor of skimming a file read in bursts (01ae405)
    - Some performance information in progress (20aef2c)
    - Nodes now provide access to the pack offset (61c1497)
    - Basic tree access for the entry graph (c5e5c77)
    - Fix clippy (ec40e09)
    - hookup new indexing step (313064f)
    - frame for running the new streaming code on bigger packs (e0b34eb)
    - refactor (fdfab40)
    - refactor (1fbeb35)
    - refactor (385e935)
    - Now it works :D (008b4de)
    - Initial (failing) implementation of building an index tree (25dc83d)
    - Easy access to sorted offsets in pack index files (d93540f)
    - refactor (cb8d561)
    - refactor (c7ae705)
    - refactor (2fc449c)
    - Change course and do pack streaming first (bcb275e)
    - roundtrip Rust repo in stress test; accept more diverse trees when parsing (0347cdb)
    - Allow some very special trees not to be round-trippable (8fe1358)
    - Consume PGP signature in tags fully (ffd6c31)
    - make tagger signature optional (3358f9a)
    - remove now unused pgp_signature field - it's in extra-headers (c8c937c)
    - proper support for extra-headers (d0feb2b)
    - Switch to latest quick-error (9760856)
    - Fully implement --encode and --re-encode flags (a7cfac8)
    - refactor (56b66ac)
    - prepare for re-encoding each pack object (afae684)
    - fix build with rustc 1.45 (8c2a1ee)
    - refactor (ec5e50f)
    - prepare for writing out owned trees (2b6eced)
    - Use borrowed::Id in trees for full type safety (5d57c1f)
    - refactor (f7b8826)
    - fix odb test (a792f44)
    - Prepare for allowing an owned, processed version of multi-line headers (f966e7f)
    - Use borrowed::Id everywhere (9f876f0)
    - move git_object::Id into git_object::owned::Id - much better already! (50c7136)
    - basic integration of borrowed Id; translate between owned and borrowed (84ff638)
    - prepare to allow Id be owned and borrwed; abstract over hash type (d883c31)
    - introduce the notion of IdRef (7007361)
    - Use statically known borrowed arrays for perfect type safety! (3ead048)
    - refactor (766f3e4)
    - refactor (bca1f16)
    - 'data -> 'a as it's shorter and also more idiomatic (71821e9)
    - refactor (dedd4dc)
    - refactor (de0bc3c)
    - refactor (e5391d3)
    - refactor (163909b)
    - refactor (49f64db)
    - refactor (9f825b8)
    - refactor (2fbc2e1)
    - fix naming change, which was introduced accidentally (fbb9f98)
    - make it easier to validate bundles, for completeness (8ea05de)
    - refactor (34e85f2)
    - refactor (b3bde87)
    - refactor (0b540c2)
    - refactor (2888f1b)
    - refactor (0817b24)
    - refactor (dcacd3b)
    - refactor (b113da9)
    - refactor (6659174)
    - refactor (bed5dc8)
    - refactor (4867740)
    - refactor (f6cc80e)
    - refactor (8b416d4)
    - refactor (23e05d7)
    - refactor (d3b36f4)
    - More tests for various object types (f4703e0)
    - refactor (86fa00f)
    - Basic decode implementation (7ff02cb)
    - Support for in-pack object lookup in Bundle::locate (7e3d6be)
    - First dummy implementation of borrowing a buffer provided by the user (9c31fcb)
    - Make it easy to learn that objects couldn't be located by using options (a916f36)
    - mild refactor - need combined pack + index (6bf8ed4)
    - Respect thread limit in 'in_parallel' (babfd84)
    - apply cargo diet (79b9b73)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 167 commits contributed to the release over the course of 90 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add missing license description (2b80181)
    - Make crates publishable (5688a34)
    - cargo clippy (from CI) (0a28857)
    - Proper implementation of line renderer into 'lean' CLI (e98e7c2)
    - Handle windows newlines in test suite for packs as well. (ebd5176)
    - Add metadata to allow docs.rs build all featueres (10f9386)
    - update tasks (269280a)
    - Allow to limit the logging depth for less cluttered output (fce7035)
    - Looks like this performs much better already, but… ideally subprogress isn't shown (3b96d18)
    - finally speed up logging progress properly - needs input throttling (1a550c6)
    - provide average throughput per second (5b23d17)
    - git-odb with serde support (0da930c)
    - Remove dependency to git-object from git-features - it better remains free (67c3a6a)
    - commit to using bstr whenever something is not data bytes; remove miniserde (3183d1b)
    - Prepare centralization of bstr as optional component (aa857d9)
    - \#[forbid(unsafe)] for all crates (afda803)
    - Allow for more screen space when formatting (6794300)
    - prepare next task (74bcbb5)
    - display object throughput per second, even though it won't be visible in TUI… (53b4513)
    - disable LRU cache if we have to get statistics (befba3b)
    - wonderful statistics on compression efficiency! (1bb09c5)
    - count objects per chain level (209d53f)
    - pass average stats through to the top level (5b4979c)
    - refactor (4dd9fd4)
    - closer to actually producing statistics (5f087ec)
    - refactor (7add82c)
    - Also average statistics on chunk level (3b927e5)
    - Provide more detailed information when decoding an entry (80c5da9)
    - no need to say 'begin' before doing something, it's primarily for logging (13eba3a)
    - throughput for pack (81f5c33)
    - print performance stats at the end of hashing the index (9c94417)
    - assure hashing progress is dropped when done (db6e067)
    - First implementation of logging per thread (477dd90)
    - Support for providing progress to threads (2815858)
    - properly count objects (d398e7e)
    - first very basic progress implementation (b820717)
    - Pass progress everywhere, for now just to discard it (da3ae1c)
    - Control which hashing crates to use from the top-level as well. (dfe9b20)
    - Use git-features to toggle 'parallel' mode from the 'gitoxide' level (d944fbf)
    - sketch out `Progress` trait; don't forget to write docs at some point (534b3c7)
    - refactor (baeb4ef)
    - refactor (e12bfd6)
    - Make `in_parallel` trait bound more loose: Clone instead of copy (3e91b05)
    - Using all cores actually does speed things up (ed944b9)
    - Also run index+pack validation in parallel; only parallelize bigger packs (dc15b26)
    - avoid running anything in parallel for small packs (c2df183)
    - Don't send every single entry, instead send reasonably sized chunks (56298a6)
    - refactor (down to 6 minutes for big pack verification) (4157b51)
    - first working version of actually parallel `in_parallel` (145ee39)
    - first implementation of 'parallel' without threads. How will scoped fare? (735744e)
    - A sketch of a minimal helper for parallel work (377252a)
    - refactor (be4795f)
    - refactor (3e2efff)
    - bigger LRU caches are better, but with this one we can't go too large (5e1f7ae)
    - First implementation of an LRU cache - it gets hit, let's see how it fares! (5a21031)
    - also set the cache with bases and deltas (915a3fb)
    - first sketch of cache implementation - get() is there, next is put() (ce54756)
    - Allow delta base resolution to fail (similar to how lookups can fail) (b721424)
    - Allow in-pack lookups for V1 packs (2e51bbb)
    - Add CRC32 reading at index (268f855)
    - Pack offset by index (69e35b1)
    - V2 pack lookup (9e56902)
    - test V1 lookup (e9c7127)
    - Add CRC32 check during pack verification (04ff1a0)
    - prepare for CRC32 check - needs understanding of size of bytes in packed object (3ab2df1)
    - refactor (dd2d623)
    - Finally delta-objects can be read as expected. (81f2f54)
    - definitely an improvement to the way add-deltas are applied… (c6cdb12)
    - Fix one issue with Trees being declared as tags (ada66cd)
    - validate sha1 of pack objects, some work, some don't for some reason… (aa8799a)
    - Capability to write loose object headers, fast (de0aeff)
    - refactor (5364bbe)
    - Fix another implicit assumption that doesn't hold: deltas are NOT… (093637d)
    - Finish delta-application to take into account the biggest possible result… (0ee2b69)
    - first stab at dealing with bigger-than-expected intermediate result sizes… (8027ff4)
    - First simple implementation of fetching all objects in a pack (without validation) (053045b)
    - Support for verifying pack files and index files (b09b4e1)
    - simple index file verification (internal) (1d27050)
    - refactor (4023b02)
    - refactor (855a769)
    - refact[r (c84410b)
    - refactor (c24c79d)
    - test --no-default-features for git-odb (2394bca)
    - refactor; prevent trailing bytes to become part of the digets (043813c)
    - try a version that doesn't rely on memory mapped files for throughput… (d59ddfc)
    - try to speed it up with prefetching - not really :D (8485185)
    - simplify folder names (36fde1f)
    - Fix LSB parsing code with python based code written 6 years ago :D (c12fdad)
    - improved packed header parsing… it works a little better now it seems, but… (ca779ed)
    - refactor; and figured out what the header parsing issue is (d364049)
    - some more tests (85e541f)
    - refactor; better error handling (031df11)
    - first very rough version of full-object decompression without allocation (7c704a7)
    - refactor (dcb1997)
    - refactor (baaf06e)
    - refactor (3edaaec)
    - Finish Object Reader implementation, now for in-memory objects, too (35e69b8)
    - a simpler implementation to skip the header (47ca6ab)
    - Allow skipping the header when decompressing files (streaming) (ff35032)
    - first step towards supporting skipping the header in the stream (8e45f53)
    - Fix stream decoding - it seems to work, but we need to deal with the header (f10ed75)
    - tests for streamed reading of bigger objects (FAIL) (b4a6b72)
    - refactor (80aad4b)
    - Add missing parts to implement Read, need refactoring to make it work though (13d4cdb)
    - First step towards streaming of ZLIB deflated content (a870f7a)
    - cleanup (a2f0a5d)
    - fix clippy (a9c5da7)
    - Make decompression of bigger objects work (on the fly) (7e4f5a9)
    - It becomes obvious that this way of decompressing things won't work (1818bda)
    - Don't do so much logic if we already decompressed everything (26cb36c)
    - refactor (423b885)
    - more convenient access to our four object types (ecda6d2)
    - It's proably OK to make parsed pack entries avaialble, why not (8a64e10)
    - refactor (13f0e77)
    - Memory size checks for objects (ab51616)
    - Reduce loose Object memory footprint (38a81b0)
    - first Blob test for blobs that are already in memory (f503324)
    - Make single-field objects blob and tree more explicit (1aef68f)
    - add Blob type to parsed objects (d3e8e4b)
    - See 'parsed' blobs as in-memory representations… (6a6e105)
    - Make clear that not all objects can be parsed at the expense of convenience (ce3031d)
    - don't conflate errors with 'there is no suitable object' to parse (b9b796f)
    - fix imports (10f2967)
    - try pub use with rename. Not bad in the docs, but maybe a bit confusing (526f3f8)
    - refactor (b9a1647)
    - Integrate Commit object into Loose DB (7e9fe50)
    - test for parsing trees from loose dbs (4f48249)
    - refactor (9f9ccad)
    - refactor (427c480)
    - refactor loose db (6ea4f53)
    - handle commits without newlines; make tag newlines optional (c0b54be)
    - Make Commit available in borrowed object (b2d1b5d)
    - avoid unnecessary allocation when creating SHA1 paths in loose ODB (09d8d3a)
    - first silly attempt to randomly remove an allocation (4ff2168)
    - document existing use of unsafe, deny everywhere else (41f4bce)
    - cleanup integer parsing in loose object database (ecdce1a)
    - the defining property is actually that the object is borrowing data (e0125fd)
    - fix cargo fmt (642dd13)
    - cleanup; all tests work! (7c96603)
    - first version of tag message parsing - it's actually changed now (74b2328)
    - remove itertools in favor of vendoring the little code we need (8340508)
    - optimize macro usage (0c9960b)
    - optimize dependencies (3ea2195)
    - Use git-object in git-odb (07f7c31)
    - Add the latest nom, hoping it will be come out of alpha… (85958f1)
    - refactor; use pretty-assertions for massively more readable test-failures (ea8d311)
    - Switch everything parsed to BStr (62ae90a)
    - refactor (9a86f63)
    - Use btoi to parse all integers, directly from ascii-bytes (4f6ef42)
    - refactor (2990902)
    - move parsing tests close to actual parsing (3ca2c59)
    - move examples into demos, having their very own dependencies; optimize tests (b757712)
    - fix (untested) extraction of delta object information (55a56b7)
    - parallelize git-conut, optimize for speed (debd044)
    - refactor (9fc9fc0)
    - Fix big-pack 64 bit offset handling in index v2 (3b485b5)
    - make refactor (cd6a18a)
    - cargo clippy first pass (8b0a2a8)
    - Finally remove failure and equip example with anyhow (f5e4ec5)
    - remove failure from Index (55034a7)
    - And one more module without failure (d0575bf)
    - A big step towards removing failure (d862bd8)
    - refactor (87c8a2e)
    - git-core: get rid of failure crate in favor of quick-error (91c8fc1)
    - Get rid of nightly requirement, just parse tags differently soon (f037c4d)
    - cargo fmt (2aa0857)
    - reorganize repository a bit; use different contact email address (cb9fa28)
</details>

