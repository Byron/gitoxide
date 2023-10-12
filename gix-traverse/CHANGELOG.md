# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.34.0 (2023-10-12)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 6 calendar days.
 - 17 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Fix docs ([`995bc84`](https://github.com/Byron/gitoxide/commit/995bc840664cbd4aeb7f95592e3125dee63bdcd4))
</details>

## 0.33.0 (2023-09-24)

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

## 0.32.0 (2023-09-08)

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

## 0.31.0 (2023-08-22)

<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>

### Chore

 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 18 calendar days.
 - 30 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge branch 'gix-submodule' ([`8f3f358`](https://github.com/Byron/gitoxide/commit/8f3f358800f1fe77d7ba7ebd396a90b692d3c0c1))
    - More cleanup of test crates ([`73c685a`](https://github.com/Byron/gitoxide/commit/73c685a67debcfa26a940f37bbca69cb3a4af57e))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.30.1 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.30.0 (2023-07-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 19 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`4aca8c2`](https://github.com/Byron/gitoxide/commit/4aca8c2ae2ec588fb65ec4faa0c07c19d219569f))
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
</details>

## 0.29.0 (2023-06-29)

### Bug Fixes

 - <csr-id-9cfc4aa318bc44c9e4310db7d3764b015472e1af/> use type for time consistently.
   This will allow it to be changed more easily later.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
    - Merge branch 'i64-times' ([`b407461`](https://github.com/Byron/gitoxide/commit/b407461d8991db67a5bdb2ab13f518f78a85ed40))
    - Adapt to changes in `gix-date` ([`fba45c6`](https://github.com/Byron/gitoxide/commit/fba45c68d57d5f73070a6949556a04187d42e427))
    - Use type for time consistently. ([`9cfc4aa`](https://github.com/Byron/gitoxide/commit/9cfc4aa318bc44c9e4310db7d3764b015472e1af))
</details>

## 0.28.0 (2023-06-22)

### Changed (BREAKING)

 - <csr-id-e4e8ddc8b6e38df66fc0461280074f975d2ffd11/> rename `commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan {time_in_seconds_since_epoch}` to `seconds`.
   This is possible now that there is a type for the time with that name.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - `just fmt` ([`871dd0b`](https://github.com/Byron/gitoxide/commit/871dd0b977caf17159092a4739ba5408403cdb2c))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Merge branch 'future-dates' ([`8d2e6a9`](https://github.com/Byron/gitoxide/commit/8d2e6a91ac92a033e9e3daad5cffa90263075536))
    - Rename `commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan {time_in_seconds_since_epoch}` to `seconds`. ([`e4e8ddc`](https://github.com/Byron/gitoxide/commit/e4e8ddc8b6e38df66fc0461280074f975d2ffd11))
    - Adapt to changes in `gix-actor` ([`4a80e86`](https://github.com/Byron/gitoxide/commit/4a80e868f9530896616e649838e9be64b6d10036))
    - Adapt to changes in `gix-date` ([`d575336`](https://github.com/Byron/gitoxide/commit/d575336c26e6026e463cd06d88266bb2bdd3e162))
</details>

## 0.27.0 (2023-06-10)

### New Features

 - <csr-id-6c5c66e10550923941b3d22e862f0f3f26d7ba03/> `commit::Ancestors::with_commit_graph(graph)` to set and use a commitgraph.
   That way, traversal speed is greatly improved for traversals that only need
   access to parents and commit time, while still being more efficient if it
   saves the caller to retrieve a commit again to parse it for parents.

### New Features (BREAKING)

 - <csr-id-8d7b62736848841e0577aa0498eb39aec492a2fa/> `gix_traverse::commit::Ancestors` now returns rich commit information.
   As part of the iteration, it will decode part of the commit and read parents and possibly
   the commit time if time-based sorting is desired. These values will now be made available
   instead of just the `id` of the commit.
   
   This saves the caller time to parse the commit again in case it needs similar information.

### Bug Fixes (BREAKING)

 - <csr-id-affc9df48e7b7540a95e2d6024f05b391f38c8f0/> previously the commit-traversal sorted by date must have been wonky & rename `Sorting::Topological` to `Sorting::BreadthFirst`.
   This is now fixed by using a proven data structure, the `BinaryHeap` as priority
   queue.
   
   The rename is to differentiate `git log --topo-order` from what we are doing, which is actually quite
   different.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 3 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-attributes v0.13.1, gix-diff v0.30.1, gix-revwalk v0.1.0, gix-traverse v0.27.0, gix-index v0.18.0, gix-revision v0.15.2, gix-negotiate v0.2.1, gix-pack v0.37.0, gix-odb v0.47.0, gix-protocol v0.33.2, gix-worktree v0.19.0, gix v0.46.0, safety bump 7 crates ([`2560a2c`](https://github.com/Byron/gitoxide/commit/2560a2cc3e1d8c60cd812e15696fa4761d036e19))
    - Prepare changelogs prior to release ([`298f3d7`](https://github.com/Byron/gitoxide/commit/298f3d7359c5b183314d8c584e45dcdd559d88b3))
    - Merge branch 'walk-with-commitgraph' ([`fdee9a2`](https://github.com/Byron/gitoxide/commit/fdee9a22873a13ae644d3dc92f8fe93f8f0266c0))
    - `commit::Ancestors::with_commit_graph(graph)` to set and use a commitgraph. ([`6c5c66e`](https://github.com/Byron/gitoxide/commit/6c5c66e10550923941b3d22e862f0f3f26d7ba03))
    - Previously the commit-traversal sorted by date must have been wonky & rename `Sorting::Topological` to `Sorting::BreadthFirst`. ([`affc9df`](https://github.com/Byron/gitoxide/commit/affc9df48e7b7540a95e2d6024f05b391f38c8f0))
    - `gix_traverse::commit::Ancestors` now returns rich commit information. ([`8d7b627`](https://github.com/Byron/gitoxide/commit/8d7b62736848841e0577aa0498eb39aec492a2fa))
</details>

## 0.26.0 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 25 calendar days.
 - 41 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge pull request #878 from blinxen/main ([`67da689`](https://github.com/Byron/gitoxide/commit/67da6894c8d8a24b982c732a1753a3e0a3300cc3))
    - Include missing changelog file in some crates ([`0269eed`](https://github.com/Byron/gitoxide/commit/0269eedc08c21589b5381d9b7d7fcc7004160bf8))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Auto-fix clippy to remove explicit iter looping ([`3eff567`](https://github.com/Byron/gitoxide/commit/3eff567c683b5c650c14792b68968cbdbc90ec5c))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Release gix-object v0.29.2 ([`4f879bf`](https://github.com/Byron/gitoxide/commit/4f879bf35653bdc8f9729d524c6e8e1fb3c6886b))
</details>

## 0.25.0 (2023-04-26)

### Bug Fixes

 - <csr-id-a70085ec6e0826720b8c203bc7d7fcf5a5d36041/> `Debug` for `commit::Sorting`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 25 calendar days.
 - 53 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#801](https://github.com/Byron/gitoxide/issues/801)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#801](https://github.com/Byron/gitoxide/issues/801)**
    - `Debug` for `commit::Sorting`. ([`a70085e`](https://github.com/Byron/gitoxide/commit/a70085ec6e0826720b8c203bc7d7fcf5a5d36041))
 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - Thanks clippy ([`14e64e7`](https://github.com/Byron/gitoxide/commit/14e64e74649cfb1f2f99da87015939af98fae5c8))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
    - Merge branch 'fix-801' ([`a884121`](https://github.com/Byron/gitoxide/commit/a88412194ff8960cd69a3794042d9c6c29428ea6))
</details>

## 0.24.0 (2023-03-04)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.28.0, gix-actor v0.19.0, gix-object v0.28.0, gix-diff v0.28.0, gix-traverse v0.24.0, gix-pack v0.32.0, safety bump 20 crates ([`0f411e9`](https://github.com/Byron/gitoxide/commit/0f411e93ec812592bb9d3a52b751399dd86f76f7))
    - Prepare changelogs prior to release of `gix-pack` ([`6db30ef`](https://github.com/Byron/gitoxide/commit/6db30ef6b5e931bbf12135507a3d922051de4d4b))
</details>

## 0.23.0 (2023-03-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
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
    - Make fmt ([`8ef1cb2`](https://github.com/Byron/gitoxide/commit/8ef1cb293434c7b9e1fda4a6963368e0435920a9))
</details>

## 0.22.2 (2023-02-20)

### New Features

 - <csr-id-9e391e916402aafa7a20c704d11e21a91bda63b5/> add `tree::Recorder::path_clone()` and `tree::Recorder::track_filename()` builder method.
   That way, the recorder can be used in other applications to keep track of the full
   path *or* the filename.

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.3, gix-diff v0.26.2, gix-traverse v0.22.2, gix v0.37.0, safety bump 3 crates ([`8b3e42f`](https://github.com/Byron/gitoxide/commit/8b3e42f69fe97fe5083eb845c323f10d7ac087b2))
    - Merge branch 'rename-tracking' ([`35415c5`](https://github.com/Byron/gitoxide/commit/35415c5061bf5ea90a04db80d06ac3622d0b0f1a))
    - Add `tree::Recorder::path_clone()` and `tree::Recorder::track_filename()` builder method. ([`9e391e9`](https://github.com/Byron/gitoxide/commit/9e391e916402aafa7a20c704d11e21a91bda63b5))
</details>

## 0.22.1 (2023-02-17)

<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>
<csr-id-7bce49c1d27cb279b61ff51de0038e01fcf3561e/>
<csr-id-cbc5b8171cdef5933d684c481300d9fcff43cf4b/>
<csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Refactor (BREAKING)

 - <csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/> remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - <csr-id-2290d006705ff47ad780b009fe58ee422b3285af/> move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

### Other (BREAKING)

 - <csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/> Avoid duplicate module paths in 'tree' and 'commit'

### Bug Fixes (BREAKING)

 - <csr-id-dcdf6573a90c7f9a6855aa7be8bf707b92b73ecf/> commit traversal now sorts by commit-time more thoroughly
   Previously the sorting was only partial as it was only among parents.
   Now it is full and among all currently visible commits.
   
   That way it is similar to what `git log` would produce.

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

 - <csr-id-83746f613a559a86a0ea81370fca3f094bc81e35/> require `Ancestors` traversal `find()` to return `Result`
   Previously it would return an `Option` and squelch legitimate errors
   which already let to quite a bug hunt.
   
   Now `find()` should always return the original error when finding an
   existing object, and keep it around in a Box.
   That way we don't add yet another type parameter.
   
   Performance implications are measurable, but marginable. The error type
   might be too big which is something we can indeed look at.
 - <csr-id-5cf9323dbe09789e806ed55d865281298af1a11b/> rename `commit::Ancestors::mode()` to `*::parents()`
   The previous name was too generic to be helpful or discoverable.

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

### Other

 - <csr-id-7bce49c1d27cb279b61ff51de0038e01fcf3561e/> commit traversal along the first parent…
   …which allows to traverse only the 'mainline' part of the commit
   ancestry, ignoring merges entirely.
   
   This kind of traversal is more suited to algorithms which want
   to see a linear history as well as segments of commits between
   references.
 - <csr-id-cbc5b8171cdef5933d684c481300d9fcff43cf4b/> try git-cliff…
   …but to no avail as it simpy won't work for us if we don't follow
   conventional commits properly, especially the absence of breaking
   changes support is a real issue.
   
   It's probably better to just implement it ourselves in smart-release.
 - <csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/> object_id

### Bug Fixes

 - <csr-id-153dad1450d1d0416601a07548e853253fe150f6/> Ordering of tips is now enforced if topo-order isn't chosen.
   Previously the starting tips might still have been unordered, despite
   their values being correct, which could have caused all kinds of issues
   down the road. The most important one was that traversal by date
   wouldn't work with more than one tip.
 - <csr-id-921f565087c8ad77b9035b46843a11c8cd8f12d3/> docs now highlight performance implications of commit traversal by commit time

### New Features

 - <csr-id-86a99a90eb992322099a870fbba10dddbcb80e83/> add `commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan(time)`.
   It allows to stop traversals early if all commmits to be traversed are
   older than a given time.
 - <csr-id-6b2af578fbff43999266c2af324070b013d1699f/> Make it possible to access the current commits buffer during commit ancestor iteration.
   This is useful to avoid additional lookups of the same object for
   reading additional data from it.
   
   Currently one needs an object cache to avoid duplciate object extraction
   work, with such a cache being slower than accessing the same buffer
   again.
 - <csr-id-eb36a3dda83a46ad59078a904f4e277f298a24e1/> Add sorting mode to ancestor traversal  #270

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 278 commits contributed to the release over the course of 658 calendar days.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 17 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#196](https://github.com/Byron/gitoxide/issues/196), [#198](https://github.com/Byron/gitoxide/issues/198), [#215](https://github.com/Byron/gitoxide/issues/215), [#222](https://github.com/Byron/gitoxide/issues/222), [#254](https://github.com/Byron/gitoxide/issues/254), [#266](https://github.com/Byron/gitoxide/issues/266), [#270](https://github.com/Byron/gitoxide/issues/270), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#384](https://github.com/Byron/gitoxide/issues/384), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/Byron/gitoxide/issues/164)**
    - Avoid duplicate module paths in 'tree' and 'commit' ([`2f2d856`](https://github.com/Byron/gitoxide/commit/2f2d856efe733d3cf81110c0e0607d2e7c40d968))
    - Object_id ([`329d183`](https://github.com/Byron/gitoxide/commit/329d183ad4e256a4f9cdeb34589b5f3432495f79))
 * **[#196](https://github.com/Byron/gitoxide/issues/196)**
    - Revert "traverse(chore): try git-cliff…" ([`cd293ae`](https://github.com/Byron/gitoxide/commit/cd293aee7cf7fefba9e1f61108eba5400e48b9a7))
    - Try git-cliff… ([`cbc5b81`](https://github.com/Byron/gitoxide/commit/cbc5b8171cdef5933d684c481300d9fcff43cf4b))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - Deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - Regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - Greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Commit traversal along the first parent… ([`7bce49c`](https://github.com/Byron/gitoxide/commit/7bce49c1d27cb279b61ff51de0038e01fcf3561e))
 * **[#215](https://github.com/Byron/gitoxide/issues/215)**
    - Refactor ([`9af2a94`](https://github.com/Byron/gitoxide/commit/9af2a9431005f6bd235881c34baf176b6fc9f686))
    - Rename `commit::Ancestors::mode()` to `*::parents()` ([`5cf9323`](https://github.com/Byron/gitoxide/commit/5cf9323dbe09789e806ed55d865281298af1a11b))
 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - Update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - Stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - Adapt to changes in git-odb ([`a44dd4b`](https://github.com/Byron/gitoxide/commit/a44dd4b5d1910856d7a21e156e7bca3138c04484))
    - Provide handle with a snapshot of the store's state ([`6e0cd6d`](https://github.com/Byron/gitoxide/commit/6e0cd6d38c5df874990ace6c2c3c0b39342c4d05))
    - Remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
    - Move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
 * **[#270](https://github.com/Byron/gitoxide/issues/270)**
    - Add test to validate parent mode is handled; don't clear object buffer ([`9498816`](https://github.com/Byron/gitoxide/commit/9498816b305697c25275c9f8a2ccd07835cf47ff))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Restrict signature changes to 'Ancestores::sorting()` ([`d71bd9d`](https://github.com/Byron/gitoxide/commit/d71bd9ded1e5e5a61a27be3d55f4b85ee4049bcf))
    - Commit traversal now sorts by commit-time more thoroughly ([`dcdf657`](https://github.com/Byron/gitoxide/commit/dcdf6573a90c7f9a6855aa7be8bf707b92b73ecf))
    - Higher-performance 'seen' tracking for commit traversal ([`8c530d1`](https://github.com/Byron/gitoxide/commit/8c530d1ff264b756bf627c905edc5801c17a6139))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
    - More speedy access to author/committer ([`6129607`](https://github.com/Byron/gitoxide/commit/61296077cebaaf2eb939fa6082121304bc6cf39b))
    - Fix docs ([`0a1caeb`](https://github.com/Byron/gitoxide/commit/0a1caebfcb5fe019fc8db3b51d16908da37be59f))
    - Require `Ancestors` traversal `find()` to return `Result` ([`83746f6`](https://github.com/Byron/gitoxide/commit/83746f613a559a86a0ea81370fca3f094bc81e35))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - Add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - Ordering of tips is now enforced if topo-order isn't chosen. ([`153dad1`](https://github.com/Byron/gitoxide/commit/153dad1450d1d0416601a07548e853253fe150f6))
    - Docs now highlight performance implications of commit traversal by commit time ([`921f565`](https://github.com/Byron/gitoxide/commit/921f565087c8ad77b9035b46843a11c8cd8f12d3))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Add `commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan(time)`. ([`86a99a9`](https://github.com/Byron/gitoxide/commit/86a99a90eb992322099a870fbba10dddbcb80e83))
    - Fix docs ([`60c136c`](https://github.com/Byron/gitoxide/commit/60c136ca38c6ad0cd7e0ea5e3e0e83bb828a9c94))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
    - Make it possible to access the current commits buffer during commit ancestor iteration. ([`6b2af57`](https://github.com/Byron/gitoxide/commit/6b2af578fbff43999266c2af324070b013d1699f))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - Prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
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
    - Adjust to renaming of `git-lfs` to `gix-lfs` ([`b9225c8`](https://github.com/Byron/gitoxide/commit/b9225c830daf1388484ee7e05f727990fdeff43c))
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
    - Rename `git-traverse` to `gix-traverse` ([`1781b34`](https://github.com/Byron/gitoxide/commit/1781b34a4f198b3f3d3dad6701c1f6391165dad3))
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
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Prepare changelogs prior to release ([`d679f5b`](https://github.com/Byron/gitoxide/commit/d679f5b6f018633e858d3ebbdaf1cd5098bbc5e7))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'optimize_hashtables' ([`95ad56c`](https://github.com/Byron/gitoxide/commit/95ad56c11489bc46d6eb2b2f48cf0bf01e954c58))
    - Use newly added git-hashtable ([`50cb436`](https://github.com/Byron/gitoxide/commit/50cb4362010e1a5799fe782df36ac5fcdb48dd8a))
    - Switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'breadthfirst-improvements' ([`b755b5b`](https://github.com/Byron/gitoxide/commit/b755b5bd4cbf8839ba43a143183ae785584f1d59))
    - Refactor ([`c0bfb42`](https://github.com/Byron/gitoxide/commit/c0bfb42f3b8bd7d94452d2023aab3901387178f9))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - First test to validate new sort-by-date with cutoff sorting mode (breaking) ([`0699c7e`](https://github.com/Byron/gitoxide/commit/0699c7e81152f68d21c882e251d2ae1aec581b61))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`2571831`](https://github.com/Byron/gitoxide/commit/2571831e5939bf4ea6f19537b0c1ccd71dc99088))
    - Prepare changelog  prior to release ([`fc6b958`](https://github.com/Byron/gitoxide/commit/fc6b9583d0534f70e0c8afdcad46e09a5001d62b))
    - Fix docs ([`34e899a`](https://github.com/Byron/gitoxide/commit/34e899ae0e4a23bba6dc36dfd7429c0d572736bd))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Use thiserror instead of quickerror ([`2ed0ee2`](https://github.com/Byron/gitoxide/commit/2ed0ee244f3f9d05d2dbedf4cf29b368e77405fa))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - Adapt to breaking changes in git-actor ([`40c48c3`](https://github.com/Byron/gitoxide/commit/40c48c390eb796b427ebd516dde92e9538ce5fb7))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - Thanks clippy ([`03d0660`](https://github.com/Byron/gitoxide/commit/03d06609002933f23abe37a7208841cd152bd63d))
    - Merge branch 'oknozor-feat/traversal-sort-by-committer-date' ([`6add377`](https://github.com/Byron/gitoxide/commit/6add3773c64a9155c236a36bd002099c218882eb))
    - Add sorting mode to ancestor traversal  #270 ([`eb36a3d`](https://github.com/Byron/gitoxide/commit/eb36a3dda83a46ad59078a904f4e277f298a24e1))
    - Ensure tests use 'merge.ff false' and recreate fixtures on each run ([`1d5ab44`](https://github.com/Byron/gitoxide/commit/1d5ab44145ccbc2064ee8cc7acebb62db82c45aa))
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
    - Release git-traverse v0.8.2 ([`1fd56ff`](https://github.com/Byron/gitoxide/commit/1fd56ff084c0acb6b2b7c237b11c27cb84ef6b2b))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - Release git-traverse v0.8.1 ([`1d508f1`](https://github.com/Byron/gitoxide/commit/1d508f18ebf02bb108881417e8613c1d42602b26))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com/Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] dissolve 'immutable' module ([`70e11c2`](https://github.com/Byron/gitoxide/commit/70e11c21b0637cd250f54381d5490e9976880ad9))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com/Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] fix docs ([`07be661`](https://github.com/Byron/gitoxide/commit/07be6611d1742633815566443f71eef8b85ad5c0))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - [actor #173] fix docs ([`2d7956a`](https://github.com/Byron/gitoxide/commit/2d7956a22511d73b767e443dac21b60e93f286dd))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-traverse v0.7.2 ([`4684f1c`](https://github.com/Byron/gitoxide/commit/4684f1c65d1110a4cb206986639cb1f793f0779e))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - Release git-traverse v0.7.1 ([`fbf2157`](https://github.com/Byron/gitoxide/commit/fbf21575a9810182811a078fdaa36c1eba2b0ade))
    - Remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com/Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com/Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - Revert "break more dev-depedency cycles up to git-odb" ([`22337ce`](https://github.com/Byron/gitoxide/commit/22337ce23995eee474e7dfb2e37fb56814522942))
    - (cargo-release) version 0.3.1 ([`7453184`](https://github.com/Byron/gitoxide/commit/7453184cc6445798cff9ab61aa0b792e2594751e))
    - Break more dev-depedency cycles up to git-odb ([`7ee278b`](https://github.com/Byron/gitoxide/commit/7ee278bf5b04adc5e4ab82cb83a3519f93587176))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - Clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - Refactor ([`a92f1e6`](https://github.com/Byron/gitoxide/commit/a92f1e68beb0f946d0e117934b244d5aa1b6b5fc))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - [git-odb] refactor ([`2958145`](https://github.com/Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com/Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] the world compiles again ([`f0c0e36`](https://github.com/Byron/gitoxide/commit/f0c0e36a1fb15d44776678567162ac754fdd26c0))
    - [git-odb] refactor ([`721303d`](https://github.com/Byron/gitoxide/commit/721303db232f87857aae58e12b342e5fb0139306))
    - [git-odb] refactor ([`ea224e9`](https://github.com/Byron/gitoxide/commit/ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50))
    - [git-odb] refactor ([`6a1b16a`](https://github.com/Byron/gitoxide/commit/6a1b16ae98edc9a694b945a12a7866eb17fc6be3))
    - Merge pull request #88 from avoidscorn/traverse-partial-ancestors ([`966f058`](https://github.com/Byron/gitoxide/commit/966f058beac9bec8277abb26b7cb3caf76df0cbf))
    - Replace OidPredicate+AlwaysTrue usage with bare function pointer. ([`6688073`](https://github.com/Byron/gitoxide/commit/6688073d92d478edf29fc78432bf7a451aafb850))
    - Support pruning subgraphs from ancestor traversal. ([`f057aa9`](https://github.com/Byron/gitoxide/commit/f057aa972360815265a93f6f578b80971ff22d29))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com/Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - [git-traverse] accept tree iterators instead of object ids ([`f343dad`](https://github.com/Byron/gitoxide/commit/f343dad60d34dfd88247a14c7e3de906a761cf2d))
    - [git-diff] refactor ([`087e853`](https://github.com/Byron/gitoxide/commit/087e85367c27bb3684c6ad543c7eae46762e5e44))
    - [git-traverse] refactor ([`323d92f`](https://github.com/Byron/gitoxide/commit/323d92ffff68c2515460a84224287306183f803d))
    - [git-traverse] refactor ([`85de287`](https://github.com/Byron/gitoxide/commit/85de2874087f64fc166797a3219eeb26be460619))
    - [git-traverse] refactor ([`e21f54f`](https://github.com/Byron/gitoxide/commit/e21f54fc10bc71cbdaba51c3ed599b52c861512a))
    - (cargo-release) version 0.3.0 ([`684de4b`](https://github.com/Byron/gitoxide/commit/684de4b376ecd4cc5330f7ac8643352ea9580ed3))
    - [pack-gen] Assure path-ids can be seen as unique. ([`e415c5a`](https://github.com/Byron/gitoxide/commit/e415c5a9d610a63a6df2ae143db02e561afdd163))
    - [pack-gen] quite a bit closer to tree-traversal for pack gen ([`ecd37ee`](https://github.com/Byron/gitoxide/commit/ecd37eea0154791bf9192d1225828e7d9b5ad530))
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com/Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.2.0 ([`3fb8377`](https://github.com/Byron/gitoxide/commit/3fb8377ff36422fe7607fb9172edf8bd5a4db995))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com/Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - Convenience methods for iterators ([`aa6c9e6`](https://github.com/Byron/gitoxide/commit/aa6c9e699a09b6b2b4f55aa75a1dd6f648eead09))
    - A sketch of convenient finding of commits ([`db21062`](https://github.com/Byron/gitoxide/commit/db210622b95d5f1f24d815cb35db5d46aa8a09e3))
    - Use latest round of convenience meethods ([`c977eb5`](https://github.com/Byron/gitoxide/commit/c977eb56e2cd62cd9bdf43195a0aa1f1293d2d64))
    - [traverse-tree] more proper docs ([`f66ccd3`](https://github.com/Byron/gitoxide/commit/f66ccd3fb0541aa02b01a0dad52d2eab7101dc8a))
    - Refactor ([`f7eb355`](https://github.com/Byron/gitoxide/commit/f7eb3554dd1c7e92c892b90413fa495197607949))
    - [traverse-tree] one test to pin implementation down a little ([`f0aeee1`](https://github.com/Byron/gitoxide/commit/f0aeee1ca3d9c0fd1290c1912226c7dae396e10b))
    - [traverse-tree] Allow skipping of nodes for a 10 fold speedup ([`3f9ee23`](https://github.com/Byron/gitoxide/commit/3f9ee23c1695e3a0dd9be0b2e6385c72dd90d01d))
    - Quick hack to see how fast tree traversal can be ([`7d389c8`](https://github.com/Byron/gitoxide/commit/7d389c877a6d3d6971493dfa4739abea28fae87b))
    - Quick hack of tree::breadthfirst::traverse ([`18ae6bd`](https://github.com/Byron/gitoxide/commit/18ae6bd53fb57577e8000c84c4d26ccb53cdd141))
    - Refactor ([`6447661`](https://github.com/Byron/gitoxide/commit/6447661a4254c23c7fccc052ea1858c16002b5be))
    - Refactor ([`cceff1c`](https://github.com/Byron/gitoxide/commit/cceff1cf5297a6e507f8b44672181ba2600c748c))
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com/Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - Rename 'Locate' to 'Find' - shorter and just as good ([`60f72f5`](https://github.com/Byron/gitoxide/commit/60f72f573a7696323e09bf4add80d5fbce22c99d))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com/Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - (cargo-release) version 0.1.0 ([`187f05a`](https://github.com/Byron/gitoxide/commit/187f05afda4589fdde4f8b28d2ac7bfb5dd9d82d))
    - [changes] more flexible handle of state ([`11db16b`](https://github.com/Byron/gitoxide/commit/11db16b585e7551fa0d85644ee085b95a9dc2c1e))
    - [traversal] more flexible state handling to allow people to not bother dereffing it ([`e1a35aa`](https://github.com/Byron/gitoxide/commit/e1a35aa59eb50209143320dc475c2cb4915c9d10))
    - [traversal] adapt tests to work with improved API ([`1ccaea9`](https://github.com/Byron/gitoxide/commit/1ccaea945f0363df33e8fe91e8d2ba67a099842b))
    - [traversal] use State to allow reuse ([`fcd36c2`](https://github.com/Byron/gitoxide/commit/fcd36c21f9cce1ec88871f1b1c506104b962eebc))
    - [traversal] first impl based on git-odb::traver ([`76a3017`](https://github.com/Byron/gitoxide/commit/76a3017b60d41957f5fea56bf7b2b2bf41aae0d5))
    - A new crate: git-traverse ([`1a9af50`](https://github.com/Byron/gitoxide/commit/1a9af50f1fca0e7e939f339b885c66dcb95e44e5))
</details>

## 0.22.0 (2023-01-06)

A maintenance release without user-facing changes.

## 0.21.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.20.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.19.0 (2022-11-21)

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

## 0.18.0 (2022-10-10)

### New Features

 - <csr-id-86a99a90eb992322099a870fbba10dddbcb80e83/> add `commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan(time)`.
   It allows to stop traversals early if all commmits to be traversed are
   older than a given time.

## 0.17.0 (2022-09-20)

### New Features

 - <csr-id-6b2af578fbff43999266c2af324070b013d1699f/> Make it possible to access the current commits buffer during commit ancestor iteration.
   This is useful to avoid additional lookups of the same object for
   reading additional data from it.
   
   Currently one needs an object cache to avoid duplciate object extraction
   work, with such a cache being slower than accessing the same buffer
   again.

## 0.16.4 (2022-09-01)

Maintenance release without user-facing changes.

## 0.16.3 (2022-08-28)

Maintenance release without user-facing changes.

## 0.16.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

## 0.16.1 (2022-08-17)

### Bug Fixes

 - <csr-id-153dad1450d1d0416601a07548e853253fe150f6/> Ordering of tips is now enforced if topo-order isn't chosen.
   Previously the starting tips might still have been unordered, despite
   their values being correct, which could have caused all kinds of issues
   down the road. The most important one was that traversal by date
   wouldn't work with more than one tip.
 - <csr-id-921f565087c8ad77b9035b46843a11c8cd8f12d3/> docs now highlight performance implications of commit traversal by commit time

## 0.16.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.15.0 (2022-05-18)

A maintenance release without user-facing changes.

## 0.14.0 (2022-04-05)

### Bug Fixes (BREAKING)

 - <csr-id-dcdf6573a90c7f9a6855aa7be8bf707b92b73ecf/> commit traversal now sorts by commit-time more thoroughly
   Previously the sorting was only partial as it was only among parents.
   Now it is full and among all currently visible commits.
   
   That way it is similar to what `git log` would produce.

## 0.13.0 (2022-04-03)

### Changed (BREAKING)

 - <csr-id-83746f613a559a86a0ea81370fca3f094bc81e35/> require `Ancestors` traversal `find()` to return `Result`
   Previously it would return an `Option` and squelch legitimate errors
   which already let to quite a bug hunt.
   
   Now `find()` should always return the original error when finding an
   existing object, and keep it around in a Box.
   That way we don't add yet another type parameter.
   
   Performance implications are measurable, but marginable. The error type
   might be too big which is something we can indeed look at.

## 0.12.0 (2022-01-23)

<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

### New Features

 - <csr-id-eb36a3dda83a46ad59078a904f4e277f298a24e1/> Add sorting mode to ancestor traversal  #270

### Changed (BREAKING)

 - <csr-id-5cf9323dbe09789e806ed55d865281298af1a11b/> rename `commit::Ancestors::mode()` to `*::parents()`
   The previous name was too generic to be helpful or discoverable.
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 -  move gix_pack::data::Object to gix_object::Data, massively alter gix_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

## 0.11.0 (2021-11-29)

A maintenance release, triggered by putting too many adjustments into a single commit.

## 0.10.1 (2021-11-16)

A maintenance release triggered by changes to gix-pack and changelog rewrites.

## v0.10.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `gix-hash`.

## v0.9.0 (2021-10-15)

<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-cbc5b8171cdef5933d684c481300d9fcff43cf4b/>
<csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/>
<csr-id-7bce49c1d27cb279b61ff51de0038e01fcf3561e/>

Some module paths have been removed to avoid path duplication, possibly leading to breakage.

### Other

 - <csr-id-cbc5b8171cdef5933d684c481300d9fcff43cf4b/> try git-cliff…
   …but to no avail as it simpy won't work for us if we don't follow
   conventional commits properly, especially the absence of breaking
   changes support is a real issue.
   
   It's probably better to just implement it ourselves in smart-release.
 - <csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/> object_id
 - <csr-id-7bce49c1d27cb279b61ff51de0038e01fcf3561e/> commit traversal along the first parent…
   …which allows to traverse only the 'mainline' part of the commit
   ancestry, ignoring merges entirely.
   
   This kind of traversal is more suited to algorithms which want
   to see a linear history as well as segments of commits between
   references.

### BREAKING

 - Avoid duplicate module paths in `tree` and `commit`

## v0.8.2 (2021-09-08)

## v0.8.1 (2021-09-07)

## v0.8.0 (2021-08-27)

## v0.7.2 (2021-08-17)

## v0.7.1 (2021-08-13)

## v0.7.0 (2021-08-12)

## v0.5.0 (2021-08-11)

## v0.4.0 (2021-08-11)

## v0.3.1 (2021-08-10)

## v0.3.0 (2021-08-10)

## v0.2.0 (2021-05-09)

## v0.1.0 (2021-04-30)

## v0.0.0 (2021-04-30)

