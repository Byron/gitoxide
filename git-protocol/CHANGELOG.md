# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### Unreleased

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 16 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 221 commits contributed to the release over the course of 12 calendar days.
 - 53 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

#### Commit Details

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
    - fix docs (90056c8)
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
    - Show with simple example how the round-tripping works, neat (9510d9b)
    - collect unknown text so things don't get lost entirely… (60040c9)
    - parse back what we write out, perfectly… (5cab315)
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
    - commit traversal along the first parent… (7bce49c)
    - git-ref(docs): improve changelog format (90e6128)
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
    - describe variants (899c579)
    - parse entry mode into number instead of comparing it to byte strings (83d591d)
    - ObjectID specific hashers, using the fact that object ids are hashes (f9232ac)
    - Tree parsing now probably is twice as fast… (d1e2b89)
    - Use a custom hasher for 'seen' objects hashset… (70179e2)
    - don't include submodules in count… (faf6f81)
    - control pack and object cache size in megabytes (60c9fad)
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
    - Merge branch 'changelog-generation' (bf0106e)
    - thanks clippy (b856da4)
    - thanks clippy (31498bb)
    - let's not force folks to not use debug info… (bc458c8)
    - Merge branch 'main' into changelog-generation (c956f33)
    - thanks clippy (c55f909)
    - don't claim to change manifest version if it's the same one (11eebdc)
    - thanks clippy (b200ee8)
    - thanks clippy (4b3407d)
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
    - Release gitoxide-core v0.10.5 (590e59b)
    - Release git-ref v0.7.2 (e940e9a)
</details>

### v0.10.4 (2021-09-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
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

### v0.10.3 (2021-09-07)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.3 (aa90f98)
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
</details>

### v0.10.2 (2021-08-29)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

### v0.10.1 (2021-08-27)

- instruct docs.rs which features to use for more useful documentation



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

### v0.10.0 (2021-08-27)

- Various minor updates of pre-release dependencies

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 168 commits contributed to the release over the course of 3 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6b)
 * **Uncategorized**
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
    - Fix formatting of performance-tasks.md (917967e)
    - Merge branch 'Byron:main' into main (dc58eca)
    - Release git-actor v0.4.0 (16358c9)
    - Allow creation of empty indices (d122fc7)
    - Release git-testtools v0.5.0 (574ede9)
    - [actor #173] fix docs (2d7956a)
    - A note about the project board to help with transparency (d850004)
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
    - cleanup imports (e669303)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (7bd3671)
    - update dependencies (e9a98bc)
    - [pack #164] fix docs (08ee674)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58)
    - Merge branch 'main' into 162-repo-design-sketch (e63b634)
    - [stability #171] update README with stability information… (f330daa)
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" (4c2ea21)
    - [stability #171] How to handle the MSRV (9be1fce)
    - [pack #167] Use custom uluru version to avoid a lot of allocations… (8d49976)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f)
    - [pack #167] a single-threaded special case for counting… (65e29de)
    - [stability #171] about transitioning from pre-release to release (bdbdb65)
    - [pack #167] generalize over immutable insertions… (169f000)
    - [stability #171] finish tier description… (4fe1259)
    - [pack #167] refactor (6bf0f7e)
    - [stability #171] Rough descriptions of ST 3 and 2 (340935c)
    - [pack #167] progress is handled by reducer… (a22f8e1)
    - [stability #164] First sketch of stability MD… (a7353cd)
</details>

### v0.9.0 (2021-08-17)

#### BREAKING

- Add fifth argument to `fetch(…)`


#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.9.0 (466d8ea)
    - [protocol] prepare release to fix crates-io instalations (83d7423)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e3340)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

### v0.8.1 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.8.1 (b57c339)
    - Release git-transport v0.10.0 (b944278)
    - Release git-packetline v0.9.0 (7ffbd60)
    - remove dev-dependency cycles by removing their version (c40faca)
    - bump transport version to 0.10 (f26a3d3)
    - (cargo-release) version 0.8.0 (ad6d7f9)
    - (cargo-release) version 0.7.0 (2ef3106)
    - [transport] A much better name for 'is_stateful()` (f15f1e8)
    - [protocol] Make fetch-connection usage explicit (29696f9)
</details>

### v0.8.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 125 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - [protocol RL-#741] Respect delegate configuration when running only ls-refs (65ce8e1)
    - [protocol #145] Unify the `previous` and `previous_result` parameters… (96f77c7)
    - Merge pull request #145 from kim/ref-in-want-corrections (8dfc943)
    - Turn incremental compilation in dev off for now (2449621)
    - [protocol] remove misleading documentation about ref-in-want (9a8f6b5)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820)
    - [protocol] Delegate will indicate end-of-operation when fetch is done (928f75a)
    - [protocol] Let 'fetch()' only be used via `git_protocol::fetch` (4bae2f9)
    - thanks clippy (eccbecb)
    - [protocol] fix build (38aca40)
    - [protocol] Allow both preparation delegate methods to fail (d89393b)
    - [protocol] start trying LsRefsAction::Abort(Box<dyn Error>)… (660b9dc)
    - Merge branch 'negotiate-fallible' (27c8abe)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97)
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6)
    - [actor] refactor (bccb738)
    - [protocol] fallible negotiation (e269a2c)
    - [actor] FAIL an attempt to remove btoi errors (3f99cf5)
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… (4bb9bba)
    - [protocol] only send flush packets in stateful connections (0995c22)
    - [transport] remove Transport::close()… (4268a9b)
    - [ref] rename Action::Close to Action::Cancel… (cac1f6c)
    - [transport] impl Delegate for &mut T: Delegate; refactor fetch() signature (2ded7f9)
    - [transport] implement Transport for &mut T: Transport as well (372fb81)
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
    - [transport] tests for extra parameters (fffd926)
    - [protocol] extra_parameters are forwarded from delegate to handshake (03e3db3)
    - [transport] unsupported protocol versions now abort the fetch operation (812aa3b)
    - [transport] flexible version of version support check doesn't actually work :D (2b220f0)
    - [protocol] make refs parsing functionality public (d6da891)
    - [protocol] async-io path handles improved refs parsing (328ab9c)
    - [protocol] first step towards keeping InternalRef internal in blocking-io (6c4ed2d)
    - refactor (24697bc)
    - [async-client] cleanup Send bounds! (c7dee44)
    - [async-client] refactor (b252932)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - Revert "[async-client] a taste of what it means to unblock the delegate" (2ba452f)
    - [async-client] a taste of what it means to unblock the delegate (4d6c10a)
    - [async-client] prepare for unblocking the protocol delegate (796c7d5)
    - [async-client] refactor (0d5b911)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - [git-protocol] fix test (e30ea36)
    - [git-protocol] no warnings when building without client (2f30666)
    - (cargo-release) version 0.2.0 (3286e42)
    - Manually fix crc in tooling (48fa9bc)
    - [git-protocol] update cargo-features (1fdb5ac)
    - Bump crc from 1.8.1 to 2.0.0 (07f08ac)
    - [git-protocol] remove compile warnings if no client type is specified… (478a980)
    - tryout dependabot (872eb12)
    - thanks clippy (57106e2)
    - fix docs (bca7594)
    - [git-transport] Fix http build (3469e99)
    - [git-protocol] fix build (4cce648)
    - [git-protocol] builds without features work (a1945ff)
    - [git-protocol] async Delegate (1aa6781)
    - [git-protocol] async fetch tests work (fe434a5)
    - thanks clippy (0759ade)
    - [git-protocol] fetch tests nearly compile in async (97fb186)
    - [git-transport] refactor (d09153f)
    - [git-protocol] fetch in sync and async… (4776039)
    - [git-transport] Properly implement Transport for Boxed types (47b10c9)
    - [git-protocol] refactor (80379fd)
    - [git-transport] refactor (3b0baee)
    - [git-protocol] build should fail if mutually exclusiive features are set (72cf940)
    - [git-protocol] refactor (94d7be4)
    - dependency update (6d2278b)
    - [git-protocol] refactor (990099b)
    - Bump crossbeam-utils from 0.8.4 to 0.8.5 (fce4d10)
    - [git-protocol] refactor (d623cf7)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a8)
    - [git-protocol] async response (c498557)
    - Bump cargo_toml from 0.9.1 to 0.9.2 (28687b1)
    - [git-protocol] refactor (a8dc078)
    - refactor (2eefe17)
    - [git-protocol] prepare response module for async (08b891b)
    - [git-protocol] fix tests without any feature toggles (1da0b1a)
    - thanks clippy (91fdfba)
    - [git-protocol] refs now available in async (3a5b2cf)
    - [git-protocol] refactor (abf0b9d)
    - [git-protocol] prepare to translate refs (bf79c91)
    - [git-protocol] no warnings if there is no client feature set (335e831)
    - [git-protocol] fix tests in case there is no client feature set (1ee5518)
    - [git-protocol] refactor (0b4ff16)
    - [git-protocol] refactor (e99a03b)
    - [git-protocol] async capabilities and arguments abstractions (aa3eacb)
    - [git-protocol] now just a dummy async transport impl and… (c7f0b80)
    - [git-protocol] a big step towards getting 'Arguments' test into async (5d1c30f)
    - [git-protocol] move everything into `blocking_io` for later translation… (fa03374)
    - [git-protocol] all blocking fetch tests (0d39b5d)
    - [git-protocol] re-introduce credentials helper code (6a5575f)
    - [git-protocol] separate test configuration for async mode (62a117c)
    - [git-transport] fix git-protocol (0cc9537)
    - [git-protocol] simplify test setup (189ed2c)
    - refactor (2ba9f91)
    - (cargo-release) version 0.4.0 (866f86f)
    - Switch to latest nom (859e57e)
    - (cargo-release) version 0.15.0 (d69d9fb)
    - Put prodash behind a feature toggle, too (966058d)
    - [git-packetline] refactor (1328c5b)
    - (cargo-release) version 0.6.0 (ec5a54e)
    - [git-packetline] refactor (e5769d1)
    - (cargo-release) version 0.8.0 (ccea4b6)
    - (cargo-release) version 0.9.0 (18f6d01)
    - [git-transport] simplify parsing capabilities from lines (401af09)
    - [git-protocol] separate tests those who need feature toggles (4a49d64)
    - [git-transport] remove default features to force being explicit everywhere (d1b39f8)
    - Fix git-protocol (284f8af)
    - refactor (1412282)
</details>

### v0.7.0 (2021-05-09)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-transport/0.8.0 (76f7f1c)
    - (cargo-release) version 0.7.0 (069184e)
    - (cargo-release) version 0.8.0 (411a05e)
    - (cargo-release) version 0.5.0 (8c4cc3f)
    - thanks clippy (17258cc)
    - (cargo-release) version 0.14.0 (a760f8c)
    - (cargo-release) version 0.3.0 (e9665c7)
    - (cargo-release) version 0.13.0 (ac2eddb)
</details>

### v0.6.0 (2021-04-08)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - git-protocol uses `oid` type (3930a6f)
    - refactor; better errors for invalid hash sizes (be84b36)
    - Make ObjectId/oid happen! (ca78d15)
    - Remove all public exports of git-hash types in git-object (accf89d)
    - Remove re-export of git_object::borrowed::Id (a3f2816)
    - Make git-hash Error usage explicit (it's for decoding only) (4805cfc)
 * **Uncategorized**
    - (cargo-release) version 0.6.0 (8513f0f)
    - (cargo-release) version 0.7.0 (334b7e1)
    - (cargo-release) version 0.12.0 (3b71e7e)
    - (cargo-release) version 0.2.0 (4ec09f4)
</details>

### v0.5.0 (2021-03-26)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (3cc4a57)
    - (cargo-release) version 0.6.0 (50fb6f2)
    - thanks clippy (0fc239c)
    - thanks clippy (749ceba)
    - (cargo-release) version 0.11.0 (1aa1f5e)
</details>

### v0.4.1 (2021-01-05)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (6244fb4)
    - finish docs for `git-protocol` crate (598f700)
    - revise trait documentation of git-protocol (5271128)
    - docs for response in git-protocol (487de13)
    - more docs for git-protocol (bca0cbd)
    - docs for fetch::refs (6a97a3e)
    - docs for git credentials helper utilities (eb6bb6e)
    - first pieces of docs for git-protocol (12d8a83)
    - thanks clippy (343ab9a)
</details>

### v0.4.0 (2020-12-16)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - use git-hash in git-features (5b307e0)
</details>

### v0.3.0 (2020-12-15)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (e60dbe6)
    - (cargo-release) version 0.4.0 (32aefc0)
    - (cargo-release) version 0.4.0 (72eaece)
    - (cargo-release) version 0.9.0 (a89fdb9)
    - (cargo-release) version 0.5.0 (fc7d600)
</details>

### v0.2.0 (2020-12-15)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 67 commits contributed to the release over the course of 82 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (a476a46)
    - (cargo-release) version 0.3.0 (d19ee35)
    - (cargo-release) version 0.3.0 (eade7d1)
    - (cargo-release) version 0.8.0 (47c00c2)
    - cargo clippy Rust 1.48 (475a68c)
    - (cargo-release) version 0.7.0 (7fa7bae)
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
</details>

### v0.1.1 (2020-09-14)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 201 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [clone] refactor (ded46fd)
    - [clone] support for progress that can handle writing pack files (46e0055)
    - [clone] leave aborting the negotiation loop in the hands of the delegate (ea83ce7)
    - [clone] sideband-all support (ecc8e09)
    - [clone] Actually pass pack file to the delegate (94c5e62)
    - [clone] Response parsing up to (optional) pack (24064c7)
    - [clone] FAIL: try to model pack reading using ownership… (4ee14e3)
    - [clone] properly handle V2 response parsing (0d7d768)
    - refactor (f2c31ec)
    - refactor (fab9f99)
    - [clone] Provide a handle to the packfile, if it is present in the response (fcb4cc1)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912)
    - refactor (feec5be)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a)
    - [ref-ls] basic V2 acknowledgement and packfile parsing, but… (549f404)
    - thanks clippy (ac88eef)
    - [ref-ls] parse all V1 acknowledgements, without duplication (f7c1580)
    - [ref-ls] first stab at V1 acknowledgement parsing (1d21cd4)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba787)
    - thanks clippy (27f30df)
    - [ref-ls] support for line peeking in packet line readers (0c0c575)
    - [ref-ls] Let's make Acks copy, because owned::Id is as well (1f9cc44)
    - refactor (935d5fe)
    - [ref-ls] first sketch of V1 tests for result parsing (ack + pack) (fd16a5f)
    - [ref-ls] tests for stateless V1/V2 (d34afc6)
    - [ref-ls] first step towards parsing negotiation result (51ecf7e)
    - refactor (61e9812)
    - thanks clippy (6b1294a)
    - [ref-ls] Argument tests for fetches (50cd260)
    - [ref-ls] first argument tests for clone (83490ef)
    - [ref-ls] Also add 'haves' in V2; some more assertions (3e6bfb1)
    - [ref-ls] Do feature assertions to not have to support old servers (9980ff9)
    - [ref-ls] don't do anything on drop (9f18d9b)
    - [ref-ls] A step towards getting the negotiation right, really need tests (abb56d8)
    - [ref-ls] Transport layer knows whether it's stateful or not (22c3640)
    - [ref-ls] Also re-send V1 features in each request, independently of statefulness for now (f8669d6)
    - [ref-ls] potentially fix 'is-done' logic (f9e338f)
    - [ref-ls] Sketch of sending arguments in V1 & V2 (e1d27b6)
    - [ref-ls] first step towards supporting negotiation (27b6d2d)
    - [ref-ls] probably all it takes to handle all capabilities of fetch arguments (d956ecc)
    - [ref-ls] first sketch of argument utility to help creating wants/haves (b0b0166)
    - [ref-ls] fix feature validation in V2 (eb387d2)
    - update tasks (079fc02)
    - [ref-ls] Always send a flush before closing the connection (918f19f)
    - [ref-ls] Make credentials helper truly work (7f3c3a7)
    - [ref-ls] And it even doesn't work if it is the very same transport (4ba50fe)
    - [clone] support automatic downgrade to protocol version 1 (4cf3643)
    - [clone] basic progress for fetch in protocol (1925d02)
    - refactor (aa7e8b1)
    - refactor (b97507e)
    - [clone] update README, improve delegate docs (dc7908f)
    - [clone] test ls-remote V2 (0907771)
    - thanks clippy (baf0b2c)
    - [clone] more tests for fetch features and arguments (a946861)
    - [clone] features for V1 fetch (5b24a55)
    - [clone] assert on ref-prefix for ls-refs command (70347a5)
    - thanks clippy (d55cd56)
    - refactor (f02232d)
    - [clone] Getting there with feature handling for ls-refs (27c5adc)
    - [clone] Remove intermediary mutable Capabilities implementation (f59344a)
    - refactor (5ea42ba)
    - [clone] first step towards better organizing features/capabilities/argument names (7d45f3a)
    - dependency update (dea0028)
    - [clone] first sign of somethign working: ls-remote (df58fa1)
    - refactor; thanks clippy (03c3d17)
    - refactor (25122f2)
    - [clone] V2 ref parsing (455fa0f)
    - [clone] A better way to set the agent in V2 invocations (325d3a2)
    - [clone] Make the actual ls-refs call (898cb8b)
    - [clone] sketch of delegating simple commands along with arg/feature verification (c2ebc48)
    - refactor (a6bcdc4)
    - ignore keep-alive packages in case of 'sideband-all' (2e77b86)
    - refactor (ad0b2e9)
    - thanks clippy (8b1ea29)
    - [clone] apply another mild workaround to be able to use 'transport.close()' (ea636ae)
    - [clone] remove workaround (55cf167)
    - [clone] more safety checks (6f5a9f3)
    - thanks clippy (423458e)
    - refactor (f29ea65)
    - [clone] proper parsing of V1 refs (d262307)
    - [clone] A little more ref V1 parsing (4bc7842)
    - [clone] preparation of test for proper ref parsing (V1) (85cd580)
    - refactor (99247f4)
    - refactor (c985370)
    - [clone] symref parsing from capabilities (8c2ff64)
    - [clone] A step closer to parsing symrefs correctly (250a340)
    - [clone] attempt to make refs more accessible… (fa1112c)
    - refactor (c138059)
    - [clone] Prevent accidental leakage by transforming back to the 'right' type (2d469c6)
    - thanks clippy (9afa7f9)
    - [clone] a better workaround for the 'drop scope' issue (3ccf32b)
    - [clone] First step of workarounding rusts drop rules (6b47923)
    - [clone] update tracking ticket information (650c452)
    - [clone] add Rustc issue to see if this is just my bad (ccb9b53)
    - thanks clippy (fd6f9e5)
    - [clone] Workaround for the drop-issue (43c6159)
    - [clone] first attempt at adding authentication logic, but… (a36d14a)
    - [clone] first rough sketch of (mutable) capabailities in the protocol side (13f7ecb)
    - refactor (a567b24)
    - refactor (88ecda1)
    - [clone] frame for first 'fetch' tests (2da70f6)
    - refactor (89aabde)
    - refactor (51f6142)
    - [clone] support for git-credentials helper (a6546da)
    - refactor (cf0e45a)
    - [clone] decoding of credential message replies (1c2f56d)
    - [clone] encode message for git credentials helper (143549e)
    - [clone] sketch for identity handling (b23f470)
    - [clone] put remaining remote progress parsing code into protocol (e03e0e5)
    - refactor - decouple protocol from packetline (dc98db2)
    - [clone] move packet-line code into own crate (879af67)
    - [clone] move packet-lint into transport layer (c0dd831)
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive (5ea49c8)
    - [url] basic frame and first failing test (60aacf0)
    - [protocol] properly implement remote progress reporting (a81954a)
    - refactor (66e9cd1)
    - thanks clippy (7f6e290)
    - [protocol] prepare passing most of remote progress on to prodash… (b8a34e5)
    - refactor (df8ebdc)
    - refactor (2ea3288)
    - refactor (2102cab)
    - [protocol] remote::Progress can now parse the usual progress (b0e5601)
    - [protocol] first steps towards parsing remote progress (c3d0e7a)
    - [protocol] even starting to parse remote progress by hand is painful… (d68db3c)
    - Less ambiguous name for 'index-from-pack': 'pack-index-from-data' (386673c)
    - [protocol] handle errors as well; transmit progress (first part) (c484398)
    - [protocol] first successful test with pack reading (ad1e8bf)
    - [protocol] first stab at decoding sidebands in Read (51fe596)
    - [protocol] allow Reader delimiter to be configured (5a01596)
    - refactor (78f27d8)
    - Revert "[protocol] an alternative version with external buffer" (157d810)
    - Revert "[protocol] But external buffers also don't help at all" (579a697)
    - [protocol] But external buffers also don't help at all (8e711df)
    - [protocol] an alternative version with external buffer (a862d22)
    - [protocol] a struggle - putting buffers in Read adapters = bad idea (e257426)
    - [protocol] FAIL: keep referenced PacketLine for minimal copy (7e4d1f3)
    - [protocol] sketch of Read impl for pack line iterator (fe3b050)
    - refactor (c81caa3)
    - Revert "[protocol] FAIL: attempt to add an actual Iterator impl for packet lines" (2989781)
    - [protocol] FAIL: attempt to add an actual Iterator impl for packet lines (a6e4cb1)
    - refactor (20b10c5)
    - [protocol] thanks clippy (10b9017)
    - [protocol] tests for the reader (86d1a40)
    - [protocol] A chance for the reader to actually work (d6aebed)
    - refactor (8ebdcbd)
    - [protocol] FAIL: finally the reader compiles with the 'slice split technique'… (58543cb)
    - [protocol] FAIL3: giving up - it's quite impossible to do that without 'bytes' (047d67c)
    - [protocol] reader FAIL: wherever the loop moves, it will not borrowcheck (cb154f2)
    - [protocol] FAIL2: lifetime issues with loop (c2ff070)
    - [protocol] decode-band can fail on malformed input (0f468f9)
    - refactor (ed1f364)
    - [protocol] better handling of text-lines (7ad1db0)
    - [protocol] attempt to implement a streaming pack line reader (FAIL :D) (cc45cec)
    - [protocol] add cargo-diet assertions (831b758)
    - refactor (73e24c9)
    - [protocol] side-band channel encoding and decoding (9b4fb3e)
    - [protocol] suppot for V2 special lines (4e46719)
    - Encode and decode errors (3f4fd90)
    - decode ERR lines as actual errors (1f58568)
    - more tests (c34d88b)
    - the first succeeding tests for streaming decoding :D (7ea25c5)
    - first stab at implementing streaming decoding of packet line… (843c6fb)
    - cargo fmt (60cd21b)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (7e3f67d)
    - packet line encoding with flush support (e924a59)
</details>

### v0.1.0 (2020-09-12)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (2272fa4)
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
    - (cargo-release) version 0.4.1 (105c501)
    - [clone] more correct handling of 'no-done'/done when sending wants/haves… (50f4516)
</details>

### v0.0.0 (2020-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - some README updates (1461514)
    - first bunch of tasks I see after studying parts of the protocol docs (9bd97ba)
</details>

