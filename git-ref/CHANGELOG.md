# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 146 commits contributed to the release over the course of 11 calendar days.
 - 39 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - remove old and unnecessary experiment (aba8e56)
    - path::is (1f4e45a)
    - rename path::is_git to path::is (ac3b9ef)
    - path::discover (1958e8a)
    - Avoid duplicate module paths in 'tree' and 'commit' (2f2d856)
    - top-level of 'path' module (066f59b)
    - object_id (329d183)
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() (a19567e)
    - repository (1a1959f)
    - ext::tree (5e091fb)
    - easy::object::peel (e376067)
    - easy::object::errors (de004b3)
    - rename `easy::Object::to_(commit|tag)_iter()`… (61793ff)
    - easy::object, sans a few child-modules (f582439)
    - update 'platform' information to reflect the current usage (42080ae)
    - rename easy::reference::log::State to easy::reference::Logs (03fe8a7)
    - rename `*::State` into `*::Platform` (0cd585e)
 * **#192**
    - smart-release: assure the current package version is actually breaking (fb750b6)
    - smart-release: better verbosity handling when comparing to crates-index (f6f2d1b)
    - smart-release(feat): turn off safety bump with its own flag (a040f7d)
    - smart-release(refactor): (443f000)
 * **#196**
    - Revert "traverse(chore): try git-cliff…" (cd293ae)
    - try git-cliff… (cbc5b81)
 * **#197**
    - smart-release: improved safety bump log message (9b78c34)
    - smart-release: commit message reveals safety bumps (b1a3904)
    - smart-release: released crates only receive minor bumps… (ecf38b8)
    - smart-release(chore): update changelog (342b443)
    - smart-release(test): way more tests to nail current log output (0d30094)
    - smart-release: dependency upgrade works (a56bd7b)
    - smart-release: calculate new version of dependent (c50704a)
    - smart-release(fix): don't claim "conservative" updates for major version change (681d743)
    - smart-release: assure we can find non-sequential connections (798b650)
    - smart-release: all logic to calculate dependent version bumps (7ca029c)
    - smart-release: an algorithm to collect dependencies by 'growing' (73794a4)
    - smart-release: foundation for bumping versions (d1145d1)
 * **#198**
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - introduce notion of essential sections in a changelog… (be891e2)
    - Preview changelog support for smart-release as well (b9e6de1)
    - Detect changes after merge; add flag for controlling changelog preview (6beb734)
    - A lot of logic to handle messaging around changelog generation and halting… (28f6e18)
    - Unconditional changelog creation in smart-release (48b5228)
    - rename --skip-* flags to --no-* for consistency (3c0a638)
    - fix windows tests by transforming line endings (e276d77)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Make use of fixed git-conventional (b7b92b6)
    - update git-conventional dependency (2d369e8)
    - first test and sketch for stripping of additional title values (55b7fe8)
    - Basic message parsing, either conventional or not, without additions (b3b6a2d)
    - Sketch Message fields from which change logs can be built (b167d39)
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
    - Research commit message trailers just to learn that I want to skip them (c332b8f)
    - Fix build (d0a956f)
    - refactor!: Use git_object::commit::MessageRef::summary()… (13e7c3a)
    - feat(commit): A summary for commit messages suitable for logs (cd3fc99)
    - More message parsing tests with windows line separators (001e8c2)
    - A manual message parse impl and more tests (f4b8a0d)
    - More message parsing tests, now with legit failure… (625be8d)
    - feat(commit): Add `message()` method and `MessageRef` type… (6150b2d)
    - Sketch data for parsed messages (32dd280)
    - smart-release: add git-conventional (0c355ed)
    - smart-release: consider nom for custom parsing, but… (5fc3326)
    - smart-release: refactor (17322fa)
    - smart-release: refactor (ac0696b)
    - smart-release: refactor (87ebacc)
    - smart-release: a seemingly slow version of path lookup, but… (41afad3)
    - smart-release: fast filter by single-component path (ae7def4)
    - smart-release: prepare for fast lookup of paths (fbf267e)
    - configure caches with env vars using `apply_environment()` (d422b9a)
    - refactor (e7c061b)
    - set package cache via RepositoryAccessExt (66292fd)
    - smart-release(feat): Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size… (5aadf75)
    - allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE (d79a1b7)
    - prepare for configurable pack cache (7d2b6b6)
    - object-cache to allow for a speed boost… (06996e0)
    - smart-release: actually build the segment vec, without pruning for now (422701b)
    - smart-release: build commit history for later use in changelog generation (daec716)
    - Allow object access during commit ancestor traversal… (4fe4786)
    - smart-release: sketch history acquisition (debe009)
    - various small API changes (89f1505)
    - add panicking `Target::id()` and `TargetRef::id()` (4ed4b2d)
    - add 'Head::peeled()' method (56e39fa)
    - move easy::head::peel::Error -> easy::head::peel::to_id::Error (f644d0e)
    - smart-release: some performance logging (1954b46)
    - smart-release: build ref lookup table (9062a47)
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b63)
    - feat: Add --reference/-r flag to gixp pack-receive (637d12c)
    - feat: add git_packetline::read::Error to represent ERR lines (454c840)
 * **#205**
    - '(null)' symref targets are turned into direct refs instead… (c77bd7a)
    - fetch::Ref::Symbolic::target is now an option… (da68bfb)
    - validate assumption about '(null)' as ref-name (2576168)
 * **#67**
    - describe variants (899c579)
    - parse entry mode into number instead of comparing it to byte strings (83d591d)
    - ObjectID specific hashers, using the fact that object ids are hashes (f9232ac)
    - Tree parsing now probably is twice as fast… (d1e2b89)
    - Use a custom hasher for 'seen' objects hashset… (70179e2)
    - don't include submodules in count… (faf6f81)
    - control pack and object cache size in megabytes (60c9fad)
    - Use 'cache::Object' trait where it matters (71c628d)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106e)
    - thanks clippy (b856da4)
    - don't claim to change manifest version if it's the same one (11eebdc)
    - thanks clippy (d78d382)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
    - thanks clippy (2b55427)
    - thanks clippy (4ea1126)
    - thanks clippy (a554b9d)
    - Bump git-repository v0.10.0 (5a10dde)
    - thanks clippy (d15fded)
    - thanks clippy (e56af5a)
    - Note about git-subtree… (4b48a14)
    - thanks clippy (ae7826e)
    - [repository #164] docs for easy::reference::log (7de7c7e)
    - [repository #164] docs for easy::reference::iter (d86c713)
    - [repository #164] refactor (437e63b)
    - [repository #164] docs for top-level of easy::reference (9e465e0)
    - [repository #164] fix build (1db5542)
    - [repository #164] docs for easy::oid (b66b6fe)
    - thanks clippy (b02edb5)
    - [repository #164] docs for easy::commit and easy::odb (abf37e5)
    - [repository #164] Documentation for `easy::borrow` (3e612f4)
    - [repository #164] docs for easy::head::* (516fde7)
    - [repository #164] refactor (65b0e0f)
    - [repository #164] docs for `easy::ext::ReferenceAccessExt` (ab4910f)
    - [repository #164] docs for easy::ext::RepositoryAccessExt (9041d47)
    - [repository #164] another test and fix for `commit()` (8d676d7)
    - [repository #164] easy::ext::ObjectAccessExt docs (c4984af)
    - [repository #164] (4111d22)
    - Release git-repository v0.9.1 (262c122)
</details>

## v0.7.3 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - Add 'references().all().peeled().'… (6502412)
    - smart-release: filter refs correctly, but… (2b4a615)
    - smart-release: find tag references by name… (72e1752)
    - commit traversal along the first parent… (7bce49c)
 * **Uncategorized**
    - Release git-ref v0.7.3 (b0a9815)
    - try not to force native insutrction sets (53ea9c8)
    - Release git-tempfile v1.0.2 (310ea73)
    - Update changelogs once more… (d57d279)
    - thanks clippy (68ea77d)
    - [repository] don't enforce feature flags that may fail on windows by default (afdec2e)
    - Dependency update (d2f23f8)
    - thanks clippy (7899ef1)
    - Update changelogs retro-actively… (78cfe0a)
    - Release gitoxide v0.8.4 (effb2a6)
    - Release gitoxide-core v0.10.5 (590e59b)
</details>

## v0.7.2 (2021-09-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - git-ref(docs): improve changelog format (90e6128)
    - smart-release: sketch first step of info generation (ff894e5)
    - smart-release: changelog gets crates to work on (78d31d9)
    - smart-release: handle unborn heads (0e02831)
    - smart-release: fmt (d66c5ae)
    - smart-release: refactor (d4ffb4f)
    - smart-release: refactor (9fc15f9)
    - smart-release: refactor (9e430df)
    - smart-release: initial test for changelog (a33dd5d)
    - smart-release: very basic support for changelog command… (1a683a9)
    - smart-release: add 'cargo changelog' sub-command binary (3677b78)
    - smart-release(test): add changelog to most tests (cdf4199)
 * **Uncategorized**
    - Release git-ref v0.7.2 (e940e9a)
    - Release git-protocol v0.10.4 (898ee08)
    - Release git-odb v0.21.3 (223f930)
    - Release git-tempfile v1.0.1 (295eb37)
    - [smart-release] auto-detect changes in production crates as well (24bc1bd)
    - [smart-release #195] update test output to match CI… (f864386)
    - [smart-release #195] better error for untracked files. (f5266f9)
    - [#195] Provide nix-shell target for macos… (5e75e8c)
    - [tempfile #195] adapt to Rust 1.55 (d9e71ac)
    - [#195] Fix previously incorrect usage of io::Kind::Other… (4dae07d)
    - thanks clippy (4701296)
    - [smart-release #195] fix docs (8d7e132)
    - improved changelog… (8b82f7d)
    - [smart-release #195] assure dependent packages are not packages to be published (6792ebc)
</details>

## v0.7.1 (2021-09-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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

 - 76 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-ref v0.7.0 (24300b1)
    - Merge branch 'repository-integration' (49f5453)
    - [features #189] simple UTC-offset support for git-features (b58134b)
    - [odb #190] Read all eligble packed refs, no "pack-" prefix needed (ab250f7)
    - [features #???] WIP local time (1388ebf)
    - [repository #190] test for oid.ancestors().all() (fdc3678)
    - [#189] remove special handling of time from deny.toml (72050ef)
    - [repository #190] fix build, lets just make traversal available by default (6da3599)
    - [#189] Upgrade to prodash 16… (8e98418)
    - Bump git-pack v0.10.0 (e5e3c80)
    - [repository #185] rustfmt (dfbb015)
    - [repository #190] access to repository directories (f4d1ec4)
    - [config #185] refactor (509c938)
    - [repository #190] first shot at ancestor iteration… (85f1a48)
    - [config #185] Count lines correctly on windows… (57203ce)
    - [repository #190] refactor (e7188e0)
    - [config #185] add test for handling windows formatted files… (2a2a89f)
    - [ref #190] refactor (010be48)
    - [repository #185] remove quick-error infavor of thiserror (212c44c)
    - [ref #190] fix tests (e426e15)
    - [repository #185] on the way to removing quick-error (6ecd431)
    - [repository #190] fix tests; needs inbound transaction handling… (e5a5c09)
    - [config #185] flyby refactor (9b9ffa3)
    - [ref #190] don't provide namespace support for loose and packed refs… (c663da1)
    - [repository #185] support for initializing bare repositories (9e8a39e)
    - [ref #190] find() with namespace support (1240c21)
    - [repository #185] use git-config to handle bare repos more properly (8a5aac5)
    - [ref #190] prepare test for namespaced find(…) (5fcd0e4)
    - [repository #185] sketch of how to open a repository… (48207b5)
    - [repository #190] leverage git-ref namespace support (1aa9c11)
    - [repository #185] refactor (63089ff)
    - [ref #190] iteration with namespace support (d5987d4)
    - [repository #185] refactor (7604935)
    - [ref #190] refactor (3c7968c)
    - [repository #185] refactor repository initialization… (5ff7eaa)
    - [#190] disable caching to see if this fixes windows (0660a6f)
    - Notes about 'git-notes' and 'git-trailers' (459dd37)
    - [repository #190] prepare for namespacing support on file store level (d2d1db6)
    - Release gitoxide-core v0.10.3 (e132680)
    - [repository #190] refactor (609c249)
    - Release git-protocol v0.10.2 (54a4400)
    - [ref #190] refactor (1ef6cb3)
    - Release git-transport v0.11.1 (0952976)
    - [repository #190] fix build (f5e118c)
    - Release git-config v0.1.5 (150ed76)
    - [repository #190] note a known limitation about finding references in namespaces… (d335731)
    - Release git-commitgraph v0.4.3 (7dfe16b)
    - [ref #190] more assetions to understand 'find(…)' for namespaced refs… (f58a0ff)
    - [various #184] configure docs.rs build features (cc50249)
    - [repository #190] transparent namespace support (d14f073)
    - Release git-repository v0.8.1 (b269a12)
    - [#190] run tests faster (at the cost of compile time) (a22c95b)
    - [repository #164] make EasyArcExclusive available (2fa3dcb)
    - [#190] faster builds with debug=false and dependency caching (0b0fea4)
    - Release cargo-smart-release v0.3.0 (82f0cec)
    - [ref #190] Make References sortable (16b2232)
    - Release git-repository v0.8.0 (15ae2b8)
    - [repository #190] turns out we need bstr with unicode support (3d8796e)
    - [repository #174] keep assets (e0fca77)
    - [repository #190] public bstr re-export (3b7ffde)
    - [repository #174] remove arc_lock code entirely (dcbe742)
    - [repository #190] cleanup usage of bstr… (e4411ff)
    - Release git-repository v0.8.0 (1c9c5f1)
    - [ref #190] more conversion trait impls (1795a33)
    - Release git-protocol v0.10.1 (cec8ee3)
    - [repository #190] prefixed reference iteration (a6e19c9)
    - [repository #174] conditionally compile future parking_lot version… (5375fc8)
    - [repository #190] implementation of reference iteration (all() for now)… (2c0939a)
    - [protocol #174] fix tests… (cdc16fc)
    - [repository #190] refactor (8c532a4)
    - [smart-release #174] add asciinema recording of failed release (6668527)
    - [repository #190] prepare reference iteration (427f146)
    - Release git-repository v0.8.0 (e191eab)
    - Bump git-hash v0.6.0 (6efd90d)
    - Release git-repository v0.8.0 (403ef0a)
    - [repository #190] obtain the kind fo hash used in a repo (a985491)
</details>

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.5.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

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
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 (6f61fca)
    - [ref] break dev-dependency cycle (d5af428)
</details>

## v0.4.1 (2020-12-19)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 88 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (25d2c2e)
    - Document `git-ref` (91dce23)
    - remove dash in all repository links (98c1360)
    - increase git-odb crate size limit (75bcc85)
    - [commitgraph] Ditch pre-generated test repos. (1ce8468)
    - prepare for unquoting c-strings (47e2fa0)
    - [commitgraph] Include in `make check` target. (724f391)
    - Read multiple alternates from single file; ignore comments (1f8d367)
    - [commitgraph] Remove `Kind` enum. (3c92761)
    - support for relateive alternates (b20e9ee)
    - [commitgraph] Take `info` dir as arg, not `objects` dir. (36953e0)
    - Ignore all cycles and be happy if we have found at least one actual odb (1effdfd)
    - [commitgraph] implement basic, low-level read API (d1f0e9c)
    - prepare for multi-line parsing and all the bells and whistles (08f9ec4)
    - Revert "FAIL: try to get rid of tree-traversal Boxed error…" (1b42b31)
    - Make compound DB initialization less lazy… (6dc57b3)
    - try to get rid of tree-traversal Boxed error… (13159eb)
    - Use parallel walkdir (via jwalk) when parallel feature is enabled (f444c85)
    - Parameterize traversal error with Processor error (1513a13)
    - alternate now handles cycles (71167e4)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d9)
    - first simple alternate tests (7372118)
    - Prepare next iteration (4f656b2)
    - test for circular alternates (fc92709)
    - Provide terminal dimensions to better use horizontal space (11f6b84)
    - dependency update (6b0796a)
    - asciinema link for pack-receive (79ac34c)
    - thanks clippy (4ddc64f)
    - asciinema link for remote-ref-list (aafd5f8)
    - Actually resolve alternates when creating a compound DB (9be7aed)
    - (cargo-release) version 0.4.0 (f667785)
    - refactor (c1eff58)
    - (cargo-release) version 0.4.0 (92e8b27)
    - first sketch of alternate resolution (6cc8a94)
    - (cargo-release) version 0.4.0 (2b1bca8)
    - take not of a few more obscure features (8f9570c)
    - refactor (7c3c80a)
    - (cargo-release) version 0.4.3 (5b47a1a)
    - refactor (8930610)
    - Enforce using the correct version of clap (fd6457f)
    - refactor (e4bcfe6)
    - remove quickerror dependency from git-odb (7e27495)
    - refactor (6a84f13)
    - refactor (7874c35)
    - refactor (4e89c3b)
    - refactor (3ec99dc)
</details>

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 83 commits contributed to the release over the course of 29 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (f9dd225)
    - Document why we won't use nightly for fixing NLL issue (ca29368)
    - refactor (519dd12)
    - (cargo-release) version 0.5.0 (82b7313)
    - Revert "Fix NLL issue by using nightly" (6864a55)
    - refacator (7ac2153)
    - thanks clippy (e5d80b1)
    - Fix NLL issue by using nightly (8c5bd09)
    - refactor (d4f288c)
    - [clone] make cloning the linux kernel work (e780526)
    - Update tasks, prepare for NLL fix (52af8d1)
    - refactor (3a8fb61)
    - dependency update (446d4a5)
    - Thanks clippy (6c4d1ec)
    - dependency update (4a762f6)
    - refactor (dc022ce)
    - This works, but locates twice… (4e709f6)
    - Fixes for clap beta-2 (3fcdc5d)
    - [clone] refs can now be written into a specified directory (fb1f048)
    - Also the imperative version doesn't borrowcheck… (c5720f1)
    - refactor (98b3f4a)
    - [clone] First version of writing references, but… (445be27)
    - Looks like the functional approach to locate(…) doesn't borrowcheck (5df6867)
    - dependency update (e897b50)
    - [clone] add remaining journey tests (354e63f)
    - refactor (9e68c6b)
    - refactor (127b8b2)
    - [clone] v2 tests for pack receive (25cdd63)
    - refactor (f219d5a)
    - refactor (669b726)
    - [clone] better JSON output for pack-receive (bc6b8e8)
    - sketch compound::Db::locate; sort packs by size (6609a53)
    - refactor (7bc321e)
    - [clone] initial implementation of Json format for pack-receive (9090ac6)
    - refactor (4a09754)
    - lower velocity (69f7930)
    - [clone] nicer pack-receive output for humans (09c6c57)
    - Implement Write in terms of writing to the loose object DB (02b88c2)
    - refactor (0752b45)
    - [clone] first journey test for pack-receive (46a3511)
    - First sketch of compound Db (9bf2279)
    - (cargo-release) version 0.4.1 (64fff36)
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! (d9ced27)
    - refactor (203ba99)
    - (cargo-release) version 0.4.1 (105c501)
    - [clone] more correct handling of 'no-done'/done when sending wants/haves… (50f4516)
    - (cargo-release) version 0.2.1 (ebf3419)
    - (cargo-release) version 0.4.1 (60ac8b0)
    - [clone] Don't hide nested pack-decoding information (4d4be97)
    - (cargo-release) version 0.6.0 (9ef184e)
    - refactor (ad17bfd)
    - [clone] Don't try to explicitly close the connection… (17200b3)
    - (cargo-release) version 0.1.1 (bb38c6b)
    - refactor (91d9f78)
    - [clone] Fix encoding of V1 capabilities in first want (b68a5c5)
    - (cargo-release) version 0.2.1 (abc218c)
    - refactor (6ebb5d1)
    - [clone] When unpacking peeled refs, use the object that refers to the tag… (fe8bb39)
    - Support V2 shallow-info section (6679c91)
    - [clone] All it took was a an intermediary to call 'read' as expected (7c8ecb7)
    - Tests for V2 shallow section parsing (5bf58ab)
    - [clone] minor refactor; it's definitely the read() that doesn't work… (406829b)
    - Support for the 'deepen-relative' argument (b86fed6)
    - [clone] none the wiser - it really looks like everything is alright… (3b8d613)
    - Assure peek behaves exactly as we want it to with ERR lines (bbdaee5)
    - [clone] it looks like in order to figure out the issue, it needs tests higher up… (edf1540)
    - V1 parsing of shallow and unshallow lines… (8bcf535)
    - [clone] Don't send V2 capabilities that don't have a value… (9c9a4ee)
    - remove unused fixtures (6ae69f5)
    - [clone] Handle remote progress name prefixing (more) correctly (51d4d15)
    - Fix wants/haves separator handling for stateful V1 (1629575)
    - [clone] This actually works: first MVP of retrieving packs via clone (c06d819)
    - Make really clear that V2 is stateless no matter what the transport supports :D (c296845)
    - [clone] First step towards implementing a working pack receiving… (264ec82)
    - Assure the first 'want' in V1 is always first (e729ec8)
    - Use git attributes to prevent crlf conversion of fixtures on windows (80ca8b2)
    - Properly handle statelessness in V2 protocol (1b49f1e)
    - [clone] increase git transport size limit (422993d)
    - add some samples for deepen clones (61bc41a)
    - [clone] Support for reading multi-step negoritaions, but… (507d342)
    - upgrade futures-lite (1d83033)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (63c1292)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1)
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
</details>

## v0.2.0 (2020-07-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

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

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - incorporate git-ref crate into releases (e66c9ed)
    - refactor (6ad9304)
    - refactor (1fd90f7)
    - test for common ascii control characters (ae0c885)
    - all test for valid ref name except for ascii control chars (a157acf)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e)
</details>

## v0.5.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 406 commits contributed to the release over the course of 78 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix release order to match actual dependencies (65ff8c1)
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
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - Remove unnecessary unsafe code (83e207a)
    - [ref] fix compile warning on windows (c328774)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Implement Parser::into_iter without extra allocation (aa79924)
    - dependency update (059fa33)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
    - Remove unnecessary pub(crate) exports (3d2456e)
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
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97)
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6)
    - [actor] refactor (bccb738)
    - [protocol] fallible negotiation (e269a2c)
    - [actor] don't leak btoi errors… (e6c7fc1)
    - Revert "[ref] Try using BorrowMut to avoid blanket trait impls, but…" (8212536)
    - [actor] FAIL an attempt to remove btoi errors (3f99cf5)
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… (4bb9bba)
    - [actor] pure nom error handling… (78cbe18)
    - [ref] refactor (8694488)
    - [protocol] refactor (11b2fd1)
    - [ref] getting there! (bd73d8e)
    - [protocol] refactor (967946a)
    - [ref] a step forward to nom error handling, but… (426ae5b)
    - [protocol] refactor (8dc425f)
    - [ref] try really hard to use generic verbose nom errors but… (1031625)
    - [protocol] assure we don't coerce refs into UTF-8 representation (5ceb64d)
    - [ref] tests and impl for happy cases (7be82f0)
    - [protocol] support ref-in-want (b6df400)
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

## v0.6.0 (2021-08-27)

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 252 commits contributed to the release over the course of 8 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6b)
 * **Uncategorized**
    - Release git-ref v0.6.0 (b191a88)
    - [ref #190] refactor (e34be7e)
    - Release git-protocol v0.10.0 (b60ddae)
    - [ref #190] more Target conversions… (1fe1b42)
    - Release git-transport v0.11.0 (cac343c)
    - [repository #190] refactor (7a111b1)
    - Release git-packetline v0.10.0 (0899338)
    - [repository #190] shortcut to create references (28afd8e)
    - Release git-odb v0.21.0 (d4a6341)
    - [ref #190] add forward log iter and localize iter types… (c3e240d)
    - Release git-pack v0.9.0 (355d6c4)
    - [repository #190] refactor (e751688)
    - Release git-traverse v0.8.0 (40c8506)
    - [ref #190] refactor (49fe1dc)
    - Release git-features v0.16.3 (342475f)
    - thanks clippy (023dedc)
    - Release git-diff v0.9.0 (021318f)
    - [ref #190] reverse reflog ergonomics (2de86f9)
    - Release git-object v0.13.0 (bfaaf52)
    - [repository #190] ref log for HEAD specifically (946bbf1)
    - Release git-actor v0.5.0 (f74e89b)
    - [ref #190] check for zero sized buffers in reverse log iterators… (998c7c6)
    - [smart-release #174] prepare changelog (0d9a2b8)
    - [repository #190] reflog tests (641edde)
    - Bump git-repository v0.8.0 (cdb45ff)
    - [ref #190] First working sketch of reverse log iter access (4a36ded)
    - [repository #174] adjust various changelogs (081faf5)
    - [ref #190] move remaining file store functions to extension trait (60fc215)
    - Bump git-protocol v0.10.0 (82d5a0b)
    - [ref #190] Move file-log-specific functionality into own extension trait. (0b635e9)
    - Bump git-transport v0.11.0 (1149f1b)
    - thanks clippy (376c045)
    - [transport #174] prepare for release (f8bc517)
    - [repository #190] refactor (15d4ac8)
    - [odb #180] fix docs (bd50752)
    - [repository #190] a major step forward with `head()` access (43ac4f5)
    - [odb #180] refactor (eff21da)
    - [ref #190] cache peeled objects properly (2cb511e)
    - Bump git-odb v0.21.0 (7b9854f)
    - [ref #190] fix docs (3e64ec1)
    - [odb #180] add changelog (acf1193)
    - Bump git-ref v0.7.0 (ac4413c)
    - [pack #179] refactor (76e66d1)
    - [repository #190] experiment with 'HEAD' API… (c55ce4d)
    - [pack #179] move Tree traversal cache private (34e45d7)
    - [ref #190] fix remaining tests (df21f25)
    - [pack #179] refactor (5a3677d)
    - thanks clippy (14dff63)
    - [pack #179] refactor bundle (420dca2)
    - [ref #190] Use Raw Reference everywhere for great simplification… (7aeea9c)
    - [pack #179] fix docs (7ad7a44)
    - [ref #190] raw reference peeling (9473a71)
    - [pack #179] refactor (ab6554b)
    - [repository #190] refactor (d6bef3a)
    - [pack #179] refactor (620d8a5)
    - [ref #190] introduce Raw reference type that simplifies everything… (8634341)
    - [pack #179] add changelog (2102569)
    - [ref #190] more tests (980e16a)
    - [packetline #178] fix compile warnings (c8d2e72)
    - [ref #190] deletions also use PreviousValue now (74f85b1)
    - Bump git-packetline v0.10.0 (b09f391)
    - [ref #190] refactor (0e65559)
    - [packetline #178] fix docs (878d8e8)
    - [ref #190] be explicit about what the previous reflog oid is for… (c04c8b9)
    - [packetline #178] refactor (0c7c599)
    - [ref #190] don't claim there was a previous oid unnecessarily… (68f7fc2)
    - [packetline #178] fix docs (b3fd65d)
    - [ref #190] refactor (07126d6)
    - [packetline #178] refactor (23438fd)
    - [ref #190] Allow for explicit expected previous values (1a4786f)
    - [packetline #178] rename PacketLine to PacketLineRef… (d4c16a9)
    - [ref #190] prepare massive refactoring to get additional constraint (9741987)
    - [packetline #178] add changelog in preparation for breaking changes (ffd96f9)
    - [repository #190] show that unconditional creation of references doesn't is lacking… (06b9270)
    - Bump git-traverse v0.8.0 (54f3541)
    - allow incremental builds… (e4abcf3)
    - Bump git-diff v0.9.0 (2e2e798)
    - [repository #190] another commit() test… (4ec631c)
    - [smart-release] Adjust commit message depending on whether we are skipping the publish… (c190c6b)
    - [repository #190] produce nice reflog messages (e7a8b62)
    - [object #177] cleanup CommitRefIter imports and git_object::Error (058f68a)
    - [repository #190] commit::summary() (43f7568)
    - [object #177] dissolve 'immutable' module (70e11c2)
    - [repository #190] thanks clippy (0763ac2)
    - [object #177] fix docs (2fd23ed)
    - [repository #190] first version of 'commit(…)' without reflog message handling (bfcf8f1)
    - [object #177] resolve 'mutable' module (b201b32)
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly (63bedc7)
    - [object #177] refactor (216dd0f)
    - [object #190] consistent method naming (c5de433)
    - [object #177] refactor (472e13b)
    - [features #190] be more explicit about why sha1-asm is disabled (507d710)
    - [object #177] Commit::write_to migration (60b9365)
    - [ref #190] refactor (3f36a01)
    - [object #177]  commit::RefIter -> CommitRefIter (e603306)
    - [object #190] More conversion methods for Object (78bacf9)
    - [object #177] migrate immutable::commit into crate::commit (45d3934)
    - [repository #190] put git-lock into ST1… (26a6637)
    - [object #177] refactor tag write_to (7f19559)
    - [repository #190] refactor (1e029b4)
    - [object #177] tag::RefIter -> TagRefIter (28587c6)
    - [repository #190] A way to write objects and the empty tree specifically (7c559d6)
    - [object #177] into_mutable() -> into_owned() (7e701ce)
    - [various #190] rename 'local-offset' to 'local-time-support' (3a7d379)
    - [object #177] fix docs (25d8e7b)
    - [repository #190] Make local-offset available on demand only… (1927be7)
    - [object #177] move mutable objects to crate::* (c551c02)
    - [actor #190] methods to get an actor signature at the current time (6d0bedd)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - Merge branch 'git-ref-refactor' (5dbf753)
    - [pack #172] A note about empty packs in Bundle writer (09a777f)
    - [ref #175] follow (try_)find(_what) naming convention (679895c)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [ref #175] fix docs (dd1edc3)
    - Fix formatting of performance-tasks.md (917967e)
    - Merge branch 'Byron:main' into main (dc58eca)
    - [ref #175] refactor log line (7ac948a)
    - Release git-actor v0.4.0 (16358c9)
    - Allow creation of empty indices (d122fc7)
    - Release git-testtools v0.5.0 (574ede9)
    - [ref #175] refactor (1243459)
    - [actor #173] fix docs (2d7956a)
    - A note about the project board to help with transparency (d850004)
    - Release git-testtools v0.5.0 (86e0a92)
    - [ref #175] make 'mutable' module private (a80dbcf)
    - [actor #173] refactor (08a1849)
    - Upgrade to nom-7 (f0aa3e1)
    - Release git-actor v0.5.0 (a684b0f)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - some helpful remarks; be more specific about fixing breakage (7783965)
    - [actor #175] refactor (ec88c59)
    - [stability #171] Another question to ask before stabilizing a crate… (c2bc1a6)
    - Update COLLABORATING.md (e1a04cf)
    - [ref #175] refactor (292e567)
    - Release git-lock v1.0.0 (f38f72c)
    - First draft of collaboration guide (ec3f0ec)
    - Release git-tempfile v1.0.0 (1238535)
    - Adjust contribution recommendation (3aae2e2)
    - [smart-release #171] it's about time we get some tests (48a489b)
    - [pack #170] there can only be one (dce4f97)
    - [stability #171] prepare git-lock and git-tempfile release (3a1cf4d)
    - [pack #170] clru allows for free lists, reducing allocation pressure... (4d820d2)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe)
    - [pack #170] basic progress for resolution (ada0b96)
    - [stability #171] mark git-hash and git-actor as ST1 as well (32caae1)
    - [pack #170] Basic entry resolution without progress (7461f31)
    - [stability #171] does this fix the issue with cargo doc? (0475532)
    - [pack #170] first step towards resolving in multi-threaded mode… (f3c21f9)
    - [stability #171] git-ref is now ST1 and available through git-repository (50154cd)
    - [pack #170] Don't double-lookup trees during traversal… (7b06829)
    - [stability #171] fix schematic (999e813)
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" (811bb54)
    - [stability #171] Simply commit on git-ref/git-config stability tier 1… (f6560ff)
    - [pack #67] Don't pre-fetch packed objects during counting (d08b673)
    - [stability #171] Add the concept of Foundation Crates… (8819bde)
    - Release git-pack v0.9.0 (7fbc961)
    - [smart-release #171] Try to avoid unstable git-repository features… (c8f325b)
    - [pack #67] refactor (14717f6)
    - [stability #171] Don't suggest pinning of pre-release crates… (9301bbf)
    - [pack #67] Optimize caches based on cache debugging (1271c01)
    - Merge branch 'main' into stability (11bae43)
    - [pack #67] Add cache debugging capabilities to git-features (8776c98)
    - cleanup imports (e669303)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671)
    - update dependencies (e9a98bc)
    - [pack #164] fix docs (08ee674)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58)
    - Merge branch 'main' into 162-repo-design-sketch (e63b634)
    - [stability #171] update README with stability information… (f330daa)
    - [repository #164] top-level easy docs (6b71c51)
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" (4c2ea21)
    - [stability #171] How to handle the MSRV (9be1fce)
    - [repository #165] see if `git-config` can already be placed… (d287a4a)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (8d49976)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f)
    - [repository #165] fix docs (b4fdfd7)
    - [pack #167] a single-threaded special case for counting… (65e29de)
    - [stability #171] about transitioning from pre-release to release (bdbdb65)
    - [repository #165] add limitations along with possible workarouds (7578f1e)
    - [pack #167] generalize over immutable insertions… (169f000)
    - [stability #171] finish tier description… (4fe1259)
    - [repository #165] assure packed-refs are always uptodate (a5605df)
    - [pack #167] refactor (6bf0f7e)
    - [stability #171] Rough descriptions of ST 3 and 2 (340935c)
    - [repository #165] Allow cloning packed-refs and try to see how it differs… (7ec32b7)
    - [pack #167] progress is handled by reducer… (a22f8e1)
    - [stability #164] First sketch of stability MD… (a7353cd)
    - Release git-ref v0.6.0 (0bb4c13)
    - [pack #167] Error handling for object input (0aac40c)
    - [ref #165] refactor (66624c3)
    - thanks clippy (d689599)
    - Revert "[repository #165] PROOF: GATs will work as expected!" (853f072)
    - [pack #167] remove iterator based count objects impl… (7ec2f2b)
    - [repository #165] PROOF: GATs will work as expected! (7f56dbd)
    - [features] refactor (0958fc8)
    - [repository #165] refactor (1547d0b)
    - [pack] A non-iterator version of parallel object counting… (04fe855)
    - [repository #165] refactor; fine grained allow(missing_docs)… (aa0511f)
    - [features] refactor (d4605cd)
    - [repository #165] prepare for writing light docs for Easy (f8834c9)
    - thanks clippy (41d7a44)
    - [repository #165] refactor (3a0160e)
    - [repository #162] cleanup imports (983d11a)
    - [repository #165] fmt (a02d5aa)
    - [smart-release #162] use TreeRef capabilities to lookup path (51d1943)
    - [repository #165] Don't panic on repo borrow error… (b2f644a)
    - [repository #162] what could be a correct implementation of a tree path lookup (1f638ee)
    - thanks clippy (b496d99)
    - [repository #162] detachable ObjectRefs and a few conversions (ec123bb)
    - [repository #165] Write about the GAT plan to make this better one day (d793ecd)
    - [repository #162] finally let smart-release use the correct abstraction for peeling (ba243a3)
    - [repository #165] quick test to see if Access2 can become Access… (45acc7a)
    - [repository #162] Add id field to ObjectRef… (f5ba98e)
    - [repository #165] Generalizing over mutable Repos is possible too… (0f7efe3)
    - [repository #162] Make clear that Objects are actually references… (d1e6843)
    - [repository #165] show that Access2 works for all Easy* types… (b8ceefe)
    - [repository #162] another attempt to find a decent peeling abstraction… (716d623)
    - [repository #165] First success with creating a shared borrow to the repo (f2a38b2)
    - [repository #162] attach the Object to 'Access' (9a12564)
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" (a1443e4)
    - [repository #162] refactor (a32d361)
    - [repository #165] FAIL Look into `owned_ref` crate (09aa714)
    - [repository #162] trying new names (b3f453b)
    - [repository #165] FAIL AsRef works for basic refs but… (02979b6)
    - [repository #162] put impl for finding object data into the extension trait (91b9446)
    - [repository #165] FAIL try to generalize with Borrow… (295ba95)
    - [repository #162] experiment with finding objects… (312a692)
    - [repository #165] FAIL See if EasyExclusive can work… (016debb)
    - thanks clippy (f2fb026)
    - [repository #165] introduce EasyShared (a119ad9)
    - [repository #162] Cannot ever store a RefCell Ref in an object… (5c17199)
    - [repository #165] First thoughts about stale caches (7f8b63e)
    - [repository #162] experiemnt with optionally keeping data in Object (b8a8e08)
    - [repository #165] hide all easy::State fields behind result-enforcing methods (000c537)
    - [smart-release #162] Fix short flags (08f3418)
    - [repository #165] pack cache access only with errors (2353e50)
    - [smart-release #162] Object can be used like a git_hash::ObjectId (c7bc730)
    - [repository #165] assure packed-refs is only used non-panicking (a355d94)
    - [smart-release #162] format everything (8ff83e5)
    - [repository #165] refactor (16fce63)
    - Update crate status of git-index to reflect recent advancements (a258d2d)
    - [repository #165] a sample of a simpler way to create a tag (fb8f584)
    - [smart-release #162] don't throw away work… (b43b780)
    - [smart-release #165] Use generic edit-reference functionality (be3e57f)
    - [smart-release #162] a demo of attaching and detaching objects… (ff2927c)
    - [repository #165] sketch generic ref file editing (3a026ae)
    - [smart-release #162] an actual Data type… (7fd996f)
    - [repository #165] refactor (00ec15d)
</details>

