# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.20.0 (2023-10-12)

### Bug Fixes

 - <csr-id-ac1d8d4a323fc5fb27b8c458f1babb4acc56ca52/> revert 'byteyarn' addition in favor of `kstring`
   It turns out not to work on certain 32bit targets that `gix` now has to support
   due to Fedora packaging.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 10 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1050](https://github.com/Byron/gitoxide/issues/1050)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1050](https://github.com/Byron/gitoxide/issues/1050)**
    - Revert "feat!: Use `yarn` for lower memory footprint and better performance." ([`355fd19`](https://github.com/Byron/gitoxide/commit/355fd191dbdae803c660781603d7986047760471))
 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Revert 'byteyarn' addition in favor of `kstring` ([`ac1d8d4`](https://github.com/Byron/gitoxide/commit/ac1d8d4a323fc5fb27b8c458f1babb4acc56ca52))
    - Thanks clippy ([`345712d`](https://github.com/Byron/gitoxide/commit/345712dcdfddcccc630bbfef2ed4f461b21550d3))
    - Merge branch 'improvements' ([`3939a45`](https://github.com/Byron/gitoxide/commit/3939a455be2269280248cdfed4a5983f8d178141))
    - Optimize Yarn instantiation ([`82874cd`](https://github.com/Byron/gitoxide/commit/82874cd04d6bb4635016391a43f51fca078195c7))
</details>

## 0.19.0 (2023-09-24)

### New Features (BREAKING)

 - <csr-id-7911d53b48263c35d9684e1a329588499eadd822/> Use `yarn` for lower memory footprint and better performance.
   However, this also removes `serde` support for now.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 14 calendar days.
 - 16 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
    - Merge branch 'yarn' ([`53bbd06`](https://github.com/Byron/gitoxide/commit/53bbd06d2b4380452d24d2c999f43b489b7446af))
    - Use `yarn` for lower memory footprint and better performance. ([`7911d53`](https://github.com/Byron/gitoxide/commit/7911d53b48263c35d9684e1a329588499eadd822))
</details>

## 0.18.0 (2023-09-08)

### New Features

 - <csr-id-2b8d09f785f471aa12fc6793f0ea40c1f8d9ea4a/> remove `log` dependency in favor of `gix-trace`
 - <csr-id-7df8cf31f794ae75868c606329a5685d198b0f59/> make it easier to access the value of `StateRef`.

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch 'optimizations' ([`6135a5e`](https://github.com/Byron/gitoxide/commit/6135a5ea8709646f01da62939a59dd3a9750e007))
    - Remove `log` dependency in favor of `gix-trace` ([`2b8d09f`](https://github.com/Byron/gitoxide/commit/2b8d09f785f471aa12fc6793f0ea40c1f8d9ea4a))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'improvements' ([`8a7c2af`](https://github.com/Byron/gitoxide/commit/8a7c2af0d302d5acc87ef2d432bd6870017af63e))
    - Make it easier to access the value of `StateRef`. ([`7df8cf3`](https://github.com/Byron/gitoxide/commit/7df8cf31f794ae75868c606329a5685d198b0f59))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.17.0 (2023-08-22)

<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>

### Chore

 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### New Features

 - <csr-id-6892999822008341ae3e6ef97b94ba2fedd4a800/> `search::Outcome::copy_into()` can now copy values from a previous search.
   This allows to perform a search once, finding all attributes, and
   then copy the results over to another outcome, typically one with
   selected attributes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 18 calendar days.
 - 30 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Merge branch 'pathspec-matching' ([`9f4dfe0`](https://github.com/Byron/gitoxide/commit/9f4dfe0f0b948280692916b596923959ea2fd9da))
    - `search::Outcome::copy_into()` can now copy values from a previous search. ([`6892999`](https://github.com/Byron/gitoxide/commit/6892999822008341ae3e6ef97b94ba2fedd4a800))
    - Add `Debug` to `search::Outcome` ([`72df372`](https://github.com/Byron/gitoxide/commit/72df372f1f53d54e9b3c644e86438cf7e2b9a05e))
    - Adapt to changes in `gix-glob` ([`f564fed`](https://github.com/Byron/gitoxide/commit/f564fedb0d047c71af0f093b41c2afcc63723910))
    - Adapt to changes in `gix-glob` ([`a943647`](https://github.com/Byron/gitoxide/commit/a9436472a9ef880adc8b47e2da221d42a783f89f))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## 0.16.0 (2023-07-22)

### Bug Fixes (BREAKING)

 - <csr-id-0634d543dacc9b3ce3d39938b116a890bcb0686f/> make information about whether or not the matched item is a directory available.
   This is important for matching, even though it works differently than excludes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Merge branch 'gix-archive' ([`1dda48b`](https://github.com/Byron/gitoxide/commit/1dda48ba2fccb93ebac00fe3460e923af43c86ce))
    - Make information about whether or not the matched item is a directory available. ([`0634d54`](https://github.com/Byron/gitoxide/commit/0634d543dacc9b3ce3d39938b116a890bcb0686f))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.15.0 (2023-07-19)

### Bug Fixes (BREAKING)

 - <csr-id-0634d543dacc9b3ce3d39938b116a890bcb0686f/> make information about whether or not the matched item is a directory available.
   This is important for matching, even though it works differently than excludes.

### Bug Fixes

 - <csr-id-8a361b46bc84428af88e5adc10a53b23bac41edf/> make sure that `search::Outcome` initialized with a selection works.
   Previously, the selection might not have been applied if it wasn't created
   after the respective attributes were loaded already.
   
   Now such an update happens during initialization (and efficiently so) so that
   there are no surprises.
   
   Also, implement `Clone` for more types where possible and for convenience.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 12 calendar days.
 - 19 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Make sure that `search::Outcome` initialized with a selection works. ([`8a361b4`](https://github.com/Byron/gitoxide/commit/8a361b46bc84428af88e5adc10a53b23bac41edf))
</details>

## 0.14.1 (2023-06-29)

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

## 0.14.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 10 calendar days.
 - 12 days passed between releases.
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

## 0.13.1 (2023-06-10)

### New Features

 - <csr-id-b061df82b88b1fa3670240ffc85220ef97b9d892/> add `StateRef::is_set()` and `StateRef::is_unset()` for convenience().
   Many users of attributes will just need to know that.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-attributes v0.13.1, gix-diff v0.30.1, gix-revwalk v0.1.0, gix-traverse v0.27.0, gix-index v0.18.0, gix-revision v0.15.2, gix-negotiate v0.2.1, gix-pack v0.37.0, gix-odb v0.47.0, gix-protocol v0.33.2, gix-worktree v0.19.0, gix v0.46.0, safety bump 7 crates ([`2560a2c`](https://github.com/Byron/gitoxide/commit/2560a2cc3e1d8c60cd812e15696fa4761d036e19))
    - Prepare changelogs prior to release ([`298f3d7`](https://github.com/Byron/gitoxide/commit/298f3d7359c5b183314d8c584e45dcdd559d88b3))
    - Merge branch 'walk-with-commitgraph' ([`fdee9a2`](https://github.com/Byron/gitoxide/commit/fdee9a22873a13ae644d3dc92f8fe93f8f0266c0))
    - Add `StateRef::is_set()` and `StateRef::is_unset()` for convenience(). ([`b061df8`](https://github.com/Byron/gitoxide/commit/b061df82b88b1fa3670240ffc85220ef97b9d892))
</details>

## 0.13.0 (2023-06-06)

### Bug Fixes (BREAKING)

 - <csr-id-c20ec28281958705fc7fda147fb37bdaf46eb17a/> Provide a means to prevent keeping any macro definitions.
   Previously we would simply skip macro lines when matching, but they would still
   contibute to our attribute listings.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 27 calendar days.
 - 40 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'gix-attributes-validate' ([`a849da8`](https://github.com/Byron/gitoxide/commit/a849da8e35ca14fef9a2431fe1bb1c05b249680e))
    - Provide a means to prevent keeping any macro definitions. ([`c20ec28`](https://github.com/Byron/gitoxide/commit/c20ec28281958705fc7fda147fb37bdaf46eb17a))
</details>

## 0.12.0 (2023-04-27)

A maintenance release without user-facing changes.

### New Features

 - <csr-id-c18482bd388c684c553d839f6360f085e68d754b/> `std::fmt::Display` for `AssignmentRef`.
   It displays exactly as it would in a `.gitattributes` file.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.8.0, gix-glob v0.7.0, gix-attributes v0.12.0, gix-config-value v0.12.0, gix-ref v0.29.0, gix-sec v0.8.0, gix-config v0.22.0, gix-prompt v0.5.0, gix-url v0.18.0, gix-credentials v0.14.0, gix-discover v0.18.0, gix-ignore v0.2.0, gix-pack v0.35.0, gix-odb v0.45.0, gix-transport v0.31.0, gix-protocol v0.32.0, gix-refspec v0.10.1, gix-worktree v0.17.0, gix v0.44.1 ([`7ebc9f7`](https://github.com/Byron/gitoxide/commit/7ebc9f734ec4371dd27daa568c0244185bb49eb5))
    - Prepare changelogs prior to release ([`0135158`](https://github.com/Byron/gitoxide/commit/013515897215400539bfd53c25548bd054186ba6))
    - Bump gix-path v0.8.0, safety bump 20 crates (gix set to 0.44.1 manually) ([`43ebaf2`](https://github.com/Byron/gitoxide/commit/43ebaf267557218865862538ffc7bdf00558492f))
    - `std::fmt::Display` for `AssignmentRef`. ([`c18482b`](https://github.com/Byron/gitoxide/commit/c18482bd388c684c553d839f6360f085e68d754b))
</details>

## 0.11.0 (2023-04-26)

### New Features

 - <csr-id-1eac372584e94689fd8b08d20d38d240dcfda26a/> add `Search::pop_pattern_list()`.
   Also, improve method documentation.

### New Features (BREAKING)

 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.
 - <csr-id-424ad62c852112b1104fe853b71de4dfe3a9f915/> an API for matching attributes.
   An implementation for `Search<Attributes>` along with various
   breaking modifications of the original module and type layout.

### Bug Fixes (BREAKING)

 - <csr-id-1d15173a5f492bd695f6456774c65261cddb5fe4/> Avoid lifetime in `Outcome`.
   Otherwise it's not possible to keep allocations for matches,
   while also adjusting pattern lists due to stack changes.
   
   On the bright side, this adds some more deduplication to the implementation,
   at the cost of dealing with some hashmaps that are updated when matching.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 23 calendar days.
 - 53 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'attributes-cache' ([`3456c84`](https://github.com/Byron/gitoxide/commit/3456c845dfeedd2fa96b4313b1a84c8cbe9433c5))
    - Avoid lifetime in `Outcome`. ([`1d15173`](https://github.com/Byron/gitoxide/commit/1d15173a5f492bd695f6456774c65261cddb5fe4))
    - Add `Search::pop_pattern_list()`. ([`1eac372`](https://github.com/Byron/gitoxide/commit/1eac372584e94689fd8b08d20d38d240dcfda26a))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Make fmt ([`5d2b5d0`](https://github.com/Byron/gitoxide/commit/5d2b5d02c3869e07dc2507a8f2519ee1df633df7))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Create new `gix-fs` crate to consolidate all filesystem utilities ([`f8cc33c`](https://github.com/Byron/gitoxide/commit/f8cc33cb372dd2b4bbe4a09cf4f64916681ab1dd))
    - Merge branch 'main' into dev ([`23ee47f`](https://github.com/Byron/gitoxide/commit/23ee47fb24c197f8437bd426544b2aa74e005bdc))
    - Merge branch 'worktree-stack' ([`3d47919`](https://github.com/Byron/gitoxide/commit/3d47919c1a2f83fc7c1fd7ae590d098057a22626))
    - An API for matching attributes. ([`424ad62`](https://github.com/Byron/gitoxide/commit/424ad62c852112b1104fe853b71de4dfe3a9f915))
    - Refactor ([`0677406`](https://github.com/Byron/gitoxide/commit/067740636b3ca24ce90db91923dfd4ee592fa7f6))
    - Merge branch 'patch-1' ([`d0052c1`](https://github.com/Byron/gitoxide/commit/d0052c13cabcde8058177d2439053b50ea5adbfc))
    - Upgrade various dependencies ([`f9ad837`](https://github.com/Byron/gitoxide/commit/f9ad83712deb53e0f8ac2be3cd35df8edcc5253c))
</details>

## 0.10.0 (2023-03-04)

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

## 0.9.0 (2023-03-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.27.0, gix-actor v0.18.0, gix-quote v0.4.3, gix-attributes v0.9.0, gix-object v0.27.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0 ([`e6cc618`](https://github.com/Byron/gitoxide/commit/e6cc6184a7a49dbc2503c1c1bdd3688ca5cec5fe))
    - Adjust manifests prior to release ([`addd789`](https://github.com/Byron/gitoxide/commit/addd78958fdd1e54eb702854e96079539d01965a))
    - Prepare changelogs prior to release ([`94c99c7`](https://github.com/Byron/gitoxide/commit/94c99c71520f33269cc8dbc26f82a74747cc7e16))
    - Merge branch 'adjustments-for-cargo' ([`d686d94`](https://github.com/Byron/gitoxide/commit/d686d94e1030a8591ba074757d56927a346c8351))
    - Change the internal representation of keys and values to allocate (avoiding compact_str). ([`d2e779c`](https://github.com/Byron/gitoxide/commit/d2e779c3ff5ae906f9a8a817361788d692972d79))
</details>

## 0.8.3 (2023-02-20)

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

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
    - Release gix-glob v0.5.4 ([`c56d336`](https://github.com/Byron/gitoxide/commit/c56d3365fde21120cf6101cf34f8b5669804977c))
</details>

## 0.8.2 (2023-02-17)

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

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-120d9085c35ac72d4b83daee7f2cb59fde91890e/> use git-glob crate for pattern parsing

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 225 commits contributed to the release over the course of 337 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#366](https://github.com/Byron/gitoxide/issues/366), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Adjust for different errors on windows when handling errors opening files… ([`9625829`](https://github.com/Byron/gitoxide/commit/962582996bb8d53739393acfcd150e9aa5132bae))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - Adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - Adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Adjustments to go along with changes in git-features ([`c55cac6`](https://github.com/Byron/gitoxide/commit/c55cac6a1ada77619bb5723717a5a6d757499fa9))
    - Add option to not follow symlinks when reading attribute files ([`5d619e6`](https://github.com/Byron/gitoxide/commit/5d619e66d48cf955958a0e844e832ed59756124f))
    - First primitive ignore pattern test works ([`0424136`](https://github.com/Byron/gitoxide/commit/04241367e8ce99ce6c7583d5dac4955fad3d6542))
    - Re-export `git-glob` as its `Case` type is part of the public API ([`4b72045`](https://github.com/Byron/gitoxide/commit/4b7204516a7c61162a2940eb66e8a7c64bf78ce7))
    - Sketch how attribute states can be used ([`d80b321`](https://github.com/Byron/gitoxide/commit/d80b321cf1863ad4436e69c0ee436f628f72531a))
    - Also skip negative attribute patterns ([`04ab5d3`](https://github.com/Byron/gitoxide/commit/04ab5d3ed351dc0d0b64226a3d8710ae8b522b70))
    - Allow basename matches to work like before ([`4f6cefc`](https://github.com/Byron/gitoxide/commit/4f6cefc96bea5f116eb26a9de8095271fd0f58e2))
    - Adapt to changes in git-glob and add failing test ([`cd58a1c`](https://github.com/Byron/gitoxide/commit/cd58a1c3445f97fd73d68e9f6b0af988806bea0d))
    - Refactor ([`fe9fb4c`](https://github.com/Byron/gitoxide/commit/fe9fb4cbab2def0d85fb1d961b911d7f7e62dbcc))
    - Fix crate release size by adding includes ([`f06f666`](https://github.com/Byron/gitoxide/commit/f06f666a8919a1a844a18aed8895a824534f1ae9))
    - Try using compatct_str for attribute storage ([`50b8c64`](https://github.com/Byron/gitoxide/commit/50b8c647c85793bd82dab1ac5bf6882884c2d11c))
    - Generalize parsing of paths in pattern lists ([`f66c27e`](https://github.com/Byron/gitoxide/commit/f66c27e41670d0702e78739bed5ca8575d22cf1a))
    - Support for loading per-directory pattern lists as well ([`457c921`](https://github.com/Byron/gitoxide/commit/457c921ef96c0d276e287009d3f8292ba1bface9))
    - More pendantic baseline parsing ([`99c7b5f`](https://github.com/Byron/gitoxide/commit/99c7b5fee5f95d9840238eb96077f3b4af5df7b8))
    - First succeding tests for global repository excludes ([`4a1e797`](https://github.com/Byron/gitoxide/commit/4a1e79780374726b84be0de44d1e1907c2a6a68e))
    - Enforce nicer/unified names so use struct instead of tuple ([`4c9a51e`](https://github.com/Byron/gitoxide/commit/4c9a51ee7206a90a07199d6a36a59f4e16a2d6bc))
    - Refactor ([`0852f13`](https://github.com/Byron/gitoxide/commit/0852f132b2d49b674891b85c401a8e4a9463e385))
    - Baseline tests for global excludes and instantiation of pattern lists from files ([`afbb295`](https://github.com/Byron/gitoxide/commit/afbb295b7917c183e0923e018428c7e51e9b6a96))
    - Basic match group pattern matching ([`cc1312d`](https://github.com/Byron/gitoxide/commit/cc1312dc06d1dccfa2e3cf0ae134affa9a3fa947))
    - Adapt to changes in git-glob ([`0effef0`](https://github.com/Byron/gitoxide/commit/0effef039b15417bbc225083d427ba1973bf1e0e))
    - Push base path handling to the caller ([`e4b57b1`](https://github.com/Byron/gitoxide/commit/e4b57b197884bc981b8e3c9ee8c7b5349afa594b))
    - Match group from overrides ([`f4f5a11`](https://github.com/Byron/gitoxide/commit/f4f5a115e9d3cf167eca1e213310c755e53f98e2))
    - A MatchGroup for later matching in stages, and for encapsulating some knoweldge about git repositories ([`0a5b5c4`](https://github.com/Byron/gitoxide/commit/0a5b5c4682bb321747398bb90041b12a2f8bf095))
    - A sketch of something that shouldn't be: a Description to instantiate patterns ([`388a8cd`](https://github.com/Byron/gitoxide/commit/388a8cd55a31d309f5c683645fc18ace6ddf4af3))
    - Adapt to changes in git-glob ([`229ac13`](https://github.com/Byron/gitoxide/commit/229ac135235ba96aff651fc865fab0d2cf61aea6))
    - Make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
    - A way to set a globs base path ([`3d58db8`](https://github.com/Byron/gitoxide/commit/3d58db8a9abfb91600216b8fc6f4109f5289d776))
    - Keep track of absolute patterns, those that have to start with it ([`3956480`](https://github.com/Byron/gitoxide/commit/3956480e6fb5f4766a67ebf2860cae2f48125594))
    - Also parse the position of the first wildcard ([`4178a63`](https://github.com/Byron/gitoxide/commit/4178a6356ad11013ae08b6233de2bfb366bf4278))
    - Prepare for upcoming wildcard-length field in glob pattern ([`a11f5d4`](https://github.com/Byron/gitoxide/commit/a11f5d441a22b844caefd31b9cb7783dd6b048ad))
    - Use git-glob crate for pattern parsing ([`120d908`](https://github.com/Byron/gitoxide/commit/120d9085c35ac72d4b83daee7f2cb59fde91890e))
    - A more realistic git-attributes file for parser testing ([`42aae32`](https://github.com/Byron/gitoxide/commit/42aae3232694656e5256d9b410e7b326118eac38))
    - Differentiate macro and attribute errors ([`a9e2b60`](https://github.com/Byron/gitoxide/commit/a9e2b608964eec7b6e4d7d7614941cc2e0e51ebd))
    - Refactor ([`eaab5a5`](https://github.com/Byron/gitoxide/commit/eaab5a5bc97c4cc16ac5b90d9f105b348fc816a2))
    - Macro parsing ([`0f677ce`](https://github.com/Byron/gitoxide/commit/0f677ceb7df4ec54ef615e4c4069f549e861f339))
    - Prepare for macro support ([`1981f6f`](https://github.com/Byron/gitoxide/commit/1981f6f8e8ab719bf4f67aabff9c72cf0ec1b25b))
    - Attribute name validation ([`65c416b`](https://github.com/Byron/gitoxide/commit/65c416bef3323250d0fb82085049ea68adae8001))
    - Parse all kinds of attributes, lacking name validation ([`96b0fca`](https://github.com/Byron/gitoxide/commit/96b0fcad1229ad2563e5e628d24289207a165005))
    - Very basic parsing of attributes ([`3409a66`](https://github.com/Byron/gitoxide/commit/3409a66a0b8f279d5c10ef4a948824e7809394da))
    - Add quote tests ([`93bf118`](https://github.com/Byron/gitoxide/commit/93bf1189902f3a6bff3ea5922bf62006b983e5b5))
    - A first stab at unquoting ansi_c style patterns ([`8ec7b30`](https://github.com/Byron/gitoxide/commit/8ec7b30f6bfaab8273c1007f16a7a1375fe46239))
    - All path-related tests are green ([`81d2bf2`](https://github.com/Byron/gitoxide/commit/81d2bf2ec5f571245d56eb853306d07ede3010a2))
    - Part of line handling implemented, but test still fails for good reason ([`311db97`](https://github.com/Byron/gitoxide/commit/311db977049216928bba66201620c3a08d05f07f))
    - API and first test for attributes parsing ([`ccc87de`](https://github.com/Byron/gitoxide/commit/ccc87defb4e739ccc1de8a0deae57233901f674d))
    - Refactor ([`3f62795`](https://github.com/Byron/gitoxide/commit/3f627954d2e992dd56eeee82a99f7ad41e619fb2))
    - Skip the BOM as well ([`0c256d3`](https://github.com/Byron/gitoxide/commit/0c256d3a60b83ae20575f26ac1a9152fd30c7b29))
    - Prepare for git-attribute file parsing ([`939d210`](https://github.com/Byron/gitoxide/commit/939d210de9f490f7e4014b11b7eae51dd801b596))
    - Refactor ([`9a9115f`](https://github.com/Byron/gitoxide/commit/9a9115f8db0a84818600f125b1185d1773f10d39))
    - Support for 'ends_with' matching mode ([`e9d222a`](https://github.com/Byron/gitoxide/commit/e9d222a19541e2c75370d3e2feeb24beec093859))
    - Iterator actually iterates all lines in a buffer ([`6a37eee`](https://github.com/Byron/gitoxide/commit/6a37eee5292bacdaca8c97608e900872128ae9bf))
    - Make line number accessible ([`0906bed`](https://github.com/Byron/gitoxide/commit/0906bedc7525979eb02192beb007f096cd6ac45f))
    - Implement most of the ignore flags ([`d95905f`](https://github.com/Byron/gitoxide/commit/d95905f57e10c90d615243ec692a81404b3571da))
    - Handle trailing whitespaces ([`9a5d089`](https://github.com/Byron/gitoxide/commit/9a5d089010c5f46dc0470a8611b88c532836f841))
    - A sketch of the parser API for ignore files ([`a161e33`](https://github.com/Byron/gitoxide/commit/a161e330fb90f23eb8760cd170316358c34f7359))
    - Name-crates for git-ignore and git-attributes handling ([`2e04a49`](https://github.com/Byron/gitoxide/commit/2e04a4934a42cc2bb90334cf75e4af2ab394cffa))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - All tests (so far) green ([`67a2050`](https://github.com/Byron/gitoxide/commit/67a2050156cc809767ca026f467f35b552bea043))
    - Fix serde support ([`2fb4310`](https://github.com/Byron/gitoxide/commit/2fb43102cf8bbfa9c26877d81d8fd3208fc5e183))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
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
    - Rename `git-attributes` to `gix-attributes` ([`16609e1`](https://github.com/Byron/gitoxide/commit/16609e11f47001f6fc79c33dddebca2e695a027a))
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
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Upgrade all dependencies, except for `windows` ([`2968181`](https://github.com/Byron/gitoxide/commit/29681819ffe53d3926d631dc482f71d6200cb549))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'pathspec' ([`7db59a4`](https://github.com/Byron/gitoxide/commit/7db59a4074111086adfc2f79fd0d26bb30303ca9))
    - Improve docs and use 'new-style' in error messages. ([`e36d83e`](https://github.com/Byron/gitoxide/commit/e36d83e62eb7969726e7c8b3d25dbb743a508f8a))
    - Add docs for `git-attributes` ([`0eabea9`](https://github.com/Byron/gitoxide/commit/0eabea9772ce67f70442bc8ded02a7e82f5c17cc))
    - Refactor ([`1cbc142`](https://github.com/Byron/gitoxide/commit/1cbc142d37599f4d7bfaf9cb07de41ee4b3f4c24))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Refactor ([`63baa75`](https://github.com/Byron/gitoxide/commit/63baa752901388a46a4211c70f3b3a64aa36d4ec))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Refactor of `Name` and `Assignment` ([`6449e77`](https://github.com/Byron/gitoxide/commit/6449e77e11ef0d25c2990f1c29e9fbea3c97fb0a))
    - Refactor ([`e83879c`](https://github.com/Byron/gitoxide/commit/e83879cee1666ba927a95c05c714d132a109eeef))
    - Impl '.as_ref()' for State ([`82074d5`](https://github.com/Byron/gitoxide/commit/82074d5e026c31b382cf97eade22eeca1bce3390))
    - Use "to_owned" instead of "into" ([`35c6d38`](https://github.com/Byron/gitoxide/commit/35c6d38088e09d88b30e12538e175a4a286980cd))
    - Assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Refactor ([`3be7a2d`](https://github.com/Byron/gitoxide/commit/3be7a2dc3cb8f476184555a1c62e230b7703db54))
    - Fix build ([`1838f3d`](https://github.com/Byron/gitoxide/commit/1838f3db13eae6d278264dcdbc48d202de992349))
    - Refactor ([`957356b`](https://github.com/Byron/gitoxide/commit/957356b4e5ea30ff5fa4390859f9e91093df9feb))
    - Fix rust fmt issue ([`a9cb68b`](https://github.com/Byron/gitoxide/commit/a9cb68b5e04e11dc1bd7a4dc152001f26a87a445))
    - Implement name::error for git-attributes ([`0849ebf`](https://github.com/Byron/gitoxide/commit/0849ebf4bc2052d7886f9425800a547bf530e967))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Refactor ([`9945ceb`](https://github.com/Byron/gitoxide/commit/9945ceb0a99c1343cb6e652e44900b36d3786e22))
    - Refactor ([`3b2bab8`](https://github.com/Byron/gitoxide/commit/3b2bab89172b86068bda9704bc9d69690bcfb2ba))
    - Quickerror to thiserror ([`da84b67`](https://github.com/Byron/gitoxide/commit/da84b675d3e825d2f815957fbed9928a0480ea4a))
    - Protected attribute name via "AttributeName" type ([`7bb408e`](https://github.com/Byron/gitoxide/commit/7bb408e631138854a6dff85ce356da96f61367de))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - Branch start, upgrade to compact_str v0.4 ([`b2f56d5`](https://github.com/Byron/gitoxide/commit/b2f56d5a279dae745d9c2c80ebe599c00e72c0d7))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Thanks clippy ([`5992883`](https://github.com/Byron/gitoxide/commit/59928836cb23fdc8bcf0d083ba05deccc0dbf7e0))
    - Thanks clippy ([`ac53780`](https://github.com/Byron/gitoxide/commit/ac537802dde00553f9f11908e5c484aa1c7153b6))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - Thanks clippy ([`d6787e4`](https://github.com/Byron/gitoxide/commit/d6787e4e05d24c2b36fcacf2346884fed62f2fec))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - Refactor ([`3e78ff5`](https://github.com/Byron/gitoxide/commit/3e78ff53125be2a75142534b6fd6f356b6bc8c5f))
    - Thanks clippy ([`365a8f0`](https://github.com/Byron/gitoxide/commit/365a8f08134a023bac7b78f3eee7baff410ba4cb))
    - Thanks clippy ([`32b0634`](https://github.com/Byron/gitoxide/commit/32b063477bc12b6b823de3dc390c3dd51012ba20))
    - Merge branch 'parse-git-ignore' ([`8ab19a6`](https://github.com/Byron/gitoxide/commit/8ab19a639b25b70872e89a933245abeea2b10ded))
    - Thanks clippy ([`f5639b6`](https://github.com/Byron/gitoxide/commit/f5639b688df78648479fe1666a7aa2ed65ea6753))
    - Release git-attributes v0.0.0 ([`5da2e98`](https://github.com/Byron/gitoxide/commit/5da2e98001d7602480fbf561355cfbe866bdf820))
    - Release git-ignore v0.0.0, git-attributes v0.0.0 ([`c128f27`](https://github.com/Byron/gitoxide/commit/c128f27df83be2473bd1788cc58118ca4c5ba407))
</details>

## 0.8.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.8.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.7.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.6.0 (2022-11-21)

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

## 0.5.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.4.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.3.3 (2022-08-27)

Maintenance release without user-facing changes.

## 0.3.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.3.1 (2022-08-17)

A maintenance release without user facing changes.

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.2.0 (2022-06-13)

A maintenance release without user-facing changes.

## 0.1.0 (2022-05-18)

### Changed (BREAKING)

 - <csr-id-120d9085c35ac72d4b83daee7f2cb59fde91890e/> use gix-glob crate for pattern parsing

## 0.0.0 (2022-03-17)

Initial release with no content.

