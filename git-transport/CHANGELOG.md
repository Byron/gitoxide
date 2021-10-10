# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: #198, #200

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - respect release-wide ignore list to allow removing entire conventional headlines (145103d)
    - Rebuild all changelogs to assure properly ordered headlines (4a9a05f)
    - Sort all commits by time, descending… (f536bad)
    - greatly reduce changelog size now that the traversal fix is applied (a0bc98c)
    - Fixup remaining changelogs… (2f75db2)
    - Generate changelogs with details (e1861ca)
    - Update all changelogs with details (58ab2ae)
    - Update changelogs (c857d61)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b63)
 * **Uncategorized**
    - Update changelogs just for fun (21541b3)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.11.1 (2021-08-29)

- instruct docs.rs which features to use for more useful documentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.11.1 (0952976)
    - [various #184] configure docs.rs build features (cc50249)
</details>

## v0.11.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-transport v0.11.0 (1149f1b)
    - [transport #174] prepare for release (f8bc517)
    - Bump git-packetline v0.10.0 (b09f391)
    - [packetline #178] refactor (23438fd)
    - [packetline #178] rename PacketLine to PacketLineRef… (d4c16a9)
</details>

## v0.10.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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

 - 11 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 116 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.16.0 (1231dbd)
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - Bump async-trait from 0.1.50 to 0.1.51 (ce0b81e)
    - [transport] more convenient check for available capabilities (e9ed952)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
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
    - Add missing docs (a6cbbde)
    - [git-transport]: make capabilities parsing public (2f3725e)
    - thanks clippy (6200ed9)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - refactor (2a406d6)
    - [async-client] frame for async connect (9ada080)
    - Prevent selecting mutually exclusive features (7f5da18)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - [git-transport] Fix http build (3469e99)
    - [git-protocol] fetch in sync and async… (4776039)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a8)
    - fix docs (bca7594)
    - [git-protocol] fix build (4cce648)
    - [git-transport] refactor (d09153f)
    - [git-transport] Properly implement Transport for Boxed types (47b10c9)
    - [git-transport] refactor (3b0baee)
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

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.8.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
 - 0 issues like '(#ID)' where seen in commit messages

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

 - 7 commits contributed to the release over the course of 70 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (50fb6f2)
    - (cargo-release) version 0.3.0 (d5c6643)
    - thanks clippy (749ceba)
    - [gix] Add optional zlib feature (f1f9665)
    - (cargo-release) version 0.11.0 (1aa1f5e)
    - (cargo-release) version 0.2.0 (0c39373)
    - support for radicle urls (2c5b955)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.5.1 (2021-01-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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
 - 0 issues like '(#ID)' where seen in commit messages

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
 - 0 issues like '(#ID)' where seen in commit messages

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

 - 11 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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
    - refactor (ba1d883)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.1 (ebf3419)
    - (cargo-release) version 0.6.0 (9ef184e)
</details>

## v0.2.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 189 commits contributed to the release over the course of 28 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (779e9d0)
    - (cargo-release) version 0.2.0 (da830de)
    - (cargo-release) version 0.5.0 (82b7313)
    - thanks clippy (e5d80b1)
    - [clone] make cloning the linux kernel work (e780526)
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! (d9ced27)
    - [clone] none the wiser - it really looks like everything is alright… (3b8d613)
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

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

## v0.0.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add missing license description (2b80181)
    - Make crates publishable (5688a34)
    - \#[forbid(unsafe)] for all crates (afda803)
    - cleanup - don't build and run tests while there is nothing to test (4a153da)
    - prepare git-transport just so that we don't forget to take the name (2c3ad7d)
</details>

