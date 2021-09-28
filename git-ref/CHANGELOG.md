# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 13 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Generate changelogs with details (fd0f3bd)
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - add panicking `Target::id()` and `TargetRef::id()` (4ed4b2d)
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - git-ref(docs): improve changelog format (90e6128)
 * **Uncategorized**
    - Merge branch 'main' into changelog-generation (c956f33)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.7.3 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.3 (b0a9815)
    - Update changelogs once more… (d57d279)
    - Update changelogs retro-actively… (78cfe0a)
</details>

## v0.7.2 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.2 (e940e9a)
    - [#195] Fix previously incorrect usage of io::Kind::Other… (4dae07d)
    - thanks clippy (4701296)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.1 (d34191d)
    - Bump git-object v0.14.0 (d4fc81f)
</details>

## v0.7.0 (2021-09-07)

### Breaking

* Replace `transaction::Create` with `transaction::PreviousValue` and remove `transaction::Create`
* Remove `file::Reference` in favor of `Reference`
* Move `file::log::Line` to `log::Line`
* `TargetRef::Symbolic(&BStr)` -> `TargetRef::Symbolic(FullNameRef)`
* replace `Transaction::namespacce()` with `file::Store::namespace`
 
### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #190] refactor (e7188e0)
    - [ref #190] refactor (010be48)
    - [ref #190] fix tests (e426e15)
    - [ref #190] don't provide namespace support for loose and packed refs… (c663da1)
    - [ref #190] find() with namespace support (1240c21)
    - [ref #190] prepare test for namespaced find(…) (5fcd0e4)
    - [repository #190] leverage git-ref namespace support (1aa9c11)
    - [ref #190] iteration with namespace support (d5987d4)
    - [ref #190] refactor (3c7968c)
    - [repository #190] prepare for namespacing support on file store level (d2d1db6)
    - [repository #190] refactor (609c249)
    - [ref #190] refactor (1ef6cb3)
    - [repository #190] fix build (f5e118c)
    - [repository #190] note a known limitation about finding references in namespaces… (d335731)
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… (f58a0ff)
    - [repository #190] transparent namespace support (d14f073)
    - [ref #190] Make References sortable (16b2232)
    - [repository #190] cleanup usage of bstr… (e4411ff)
    - [ref #190] more conversion trait impls (1795a33)
    - Bump git-hash v0.6.0 (6efd90d)
    - [repository #190] obtain the kind fo hash used in a repo (a985491)
    - [ref #190] refactor (e34be7e)
    - [ref #190] more Target conversions… (1fe1b42)
    - [repository #190] refactor (7a111b1)
    - [ref #190] refactor (49fe1dc)
    - [ref #190] reverse reflog ergonomics (2de86f9)
    - [ref #190] check for zero sized buffers in reverse log iterators… (998c7c6)
    - [ref #190] move remaining file store functions to extension trait (60fc215)
    - [ref #190] Move file-log-specific functionality into own extension trait. (0b635e9)
    - [repository #190] a major step forward with `head()` access (43ac4f5)
    - [ref #190] cache peeled objects properly (2cb511e)
    - [ref #190] fix docs (3e64ec1)
    - Bump git-ref v0.7.0 (ac4413c)
    - [ref #190] fix remaining tests (df21f25)
    - thanks clippy (14dff63)
    - [ref #190] Use Raw Reference everywhere for great simplification… (7aeea9c)
    - [ref #190] raw reference peeling (9473a71)
    - [ref #190] introduce Raw reference type that simplifies everything… (8634341)
    - [ref #190] more tests (980e16a)
    - [ref #190] deletions also use PreviousValue now (74f85b1)
    - [ref #190] refactor (0e65559)
    - [ref #190] be explicit about what the previous reflog oid is for… (c04c8b9)
    - [ref #190] don't claim there was a previous oid unnecessarily… (68f7fc2)
    - [ref #190] refactor (07126d6)
    - [ref #190] Allow for explicit expected previous values (1a4786f)
    - [ref #190] prepare massive refactoring to get additional constraint (9741987)
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly (63bedc7)
    - [ref #190] refactor (3f36a01)
    - [object #190] More conversion methods for Object (78bacf9)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.4 (bc5d860)
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… (65db010)
    - [ref] Out of bounds check to prevent legitimate panic (303608c)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.5.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.3 (e6a8020)
    - [ref #157] Support for unsorted packed refs and those without header (2724688)
</details>

## v0.5.2 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.5.2 (50dcca9)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
    - [utils #154] commit manifest changes; create tags (95dcd9d)
    - (cargo-release) version 0.3.0 (263088b)
    - (cargo-release) version 0.18.0 (b327590)
    - (cargo-release) version 0.17.0 (c52a491)
    - (cargo-release) version 0.6.0 (d58f37e)
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.5.0 (bf15c2a)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
    - Revert "[ref] break dev-dependency cycle" (436e89b)
</details>

## v0.5.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 (6f61fca)
    - [ref] break dev-dependency cycle (d5af428)
</details>

## v0.4.1 (2020-12-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 94 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (25d2c2e)
    - Document `git-ref` (91dce23)
    - remove dash in all repository links (98c1360)
    - refactor (ba1d883)
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (f9dd225)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (63c1292)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (d350a13)
    - Switch to latest quick-error (9760856)
    - assert we don't exeed package sizes (df66d74)
</details>

## v0.1.0 (2020-07-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - refactor (6ad9304)
    - refactor (1fd90f7)
    - test for common ascii control characters (ae0c885)
    - all test for valid ref name except for ascii control chars (a157acf)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e)
</details>

## v0.5.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 394 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.4.0 (0d5c8b9)
    - (cargo-release) version 0.16.0 (1231dbd)
    - (cargo-release) version 0.2.0 (20d8e27)
    - (cargo-release) version 0.5.0 (0e11e98)
    - (cargo-release) version 0.2.0 (8ff5115)
    - [ref] refactor (501182b)
    - [ref #152] remaining tests for transaction namespacing (63d80c0)
    - [ref #152] first succeeding test for namespace rewriting (758c8f6)
    - [ref #152] first failing test for namespaced updates (a81f1d4)
    - [ref #152] refactor (f9c63fb)
    - [ref #152] namespace prefix stripping and fixed test expectations (bce135b)
    - [ref #152] a test for namespaced iteration (2338c6e)
    - [ref #152] packed-refs are optional for generalized iteration, too (88525a9)
    - [ref #152] FAIL: cleanup iter API by allowing Option<packed::Buffer> (1836243)
    - [ref #152] prepare namespaced iteration tests (cf5abc9)
    - [ref #152] no silent failure if path conversion isn't possible (8df04d8)
    - [ref #152] introduce Namespace type (67d5c85)
    - [ref #152] sketch API for namespaces (138be95)
    - [ref #152] docs (8d6c856)
    - [ref #152] refactor (bfb82fb)
    - [ref #152] all tests and impl for refname expansion (9cef2f2)
    - [ref #152] refactor (431dd86)
    - [ref #152] basic test setup for namespace expansion (e852399)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref #140] finish implementation of tag peeling, with test (c06e729)
    - [ref #140] refactor (edcc395)
    - [ref #140] sketch ref tag peeling (ef90652)
    - [ref #140] refactor (8e1a730)
    - [ref #139] add missing docs (5422ec8)
    - [ref #139] my first empty test but where else to document this :)? (0f00065)
    - [ref #139] refactor (a8f5d8d)
    - [ref #139] peeling for all refs to be written to a pack (cc891a1)
    - [ref #139] refactor (7e15817)
    - [ref #139] Allow packed-refs creation in the presence of updates (0cf7314)
    - [ref #139] impl of loose ref deletion, but it doens't work yet… (f6631ad)
    - [ref #139] a failing test for pruning loose refs into packed refs (437c610)
    - [ref #139] refactor (62558cb)
    - [ref #139] a first sketch to resolve object chains for packed ref peeling (54bc116)
    - [ref #139] Allow 'git pack-ref --no-purge' essentially (c32d8b7)
    - [ref #139] refactor (e5fbc4c)
    - [ref #139] refactor (4e1b95e)
    - [ref #139] refactor (42215a1)
    - [ref #139] a complete test for the first packed-refs mode (f332dcf)
    - [ref #138] delete packed-refs when it's empty after rewrite (8b7c359)
    - [ref #138] refactor (3fc0014)
    - [ref #138] no need for preprocessing, input is already checked (a6fca6e)
    - [ref #138] less is more… (6f39713)
    - thanks clippy (169a39d)
    - [ref] the first green packed deletion… (76a23b0)
    - [ref] refactor (packed refs aren't changed in memory) (0a7e8ce)
    - [ref] basic packed transaction commit impl, but it doesn't work yet (1913099)
    - [ref] fix order of operations when committing the transaction (be5774a)
    - [ref] refactor (69d53f9)
    - [ref] first revised sketch of packed-refs writing (f942c76)
    - [ref] work on first naive transaction, but… (b08cc4a)
    - [ref] tests incorporating packed-ref deletion (399096e)
    - [ref] validate packed refs are taken into consideration during create/update (25999b4)
    - [ref] allow creating new packed-refs files as well; prepare test arena (8494c74)
    - [ref] refactor (e379177)
    - [ref] refactor (a844146)
    - [ref] refactor (bd94ea5)
    - [ref] actually make use of packed refs in file transactions (7746238)
    - [ref] refactor (7a7b0dc)
    - [ref] refactor (74ed358)
    - [ref] first basic sketch of packed-ref transaction (8aac30c)
    - [ref] on the way to requiring a packed transaction for file transactions (85f30ac)
    - [ref] prepare existing refs to take packed-refs into account… (5849b44)
    - [ref] remove one todo, add another… (46c47ab)
    - [ref] all todos done (7632573)
    - [ref] refactor (fb37e96)
    - [ref] refactor (23ea139)
    - [ref] rev-iter for overlay references (8b28d4a)
    - [ref] refactor (a80b8c1)
    - [ref] tests for remaining todos (0ef6b3d)
    - [ref] remove loose::Reference backref to simplify everything (9f1d960)
    - Revert "[ref] back-reference of packed refs to their packed buffer" (464aefe)
    - Revert "[ref] FAIL: let's not add more back-refs, let's add less" (eaf4e9a)
    - [ref] FAIL: let's not add more back-refs, let's add less (8e90d75)
    - [ref] back-reference of packed refs to their packed buffer (da860ef)
    - [ref] refactor (61972a2)
    - [ref] refactor (f03c614)
    - thanks clippy (08f8bc4)
    - [ref] probably fix windows (6eb2532)
    - [ref] refactor (3df606a)
    - [ref] test for peel one level of packed ref (3d8602f)
    - [ref] assure packed-refs have a consistent target after peeling. (29a352a)
    - thanks clippy (321908e)
    - [ref] improve import paths (2dbe785)
    - [ref] refactor (49fc212)
    - [ref] prepare to create loose:Reference (8ed3916)
    - [ref] refactor (f222525)
    - [ref] finally peeling works again (d5bd75a)
    - [ref] packed-refs are now enforcing valid names (5d92919)
    - [ref] prepare peel test; realize another refactoring requirement (62f7155)
    - [ref] refactor (ae4d5da)
    - [ref] refactor (e26c72f)
    - [ref] refactor (f4bb7a0)
    - [ref] another test to run into one more todo (13502f5)
    - [ref] some TODOs to not forget (4d6a75c)
    - [ref] and it compiles again, may todos left (16618b9)
    - [ref] all required Reference methods are defined, but… (3c976a6)
    - [ref] refactor (65f7a7d)
    - [ref] changing the ref type means a lot of breakage and some unsolved problems (407dc4d)
    - [ref] refactor to be able to use loose_then_packed::Reference for top-level find (2c4e45a)
    - [ref] figure out how peeling works with packed-refs… (2801f7a)
    - Revert "[ref] FAIL: actually it's enough to give access to 'packed' when peeling only" (8dc6295)
    - [ref] FAIL: actually it's enough to give access to 'packed' when peeling only (5173a97)
    - [ref] put packed-ref lookups into the correct spot (6d11e22)
    - [ref] remove over-complicated refs store trait which… (1cc876c)
    - [ref] refactor (62e682c)
    - [ref] API sketch for allowing packed-refs to be used in find() (ca736ab)
    - [ref] fix windows build (f99851b)
    - [ref] assure names are using forward slashes in file-based refs (ff695e4)
    - [ref] prefix iteration for all references (228ca00)
    - [ref] improve structure; fix docs (aa6052a)
    - [ref] overlay really seems to work (d2ec30a)
    - [ref] more detailed overlay test (d747d73)
    - thanks clippy (636e1fd)
    - [ref] fix windows build… (65e6953)
    - [ref] first successful test for overlay iterator (5f92488)
    - [ref] conversion for packed refs (929bb0f)
    - [ref] loose refs iteration in overlay iterator (0b0f64d)
    - [ref] leverage sorted file iteration (036257e)
    - [ref] add setup for parallel file traversal tests (1306647)
    - [ref] reproducible loose ref iteration with built-in sorting (e138748)
    - [ref] sketch remaining overlay types, now on to 'next()' (6792cf1)
    - [ref] a way to obtain valid ref names along with their path for overlay iteration (bbaa1eb)
    - [ref] first steps towards test and impl for overlay iterator (f5d07b6)
    - [ref] add missing docs (e6052a5)
    - [ref] all remaining tests (ee9bc21)
    - [ref] first successful test for prefix filtering in packed refs (430549d)
    - [ref] run all performance tests (3635b25)
    - [ref] simple performance tests to get an idea of what it can do… (06bedcd)
    - [ref] perf 'test' for ref iteration (922d129)
    - thanks clippy (a39a68a)
    - [ref] rename find_one to 'find' in git-ref… (ae7746a)
    - [ref] refactor (758c090)
    - [ref] finish packed find() lookup testing (5f67c19)
    - [ref] refactor (953939c)
    - [ref] prevent unnecessary rounds for full names that aren't found (fb765de)
    - [ref] Assure ref-misses misses aren't parse-errors (d9d1360)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - [ref] fix compile warning on windows (c328774)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - [ref] refactor (140da9a)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
    - [ref] refactor (959abc7)
    - [ref] prepare for proper full-name conversion (0e6d3f2)
    - [ref] searching fully qualified reference names actually works. (9b2579c)
    - [ref] prepare find() impl… (b26dd1e)
    - [ref] assure packed-refs buffers are sorted (a797493)
    - [ref] refactor (897a49a)
    - [ref] windows fix; now maybe? (0e1a204)
    - [ref] windows pathname replacement: \ -> /… (94a1e02)
    - [ref] fix one test failure on windows (21f1031)
    - [ref] rough frame for finding packed refs (a24a54f)
    - [ref] learn more about the windows issue… (dde6276)
    - [ref] refactor (c150aba)
    - [ref] prefixed loose ref iteration (49ce1e2)
    - [ref] refactor; tests for prefix iteration (63566eb)
    - [ref] loose ref iteration with broken ref support (2d1234f)
    - [ref] maybe fix windows (6fc7784)
    - [ref] first rough implementation of loose ref iteration (918af42)
    - [ref] packed-refs iteration… (ea97e06)
    - [ref] docs for packed refs iterator (02690bc)
    - [ref] fix 'small' build (5fd10fe)
    - [ref] packed-refs iteration works, incl. decent error handling (e5a6b9d)
    - [ref] the first packed-refs iterator tests (f6d769e)
    - [ref] refactor (207a799)
    - [ref] flexible and simple support for different hash lengths (9c2edd5)
    - Revert "[ref] parameterize all uses of hash length…" (21f187e)
    - [ref] sketch of iterator (6c05243)
    - [ref] refactor (79184cf)
    - [ref] parameterize all uses of hash length… (5c7285e)
    - [ref] less lenient packed-ref header parsing (45b41e0)
    - thanks clippy (33f1b00)
    - [ref] refactor (de526b3)
    - [ref] first working packed ref line parsing (bc60229)
    - [ref] first test for line (and peeled ref) parsin (7af27c5)
    - [ref] refactor (b74913e)
    - [ref] refactor (d0eb819)
    - [ref] packed refs header line parsing (fde5543)
    - [ref] first rough steps to testing parsing a little (57659e9)
    - [ref] sketch packed refs, but… (8951b3f)
    - [ref] refactor + docs review (4b9b034)
    - [ref] the last TODO is gone (01dc422)
    - [ref] down to the last todo (23cea99)
    - [ref] two more tests but only one todo down (bf947d6)
    - [ref] the drop test (e472bde)
    - [ref] refactor (059f836)
    - [ref] refactor (7faf6f2)
    - [ref] adjust expectation to not do any special HEAD business (49d294a)
    - Revert "[ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions…" (8b0d7b6)
    - [ref] FAIL: realize that HEAD-reverse-lookup isn't done in transactions… (6098ba0)
    - [ref] test to validate HEAD update as special case of… (276aa9a)
    - [ref] refactor (861483a)
    - [ref] validate non-empty directories (8fb625d)
    - [ref] moving a ref onto empty directories works now… (a237f77)
    - [ref] refactor (ed40a87)
    - [ref] another complex test works (ebdbfae)
    - [ref] fix build (b4dcdfc)
    - [ref] try fix windows, once again (95e74dd)
    - [ref] refactor (a261b82)
    - [ref] probably fix windows (a8b7c8d)
    - [ref] allow reflogs to be created in place of empty directory trees (80a6e0e)
    - [tempfile] a way to delete empty dirs recursively (6025aa0)
    - [ref] refactor (21920ec)
    - [ref] refactor directory handling (45dbf22)
    - [ref] refactor (92867c5)
    - [ref] handle existng empty directories more gracefully… (0849c70)
    - thanks clippy (d967e30)
    - [ref] handle create-or-append when writing valid reflog files… (9175085)
    - [ref] refactor (1ee3419)
    - [ref] auto-creation logic for reflogs (80f71dc)
    - [ref] reflog creation test is quite complete (b67e79c)
    - [ref] allow commiter to be passed for use in reflog (80f5627)
    - [ref] tests for converting reflock paths into log paths (1f2e754)
    - [ref] refactor (a29fcf1)
    - [ref] frame for reflog creation or update (81cb790)
    - [ref] refactor (a76929b)
    - [ref] disambiguate create-or-update logic (585f369)
    - [ref] write out Create-or-Update logic to see that's its probably not going to cut it. (54d084f)
    - [ref] show how the original name can be displayed for lock failures… (07f0c2d)
    - [ref] write peeled previous OID through to parent refs (3355dd8)
    - [ref] fix child link transformation (5d9a685)
    - [ref] refactor (2f92f36)
    - [ref] sketch of inverting parent links for later oid lookup (a050f18)
    - [ref] refactor (1e88948)
    - [ref] add reflog message to change… (b31e103)
    - [ref] sketch more detailed test for updating reflogs (5a657cd)
    - thanks clippy (eb8ea22)
    - [ref] the last deletion test (258a494)
    - [ref] refactor (db76cfd)
    - [ref] deletion won't have problems with broken refs (286b5c1)
    - thanks clippy (e5da69e)
    - [ref] add failing deletion test for broken refs (578413f)
    - [ref] another del test (d935d6f)
    - [ref] another deletion test (8b756e0)
    - [ref] another deletion test (69ede1b)
    - [ref] refactor (d05a646)
    - [ref] Make sure edit preprocessing happens in the right order (2d5f9aa)
    - [ref] refactor (dd9c99b)
    - [ref] refactor (97fc864)
    - thanks clippy (f436f18)
    - [ref] splitting handles reference cycles (09b4fc1)
    - [ref] splitting actually works! (a9f824b)
    - [ref] first stab at splitting refs, needs more elaboration to fulfil expectations (66b1f37)
    - [ref] refactor (eb0328f)
    - [ref] first part of ref splitting is tested (ce7f83b)
    - [ref] refactor; prep slitting tests (7ffc619)
    - [ref] refactor (683651d)
    - [ref] first sketch of generalized splitting of edits (1f2efdc)
    - [ref] working on splits really shows that we want more than one enum maybe… (1b62838)
    - [ref] need ref splitting for the first time. (f52989f)
    - [ref] better deletion tests; more useful return value (96848f6)
    - thanks clippy (ef9bfd2)
    - [ref] another deletion test succeeds (6037900)
    - [ref] refactor, not quite sure about delete mode… (683991a)
    - [ref] another test; failing for now (1908b69)
    - [ref] another test green (104598e)
    - [ref] first succeeding deletion test (3445d7d)
    - [ref] refactor (d2e2e8f)
    - [ref] first deletion tests (e41f8c8)
    - [ref] write more details on how prepare and commit should work overall. (a7d988b)
    - [ref] refactor; get closer to what git does… (488f311)
    - [ref] refactor (58a5653)
    - [ref] first very basic ref writing (7ebed3f)
    - [ref] remove complexity in the name of performance, fix windows… (77c3f24)
    - [ref] (probably) fix windows (7c1eead)
    - thanks clippy (6865549)
    - [ref] slowly getting there (6506924)
    - [ref] a way to determine if a reflog exists. (e6fbba8)
    - [ref] reference::log_iter_rev() (1f7af5d)
    - [ref] reference.log_iter() works, but… (c298473)
    - [ref] [FAIL] try to forward iterator creation to reference… (ef1737c)
    - [ref] refactor (129bccf)
    - [ref] refactor (96dd98b)
    - [ref] refactor (a7dd994)
    - [ref] refactor (3460127)
    - [ref] store ref log reverse iterator (34d7957)
    - [ref] store can provide reflog forward iter… (9adb9ca)
    - [ref] more assertions (8000677)
    - [ref] a fully implemented first test with assertions (29a5893)
    - [ref] sketch more tests that will be needed (01690be)
    - [ref] add control over handling lock failures during transaction (7c4057a)
    - [ref] generic operation on input edits, split-suitable now (7f4f637)
    - [ref] try using borrow on a slice intead of iterator… (b2371d9)
    - [ref] duplicate ref edit checks… (3ec0182)
    - [ref] a more fleshed out API for file transactions (918123f)
    - [ref] on the way towards realistic transactions… (c808cb1)
    - [ref] on the way to setup the first transaction test (29c0b51)
    - [ref] file store can ignore all writes; sketch transaction API (52a81e9)
    - [ref] refactor (6a84790)
    - [ref] log line writing (3da8fcf)
    - [ref] Line::from_bytes(…); iter uses that now (7895995)
    - [ref] test for small buffer sizes (6183772)
    - [ref] handle multiple buffer reloads (4559c7a)
    - [ref] refactor (65e333d)
    - [ref] refactor (2b416ee)
    - [ref] refactor (82b18e5)
    - [ref] multi-line reverse iteration works, without window shift for now (f1e3861)
    - [ref] first reverse iter test succeeding (8875601)
    - [ref] let's not forget to simply not try to return borrowed things from iterators (bcc934d)
    - [ref] FAIL: try it with included buffer (189080e)
    - [ref] FAIL another attempt this time without iterator… (5e73dc2)
    - [ref] FAIL at attempt to to have self-referential iterators :D… (bc4012e)
    - [ref] first test for reverse iterator and more boilerplate (40db355)
    - [ref] refactor (4daddb1)
    - [ref] sketch of reverse iterator (c581d16)
    - [ref] thanks clippy (4ba3b08)
    - [ref] significantly simplify error messages… (b15cb16)
    - [ref] don't include terminators to get slightly nicer error messges (09bbc6d)
    - [ref] another test for iter::forward() (1d84302)
    - [ref] a forward iterator with a single test (917040c)
    - [ref] log line docs (10ab8e0)
    - [ref] refactor (cd89e21)
    - [ref] more context for line parsing (ddb5f9d)
    - [ref] refactor (a08fb77)
    - [ref] be truly zero copy and delay work to when it's first asked for (b4e594b)
    - Merge branch 'negotiate-fallible' (27c8abe)
    - [actor] FAIL an attempt to remove btoi errors (3f99cf5)
    - [actor] pure nom error handling… (78cbe18)
    - Merge branch 'ref-in-want' (f248557)
    - [ref] refactor (8694488)
    - [ref] getting there! (bd73d8e)
    - [ref] a step forward to nom error handling, but… (426ae5b)
    - [ref] try really hard to use generic verbose nom errors but… (1031625)
    - [ref] tests and impl for happy cases (7be82f0)
    - [ref] the first test for log line parsing; make serde1 work (cba3cdc)
    - [refs] try to get structure in place for reflog parsing (727c66a)
    - [refs] sketch more of transactions so it has all it needs (8f9a015)
    - [refs] allow writing any valid ref value instead of limiting ourselves to object ids (114fce8)
    - [refs] finish transaction sketch (or so it seems) (976a079)
    - [refs] this gets more and more interesting (e056495)
    - [refs] finish research for transactions and their flags (2eb3bcc)
    - [refs] sketch some parts of a transaction based on git source (d9a5d32)
    - (cargo-release) version 0.3.0 (87db688)
    - (cargo-release) version 0.3.0 (6b33678)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - (cargo-release) version 0.2.0 (3286e42)
    - [git-refs] a way to build a big packed-refs file (5113529)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] traversal program uses new facilities, and it's cumbersome (29ea2de)
    - [git-repository] most of the git repository discovery (72a49c8)
    - [git-ref] refactor (0c795c5)
    - [git-ref] fix docs (4fbc476)
    - [git-ref] docs complete (93a1f4e)
    - [git-ref] nicer semantics for peel_in_place_to_id() (d3250a7)
    - Revert "[git-ref] refactor (Option<Result… -> Result<Option…" (d4046e9)
    - [git-ref] refactor (Option<Result… -> Result<Option… (774e86c)
    - [git-ref] refactor (928b637)
    - [git-ref] more docs (f962c74)
    - [git-ref] refactor (415f15a)
    - [git-ref] a bunch of docs (7cfc5ab)
    - thanks clippy (93915fa)
    - [git-ref] peel to id done (f74771c)
    - [git-ref] first working peel-to-id() (3574f87)
    - [git-ref] frame for peel_to_id (3710b6c)
    - [git-ref] peeling without an iterator, fine (b118946)
    - [git-ref] first stab at reference iteration… (806d10e)
    - [git-ref] refactor (c363269)
    - [git-ref] find_one_existing(…) for convenience (7a443ff)
    - [git-ref] some find failure cases (d855051)
    - [git-ref] handle all find_one cases as per docs (3c0acc6)
    - [git-ref] more ways of finding reference (b3c4e92)
    - [git-ref] the first green find_one test (30177e8)
    - thanks clippy (8f0e9ed)
    - [git-ref] first basic impl shows validation needs a little adjustment (8b901c7)
    - [git-ref] a sketch of find_one - easiest for the caller for sure (ec96256)
    - [git-ref] refactor (5bac585)
    - [git-ref] frame for loose store reference lookup (30b0d54)
    - (cargo-release) version 0.2.0 (1327894)
    - [git-ref] use git-validate crate (6b4f937)
    - [git-ref] Setup more tests to realize we really want validate::tag (54ee5b5)
    - [git-ref] frame for validation (9656ac6)
    - [git-ref] failure tests (567e86c)
    - [git-ref] more tests (048fb77)
    - [git-ref] refactor (77d0cc0)
    - [git-ref] don't support serde for now (2a6295b)
    - [git-ref] refactor (02e545b)
    - [git-ref] first basic 'ref: ' parsing (60fa3ba)
    - [git-ref] refactor (9a30f87)
    - [git-ref] the first succeeding test (cebfdb4)
    - [git-ref] the first failing test (7e802a0)
    - [git-ref] sketch ref creation (c5241b8)
    - [git-ref] A sketch of how it looks like with Store backref (1a08f1c)
    - [git-ref] more scaffolding (8c6e884)
    - [git-ref] clear it out and move existing functionality to git-object (fa548ce)
    - (cargo-release) version 0.5.0 (b6b5856)
    - [pack-gen] refactor (61554e2)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [odb #180] refactor (eff21da)
    - [pack #179] refactor (ab6554b)
    - [object #177] fix docs (2fd23ed)
    - [object #177] tag::RefIter -> TagRefIter (28587c6)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - [ref #175] follow (try_)find(_what) naming convention (679895c)
    - [ref #175] fix docs (dd1edc3)
    - [ref #175] refactor log line (7ac948a)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [ref #175] refactor (1243459)
    - [ref #175] make 'mutable' module private (a80dbcf)
    - Release git-actor v0.5.0 (a684b0f)
    - [actor #175] refactor (ec88c59)
    - [ref #175] refactor (292e567)
    - Release git-actor v0.4.0 (16358c9)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - Release git-lock v1.0.0 (f38f72c)
    - Release git-tempfile v1.0.0 (1238535)
    - Upgrade to nom-7 (f0aa3e1)
    - Merge branch 'main' into 162-repo-design-sketch (e63b634)
    - [repository #165] fix docs (b4fdfd7)
    - Release git-ref v0.6.0 (0bb4c13)
    - [ref #165] refactor (66624c3)
    - [repository #165] refactor (00ec15d)
</details>

