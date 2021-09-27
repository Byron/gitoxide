# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 16 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 216 commits contributed to the release over the course of 13 calendar days.
 - 48 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - Generate changelogs with details (fd0f3bd)
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
    - Implement --write actually (69d36ff)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
    - Sketch merging logic… (1932e2c)
    - Make use of fixed git-conventional (b7b92b6)
    - prepare test for basic merging… (0a14ced)
    - update git-conventional dependency (2d369e8)
    - nicer 'thanks clippy' message (4344216)
    - first test and sketch for stripping of additional title values (55b7fe8)
    - Basic message parsing, either conventional or not, without additions (b3b6a2d)
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
    - split data::output::count::objects into files (8fe4612)
    - use new git_pack::cache::Object trait (b209da2)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106e)
    - thanks clippy (b856da4)
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

### v0.10.1 (2021-09-07)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 64 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.1 (4f9da02)
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
</details>

### v0.9.1 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.1 (2276e2a)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

### v0.9.0 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.0 (7ffbd60)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
</details>

### v0.8.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (ad6d7f9)
    - (cargo-release) version 0.18.0 (b327590)
</details>

### v0.7.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (2ef3106)
    - (cargo-release) version 0.17.0 (c52a491)
</details>

### v0.6.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 136 commits contributed to the release over the course of 89 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#77**
    - [git-packetline] refactor (aa61993)
 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref] refactor (bd94ea5)
    - [pack] fix docs (e7b9d96)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820)
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - Remove unnecessary unsafe code (83e207a)
    - [ref] fix compile warning on windows (c328774)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Implement Parser::into_iter without extra allocation (aa79924)
    - dependency update (059fa33)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - fix docs (2698dae)
    - fix build (22bda81)
    - thanks clippy (3f7e27b)
    - thanks clippy (6200ed9)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - Prevent selecting mutually exclusive features (7f5da18)
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
    - refactor (14c9093)
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support (ee01c79)
    - [git-transport] ExtendedBufRead for Async… (d4e56c8)
    - (cargo-release) version 0.16.0 (769c649)
    - [git-packetline] refactor (7e513f1)
    - [git-packetline] Switch back to pin-project-lite (63cb0fc)
    - [git-packetline] all tests green (fed6c69)
    - [git-packetline] Nearly there - one failing test and its known why it does that (51c63c0)
    - [git-packetline] another green test (e67d77d)
    - [git-packetline] Custom implementation of read_line future to avoid extra work… (91c2895)
    - [git-packetline] read_line test green, but… (8007c65)
    - [git-packetline] fix compile errors if no features are specified (a2b44c8)
    - [git-packetline] YES, finally, the first green test (f16b012)
    - Revert "Revert "[git-packetline] It compiles with parent as option, even with state machine"" (e300f9f)
    - Revert "[git-packetline] An Option really does the trick" (8eb78f5)
    - [git-packetline] An Option really does the trick (c05bd79)
    - Revert "[git-packetline] It compiles with parent as option, even with state machine" (890cc50)
    - [git-packetline] It compiles with parent as option, even with state machine (a97bbfd)
    - [git-packetline] Even without pin projection lifetimes don't add up (7e834f5)
    - [git-packetline] [FAIL] For some reason the is a lifetime mismatch again… (b4ff4e7)
    - [git-packetline] first step towards state based impl (22740c5)
    - [git-packetline] Use what's learned previously to make it compile without added buffer (88511f7)
    - Revert "[git-packetline] get it to compile by resorting to another buffer" (3866517)
    - [git-packetline] get it to compile by resorting to another buffer (01e15c8)
    - [git-packetline] [HACKY-SUCCESS] It's possible to do it, but how to do it without unsafe? (96d0ecf)
    - [git-packetline] [FAIL] No, cannot poll a dynamically created future (194c991)
    - [git-packetline] [FAIL] try to brute-force keeping futures for polling… (42a7d00)
    - [git-packetline] [FAIL] try to impl fill_buf - can't return parent buffer (1e8b006)
    - [git-packetline] Upgrade to pin_project as drop impl is needed (3d53404)
    - [git-packetline] A step towards implementing poll_fill_buf (3c487de)
    - [git-packetline] Frame for async sideband (adc365e)
    - [git-packetline] Use underlying StreamPeekIter buffer instead of copying into own (88b8bc3)
    - [git-packetline] [FAIL] try to get rid of second buffer in sideband reader (4d8f4b5)
    - [git-packetline] streaming peek iter with async support (60164fd)
    - [git-packetline] fix docs (4a47c9e)
    - [git-packetline] refactor (e8b2dd1)
    - [git-packetline] Async IO for packetline serialization. (3bb9cf1)
    - [git-packetline] refactor (2a84b78)
    - [git-packetline] encode module now available as async edition (119fcc3)
    - [git-packetline] Use io::(Result|Error) everywhere (374f129)
    - [git-packetline] Deduplicate 'encode' module tests (34f48c3)
    - [git-packetline] refactor (f038ca1)
    - [git-packetline] remove now unnecessary duplicate tests (c8178d7)
    - [git-packetline] Use maybe_async to deduplicate tests - neat (439a7b7)
    - [git-packetline] refactor (d698d7b)
    - [git-packetline] All tests for high-level writer pass (eef8c9f)
    - [git-packetline] OMG it's green! (fbffd89)
    - [git-packetline] An owning inplementation of the LineWriter (70ce3c9)
    - [git-packetline] An owning LineWriter (445fac6)
    - Revert "[git-packetline] Use no pin projections" - let's own the writer (6c5750a)
    - [git-packetline] Use no pin projections (dc4e0e5)
    - [git-packetline] Allow different lifetimes for writer and buffers (3b3c53d)
    - [git-packetline] A complete LineWriter implementation by hand, OMG (3299548)
    - [git-packetline] write prefix properly (432b214)
    - [git-packetline] write hex_len properly (acdcfb7)
    - [git-packetline] it compiles, but write_all needs to be implemented by hand (2c44350)
    - [git-packetline] First draft of LineWriter - and it shows some teeth (13127ee)
    - [git-packetline] Make failing test pass officially for now (cbd6291)
    - [git-packetline] it turns out that a simple write trait isn't simple (7933698)
    - [git-packetline] Calling auto-generated futures isn't easy :D (8361238)
    - [git-packetline] All encode capabilities that Write needs (88a971d)
    - [git-packetline] the first green encode test (ebc4703)
    - [git-packetline] Now maybe_async would be useful (ab4b30e)
    - [git-packetline] refactor (7d79288)
    - [git-packetline] fix tests (b26c43b)
    - [git-packetline] prepare 'packetline' and 'encode' for async (1a986fb)
    - [git-packetline] One tiny step closer, and it's obvious there is more IO :D (0bef59c)
    - [git-packetline] the first green test (916c862)
    - [git-packetline] the first very failing test… (0220bca)
    - [git-packetline] add async-io feature toggle (727ad97)
    - refactor (c8ba842)
    - [git-packetline] 'blocking-io' feature toggle and tests'blocking-io' feature toggle and tests (380e8b2)
    - [git-packetline] fix doc links (cf50f28)
    - [git-packetline] refactor (1328c5b)
    - thanks clippy (334e129)
    - [git-packetline] Fix performance regression (513e7ad)
    - [git-packetline] Deduplicate read-line logic as well, with perf regression (1c13706)
    - [git-packetline] refactor (17ab380)
    - [git-packetline] Step one towards less code duplication (d863de0)
    - [git-packetline] more docs (4591e46)
    - (cargo-release) version 0.6.0 (ec5a54e)
    - [git-packetline] refactor (e5769d1)
    - [git-packetline] refactor (fef3c9f)
</details>

### v0.5.0 (2021-05-09)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 133 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-odb/0.15.0 (7617998)
    - (cargo-release) version 0.5.0 (8c4cc3f)
    - (cargo-release) version 0.15.0 (d91b241)
    - (cargo-release) version 0.14.0 (d9514ee)
    - (cargo-release) version 0.13.0 (5c791af)
    - refactor (77764f3)
    - refactor (edf7d38)
    - refactor (ca98221)
    - bump git-odb minor version (5c833ce)
    - (cargo-release) version 0.11.0 (fd698e3)
    - (cargo-release) version 0.10.0 (3161777)
    - (cargo-release) version 0.9.0 (efc8983)
    - (cargo-release) version 0.8.0 (1ccfdcd)
    - thanks clippy (343ab9a)
    - deny missing docs for git-packetline (3a78840)
</details>

### v0.4.1 (2020-12-26)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 9 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 (7c623de)
    - Finish git-packetline docs (7ae3e73)
    - last remaining docs prior to refactoring (da966fc)
    - docs for encode (213924d)
    - docs for ReadWithSidebands (e277cce)
    - Finish `Provider` docs (832f7f3)
    - more docs for git-packetline (3c7e727)
    - Some more docs for git-packetline (77edb62)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
</details>

### v0.4.0 (2020-12-15)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (72eaece)
    - (cargo-release) version 0.6.0 (27f5955)
</details>

### v0.3.0 (2020-12-15)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (eade7d1)
    - (cargo-release) version 0.5.0 (c767e07)
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
    - (cargo-release) version 0.4.0 (2272fa4)
    - (cargo-release) version 0.4.3 (5b47a1a)
    - (cargo-release) version 0.4.0 (0d7b60e)
    - Enforce using the correct version of clap (fd6457f)
    - update dependency chain in release script (9af4799)
    - refactor (e4bcfe6)
    - remove quickerror dependency from git-odb (7e27495)
    - (cargo-release) version 0.2.0 (779e9d0)
    - refactor (6a84f13)
    - refactor (7874c35)
</details>

### v0.2.1 (2020-09-14)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 69 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - thanks clippy (6aeb68c)
    - [clone] support for stopped_at() in provider and reader (6bd8c87)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912)
    - refactor (feec5be)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba787)
    - [ref-ls] support for line peeking in packet line readers (0c0c575)
    - [ref-ls] don't do anything on drop (9f18d9b)
    - fix packet-line tests (0939e6c)
    - [clone] Don't expose hex-error in public interfaces anymore (92dab30)
    - refactor (c138059)
    - refactor (f2ff90d)
    - [clone] a way to change progress handling on the fly (c1bcc0a)
    - refactor (aceaaed)
    - refactor (2cdda7a)
    - [clone] Sketch 'request()' implementation for git protocol (fd0e0e9)
    - [clone] the problem actually was rooted in trying to read binary data (b7af002)
    - [clone] first impl of custom read-line (still fails) (7f2bdfa)
    - [clone] Add test which probably indicates the need for a custom read_line(…) (2360a70)
    - refactor (359765a)
    - [clone] more tests for progress line handling (66c2958)
    - [clone] decouple packet line from git-features and progress (13bf25e)
    - refactor (fb7dd26)
    - thanks clippy (what would I do without you <3) (631af04)
    - refactor (94f0d8a)
    - [clone] Keep line reader around in http transport (feb2596)
    - [clone] packet line readers now reset the parent automatically… (8250448)
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' (abf9c3b)
    - [clone] Finally it all works exactly as desired… (c5bbb57)
    - [clone] FAIL: can't pass line reader as box (633341d)
    - [clone] sketching how to possibly return Line readers while keeping it sane… (4ba123b)
    - [clone] Add Peek support for packet line reader (10f1ef7)
    - [clone] a simpler peek version that will soon work (c35051b)
    - [clone] FAIL: try to have peek_line() borrowcheck (dea5672)
    - refactor (f3c5c05)
    - packet line writer deals with long lines and definitely isn't smart (549e6e6)
    - First rough implementation of packet line writer (721c215)
    - Don't try to find 'ERR ' in every packet line we parse… (922fcb6)
    - thanks clippy (25cdbec)
    - no panics in packet line to let caller handle invariants; read… (a89a443)
    - [clone] as_read() support for packet lines (e214df5)
    - [clone] first stab at making packet liner reader more 'practical' (7178543)
    - [clone] prepare for making progress in packet line reader optional (ffe84c0)
</details>

### v0.2.0 (2020-09-12)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 58 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - (cargo-release) version 0.2.1 (ebf3419)
    - (cargo-release) version 0.4.1 (60ac8b0)
    - [clone] Don't hide nested pack-decoding information (4d4be97)
    - (cargo-release) version 0.6.0 (9ef184e)
    - refactor (ad17bfd)
    - [clone] Don't try to explicitly close the connection… (17200b3)
    - (cargo-release) version 0.1.1 (bb38c6b)
    - refactor (91d9f78)
    - [clone] Fix encoding of V1 capabilities in first want (b68a5c5)
</details>

### v0.1.0 (2020-08-18)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b879)
    - [clone] move packet-line code into own crate (879af67)
</details>

### v0.10.0 (2021-08-27)

#### Breaking

* **renames / moves**
    - `immutable::PacketLine` -> `PacketLineRef`
    - `immutable::Error` -> `ErrorRef`
    - `immutable::Text` -> `TextRef`
    - `immutable::Band` -> `BandRef`
    - `immutable::DecodeBandError` -> `decode::band::Error`
    - `pub immutable::` -> `line::`
    - `pub write::` -> `write::`

* **removals**
   - `write::Writer` (is now only `Writer`)
   - `read::StreamingPeekableIter` (is now only `StreamingPeekableIter`)
#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.0 (0899338)
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
</details>

