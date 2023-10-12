# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.6.0 (2023-10-12)

### Bug Fixes

 - <csr-id-b5b50f8fdb781b7c9a2ff5c8b462940e5098a94d/> hanging `Read` implementation of `pipeline::convert::ToGitOutcome`.
   This codepath was never tested and its function more subtle than one could have known.
   Also fix incorrect configuration handling which could lead to binary files with `text=auto`
   to be identified as text, which would then require conversion.

### Bug Fixes (BREAKING)

 - <csr-id-260c78191e88d2e2c744d85e6f75e7941c24ef43/> don't provide path to object-retrieval callback of `Pipeline::convert::to_git::IndexObjectFn()`.
   It implies that one has to be ready to fetch any kind of path from the index, even though it's always the path to
   the file that is currently converted.
   
   Also fix a bug that could cause it to return input as unchanged even though it was read into a buffer already.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 8 calendar days.
 - 17 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Fix docs ([`995bc84`](https://github.com/Byron/gitoxide/commit/995bc840664cbd4aeb7f95592e3125dee63bdcd4))
    - Thanks clippy ([`345712d`](https://github.com/Byron/gitoxide/commit/345712dcdfddcccc630bbfef2ed4f461b21550d3))
    - Merge branch 'reset' ([`b842691`](https://github.com/Byron/gitoxide/commit/b8426919a491dc3a7df01ee3f258fc0d8a3a327c))
    - Hanging `Read` implementation of `pipeline::convert::ToGitOutcome`. ([`b5b50f8`](https://github.com/Byron/gitoxide/commit/b5b50f8fdb781b7c9a2ff5c8b462940e5098a94d))
    - Don't provide path to object-retrieval callback of `Pipeline::convert::to_git::IndexObjectFn()`. ([`260c781`](https://github.com/Byron/gitoxide/commit/260c78191e88d2e2c744d85e6f75e7941c24ef43))
</details>

## 0.5.0 (2023-09-24)

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

## 0.4.0 (2023-09-08)

### Bug Fixes

 - <csr-id-96a07e08e6090222cf398b46aa8d46b56f81f14d/> Use `Vec::resize()` instead of set_len()
   Otherwise it's possible for uninitialized memory to be used as if it was initialized,
   which can lead to strange behaviour.
   
   As the buffer is re-used, it's not actually zeroing that much memory either.

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch 'optimizations' ([`6135a5e`](https://github.com/Byron/gitoxide/commit/6135a5ea8709646f01da62939a59dd3a9750e007))
    - Adapt to changes in `gix-worktree` ([`d7fc182`](https://github.com/Byron/gitoxide/commit/d7fc182156847752ee872016b6de0c78f5fb190b))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'perf-and-safety' ([`9ad9c5b`](https://github.com/Byron/gitoxide/commit/9ad9c5b1cfa3afff5273558b6ef98ca4714d4272))
    - Use `Vec::resize()` instead of set_len() ([`96a07e0`](https://github.com/Byron/gitoxide/commit/96a07e08e6090222cf398b46aa8d46b56f81f14d))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.3.0 (2023-08-22)

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
    - Merge branch 'worktree-organization' ([`8d0d8e0`](https://github.com/Byron/gitoxide/commit/8d0d8e005d7f11924a6717954d892aae5cec45e7))
    - Adapt to changes in `gix-worktree` ([`e5717e1`](https://github.com/Byron/gitoxide/commit/e5717e1d12c49285d31a90b03b7f8e9cbc6c1108))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.2.0 (2023-07-22)

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

## 0.1.0 (2023-07-19)

### New Features (BREAKING)

 - <csr-id-29a64c289946301d5e502ee956f3606280409faf/> make it possible to use filter pipeline with streaming for checkouts.
   This is done by providing a `gix_filter::Pipeline` to `checkout::Options` to control
   how filters are applied.
 - <csr-id-94df6e0e56c0c94e15da89dd9716bcdea5cd4380/> a complete, streaming compatible, `git` filter pipeline.
   It's configuring itself based on each input path and supports streaming.
   
   Some name changes where also performed while at it.

### Bug Fixes (BREAKING)

 - <csr-id-35415db5d82b82d3908709b6db177bb8d7c3542c/> improve conversion API of `Pipeline::convert_to_git(index_object)` to be more consistent and practical.
   `index_object` now receives the path to the entry as first argument, notably in a data-type that is fitting for
   the purpose.
   Additionally, implement `Clone` for `Pipeline`.
   We also make sure that the input stream is turned into a buffer in case there is no driver, previously filtering
   had nothing to work with if there was no driver.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 12 calendar days.
 - 18 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`4aca8c2`](https://github.com/Byron/gitoxide/commit/4aca8c2ae2ec588fb65ec4faa0c07c19d219569f))
    - Fix gix-filter dev-dependencies ([`a7f9441`](https://github.com/Byron/gitoxide/commit/a7f9441ed644cf1c2fd543bede179422d16dc1bf))
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Just fmt ([`a063c62`](https://github.com/Byron/gitoxide/commit/a063c62e3a30006d837b267e2ce74e70e48b4fb6))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Make it possible to use filter pipeline with streaming for checkouts. ([`29a64c2`](https://github.com/Byron/gitoxide/commit/29a64c289946301d5e502ee956f3606280409faf))
    - Improve conversion API of `Pipeline::convert_to_git(index_object)` to be more consistent and practical. ([`35415db`](https://github.com/Byron/gitoxide/commit/35415db5d82b82d3908709b6db177bb8d7c3542c))
    - A complete, streaming compatible, `git` filter pipeline. ([`94df6e0`](https://github.com/Byron/gitoxide/commit/94df6e0e56c0c94e15da89dd9716bcdea5cd4380))
</details>

## 0.0.0 (2023-07-01)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

An empty crate without any content to reserve the name for the gitoxide project.

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

### New Features

 - <csr-id-198ce27e05fd07d4e511dff1ac50c3f4ed163cdf/> API support for receiving delayed entries
 - <csr-id-5670bbba73c7d48ba6220c2e3773633c234fb21c/> Ability to steer long running filter processes.
 - <csr-id-c538c6eba466f22a3000a76a0d37902ac5256e7a/> ability to run define and run simple filters.
   Simple filters run in real-time and are piped their content to stdin
   while we read it from stdout.
 - <csr-id-d1fed3e9907d0a9e3fe45dbfe2ff27bd10b3e1f4/> `worktree::encode_to_worktree()` to turn UTf-8 into the worktree encoding.
 - <csr-id-1b8f2b7f51e7d17b9b0839f42b75781ae6f940ec/> `worktree::encode_to_git()` to turn worktree encoding to UTF-8.
 - <csr-id-1517cbc42c43b253046b7359c79731771fd7b941/> add `eol::convert_to_worktree()`.
   It's the inverse of `eol::convert_to_git()` to re-add CRLF where there were LF only.
 - <csr-id-e45fec9663f87b7ba4162a9517677f6278c20a98/> Add `eol::convert_to_git()`.
   This function supports all the logic that git executes to determine
   if a converion should actually be done.
 - <csr-id-b79ffeb9ed584c47f2609eea261e1ada557a744c/> `eol::Stats::from_bytes()` to obtain stats about a buffer.
   It can help to determine if it is binary and if line conversions should be performed at all.
 - <csr-id-306c8eabcffe80da1d627283c4b188a1b979f692/> add `ident::apply()` to substitute `$Id$` with `$Id: <hex>$`
 - <csr-id-496445ca97687a38ecb80e871a1cbdc7ecd6b313/> `ident::undo()` to replace `$Id: XXX$` with `$Id$`

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 438 calendar days.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#691](https://github.com/Byron/gitoxide/issues/691)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Add git-filter crate for name generation ([`5a3c628`](https://github.com/Byron/gitoxide/commit/5a3c628c757a7eb4ecfc5fd7265fa36c8362797b))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-filter v0.0.0 ([`c3ee57b`](https://github.com/Byron/gitoxide/commit/c3ee57b9d71f650784dc0a5022dbf54fe71e5fe5))
    - Use latest version of `gix-path` to allow `gix-filter` release. ([`bb9f308`](https://github.com/Byron/gitoxide/commit/bb9f3088aee60f140e960602429cc7eb46121b75))
    - Release gix-packetline-blocking v0.16.3, gix-filter v0.0.0 ([`fb3ad29`](https://github.com/Byron/gitoxide/commit/fb3ad29967d08558e42cbe8e80de5dd0b38f12c5))
    - Merge branch 'filter-programs' ([`97f8e96`](https://github.com/Byron/gitoxide/commit/97f8e960ed52538bb55b72f9dfc5f9d144d72885))
    - API support for receiving delayed entries ([`198ce27`](https://github.com/Byron/gitoxide/commit/198ce27e05fd07d4e511dff1ac50c3f4ed163cdf))
    - Handle process error codes similarly to how git does it ([`d00e6c5`](https://github.com/Byron/gitoxide/commit/d00e6c569a4320e334ef15a82867433497dc235e))
    - Make it possible to not run a process at all and test graceful shutdowns ([`cb74130`](https://github.com/Byron/gitoxide/commit/cb7413062e3ef46c896f552c889bd7984a35bd03))
    - Ability to steer long running filter processes. ([`5670bbb`](https://github.com/Byron/gitoxide/commit/5670bbba73c7d48ba6220c2e3773633c234fb21c))
    - Ability to run define and run simple filters. ([`c538c6e`](https://github.com/Byron/gitoxide/commit/c538c6eba466f22a3000a76a0d37902ac5256e7a))
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Merge branch 'worktree-encoding' ([`5af2cf3`](https://github.com/Byron/gitoxide/commit/5af2cf368dcd05fe4dffbd675cffe6bafec127e7))
    - `worktree::encode_to_worktree()` to turn UTf-8 into the worktree encoding. ([`d1fed3e`](https://github.com/Byron/gitoxide/commit/d1fed3e9907d0a9e3fe45dbfe2ff27bd10b3e1f4))
    - `worktree::encode_to_git()` to turn worktree encoding to UTF-8. ([`1b8f2b7`](https://github.com/Byron/gitoxide/commit/1b8f2b7f51e7d17b9b0839f42b75781ae6f940ec))
    - Refactor ([`7ae7ebd`](https://github.com/Byron/gitoxide/commit/7ae7ebd673b7062f7e4116e9ae4fc51a1451e34b))
    - Merge branch 'basic-filtering' ([`3fd5e16`](https://github.com/Byron/gitoxide/commit/3fd5e16e205db18edc21341fb4c2a75d0726f5a5))
    - Add `eol::convert_to_worktree()`. ([`1517cbc`](https://github.com/Byron/gitoxide/commit/1517cbc42c43b253046b7359c79731771fd7b941))
    - Add `eol::convert_to_git()`. ([`e45fec9`](https://github.com/Byron/gitoxide/commit/e45fec9663f87b7ba4162a9517677f6278c20a98))
    - `eol::Stats::from_bytes()` to obtain stats about a buffer. ([`b79ffeb`](https://github.com/Byron/gitoxide/commit/b79ffeb9ed584c47f2609eea261e1ada557a744c))
    - Refactor ([`9bb9c48`](https://github.com/Byron/gitoxide/commit/9bb9c48e0c935179885b774cd685bcaf1008c043))
    - Add `ident::apply()` to substitute `$Id$` with `$Id: <hex>$` ([`306c8ea`](https://github.com/Byron/gitoxide/commit/306c8eabcffe80da1d627283c4b188a1b979f692))
    - `ident::undo()` to replace `$Id: XXX$` with `$Id$` ([`496445c`](https://github.com/Byron/gitoxide/commit/496445ca97687a38ecb80e871a1cbdc7ecd6b313))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Rename `git-filter` to `gix-filter` ([`0875ae6`](https://github.com/Byron/gitoxide/commit/0875ae61d9e2e7553ea990ea1c391a26f0eb1a59))
    - Rename `git-filter` to `gix-filter` ([`4aa193f`](https://github.com/Byron/gitoxide/commit/4aa193f359ba31fc6ca7c3c28654e08b12ace6b3))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Release git-filter v0.0.0 ([`2465381`](https://github.com/Byron/gitoxide/commit/2465381626a1f6de58f45df2e68f36c6b585b68f))
</details>

