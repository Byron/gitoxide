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

 - 210 commits contributed to the release over the course of 13 calendar days.
 - 47 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on

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
    - thanks clippy (c55f909)
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

## v0.11.1 (2021-08-29)

- instruct docs.rs which features to use for more useful documentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

## v0.11.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 104 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
</details>

## v0.10.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.10.1 (dc74d19)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.10.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.10.0 (b944278)
    - Release git-packetline v0.9.0 (7ffbd60)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - bump transport version to 0.10 (f26a3d3)
    - (cargo-release) version 0.8.0 (ad6d7f9)
    - (cargo-release) version 0.6.0 (d704bca)
    - (cargo-release) version 0.7.0 (2ef3106)
    - (cargo-release) version 0.5.0 (c2f94a5)
    - (cargo-release) version 0.4.0 (d69d0ac)
    - [transport] A much better name for 'is_stateful()` (f15f1e8)
</details>

## v0.9.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 147 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Revert "[ref] break dev-dependency cycle" (436e89b)
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.16.0 (1231dbd)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e)
    - [transport] more convenient check for available capabilities (e9ed952)
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
    - Bump thiserror from 1.0.25 to 1.0.26 (9682590)
    - [transport] remove Transport::close()… (4268a9b)
    - [transport] implement Transport for &mut T: Transport as well (372fb81)
    - [transport] tests for extra parameters (fffd926)
    - [protocol] extra_parameters are forwarded from delegate to handshake (03e3db3)
    - [transport] allow setting a custom URL in git::Connection (f7437e0)
    - [transport] async transports support extra params (a0d6756)
    - [transport] extra_headers for http (6026dcc)
    - [transport] extra-parameters for the http protocol (d30bcf1)
    - [transport] git::Connection handles extra-parameters (961b6a4)
    - [transport]  File implementation doesn't need to inherit git::Connection's… (951b1e2)
    - [transport] unsupported protocol versions now abort the fetch operation (812aa3b)
    - [transport] flexible version of version support check doesn't actually work :D (2b220f0)
    - [transport] improve docs for `is_stateful()` (22f7e67)
    - Merge branch 'pubcap' (292f8ff)
    - (cargo-release) version 0.1.1 (e9cdc95)
    - Add missing docs (a6cbbde)
    - [actor] fix dependencies (3ff918e)
    - [git-transport]: make capabilities parsing public (2f3725e)
    - thanks clippy (6200ed9)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - refactor (2a406d6)
    - [async-client] frame for async connect (9ada080)
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
    - refactor (2eefe17)
    - refactor (14c9093)
    - [git-protocol] async capabilities and arguments abstractions (aa3eacb)
    - [git-transport] see how things can be moved to a different thread… (c271d32)
    - [git-transport] partial transfer to thread doesn't work in test… (4a6dfd4)
    - [git-transport] allow fetch processing to be offloading to another thread (a1302e0)
    - Revert "[git-transport] async-executor (Local) hangs…" (ec8bcd0)
    - [git-transport] async-executor (Local) hangs… (68ac51b)
    - Revert "[git-transport] attempt to mix 'blocking' but realize that now things need to be static." (e367753)
    - [git-transport] attempt to mix 'blocking' but realize that now things need to be static. (3d296fa)
    - [git-transport] V2 transport tests work on async (e04a1c9)
    - [git-transport] first V2 test (f9da975)
    - [git-transport] adapt extension trait in blocking code to match async version (95eee30)
    - [git-transport] extension trait working (28fbd28)
    - [git-transport] a first step towards getting the extension trait to compile (b692979)
    - [git-transport] no warnings when building without any choice of client (3dc568a)
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support (ee01c79)
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports (de2ba3c)
    - [git-transport] handshakeV1 tests run in async! (d1c0e35)
    - [git-transport] And a chance to have V1 working in async (2bf93fc)
    - [git-transport] refactor (64bb8b3)
    - [git-transport] improve error handling considerably… (7b7d314)
    - [git-transport] Add remaninig git connection method… (73fcf38)
    - [git-transport] refactor (db83600)
    - [git-transport] the first part of async transport for git connections (d94fbf8)
    - [git-transport] Split git connection into shared and blocking parts (0bfe693)
    - [git-transport] refactor (2342e8a)
    - [git-transport] refactor (957403e)
    - [git-transport] refactor (e580354)
    - [git-transport] re-enable `request()` method of main trait… (3adbade)
    - [git-transport] RequestWriter complete (a05fff3)
    - [git-transport] refactor (03a3aed)
    - [git-transport] ARGH: PIN!!! (71379ac)
    - [git-transport] naive attempt to make Request async… (b819546)
    - [git-transport] ExtendedBufRead for Async… (d4e56c8)
    - [git-transport] First stab at ExtendedBufRead, but… (13f73d2)
    - [git-transport] put request writer into general spot… (af07ebf)
    - [git-transport] refactor (5f98ac1)
    - [git-transport] fix docs (fbfc827)
    - [git-transport] refactor (011ece0)
    - [git-transport] the first async trait (2abac2a)
    - [git-transport] refactor (73df129)
    - [git-transport] the first async-only type (88109a5)
    - [git-transport] all non-IO types are now shared (209c780)
    - [git-transport] feature toggle for async-client; prepare for test (95e6801)
    - [git-transport] refactor (592d9ac)
    - [git-transport] remove maybe_async from dependencies, add async-client feature (e57aad3)
    - (cargo-release) version 0.15.0 (d69d9fb)
    - [git-packetline] Use io::(Result|Error) everywhere (374f129)
    - [git-packetline] refactor (f038ca1)
    - [git-packetline] document feature toggle (8b8a1aa)
    - [git-packetline] refactor (1328c5b)
    - (cargo-release) version 0.6.0 (ec5a54e)
    - [git-packetline] refactor (e5769d1)
    - [git-packetline] refactor (fef3c9f)
    - (cargo-release) version 0.9.0 (18f6d01)
    - [git-transport] simplify parsing capabilities from lines (401af09)
    - refactor (8ce28e7)
    - [git-transport] test capabilities in blocking and async mode (66eb2a5)
    - refactor (558b208)
    - [git-transport] first round of getting capabilities into 'dual' mode… (3af353b)
    - [git-transport] remove default features to force being explicit everywhere (d1b39f8)
    - [git-transport] A first async test, right now there is nothing to test though (9741ae1)
    - Tests follow crate structure closely (again) (8d6e46a)
    - Make the blocking client the default… (9d62ca3)
    - Revert "Remove maybe-async for now" (ebd5701)
    - refactor (84d1509)
    - refactor (1412282)
    - refactor (f16d057)
    - refactor (976da51)
    - refactor (7ac6a05)
    - refactor (cd02749)
    - Remove maybe-async for now (97e96f4)
    - refactor (6e6f4ac)
    - refactor git-transport test in preparation for async testing (42d5bf7)
</details>

## v0.8.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [track publish] git-packetline/0.5.0 (422932a)
    - (cargo-release) version 0.8.0 (411a05e)
    - (cargo-release) version 0.5.0 (8c4cc3f)
    - [async-transport] Cargo.toml and traits to be more 'realistic' (9a617a5)
    - [async-transport] The very first step (b9e5559)
    - (cargo-release) version 0.14.0 (a760f8c)
    - (cargo-release) version 0.13.0 (ac2eddb)
</details>

## v0.7.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (334b7e1)
    - (cargo-release) version 0.12.0 (3b71e7e)
</details>

## v0.6.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 70 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (50fb6f2)
    - (cargo-release) version 0.3.0 (d5c6643)
    - thanks clippy (749ceba)
    - Merge pull request #50 from Byron/edward-shen/odb-zlib-ng (acb90d7)
    - Clear out non-gitoxide tasks from tasks.md (fb52a24)
    - Conform imports (fd73731)
    - [git-config] Fix must_use lints (71aff75)
    - Fix error type argument order and spell fields out (819568e)
    - Update tasks list with possible features for `dua`, `treediff` and google apis (f05037c)
    - [git-odb] Return error on invalid packs (88de64d)
    - dependency update (80d5cb6)
    - [git-odb] Fix Inflate::once (36f6bbd)
    - Update git-config information in README with planned features (1f34be9)
    - [git-odb] Remove unnecessary tests (ebe41ca)
    - [git-config] Update README.md (cb94dd7)
    - [gix] Use flate2 by default (f1158a1)
    - Slim down git-config with cargo-diet (1c555e0)
    - [gix] Add optional zlib feature (f1f9665)
    - (cargo-release) version 0.11.0 (1aa1f5e)
    - (cargo-release) version 0.2.0 (0c39373)
    - support for radicle urls (2c5b955)
</details>

## v0.5.1 (2021-01-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 (0cf1d06)
    - silence so far unknown clippy lints (b5f2a4b)
    - thanks clippy (343ab9a)
    - complete git-transport docs (fa2dc9d)
    - documentation for capabilities in git-transport (5ec79fa)
    - more docs for git-transport (3a867e9)
    - more git-transport docs (6cd69b9)
</details>

## v0.5.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9)
    - use git-hash in git-features (5b307e0)
</details>

## v0.4.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (32aefc0)
    - (cargo-release) version 0.4.0 (72eaece)
    - (cargo-release) version 0.9.0 (a89fdb9)
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (d19ee35)
    - (cargo-release) version 0.3.0 (eade7d1)
    - thanks clippy (ba9b3c2)
    - uograde everything else (0cd79d0)
    - (cargo-release) version 0.8.0 (47c00c2)
    - refactor (b3a8bb5)
    - refactor (f9e8d29)
    - cargo clippy Rust 1.48 (475a68c)
    - (cargo-release) version 0.7.0 (7fa7bae)
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
    - refactor (7c3c80a)
    - (cargo-release) version 0.4.3 (5b47a1a)
    - (cargo-release) version 0.4.0 (0d7b60e)
    - refactor (8930610)
    - Enforce using the correct version of clap (fd6457f)
</details>

## v0.2.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 217 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 14 times to make code idiomatic. 

### Commit Details

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
    - [clone] reassure ourselves that ERR lines are handled, always (925370b)
    - [clone] Response parsing up to (optional) pack (24064c7)
    - [clone] properly handle V2 response parsing (0d7d768)
    - [ref-ls] A way to abort on multiple delimiters; first tests work (8d44912)
    - refactor (feec5be)
    - [ref-ls] Allow multiple delimiters at the same time (cfae63a)
    - [ref-ls] It would be practical to simply have access to the line provider… (5fba787)
    - thanks clippy (27f30df)
    - [ref-ls] don't leak the PacketLine error type in Transport interface (58ddd29)
    - [ref-ls] support for line peeking in packet line readers (0c0c575)
    - [ref-ls] don't enforce V1 for local interactions (7b33336)
    - [ref-ls] don't do anything on drop (9f18d9b)
    - [ref-ls] Transport layer knows whether it's stateful or not (22c3640)
    - [ref-ls] Always send a flush before closing the connection (918f19f)
    - [ref-ls] git protocol now supports user expansion… (d88e9da)
    - refactor (e07fbd6)
    - refactor (7b5ce69)
    - [ref-ls] allow ssh to work with tildes in paths (301ae81)
    - [ref-ls] first stab at leaving path resolution to upload pack (51dad09)
    - [ref-ls] verify also ssh works (1ef39ae)
    - [ref-ls] tune request to actually work in all cases, particularly for github (6bab2f3)
    - [ref-ls] Make credentials helper truly work (7f3c3a7)
    - [ref-ls] Finally fix http content encoding (and fixtures to go with it) (49b7ad9)
    - [ref-ls] This actually makes things work in real-life (24ebc59)
    - [ref-ls] provide blanket impl at least to be less specific (0223e7f)
    - [ref-ls] Make things compile (b6506a4)
    - refactor (b38290e)
    - refactor (202383a)
    - thanks clippy (b060f42)
    - [clone] support automatic downgrade to protocol version 1 (4cf3643)
    - [clone] transport provides desired protocol version (c39b645)
    - [clone] update README, improve delegate docs (dc7908f)
    - [clone] features for V1 fetch (5b24a55)
    - [clone] Support explicitly closing (v2) connections (41e4cb2)
    - refactor (dda62fc)
    - [clone] Prevent accidental leakage by transforming back to the 'right' type (2d469c6)
    - refactor (88ecda1)
    - [clone] support for git-credentials helper (a6546da)
    - [clone] make URL available in transport layer (6778447)
    - [clone] http basic auth support for all kinds of calls (572fb54)
    - [clone] first sketch of basic authentication support for http (c5b2d04)
    - [clone] sketch for identity handling (b23f470)
    - refactor (22cb37d)
    - [clone] Add (hardcoded) timeout when connecting via TCP (02c195b)
    - thanks clippy (712527f)
    - [clone] Finish implementing ssh access, which might even work (8b843f2)
    - [clone] Add support for SSH prefixes in otherwise local service invocations (a1db217)
    - [clone] once again, for SSH we need to delay calling the actual service (2c70275)
    - [clone] Support for the probably very unnkown VIRTUAL_HOST env var (36fe20c)
    - [clone] Allow connecting via the protocol (eb7be2b)
    - [clone] be sure to wait on the spawned child on drop to prevent resource depletion (768d7f2)
    - thanks clippy (2528c82)
    - [clone] implement file based protocol using git-<service> processes (be254a9)
    - [clone] add 'Process' mode for git connection (e38c7bf)
    - refactor (2ecb975)
    - [clone] first steps towards launching git-upload-pack while… (41f05f1)
    - [clone] Http fetch test for V2 (81618ae)
    - [clone] http test for ls-refs V2 (3ef1e47)
    - [clone] finish test for git-based V2 command invocation (9384f32)
    - [clone] support for V2 arguments (8d56e79)
    - refactor (f46c89d)
    - refactor (9ed859a)
    - [clone] Using a normal writer, we can't produce delimiter packets (1877b5f)
    - [clone] first sketch of extension trait to invoke V2 commands (90eed9d)
    - [clone] Finally, HTTP requests are properly handled, it all works out! (a6121d9)
    - [clone] Helper now integrates with Http trait, neat (b462bc7)
    - [clone] first sketch of 'HeaderAndBody' helper (226f096)
    - refactor (f2ff90d)
    - [clone] a way to change progress handling on the fly (c1bcc0a)
    - [clone] first steps towards more flexible sideband switching (3d959e6)
    - [clone] Issue: shoehorning header handling into the body reader (4c304f1)
    - thanks clippy (bdcaf36)
    - [clone] Now we get to the point where uploads start, but… (8bd6182)
    - [clone] first steps towards testing posting via http… (b6a7e75)
    - refactor (a810f9f)
    - refactor (5c2bd5f)
    - [clone] make on-drop messages do the right thing (5a39d70)
    - [clone] first test for request - ideally we manage to add a lifetime to the closure box… (db1a5b8)
    - thanks clippy (913e55d)
    - refactor (de22323)
    - refactor (bad8361)
    - refactor (466557c)
    - [clone] on-demand line writer, it's cheap (8ddd0fa)
    - [clone] it shows that the packetline writer is better to be owned (f2c6e9f)
    - refactor (aceaaed)
    - refactor (2cdda7a)
    - refactor (521516f)
    - refactor (3738897)
    - refactor (2e68315)
    - [clone] first sketch of http request (8b4befb)
    - refactor (23af7e1)
    - [clone] support writing multiple messages on drop for the 'clone' case (9266442)
    - thanks clippy (2ed10de)
    - [clone] Sketch 'request()' implementation for git protocol (fd0e0e9)
    - [clone] Allow progress callback to know if it's an error line (0c41844)
    - [clone] sketch for generic request/response pattern suitable for clone/fetch (e0fd5a6)
    - thanks clippy (what would I do without you <3) (631af04)
    - [clone] Capabilities now can have multiple values (per command) for V2 (44dcea6)
    - [clone] First step towards http V2 handshake shows capabilities are… (f58a785)
    - [clone] remaining handshake V2 assertions (1a58955)
    - [clone] first sketch of git handshake, V2 (bf1f05b)
    - [clone] git protocol sends in packet line format, which is now enforced (4ce5916)
    - refactor (44b06a7)
    - thanks clippy (ee5abfc)
    - [clone] Configure http timeouts, just so that it is done (070855a)
    - refactor (8b1dc48)
    - [clone] Allow differentiating HTTP permission errors (4c9c413)
    - [clone] abort early on HTTP status errors (e829c0a)
    - refactor (791c05e)
    - [clone] more http test validations (e697b8c)
    - Revert "[clone] FAIL: try to communicate error codes after request" (350de7c)
    - [clone] FAIL: try to communicate error codes after request (2501ddd)
    - [clone] Check for 'smart' protcols (2960645)
    - [clone] validate expected http service announcement (a224a2c)
    - [clone] Keep line reader around in http transport (feb2596)
    - thanks clippy (I really tried) (e8880fb)
    - [clone] unbelievable, but it now seems to work as expected (88dbbf5)
    - [clone] quick hack to finish http set service, but something is seriously wrong… (dd93504)
    - [clone] non-deterministic behaviour when parsing HTML, despite ignoring the encoding (bab3ec3)
    - [clone] It definitely doesn't want to read the data to the end with 'chunked' (49f1aca)
    - [clone] for good measure: configure posts (more) correctly (e491e58)
    - [clone] Disabling transfer decoding makes it better, but… (3a1b8bc)
    - [clone] It looks like curl is stumbling over the 'chunked' header (279a386)
    - [clone] Fix deadlock - classic, and obvious (72a165e)
    - [clone] possibly correct impl of Handler; still hangs though :D (aefd8d4)
    - [clone] Fair enough - it locks up somewhere, let's see :D (33a1a22)
    - [clone] Improve usability of posts… (e1b944e)
    - [clone] Actually use that abstraction (d0bdbe4)
    - [clone] generalization of get and post (e62adc9)
    - [clone] Curl can now use remote to perform operations (get only for now) (a82f028)
    - [clone] try sending curl error even harder… (b450bfc)
    - [clone] first sketch of remote-curl, a way to transform curl into Read/Write (22b4b39)
    - [clone] Send headers with BufReaders (6a95aaa)
    - refactor (d427671)
    - [clone] Fixed shortcomings of http error handling, with thiserror (424e159)
    - [clone] Allow to use specific HttpErrors, at the expense of source (b16a8c5)
    - [clone] Fix 'small' compile (without http) (29ca5e8)
    - [clone] First step towards 'remote http executor' (f1e48d7)
    - [clone] things get more complicated…take a step back maybe? (f778637)
    - [clone] Right before actually performing the http call… (5bf9e6a)
    - [clone] better user agent header (4396587)
    - [clone] in small steps to getting the http 'interface' right (43f2a92)
    - [clone] A utility to respond with mock replies on a socket (1bf7ef7)
    - [clone] improvements to Http trait; prep for curl tests (9f69d6a)
    - [clone] a piped iterator (5148c85)
    - thanks clippy (c4f570f)
    - [clone] frame for implementing 'pipe' support (c555681)
    - refactor (bfda633)
    - [clone] sketch for http infrastructure to get going with curl (8351299)
    - [clone] an easy way to get a few HTTP replies for consumption by the client (8b082d0)
    - refactor (0bbd87e)
    - refactor (bbce340)
    - thanks clippy (73a6868)
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' (abf9c3b)
    - [clone] Finally it all works exactly as desired… (c5bbb57)
    - [clone] Most of the V1 handshake works, but… (318024b)
    - [clone] YES! Boxes with dyn traits and lifetimes… (5e35d0a)
    - [clone] FAIL: Right, need a box after all (6e57927)
    - [clone] FAIL: can't pass line reader as box (633341d)
    - [clone] sketching how to possibly return Line readers while keeping it sane… (4ba123b)
    - thanks clippy (81c0185)
    - refactor (f8ff1c7)
    - [clone] capability parsing (5b019af)
    - refactor (2b40961)
    - [clone] a little closer to handling the client handshake (1a4f84d)
    - [clone] first frame for testing transport layer interactions (e1100c8)
    - refactor (f3c5c05)
    - bump git-features to 0.4 to allow publishes after breaking changes (9d6b879)
    - [clone] move packet-line code into own crate (879af67)
    - [clone] http protocol is now optional (06c0816)
    - [clone] (very) First stab at http protocol connection (218a5eb)
    - [clone] Better error handling for generalized `connect(…)` (713808c)
    - [clone] fix git-transport crate size (720f444)
    - [clone] enable git-transport tests (8e07be4)
    - refactor (104b7fe)
    - thanks clippy (c62bfa2)
    - [clone] expand-path should be server-side (8a38856)
    - [clone] the return of actually parsing remote progress (c465fde)
    - [clone] move packet-lint into transport layer (c0dd831)
    - [clone] sample on how SSH connection fits in (a562059)
    - [clone] first sketch of transport layer's connection logic (f10cee5)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
</details>

## v0.2.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 55 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

## v0.0.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Cargo-diet for the top-level crate (19e7fec)
    - add missing license description (2b80181)
    - Make crates publishable (5688a34)
    - \#[forbid(unsafe)] for all crates (afda803)
    - cleanup - don't build and run tests while there is nothing to test (4a153da)
    - prepare git-transport just so that we don't forget to take the name (2c3ad7d)
</details>

