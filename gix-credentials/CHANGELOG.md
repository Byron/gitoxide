# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.21.0 (2023-10-12)

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

## 0.20.0 (2023-09-24)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 16 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
</details>

## 0.19.0 (2023-09-08)

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

## 0.18.0 (2023-08-22)

<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>

### Chore

 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 18 calendar days.
 - 30 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0 ([`6c62e74`](https://github.com/Byron/gitoxide/commit/6c62e748240ac0980fc23fdf30f8477dea8b9bc3))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.17.1 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.17.0 (2023-07-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 19 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
</details>

## 0.16.1 (2023-06-29)

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
    - Release gix-glob v0.9.1, gix-attributes v0.14.1, gix-config-value v0.12.3, gix-ref v0.32.1, gix-sec v0.8.3, gix-config v0.25.1, gix-url v0.20.1, gix-credentials v0.16.1, gix-discover v0.21.1, gix-ignore v0.4.1, gix-pack v0.39.1, gix-odb v0.49.1, gix-worktree v0.21.1, gix v0.48.0 ([`69c6a36`](https://github.com/Byron/gitoxide/commit/69c6a36ba14cbef129deebda9fd8870005fefa17))
    - Prepare changelogs prior to release ([`c143cf4`](https://github.com/Byron/gitoxide/commit/c143cf48ee1885467e3e9262a3f8823a1247bfe0))
    - Align usage of `gix-path` across all crates ([`73c1292`](https://github.com/Byron/gitoxide/commit/73c1292be393986c4a1adde1400abf551e850da0))
</details>

## 0.16.0 (2023-06-22)

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

## 0.15.0 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 12 calendar days.
 - 40 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include custom clippy settings ([`b057500`](https://github.com/Byron/gitoxide/commit/b057500dd3e6b75be3ebcd258cda0b946bedd9e1))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
</details>

## 0.14.0 (2023-04-27)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.8.0, gix-glob v0.7.0, gix-attributes v0.12.0, gix-config-value v0.12.0, gix-ref v0.29.0, gix-sec v0.8.0, gix-config v0.22.0, gix-prompt v0.5.0, gix-url v0.18.0, gix-credentials v0.14.0, gix-discover v0.18.0, gix-ignore v0.2.0, gix-pack v0.35.0, gix-odb v0.45.0, gix-transport v0.31.0, gix-protocol v0.32.0, gix-refspec v0.10.1, gix-worktree v0.17.0, gix v0.44.1 ([`7ebc9f7`](https://github.com/Byron/gitoxide/commit/7ebc9f734ec4371dd27daa568c0244185bb49eb5))
    - Prepare changelogs prior to release ([`0135158`](https://github.com/Byron/gitoxide/commit/013515897215400539bfd53c25548bd054186ba6))
    - Bump gix-path v0.8.0, safety bump 20 crates (gix set to 0.44.1 manually) ([`43ebaf2`](https://github.com/Byron/gitoxide/commit/43ebaf267557218865862538ffc7bdf00558492f))
</details>

## 0.13.0 (2023-04-26)

### New Features (BREAKING)

 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 46 calendar days.
 - 46 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## 0.12.0 (2023-03-10)

### Bug Fixes

 - <csr-id-ee1e2696331f48bbaf90377402081a6bc69d4a3c/> it's no error if credential helpers don't receive context as input.
   They might ignore all input and output hardcoded credentials for good reason,
   so we don't validate writes to the helper similarly to how git does it.
   
   Note that we also don't care if credentials are stored or erased for similar reasons.
   I would happily make the case that we should be more adamant about error handling in that
   case, but typically we check error codes and that's the only indicator for success or failure.
   
   Related to https://github.com/rust-lang/cargo/issues/11821

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
    - It's no error if credential helpers don't receive context as input. ([`ee1e269`](https://github.com/Byron/gitoxide/commit/ee1e2696331f48bbaf90377402081a6bc69d4a3c))
</details>

## 0.11.0 (2023-03-04)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-attributes v0.10.0, gix-ref v0.26.0, gix-config v0.18.0, gix-url v0.15.0, gix-credentials v0.11.0, gix-discover v0.15.0, gix-index v0.14.0, gix-mailmap v0.11.0, gix-odb v0.42.0, gix-transport v0.27.0, gix-protocol v0.28.0, gix-revision v0.12.0, gix-refspec v0.9.0, gix-worktree v0.14.0, gix v0.39.0 ([`93e75fe`](https://github.com/Byron/gitoxide/commit/93e75fed454ed8b342231bde4638db90e407ce52))
    - Prepare changelogs prior to release ([`895e482`](https://github.com/Byron/gitoxide/commit/895e482badf01e953bb9144001eebd5e1b1c4d84))
    - Release gix-features v0.28.0, gix-actor v0.19.0, gix-object v0.28.0, gix-diff v0.28.0, gix-traverse v0.24.0, gix-pack v0.32.0, safety bump 20 crates ([`0f411e9`](https://github.com/Byron/gitoxide/commit/0f411e93ec812592bb9d3a52b751399dd86f76f7))
</details>

## 0.10.0 (2023-03-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v4.1.0, gix-lock v4.0.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0, safety bump 6 crates ([`ea9fd1d`](https://github.com/Byron/gitoxide/commit/ea9fd1d9b60e1e9e17042e9e37c06525823c40a5))
    - Release gix-features v0.27.0, gix-actor v0.18.0, gix-quote v0.4.3, gix-attributes v0.9.0, gix-object v0.27.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0 ([`e6cc618`](https://github.com/Byron/gitoxide/commit/e6cc6184a7a49dbc2503c1c1bdd3688ca5cec5fe))
    - Adjust manifests prior to release ([`addd789`](https://github.com/Byron/gitoxide/commit/addd78958fdd1e54eb702854e96079539d01965a))
    - Prepare changelogs prior to release ([`94c99c7`](https://github.com/Byron/gitoxide/commit/94c99c71520f33269cc8dbc26f82a74747cc7e16))
</details>

## 0.9.2 (2023-02-20)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
</details>

## 0.9.1 (2023-02-17)

<csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/>
<csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/>
<csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Other (BREAKING)

 - <csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/> `helper::Kind` to `program::Kind`
 - <csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/> `helper::(encode|decode)_message(…)` to `::message::(encode|decode)(…)`

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

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-783a1a7dfd64a64fa765fa3d3ef41b1e954413ee/> rename `git()` to `builtin()`
 - <csr-id-bfa2545883daf8c4d9e97d2fc91c9328d73ab0eb/> rename `Program::Custom*` variants to `Program::External*`
   It's more true to what it is.
 - <csr-id-811985aba024385465104ed826a9989961555201/> differentiate between top-level functions and those which are invoked
   That way it's easier to use as it can assure an account was actually
   provided.
 - <csr-id-49b9bd501f33f1e10ce0180e814b84e293bd3898/> invoke::Outcome can now represent partial identities
   That way these can be assembled by multiple helpers called in a row.
 - <csr-id-4b7d0b6d2c43cac9823885bc69510cc4bb6a3f00/> move `helper::(Next)Action` into `helper::invoke::` module
   These are only relevant for invoke, hence the change.
 - <csr-id-ddd53988a6d5da17fc65451a059bed1bfa2eb454/> rename `helper::NextAction` variants to `store` and `erase`
 - <csr-id-2073b583dc2bd83b800584edda6592bb71a01538/> rename `helper::Action` variants to 'Get', 'Store', 'Erase'
   It's more obvious what it does and is more typical for what credentials
   helpers do.
 - <csr-id-9c6f024f838d866645937a67cd67dffb8be17259/> Use `helper::Context` in `helper::Action::Fill()`
   That way additional information, like from configuration, can be passed
   as well.
 - <csr-id-71f651930e6fd53e3c3f9e82dfd95828e4981d92/> move `helper::invoke()` related types into `helper::invoke` module.
   Also allow to pass arbitrary bytes (more or less) as context by not
   forcing it all into a string. Values can now be everything, which
   helps with passing paths or other values.
 - <csr-id-4c1a1a28558c4f8d084b8046afd5d87a11fd25b7/> use `thiserror` instead of `quickerror`
 - <csr-id-081934ca4452e550cf2663026905bce67253af81/> hide `helper::action()` in favor of single path via `helper()`
 - <csr-id-12589cc6f08e4d7aabae30bcdadaa0c2b4850229/> adapt to changes in `git-url` and use `BString` to represent URLs.
   They can contain paths, which is why `String` can't repsent a URL
   losslessly.
   
   For HTTP urls these are ultimately UTF-8 strings though.

### Other

 - <csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/> :main::Action::as_str()`

### New Features

 - <csr-id-6b375d3061f6d307101199eff1ecdfdeb6769965/> `helper::Cascade::query_user_only()` can avoid to ask for the password if the transport wouldn't use it.
   That way, `ssh` while using ssh programs can query the user to try next.
 - <csr-id-15f1afccb7ed0ebaf217cbbdd58e6ae651a31e42/> `protocol::Context::to_bstring()`, and use it in `example/git-credential-lite`
 - <csr-id-b1d528ae60001ae51dd89b29c26ea505eacbef45/> an example implementing a custom credential helper program
 - <csr-id-eaff67c14366f149ccca346acb46af12531a24e6/> `helper::main` to easily create credential helper implementations
 - <csr-id-a253d30093122e37b5560ff86a7888f8062c7014/> add `helper::Action::get_for_url(…)`
 - <csr-id-64bc2ec666dacba486bd1de2fbd95f97f2efc7a5/> `helper::invoke(helper, action, context)` function that allows for more flexible helper invocation
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-3d339d5c24630fac0192b5d27f9b1cbd94418730/> use `git-sec::Identity` type
   It's shared across crates.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Changed

 - <csr-id-43656d5ce84834c847cf8650d4c486c634f209b6/> use `git-config-value` crate

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 222 commits contributed to the release over the course of 308 calendar days.
 - 30 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#386](https://github.com/Byron/gitoxide/issues/386), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - More details for path permissions ([`ca26659`](https://github.com/Byron/gitoxide/commit/ca26659eb870c8e947962fe0647a07d01b3e95e4))
    - Adapt to changes in git-sec ([`c5e2346`](https://github.com/Byron/gitoxide/commit/c5e2346cee53019b1b321e45cf080b210e60bb7a))
    - Use `git-sec::Identity` type ([`3d339d5`](https://github.com/Byron/gitoxide/commit/3d339d5c24630fac0192b5d27f9b1cbd94418730))
    - Fill git-credentials with existing impleemntation ([`6016c22`](https://github.com/Byron/gitoxide/commit/6016c2252aea6892a813b7dc1b0c870a156b3cfd))
    - Add frame for git-credentials crate ([`be7a9cf`](https://github.com/Byron/gitoxide/commit/be7a9cf776f958ac7228457bb4e1415f86f8e575))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - :main::Action::as_str()` ([`d95029e`](https://github.com/Byron/gitoxide/commit/d95029eac0e9179a7cd730d1d60a08696584bfd1))
    - `protocol::Context::to_bstring()`, and use it in `example/git-credential-lite` ([`15f1afc`](https://github.com/Byron/gitoxide/commit/15f1afccb7ed0ebaf217cbbdd58e6ae651a31e42))
    - Assure that protocol::Context::to_url() never shows passwords ([`e9f4d40`](https://github.com/Byron/gitoxide/commit/e9f4d40b6f04414c04f153f74f13d2e1fe89e43a))
    - Make sure the helper::Cascade never sends the URL to helper programs ([`9059de8`](https://github.com/Byron/gitoxide/commit/9059de825d310c2c28f28d4009b09115acccc2bf))
    - Fix docs ([`9a5ec7b`](https://github.com/Byron/gitoxide/commit/9a5ec7bd8b23bbef2c21de07638681160a7bbdee))
    - Move `program::Cascade` to `helper::Cascade` which is what it is ([`6149a14`](https://github.com/Byron/gitoxide/commit/6149a14af1742bcfc30bfbe65656b411c6f771c9))
    - An example on how to invoke the git credential helper driver ([`be0f834`](https://github.com/Byron/gitoxide/commit/be0f83411371e445beceabfcc6e0458eedccf31a))
    - Context has to serialize url or else the builtin credential helper may not work. ([`87ae404`](https://github.com/Byron/gitoxide/commit/87ae40493cc0dbe11e5de5fd21e2391caa7161db))
    - Credential context won't send url and quit fields to helpers ([`337a53b`](https://github.com/Byron/gitoxide/commit/337a53b945da26e253c9ba1c72b623d6a06d2e7c))
    - Cascade supports `use_http_path` and respects it when setting up the context ([`959c0bd`](https://github.com/Byron/gitoxide/commit/959c0bdfb6a634f590969f2c26d13ff8c05a4bb8))
    - Make it less easy to start a cascade with platform_defaults() ([`4b5d63f`](https://github.com/Byron/gitoxide/commit/4b5d63f7e0ea6bc43f54c95dd30f823ead9fa1a2))
    - Make clearer what platform builtins actually are ([`9788e30`](https://github.com/Byron/gitoxide/commit/9788e3070edc5c1d84099a2fc5fa9262604170e7))
    - Credential-cascade now passes on prompt options ([`baad8a0`](https://github.com/Byron/gitoxide/commit/baad8a077ffd556cb29da93fb0081b245f4663ff))
    - Refactor ([`c8f1b41`](https://github.com/Byron/gitoxide/commit/c8f1b41408f2ace5b01292ef95189b9e66ab4d8e))
    - Always compile prompting support in ([`bd0ea68`](https://github.com/Byron/gitoxide/commit/bd0ea68225a73fb83c9fc1b8594fc6ad288a77a9))
    - Set version of git-prompt to 0.1 and turn prompting on ([`7657693`](https://github.com/Byron/gitoxide/commit/7657693b8e23dfb69d6da4376bcd1b8e4e264f7e))
    - Fix warnings ([`e011242`](https://github.com/Byron/gitoxide/commit/e011242c0c9f6779632f5d33dc7b185495f3868e))
    - More helpful prompt error messages when asking for credentials ([`b0c6863`](https://github.com/Byron/gitoxide/commit/b0c6863e6b91ded34ed1860ed449f797d28be81e))
    - Use `git-config-value` crate ([`43656d5`](https://github.com/Byron/gitoxide/commit/43656d5ce84834c847cf8650d4c486c634f209b6))
    - Proper prompt generation ([`63ee38d`](https://github.com/Byron/gitoxide/commit/63ee38dab45fd9d07532f6c01afc2d8dd1c1e904))
    - Remove rustyline in favor of `git-prompt` ([`b3e5e59`](https://github.com/Byron/gitoxide/commit/b3e5e59cafaab0d4866c52722cd2a67aa313b395))
    - Add interactive example for prompt, but… ([`a3fadea`](https://github.com/Byron/gitoxide/commit/a3fadea7759a20fe409762e48d0f1bb9c07f39ba))
    - Blindly implement prompting if it is allowed ([`c78f4b8`](https://github.com/Byron/gitoxide/commit/c78f4b80d1554fdae49d8d5e7d1cfef6c1bf3b05))
    - Frame to support prompting (as compile-time feature) ([`afaae28`](https://github.com/Byron/gitoxide/commit/afaae2880a77c30f845ccf2b3c2b7dc5210665f8))
    - Another test ([`569b7bc`](https://github.com/Byron/gitoxide/commit/569b7bc3d8d8acfe8ad16fe1bc0480e3dbd263d2))
    - Remove unnecessary `Helper` trait ([`19b84f0`](https://github.com/Byron/gitoxide/commit/19b84f0636f6a8d28e938c3a56b3e2cf0a3b4711))
    - Use fixtures in all tests ([`24da911`](https://github.com/Byron/gitoxide/commit/24da911f2fcbc0073fcdab1a217686ac3e3b1c79))
    - Fix tests on linux ([`89db8ee`](https://github.com/Byron/gitoxide/commit/89db8ee938f05f8f9066f34325619f434a5ea00f))
    - More tests ([`57e9060`](https://github.com/Byron/gitoxide/commit/57e906094683860b43f5b7ff71e0189bd2fd0a91))
    - Refactor ([`561bb35`](https://github.com/Byron/gitoxide/commit/561bb356850715c2f4377dd36d1daff69126f543))
    - Another test ([`52d2e54`](https://github.com/Byron/gitoxide/commit/52d2e547b18aa5a00d9d1ada9c88bd84e951e1ed))
    - Fix CI ([`d526c6d`](https://github.com/Byron/gitoxide/commit/d526c6d111bfa05dfa20aca8426d78217ae41558))
    - Improve path normalization; a new ignored test ([`ece5a3f`](https://github.com/Byron/gitoxide/commit/ece5a3f16bfbf84eddce42c64c32736ad98b5356))
    - Parse 'quit' according to spec ([`5e260da`](https://github.com/Byron/gitoxide/commit/5e260dab2edd40092501ab52684f6370104a4eb1))
    - Allow disabling stderr on credential programs ([`4abec50`](https://github.com/Byron/gitoxide/commit/4abec50dc620e965fc03dda4c801753365839691))
    - Refactor ([`cdfcea4`](https://github.com/Byron/gitoxide/commit/cdfcea4eb92097927d4c90639fc211e427b6415c))
    - Url-preprocessing for scripts ([`c00cc35`](https://github.com/Byron/gitoxide/commit/c00cc35493cec8f0b2673248caf1f0d83590dd54))
    - Breaking credential helpers don't stop everything ([`0cdbde7`](https://github.com/Byron/gitoxide/commit/0cdbde78a200ff8585fb217bab3daf81ff46dd6e))
    - Refactor; try harder not to spill secrets in errors ([`525fa97`](https://github.com/Byron/gitoxide/commit/525fa9748b966d515fbdeaa48abd34798e97b78e))
    - First step towards our own `git credential` -like implementation ([`1d1622a`](https://github.com/Byron/gitoxide/commit/1d1622a0dd66ce181d0fa07fc440f85ad0212791))
    - Refactor ([`ce16f51`](https://github.com/Byron/gitoxide/commit/ce16f513dc0a482583cdff168dcdbe2cdd379ad7))
    - Platform specific defaults for the program cascade ([`b66258f`](https://github.com/Byron/gitoxide/commit/b66258f3827e8ca4c7da4a5bca7768888a09e6d5))
    - Refactor ([`85f8cd9`](https://github.com/Byron/gitoxide/commit/85f8cd9b9ef9e93c6495495a83b1ec96437672a5))
    - Refactor ([`23fb302`](https://github.com/Byron/gitoxide/commit/23fb3025112d2f627896383fb0f74f7e91139116))
    - A sketch of how a custom 'git credential' could look like ([`4767a14`](https://github.com/Byron/gitoxide/commit/4767a14d2390edacf46d5436a07685b7d7b79cab))
    - Make 'quit' handler request representable and raise it to an error ([`39b6514`](https://github.com/Byron/gitoxide/commit/39b6514928304807b3d43bd60be716a7f42169c7))
    - Rename `git()` to `builtin()` ([`783a1a7`](https://github.com/Byron/gitoxide/commit/783a1a7dfd64a64fa765fa3d3ef41b1e954413ee))
    - Fix docs ([`f86364c`](https://github.com/Byron/gitoxide/commit/f86364c4e2d9efd04027978679232946494a4734))
    - Rename `Program::Custom*` variants to `Program::External*` ([`bfa2545`](https://github.com/Byron/gitoxide/commit/bfa2545883daf8c4d9e97d2fc91c9328d73ab0eb))
    - Refactor ([`52e958d`](https://github.com/Byron/gitoxide/commit/52e958d62cdf49c35ed56cb26699b155ee0e7732))
    - Fix build ([`99958c6`](https://github.com/Byron/gitoxide/commit/99958c6f87a09b99f21b88e42095a1326d6b8a82))
    - Differentiate between top-level functions and those which are invoked ([`811985a`](https://github.com/Byron/gitoxide/commit/811985aba024385465104ed826a9989961555201))
    - Invoke::Outcome can now represent partial identities ([`49b9bd5`](https://github.com/Byron/gitoxide/commit/49b9bd501f33f1e10ce0180e814b84e293bd3898))
    - Make clear what `helper()` does by renaming it to `git` ([`2edb58b`](https://github.com/Byron/gitoxide/commit/2edb58b6c7395b67c8a7f7c9f6162e6e7c290aac))
    - Make clear in the error type that the helper program couldn't be started ([`c09d223`](https://github.com/Byron/gitoxide/commit/c09d2234cb7e89a2b6ae54e7c8497e86b38621f0))
    - Improved error when identity could not be obtained ([`08c1287`](https://github.com/Byron/gitoxide/commit/08c12876d763a4ade3d4013ce1be66d9594e4ff1))
    - Support for `quit` field in context ([`5a50528`](https://github.com/Byron/gitoxide/commit/5a50528a6f2b1a547fdc9a656e5ea2ca07396ecf))
    - Refactor ([`7487b5a`](https://github.com/Byron/gitoxide/commit/7487b5a4142679ef423c5bd996e40e473c5dfc27))
    - Support for non-consuming operation of `Program` ([`bcfe5ca`](https://github.com/Byron/gitoxide/commit/bcfe5ca22636114bb232d1208ab7c9d78d1fe1de))
    - Disable test that seems to fail on linux ([`419ca56`](https://github.com/Byron/gitoxide/commit/419ca56f7a97cdb0c0e18a4a6f8fda6320692518))
    - More tests for custom helper scripts ([`b396032`](https://github.com/Byron/gitoxide/commit/b3960320d1ef86b42fe8d42c8d7f7abfe66e1710))
    - Support for script invocations ([`377cf14`](https://github.com/Byron/gitoxide/commit/377cf142996279394af38179ad5b51c419642f90))
    - Use git_path::is_absolute() ([`2ba836f`](https://github.com/Byron/gitoxide/commit/2ba836f3e9e5231e8bc42d57d8ff939d85acfe16))
    - Fix tests on windows ([`f4bc860`](https://github.com/Byron/gitoxide/commit/f4bc86011d4aafb5bdeafadd43adb0022ff9b449))
    - Also fill in the git credential command prefix ([`b2f4fe8`](https://github.com/Byron/gitoxide/commit/b2f4fe8f96785222edc3c0472ccef3acf1f069f8))
    - Initial version of parsing of custom helper definitions ([`2b2cd00`](https://github.com/Byron/gitoxide/commit/2b2cd0001babdc16e940fa7242c6d723fc9f789b))
    - `helper::Kind` to `program::Kind` ([`b8c54f0`](https://github.com/Byron/gitoxide/commit/b8c54f03fdb6060caf9c04557c0530c090e7a975))
    - Sketch additional credentials programs ([`46e3045`](https://github.com/Byron/gitoxide/commit/46e3045e04e5197560d8c786642b8f1924a577f9))
    - First test for launching the git credential helper ([`4d7b1dd`](https://github.com/Byron/gitoxide/commit/4d7b1ddec6ef747665edcfddbba68ed12e3970c2))
    - An example implementing a custom credential helper program ([`b1d528a`](https://github.com/Byron/gitoxide/commit/b1d528ae60001ae51dd89b29c26ea505eacbef45))
    - Add docs ([`a360594`](https://github.com/Byron/gitoxide/commit/a360594fac3102cd48aac0039efbe71693c5fa25))
    - `helper::main` to easily create credential helper implementations ([`eaff67c`](https://github.com/Byron/gitoxide/commit/eaff67c14366f149ccca346acb46af12531a24e6))
    - Move `helper::(Next)Action` into `helper::invoke::` module ([`4b7d0b6`](https://github.com/Byron/gitoxide/commit/4b7d0b6d2c43cac9823885bc69510cc4bb6a3f00))
    - Sketch for helper::invoke (get) test ([`c48eb39`](https://github.com/Byron/gitoxide/commit/c48eb390a2f95954f542992806d4e8667ee97981))
    - Refactor ([`cb9d32a`](https://github.com/Byron/gitoxide/commit/cb9d32a3611463f983afea3b3ea875c33092207b))
    - Rename `helper::NextAction` variants to `store` and `erase` ([`ddd5398`](https://github.com/Byron/gitoxide/commit/ddd53988a6d5da17fc65451a059bed1bfa2eb454))
    - Fix docs ([`d9b4ba5`](https://github.com/Byron/gitoxide/commit/d9b4ba5a00c1c9f9c199ac218da76cb716896b75))
    - Add `helper::Action::get_for_url(…)` ([`a253d30`](https://github.com/Byron/gitoxide/commit/a253d30093122e37b5560ff86a7888f8062c7014))
    - Rename `helper::Action` variants to 'Get', 'Store', 'Erase' ([`2073b58`](https://github.com/Byron/gitoxide/commit/2073b583dc2bd83b800584edda6592bb71a01538))
    - Use `helper::Context` in `helper::Action::Fill()` ([`9c6f024`](https://github.com/Byron/gitoxide/commit/9c6f024f838d866645937a67cd67dffb8be17259))
    - Remaining decode tests ([`0e76efe`](https://github.com/Byron/gitoxide/commit/0e76efe035a48f9d042096342ac79804f1eeebdc))
    - Test context value validation ([`20dde9e`](https://github.com/Byron/gitoxide/commit/20dde9eb93ecfb56e72bc5d59caacf31328a55e4))
    - Basic round-tripping of fully fleshed-out context ([`280e4a3`](https://github.com/Byron/gitoxide/commit/280e4a3f69699e11428decc6858711b35ae8249e))
    - Flesh out `helper::Context` as it will soon be used. ([`0cb1ed4`](https://github.com/Byron/gitoxide/commit/0cb1ed4600c614169118b2a94fed83699989a6be))
    - Move `helper::invoke()` related types into `helper::invoke` module. ([`71f6519`](https://github.com/Byron/gitoxide/commit/71f651930e6fd53e3c3f9e82dfd95828e4981d92))
    - Refactor ([`03bf747`](https://github.com/Byron/gitoxide/commit/03bf747292af7792bc175c4f06939b1e02f7234c))
    - Express `helper()` in terms of `helper::invoke()` ([`f2a2c5e`](https://github.com/Byron/gitoxide/commit/f2a2c5ebb7d7428460fe22e9b84dec242a992302))
    - `helper::invoke(helper, action, context)` function that allows for more flexible helper invocation ([`64bc2ec`](https://github.com/Byron/gitoxide/commit/64bc2ec666dacba486bd1de2fbd95f97f2efc7a5))
    - Refactor ([`af27d20`](https://github.com/Byron/gitoxide/commit/af27d20909d14f2737fbad5edd9a6c9d86c93e24))
    - Prepare for more additional implementations of helpers ([`486ef98`](https://github.com/Byron/gitoxide/commit/486ef98b792cc57412a4a90d2cf28586a06d7041))
    - Refactor ([`167b521`](https://github.com/Byron/gitoxide/commit/167b5215326ff2f39e89f2130ff575f4ef6c02d6))
    - Fix docs ([`db46b60`](https://github.com/Byron/gitoxide/commit/db46b60d8f9b4341cf215da6e2cd74bf554fe4b8))
    - Re-add `Result` type ([`de92fce`](https://github.com/Byron/gitoxide/commit/de92fce44496b050e5697aab6d6d1ea98a5954dc))
    - Use `thiserror` instead of `quickerror` ([`4c1a1a2`](https://github.com/Byron/gitoxide/commit/4c1a1a28558c4f8d084b8046afd5d87a11fd25b7))
    - Hide `helper::action()` in favor of single path via `helper()` ([`081934c`](https://github.com/Byron/gitoxide/commit/081934ca4452e550cf2663026905bce67253af81))
    - `helper::(encode|decode)_message(…)` to `::message::(encode|decode)(…)` ([`4c39521`](https://github.com/Byron/gitoxide/commit/4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1))
    - Refactor ([`a395308`](https://github.com/Byron/gitoxide/commit/a395308fdc01b5449a851b1dcb6c3e97a205a5d0))
    - Adapt to changes in `git-url` and use `BString` to represent URLs. ([`12589cc`](https://github.com/Byron/gitoxide/commit/12589cc6f08e4d7aabae30bcdadaa0c2b4850229))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`a5869e0`](https://github.com/Byron/gitoxide/commit/a5869e0b223406820bca836e3e3a7fae2bfd9b04))
    - Release gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`41d57b9`](https://github.com/Byron/gitoxide/commit/41d57b98964094fc1528adb09f69ca824229bf25))
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
    - Rename `git-credentials` to `gix-credentials` ([`8450d09`](https://github.com/Byron/gitoxide/commit/8450d09a50e350180428076472989db205bc4120))
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
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Merge branch 'adjustments-for-cargo' ([`f8c562a`](https://github.com/Byron/gitoxide/commit/f8c562a559e6dc3377583cc7200585dad7c3d481))
    - `helper::Cascade::query_user_only()` can avoid to ask for the password if the transport wouldn't use it. ([`6b375d3`](https://github.com/Byron/gitoxide/commit/6b375d3061f6d307101199eff1ecdfdeb6769965))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'http-config' ([`a4ff140`](https://github.com/Byron/gitoxide/commit/a4ff140a0d3607cf282c49228c1248bd36d464fd))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Improve docs ever so slightly ([`ca5d89c`](https://github.com/Byron/gitoxide/commit/ca5d89c6c94ca5e26098fcbe449a723e6a6b4b69))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Thanks clippy ([`c1399d1`](https://github.com/Byron/gitoxide/commit/c1399d155889e6142eafd65b9bbd2ed005f580dd))
    - Thanks clippy ([`e8e80f5`](https://github.com/Byron/gitoxide/commit/e8e80f5b176ebca4ee81727a551d83383a0b38f8))
    - Thanks clippy ([`9b8a6d6`](https://github.com/Byron/gitoxide/commit/9b8a6d6afeab13968dea61619b1e742e93f60fab))
    - Thanks clippy ([`8317b46`](https://github.com/Byron/gitoxide/commit/8317b4672c8cd38520ed90c42eaadd539ea4df66))
    - Thanks clippy ([`01efe88`](https://github.com/Byron/gitoxide/commit/01efe88cff52e75ba0b76ecc27a41994ee908d2c))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Refactor ([`1dc342f`](https://github.com/Byron/gitoxide/commit/1dc342f9a60cb20e1fafa8c7e913c4a957367662))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - Merge branch 'git-sec' ([`cd723b5`](https://github.com/Byron/gitoxide/commit/cd723b5ae11148e7e9fd07daf28bc04455d5c46f))
    - Release git-credentials v0.0.0 ([`7db45ab`](https://github.com/Byron/gitoxide/commit/7db45abb822b7c28ac84bf0229ec23ce0f46c8f2))
</details>

## 0.9.0 (2022-12-30)

### New Features

 - <csr-id-6b375d3061f6d307101199eff1ecdfdeb6769965/> `helper::Cascade::query_user_only()` can avoid to ask for the password if the transport wouldn't use it.
   That way, `ssh` while using ssh programs can query the user to try next.

## 0.8.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.7.0 (2022-11-21)

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

## 0.6.1 (2022-11-06)

A maintenance release without user-facing changes.

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

<csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/>
<csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/>
<csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/>

### Changed

 - <csr-id-43656d5ce84834c847cf8650d4c486c634f209b6/> use `gix-config-value` crate

### New Features

 - <csr-id-15f1afccb7ed0ebaf217cbbdd58e6ae651a31e42/> `protocol::Context::to_bstring()`, and use it in `example/git-credential-lite`
 - <csr-id-b1d528ae60001ae51dd89b29c26ea505eacbef45/> an example implementing a custom credential helper program
 - <csr-id-eaff67c14366f149ccca346acb46af12531a24e6/> `helper::main` to easily create credential helper implementations
 - <csr-id-a253d30093122e37b5560ff86a7888f8062c7014/> add `helper::Action::get_for_url(…)`
 - <csr-id-64bc2ec666dacba486bd1de2fbd95f97f2efc7a5/> `helper::invoke(helper, action, context)` function that allows for more flexible helper invocation

### Other

 - <csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/> :main::Action::as_str()`

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-783a1a7dfd64a64fa765fa3d3ef41b1e954413ee/> rename `git()` to `builtin()`
 - <csr-id-bfa2545883daf8c4d9e97d2fc91c9328d73ab0eb/> rename `Program::Custom*` variants to `Program::External*`
   It's more true to what it is.
 - <csr-id-811985aba024385465104ed826a9989961555201/> differentiate between top-level functions and those which are invoked
   That way it's easier to use as it can assure an account was actually
   provided.
 - <csr-id-49b9bd501f33f1e10ce0180e814b84e293bd3898/> invoke::Outcome can now represent partial identities
   That way these can be assembled by multiple helpers called in a row.
 - <csr-id-4b7d0b6d2c43cac9823885bc69510cc4bb6a3f00/> move `helper::(Next)Action` into `helper::invoke::` module
   These are only relevant for invoke, hence the change.
 - <csr-id-ddd53988a6d5da17fc65451a059bed1bfa2eb454/> rename `helper::NextAction` variants to `store` and `erase`
 - <csr-id-2073b583dc2bd83b800584edda6592bb71a01538/> rename `helper::Action` variants to 'Get', 'Store', 'Erase'
   It's more obvious what it does and is more typical for what credentials
   helpers do.
 - <csr-id-9c6f024f838d866645937a67cd67dffb8be17259/> Use `helper::Context` in `helper::Action::Fill()`
   That way additional information, like from configuration, can be passed
   as well.
 - <csr-id-71f651930e6fd53e3c3f9e82dfd95828e4981d92/> move `helper::invoke()` related types into `helper::invoke` module.
   Also allow to pass arbitrary bytes (more or less) as context by not
   forcing it all into a string. Values can now be everything, which
   helps with passing paths or other values.
 - <csr-id-4c1a1a28558c4f8d084b8046afd5d87a11fd25b7/> use `thiserror` instead of `quickerror`
 - <csr-id-081934ca4452e550cf2663026905bce67253af81/> hide `helper::action()` in favor of single path via `helper()`

### Other (BREAKING)

 - <csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/> `helper::Kind` to `program::Kind`
 - <csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/> `helper::(encode|decode)_message(…)` to `::message::(encode|decode)(…)`

## 0.4.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Changed (BREAKING)

 - <csr-id-12589cc6f08e4d7aabae30bcdadaa0c2b4850229/> adapt to changes in `gix-url` and use `BString` to represent URLs.
   They can contain paths, which is why `String` can't represent a URL
   losslessly.
   
   For HTTP urls these are ultimately UTF-8 strings though.

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.2.0 (2022-06-13)

A maintenance release without user-facing changes.

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-3d339d5c24630fac0192b5d27f9b1cbd94418730/> use `gix-sec::Identity` type
   It's shared across crates.

## 0.0.0 (2022-04-15)

An empty crate without any content to reserve the name for the gitoxide project.

