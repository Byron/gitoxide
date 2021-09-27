# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### Unreleased

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 17 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 242 commits contributed to the release over the course of 13 calendar days.
 - 54 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - prepare test for basic merging… (0a14ced)
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

### v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.11.0 (c815bbe)
    - Bump git-pack v0.11.0 (5ae6ff5)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] generic write_object() (c569f83)
</details>

### v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`



#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 86 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.10.0 (b995441)
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
</details>

### v0.9.0 (2021-08-27)

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
#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 185 commits contributed to the release over the course of 5 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [object #177] fix docs (07be661)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - [pack #172] A note about empty packs in Bundle writer (09a777f)
    - [ref #175] follow (try_)find(_what) naming convention (679895c)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - Fix formatting of performance-tasks.md (917967e)
    - Merge branch 'Byron:main' into main (dc58eca)
    - Release git-actor v0.4.0 (16358c9)
    - Allow creation of empty indices (d122fc7)
    - Release git-testtools v0.5.0 (574ede9)
    - [actor #173] fix docs (2d7956a)
    - Release git-testtools v0.5.0 (86e0a92)
    - [ref #175] make 'mutable' module private (a80dbcf)
    - [actor #173] refactor (08a1849)
    - Upgrade to nom-7 (f0aa3e1)
    - Release git-actor v0.5.0 (a684b0f)
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
</details>

### v0.8.2 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.2 (39a3f71)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

### v0.8.1 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.1 (045eb09)
    - remove dev-dependency cycles by removing their version (c40faca)
</details>

### v0.8.0 (2021-08-12)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - (cargo-release) version 0.18.0 (b327590)
</details>

### v0.6.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (d704bca)
    - (cargo-release) version 0.6.0 (4b71e15)
    - (cargo-release) version 0.5.0 (e21142b)
    - (cargo-release) version 0.17.0 (c52a491)
</details>

### v0.5.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

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

### v0.3.1 (2021-08-10)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.16.1 (e10e55c)
    - (cargo-release) version 0.3.1 (8b24197)
    - break more dev-depedency cycles up to git-odb (7ee278b)
</details>

### v0.3.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 142 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

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
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - Remove unnecessary unsafe code (83e207a)
    - [ref] fix compile warning on windows (c328774)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Implement Parser::into_iter without extra allocation (aa79924)
    - dependency update (059fa33)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
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
    - (cargo-release) version 0.2.0 (3286e42)
    - refactor (a25a774)
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports (de2ba3c)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
    - [git-ref] the first failing test (7e802a0)
</details>

### v0.2.0 (2021-05-25)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

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

### v0.1.0 (2021-05-24)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-pack] prepare first release (4f9eb70)
    - [git-pack] the very first version… (8c06cdb)
</details>

