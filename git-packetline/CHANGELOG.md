# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 13 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#198**
    - refactor and improve path filtering to find relevant commits… (99db079)
    - The first headline level controls all the other ones (302b267)
    - Fixup remaining changelogs… (0ac488a)
    - Generate changelogs with details (fd0f3bd)
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Parse more user generated section content, adapt existing changelogs to work correctly (2f43a54)
 * **#200**
    - parse issue numbers from description and clean it up (95c0a51)
    - feat: add git_packetline::read::Error to represent ERR lines (454c840)
 * **Uncategorized**
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
</details>

## v0.10.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.10.1 (4f9da02)
    - [ref #190] more conversion trait impls (1795a33)
</details>

## v0.9.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.1 (2276e2a)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.9.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.0 (7ffbd60)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
</details>

## v0.8.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 (ad6d7f9)
    - (cargo-release) version 0.18.0 (b327590)
</details>

## v0.7.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (2ef3106)
    - (cargo-release) version 0.17.0 (c52a491)
</details>

## v0.6.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 103 commits contributed to the release over the course of 89 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#77**
    - [git-packetline] refactor (aa61993)
 * **Uncategorized**
    - clippy on tests and thanks clippy (a77a71c)
    - thanks clippy (e1964e4)
    - [ref] refactor (bd94ea5)
    - [pack] fix docs (e7b9d96)
    - Bump futures-io from 0.3.15 to 0.3.16 (3c23820)
    - Merge branch 'remove-unnecessary-unsafe' (7a3c5c1)
    - Remove unnecessary pub(crate) exports (3d2456e)
    - fix docs (2698dae)
    - fix build (22bda81)
    - thanks clippy (3f7e27b)
    - thanks clippy (6200ed9)
    - [async-client] unblock the async delegate in the cheapest possible way… (a3b5d75)
    - Revert "[async-client] Try to bring 'Send' back but…" (52eb953)
    - [async-client] Try to bring 'Send' back but… (3a06adb)
    - Prevent selecting mutually exclusive features (7f5da18)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - [git-protocol] fix build (4cce648)
    - [git-protocol] fetch in sync and async… (4776039)
    - Bump maybe-async from 0.2.4 to 0.2.6 (d99a1a8)
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

## v0.5.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 133 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

## v0.4.1 (2020-12-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 9 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

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

## v0.4.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (72eaece)
    - (cargo-release) version 0.6.0 (27f5955)
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 (eade7d1)
    - (cargo-release) version 0.5.0 (c767e07)
    - remove dash in all repository links (98c1360)
    - (cargo-release) version 0.4.0 (2272fa4)
    - refactor (8930610)
</details>

## v0.2.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 47 commits contributed to the release over the course of 26 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.1 (abc218c)
    - Assure peek behaves exactly as we want it to with ERR lines (bbdaee5)
    - V1 parsing of shallow and unshallow lines… (8bcf535)
    - [clone] Support for reading multi-step negoritaions, but… (507d342)
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

## v0.2.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 (da830de)
</details>

## v0.1.0 (2020-08-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

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
#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs (081faf5)
    - Bump git-packetline v0.10.0 (b09f391)
    - [packetline #178] fix docs (878d8e8)
    - [packetline #178] refactor (0c7c599)
    - [packetline #178] fix docs (b3fd65d)
    - [packetline #178] refactor (23438fd)
    - [packetline #178] rename PacketLine to PacketLineRef… (d4c16a9)
    - [packetline #178] add changelog in preparation for breaking changes (ffd96f9)
</details>

