# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## v0.3.3 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-url v0.3.3 (fdd5bdb)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.3.2 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 94 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#79**
    - refactor; add test for empty tree iteration (6340296)
 * **Uncategorized**
    - (cargo-release) version 0.3.2 (03de99e)
    - (cargo-release) version 0.3.1 (4deef67)
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
</details>

## v0.3.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 12 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (d5c6643)
    - thanks clippy (e13adb2)
    - [gitoxide-core] Use git-config for remote url parsing (c45feed)
</details>

## v0.2.0 (2021-01-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (0c39373)
    - support for radicle urls (2c5b955)
</details>

## v0.1.1 (2020-12-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 87 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 (e94fefa)
    - finish git-url docs (4099508)
    - begin of documenting git-url crate (c891901)
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
    - refactor (ba1d883)
    - take not of a few more obscure features (8f9570c)
    - refactor (7c3c80a)
    - (cargo-release) version 0.4.3 (5b47a1a)
    - refactor (8930610)
    - Enforce using the correct version of clap (fd6457f)
</details>

## v0.1.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 93 commits contributed to the release over the course of 28 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update dependency chain in release script (9af4799)
    - refactor (e4bcfe6)
    - remove quickerror dependency from git-odb (7e27495)
    - refactor (6a84f13)
    - refactor (7874c35)
    - refactor (4e89c3b)
    - refactor (3ec99dc)
    - Document why we won't use nightly for fixing NLL issue (ca29368)
    - refactor (519dd12)
    - Revert "Fix NLL issue by using nightly" (6864a55)
    - refacator (7ac2153)
    - Fix NLL issue by using nightly (8c5bd09)
    - refactor (d4f288c)
    - Update tasks, prepare for NLL fix (52af8d1)
    - refactor (3a8fb61)
    - Thanks clippy (6c4d1ec)
    - dependency update (4a762f6)
    - This works, but locates twice… (4e709f6)
    - Fixes for clap beta-2 (3fcdc5d)
    - Also the imperative version doesn't borrowcheck… (c5720f1)
    - refactor (98b3f4a)
    - Looks like the functional approach to locate(…) doesn't borrowcheck (5df6867)
    - dependency update (e897b50)
    - refactor (9e68c6b)
    - refactor (127b8b2)
    - refactor (f219d5a)
    - refactor (669b726)
    - sketch compound::Db::locate; sort packs by size (6609a53)
    - refactor (7bc321e)
    - refactor (4a09754)
    - lower velocity (69f7930)
    - Implement Write in terms of writing to the loose object DB (02b88c2)
    - refactor (0752b45)
    - First sketch of compound Db (9bf2279)
    - (cargo-release) version 0.4.1 (64fff36)
    - refactor (203ba99)
    - (cargo-release) version 0.4.1 (105c501)
    - (cargo-release) version 0.2.1 (ebf3419)
    - (cargo-release) version 0.4.1 (60ac8b0)
    - (cargo-release) version 0.6.0 (9ef184e)
    - refactor (ad17bfd)
    - (cargo-release) version 0.1.1 (bb38c6b)
    - refactor (91d9f78)
    - (cargo-release) version 0.2.1 (abc218c)
    - refactor (6ebb5d1)
    - refactor (e07fbd6)
    - [clone] encode message for git credentials helper (143549e)
    - [clone] make URL available in transport layer (6778447)
    - [clone] Finish round-trip testing (df617fd)
    - refactor (aea52fe)
    - [clone] first sketch of roundtripping URLs (23678f8)
    - [clone] first steps towards launching git-upload-pack while… (41f05f1)
    - [clone] Better error handling for generalized `connect(…)` (713808c)
    - [clone] expand-path should be server-side (8a38856)
    - thanks clippy (0506fd9)
    - [url] more specific 'missing user home' error (ec5721a)
    - refactor (e54681a)
    - [url] Actually the is_relative() case should never be triggered (ac89d38)
    - [url] try again, maybe this works on windows… (f14fdd1)
    - [url] Once more with feeling (2ea4a8c)
    - [url] all debug output there is… (3df5b41)
    - [url] yikes, more debugging for windows on CI (9a430e7)
    - [url] Another try to make this work on windows - tests probably (a51647f)
    - [url] See if this fixes the windows tests (534c6a6)
    - [url]  add standard conversions (27e3bdc)
    - refactor (73e2b1b)
    - [url] BString in public interface (745662d)
    - [url] Commit to 'bstr' (3d26ae1)
    - [url] remove feature toggle, 'home' dependency is small enough (a5a6f0f)
    - [url] Add user expansion support (behind feature toggle) (a684cfe)
    - [url] first stab at expanding paths with user names (37459dc)
    - thanks clippy (50acab7)
    - [url] Support for git and http urls, as well as user expansion parsing (5ef201d)
    - refactor (6ab7cc6)
    - [url] first stab at implementing username expansion reasonably (86d17a3)
    - [url] fix serde (569014d)
    - [url] Now with support for non-utf8 byte strings (81f01fd)
    - [url] more tests and additional limitations (3c2811f)
    - [url] handle trivial file protocol URLs better (18eb512)
    - [url] Disable URL parsing for things that look like paths (03b0de9)
    - [url] turns out that relative URLs and windows paths are killing it (0bee58e)
    - [url] Switch to 'url' crate, as correctness certainly is more important than compile times (da6ad48)
    - thanks clippy (a37c7a3)
    - [url] user and IPv4 parsing/simple validation (d1929ac)
    - [url] parse port number (bc8bd99)
    - try for leaner tests, but it does the opposite kind of :D (098f802)
    - refactor (4499a08)
    - refactor (42a1b51)
    - [url] the first green tests (a501bc1)
    - refactor (9c5fb91)
    - [url] infrastructure for nom errors, taken from git-object (0ae38ed)
    - [url] basic frame and first failing test (60aacf0)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
</details>

## v0.0.0 (2020-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add git-url crate (fd2e5ba)
</details>

