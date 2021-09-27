# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking

- Change return value of `prelude::RepositoryAccessExt::committer()` from `git_actor::Signature` to `Result<git_actor::Signature, easy::borrow:repo::Error>`
- Change return value of `prelude::ReferenceAccessExt` from `Result<Vec<RefEdit>>, _>` to `Result<easy::Reference, _>`.
- Rename `State` structs that serve as platform for iterators or other dependent types into `Platform`. These are usually intermediate objects only.
- Rename `easy::Reference::log()` into `easy::Reference::logs()`

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 206 commits contributed to the release over the course of 11 calendar days.
 - 49 commits where understood as [conventional](https://www.conventionalcommits.org).
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
</details>

## v0.9.1 (2021-09-10)

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 44 commits contributed to the release.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
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
 * **Uncategorized**
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

## v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport
- 
### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.9.0 (e8594de)
    - Bump git-pack v0.11.0 (5ae6ff5)
    - Bump git-repository v0.9.0 (b797fc1)
    - [repository #193] Add feature flags for async/blocking (57f482c)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7)
    - [repository #164] Support for refreshing the object database (46e10f8)
    - [odb #164] Add refresh() functionality (ee16d04)
</details>

## v0.8.2 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.2 (3fc23be)
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
</details>

## v0.7.2 (2021-08-17)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.2 (c5791b1)
    - [smart-release #162] separate mutable state more cleanly… (f00de95)
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… (65db010)
    - [smart-release #162] a promising lead, this might just work (0c4f77b)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e3340)
    - [smart-release #162] a barely working version of refs handling… (3e01025)
    - [smart-release #162] a sign - can't store references, but… (7862652)
    - Revert "[smart-release #162] FAIL try to use Rc<RefCell<_>>…" (58529a1)
    - [smart-release #162] FAIL try to use Rc<RefCell<_>>… (180be72)
    - [smart-release #162] refactor (8f558af)
    - thanks clippy (b63cd40)
    - [smart-release #162] refactor (35ff637)
    - [smart-release #162] First compiling version, non-threadsafe… (d2b2ce9)
    - [smart-release #162] FAIL: RefCell as self param also doesn't work :D… (ec0c863)
    - [smart-release #162] back to a more humble, hard-coded approach… (bdceb7c)
    - Revert "[smart-release #162] FAIL: cannot use extension traits…" (2878a14)
    - [smart-release #162] FAIL: cannot use extension traits… (e115631)
    - [smart-release #162] FAIL: try to do things borrowck doesn't like… (853ae9c)
    - [smart-release #162] a sketch of an API that seems to satisfy the constraints… (bec8473)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.7.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.1 (4369697)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
    - (cargo-release) version 0.18.0 (b327590)
    - (cargo-release) version 0.6.0 (d704bca)
    - (cargo-release) version 0.6.0 (4b71e15)
    - (cargo-release) version 0.5.0 (e21142b)
    - (cargo-release) version 0.17.0 (c52a491)
    - (cargo-release) version 0.5.0 (c2f94a5)
    - (cargo-release) version 0.4.0 (d69d0ac)
    - (cargo-release) version 0.6.0 (d58f37e)
    - (cargo-release) version 0.5.0 (1687e59)
    - (cargo-release) version 0.4.0 (28e58f6)
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
</details>

## v0.7.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 63 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (1c5dfb8)
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - (cargo-release) version 0.5.0 (0e11e98)
    - (cargo-release) version 0.2.0 (8ff5115)
    - [repository #149] pre-emptively fix windows (b4d3934)
    - [repository #149] only canonicalize if absolutely required (d537fac)
    - [repository #149] canonicalize only when needed (57f42bd)
    - [repository #149] prepare for canonicalizing only when needed (cac9d70)
    - [repository #149] refactor (3c368ec)
    - [repository] Fix TreeExt trait name - it's actually for TreeIters (f8e0747)
    - Canonicalize path when discovering repositories (7bfaa14)
    - thanks clippy (e1964e4)
    - [ref] fix build (1dcc590)
    - [ref] refactor (e26c72f)
    - [ref] and it compiles again, may todos left (16618b9)
    - [ref] fix build (83002df)
    - [ref] rename find_one to 'find' in git-ref… (ae7746a)
    - [ref] refactor (758c090)
    - Revert "[ref] parameterize all uses of hash length…" (21f187e)
    - [ref] parameterize all uses of hash length… (5c7285e)
    - [ref] another deletion test succeeds (6037900)
    - [ref] file store can ignore all writes; sketch transaction API (52a81e9)
    - [actor] fix gix hours (b4e95fd)
    - (cargo-release) version 0.4.0 (4512798)
    - [lock] cleanup signal handling even more… (9fb13d2)
    - (cargo-release) version 0.3.0 (92f3a83)
    - (cargo-release) version 0.2.0 (7c2eb36)
    - fix docs (e68d460)
    - fix build (dbfa49a)
    - Remove mentions of interrupt handling feature toggles (833ac04)
    - Fix everything up so that… (5930563)
    - A first attempt to make intrerupt tools work, but… (8fb8d37)
    - First step towards moving git-features::interrupt… (8a741d0)
    - [pack] add --statistics flag to pack-create (51a3077)
    - [async-client] frame for async connect (9ada080)
    - Separate networking via feature toggles and pass that through in the main crate (2c749f1)
    - (cargo-release) version 0.3.0 (6b33678)
    - (cargo-release) version 0.2.0 (3286e42)
</details>

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare git-repository release (b9d7722)
    - (cargo-release) version 0.6.0 (d35c55d)
    - [git-repository] better docs (f60a7c5)
    - [git-repository] gitoxide-core uses more of git-repository (bb5b074)
    - [git-repository] replaces git-features and git-protocol in gitoxide-core (081d20f)
    - [git-repository] used by gix-hours (24e0258)
    - [git-repository] refactor (b5ebcfa)
    - [git-repository] now used by gixp-organize (aa91fad)
    - [git-repository] make it easy to get maximum performance in apps using this crate (dc150a5)
    - [git-repository] prevent other implementations of extension traits; refactor (e14df75)
    - [git-repository] finish 'diffing' program upgrade (7eea39a)
    - [git-repository] more details on how this crate is intended (cd85050)
    - [git-repository] refactor (b9f4d25)
    - [git-repository] try out an API for ancestor iteration (de0b5bb)
    - [git-repository] the first extension trait for more convenience (63a1fee)
    - [git-repository] now with a prelude for traits (7f7a5ea)
    - [git-repository] more re-exports for convenience (6a5c00e)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
    - [git-repository] repo-init sketch (5855c95)
    - [git-repository] refactor (63c22af)
    - [git-repository] refactor (996944a)
    - [git-repository] refactor (a2d58c1)
    - [git-repository] a sketch of how the repository could look like (3854cef)
    - [git-repository] traversal uses git-repository (db564c5)
    - [git-repository] an actual repository abstraction (3f20b26)
    - [git-repository] refactor (c8323e4)
    - [git-repository] traversal program uses new facilities, and it's cumbersome (29ea2de)
    - [git-repository] bare repository handling (3a8e6ff)
    - [git-repository] tests pass, bare repo tests missing (a5ed9ea)
    - [git-repository] most of the git repository discovery (72a49c8)
    - [git-repository] frame for repository testing; sketch of discovery API (467e340)
</details>

## v0.5.0 (2021-04-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 47 commits contributed to the release over the course of 196 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (02df134)
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
    - Spelling fix in error message (944d0f4)
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
</details>

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 99 commits contributed to the release over the course of 28 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (2b1bca8)
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
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1)
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
    - Switch to latest quick-error (9760856)
    - refactor (2888f1b)
    - explicitly include assets in git-repository crate (9da6071)
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add missing license description (2b80181)
    - Make crates publishable (5688a34)
    - Fix tests (59ed51d)
    - Use 'main' branches instead of the previous default when initializing a repository (da77cc8)
    - Allow for more screen space when formatting (6794300)
    - goodbye git-core, hello git-repository (7cec2b6)
</details>

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.1 (b269a12)
    - [#190] run tests faster (at the cost of compile time) (a22c95b)
    - [repository #164] make EasyArcExclusive available (2fa3dcb)
    - [#190] faster builds with debug=false and dependency caching (0b0fea4)
    - Release cargo-smart-release v0.3.0 (82f0cec)
    - [ref #190] Make References sortable (16b2232)
</details>

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 292 commits contributed to the release over the course of 10 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - Release git-testtools v0.5.0 (574ede9)
    - [ref #175] refactor (1243459)
    - [actor #173] fix docs (2d7956a)
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
    - [smart-release #162] unify 'ext' visibility (ca082a7)
    - [repository #165] refactor (0f13104)
    - thanks clippy (1f2d458)
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
    - [smart-release #162] replace reference peeling with git_easy (7cfd5f9)
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode (4fb672a)
    - [smart-release #162] refactor (ef623a6)
    - [smart-release #162] reduce visibility of Cache (397fbfe)
    - [smart-release #162] more granular cache control WORKS (25dce2a)
    - Revert "[smart-release #162] FAIL: definitely need better granularity" (499993f)
    - [smart-release #162] FAIL: definitely need better granularity (5f27871)
    - [smart-release #162] FAIL: promising at first, but not really working… (fa01f76)
</details>

