# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

<csr-id-3c0a6389fe5ff981dadca20e8a4a4a0d2ef66e13/>
<csr-id-77ed17c703e502e132cda9a94eb8c63db0b627ad/>

### New Features

 - <csr-id-ae8780e08303946412cedc19ea4d2679be49ec97/> smart-release with --changelog-without option…
   
   …to allow disabling various changelog segments like clippy, or commit
   statistics.
   
   Note that it's always possible to delete individual sections afterwards.
 - <csr-id-509550f8aa8210f3688c78167a56a21fc1817515/> changelog command learns the --without <section> option
   With it one can omit auto-generated sections of the given name.

### Fixed

 - <csr-id-11eebdcc572a72b2e66a9db3cae0a01f12a81619/> don't claim to change manifest version if it's the same one

### refactor (BREAKING)

 - <csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/> Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.
 - rename --skip-* flags to --no-* for consistency
 - rename --skip-dependencies to --no-dependencies…
  
   …to be more inline with existing terminology of other flags.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 200 commits contributed to the release over the course of 31 calendar days.
 - 14 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: #192, #197, #198, #200, #67

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 18 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#192**
    - smart-release: assure the current package version is actually breaking (fb750b6)
    - smart-release: better verbosity handling when comparing to crates-index (f6f2d1b)
    - smart-release(feat): turn off safety bump with its own flag (a040f7d)
    - smart-release(refactor): (443f000)
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
    - Also parse 'style' if there are breaking changes (bc9d85a)
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes (4eebaac)
    - Support writing whole bodies in conventional messages… (c1f3c9d)
    - Support for paragraphs in conventional items (7f52823)
    - respect release-wide ignore list to allow removing entire conventional headlines (145103d)
    - Only write headlines that we can parse back… (d44369a)
    - handle all possible changelog headlines and add roundtrip tests (fda5ccf)
    - First basic parsing of conventional user and generated messages (56cd4ac)
    - parsing of removed conventional messages from changelogs (c593252)
    - first basic merging of conventional messages… (9af3248)
    - Trivially emulate gits way of handling commit dates… (f58b30a)
    - Also consider changes of changelogs themselves… (8a2042c)
    - Adjust date of upcoming version as well (fab4649)
    - assure git-conventional is treated like user generated content for statistics (1fd5acb)
    - merge doesn't consider user generated sections, only the ones it would want to add (ebbebdd)
    - Quick and dirty writing of conventional messages… (adfbd0d)
    - basic generation of git-conventional information (77b0feb)
    - Sketch out data structure for git-conventional segments (2713c02)
    - refactor (bcdec5e)
    - smart-release with --changelog-without option… (ae8780e)
    - changelog command learns the --without <section> option (509550f)
    - Easy removal of statistical sections, by just removing them… (91efd68)
    - Rebuild all changelogs to assure properly ordered headlines (4a9a05f)
    - reorder headlines according to version ordering… (2ff0c20)
    - Sort all commits by time, descending… (f536bad)
    - greatly reduce changelog size now that the traversal fix is applied (a0bc98c)
    - Use most relevant parent tree for change comparison… (5b9dd14)
    - Use hashmap based lookup for trees… (48a0c76)
    - refactor and improve path filtering to find relevant commits… (01b2466)
    - The first headline level controls all the other ones (715ea54)
    - adapt to git-hash refactor (925d668)
    - Fixup remaining changelogs… (2f75db2)
    - Generate changelogs with details (e1861ca)
    - Only use short hashes for logs, without detecting ambiguity for now (772525c)
    - boost allowed package sizes… (1b21d71)
    - Stable smart-release journey tests… (fc79188)
    - Update all changelogs with details (58ab2ae)
    - Put commit details to the end of generated segments (054d207)
    - Use message commit id instead of body… (9b46f32)
    - fix md formatting on github (262c000)
    - create details headline based on log message (04bbcbb)
    - Add details behind a fold, but… (3360b2e)
    - Use the notion of 'changes after merge' only to drive previews… (634267c)
    - Update changelogs (c857d61)
    - refactor (7a83103)
    - Also provide a duration in days for preparing a release as part of statistics (bd12cac)
    - Fix tests (6c98afc)
    - refactor (65fa0a4)
    - More commit statistics (0840e7e)
    - Basic commit statistics with round-trip, more actual information to come (6d097ae)
    - refactor… (ce0dda2)
    - More robust parsing of read-only sections (a3954f4)
    - treat clippy as generated statistical section… (1cff425)
    - Add new section type and write it out: clippy (6fca2ac)
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
    - Actually integrated generated changelog with existing ones… (aa095e2)
    - inform about 'bat's  absence (c82c5bc)
    - rename --no-bat to --no-preview… (1087dd8)
    - basic merging now works (6c6c200)
    - sketch for finding insertion points and merging sections (2a49033)
    - Sketch merging logic… (1932e2c)
    - prepare test for basic merging… (0a14ced)
    - nicer 'thanks clippy' message (4344216)
    - Show with simple example how the round-tripping works, neat (9510d9b)
    - collect unknown text so things don't get lost entirely… (60040c9)
    - parse back what we write out, perfectly… (5cab315)
    - fix journey test (3006e59)
    - Write new changelogs with bat if available (cca8e52)
    - Use `cargo diet` to reduce package size (cc5709e)
    - Write markdown changelog to lock file (400046e)
    - refactor (b05ce15)
    - Basic serialization of ChangeLog (205b569)
    - support for generated headers (bcc4323)
    - refactor (1ebb736)
    - Use 'to_*' when converting `easy::Object` to specific object kind (1cb41f8)
    - transform history segments into changelog parts (348b05c)
    - layout structure for ChangeLog generation from history items (40e9075)
    - more general commit history (39522ec)
    - Invert meaning of changelog's --dependencies flag… (51eb8cb)
    - rename --skip-dependencies to --no-dependencies… (77ed17c)
    - Remove strong-weak typing for conventional type (b71c579)
    - Fix panic related to incorrect handling of character boundaries (9e92cff)
    - Parse message fully (and own it) to allow markdown generation (b8107e5)
    - tests for conventional and unconventional description parsing (faade3f)
    - Make use of fixed git-conventional (b7b92b6)
    - update git-conventional dependency (2d369e8)
    - first test and sketch for stripping of additional title values (55b7fe8)
    - Basic message parsing, either conventional or not, without additions (b3b6a2d)
    - Sketch Message fields from which change logs can be built (b167d39)
    - Fix build (d0a956f)
    - More message parsing tests, now with legit failure… (625be8d)
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
    - object-cache to allow for a speed boost… (06996e0)
    - smart-release: actually build the segment vec, without pruning for now (422701b)
    - smart-release: build commit history for later use in changelog generation (daec716)
    - smart-release: sketch history acquisition (debe009)
    - add 'Head::peeled()' method (56e39fa)
    - smart-release: some performance logging (1954b46)
    - smart-release: build ref lookup table (9062a47)
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - Add 'references().all().peeled().'… (6502412)
    - smart-release: filter refs correctly, but… (2b4a615)
    - smart-release: find tag references by name… (72e1752)
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
    - parse issue numbers from description and clean it up (95c0a51)
 * **#67**
    - split data::output::count::objects into files (8fe4612)
 * **Uncategorized**
    - thanks clippy (ce48e18)
    - thanks clippy (af9d137)
    - Update changelogs just for fun (21541b3)
    - thanks clippy (bf514a2)
    - thanks clippy (ead04f2)
    - thanks clippy (e4f1c09)
    - thanks clippy (b856da4)
    - thanks clippy (31498bb)
    - thanks clippy (c55f909)
    - thanks clippy (b200ee8)
    - thanks clippy (4b3407d)
    - thanks clippy (1dece2b)
    - thanks clippy (a89d08c)
    - Merge branch 'main' into changelog-generation (c956f33)
    - don't claim to change manifest version if it's the same one (11eebdc)
    - thanks clippy (68ea77d)
    - thanks clippy (7899ef1)
    - thanks clippy (2b55427)
    - thanks clippy (a554b9d)
    - Bump git-repository v0.10.0 (5a10dde)
    - thanks clippy (d15fded)
    - [repository #164] fix build (1db5542)
    - Release git-repository v0.9.1 (262c122)
    - [smart-release] auto-detect changes in production crates as well (24bc1bd)
    - [smart-release #195] update test output to match CI… (f864386)
    - [smart-release #195] better error for untracked files. (f5266f9)
    - [smart-release #195] assure dependent packages are not packages to be published (6792ebc)
    - [smart-release #195] refactor (f354b61)
    - [smart-release #195] refactor (968b6e1)
    - [smart-release #195] don't tout changes that aren't really there… (5931012)
    - [smart-release #195] another test to validate log output (6148ebf)
    - [smart-release #195] a test that in theory should trigger the desired behaviour (fd50208)
    - [smart-release #194] basic journey test setup (d5d90a6)
    - thanks clippy (8fedb68)
    - [smart-release #194] conservative pre-release version updates (f23442d)
    - Bump git-repository v0.9.0 (b797fc1)
</details>

## v0.3.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.3.1 (1bcea9a)
    - [repository #190] refactor (e7188e0)
    - [repository #190] fix build (f5e118c)
    - [repository #190] a major step forward with `head()` access (43ac4f5)
</details>

## v0.3.0 (2021-08-27)

- add `--skip-dependencies` flag
- add `--verbose` flag and be less verbose in dry-runs by default to provide only essential information
- improvements to notification clarity

### Breaking

- Use short flag for `--no-bump-on-demand` in `--bump-dependencies`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.3.0 (82f0cec)
    - [smart-release #174] add asciinema recording of failed release (6668527)
    - [smart-release #174] prepare changelog (0d9a2b8)
    - Bump git-repository v0.8.0 (cdb45ff)
    - [smart-release] Adjust commit message depending on whether we are skipping the publish… (c190c6b)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [ref #175] make 'mutable' module private (a80dbcf)
    - Release git-lock v1.0.0 (f38f72c)
    - [stability #171] git-ref is now ST1 and available through git-repository (50154cd)
    - [smart-release #171] Try to avoid unstable git-repository features… (c8f325b)
    - Merge branch 'main' into stability (11bae43)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f)
    - [stability #171] finish tier description… (4fe1259)
    - Merge branch 'main' into 162-repo-design-sketch (e63b634)
    - [ref #165] refactor (66624c3)
    - [repository #165] refactor (1547d0b)
    - [repository #165] refactor; fine grained allow(missing_docs)… (aa0511f)
    - [repository #165] prepare for writing light docs for Easy (f8834c9)
    - [repository #165] refactor (3a0160e)
    - [repository #165] a sample of a simpler way to create a tag (fb8f584)
    - [smart-release #165] Use generic edit-reference functionality (be3e57f)
    - [repository #165] refactor (00ec15d)
    - [repository #165] offer panicking type conversions for objects (f802f8c)
    - [repository #165] try a more common naming convention for fallbile things… (fc70393)
    - [smart-release #162] use TreeRef capabilities to lookup path (51d1943)
    - [repository #162] finally let smart-release use the correct abstraction for peeling (ba243a3)
    - [repository #162] Add id field to ObjectRef… (f5ba98e)
    - [repository #162] experiment with finding objects… (312a692)
    - [repository #162] Cannot ever store a RefCell Ref in an object… (5c17199)
    - [repository #162] experiemnt with optionally keeping data in Object (b8a8e08)
    - [smart-release #162] Fix short flags (08f3418)
    - [smart-release #162] don't throw away work… (b43b780)
    - [smart-release #162] refactor (7f2421b)
    - [smart-release #162] peeling objects to a certain target kind… (5785136)
    - [smart-release #162] a single import path for ReferenceExt (7060797)
    - [smart-release #162] replace reference peeling with git_easy (7cfd5f9)
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode (4fb672a)
    - [smart-release #164] improve handling of empty commits (bd93fcb)
    - [smart-release #164] Make it easier to change a single crate's version only (38d28ce)
    - [smart-release #162] only warn if there is working tree modifications in dry-run mode… (f8ce62f)
    - [smart-release #162] clearer messages (aa7417f)
    - thanks clippy (45c5c3c)
    - [smart-release #162] top-level crate uses version-only tag (85e5b1a)
    - [smart-release #162] FAIL: single-crate workspaces use version-only tags (c5947c4)
    - [smart-release] better --verbosity handling (8cccb11)
    - [smart-release] properly obtain top-level crate name using manifest (d74b32e)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.4 (2021-08-15)

- Fix auto-push functionality

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.4 (19f21a4)
    - [smart-release #160] fix auto-push issue (73051d3)
</details>

## v0.2.3 (2021-08-15)

- Less verbosity by default which is helpful on the first run to get an overview. Use `--verbose/-v` for all the details.
- Also push tags and HEAD by default, unless `--skip-push` is specified.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.3 (f50bac8)
    - [smart-release #160] update chnagelog (7c4ff64)
    - [smart-release #160] Add the --skip-push flag… (6ebfc85)
    - [smart-release #160] Push after creating a single tag (6add57f)
    - [smart-release #160] a seemingly nice '--verbose' mode… (bf55679)
    - thanks clippy (bc7c9a8)
    - [smart-release #160] avoid trying to use an empty path when detecting changes… (836324e)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.2 (2021-08-15)

- support for unsorted packed-refs files

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.2 (f73c729)
</details>

## v0.2.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.1 (a3c45de)
    - [smart-release #155] Another note (5feb437)
</details>

## v0.2.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release #155] how to increase version numbers (0bad7b7)
    - Release cargo-smart-release v0.2.0 (b95d7ed)
    - [smart-release #155] keep dependency versions by default (4f53287)
    - [smart-release #155] fix bug :D (3d2a044)
    - [smart-release #155] workflow notes and inversion of flag for comfort (1ffb66c)
    - thanks clippy (c50bd73)
    - [smart-release #155] inform about latest features (133e43a)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2021-08-13)

- initial release
### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release #155] refactor (21192b8)
    - [smart-release #155] prepare release (4684557)
    - [smart-release #155] even smarter bumping (1f38680)
    - [smart-release #155] --bump-dependencies only (19d87a6)
    - [smart-release #155] incorporate crates-index for additional version check (08bd13d)
    - [smart-release #155] prepare for crates-index; refactor (498b6cc)
    - [smart-release #155] make it an actual depth-first traversal :D (b05a21f)
    - [smart-release #155] sanity check for dry-run/no-dry-run-cargo-publish (2fa7b0b)
    - [smart-release #155] update README, add changelog (b5dd553)
    - thanks clippy (7709ca0)
    - [smart-release #155] graceful handling of unspecified crate to publish (e65b657)
    - [smart-release #155] rely only on cargo metadata for root paths (217dafb)
    - [smart-release #155] also ignore provided crate names if they didn't change (2110a8c)
    - [smart-release #155] gracefully fail when encountering unknown comparators (bee367b)
    - [smart-release #155] don't set versions if the new ones match (dd0f428)
    - [smart-release #155] refactor (07dc6d8)
    - [smart-release #155] remove dia-semver (07f84c7)
    - [smart-release #155] don't set versions where there are none when fixing manifests (a1cc79f)
    - [smart-release #155] also find renamed dependencies when updating versions (06bc6a9)
    - [smart-release #155] a note (a726225)
    - [smart-release #155] invert meaning of cargo-publish dryrun flag (cc57eb8)
    - [smart-release #155] allow dry-running cargo publish, too… (15e611e)
    - [smart-release #155] allow dry-running cargo-publish, too (a3add55)
    - [smart-release #155] Flag to auto-publish dependent stable crates as well (bded12f)
    - [smart-release #155] don't auto-add stable crates but suggest to do something about it (d1dca70)
    - [smart-release #155] refactor (8e78e77)
    - thanks clippy (507cb94)
    - [smart-release #155] refactor (fb1fb57)
    - [smart-release #155] don't rely on cargo resolution order for cyclic case/publish groups (7c97fa4)
    - [smart-release #155] avoid using cargo resolution order (4b7d9d1)
    - [smart-release #155] properly handle multi-crate dependencies (if there is no cycle) (e8838a9)
    - [smart-release #155] trust our own resolution order more… (a977925)
    - [smart-release #155] refactor (0841088)
    - [smart-release #155] don't check cycles on dependencies without version (9eeaa2f)
    - [smart-release #155] refactor (3f887a7)
    - [smart-release #155] refactor (680675b)
    - [smart-release #155] refactor (20a3aef)
    - remove dev-dependency cycles by removing their version (c40faca)
    - [smart-release #155] prepare release (1330dff)
    - [smart-release #155] cargo compatibility (d432a8e)
    - [smart-release #155] add readme (86252eb)
    - [smart-release #155] --skip-tag flag (469de34)
    - [smart-release #155] --bump option (552d244)
    - [smart-release #155] remove subcommands (9f82828)
    - [smart-release #155] rename from 'utils' (a9e6fcc)
</details>

