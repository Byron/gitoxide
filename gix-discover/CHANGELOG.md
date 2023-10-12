# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.26.0 (2023-10-12)

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

## 0.25.0 (2023-09-24)

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

## 0.24.0 (2023-09-08)

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

## 0.23.0 (2023-08-22)

### Bug Fixes

 - <csr-id-c51c8daee1ab54130ae3ed83ce67d08f01c4881a/> fix incorrect s/git-config/gix-config/
   3a861c8f049f6502d3bcbdac752659aa1aeda46a just blindly replaced any
   occurence of "git-config" or "git_config" with "gix-config"/"gix_config".
   
   There is no such thing as a gix-config file.
   gix-config is a git-config file parser.

### Bug Fixes (BREAKING)

 - <csr-id-169228b12f826135d27fd674acbca932b2691a91/> `bare()` can now make sure it detects bare repos, at an expense.
   In order to be very sure it's the correct guess, we have to open and quick-parse
   a the bare line.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 18 calendar days.
 - 30 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#980](https://github.com/Byron/gitoxide/issues/980)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#980](https://github.com/Byron/gitoxide/issues/980)**
    - `bare()` can now make sure it detects bare repos, at an expense. ([`169228b`](https://github.com/Byron/gitoxide/commit/169228b12f826135d27fd674acbca932b2691a91))
 * **Uncategorized**
    - Release gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0 ([`6c62e74`](https://github.com/Byron/gitoxide/commit/6c62e748240ac0980fc23fdf30f8477dea8b9bc3))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge pull request #988 from not-my-profile/fix-gix-config-sub ([`7735047`](https://github.com/Byron/gitoxide/commit/7735047198bd7cc5059ca338f5c2147dd273f711))
    - Fix incorrect s/git-config/gix-config/ ([`c51c8da`](https://github.com/Byron/gitoxide/commit/c51c8daee1ab54130ae3ed83ce67d08f01c4881a))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.22.1 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`0062971`](https://github.com/Byron/gitoxide/commit/00629710dffeb10fda340665530353703cf5d129))
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.22.0 (2023-07-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 4 calendar days.
 - 19 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Thanks clippy ([`3ef32af`](https://github.com/Byron/gitoxide/commit/3ef32af9bf477cbc60d24da8bb3f15d20976e9e0))
</details>

## 0.21.1 (2023-06-29)

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
    - Release gix-glob v0.9.1, gix-attributes v0.14.1, gix-config-value v0.12.3, gix-ref v0.32.1, gix-sec v0.8.3, gix-config v0.25.1, gix-url v0.20.1, gix-credentials v0.16.1, gix-discover v0.21.1, gix-ignore v0.4.1, gix-pack v0.39.1, gix-odb v0.49.1, gix-worktree v0.21.1, gix v0.48.0 ([`69c6a36`](https://github.com/Byron/gitoxide/commit/69c6a36ba14cbef129deebda9fd8870005fefa17))
    - Prepare changelogs prior to release ([`c143cf4`](https://github.com/Byron/gitoxide/commit/c143cf48ee1885467e3e9262a3f8823a1247bfe0))
    - Align usage of `gix-path` across all crates ([`73c1292`](https://github.com/Byron/gitoxide/commit/73c1292be393986c4a1adde1400abf551e850da0))
</details>

## 0.21.0 (2023-06-29)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
</details>

## 0.20.0 (2023-06-22)

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

## 0.19.0 (2023-06-06)

### Bug Fixes

 - <csr-id-96267708958ead2646aae8766a50fa060739003c/> make it possible to discover a bare repository with an index file.
   It's perfectly viable and possible to manually create an index in a bare
   repository. When that happens, previously it wasn't possible to open a
   repository on it. This is now fixed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 13 calendar days.
 - 37 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include custom clippy settings ([`b057500`](https://github.com/Byron/gitoxide/commit/b057500dd3e6b75be3ebcd258cda0b946bedd9e1))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Make it possible to discover a bare repository with an index file. ([`9626770`](https://github.com/Byron/gitoxide/commit/96267708958ead2646aae8766a50fa060739003c))
</details>

## 0.18.1 (2023-04-29)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-discover v0.18.1, gix-worktree v0.17.1, gix-testtools v0.12.0 ([`f7b6c6f`](https://github.com/Byron/gitoxide/commit/f7b6c6f27c090cbc584fbd3f5403da5ac1a9ff02))
    - Prepare changelogs prior to release of gix-testtools ([`fc45f1b`](https://github.com/Byron/gitoxide/commit/fc45f1b417bf545d4a0a105c40b37f92c24decad))
    - Release gix-ref v0.29.1 ([`13e01f5`](https://github.com/Byron/gitoxide/commit/13e01f5742ed2121f00f4b16c1df0cce5e7708ef))
</details>

## 0.18.0 (2023-04-27)

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

## 0.17.0 (2023-04-26)

### New Features

 - <csr-id-1b8d9e6a408e480ae1912e919c37a26e5c46639d/> `discover_opts` supports an option to see non-bare repos only.
   Knowing this allows for an optimization that saves a few syscalls
   for maximum performance, or minimum penalty on very slow filesystems.

### Bug Fixes

 - <csr-id-abbae1cbca6fb5c4347e51f7122f02d106c8d30a/> common-dir is now queried only when needed.
   Previously it added costs that were accumulating during discovery.
 - <csr-id-761324953a20d061f859c995180d603bb3734f9a/> speedup HEAD detection in `is_git()`
   Previously it would always check with a `store` which has a fuzzy search
   right now. Instead, we check once ourselves before trying to read HEAD
   for validation.
 - <csr-id-7543fab01c337c086c7b1331ad5fb1d48f9d11ac/> try `.git` subdir early for higher chances of success.
   This will find worktrees faster, which should be most of the
   dev directories out there.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 23 calendar days.
 - 27 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'faster-discovery' ([`282a1a3`](https://github.com/Byron/gitoxide/commit/282a1a3f29572250e6d741c6daa97b0bed679753))
    - `discover_opts` supports an option to see non-bare repos only. ([`1b8d9e6`](https://github.com/Byron/gitoxide/commit/1b8d9e6a408e480ae1912e919c37a26e5c46639d))
    - Common-dir is now queried only when needed. ([`abbae1c`](https://github.com/Byron/gitoxide/commit/abbae1cbca6fb5c4347e51f7122f02d106c8d30a))
    - Avoid redundant metadata queries for directories ([`f36abd5`](https://github.com/Byron/gitoxide/commit/f36abd57ff0719edc2c95058832ef57ff8c53bba))
    - Speedup HEAD detection in `is_git()` ([`7613249`](https://github.com/Byron/gitoxide/commit/761324953a20d061f859c995180d603bb3734f9a))
    - Try `.git` subdir early for higher chances of success. ([`7543fab`](https://github.com/Byron/gitoxide/commit/7543fab01c337c086c7b1331ad5fb1d48f9d11ac))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - Add test to assure we can handle bare repos with `index` file. ([`a68692b`](https://github.com/Byron/gitoxide/commit/a68692b5bfb8082f07dd907b091240106c6b358e))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
    - Merge branch 'patch-1' ([`d0052c1`](https://github.com/Byron/gitoxide/commit/d0052c13cabcde8058177d2439053b50ea5adbfc))
    - Upgrade serial-test to v2 ([`6932017`](https://github.com/Byron/gitoxide/commit/69320174685e72940cd0fe600c94abb948a62bdd))
</details>

## 0.16.2 (2023-03-30)

### Documentation

 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.7.3, gix-config-value v0.10.2, gix-config v0.20.1, gix-discover v0.16.2, gix-index v0.15.1, gix-odb v0.43.1, gix-packetline v0.15.1, gix-protocol v0.30.2, gix-worktree v0.15.2, gix v0.43.1 ([`38eed1d`](https://github.com/Byron/gitoxide/commit/38eed1d06e7cbb8fbcd54b2cad3163ca45e0baf1))
    - Fix minor typos ([`cc48c35`](https://github.com/Byron/gitoxide/commit/cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5))
    - Release gix-ref v0.27.2 ([`e965b18`](https://github.com/Byron/gitoxide/commit/e965b18aedcf13ec4538bc7bc700269a56ca615e))
</details>

## 0.16.1 (2023-03-26)

A maintenance release without any user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 15 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-prompt v0.3.3, gix-diff v0.28.1, gix-discover v0.16.1, gix-pack v0.33.2, gix-transport v0.29.1, gix-protocol v0.30.1, gix-revision v0.12.1, gix-worktree v0.15.1, gix v0.43.0, safety bump gix v0.43.0 ([`5dc1f9f`](https://github.com/Byron/gitoxide/commit/5dc1f9f2bcb8b3e147115fcb6f76558e8f48ffef))
    - Prepare changelogs prior to release ([`3016a28`](https://github.com/Byron/gitoxide/commit/3016a285f566bdfe7de2774fa6f2254c1b1a2c51))
    - Merge branch 'fix-790' ([`ee36e5b`](https://github.com/Byron/gitoxide/commit/ee36e5bb985e9ad90bc382cdd051a8b5295ca18c))
    - Less dependencies for tests (via `serial_test` no default features) ([`8f2accd`](https://github.com/Byron/gitoxide/commit/8f2accdf738def9aa4abdf08fc299d0e9807bc3e))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## 0.16.0 (2023-03-10)

### Bug Fixes (BREAKING)

 - <csr-id-61ee9057f38d53d7cd4317503625359023747725/> add missing 'not' in error messages

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
    - Merge branch '771_missing_negtive_in_error_message_returned_by_gix_discover' ([`7f6e67d`](https://github.com/Byron/gitoxide/commit/7f6e67d1339b9b9548d5ea012954c9e45588af9d))
    - Add missing 'not' in error messages ([`61ee905`](https://github.com/Byron/gitoxide/commit/61ee9057f38d53d7cd4317503625359023747725))
</details>

## 0.15.0 (2023-03-04)

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

## 0.14.0 (2023-03-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
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
    - Prepare for git-tempfile release ([`56c005b`](https://github.com/Byron/gitoxide/commit/56c005b13c44376f71e61781e73c0bf93416d0e4))
</details>

## 0.13.1 (2023-02-20)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-07cd4a869d22476f812e4dafc847894ddebff5a8/> `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730).
   Previously ceiling directories that were UNC paths would not work together
   with directories which were not due to different bases.
   
   Now ceiling directories will be converted non-UNC paths before using them.
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

## 0.13.0 (2023-02-17)

<csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/>
<csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/>
<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>
<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>
<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### New Features (BREAKING)

 - <csr-id-b4dcfc716a80ffccbab6f7ccc586d8063cc10fff/> `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>`
   That way it's possible to avoid at least one call of
   `std::env::current_dir()` per invocation, which also is more consnstent
   with similar plumbing methods.
   
   Furthermore it can signal with `None` if the input directory was invalid.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.
 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-1b0ef1856f542e5dc23310be93c2b39a9d84cb80/> Provide more details when classifying submodule directories.
   That way we can avoid feeding `.git` files to `git-repository::open()`
   and avoid work duplication, which ultimately allows to open submodules
   directories of all kinds.

### Other

 - <csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/> test `is_git` on macOS exfat
 - <csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/> allow opening repos on macos exfat
 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths
 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules
 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery
 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### New Features

 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.
 - <csr-id-2e015a3874ab4db40449ec4b8ffb126d8e1a22a4/> add `is_submodule_git_dir()`
 - <csr-id-4a3e1cfbc537963b851c66754e7724875b49b1f4/> `is_git()` now detects submodule worktrees correctly.
 - <csr-id-aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f/> `git_discover::is_git()` can detect submodule dirs correctly enough.
   We currently detect them as possibly bare, which could be improved if we
   allow ourselves to see `.git/modules` as `submodule` always.
 - <csr-id-010350180459aec41132c960ddafc7b81dd9c04d/> add `DOT_GIT_DIR` constant, containing the name ".git".
 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-1a489bee7bb5e9cb168fd2758930dc0b92040dcc/> search dirs that are equal to ceiling dirs are allowed to 'break through' said ceiling.
   This makes the discovery behaviour similar to what `git` does, even though
   by default we consider it an error if that happens as no ceiling directory
   matches.
 - <csr-id-c91ac5b0916a2d2e1bb9ca096e50d627f1f6a8f3/> ignore cwd as ceiling directory
 - <csr-id-70169794bc27936382e75e75e18806649630d7cf/> `upwards::Options::apply_environment()` uses a more 'git'-like default setup.
   When environment variables are used, this won't lead to an error just because the ceiling directories
   don't cover the paths that are currently used for discovery, making it more similar to
   the behaviour git exhibits as well.
 - <csr-id-8840b649b68dd2ae17e43a61ab04cf866124ef46/> make `parse_ceiling_dirs` resilient to illformed UTF-8
   Previously it performed a conversion from bytes to OsStr which might have failed,
   now it stays with `OsStr`all the way.
 - <csr-id-1eddfbd22f1d7362f15021897b0a4fecda5f4c3f/> split directories by semicolon on Windows
 - <csr-id-07cd4a869d22476f812e4dafc847894ddebff5a8/> `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730).
   Previously ceiling directories that were UNC paths would not work together
   with directories which were not due to different bases.
   
   Now ceiling directories will be converted non-UNC paths before using them.
 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning
 - <csr-id-6fc5c06482636540804f7a8fb74794f52c72eda6/> Discover repo with relative path and ceiling
   A couple of problems are repaired to allow discovering a repository from
   "." with a ceiling directory.
   
   One problem was that find_ceiling_height() did the wrong thing when
   confronted with any relative search_dir. This is resolved by converting
   search_dir to be absolute if it is relative.
   
   The other problem was that discover_opts() also mishandled relative paths.
   When the cursor started out as ".", cursor.pop() would be blindly called
   such that cursor would be "" for the second iteration. When a ceiling
   directory was in use such that there was a max_height, the current height
   would be burned going from ".", to "", and then to "<cwd>", before finally
   actually getting to a real parent directory. This problem is ameliorated by
   testing whether the cursor has a non-empty parent before popping.
   
   N.B. the new test case relies on the test running from the git-discover
   directory such that the gitoxide repository will be found. This is a bit
   fragile and will fail if, for example, the test is run from an unpacked
   gitoxide source tarball.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 271 commits contributed to the release over the course of 287 calendar days.
 - 28 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#426](https://github.com/Byron/gitoxide/issues/426), [#427](https://github.com/Byron/gitoxide/issues/427), [#429](https://github.com/Byron/gitoxide/issues/429), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#482](https://github.com/Byron/gitoxide/issues/482), [#544](https://github.com/Byron/gitoxide/issues/544), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 9 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - A test to assure non-existing worktree directories don't hinder discovery ([`131481c`](https://github.com/Byron/gitoxide/commit/131481cac26959d46f62d40adcfc895faa4a1698))
    - Allow discovery of linked worktree git dirs ([`050f795`](https://github.com/Byron/gitoxide/commit/050f795bfb0fe11655cd7e45c10d87c89ba82625))
    - Path shortening now calculates the actual path length, not just component count ([`c0d24e0`](https://github.com/Byron/gitoxide/commit/c0d24e0fb422f990213606d781ba056c238bd70b))
    - Adjust to changes in git-path ([`a627ddc`](https://github.com/Byron/gitoxide/commit/a627ddc48be034f8a42d86edf2a9295299297d59))
    - Adapt to changes in git-path ([`c258d77`](https://github.com/Byron/gitoxide/commit/c258d77660755559eaa5389bfabe0cebdf478a80))
    - Refactor ([`81a542f`](https://github.com/Byron/gitoxide/commit/81a542fbb680a19322877cf3740ae2ae66b95011))
    - Rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - Brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Another test to assure invalid worktree dirs aren't git dirs ([`e6e3608`](https://github.com/Byron/gitoxide/commit/e6e3608490bda2577c23f93d905a51c33fa4f777))
    - Adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - Fix tests on windows ([`8080ad2`](https://github.com/Byron/gitoxide/commit/8080ad2baedf83237a0d9a01550582fc8592e568))
    - Better tests for worktree discovery ([`9d0f134`](https://github.com/Byron/gitoxide/commit/9d0f134b3186635cbc253464fc7cec2ff522a4e2))
    - The first working version of worktree detection ([`dc73e96`](https://github.com/Byron/gitoxide/commit/dc73e96fdd21713083d9fe0464d1a5689405c116))
    - Fix windows tests ([`b025008`](https://github.com/Byron/gitoxide/commit/b0250080c5a1e8ba3fbdb3f70038521b75634306))
    - Refactor ([`6b73e0d`](https://github.com/Byron/gitoxide/commit/6b73e0dc60488fcadf21366322e75f711bada6f4))
    - Support for gitdir file parsing with all of gits safety ([`1223fc7`](https://github.com/Byron/gitoxide/commit/1223fc7777f57d73c3693407f6a1e41393919a99))
    - Basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - Fix test on windows maybe ([`9583048`](https://github.com/Byron/gitoxide/commit/95830480928421de1e38c99a7e9a65629ec1cc41))
    - Try to fix winodws build ([`f2c5ed3`](https://github.com/Byron/gitoxide/commit/f2c5ed3326a3076cc25afbaeec9a540d823e0606))
    - Refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - Refactor ([`eead214`](https://github.com/Byron/gitoxide/commit/eead2144e10b96e997cd702cb651f4c92693df95))
    - Avoid running failing test on _any_ windows ([`9351a3d`](https://github.com/Byron/gitoxide/commit/9351a3db2a7e9da13be3071c1d2d5aa0cb7a35b6))
    - Fix tests on windows maybe ([`8084581`](https://github.com/Byron/gitoxide/commit/8084581619c7261f779ae42856be756eac7fbfa4))
    - Don't archive repositories with worktrees ([`389ef66`](https://github.com/Byron/gitoxide/commit/389ef663ce0417f3840f90dedfc91a57eae6fd0c))
    - Prepare test to provide worktrees of different kinds as well ([`a4cec4a`](https://github.com/Byron/gitoxide/commit/a4cec4ab8ba80a8f439b283d369d48cdafbe7b74))
    - Add required support for reading path files ([`bc08511`](https://github.com/Byron/gitoxide/commit/bc085116241e8824c623efc9893ded220fe9a5a2))
    - Remove handling of environment variables ([`4e914f8`](https://github.com/Byron/gitoxide/commit/4e914f89bbf04dbe55eae01f7be09899296ad8b0))
    - Add git-discover to check-size ([`85b29a6`](https://github.com/Byron/gitoxide/commit/85b29a6ba6b3ee4b925fd10c688b2aabcf4273f2))
    - Fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - Refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - Migrate all relevant code from git-repository to git-discover ([`631e70e`](https://github.com/Byron/gitoxide/commit/631e70e6f210df40eb789023970ec17095ec3556))
    - Add empty git-discover crate ([`6565f16`](https://github.com/Byron/gitoxide/commit/6565f163ba464f5adf63d214143a4e12f48b00fc))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Add `DOT_GIT_DIR` constant, containing the name ".git". ([`0103501`](https://github.com/Byron/gitoxide/commit/010350180459aec41132c960ddafc7b81dd9c04d))
    - Adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
 * **[#426](https://github.com/Byron/gitoxide/issues/426)**
    - Allow tests to deal with shared drives by adjusting trust expectation ([`85ab096`](https://github.com/Byron/gitoxide/commit/85ab0964a1a35556a5efe392576c9b028e601c6c))
    - Cleanup ([`e029eb4`](https://github.com/Byron/gitoxide/commit/e029eb4589942e06a53bc0f6ed472d1d33c4a7e6))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - Turn on performance mode for sha-1 computation ([`44371a1`](https://github.com/Byron/gitoxide/commit/44371a10f464f32db346aa6b8309e983cfa20933))
 * **[#429](https://github.com/Byron/gitoxide/issues/429)**
    - Adjust changelogs prior to release ([`7397805`](https://github.com/Byron/gitoxide/commit/7397805fd032a752d6c2f2c2c28ac11ddecc7193))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - Adapt to changes in `git-sec` ([`0ce21b1`](https://github.com/Byron/gitoxide/commit/0ce21b1e95e00f2d4cebc1c62349e0dc66a1c705))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#482](https://github.com/Byron/gitoxide/issues/482)**
    - Add `is_submodule_git_dir()` ([`2e015a3`](https://github.com/Byron/gitoxide/commit/2e015a3874ab4db40449ec4b8ffb126d8e1a22a4))
    - Query the `cwd` only once instead of potentially multiple times, allocating a Vec each time. ([`6be38f2`](https://github.com/Byron/gitoxide/commit/6be38f294087f8c5e9bb8522bbf928a761c5cdf0))
    - Provide more details when classifying submodule directories. ([`1b0ef18`](https://github.com/Byron/gitoxide/commit/1b0ef1856f542e5dc23310be93c2b39a9d84cb80))
    - Fix windows build ([`ff488fa`](https://github.com/Byron/gitoxide/commit/ff488fac628b95ec8f64b136a125913b7b96d3a6))
    - Fix docs ([`3724f31`](https://github.com/Byron/gitoxide/commit/3724f31c0563d79a0cad88d773fa60ea21f504b8))
    - `is_git()` now detects submodule worktrees correctly. ([`4a3e1cf`](https://github.com/Byron/gitoxide/commit/4a3e1cfbc537963b851c66754e7724875b49b1f4))
    - Refactor ([`1ee9918`](https://github.com/Byron/gitoxide/commit/1ee991847a5adeaaeb6e80ae626c28b0ba89e0af))
    - `git_discover::is_git()` can detect submodule dirs correctly enough. ([`aa6fd97`](https://github.com/Byron/gitoxide/commit/aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f))
    - Another test indicating git-discover misclassifies `.git/modules` as worktree ([`9133141`](https://github.com/Byron/gitoxide/commit/91331416146afb9e868e0868b47097865d2d7117))
    - Failing test to show discovery skips submodules entirely ([`b56b57c`](https://github.com/Byron/gitoxide/commit/b56b57c4111512853e42397ee1fdfd4c11cadf75))
 * **[#544](https://github.com/Byron/gitoxide/issues/544)**
    - Fix a test that assumed a non-worktree parent repo. ([`1908229`](https://github.com/Byron/gitoxide/commit/190822908d99c7e2f9f118cd1eff80607e4558c4))
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
    - Rename `git-discover` to `gix-discover` ([`6e619de`](https://github.com/Byron/gitoxide/commit/6e619deb6fd063b7fe524b9e2f837c2168adeaa3))
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
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
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
    - Merge branch 'unc-paths' ([`ff0387e`](https://github.com/Byron/gitoxide/commit/ff0387e9975e61a2d796b86f4d857c3b8528c94b))
    - `GIT_CEILING_DIRECTORIES` now handles UNC paths correctly #(730). ([`07cd4a8`](https://github.com/Byron/gitoxide/commit/07cd4a869d22476f812e4dafc847894ddebff5a8))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Search dirs that are equal to ceiling dirs are allowed to 'break through' said ceiling. ([`1a489be`](https://github.com/Byron/gitoxide/commit/1a489bee7bb5e9cb168fd2758930dc0b92040dcc))
    - Add test to help understand why it acts the way it does. ([`48057d3`](https://github.com/Byron/gitoxide/commit/48057d305b5bddcd6a05a73b80d1c17045b54ade))
    - Ignore cwd as ceiling directory ([`c91ac5b`](https://github.com/Byron/gitoxide/commit/c91ac5b0916a2d2e1bb9ca096e50d627f1f6a8f3))
    - Upgrade serial-test to 1.0 ([`9ab3e25`](https://github.com/Byron/gitoxide/commit/9ab3e25e4c1ff6d504a79521cfa9453a7e7e7465))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Make fmt ([`e22080e`](https://github.com/Byron/gitoxide/commit/e22080e4a29d0bad15a99d565a5e3e304a8743ec))
    - `upwards::Options::apply_environment()` uses a more 'git'-like default setup. ([`7016979`](https://github.com/Byron/gitoxide/commit/70169794bc27936382e75e75e18806649630d7cf))
    - Allow ceiling dirs not to match if `apply_environment` is used ([`9b7df62`](https://github.com/Byron/gitoxide/commit/9b7df6237611cd1b6328364e6d95c22c99c63ca7))
    - Merge branch 'fix/ceiling-directories-win' ([`78517a6`](https://github.com/Byron/gitoxide/commit/78517a616b4b1b6fefc2367057a9b3b786d93750))
    - Make `parse_ceiling_dirs` resilient to illformed UTF-8 ([`8840b64`](https://github.com/Byron/gitoxide/commit/8840b649b68dd2ae17e43a61ab04cf866124ef46))
    - Split directories by semicolon on Windows ([`1eddfbd`](https://github.com/Byron/gitoxide/commit/1eddfbd22f1d7362f15021897b0a4fecda5f4c3f))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Uprgade serial_test to 0.10 ([`8623b5a`](https://github.com/Byron/gitoxide/commit/8623b5a36f3e82cd694f7772798e36656cbfcd0a))
    - Release git-ref v0.23.0, git-config v0.15.0, git-command v0.2.2, git-diff v0.26.0, git-discover v0.12.0, git-mailmap v0.9.0, git-pack v0.30.0, git-odb v0.40.0, git-transport v0.25.2, git-protocol v0.26.1, git-revision v0.10.0, git-refspec v0.7.0, git-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - Prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - Adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'adjustments-for-cargo' ([`94750e1`](https://github.com/Byron/gitoxide/commit/94750e15831969059551af35d31c21009462084d))
    - Assure git-discover can recognize a repository without a 'config' file ([`7994416`](https://github.com/Byron/gitoxide/commit/79944160f3cad4b977521853e881070f23b7dcc1))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - Improve error verbosity when fetching and cloning ([`40f7379`](https://github.com/Byron/gitoxide/commit/40f7379b7a89f7fe6f916801384e9e65e5b85c57))
    - Read worktree specific configuration to override the one from the shared repository. ([`d1b7ec6`](https://github.com/Byron/gitoxide/commit/d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2))
    - Fix typo in docs ([`2d9b2f1`](https://github.com/Byron/gitoxide/commit/2d9b2f16d938e1a4a391534df37f99f1c64c8ea6))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'path-normalize' ([`805329a`](https://github.com/Byron/gitoxide/commit/805329a0a5f6543bbc1d5885977b47bf7baa7f08))
    - Adjust for changes in `git-path` ([`cf25e35`](https://github.com/Byron/gitoxide/commit/cf25e3594b99909defb431f34fb3a4d8a25bd37c))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'discover-rel-path' ([`5f908fb`](https://github.com/Byron/gitoxide/commit/5f908fb86857d565715b9b0b8b453b29273fb022))
    - Add test that actually uses chdir like in the real world. ([`048c8b2`](https://github.com/Byron/gitoxide/commit/048c8b260f78ee1eac74bf8f4ee2374d6b0eb308))
    - Refactor ([`cdb9556`](https://github.com/Byron/gitoxide/commit/cdb95567c83d52cf93a621bf616883f09f5544bb))
    - Discover repo with relative path and ceiling ([`6fc5c06`](https://github.com/Byron/gitoxide/commit/6fc5c06482636540804f7a8fb74794f52c72eda6))
    - Merge branch 'cwd-consistency' ([`ea7c6a3`](https://github.com/Byron/gitoxide/commit/ea7c6a3b069c9e13905b51b87538c57ba9182dca))
    - `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>` ([`b4dcfc7`](https://github.com/Byron/gitoxide/commit/b4dcfc716a80ffccbab6f7ccc586d8063cc10fff))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - Prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'fix-544' ([`a0dd96b`](https://github.com/Byron/gitoxide/commit/a0dd96b509f46e261482eb4027fbbd56380e3afc))
    - Merge branch 'main' into fetch-pack ([`93917cb`](https://github.com/Byron/gitoxide/commit/93917cb6ecbb30daf3d20bb5a7c65e12211f084f))
    - Don't run typically failing MacOS specific test on CI anymore ([`6c9d18b`](https://github.com/Byron/gitoxide/commit/6c9d18b1435f28c5ef70371d909cc884eeb6b86a))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`2571831`](https://github.com/Byron/gitoxide/commit/2571831e5939bf4ea6f19537b0c1ccd71dc99088))
    - Prepare changelog  prior to release ([`fc6b958`](https://github.com/Byron/gitoxide/commit/fc6b9583d0534f70e0c8afdcad46e09a5001d62b))
    - Merge branch 'macos-exfat' ([`f256f8f`](https://github.com/Byron/gitoxide/commit/f256f8fb7603f83d44acda07386f277c65ac652c))
    - Refactor ([`2cd1c00`](https://github.com/Byron/gitoxide/commit/2cd1c007330749f69fc1f154ba1a6e7a0f6e23d0))
    - Test `is_git` on macOS exfat ([`7057ad2`](https://github.com/Byron/gitoxide/commit/7057ad2251f76756a5b35b6bbba10ab9b6601226))
    - Allow opening repos on macos exfat ([`08d0c27`](https://github.com/Byron/gitoxide/commit/08d0c27b0641526aa8b70ee5aadd3658862bef1b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'submodule-open' ([`8f5f3ab`](https://github.com/Byron/gitoxide/commit/8f5f3ab588cf0165d50a82365119ad5804745017))
    - Thanks clippy ([`2135fb8`](https://github.com/Byron/gitoxide/commit/2135fb80a126afc6d95b9eaa9f1cd32acb46615b))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'config-comfort' ([`84b98d9`](https://github.com/Byron/gitoxide/commit/84b98d94177ceaf931aaa521e44eca0fa484d2d3))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Remove another special case on windows due to canonicalize() ([`61abb0b`](https://github.com/Byron/gitoxide/commit/61abb0b006292d2122784b032e198cc716fb7b92))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Refactor ([`ec37cb8`](https://github.com/Byron/gitoxide/commit/ec37cb8005fa272aed2e23e65adc291875b1fd68))
    - Refactor ([`b27a8c2`](https://github.com/Byron/gitoxide/commit/b27a8c243cdc14730478c2a94cafdc8ccf5c60d3))
    - Refactor ([`06e96a4`](https://github.com/Byron/gitoxide/commit/06e96a435d820a1ef1e567bf93e7b9ca5fa74829))
    - Refactor ([`b555bda`](https://github.com/Byron/gitoxide/commit/b555bdae9964628b6d43e00dd7d7ee8fe674d309))
    - Unify the way `dir_made_absolute` is computed ([`48417fc`](https://github.com/Byron/gitoxide/commit/48417fc8d8d3ec0005ef7b6ef35ced09d92282c2))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - Refactor ([`7b307f5`](https://github.com/Byron/gitoxide/commit/7b307f5acb7214f4ff674e00167933cdcccc353a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Avoid shortening absolute paths ([`6106521`](https://github.com/Byron/gitoxide/commit/6106521581029c5c24b23a47bb91c1921edfa0af))
    - Add discovery opt env-overrides & env discovery helpers ([`e521d39`](https://github.com/Byron/gitoxide/commit/e521d39e1b0f4849280bae1527bf28977eec5093))
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - Prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - Adopt git-for-windows exception rules ([`136eb37`](https://github.com/Byron/gitoxide/commit/136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - Refactor ([`aac5169`](https://github.com/Byron/gitoxide/commit/aac5169aa7c753eb3510e61bc01b47a4b7b01c6f))
    - Use `defer` to make dmg unmounting more reliable ([`dbc5caa`](https://github.com/Byron/gitoxide/commit/dbc5caa99b050ac7a96cd5e7a73786072c695530))
    - Refactor ([`886e26e`](https://github.com/Byron/gitoxide/commit/886e26ee686776ad4caf1864f24cb358432d5c49))
    - Assure cross-fs check doesn't break if cursor is currently empty ([`4546889`](https://github.com/Byron/gitoxide/commit/4546889adb87c0d9d1c5d22785240033dc3b0dc2))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Add cross_fs option to upwards discovery ([`277c41f`](https://github.com/Byron/gitoxide/commit/277c41f1185398e14f92247aada8422f6b08afd1))
    - Re-enable discovery test on windows thanks to use of `realpath()` ([`1f4ae9e`](https://github.com/Byron/gitoxide/commit/1f4ae9e74843d108b261cbfe10e158f74286088a))
    - Fix windows test failure due to //? by ignoring it there. ([`c5fd322`](https://github.com/Byron/gitoxide/commit/c5fd3223db73ee61844477aff0e8a2438d9b2e39))
    - Fix windows tests (broke thanks to \\?\), maybe ([`e458b59`](https://github.com/Byron/gitoxide/commit/e458b5946ebf01d9f901769b8547df85ef14afcb))
    - Assure ceiling dirs are comparable after absolutize ([`32a157b`](https://github.com/Byron/gitoxide/commit/32a157b513191c60795924765825dc7dfb0b38c1))
    - Merge branch 'davidkna-discover-ceiling' ([`66944ba`](https://github.com/Byron/gitoxide/commit/66944ba986114ece2d3b31440c721d0e84b4f267))
    - Thanks clippy ([`4979d20`](https://github.com/Byron/gitoxide/commit/4979d2071cbd1a98f3d81aacd60dd99f07d3f746))
    - Refactor ([`895b772`](https://github.com/Byron/gitoxide/commit/895b772b5855818ad2227cac8dda0be00f9d5189))
    - Control if at least one ceiling dir has to match using an option ([`ca1f3eb`](https://github.com/Byron/gitoxide/commit/ca1f3ebb1306075767597c75bb288a3a0b4ebb41))
    - Revert "remove implicit canonicalization and improve tests" ([`821f3f3`](https://github.com/Byron/gitoxide/commit/821f3f383a21ddfd274cb6bdcd2228717cd21942))
    - Refactor ([`6bbc53b`](https://github.com/Byron/gitoxide/commit/6bbc53be867e5292a98c422526185c6a4736a6a5))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - Fix `special_relative_base` test ([`ae226ba`](https://github.com/Byron/gitoxide/commit/ae226ba9c08b04621ce3b42e6972e102d8af2b9c))
    - Remove implicit canonicalization and improve tests ([`cdbb4c9`](https://github.com/Byron/gitoxide/commit/cdbb4c9db56ef6eb258a9a76691614d4b45c8d7a))
    - Refactor ([`429446c`](https://github.com/Byron/gitoxide/commit/429446c87473dca98bf802fa3de020cb58625f63))
    - Refactor ([`9673aae`](https://github.com/Byron/gitoxide/commit/9673aae3da57bc12d45389de3034ab0c2c1c27ab))
    - Some more assertions and ceiling dirs ([`a30bcb8`](https://github.com/Byron/gitoxide/commit/a30bcb82413103c69827151f25be0b14b1f04f37))
    - Add ceiling_dirs option to upwards discovery ([`e63e722`](https://github.com/Byron/gitoxide/commit/e63e722791a7795cd99048bed834459595c60abc))
    - Declare `git-discover` usable as it's fully documented ([`e439015`](https://github.com/Byron/gitoxide/commit/e439015288dad806494e0a2ed8d44fb2247de372))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Thanks clippy ([`7617da0`](https://github.com/Byron/gitoxide/commit/7617da002ef5906f858c0cc2c349f442bafd6239))
    - Thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - Thanks clippy ([`3c9da80`](https://github.com/Byron/gitoxide/commit/3c9da80a39bc6638a38868f71d80e2a8ad337a4a))
    - Release git-discover v0.0.0 ([`2b5cf2b`](https://github.com/Byron/gitoxide/commit/2b5cf2b95a723d885584ac967cf956c28ea35f47))
</details>

## 0.12.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.12.0 (2023-01-09)

A maintenance release without user-facing changes.

## 0.11.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.10.0 (2022-12-19)

### New Features

 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.

### Bug Fixes

 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning

## 0.9.0 (2022-11-21)

### Bug Fixes

 - <csr-id-6fc5c06482636540804f7a8fb74794f52c72eda6/> Discover repo with relative path and ceiling
   A couple of problems are repaired to allow discovering a repository from
   "." with a ceiling directory.
   
   One problem was that find_ceiling_height() did the wrong thing when
   confronted with any relative search_dir. This is resolved by converting
   search_dir to be absolute if it is relative.
   
   The other problem was that discover_opts() also mishandled relative paths.
   When the cursor started out as ".", cursor.pop() would be blindly called
   such that cursor would be "" for the second iteration. When a ceiling
   directory was in use such that there was a max_height, the current height
   would be burned going from ".", to "", and then to "<cwd>", before finally
   actually getting to a real parent directory. This problem is ameliorated by
   testing whether the cursor has a non-empty parent before popping.
   
   N.B. the new test case relies on the test running from the gix-discover
   directory such that the gitoxide repository will be found. This is a bit
   fragile and will fail if, for example, the test is run from an unpacked
   gitoxide source tarball.

### New Features (BREAKING)

 - <csr-id-b4dcfc716a80ffccbab6f7ccc586d8063cc10fff/> `Path::from_dot_git_dir()` now takes the `current_dir` as argument and returns `Option<path>`
   That way it's possible to avoid at least one call of
   `std::env::current_dir()` per invocation, which also is more consistent
   with similar plumbing methods.
   
   Furthermore it can signal with `None` if the input directory was invalid.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.8.0 (2022-11-17)

A maintenance release without user-facing changes.

## 0.7.0 (2022-11-06)

A maintenance release without user-facing changes.

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.2 (2022-09-01)

<csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/>
<csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/>

### Other

 - <csr-id-7057ad2251f76756a5b35b6bbba10ab9b6601226/> test `is_git` on macOS exfat
 - <csr-id-08d0c27b0641526aa8b70ee5aadd3658862bef1b/> allow opening repos on macos exfat

## 0.4.1 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 0.4.0 (2022-08-17)

### New Features

 - <csr-id-2e015a3874ab4db40449ec4b8ffb126d8e1a22a4/> add `is_submodule_git_dir()`
 - <csr-id-4a3e1cfbc537963b851c66754e7724875b49b1f4/> `is_git()` now detects submodule worktrees correctly.
 - <csr-id-aa6fd9702e16b4a2bedb0dd0d2323d06133d1a7f/> `gix_discover::is_git()` can detect submodule dirs correctly enough.
   We currently detect them as possibly bare, which could be improved if we
   allow ourselves to see `.git/modules` as `submodule` always.

### Changed (BREAKING)

 - <csr-id-1b0ef1856f542e5dc23310be93c2b39a9d84cb80/> Provide more details when classifying submodule directories.
   That way we can avoid feeding `.git` files to `git-repository::open()`
   and avoid work duplication, which ultimately allows to open submodules
   directories of all kinds.

## 0.3.0 (2022-07-22)

### New Features

 - <csr-id-010350180459aec41132c960ddafc7b81dd9c04d/> add `DOT_GIT_DIR` constant, containing the name ".git".

## 0.2.0 (2022-06-13)

<csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/>

### Other

 - <csr-id-6106521581029c5c24b23a47bb91c1921edfa0af/> avoid shortening absolute paths

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.1.3 (2022-05-27)

A maintenance release without user-facing changes.

## 0.1.2 (2022-05-23)

<csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/>

### Features

- `discover` now avoid crossing file-system boundaries by default on unix.

### Other

 - <csr-id-136eb37b00c9e7ba0fd0bc2a75dee2ac1b06516d/> adopt git-for-windows exception rules

## 0.1.1 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>
<csr-id-277c41f1185398e14f92247aada8422f6b08afd1/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### Other

 - <csr-id-277c41f1185398e14f92247aada8422f6b08afd1/> add cross_fs option to upwards discovery

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-050f795bfb0fe11655cd7e45c10d87c89ba82625/> allow discovery of linked worktree git dirs
   This also works if the work-tree can't be found but it is otherwise
   a valid git dir.

## 0.0.0 (2022-05-06)

The first release, an empty crate.

