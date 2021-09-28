# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 13 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Fixup remaining changelogs… (0ac488a)
    - Generate changelogs with details (fd0f3bd)
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - fix docs (90056c8)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b63)
 * **#205**
    - '(null)' symref targets are turned into direct refs instead… (c77bd7a)
    - fetch::Ref::Symbolic::target is now an option… (da68bfb)
 * **Uncategorized**
    - Merge branch 'changelog-generation' (bf0106e)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.10.4 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.4 (898ee08)
    - thanks clippy (4701296)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.10.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.3 (aa90f98)
    - Bump git-hash v0.6.0 (6efd90d)
</details>

## v0.10.2 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.2 (54a4400)
    - [various #184] configure docs.rs build features (cc50249)
</details>

## v0.10.1 (2021-08-27)

- instruct docs.rs which features to use for more useful documentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.1 (cec8ee3)
    - [protocol #174] fix tests… (cdc16fc)
</details>

## v0.10.0 (2021-08-27)

- Various minor updates of pre-release dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-protocol v0.10.0 (82d5a0b)
    - Bump git-transport v0.11.0 (1149f1b)
    - Bump git-packetline v0.10.0 (b09f391)
    - [packetline #178] rename PacketLine to PacketLineRef… (d4c16a9)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe)
    - Upgrade to nom-7 (f0aa3e1)
</details>

## v0.9.0 (2021-08-17)

### BREAKING

- Add fifth argument to `fetch(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [protocol] prepare release to fix crates-io instalations (83d7423)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e3340)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

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

## v0.8.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 101 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - [protocol RL-#741] Respect delegate configuration when running only ls-refs (65ce8e1)
    - [protocol #145] Unify the `previous` and `previous_result` parameters… (96f77c7)
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
    - [protocol] adjust description of fetch::Error to match io::Error sources (23dafc6)
    - Revert "[ref] Try using BorrowMut to avoid blanket trait impls, but…" (8212536)
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… (4bb9bba)
    - [protocol] only send flush packets in stateful connections (0995c22)
    - [transport] remove Transport::close()… (4268a9b)
    - [ref] rename Action::Close to Action::Cancel… (cac1f6c)
    - [transport] impl Delegate for &mut T: Delegate; refactor fetch() signature (2ded7f9)
    - [transport] implement Transport for &mut T: Transport as well (372fb81)
    - [protocol] fallible negotiation (e269a2c)
    - [protocol] refactor (11b2fd1)
    - [protocol] refactor (967946a)
    - [protocol] refactor (8dc425f)
    - [protocol] assure we don't coerce refs into UTF-8 representation (5ceb64d)
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
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - [git-protocol] remove compile warnings if no client type is specified… (478a980)
    - thanks clippy (57106e2)
    - [git-protocol] builds without features work (a1945ff)
    - [git-protocol] async fetch tests work (fe434a5)
    - [git-protocol] fetch tests nearly compile in async (97fb186)
    - [git-protocol] fetch in sync and async… (4776039)
    - [git-protocol] refactor (80379fd)
    - [git-protocol] build should fail if mutually exclusiive features are set (72cf940)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a8)
    - [git-protocol] fix build (4cce648)
    - [git-protocol] async Delegate (1aa6781)
    - thanks clippy (0759ade)
    - [git-protocol] refactor (94d7be4)
    - [git-protocol] refactor (990099b)
    - [git-protocol] refactor (d623cf7)
    - [git-protocol] async response (c498557)
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

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (069184e)
    - (cargo-release) version 0.8.0 (411a05e)
    - (cargo-release) version 0.5.0 (8c4cc3f)
    - thanks clippy (17258cc)
    - (cargo-release) version 0.14.0 (a760f8c)
    - (cargo-release) version 0.3.0 (e9665c7)
    - (cargo-release) version 0.13.0 (ac2eddb)
</details>

## v0.6.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Commit Details

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

## v0.5.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (3cc4a57)
    - (cargo-release) version 0.6.0 (50fb6f2)
    - thanks clippy (0fc239c)
    - thanks clippy (749ceba)
    - (cargo-release) version 0.11.0 (1aa1f5e)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.4.1 (2021-01-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

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

## v0.4.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (28df5e9)
    - All crates use git-hash::Kind and its types, sometimes through git-object (124c171)
    - use git-hash in git-features (5b307e0)
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (e60dbe6)
    - (cargo-release) version 0.4.0 (32aefc0)
    - (cargo-release) version 0.4.0 (72eaece)
    - (cargo-release) version 0.9.0 (a89fdb9)
    - (cargo-release) version 0.5.0 (fc7d600)
</details>

## v0.2.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (a476a46)
    - (cargo-release) version 0.3.0 (d19ee35)
    - (cargo-release) version 0.3.0 (eade7d1)
    - (cargo-release) version 0.8.0 (47c00c2)
    - cargo clippy Rust 1.48 (475a68c)
    - (cargo-release) version 0.7.0 (7fa7bae)
    - thanks clippy (b9e0a87)
    - remove dash in all repository links (98c1360)
    - refactor (7c3c80a)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (9ef184e)
    - (cargo-release) version 0.1.1 (bb38c6b)
    - Support V2 shallow-info section (6679c91)
    - Tests for V2 shallow section parsing (5bf58ab)
    - Support for the 'deepen-relative' argument (b86fed6)
    - V1 parsing of shallow and unshallow lines… (8bcf535)
    - remove unused fixtures (6ae69f5)
    - Fix wants/haves separator handling for stateful V1 (1629575)
    - Make really clear that V2 is stateless no matter what the transport supports :D (c296845)
    - Assure the first 'want' in V1 is always first (e729ec8)
    - Properly handle statelessness in V2 protocol (1b49f1e)
    - add some samples for deepen clones (61bc41a)
    - Switch to prodash 10 and safe a lot of trait bounds in the process (e2fb1d9)
</details>

## v0.1.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 182 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (0d7b60e)
    - (cargo-release) version 0.2.0 (779e9d0)
    - (cargo-release) version 0.2.0 (da830de)
    - (cargo-release) version 0.5.0 (82b7313)
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! (d9ced27)
    - [clone] more correct handling of 'no-done'/done when sending wants/haves… (50f4516)
    - [clone] Don't try to explicitly close the connection… (17200b3)
    - [clone] Fix encoding of V1 capabilities in first want (b68a5c5)
    - [clone] When unpacking peeled refs, use the object that refers to the tag… (fe8bb39)
    - [clone] none the wiser - it really looks like everything is alright… (3b8d613)
    - [clone] it looks like in order to figure out the issue, it needs tests higher up… (edf1540)
    - [clone] Don't send V2 capabilities that don't have a value… (9c9a4ee)
    - [clone] Handle remote progress name prefixing (more) correctly (51d4d15)
    - [clone] This actually works: first MVP of retrieving packs via clone (c06d819)
    - Use git attributes to prevent crlf conversion of fixtures on windows (80ca8b2)
    - [clone] Support for reading multi-step negoritaions, but… (507d342)
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

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

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
    - first bunch of tasks I see after studying parts of the protocol docs (9bd97ba)
</details>

