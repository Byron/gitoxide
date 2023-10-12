# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 11.0.0 (2023-10-12)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 17 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
</details>

## 10.0.0 (2023-09-24)

<csr-id-a8b227ebe7dd140361de9bd7f0d88debfc2416d0/>

### Other

 - <csr-id-a8b227ebe7dd140361de9bd7f0d88debfc2416d0/> Improve documentation to make possibility of leaks more clear

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 16 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1026](https://github.com/Byron/gitoxide/issues/1026)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1026](https://github.com/Byron/gitoxide/issues/1026)**
    - Improve documentation to make possibility of leaks more clear ([`a8b227e`](https://github.com/Byron/gitoxide/commit/a8b227ebe7dd140361de9bd7f0d88debfc2416d0))
 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
    - Merge branch 'gix-lock-improvements' ([`b009db0`](https://github.com/Byron/gitoxide/commit/b009db018df435a8c57848dc4cdc52d4278a6d9d))
</details>

## 9.0.0 (2023-09-08)

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 8.0.0 (2023-08-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Prepare more changelogs ([`6672d8e`](https://github.com/Byron/gitoxide/commit/6672d8e9750166016fb7a55e8be5b6d7c0072453))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
</details>

## 7.0.2 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Prepare yet another changelog ([`8451aac`](https://github.com/Byron/gitoxide/commit/8451aace5161ec6f5950318d6bfd042e7c6c62cc))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Prepare more changelogs ([`13a497f`](https://github.com/Byron/gitoxide/commit/13a497f679943908793e743810f1b0b2501ba3b6))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 7.0.1 (2023-07-20)

### Bug Fixes

 - <csr-id-60b53f4aac4c8cf7cf1299e0847ca66fc8e9a18b/> allow re-release of `gix-tempfile` with latest dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v7.0.1 ([`95c30b2`](https://github.com/Byron/gitoxide/commit/95c30b249d7a1d4767cc0a96316da2734a3dd9f6))
    - Allow re-release of `gix-tempfile` with latest dependencies ([`60b53f4`](https://github.com/Byron/gitoxide/commit/60b53f4aac4c8cf7cf1299e0847ca66fc8e9a18b))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
</details>

## 7.0.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 10 calendar days.
 - 15 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
</details>

## 6.0.0 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 11 calendar days.
 - 41 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
</details>

## 5.0.3 (2023-04-26)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 10 calendar days.
 - 31 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Make fmt ([`5d2b5d0`](https://github.com/Byron/gitoxide/commit/5d2b5d02c3869e07dc2507a8f2519ee1df633df7))
    - Create new `gix-fs` crate to consolidate all filesystem utilities ([`f8cc33c`](https://github.com/Byron/gitoxide/commit/f8cc33cb372dd2b4bbe4a09cf4f64916681ab1dd))
</details>

## 5.0.2 (2023-03-26)

A maintenance release without any user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 3 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-prompt v0.3.3, gix-diff v0.28.1, gix-discover v0.16.1, gix-pack v0.33.2, gix-transport v0.29.1, gix-protocol v0.30.1, gix-revision v0.12.1, gix-worktree v0.15.1, gix v0.43.0, safety bump gix v0.43.0 ([`5dc1f9f`](https://github.com/Byron/gitoxide/commit/5dc1f9f2bcb8b3e147115fcb6f76558e8f48ffef))
    - Prepare changelogs prior to release ([`3016a28`](https://github.com/Byron/gitoxide/commit/3016a285f566bdfe7de2774fa6f2254c1b1a2c51))
    - Correct more typos with `typos` tool. ([`2321eb9`](https://github.com/Byron/gitoxide/commit/2321eb971c2b89551506e2016a3495fafd15b47d))
</details>

## 5.0.1 (2023-03-14)

A maintenance release without any user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.28.1, gix-tempfile v5.0.1, gix-ref v0.27.1, gix-pack v0.33.1, gix-packetline v0.15.0, gix-transport v0.29.0, gix-protocol v0.30.0, gix v0.42.0, safety bump 3 crates ([`c1f1bfb`](https://github.com/Byron/gitoxide/commit/c1f1bfb8dc0e73993678353e4492d0614b642ed1))
    - Prepare changelogs prior to release ([`c66e298`](https://github.com/Byron/gitoxide/commit/c66e2982577e4cd9faef63798986b8cf8ece93a2))
    - Make fmt ([`3836cc0`](https://github.com/Byron/gitoxide/commit/3836cc0c9c3e1158b56142b924483c8a77217d53))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## 5.0.0 (2023-03-10)

### Bug Fixes (BREAKING)

 - <csr-id-304b0c1ffdfc90bbfaad1bf30162070a5ce4b8c0/> Upgrade `tempfile` to 3.4, fixing a security issue.
   This is not really a breaking change, but a tactical major version increment
   to give `cargo` an escape hatch and let it use the vulnerable version for a little
   longer.
   
   This means this `gix` up until v0.40 can be used to pull in `tempfile` 3.3, and
   all future versions will use tempfile 3.4 (or 3.X) for that matter.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
    - Upgrade `tempfile` to 3.4, fixing a security issue. ([`304b0c1`](https://github.com/Byron/gitoxide/commit/304b0c1ffdfc90bbfaad1bf30162070a5ce4b8c0))
</details>

## 4.1.1 (2023-03-07)

### Bug Fixes

 - <csr-id-6778c5f6874ec2a44859de25428e6603dd4e9e8a/> switch `tempfile` to `~3.3.0` to help with `cargo` build.
   `gix-tempfile` is now used in `cargo`, and `tempfile` seems to cause trouble
   in at least one tier (see https://github.com/rust-lang/rust/pull/108665#issuecomment-1457677883).
   
   This change pins `tempfile` to a working version until this is sorted out together with
   the tempfile maintainers.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v4.1.1 ([`2574b75`](https://github.com/Byron/gitoxide/commit/2574b7592b5e0e7a709608ce674568d15b8fc3e8))
    - Switch `tempfile` to `~3.3.0` to help with `cargo` build. ([`6778c5f`](https://github.com/Byron/gitoxide/commit/6778c5f6874ec2a44859de25428e6603dd4e9e8a))
</details>

## 4.1.0 (2023-03-01)

### New Features

 - <csr-id-1ff1342da793442859526df3dd0ec1ed968a6ded/> add `hp-hashmap` feature toggle (enabled by default).
   This allows avoiding the `dashmap` dependency, and to optimize load
   for different applications. It seems that this simple implementation
   has performance benefits in some situations at least even though it
   may leave all tempfiles in place if a signal handler tries to cleanup
   while a lock is held.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v4.1.0, gix-lock v4.0.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0, safety bump 6 crates ([`ea9fd1d`](https://github.com/Byron/gitoxide/commit/ea9fd1d9b60e1e9e17042e9e37c06525823c40a5))
    - Prepare changelogs prior to release ([`94c99c7`](https://github.com/Byron/gitoxide/commit/94c99c71520f33269cc8dbc26f82a74747cc7e16))
    - Merge branch 'adjustments-for-cargo' ([`d686d94`](https://github.com/Byron/gitoxide/commit/d686d94e1030a8591ba074757d56927a346c8351))
    - Add `hp-hashmap` feature toggle (enabled by default). ([`1ff1342`](https://github.com/Byron/gitoxide/commit/1ff1342da793442859526df3dd0ec1ed968a6ded))
    - Avoid clearing the registry while running tests ([`d635fb7`](https://github.com/Byron/gitoxide/commit/d635fb7a060bfbc5b584bec9976549383d369f47))
</details>

## 4.0.0 (2023-02-26)

### New Features (BREAKING)

 - <csr-id-441f5ac4dd2f0636ec07065f8095e8bae5ce6985/> gate all signal handling behind the `signals` feature toggle.
   This change also consolidates all signal handling into its own module called
   `signal` to provide reusable handlers and as well as well as signal initialization.
   
   Note that the functions to cleanup tempfiles don't interact with the signal registry,
   hence they still can be called without the `signals` feature enabled.
   
   Note that this change sneakily fixes a bug that could have caused a `write_all()`
   on a tempfile that was removed by a signal to enter an infinite loop.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#339](https://github.com/Byron/gitoxide/issues/339)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#339](https://github.com/Byron/gitoxide/issues/339)**
    - Gate all signal handling behind the `signals` feature toggle. ([`441f5ac`](https://github.com/Byron/gitoxide/commit/441f5ac4dd2f0636ec07065f8095e8bae5ce6985))
 * **Uncategorized**
    - Release gix-tempfile v4.0.0 ([`ad23f1d`](https://github.com/Byron/gitoxide/commit/ad23f1dd58b3eabf5a12d02c4a1de8b1835051d5))
    - Prepare for git-tempfile release ([`56c005b`](https://github.com/Byron/gitoxide/commit/56c005b13c44376f71e61781e73c0bf93416d0e4))
    - Merge branch 'tempfile-upgrades' ([`3522cba`](https://github.com/Byron/gitoxide/commit/3522cbaac721c8079605be51b9053014bc5e863a))
    - Upgrade `tempfile` to `3.4` ([`2670938`](https://github.com/Byron/gitoxide/commit/2670938df5f6a3ed155b793e301ea0ab64b8cec1))
</details>

## 3.0.3 (2023-02-17)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

## 3.0.2 (2023-02-17)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>
<csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/>

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Bug Fixes

 - <csr-id-81ed5f5e7a3634f0fab681ca59e40099f0118f75/> Assure interrupt based tempfile cleanup can't deadlock
   We do this by using the `try_entry()` API provided by the most recent
   dashmap, at the cost of potentially trying to access a lot of indices
   that don't exist in the map anymore. The cost of this are expected
   to be low though especially since interrupts are the uncommmon case.
   
   A side-effect of this is that tempfiles that are currently being
   removed for writing to them, for example, won't be cleaned up.
   
   It will be up to the code running after the interrupt, if the case
   at all, to deal with the tempfile, which is what it does anyway.
 - <csr-id-d9451e8d7fc39c252042f9d2447061262c16ae7a/> downgrade dashmap to 4.0 to avoid unsoundness.
   See https://github.com/xacrimon/dashmap/issues/167 for tracking
   progress on resolving the issue.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere
 - <csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/> upgrade dashmap to 5.1.0 (with security fix)

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 238 commits contributed to the release over the course of 612 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#266](https://github.com/Byron/gitoxide/issues/266), [#298](https://github.com/Byron/gitoxide/issues/298), [#328](https://github.com/Byron/gitoxide/issues/328), [#336](https://github.com/Byron/gitoxide/issues/336), [#364](https://github.com/Byron/gitoxide/issues/364), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - Handle changelogs with upcoming version section if they were left for editing ([`0f5f47d`](https://github.com/Byron/gitoxide/commit/0f5f47da4662b596cbbbd9c0d83e135e2cc52c11))
    - Deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - Regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - Greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - Upgrade dashmap to latest version ([`52d4fe5`](https://github.com/Byron/gitoxide/commit/52d4fe55b6dd88f72479abd4015cab063ddaaf97))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Refactor ([`591b533`](https://github.com/Byron/gitoxide/commit/591b5338ecdc0da33151baa0781fd8dc1ee8d5a9))
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
 * **[#328](https://github.com/Byron/gitoxide/issues/328)**
    - A stress test to try conjure deadlocks in handlers ([`3bdecb5`](https://github.com/Byron/gitoxide/commit/3bdecb54570b46e4c140e783f49ff23ef465582d))
    - Prepare changelog ([`8e92494`](https://github.com/Byron/gitoxide/commit/8e924948dfa366d3d39227b63053c7ff00a5382a))
    - Assure interrupt based tempfile cleanup can't deadlock ([`81ed5f5`](https://github.com/Byron/gitoxide/commit/81ed5f5e7a3634f0fab681ca59e40099f0118f75))
 * **[#336](https://github.com/Byron/gitoxide/issues/336)**
    - Update changelog ([`2cfbe9c`](https://github.com/Byron/gitoxide/commit/2cfbe9ce81f611c0b541b9c0fd4fffd2d99dfa0a))
    - Do not install signal handlers by default ([`c863ea5`](https://github.com/Byron/gitoxide/commit/c863ea5b34fa9ee3dac21c1f85587da16045f8d8))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Add safety notes ([`910fa06`](https://github.com/Byron/gitoxide/commit/910fa06f39c9839c58d59467bca05ad511c23bef))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - Adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - Adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - Adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - Adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - Adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - Adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - Adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - Adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - Adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - Adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - Adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - Adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - Adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - Adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - Adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - Adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - Adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - Rename `git-tempfile` to `gix-tempfile` ([`192ed5d`](https://github.com/Byron/gitoxide/commit/192ed5d15c04fdfb330b3ab553eeaaf09f7fccd4))
    - Adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - Adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - Adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - Adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - Adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - Adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - Adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - Adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - Adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Merge branch 'rename-crates' ([`6461c3d`](https://github.com/Byron/gitoxide/commit/6461c3da4d6daee857606d94294c3f87fc36965a))
    - Rename `git-repository` to `gix` ([`7bed2a9`](https://github.com/Byron/gitoxide/commit/7bed2a96604397fa990f427b1a970ddeb6f09f1c))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`95f2f4f`](https://github.com/Byron/gitoxide/commit/95f2f4f17f7eae174a64c7d9f6a1513d73b21bbb))
    - Thanks clippy ([`f90d772`](https://github.com/Byron/gitoxide/commit/f90d772cf625afea605e42c92db15ed870d7e120))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - Release git-tempfile v1.0.6 ([`bd3f8ee`](https://github.com/Byron/gitoxide/commit/bd3f8ee28b51fa556a0f37c9bd62569f0ce7d49d))
    - Upgrade dashmap to 5.1.0 (with security fix) ([`2520945`](https://github.com/Byron/gitoxide/commit/25209454d3f7e27e12e8ddca92e43b1ff01d58aa))
    - Release git-tempfile v1.0.5 ([`d811154`](https://github.com/Byron/gitoxide/commit/d81115473cf04f5c3ae25b657b88c3f049451227))
    - Downgrade dashmap to 4.0 to avoid unsoundness. ([`d9451e8`](https://github.com/Byron/gitoxide/commit/d9451e8d7fc39c252042f9d2447061262c16ae7a))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - Prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Release git-tempfile v1.0.2 ([`310ea73`](https://github.com/Byron/gitoxide/commit/310ea7336e78fbedb2cefa1ecb773b25e6a77e0a))
    - Update changelogs once more… ([`d57d279`](https://github.com/Byron/gitoxide/commit/d57d279dc603cf450c151bbb6dc6c6505cc6da10))
    - Update changelogs retro-actively… ([`78cfe0a`](https://github.com/Byron/gitoxide/commit/78cfe0ac341c6c2257743d913884b50042078e6c))
    - Release git-tempfile v1.0.1 ([`295eb37`](https://github.com/Byron/gitoxide/commit/295eb374d104ac2775b9f864ef3234e2c5832b54))
    - [tempfile #195] adapt to Rust 1.55 ([`d9e71ac`](https://github.com/Byron/gitoxide/commit/d9e71acc5d619b5e78673da4fbb5a531c97ad6dd))
    - Thanks clippy ([`4701296`](https://github.com/Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [stability #171] prepare git-lock and git-tempfile release ([`3a1cf4d`](https://github.com/Byron/gitoxide/commit/3a1cf4d441b53c880b5c887916302a493ad28b41))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - Release git-tempfile v0.6.1 ([`eda952f`](https://github.com/Byron/gitoxide/commit/eda952f95e9ece78bbdbe6c26dd78f7ac5365d86))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com/Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - [pack #153] implement io traits for tempfiles ([`59d03d6`](https://github.com/Byron/gitoxide/commit/59d03d6133b301a19adfab212cf2c946110fc53b))
    - Clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - Thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [lock] support recoverable commits ([`b2217e7`](https://github.com/Byron/gitoxide/commit/b2217e7d25df9801354f702b0625d3168f8d3271))
    - [ref] support for persistence with recovery ([`d8b2d66`](https://github.com/Byron/gitoxide/commit/d8b2d661b9cf644b52950b9dedf8dbce0d348098))
    - [ref] refactor ([`a261b82`](https://github.com/Byron/gitoxide/commit/a261b82c1ee7ebdbbc82ce1c8286ca6a0f221cea))
    - [ref] allow reflogs to be created in place of empty directory trees ([`80a6e0e`](https://github.com/Byron/gitoxide/commit/80a6e0e1ff2321d9162e799d5fc0f457e7de4ade))
    - [tempfile] a way to delete empty dirs recursively ([`6025aa0`](https://github.com/Byron/gitoxide/commit/6025aa08d93cd5124c8df38c51b795b9c7d1c911))
    - Bump libc from 0.2.97 to 0.2.98 ([`caf6495`](https://github.com/Byron/gitoxide/commit/caf6495b08f77d7e4eaa058074693fffb5c5644b))
    - [tempfile] close a tempfile while keeping track of it ([`aa96ed1`](https://github.com/Byron/gitoxide/commit/aa96ed1776a615446b9864b1231f9f33805ab178))
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com/Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] add [must_use = "reason"] attribute where it matters ([`813c46b`](https://github.com/Byron/gitoxide/commit/813c46b1ac9ed5454c7832a6bad5a112f145b565))
    - [lock] refactor, remaining docs ([`956e69f`](https://github.com/Byron/gitoxide/commit/956e69fcb96085d96124b6c56d829607b36adf9f))
    - [lock] tests green ([`3706b26`](https://github.com/Byron/gitoxide/commit/3706b2669ebee5cd25a75a42d9b0a4a380707ee1))
    - [lock] cleanup signal handling even more… ([`9fb13d2`](https://github.com/Byron/gitoxide/commit/9fb13d27ccce5b0742ee9289fca891dbeb8a65de))
    - [lock] first tests and a lot of refactoring ([`3c34194`](https://github.com/Byron/gitoxide/commit/3c34194b6c0fd5ab22eb91081a563ba3bfa19110))
    - [lock] refactor; Marker is definitely not necessary… ([`6af84c9`](https://github.com/Byron/gitoxide/commit/6af84c92dbe049068be795ef4870fd830baf5384))
    - [lock] test what happens if multiple tempfiles are created ([`17942c7`](https://github.com/Byron/gitoxide/commit/17942c7960f25ad1f8f7fb2c94f251d39bb03c6e))
    - [lock] sketch API ([`f0e1427`](https://github.com/Byron/gitoxide/commit/f0e142734c1b09e6c4175b3c1b232d886449e280))
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com/Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - [tempfile] refactor ([`f3144a8`](https://github.com/Byron/gitoxide/commit/f3144a897b4e10742fef47fadd350b4e9ddf316a))
    - [tempfile] remaining tests ([`450db66`](https://github.com/Byron/gitoxide/commit/450db6609eb3dad10deed3f9769a21ae8c4b3be8))
    - [tempfile] refactor ([`3bafa7b`](https://github.com/Byron/gitoxide/commit/3bafa7b2a3cf8efd0769564026ce7b757cb8c09b))
    - [tempfile] implement 'closed' version of tempfile ([`d4bb61d`](https://github.com/Byron/gitoxide/commit/d4bb61dbc99b4270464d903978222d31c7e7dc5e))
    - [tempfile] refactor ([`4788222`](https://github.com/Byron/gitoxide/commit/4788222c28605118c03ce9f3a4dfccc26e7f7b60))
    - [tempfile] fix docs ([`3cd6712`](https://github.com/Byron/gitoxide/commit/3cd6712c22dae2e804573bca0b7a687c36066c29))
    - [tempfile] sketch of a closed registration with different types ([`db9bb11`](https://github.com/Byron/gitoxide/commit/db9bb11a3132961029e04f1cf761bfc3c96ec33d))
    - [tempfile] refactor ([`8a0ce64`](https://github.com/Byron/gitoxide/commit/8a0ce64baf5a3d77a08aa68c3245be8e7964be70))
    - [tempfile] typesafe diffentiation between writable tempfiles and closed ones ([`3b424b1`](https://github.com/Byron/gitoxide/commit/3b424b1cc071580303d37b7459e80036635eb4aa))
    - [tempfile] refactor ([`913f301`](https://github.com/Byron/gitoxide/commit/913f3014313fe0c03cd8f0af88944d8d514d89d9))
    - [tempfile] refactor ([`9384617`](https://github.com/Byron/gitoxide/commit/9384617dbe00dd59726cc418f23fb0a6e6dde415))
    - [tempfile] implement 'map' on tempfile to realize that 'close()' can't be done… ([`f4a1d33`](https://github.com/Byron/gitoxide/commit/f4a1d33e994e986604d18a85b7f85e1cea063dcf))
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com/Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - [tempfile] improve docs ([`d311b08`](https://github.com/Byron/gitoxide/commit/d311b082cdec323eb76363d986064fe771aa2bfd))
    - Thanks clippy ([`a0f9803`](https://github.com/Byron/gitoxide/commit/a0f9803533e5684cfed5ab50cd8145d071e978b2))
    - [tempfile] refactor ([`3a0f1ad`](https://github.com/Byron/gitoxide/commit/3a0f1ad0963c77a07f1984c39b127337463c030b))
    - [tempfile] refactor ([`9b8abd0`](https://github.com/Byron/gitoxide/commit/9b8abd0336d6ea1d7c088c78fc09fa935408896f))
    - [tempfile] cleanup control for named and unnamed tempfiles ([`0ef85a2`](https://github.com/Byron/gitoxide/commit/0ef85a247d60332ca232d6d993987c0b89e34466))
    - [tempfile] all remaining remove_dir tests I can think of ([`3e45e5f`](https://github.com/Byron/gitoxide/commit/3e45e5fef4f0bbd8736ae3f197f15813290fe8dc))
    - [tempfile] first bunch of tests for error handling and basic function of rmdir ([`ba41a15`](https://github.com/Byron/gitoxide/commit/ba41a15d874a2709ab92a8680d9e168ece7b4676))
    - [tempfile] quick impl of remove-dir iter without tests ([`bf48913`](https://github.com/Byron/gitoxide/commit/bf48913b3bc1a8c3ebaa230880f573ad01982f08))
    - [tempfile] refactor ([`9226dbe`](https://github.com/Byron/gitoxide/commit/9226dbe18127d7e85ba2779393cb7263e87cfbf8))
    - [tempfile] refactor ([`730b733`](https://github.com/Byron/gitoxide/commit/730b733a1a5b2c3911849eef6ffe0833e12daf73))
    - [tempfile] refactor ([`1da35ce`](https://github.com/Byron/gitoxide/commit/1da35ce045609296c189133ca439a74b550ccc6c))
    - [tempfile] improve Retries documentation; retries docs for remove_dir ([`e665a5f`](https://github.com/Byron/gitoxide/commit/e665a5fcd38c7002545079c63f0bf35dee11876d))
    - [tempfile] sketch how tempfile cleanup should be configured… ([`71acede`](https://github.com/Byron/gitoxide/commit/71acede9cba6fc222d0bde1a3fd0c232d3c877ab))
    - [tempfile] logic fixed, it's working ([`6ad4946`](https://github.com/Byron/gitoxide/commit/6ad4946e2ee603c69dad1da3e1e996cd1d4ca075))
    - [tempfile] better counting, but… ([`972113f`](https://github.com/Byron/gitoxide/commit/972113f1ea72674db61867b14f0eed0de498b310))
    - [tempfile] better retry counts ([`c7a35ca`](https://github.com/Byron/gitoxide/commit/c7a35caa295580a1b9d4f8b77eb8ded9d9c88703))
    - [tempfile] refactor ([`273d722`](https://github.com/Byron/gitoxide/commit/273d72276a73d49a633b9be1c66f1a2357dfcb0f))
    - [tempfile] a better way to count retries… ([`e110b97`](https://github.com/Byron/gitoxide/commit/e110b97b4925a10fa9a62576daf9f078508b6760))
    - [tempfile] trying to implement remove_dir really shows that… ([`1319b90`](https://github.com/Byron/gitoxide/commit/1319b908cc42ef5114d22957ebed9ed2ced11391))
    - [tempfile] docs for create_dir; frame for remove_dir; ([`aa6b6d1`](https://github.com/Byron/gitoxide/commit/aa6b6d14236c817ecc64390b110069c4c1340c03))
    - [tempfile] tests for automatic directory creation ([`3bd5cee`](https://github.com/Byron/gitoxide/commit/3bd5cee0dc0811ff3b1ab3d1a93e7dca8ae06b69))
    - [tempfile] refactor ([`d441312`](https://github.com/Byron/gitoxide/commit/d4413125c432da2e7ef809aca61973f5f55dbd5c))
    - [tempfile] use create_dir::all based on configuration… ([`156c021`](https://github.com/Byron/gitoxide/commit/156c021ac8aaabe8fed60ac4681f365c75e0e165))
    - [tempfile] remove todo ([`5a14ab6`](https://github.com/Byron/gitoxide/commit/5a14ab63555d6e3a58ce32b68d4b47287869b73f))
    - [tempfile] more information about error cases, too ([`7415141`](https://github.com/Byron/gitoxide/commit/74151415f0019a32b4759cf01873acb4102f2d1e))
    - [tempfile] refactor ([`ae0c97a`](https://github.com/Byron/gitoxide/commit/ae0c97a59d9cc56e19d3ce6fcc12b4564a66298a))
    - [tempfile] refactor ([`7c7658d`](https://github.com/Byron/gitoxide/commit/7c7658d3390fdf1b5348a482c71a9fb20a815cca))
    - [tempfile] test for racy directory creation… ([`c9073bf`](https://github.com/Byron/gitoxide/commit/c9073bf0d6dff3165cfd43733a602127b8835727))
    - [tempfile] verify existing files are handled correctly ([`28fee55`](https://github.com/Byron/gitoxide/commit/28fee552718cbbed067b8a16631aaa1080886e00))
    - [tempfile] a test for directory creation limits ([`584b4b7`](https://github.com/Byron/gitoxide/commit/584b4b7a1e6997594f1234d5feab1bc82a83b859))
    - [tempfile] limit retries for directory creation… ([`1536c7a`](https://github.com/Byron/gitoxide/commit/1536c7a58f1da4b80e83f1169b3f865f12a3d9a2))
    - [tempfile] refactor ([`fa7b8e9`](https://github.com/Byron/gitoxide/commit/fa7b8e99d2613297127b044605a2314b878d3ab9))
    - [tempfile] handle interrupts and assure there is an end to it ([`dc0afbd`](https://github.com/Byron/gitoxide/commit/dc0afbdf08c44237b6749426ebacbded6cf8a647))
    - [tempfile] first recursive directory creation ([`b01faa9`](https://github.com/Byron/gitoxide/commit/b01faa9fdc371c1226978e32a3c71dbf3be600ec))
    - [tempfile] refactor ([`7b59392`](https://github.com/Byron/gitoxide/commit/7b59392fec4c80eddd9f82271eb1a5671e44aa5a))
    - [tempfile] another test ([`9e4834d`](https://github.com/Byron/gitoxide/commit/9e4834df1142fd0ffdbf5ffba1aed63bc67b66b3))
    - [tempfile] first sketch of iterator based create directory all… ([`314693c`](https://github.com/Byron/gitoxide/commit/314693c6a4577f0b2b00274a55ec99e87c17918f))
    - [lock] frame for git-lock crate ([`e6bc87d`](https://github.com/Byron/gitoxide/commit/e6bc87d77f9b397b25694f58d347de2fc38bf71d))
    - [tempfile] add journey test to validate operation on process level ([`2d1efd4`](https://github.com/Byron/gitoxide/commit/2d1efd4915d66890b1132d5b39e08027a83047ba))
    - [tempfile] more docs ([`d0c5e6b`](https://github.com/Byron/gitoxide/commit/d0c5e6b96f27d7ae708e7182b4cd5dbaceecd3cd))
    - Refactor ([`e0b7f69`](https://github.com/Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [tempfile] clean cargo manifest ([`d43f514`](https://github.com/Byron/gitoxide/commit/d43f51438937d5bdd2bb2e02c355dcd4ee2b8680))
    - [tempfile] fix windows for good ([`3192a47`](https://github.com/Byron/gitoxide/commit/3192a47b730245f2656ccf8cd82394ec31e13126))
    - [tempfile] fix windows build (hopefully) ([`6c1df66`](https://github.com/Byron/gitoxide/commit/6c1df667031084a9e6fe9676534f80aae9a95789))
    - [tempfile] refactor ([`4a45df0`](https://github.com/Byron/gitoxide/commit/4a45df02340b55d34534be89934d2201dda261f9))
    - [tempfile] prepare release ([`c0f7fde`](https://github.com/Byron/gitoxide/commit/c0f7fde70b28526ad52dce9e2314a25af1531689))
    - [tempfile] an example to show off signal handlers ([`f325e69`](https://github.com/Byron/gitoxide/commit/f325e696c64e3f61f64c0a8d8c4e8af38104a713))
    - [tempfile] remaining docs ([`d334dc0`](https://github.com/Byron/gitoxide/commit/d334dc004d8b8eea5b586c6ada173d28d380ccce))
    - [tempfile] restore original signal handler behaviour. ([`9f91dd8`](https://github.com/Byron/gitoxide/commit/9f91dd8e95e1e51a8e0a4f7ba45628b3d93fc5de))
    - [tempfile] process-id filter on deletion to support forks ([`611056f`](https://github.com/Byron/gitoxide/commit/611056f431dc793684efad668d40b93b1cfec21e))
    - [tempfile] implement handler correctly, probably. ([`8cb0bbc`](https://github.com/Byron/gitoxide/commit/8cb0bbcf2d022401886071f9c91498977cea185c))
    - [tempfile] remove tempfiles on shutdown, but… ([`954b760`](https://github.com/Byron/gitoxide/commit/954b76029e4d9e331738137ec2c9804b0e06a890))
    - [tempfile] switch to dashmap as slab ([`6164719`](https://github.com/Byron/gitoxide/commit/61647195ce8fd0be1b3b63f19e8aaec392f33f19))
    - [tempfile] a more realistic slab test shows the index can get quite high. ([`915f14c`](https://github.com/Byron/gitoxide/commit/915f14c41145dbd3f63bd798e6d0cc18d51fef6f))
    - [tempfile] first step towards clearing tempfiles… ([`b0e0cee`](https://github.com/Byron/gitoxide/commit/b0e0cee866b643f9f9e4ebdc495abed5f5c6abf9))
    - [tempfile] precisely named tempfiles ([`edc74f0`](https://github.com/Byron/gitoxide/commit/edc74f0e8f04f45661d4909bb3e6c62f4ac1b453))
    - [tempfile] `take()` method ([`d377397`](https://github.com/Byron/gitoxide/commit/d3773976b86ad294528997104b9cfa0c803f9d6a))
    - [tempfile] basic operation of a tempfile ([`a692950`](https://github.com/Byron/gitoxide/commit/a692950ae7c32ed9dd040c0aebde494ef3029a30))
    - [tempfile] show that slabs can store a lot actually ([`0cc5b33`](https://github.com/Byron/gitoxide/commit/0cc5b33f0e421ed761e5c350fb4d3ad85ef3e884))
    - [tempfile] initial docs as there is a lot to consider ([`9dffc2b`](https://github.com/Byron/gitoxide/commit/9dffc2b918178650a3b40adfcc35730e48f46951))
    - [tempfile] crate frame ([`1b04c39`](https://github.com/Byron/gitoxide/commit/1b04c39030b436fb6850fbfa0c39a4fed7df727c))
</details>

## 3.0.1 (2023-01-10)

A maintenance release without user-facing changes.

## 3.0.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 2.0.6 (2022-11-06)

A maintenance release without user-facing changes.

## 2.0.5 (2022-09-20)

Maintenance release without observable changes.

## 2.0.4 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 2.0.3 (2022-08-17)

A maintenance release without user facing changes.

## 2.0.2 (2022-07-22)

This is a maintenance release with no functional changes.

## 2.0.1 (2022-04-03)

A maintenance release without any changes on the surface.

## 2.0.0 (2022-02-17)

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default.
   
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

## 1.0.6 (2022-02-07)

<csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/>

Fixes a potential deadlock in in an interrupt handler attempting to cleanup tempfiles.

### Chore

 - <csr-id-25209454d3f7e27e12e8ddca92e43b1ff01d58aa/> upgrade dashmap to 5.1.0 (with security fix)

### Bug Fixes

 - <csr-id-81ed5f5e7a3634f0fab681ca59e40099f0118f75/> Assure interrupt based tempfile cleanup can't deadlock.
   
   We do this by using the `try_entry()` API provided by the most recent
   dashmap, at the cost of potentially trying to access a lot of indices
   that don't exist in the map anymore. The cost of this are expected
   to be low though especially since interrupts are the uncommmon case.
   
   A side-effect of this is that tempfiles that are currently being
   removed for writing to them, for example, won't be cleaned up.
   
   It will be up to the code running after the interrupt, if the case
   at all, to deal with the tempfile, which is what it does anyway.

## 1.0.5 (2022-02-01)

### Bug Fixes

 - <csr-id-d9451e8d7fc39c252042f9d2447061262c16ae7a/> downgrade dashmap to 4.0 to avoid unsoundness.
   See https://github.com/xacrimon/dashmap/issues/167 for tracking
   progress on resolving the issue.

## 1.0.4 (2022-01-23)

A maintenance release thanks to upgraded dependencies.

## v1.0.3 (2021-10-15)

This release contains no functional changes, but a more useful changelog.

## v1.0.2 (2021-09-10)

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on in its tests.

## v1.0.1 (2021-09-10)

## v1.0.0 (2021-08-25)

- initial release

## v1.0.0 (2021-08-25)

## v0.6.1 (2021-08-17)

## v0.6.0 (2021-08-11)

## v0.5.0 (2021-08-10)

## v0.4.0 (2021-06-23)

## v0.3.0 (2021-06-20)

## v0.2.0 (2021-06-19)

## v0.1.0 (2021-06-15)

