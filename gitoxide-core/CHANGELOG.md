# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 205 commits contributed to the release over the course of 12 calendar days.
 - 46 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 16 times to make code idiomatic. 

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
    - fix windows tests by transforming line endings (e276d77)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Implement --write actually (69d36ff)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
    - a test case showing that headlines are currently ignored, and links too (2a57b75)
    - don't try to run tests in binaries that have none… (d453fe5)
    - It's already getting there, even though a few parts are completely missing (ee4aa08)
    - only parse into 'unknown' catch all in special cases… (c0296c4)
    - first basic parsing of unknown parts as segments in sections (f265982)
    - quick and dirty switch to getting access to a range of parsed input… (f5902f2)
    - setup test for old method of parsing unknown text… (996c39d)
    - refactor tests: unit to integration level (4326322)
    - Don't add a date to unreleased versions (ba4d024)
    - Remove strong-weak typing for conventional type (b71c579)
    - Actually integrated generated changelog with existing ones… (aa095e2)
    - Fix panic related to incorrect handling of character boundaries (9e92cff)
    - inform about 'bat's  absence (c82c5bc)
    - Parse message fully (and own it) to allow markdown generation (b8107e5)
    - rename --no-bat to --no-preview… (1087dd8)
    - tests for conventional and unconventional description parsing (faade3f)
    - basic merging now works (6c6c200)
    - sketch for finding insertion points and merging sections (2a49033)
    - Sketch merging logic… (1932e2c)
    - Make use of fixed git-conventional (b7b92b6)
    - prepare test for basic merging… (0a14ced)
    - update git-conventional dependency (2d369e8)
    - nicer 'thanks clippy' message (4344216)
    - first test and sketch for stripping of additional title values (55b7fe8)
    - Basic message parsing, either conventional or not, without additions (b3b6a2d)
    - Show with simple example how the round-tripping works, neat (9510d9b)
    - Sketch Message fields from which change logs can be built (b167d39)
    - collect unknown text so things don't get lost entirely… (60040c9)
    - feat: `BodyRef::without_trailer()` for more obvious access than `*body` or `body.as_ref()` (f0ea526)
    - parse back what we write out, perfectly… (5cab315)
    - refactor (ef3fc6d)
    - fix journey test (3006e59)
    - feat: `CommitRef::message_trailers()` as shortcut… (5324391)
    - more tests for trailers iterator (c3b0161)
    - Write new changelogs with bat if available (cca8e52)
    - feat: `BodyRef::trailers()` allows iterating trailer tokens and values (175e1cb)
    - Use `cargo diet` to reduce package size (cc5709e)
    - Some tests and sketch for BodyRef parsing (3953c24)
    - Write markdown changelog to lock file (400046e)
    - refactor (b05ce15)
    - feat: CommitRef::summary() and `MessageRef::body()` methods (1714d05)
    - refactor (7055dc8)
    - Basic serialization of ChangeLog (205b569)
    - Another test for footer separation, simple version (b439186)
    - support for generated headers (bcc4323)
    - Return to safety (35313b9)
    - refactor (1ebb736)
    - omg nom parsing works… (cd11704)
    - Use 'to_*' when converting `easy::Object` to specific object kind (1cb41f8)
    - FAIL: not really successful to continue down the 'fold' road (d9afc22)
    - transform history segments into changelog parts (348b05c)
    - three tests failing with nom (13646e8)
    - layout structure for ChangeLog generation from history items (40e9075)
    - Revert " FAIL: try to use nom-way of the previous body parsing…" (d1e6f62)
    - more general commit history (39522ec)
    - FAIL: try to use nom-way of the previous body parsing… (909f668)
    - Invert meaning of changelog's --dependencies flag… (51eb8cb)
    - sketch nom version of the message parser… (1ec47de)
    - rename --skip-dependencies to --no-dependencies… (77ed17c)
    - Research commit message trailers just to learn that I want to skip them (c332b8f)
    - Adjust changelog… (fb0dbfc)
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
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - Add 'references().all().peeled().'… (6502412)
    - smart-release: filter refs correctly, but… (2b4a615)
    - smart-release: find tag references by name… (72e1752)
 * **#200**
    - clear error message if upload-pack reports an error (4701c84)
    - parse issue numbers from description and clean it up (95c0a51)
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b63)
    - feat: Add --reference/-r flag to gixp pack-receive (637d12c)
    - feat: add git_packetline::read::Error to represent ERR lines (454c840)
 * **#205**
    - '(null)' symref targets are turned into direct refs instead… (c77bd7a)
    - fetch::Ref::Symbolic::target is now an option… (da68bfb)
    - validate assumption about '(null)' as ref-name (2576168)
 * **#67**
    - Use 'cache::Object' trait where it matters (71c628d)
    - split data::output::count::objects into files (8fe4612)
    - use new git_pack::cache::Object trait (b209da2)
    - cache::Object trait for caching and retrieving whole objects (50cf610)
    - object cache size is configurable (5a8c2da)
    - remove object cache impl which now lives in git-pack (741558d)
    - dynamically sized full-object speeds up diff-based object counting… (d6c44e6)
    - Count ref-deltas in thin packs as well (80c6994)
    - Assure pack-ids are actually unique, the simple way… (0509b4f)
    - Use Easy in the one spot where it is possible… (6a97bfa)
    - try to create persistent Easy iterator, but can't make it Send… (54a64a5)
    - Add '--thin' flag to pack-create and pass it on (2664d73)
 * **Uncategorized**
    - thanks clippy (31498bb)
    - let's not force folks to not use debug info… (bc458c8)
    - Merge branch 'main' into changelog-generation (c956f33)
    - thanks clippy (c55f909)
    - don't claim to change manifest version if it's the same one (11eebdc)
    - thanks clippy (b200ee8)
    - thanks clippy (4b3407d)
    - thanks clippy (d78d382)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
    - thanks clippy (1dece2b)
    - thanks clippy (2b55427)
    - thanks clippy (4ea1126)
    - thanks clippy (a89d08c)
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
</details>

## v0.10.5 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 1 calendar day.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - commit traversal along the first parent… (7bce49c)
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
    - Release gitoxide-core v0.10.5 (590e59b)
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
    - Bump git-repository v0.9.0 (b797fc1)
    - [repository #193] Add feature flags for async/blocking (57f482c)
</details>

## v0.10.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.4 (5ae584c)
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
</details>

## v0.10.3 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 243 commits contributed to the release over the course of 11 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Merge branch 'git-ref-refactor' (5dbf753)
    - [pack #172] A note about empty packs in Bundle writer (09a777f)
    - [ref #175] follow (try_)find(_what) naming convention (679895c)
    - Fix formatting of performance-tasks.md (917967e)
    - Merge branch 'Byron:main' into main (dc58eca)
    - Release git-actor v0.4.0 (16358c9)
    - Release git-testtools v0.5.0 (574ede9)
    - [actor #173] fix docs (2d7956a)
    - Release git-testtools v0.5.0 (86e0a92)
    - [actor #173] refactor (08a1849)
    - Upgrade to nom-7 (f0aa3e1)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - some helpful remarks; be more specific about fixing breakage (7783965)
    - [stability #171] Another question to ask before stabilizing a crate… (c2bc1a6)
    - Update COLLABORATING.md (e1a04cf)
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
    - [stability #171] document git-repository cargo features (8f21e3d)
    - cleanup imports (e669303)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671)
    - [stability #171] Further loosen MSRV constraints (6b1095a)
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
    - [repository #165] An experiment on transforming panics into errors… (1f52226)
    - [smart-release #162] a sketch for accessing objects data… (ba27101)
    - [repository #165] offer panicking type conversions for objects (f802f8c)
    - [smart-release #162] refactor (7f2421b)
    - [repository #165] try a more common naming convention for fallbile things… (fc70393)
    - [smart-release #162] peeling objects to a certain target kind… (5785136)
    - [repository #165] refactor (6207735)
    - [smart-release #162] a single import path for ReferenceExt (7060797)
    - [repository #162] update crate status to reflect now 'easy' mode (6d00139)
    - [smart-release #162] rename git-repository::object -> objs (ac70d81)
</details>

## v0.10.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.2 (b96a518)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e3340)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.10.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.1 (8b21d82)
    - [protocol] Make fetch-connection usage explicit (29696f9)
</details>

## v0.10.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 165 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#83**
    - [organize] Auto-strip .git suffix for non-bare repos (ea0ecc2)
 * **Uncategorized**
    - (cargo-release) version 0.10.0 (310dd22)
    - (cargo-release) version 0.7.0 (1c5dfb8)
    - [core] refactor (e3d708f)
    - [core] refactor (869d162)
    - [gitoxide-core] avoid lossy path conversions (63c2951)
    - Use AsRef<Path> when opening from path (515d256)
    - [protocol #145] Unify the `previous` and `previous_result` parameters… (96f77c7)
    - thanks clippy (e1964e4)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e)
    - Bump serde_json from 1.0.64 to 1.0.65 (9117feb)
    - [ref #140] do actual tag peeling in programs that matter (e404852)
    - [ref #140] sketch ref tag peeling (ef90652)
    - [pack] fix build (e680854)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820)
    - [pack] Make use of thin-pack resolver when writing bundles… (9f43bf0)
    - [pack] it seems git is just skipping bad objects during pack-gen (0f29b82)
    - [pack] In single-threaded mode, use a huge cache for some speedup (aec8a9b)
    - [pack] pack-create with immediate counting and traversing… (b74a98f)
    - [pack] refactor; entry-iterator now produces delta-objects (5dc370b)
    - [pack] support poor reference resolution if input is not an object hash… (1b985a1)
    - [pack] better identify the currently implemented pack generation mode. (f9e3b3c)
    - [pack] refactor (78d46c1)
    - [ref] fix build (0b732e1)
    - [ref] figure out how peeling works with packed-refs… (2801f7a)
    - [ref] fix build (83002df)
    - [ref] rename find_one to 'find' in git-ref… (ae7746a)
    - Bump anyhow from 1.0.41 to 1.0.42 (352e468)
    - Bump async-io from 1.4.1 to 1.6.0 (99e4732)
    - [protocol] fix build (38aca40)
    - Merge branch 'negotiate-fallible' (27c8abe)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97)
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6)
    - [actor] refactor (bccb738)
    - [protocol] fallible negotiation (e269a2c)
    - [ref] rename Action::Close to Action::Cancel… (cac1f6c)
    - Merge branch 'ref-in-want' (f248557)
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
    - [actor] fix gix hours (b4e95fd)
    - [actor] git-object uses git-actor (d01dd2f)
    - clippy cleanup; fix CI build (3e943f2)
    - thanks clippy (3f7e27b)
    - Fix everything up so that… (5930563)
    - A first attempt to make intrerupt tools work, but… (8fb8d37)
    - fix build (ea2bfac)
    - refactor (7f9be36)
    - And one less usage of the global interrupt handler… (5da57a3)
    - Make most interrupts local to the method or function (4588993)
    - [hours] use new interrupt::Iter; refactor (2355f0b)
    - [pack-create] also show throughput (74d8d57)
    - [tempfile] interruptible traversal (4eeaa1b)
    - [pack-create] better handling of input paths (1825e1a)
    - [pack-create] progress for ancestor traversal (9349286)
    - refactor (e0b7f69)
    - [pack] refactor (25f04ba)
    - [pack] validate tips as well… (ec8864f)
    - [pack] refactor (18cabb8)
    - [pack] Force single-threading (with toggle) for counting phase… (8d3ba0b)
    - [pack] also put counts in order for stable packs (f299160)
    - [pack] gixp pack-create uses in-order adapter as well (365c582)
    - [pack] refactor (cfdf802)
    - [pack] print the pack file name even if there is no output directory (832fa29)
    - [pack] refactor (9d9def3)
    - [pack] pack-create --output-directory is now optional (2150be8)
    - [pack] print statistics for entries iteration as well (eb6554b)
    - [pack] add --statistics flag to pack-create (51a3077)
    - refactor (24697bc)
    - [async-receive] refactor (7e28831)
    - Bump anyhow from 1.0.40 to 1.0.41 (f6d48c8)
    - [pack] write packs to a directory with the proper name (3fbca7d)
    - [pack] refactor (f10adea)
    - [pack] fix build (81ee633)
    - [pack] refactor (0514f1d)
    - [pack] refactor (37922d1)
    - Bump itertools from 0.10.0 to 0.10.1 (b54f21d)
    - [async-client] refactor (e7d115c)
    - [async-client] cleanup Send bounds! (c7dee44)
    - [async-client] refactor (89e6f66)
    - Revert "[async-client] FAIL with the brutal copy-paste way" (7f29adc)
    - [async-client] FAIL with the brutal copy-paste way (b91ecb5)
    - Revert "[async-client] the beginning of an unholy transformation…" (c8423a8)
    - [async-client] the beginning of an unholy transformation… (1f314df)
    - [async-client] refactor (b252932)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - [async-client] prepare for unblocking the protocol delegate (796c7d5)
    - [async-client] refactor (0d5b911)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - [async-client] refactor (dc742df)
    - [async-client] Unblock printing in pack-receive (156bed6)
    - [async-client] Sketch of (partially blocking) pack-receive (e58859d)
    - [async-client] ls-remote in async (but for git protocol only) (fd8edca)
    - [async-client] basic git_connect functionality using async_io/async_net (af60297)
    - [async-client] frame for async connect (9ada080)
    - [async-client] frame from A to Z to actually implement it… (ac4715c)
    - Separate networking via feature toggles and pass that through in the main crate (2c749f1)
    - dependency update (6d2278b)
    - [git-protocol] refactor (990099b)
    - Bump crossbeam-utils from 0.8.4 to 0.8.5 (fce4d10)
    - [git-protocol] refactor (d623cf7)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a8)
    - [git-protocol] async response (c498557)
    - Bump cargo_toml from 0.9.1 to 0.9.2 (28687b1)
    - [gix-organize] fast-prefilter + close look at the repository itself (eda440a)
    - [gix-organize]: this version fails to detect any git repo (8802fa7)
    - [gix-organize] use git-repository a little more (20f76a5)
    - Revert 'gix-organize' to normal thanks to performance regression (eda452e)
    - (cargo-release) version 0.6.0 (d35c55d)
    - thanks clippy (6a80d5c)
    - [git-repository] gitoxide-core uses more of git-repository (bb5b074)
    - [git-repository] replaces git-features and git-protocol in gitoxide-core (081d20f)
    - refactor (2ba9f91)
    - [git-repository] used by gix-hours (24e0258)
    - [git-repository] refactor (b5ebcfa)
    - [git-repository] now used by gixp-organize (aa91fad)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
    - [git-odb] much better docs; cleanup exposed API (3d5b229)
    - (cargo-release) version 0.2.0 (b213628)
    - [git-odb] refactor (2958145)
    - [git-odb] refactor (1eab15d)
    - [git-pack] compilation (b392a55)
    - [git-pack] refactor (157b6ff)
    - (cargo-release) version 0.16.0 (769c649)
    - [git-pack] refactor (e5b00ee)
    - [git-pack] the world compiles again (f0c0e36)
    - [git-odb] refactor (e07478c)
    - [git-odb] refactor (721303d)
    - [git-odb] refactor (ea224e9)
    - [git-odb] refactor (6a1b16a)
    - [git-odb] refactor (47c4042)
    - Configure git-features properly for gitoxide-core… (251e690)
    - (cargo-release) version 0.15.0 (d69d9fb)
    - Prevent pack-index-from-data to block if stdin is a terminal (39dec0e)
    - [pack-gen] release a little memory, hopefully (f25293a)
    - Revert "[pack-gen] remove tree-diff as traversal option." (2907a5f)
    - [pack-gen] remove tree-diff as traversal option. (8373671)
    - [pack-gen] a lot more progress, even though it's not perfect yet (480f8b7)
    - [pack-gen] basic progress for entry generation (953190d)
    - [pack-gen] better progress (fdee381)
    - [pack-gen] the first barely working progress (5b89a0e)
    - [pack-gen] the basics to get the program going (03b67b0)
    - [pack-gen] very close to a basic impl of count + entries-gen… (c927429)
    - [pack-gen] Try to just ignore the amount of objects inside… (918b222)
    - thanks clippy (89b1ee4)
    - [pack-gen] And it shows we really need to let the traversal be done first (a870eb2)
    - [pack-gen] And now it creates an entries iterator (27c9bc1)
    - [pack-gen] A step further, but it looks like input object iteration is tricky (abf4276)
    - [pack-gen] Frame for plumbing command (a2203ca)
    - (cargo-release) version 0.10.0 (5d7ee6a)
    - refactor (9f0a8cc)
    - (cargo-release) version 0.3.0 (684de4b)
    - (cargo-release) version 0.8.0 (ccea4b6)
    - [git-transport] remove default features to force being explicit everywhere (d1b39f8)
    - [organize] Be clear about what the traversal really does (ed945ab)
    - refactor (ef80fd6)
</details>

## v0.9.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 27 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#79**
    - refactor; add test for empty tree iteration (6340296)
 * **Uncategorized**
    - [track publish] git-protocol/0.7.0 (99ddf42)
    - (cargo-release) version 0.9.0 (e6cdd84)
    - (cargo-release) version 0.7.0 (069184e)
    - (cargo-release) version 0.15.0 (d91b241)
    - (cargo-release) version 0.2.0 (3fb8377)
    - (cargo-release) version 0.9.0 (84897fd)
    - Merge branch 'patch-1' (5edc076)
    - refactor (a9e4feb)
    - Allow empty trees when parsing them at once, fixes #79 (d34fd19)
    - Fix formatting (a341995)
    - Remove almost all unsafe code from Tree. (42b6033)
    - refactor (9870923)
    - An alias for tools (8dc5ed3)
    - [hours-tool] interruptability of long-running commit interations (4fd8a63)
    - Better handling of 'tools' subcommands (ee704c0)
    - Make use of auto-configuration for line renderer (d28424f)
    - [hours-demo] remove progress for deduplication - it's too fast (a81395a)
    - Fix compile warnings (42fd77b)
    - Adjust journey tests to new porcelain (232a96c)
    - dependency update to fix prodash bug (cb427fc)
    - refactor (171b1bf)
    - Add missing docs; add local-only snapshot file (7c56366)
    - Remove 'argh' implementation in porcelain, opening it up to sub-sub commands… (4382802)
    - [hours-tool] remove original example (9eadd21)
    - developer guide: differentiate examples, experiments, plumbing and porcelain. (4c87a9c)
    - [hours-tool] Better error messages (86b4570)
    - [hours-demo] deduplication is enabled by default (cede327)
    - [hours-tool] basic journey tests (6aab8e3)
    - [hours-demo] Make deduplication go fast (4b87deb)
    - [hours-tool] control verbosity and progress using global 'gix' options (cf4d5a3)
    - [hours-demo] nicer handling of unique contributors (7faf123)
    - [hours-tool] integrate progress, remove direct writes to stderr (2778447)
    - [hours-demo] Proper logic for flags; performance stats for deduplication (d5ac96d)
    - [hours-tool] bring in all the code, mostly unchanged. (df16b3c)
    - [hours-demo] Allow turning identity unification off (f6ee0f2)
    - [hours-tool] hookup new gitoxide-core command (680f274)
    - thanks clippy (17258cc)
    - refactor (8b10434)
    - (cargo-release) version 0.14.0 (a760f8c)
    - (cargo-release) version 0.14.0 (d9514ee)
    - rename 'Locate' to 'Find' - shorter and just as good (60f72f5)
    - (cargo-release) version 0.13.0 (5c791af)
    - (cargo-release) version 0.8.0 (a1ce210)
    - (cargo-release) version 0.3.0 (e9665c7)
    - Don't mention skips anymore… (afb87d9)
    - refactor (c1013dd)
    - refactor (ca98221)
    - refactor (d490b65)
    - refactor (08fafaa)
    - git-odb::borrowed::Object => git-odb::data::Object (747a13e)
    - bump git-odb minor version (5c833ce)
    - Remove loose::Object entirely #(67) (5cf4840)
    - (cargo-release) version 0.13.0 (ac2eddb)
    - (cargo-release) version 0.11.0 (fd698e3)
    - Introduce pack_id for use in pack cache, preventing (most collisions) (ad04ad3)
    - Feature toggle for uluru based Lru cache (98eec48)
    - gitoxide-core:pack-verify: be explicit about pack-cache choice in relation to algorithm (e7971a9)
    - refactor (d624d09)
    - LruCache with const-generics (93618d1)
</details>

## v0.8.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 68 commits contributed to the release over the course of 98 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - git-protocol uses `oid` type (3930a6f)
    - Use new `oid` where possible in git-odb (68a709e)
    - Make ObjectId/oid happen! (ca78d15)
    - Remove all public exports of git-hash types in git-object (accf89d)
 * **Uncategorized**
    - (cargo-release) version 0.5.0 (02df134)
    - (cargo-release) version 0.8.0 (1a2a5cc)
    - (cargo-release) version 0.6.0 (8513f0f)
    - (cargo-release) version 0.10.0 (3161777)
    - (cargo-release) version 0.7.0 (b900914)
    - (cargo-release) version 0.4.0 (06612eb)
    - (cargo-release) version 0.12.0 (3b71e7e)
    - (cargo-release) version 0.2.0 (4ec09f4)
    - Remove locate(…) -> Option<Result<…>> in favor of Result<Option<…>> (40ee743)
    - A trial for Result<Option<Object>>  for loose object databases (3842859)
    - Merge branch 'daniel-levin/main' into main (1e727af)
    - dependency update (9e00d1b)
    - refactor (170215d)
    - Update goals and non-goals to not make them appear 'fixed' forever (f606075)
    - Add journey test (5c2fe3a)
    - Add experiment based on Josh Triplett's gist, related to #59 (76236d0)
    - Ensured linter checks pass (51f2183)
    - refactor (dee8c66)
    - Ensured output of directory-less git init unchanged (539a573)
    - Remove timesheet, move it to Byron/byron/timesheets/gitoxide.csv (a8899c9)
    - Added [directory] to lean CLI as well. (9c12f90)
    - Plans for 'gixp-cat' plumbing program (942e8bc)
    - Added [directory] argument to init. (62f8dc6)
    - (cargo-release) version 0.9.0 (efc8983)
    - (cargo-release) version 0.5.0 (3cc4a57)
    - (cargo-release) version 0.3.0 (d5c6643)
    - thanks clippy (f25598a)
    - thanks clippy (0fc239c)
    - Slim down git-config with cargo-diet (1c555e0)
    - [gix] Add optional zlib feature (f1f9665)
    - [organize]: make it work with bare and non-bare repositories (b85a389)
    - [organize]: Make client state meaning explicit (0f4265f)
    - [gitoxide-core] Fix find_origin_remote location (a3c19fc)
    - [gitoxide-core] Use git-config for remote url parsing (c45feed)
    - [gitoxide-core] Use git-config as dependency (c567925)
    - Make 'find' reproducable (c5af6eb)
    - mildly improve performance in case there is nothing to do for 'organize' (4f9fdc5)
    - Fix journey tests by not allowing canonicalization of possibly… (532ff2b)
    - Avoid claiming we would move something even though we won't (in 'organize') (47c7fb3)
    - (cargo-release) version 0.8.0 (1ccfdcd)
    - Implement `find` subcommand (28d506a)
    - (cargo-release) version 0.11.0 (1aa1f5e)
    - Fix tests (da94cfc)
    - thanks clippy (de32204)
    - Avoid moving nested repositories out of their place (5d7e6bf)
    - Recurse into directories much less… (87561eb)
    - Better use of jwalk filter capabilities… (781ea7f)
    - optimize number of CPUs for directory walk for M1 chips (129a699)
    - Remove usage of gitfeatures::fs in organize subcommand (b567d37)
    - prepare to put 'organize' behind a feature flag (9986509)
    - refactor; planning (5df492c)
    - fix progress (1abd761)
    - Assure basic 'organize' operation is working as expected (deb6073)
    - A version of organize which works; in theory (800a2f4)
    - A first stab at finding git repositories (e4dc964)
    - Fix verbose parsing unit tests (ce38ede)
    - (cargo-release) version 0.2.0 (0c39373)
    - thanks clippy (9e93a71)
    - first sketch of parsing git remotes (from git :D) (f8ab261)
    - first tiny journey test for dry run of organize subcommand (7bbba5a)
    - refactor (64495b0)
    - first sketch of interface for 'organize' subcommand (4f64d12)
    - silence so far unknown clippy lints (b5f2a4b)
    - thanks clippy (343ab9a)
</details>

## v0.7.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - use git-hash in git-features (5b307e0)
</details>

## v0.6.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 116 commits contributed to the release over the course of 82 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (4df97ce)
    - (cargo-release) version 0.3.0 (e60dbe6)
    - (cargo-release) version 0.6.0 (27f5955)
    - (cargo-release) version 0.2.0 (d61ad88)
    - (cargo-release) version 0.9.0 (a89fdb9)
    - (cargo-release) version 0.5.0 (fc7d600)
    - (cargo-release) version 0.5.0 (ae9c52b)
    - (cargo-release) version 0.2.0 (a476a46)
    - (cargo-release) version 0.5.0 (c767e07)
    - (cargo-release) version 0.8.0 (47c00c2)
    - cargo clippy Rust 1.48 (475a68c)
    - finish refactoring git-odb (ec282ae)
    - (cargo-release) version 0.7.0 (7fa7bae)
    - refactor (6b909a2)
    - refactor (b511a2b)
    - Merge branch 'commit-graph' into main (9cb09b2)
    - the daily commit (single handedly) (b528c2e)
    - Note about why git_features::hash::bytes_of_file() is not yet used (ca48fc4)
    - dependency update (988f905)
    - specify the hash to create with 'hash::bytes_of_file' (c000294)
    - document `loose::Object` entirely (d5eef9c)
    - move 'git_odb::hash::bytes_of_file' into git_features::hash (c5f6b45)
    - thanks clippy (b9e0a87)
    - Add and use borrowed::Id::null_sha1() (c717492)
    - docs for Sink (e7a09f0)
    - refactor (e4935e0)
    - a path towards making config Files editable (bc008c3)
    - replace 'ImpossibleVariantError' with 'std::convert::Infallible'` (c53638c)
    - additional setters for more fluid edits (5a54dae)
    - refactor (8c658da)
    - sketch out editing lossless of Files (8f00063)
    - Add lean-plumbing docs for path of commit-graph-verify (5c7b52d)
    - dependency update (7579b43)
    - [commitgraph] Clean up `{file,graph}::verify::Error` types. (fa22cab)
    - docs for compound object databases (813df71)
    - [commitgraph] Implement basic commit-graph file verification. (2571113)
    - Skip comments as well (32cc684)
    - [commitgraph] Loosen lifetime restrictions on return values. (701f33c)
    - Stop entry iteration when next section is encountered (83a1b83)
    - [commitgraph] Replace `T as U` with `U::from(T)` or `t.try_into()`. (28f94b4)
    - sketch of iteration over sections and entries (acb8947)
    - [commitgraph] Tweak `File::iter_base_graph_ids` implementation. (5b06780)
    - sketch out section and entries access (06679d9)
    - [commitgraph] Add `Graph::at` constructor. (a783052)
    - refactor (b5fa727)
    - [commitgraph] Validate trailer section when parsing files. (1b738ac)
    - Turn off 'unused' warnings for experimental git-config crate (0b52eb0)
    - [commitgraph] Use `thiserror` instead of `quick_error`. (c8b1f74)
    - Revert "remove git-config from workspace while it's so fresh…" (99214f4)
    - [commitgraph] Stub out commit-graph-verify plumbing command. (aacf0f0)
    - remove dash in all repository links (98c1360)
    - Merge branch 'main' into commit-graph (ca5b801)
    - [commitgraph] Attempt to fix bash script execution on Windows. (5e78213)
    - dependency update (44e0f05)
    - [commitgraph] Use crate::graph::Graph instead of crate::Graph. (21e4527)
    - thanks clippy (e355b4a)
    - [commitgraph] Rearrange some `use` statements. (185d14b)
    - refactor (5a1cbf2)
    - [commitgraph] Don't export Commit symbol at crate level. (be0e845)
    - And octal values unquoting works too (5effc7b)
    - [commitgraph] Include Conor in crate manifest. (000748c)
    - All explicit escapes (1841544)
    - [commitgraph] Add some doc comments. (6cf5cd8)
    - First bunch of simple unescapes (a45c594)
    - [commitgraph] Remove unused error variant. (66588f2)
    - prepare for actual unescaping (284da44)
    - [commitgraph] Rename GraphFile -> File. (f451822)
    - basic infrastructure for unquoting c-style strings (f81bb03)
    - [commitgraph] Rename CommitData -> Commit. (d8c2007)
    - fix incorrect cycle detection, which worked on MacOS by accident (a6e7765)
    - [commitgraph] Don't re-export graph_file symbols at crate level. (7c405ab)
    - Also use alternates for looking up objects… (ade929d)
    - Merge from main. (b59bd5e)
    - increase git-odb crate size limit (75bcc85)
    - [commitgraph] Ditch pre-generated test repos. (1ce8468)
    - refactor (8877b77)
    - prepare for unquoting c-strings (47e2fa0)
    - [commitgraph] Include in `make check` target. (724f391)
    - dependency update (7c2419b)
    - Read multiple alternates from single file; ignore comments (1f8d367)
    - [commitgraph] Remove `Kind` enum. (3c92761)
    - refactor (4a0d034)
    - support for relateive alternates (b20e9ee)
    - [commitgraph] Take `info` dir as arg, not `objects` dir. (36953e0)
    - refactor (485aa91)
    - Ignore all cycles and be happy if we have found at least one actual odb (1effdfd)
    - [commitgraph] implement basic, low-level read API (d1f0e9c)
    - refactor (c1d2f41)
    - prepare for multi-line parsing and all the bells and whistles (08f9ec4)
    - Revert "FAIL: try to get rid of tree-traversal Boxed error…" (1b42b31)
    - refactor (07aff14)
    - Make compound DB initialization less lazy… (6dc57b3)
    - try to get rid of tree-traversal Boxed error… (13159eb)
    - refactor (57d463f)
    - Use parallel walkdir (via jwalk) when parallel feature is enabled (f444c85)
    - Parameterize traversal error with Processor error (1513a13)
    - refactor (c6be43d)
    - alternate now handles cycles (71167e4)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d9)
    - refactor (524d0fe)
    - first simple alternate tests (7372118)
    - Prepare next iteration (4f656b2)
    - refactor (a8f4cd7)
    - test for circular alternates (fc92709)
    - Provide terminal dimensions to better use horizontal space (11f6b84)
    - Checksum verification for compound object (3be08b0)
    - dependency update (6b0796a)
    - asciinema link for pack-receive (79ac34c)
    - refactor (59d989a)
    - thanks clippy (4ddc64f)
    - asciinema link for remote-ref-list (aafd5f8)
    - More methods for compound object (84d2b0e)
    - Actually resolve alternates when creating a compound DB (9be7aed)
    - (cargo-release) version 0.4.0 (f667785)
    - refactor (e5a9343)
    - refactor (c1eff58)
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 65 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (92e8b27)
    - Finish removal of rust 2018 idioms (0d1699e)
    - first sketch of alternate resolution (6cc8a94)
    - (cargo-release) version 0.4.0 (2b1bca8)
    - refactor (ba1d883)
    - take not of a few more obscure features (8f9570c)
    - (cargo-release) version 0.4.0 (2272fa4)
    - refactor (7c3c80a)
    - (cargo-release) version 0.4.3 (5b47a1a)
    - (cargo-release) version 0.4.0 (0d7b60e)
    - refactor (8930610)
    - Enforce using the correct version of clap (fd6457f)
    - update dependency chain in release script (9af4799)
    - refactor (e4bcfe6)
    - remove quickerror dependency from git-odb (7e27495)
    - (cargo-release) version 0.2.0 (779e9d0)
    - refactor (6a84f13)
    - refactor (7874c35)
    - (cargo-release) version 0.2.0 (da830de)
    - refactor (4e89c3b)
    - refactor (3ec99dc)
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
</details>

## v0.4.1 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 51 commits contributed to the release over the course of 35 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [clone] support for progress that can handle writing pack files (46e0055)
    - [clone] Actually pass pack file to the delegate (94c5e62)
    - refactor (61e9812)
    - [ref-ls] first step towards supporting negotiation (27b6d2d)
    - [ref-ls] usable JSON output (735ae50)
    - [ref-ls] Fix progress display (2fcb557)
    - [ref-ls] Make things compile (b6506a4)
    - [ref-ls] And it even doesn't work if it is the very same transport (4ba50fe)
    - [ref-ls] first actual call of ls-remote, but… (5fc4330)
    - [ref-ls] Frame for remote-ref-ls command in gitoxide-core (161e7df)
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b879)
    - [clone] first sketch of transport layer's connection logic (f10cee5)
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive (5ea49c8)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 68 commits contributed to the release over the course of 30 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1)
    - bump minor version to 0.3 (4351e28)
    - first step towards parallelizing file hashes and traversal! (9573836)
    - update to quick-error 2.0 (4b1b784)
    - better progress for Sha1 of pack and index (310a59e)
    - first successful test of moving the streaming iterator into its own thread (c9fcb68)
    - unify used ranges for line renderer amond pretty and lean interface (f59f66e)
    - Add convenience method to get a new bundle for the index/data just written (a6d74ad)
    - support for JSON format output (1931575)
    - first pieces of the index-from-pack journey tests (181d69c)
    - more flexible error types for processors - anything goes (be3a947)
    - refactor (c7dd581)
    - interrupt support for pretty plumbing (bca7ce2)
    - count object types as well (e04a8d1)
    - refactor (b77d148)
    - remove memory mode entirely (and some complexity with it) (8812e91)
    - turns out you never want to keep deltas in memory (657aa2c)
    - Remove support for keeping compressed memory to reduce the index size (1e2ec7e)
    - Use call to produce the resolver, allowing to delay opening a file mapping… (dd30e8d)
    - minor fixes after first local tests - it's up to twice as fast!! (43c7fd1)
    - quick and dirty impl of lean command-line for index-from-pack (9660bbf)
    - quick and dirty impl of gitoxide layer for bundle writing, aka index-pack (e78386b)
    - first sketch of gitoxide index::from_pack(…) (da0eace)
    - refactor; better tests (12d14bf)
    - update tasks (45c3520)
    - it looks like something is wrong with the object stream implementation (d187b5a)
    - Loose object verifycation - but it doesn't seem to work as expected (9dd5676)
    - prepare full 'verify' implementation (ee45c7f)
    - refactor (0a33b24)
    - Allow sink-compress configuration; choose best algorithm (29b9c23)
    - Always compress values when using a sink when exploding packs (70562fa)
    - Most tests and clearer error message if object directory is inaccessible (1d8f597)
    - Nice error message on failure (adbc82c)
    - inform about deleted files using progress (a3ee516)
    - Don't uncondionally delete packs/indices on explode :D (1979715)
    - The first 'explode' implementation… (0d31ad1)
    - Get all pieces ready for action (1805d64)
    - Pass option for safety checks down to explode(…) (0bcb790)
    - Restore original verification functionality (0e3c1b9)
    - nearly there! Interesting that anyhow errors must be sync! (eaee77e)
    - refactor (bae7781)
    - refactor (f66b116)
    - basic tests and CLI args for explode pack (f932256)
    - refactor (d3c00c8)
    - (cargo-release) version 0.2.0 (76fe0ab)
    - (cargo-release) version 0.2.0 (0bb8314)
    - Run clippy first; pacify clippy (0a5b883)
    - use faster algorithm by default (bb45c3d)
    - refactor; enable testing of reverse-delta lookup (512daf9)
    - Fix clippy (ec40e09)
    - refactor (fdfab40)
    - Easy access to sorted offsets in pack index files (d93540f)
    - refactor (cb8d561)
    - Change course and do pack streaming first (bcb275e)
    - Switch to latest quick-error (9760856)
    - Fully implement --encode and --re-encode flags (a7cfac8)
    - prepare for re-encoding each pack object (afae684)
    - move git_object::Id into git_object::owned::Id - much better already! (50c7136)
    - fix naming change, which was introduced accidentally (fbb9f98)
    - refactor (34e85f2)
    - refactor (2888f1b)
    - refactor (dcacd3b)
    - refactor (b113da9)
    - refactor (bed5dc8)
    - refactor (8b416d4)
    - Respect thread limit in 'in_parallel' (babfd84)
    - pass threadlimit down from CLIs (f98c5b1)
    - add new Context argument to support more configuration options (7c5d8b8)
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 19 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Cargo-diet for the top-level crate (19e7fec)
    - Make crates publishable (5688a34)
    - Add metadata to allow docs.rs build all featueres (10f9386)
    - first release test (3ef85fc)
    - git-odb with serde support (0da930c)
    - pass serde1 through from gitoxide (1991b9f)
    - don't print 'OK' at the end of verify-pack (4956ef2)
    - \#[forbid(unsafe)] for all crates (afda803)
    - Allow for more screen space when formatting (6794300)
    - disable LRU cache if we have to get statistics (befba3b)
    - wonderful statistics on compression efficiency! (1bb09c5)
    - pretty-print objects per delta chain length (66553b1)
    - count objects per chain level (209d53f)
    - Pretty-printing of some statistics (125b565)
    - fix pretty build (6adf615)
    - pass average stats through to the top level (5b4979c)
    - first very basic progress implementation (b820717)
    - Pass progress everywhere, for now just to discard it (da3ae1c)
    - Control which hashing crates to use from the top-level as well. (dfe9b20)
    - Use git-features to toggle 'parallel' mode from the 'gitoxide' level (d944fbf)
    - first working version of actually parallel `in_parallel` (145ee39)
    - Support for verifying pack files and index files (b09b4e1)
    - cleanup - don't build and run tests while there is nothing to test (4a153da)
    - First basic index file verification (994700f)
    - reorganize crates to make 'gitoxide' the CLI, and 'gitoxide-core' the library (0ac9c5a)
    - rename grit to 'gitoxide', CLI name is 'gio' (9d6007f)
</details>

