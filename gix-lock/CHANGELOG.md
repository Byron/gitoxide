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

<csr-id-ae298262add18049969f9451ed2a704d248eed41/>

### New Features

 - <csr-id-fda433757e3cd6645edfd2e18dcd2e67d1ef930f/> Make it easy to configure lock acquisition failure from Duration.
   Now `Duration` can be turned into `acquire::Fail`.

### Other

 - <csr-id-ae298262add18049969f9451ed2a704d248eed41/> make clear that locks can leak and what that means on acquiring a lock

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 16 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1026](https://github.com/Byron/gitoxide/issues/1026)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1026](https://github.com/Byron/gitoxide/issues/1026)**
    - Make clear that locks can leak and what that means on acquiring a lock ([`ae29826`](https://github.com/Byron/gitoxide/commit/ae298262add18049969f9451ed2a704d248eed41))
 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
    - Merge branch 'gix-lock-improvements' ([`b009db0`](https://github.com/Byron/gitoxide/commit/b009db018df435a8c57848dc4cdc52d4278a6d9d))
    - Make it easy to configure lock acquisition failure from Duration. ([`fda4337`](https://github.com/Byron/gitoxide/commit/fda433757e3cd6645edfd2e18dcd2e67d1ef930f))
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

 - 8 commits contributed to the release over the course of 7 calendar days.
 - 23 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Prepare more changelogs ([`13a497f`](https://github.com/Byron/gitoxide/commit/13a497f679943908793e743810f1b0b2501ba3b6))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
    - Thanks clippy ([`908468b`](https://github.com/Byron/gitoxide/commit/908468bf9c2e7593b4902c50c2df72bcc9d2af78))
</details>

## 7.0.1 (2023-06-29)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-lock v7.0.1, gix v0.48.0 ([`5ce81ef`](https://github.com/Byron/gitoxide/commit/5ce81ef16210f0b0b72dfd5710a064ccda96ac1c))
    - Prepare changelog prior to release ([`fcadd20`](https://github.com/Byron/gitoxide/commit/fcadd20205f51ed9e9acbabfa5f0a284d11e1e94))
    - Unify `gix-utils` version... ([`6b6eaf9`](https://github.com/Byron/gitoxide/commit/6b6eaf9401d78684350a8eb0ec337fda7a5ae19b))
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

## 5.0.1 (2023-04-26)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 46 calendar days.
 - 46 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - Thanks clippy ([`14e64e7`](https://github.com/Byron/gitoxide/commit/14e64e74649cfb1f2f99da87015939af98fae5c8))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Create new `gix-fs` crate to consolidate all filesystem utilities ([`f8cc33c`](https://github.com/Byron/gitoxide/commit/f8cc33cb372dd2b4bbe4a09cf4f64916681ab1dd))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## 5.0.0 (2023-03-10)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
</details>

## 4.0.0 (2023-03-01)

<csr-id-d08364ce7eb00b052202b623dad92cfda38b812f/>

### Chore

 - <csr-id-d08364ce7eb00b052202b623dad92cfda38b812f/> replace `quick-error` with `thiserror`
   This increases the compile time of the crate alone if there is no proc-macro
   in the dependency tree, but will ever so slightly improve compile times for `gix`
   as a whole.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v4.1.0, gix-lock v4.0.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0, safety bump 6 crates ([`ea9fd1d`](https://github.com/Byron/gitoxide/commit/ea9fd1d9b60e1e9e17042e9e37c06525823c40a5))
    - Merge branch 'adjustments-for-cargo' ([`d686d94`](https://github.com/Byron/gitoxide/commit/d686d94e1030a8591ba074757d56927a346c8351))
    - Adapt to changes in `gix-tempfile` ([`bfcd1e3`](https://github.com/Byron/gitoxide/commit/bfcd1e3a8f2d05d0d6d1f5cf06d369ac9e6e4cdf))
    - Replace `quick-error` with `thiserror` ([`d08364c`](https://github.com/Byron/gitoxide/commit/d08364ce7eb00b052202b623dad92cfda38b812f))
    - Prepare for git-tempfile release ([`56c005b`](https://github.com/Byron/gitoxide/commit/56c005b13c44376f71e61781e73c0bf93416d0e4))
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

 - <csr-id-73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0/> Interpret `PermissionDenied` as reason to retry on lock acquisition failure.
   Evidence from CI suggests that on windows 'AlreadyExists' isn't the
   common error code. Instead, maybe due to racyness, it can also emit
   PermissionDenied errors which we now handle specifically.

### New Features

 - <csr-id-f9bb71b984045a069f7caf0c650ef24bd3cc0be1/> add `backoff` for an exponential backoff iterator.
   It's used for computing the backoff on the lock itself, but is certainly
   useful in other occasions as well.
 - <csr-id-a789490fbbba1f44d79d57c8ba74bf40c600faf2/> export git-tempfile in the root
   This allows access to signal handler setup for example, which can be
   useful in some situations.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 149 commits contributed to the release over the course of 610 calendar days.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#384](https://github.com/Byron/gitoxide/issues/384), [#386](https://github.com/Byron/gitoxide/issues/386), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - Deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - Regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - Greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Refactor ([`287f47c`](https://github.com/Byron/gitoxide/commit/287f47c17a52be382f0866abaa0db6b33a09f560))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - Export git-tempfile in the root ([`a789490`](https://github.com/Byron/gitoxide/commit/a789490fbbba1f44d79d57c8ba74bf40c600faf2))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - Interpret `PermissionDenied` as reason to retry on lock acquisition failure. ([`73ecb40`](https://github.com/Byron/gitoxide/commit/73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
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
    - Rename `git-lock` to `gix-lock` ([`0422433`](https://github.com/Byron/gitoxide/commit/04224336b910fbcb8c32ef61edb345c53e945425))
    - Adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - Adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - Adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - Adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - Adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - Adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - Adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - Adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - Adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - Adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - Prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'main' into http-config ([`f4ff821`](https://github.com/Byron/gitoxide/commit/f4ff821fd4233dd1dc1a449af4d4600becf3b4ac))
    - Merge branch 'async-fetch' ([`0c9c48b`](https://github.com/Byron/gitoxide/commit/0c9c48b3b91a1396eb1796f288a2cb10380d1f14))
    - Add `backoff` for an exponential backoff iterator. ([`f9bb71b`](https://github.com/Byron/gitoxide/commit/f9bb71b984045a069f7caf0c650ef24bd3cc0be1))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'main' into git-sec ([`2fe70f9`](https://github.com/Byron/gitoxide/commit/2fe70f96cfb68e108637ce78f8edda2eb4e2e61a))
    - Merge branch 'main' into repo-status ([`8122c5c`](https://github.com/Byron/gitoxide/commit/8122c5c8a122e5297832f96c07a6fb1a5df02620))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [stability #171] prepare git-lock and git-tempfile release ([`3a1cf4d`](https://github.com/Byron/gitoxide/commit/3a1cf4d441b53c880b5c887916302a493ad28b41))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - Release git-lock v0.3.2 ([`a5ea2e7`](https://github.com/Byron/gitoxide/commit/a5ea2e7ba4bac484fad0672811ecd303214d8425))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.3.1 ([`168f5a0`](https://github.com/Byron/gitoxide/commit/168f5a0c5e9fdaf56789c884853569c5b850d7fa))
    - [lock #154] add io impls for `File` ([`be62a8b`](https://github.com/Byron/gitoxide/commit/be62a8bbe6c643017816ea23f15f04d38eb98b46))
    - (cargo-release) version 0.3.0 ([`263088b`](https://github.com/Byron/gitoxide/commit/263088b3faaccd9edae8c21dfc7d39b191d76207))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.2.0 ([`20d8e27`](https://github.com/Byron/gitoxide/commit/20d8e27dd4e93ae2234a3fe19b5f1511365eee2e))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - Bump fastrand from 1.4.1 to 1.5.0 ([`b138b43`](https://github.com/Byron/gitoxide/commit/b138b437b9fb57f5df8bd582ac288ffac00df8cd))
    - [ref] fix docs ([`536555d`](https://github.com/Byron/gitoxide/commit/536555d66705175d83ba411392b049c8ed736ee9))
    - [ref] fix build ([`b4dcdfc`](https://github.com/Byron/gitoxide/commit/b4dcdfc9b2f2edcbcf9fb144d1f97e9a841463ad))
    - [lock] support recoverable commits ([`b2217e7`](https://github.com/Byron/gitoxide/commit/b2217e7d25df9801354f702b0625d3168f8d3271))
    - [lock] refactor ([`48861b2`](https://github.com/Byron/gitoxide/commit/48861b23406bbeee33081b32b2a528533997910d))
    - [lock] FAIL: trying to make peristence recoverable… ([`1fcdd1e`](https://github.com/Byron/gitoxide/commit/1fcdd1e5da51fe8f15e9bfab86b22a82ef936bcc))
    - [ref] try fix windows, once again ([`95e74dd`](https://github.com/Byron/gitoxide/commit/95e74dd9f1510fd288f281beea3f560319ad323d))
    - [lock] access to the locked resource path ([`797bafa`](https://github.com/Byron/gitoxide/commit/797bafaa0a83449be2acb647f875a67476548afe))
    - [lock] allow accessing the lock file path more easily ([`b808b00`](https://github.com/Byron/gitoxide/commit/b808b00f62f587679ffbfe9e465b33419cdd5879))
    - [lock] Fix handling of .lock extension on files without extension ([`64ac60d`](https://github.com/Byron/gitoxide/commit/64ac60d82db9bd81ceed2f0f1167c196de340ffe))
    - [lock] close file lock and commit markers ([`f700821`](https://github.com/Byron/gitoxide/commit/f7008214497d3a80ffcd62840854f3db65c563a9))
    - [lock] Marker commit with runtime check for protection ([`b747814`](https://github.com/Byron/gitoxide/commit/b74781403716e5156dad06ecf5cf83ff72262161))
    - (cargo-release) version 0.1.0 ([`60d48b0`](https://github.com/Byron/gitoxide/commit/60d48b0d008ca3b8612fd7b7e70b89b4bdb52b2c))
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com/Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] capture amount of attempts taken when obtaining a lock ([`7fafa3e`](https://github.com/Byron/gitoxide/commit/7fafa3e70a26c040e2f50c477df314eccf90a42f))
    - [lock] validate error message when waiting for some tim ([`34d3c5a`](https://github.com/Byron/gitoxide/commit/34d3c5a357c33e6734714cbbe8f6065b59f1215e))
    - [lock] the first test for lock failure (immediate mode) ([`2d67a0e`](https://github.com/Byron/gitoxide/commit/2d67a0e3948e717df0cc4de58686f5efa3645be9))
    - [lock] add [must_use = "reason"] attribute where it matters ([`813c46b`](https://github.com/Byron/gitoxide/commit/813c46b1ac9ed5454c7832a6bad5a112f145b565))
    - Thanks clippy ([`29782e8`](https://github.com/Byron/gitoxide/commit/29782e8c32d0b062e33a77bd4d639438e88b91b4))
    - [lock] lock acquire with backoff, but without test for now ([`bb2ba81`](https://github.com/Byron/gitoxide/commit/bb2ba81b2723cca11b4d98efa176ce5d184ab594))
    - [lock] prevent flakyness due to rounding or something ([`6f8fbcc`](https://github.com/Byron/gitoxide/commit/6f8fbcc197446068b680ea4c41e79bfc6356fcff))
    - [lock] refactor ([`ddc2170`](https://github.com/Byron/gitoxide/commit/ddc2170498d3822ebf6fc35e2e4b336f7af6d9f3))
    - [lock] remaining test for everything proper exponential backoff needs ([`368d994`](https://github.com/Byron/gitoxide/commit/368d9944b2b3379b0d01be93aea02cae25d1c867))
    - [lock] support for randomization ([`220eb99`](https://github.com/Byron/gitoxide/commit/220eb9945b2006f79b82d80eaabd09b5e291953b))
    - [lock] better overshoot test for exponential backoff ([`62c17d8`](https://github.com/Byron/gitoxide/commit/62c17d8721a8d797fe7475ea36a90931e835ebc4))
    - [lock] a sketch of exponential backoff, without rnadomization ([`55670b4`](https://github.com/Byron/gitoxide/commit/55670b458d248f6249f0031ec73c5eb4f72e36f7))
    - [lock] refactor, remaining docs ([`956e69f`](https://github.com/Byron/gitoxide/commit/956e69fcb96085d96124b6c56d829607b36adf9f))
    - [lock] tests green ([`3706b26`](https://github.com/Byron/gitoxide/commit/3706b2669ebee5cd25a75a42d9b0a4a380707ee1))
    - [lock] creation of lockfiles, with immediate failure mode ([`fda7da8`](https://github.com/Byron/gitoxide/commit/fda7da8ec43f7999effdfd76791a6541f8c52338))
    - [lock] first tests and a lot of refactoring ([`3c34194`](https://github.com/Byron/gitoxide/commit/3c34194b6c0fd5ab22eb91081a563ba3bfa19110))
    - [lock] even more sketched out API ([`0dc88c9`](https://github.com/Byron/gitoxide/commit/0dc88c9c42690a3e8320fe5b701c51eed8747118))
    - [lock] refactor; Marker is definitely not necessary… ([`6af84c9`](https://github.com/Byron/gitoxide/commit/6af84c92dbe049068be795ef4870fd830baf5384))
    - [lock] test what happens if multiple tempfiles are created ([`17942c7`](https://github.com/Byron/gitoxide/commit/17942c7960f25ad1f8f7fb2c94f251d39bb03c6e))
    - [lock] sketch API ([`f0e1427`](https://github.com/Byron/gitoxide/commit/f0e142734c1b09e6c4175b3c1b232d886449e280))
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com/Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com/Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - [lock] frame for git-lock crate ([`e6bc87d`](https://github.com/Byron/gitoxide/commit/e6bc87d77f9b397b25694f58d347de2fc38bf71d))
</details>

## 3.0.1 (2022-12-19)

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

## 2.2.0 (2022-11-17)

### New Features

 - <csr-id-f9bb71b984045a069f7caf0c650ef24bd3cc0be1/> add `backoff` for an exponential backoff iterator.
   It's used for computing the backoff on the lock itself, but is certainly
   useful in other occasions as well.

## 2.1.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 2.1.0 (2022-05-18)

### New Features

 - <csr-id-a789490fbbba1f44d79d57c8ba74bf40c600faf2/> export gix-tempfile in the root
   This allows access to signal handler setup for example, which can be
   useful in some situations.

### Bug Fixes

 - <csr-id-73ecb40bb20f9c3fe7341f4ff1b0b81ac09b9ae0/> Interpret `PermissionDenied` as reason to retry on lock acquisition failure.
   Evidence from CI suggests that on windows 'AlreadyExists' isn't the
   common error code. Instead, maybe due to racyness, it can also emit
   PermissionDenied errors which we now handle specifically.

## 2.0.0 (2022-04-03)

A maintenance release due to mildly breaking changes in `gix-tempfile`.

## v1.0.1 (2021-10-15)

This release contains no functional changes, but a more useful changelog.

## v1.0.0 (2021-08-25)

- initial release

## v0.3.2 (2021-08-17)

## v0.3.1 (2021-08-11)

## v0.3.0 (2021-08-11)

## v0.2.0 (2021-08-10)

## v0.1.0 (2021-06-23)

## v0.0.0 (2021-06-17)

