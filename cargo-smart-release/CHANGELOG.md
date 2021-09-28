# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Conservative pre-release version handling along with a flag to turn it off. See [this issue][194] for details.
- Rename `--allow-auto-publish-of-stable-crates` to `--no-auto-publish-of-stable-crates`, inverting its meaning.
- Add `--no-multi-crate-release` flag to return to previous default behaviour. All crate manifest changes are put into one commit.
- automatically bump pre-release transient dependents of published crates to prevent breakage down the road unless 
  `--no-isolate-dependencies-from-breaking-change` is set.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 172 commits contributed to the release over the course of 19 calendar days.
 - 12 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 14 times to make code idiomatic. 

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
    - reorder headlines according to version ordering… (733d7f1)
    - Sort all commits by time, descending… (7c37a3d)
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Use most relevant parent tree for change comparison… (6d02ac8)
    - Use hashmap based lookup for trees… (55d2d17)
    - refactor and improve path filtering to find relevant commits… (99db079)
    - The first headline level controls all the other ones (302b267)
    - adapt to git-hash refactor (d881734)
    - Fixup remaining changelogs… (0ac488a)
    - Generate changelogs with details (fd0f3bd)
    - Only use short hashes for logs, without detecting ambiguity for now (8d48d59)
    - boost allowed package sizes… (c1636e4)
    - Stable smart-release journey tests… (d5b3099)
    - Update all changelogs with details (0732699)
    - Put commit details to the end of generated segments (bef2e0e)
    - Use message commit id instead of body… (2746d93)
    - fix md formatting on github (f90f3ce)
    - create details headline based on log message (4eaab37)
    - Add details behind a fold, but… (3c711f4)
    - Use the notion of 'changes after merge' only to drive previews… (fb0e46b)
    - Update changelogs (b30db3b)
    - refactor (90c304e)
    - Also provide a duration in days for preparing a release as part of statistics (4d36844)
    - Fix tests (ff15c1b)
    - refactor (bcec911)
    - More commit statistics (29a01d6)
    - Basic commit statistics with round-trip, more actual information to come (83e2b2d)
    - refactor… (6edf196)
    - More robust parsing of read-only sections (252e84f)
    - treat clippy as generated statistical section… (96df84d)
    - Add new section type and write it out: clippy (37280cd)
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
    - thanks clippy (11bd4a3)
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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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
 - 1 unique issue was worked on

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

