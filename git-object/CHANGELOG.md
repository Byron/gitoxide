# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 13 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 150 commits contributed to the release over the course of 10 calendar days.
 - 25 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - remove old and unnecessary experiment (aba8e56)
    - path::is (1f4e45a)
    - rename path::is_git to path::is (ac3b9ef)
    - path::discover (1958e8a)
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
</details>

## v0.14.0 (2021-09-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.14.0 (fd79b2e)
    - [object #164] refactor (883343b)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7)
    - [repository #164] generic write_object() (c569f83)
    - thanks clippy (33a8fb3)
    - [object #164] Allow referenced objects to be serialized as well (a98d298)
</details>

## v0.13.1 (2021-09-07)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 94 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.13.1 (2c55ea7)
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
    - Release git-pack v0.9.0 (355d6c4)
    - [repository #190] refactor (e751688)
    - Release git-traverse v0.8.0 (40c8506)
    - [ref #190] refactor (49fe1dc)
    - Release git-features v0.16.3 (342475f)
    - thanks clippy (023dedc)
    - Release git-diff v0.9.0 (021318f)
    - [ref #190] reverse reflog ergonomics (2de86f9)
</details>

## v0.13.0 (2021-08-27)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 223 commits contributed to the release over the course of 8 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#163**
    - Adjust collaboration guidelines to allow and even require PRs (998ae6b)
 * **Uncategorized**
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
</details>

## v0.12.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.2 (6e58edd)
    - [object] argh, remove these tests for now no time for this (13d627d)
    - [object] simply exclude the feature from testing for now… (adba3b9)
    - [object] fix magically smaller object size expectation (bf4d2d7)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.12.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.1 (086baa2)
    - remove dev-dependency cycles by removing their version (c40faca)
</details>

## v0.12.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-object v0.12.0 (9c7a99a)
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
</details>

## v0.11.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.5.0 (bf15c2a)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
</details>

## v0.10.0 (2021-08-10)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 82 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - fix release order to match actual dependencies (65ff8c1)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.4.0 (0d5c8b9)
    - (cargo-release) version 0.2.0 (8ff5115)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref] fix build (bad find&replace) (467395f)
    - [ref] refactor (e26c72f)
    - [ref] basic lookup rule impl; needs more test cases (3226f77)
    - Remove unnecessary unsafe code (83e207a)
    - [ref] fix compile warning on windows (c328774)
    - Merge branch 'parser-into-iter-without-alloc' (a799ca8)
    - [ref] a test case specifically for lookup rules (ab3a34f)
    - Implement Parser::into_iter without extra allocation (aa79924)
    - dependency update (059fa33)
    - [ref] improve parse failure handling in packed-ref lookup (ba62aab)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - [ref] refactor (207a799)
    - [ref] flexible and simple support for different hash lengths (9c2edd5)
    - thanks clippy (c437304)
    - [object] Add feature toggle for verbose errors… (4b63d8a)
    - [object] support for verbose errors for object parsing (8156f10)
    - [object] refactor (6f63983)
    - [object] Generalize nom error handling and use nom-errors instead of custom ones (47c8a97)
    - [object] remove unused dependencies (2f01e46)
    - [object] cleanup parsing error handling by removing NomDetail (e91cb40)
    - [object] refactor (1ddb5c0)
    - [object] replace custom context impl with the one by nom (9a6692d)
    - [object] refactor (8205429)
    - [actor] git-object uses git-actor (d01dd2f)
    - [actor] make signature parsing public, exposing nom :/ (a627972)
    - [refs] try to get structure in place for reflog parsing (727c66a)
    - thanks clippy (6200ed9)
    - (cargo-release) version 0.3.0 (87db688)
    - (cargo-release) version 0.3.0 (6b33678)
    - (cargo-release) version 0.2.0 (3286e42)
    - (cargo-release) version 0.4.0 (866f86f)
    - (cargo-release) version 0.2.0 (1327894)
    - [git-object] use git-validate crate (4ba98e8)
    - [git-object] refactor (d64d326)
    - [git-ref] the first failing test (7e802a0)
    - Switch to latest nom (859e57e)
    - [git-ref] clear it out and move existing functionality to git-object (fa548ce)
    - (cargo-release) version 0.5.0 (b6b5856)
    - [pack-gen] refactor (61554e2)
    - [pack-gen] tag support for tree traversal (28ed260)
    - (cargo-release) version 0.10.0 (5d7ee6a)
    - [pack-gen] more tests for Tag iterator (b69d6d6)
    - [pack-gen] the first green test for Tag iterators (df5ef8a)
</details>

## v0.9.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#79**
    - refactor; add test for empty tree iteration (6340296)
 * **Uncategorized**
    - [release-automation] no confirm mode for publishes (4471888)
    - (cargo-release) version 0.9.0 (84897fd)
    - Merge branch 'patch-1' (5edc076)
    - refactor (a9e4feb)
    - Allow empty trees when parsing them at once, fixes #79 (d34fd19)
    - Fix formatting (a341995)
    - Remove almost all unsafe code from Tree. (42b6033)
    - refactor (9870923)
    - [hours-demo] computation seems to work better now (26ecca2)
    - refactor (2d00c4e)
    - [hours-demo] Maybe the pinnacle of performance… (f70c61a)
    - remove debug-assert which doesn't hold - it's OK to have empty commit messages (13abc2d)
    - And it's a wrap for git-diff docs for now (9e09dd5)
    - [traversal] first impl based on git-odb::traver (76a3017)
    - a new crate: git-traverse (1a9af50)
</details>

## v0.8.0 (2021-04-30)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (a1ce210)
    - (cargo-release) version 0.3.0 (e9665c7)
    - [traversal] add CommitIter::tree_id() convenience method (6affd9d)
    - [traversal] trying to get things done with gitoxide shows some teeth… (3fee661)
    - refactor; better iter error handling tests (9fe139b)
    - [tree-diff] more tests for the tree iterator (91b5a02)
    - test error handling of commit iteration (fcec4b4)
    - thanks clippy (41418ed)
    - fix serde support for commit iter token (3bfcb49)
    - [tree-diff] all the tests for commit iter (7ebea87)
    - [tree-diff] more tests (4f81450)
    - [tree-diff] And there is a working commit iterator, needs more tests (d991847)
    - [tree-diff] A complete nearly working impl of a Commit iterator (4711821)
    - Frame for Commit iterator (796b74a)
    - first failing test for commit iterator; store two parents without alloc (8337514)
    - [tree-diff] one more test green + refactor (bc5549d)
    - [tree-diff] refactor into iterator based model (29b527a)
    - [tree-diff] The least intrusive way to allow dealing with tree iterators (d41dd3c)
    - [tree-diff] prototype an immutable tree iterator to avoid entry allocs (f38e5cd)
    - [tree-diff] A step closer to handling additions in a directory (a11f210)
    - refactor (a4d5f99)
    - refactor (633cba7)
    - First sketch of diff API (fc3f2b7)
    - Better ergonomics for accessing decoded objects (ae3eab6)
    - thanks clippy (8295548)
    - refactor (9d03843)
    - fix debug assert, thanks gitpython (fe954b9)
    - More explicit expectations towards entries in mutable Trees (d94f84c)
    - refactor (f19ea33)
    - An even better name for decode errors (f270850)
    - Make clear it's a decode error we are using there (f45cb4b)
    - rename git-object::(owned->mutable)|(borrowed|immutable) #(67) (91ee558)
    - The first basic traversal utility #(67) (ea6610b)
</details>

## v0.7.0 (2021-04-08)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 112 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#63**
    - Use new `oid` where possible in git-odb (68a709e)
    - refactor; better errors for invalid hash sizes (be84b36)
    - Make ObjectId/oid happen! (ca78d15)
    - Remove all public exports of git-hash types in git-object (accf89d)
    - Remove re-export of git_object::borrowed::Id (a3f2816)
    - Move git-hash::owned::Id into git-hash::Id (fdbe704)
    - Rename `git_hash::*::Digest` to `Id` (188d90a)
 * **Uncategorized**
    - (cargo-release) version 0.7.0 (b900914)
    - (cargo-release) version 0.2.0 (4ec09f4)
    - thanks clippy (cefbf3e)
    - upgrade depdendencies (e4a7711)
    - improved high-level docs for git-object (60036f2)
    - Add missing '.' at end of doc comments (7136854)
</details>

## v0.6.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 (4224c5b)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - first round of git-object doc proof reading (524ce51)
</details>

## v0.5.0 (2020-12-15)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 84 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (fc7d600)
    - `deny(missing_docs)` for git-object (8525684)
    - more docs for owned git-object (b79101d)
    - a few more comments in git-object (171d269)
    - thanks clippy (ba9b3c2)
    - refactor (d5d7cf9)
    - more git-object docs (ba595f6)
    - more docs of git-object::owned (0620dce)
    - docs for git-object::borrowed (68e524d)
    - docs for git-object::borrowed::commit (c5c1df0)
    - Merge branch 'commit-graph' into main (9cb09b2)
    - the daily commit (single handedly) (b528c2e)
    - Note about why git_features::hash::bytes_of_file() is not yet used (ca48fc4)
    - dependency update (988f905)
    - specify the hash to create with 'hash::bytes_of_file' (c000294)
    - document `loose::Object` entirely (d5eef9c)
    - move 'git_odb::hash::bytes_of_file' into git_features::hash (c5f6b45)
    - thanks clippy (b9e0a87)
    - Add and use borrowed::Id::null_sha1() (c717492)
    - Updated `expect` message (e8d8d93)
    - Update error message for type name (92cbb13)
    - Document borrowed odb objects (7626f7f)
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
    - refactor (ba1d883)
    - take not of a few more obscure features (8f9570c)
    - (cargo-release) version 0.4.0 (2272fa4)
    - refactor (7c3c80a)
    - (cargo-release) version 0.4.3 (5b47a1a)
</details>

## v0.4.0 (2020-09-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 97 commits contributed to the release over the course of 29 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
    - [clone] proper parsing of V1 refs (d262307)
    - [clone] Don't expose hex-error in public interfaces anymore (92dab30)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
    - refactor (a0bebd1)
</details>

## v0.3.0 (2020-08-12)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 72 commits contributed to the release over the course of 31 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update release script to match dependency order (e8df6c1)
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
    - thanks clippy (62d2ff3)
    - organize object type comparisons by probability… (19a5d94)
    - don't cause re-allocs of the compression buffer (2bb6fd2)
    - Reduce memory consumption (6d1a7a1)
    - Also read the pack trailer during iteration (98a8e17)
    - refactor; better tests (12d14bf)
    - first step towards putting the index file into position (d994c74)
    - Improve looks of documentation (11a32eb)
    - Finish Sink implementation (84f7908)
    - Introduce hash kind, as this should be specified when writing an object (f5d0acf)
    - (cargo-release) version 0.2.0 (76fe0ab)
    - (cargo-release) version 0.2.0 (d350a13)
    - beautifully implement shared extra-header access (920d1ac)
    - roundtrip Rust repo in stress test; accept more diverse trees when parsing (0347cdb)
    - Make sure we write out trailing newlines properly in multi-line headers! (7f044c3)
    - Consume PGP signature in tags fully (ffd6c31)
    - Support for very special tree entry mode… (2be2c9d)
    - make tagger signature optional (3358f9a)
    - remove now unused pgp_signature field - it's in extra-headers (c8c937c)
    - proper support for extra-headers (d0feb2b)
    - Abiility to read mergetags (for now only these) as extra-headers (bd3a2db)
    - Switch to latest quick-error (9760856)
    - Fully implement --encode and --re-encode flags (a7cfac8)
    - empty trees are allowed, and they are special, too (6bed200)
    - refactor (56b66ac)
    - Basic top-level object round-tripping (e851cbe)
    - refactor (ec5e50f)
    - implement blob (f30caf4)
    - refactor (335e98a)
    - tree roundtrip (8b26a0e)
    - prepare for writing out owned trees (2b6eced)
    - manual deserialize implementation, for now (9f46efd)
    - Use borrowed::Id in trees for full type safety (5d57c1f)
    - refactor (f7b8826)
    - commit round-tripping works with multi-line signatures (b692b0a)
    - first attempts to roundtrip signatures shows I parse it wrongly :D (1b48367)
    - Prepare for allowing an owned, processed version of multi-line headers (f966e7f)
    - first attempt to round-trip multi-line headers (645ef94)
    - single-line header support (478c09e)
    - The first basic version of commit serialization (5319f64)
    - make reusing round-trip code easier (3b9d66c)
    - refactor (987787e)
    - Fix tests on windows, by ignoring them (512ed6c)
    - Use borrowed::Id everywhere (9f876f0)
    - move git_object::Id into git_object::owned::Id - much better already! (50c7136)
    - basic integration of borrowed Id; translate between owned and borrowed (84ff638)
    - prepare to allow Id be owned and borrwed; abstract over hash type (d883c31)
    - introduce the notion of IdRef (7007361)
    - Use statically known borrowed arrays for perfect type safety! (3ead048)
    - refactor (766f3e4)
    - tags can write signatures (a48275e)
    - tags can write a message properly (b590b77)
    - green tests as basic tags can now be serialied (62a02b4)
    - more tests for signature serialization (5000f30)
    - time serialization (1eb1e36)
    - prepare writing of time as part of signature (f560bc5)
    - add new 'git-ref' crate; place ref name validation code there (1a0e84e)
    - refactor (b4392e8)
    - some more boilerplate to actually implement complete ref name checking (087857a)
    - very basic first steps of validated serialization (d3fd5ff)
    - it's probably OK to consume the borrowed objects when converting them to owned (101ddd5)
    - try basics of roundtrip without consuming the source object (581794e)
    - refactor (bca1f16)
    - first sketch of owned Tag in preparation for round-tripping (fa2745a)
    - refactor (90ae25d)
    - refactor (256581b)
    - 'data -> 'a as it's shorter and also more idiomatic (71821e9)
    - refactor (dedd4dc)
    - apply cargo-diet (better late than never :D) (295fc81)
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release over the course of 26 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable (5688a34)
    - Handle windows newlines in test suite for packs as well. (ebd5176)
    - Fixup text file tests on windows (2288088)
    - Add metadata to allow docs.rs build all featueres (10f9386)
    - git-odb with serde support (0da930c)
    - cut back on onnecessary annnotations: serde(borrow) (759915c)
    - serde support for all git-object types, incl. test (1ae8f9c)
    - learn from the best: with-serde -> serde1 (d651c21)
    - commit to using bstr whenever something is not data bytes; remove miniserde (3183d1b)
    - Prepare centralization of bstr as optional component (aa857d9)
    - add support for miniserde (f806647)
    - first gentle test of adding serde support selectively. (78d9bc0)
    - Allow for more screen space when formatting (6794300)
    - Pack offset by index (69e35b1)
    - test V1 lookup (e9c7127)
    - validate sha1 of pack objects, some work, some don't for some reason… (aa8799a)
    - Capability to write loose object headers, fast (de0aeff)
    - simplify folder names (36fde1f)
    - fix clippy (a9c5da7)
    - more convenient access to our four object types (ecda6d2)
    - even better trait derives (e78f9f6)
    - Better trait support for basic types (6617386)
    - Memory size checks for objects (ab51616)
    - Make single-field objects blob and tree more explicit (1aef68f)
    - add Blob type to parsed objects (d3e8e4b)
    - fix imports (10f2967)
    - try pub use with rename. Not bad in the docs, but maybe a bit confusing (526f3f8)
    - refactor (2ffd7fa)
    - refacto (ffc0089)
    - refactor (b9a1647)
    - test for parsing trees from loose dbs (4f48249)
    - refactor (9f9ccad)
    - Move git-object tests to top-level for separation and cleanness (df42a01)
    - Prefer integration level tests, but use unit-tests where appropriate (ec3be19)
    - run previously unused method of Tree (0d159c2)
    - Actually use the Tree object (635e735)
    - handle commits without newlines; make tag newlines optional (c0b54be)
    - Handle tags without newline; document fixture processing step (344a562)
    - Don't assume newlines in trees anymore (45d7c36)
    - Found huge issue with newlines polluting fixtures. (f182d22)
    - first tree implementation, which seems to work well (9694fcb)
    - boilerplate for tree parsing (48c4c07)
    - refactor (d48cafa)
    - Add conversion traits for Object<->Tag|Commit (7dcbd5d)
    - Make Commit available in borrowed object (b2d1b5d)
    - Use smallvec to save memory in the common case (single parent) (263835b)
    - more tests (56248fe)
    - Now gpg-signature parsing works correctly - thanks to peek(…) (7078dac)
    - first somewhat working version of single/multi-line signature parsing (dab5c65)
    - support single-line gpg signatures (71330b5)
    - support for commit encoding field (40bffe9)
    - more commit tests, next up: encoding (ca4d3aa)
    - first successful parsing of commit (b44765a)
    - parse BStr versions of hex-shas directly (e3a2b77)
    - parse parents (696e0a3)
    - Use BStr instead of Id to avoid parsing into something we might not use/need (7c97471)
    - factor out hex sha parsing (d650dd2)
    - refactor (0104f4c)
    - first stab at factoring header parsing into sub-parser (6f6ee8f)
    - first fixtures for commit parsing (551f2d1)
    - avoid unnecessary allocation when creating SHA1 paths in loose ODB (09d8d3a)
    - document existing use of unsafe, deny everywhere else (41f4bce)
    - cleanup integer parsing in loose object database (ecdce1a)
    - Add remaining tag tests, along with some fixes (06e22fb)
    - use bstr were possible (01dd4e2)
    - the defining property is actually that the object is borrowing data (e0125fd)
    - refactor (683360a)
    - move all tests into the top-level for nicer names basically :D (598901a)
    - refactor (0f01e9f)
    - refactor (87bbea4)
    - refactor; add more signature parsing tests (ba9c7de)
    - cleanup; all tests work! (7c96603)
    - fix whitespace (ebaaa00)
    - first version of tag message parsing - it's actually changed now (74b2328)
    - implement parse_signature with nom, starting to like it (ebdf205)
    - First part of parsing tagger signatures (5b43270)
    - generalize with Boxed error cause (824cd2c)
    - first seemingly awkward way of not discarding too much error information… (6f9a636)
    - refactor (fb287af)
    - the first sketch of parsing a tag with Nom and half-decent errors (4498dff)
    - Use git-object in git-odb (07f7c31)
    - Move all object related code into own crate… (605ef20)
</details>

