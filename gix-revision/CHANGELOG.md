# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.23.0 (2023-10-12)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 17 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.13.1, gix-features v0.36.0, gix-actor v0.28.0, gix-object v0.38.0, gix-glob v0.14.0, gix-attributes v0.20.0, gix-command v0.2.10, gix-filter v0.6.0, gix-fs v0.8.0, gix-commitgraph v0.22.0, gix-revwalk v0.9.0, gix-traverse v0.34.0, gix-worktree-stream v0.6.0, gix-archive v0.6.0, gix-tempfile v11.0.0, gix-lock v11.0.0, gix-ref v0.38.0, gix-config v0.31.0, gix-url v0.25.0, gix-credentials v0.21.0, gix-diff v0.37.0, gix-discover v0.26.0, gix-ignore v0.9.0, gix-index v0.26.0, gix-mailmap v0.20.0, gix-negotiate v0.9.0, gix-pack v0.44.0, gix-odb v0.54.0, gix-pathspec v0.4.0, gix-packetline v0.16.7, gix-transport v0.37.0, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0, safety bump 37 crates ([`68e5432`](https://github.com/Byron/gitoxide/commit/68e54326e527a55dd5b5079921fc251615833040))
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
</details>

## 0.22.0 (2023-09-24)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
</details>

## 0.21.0 (2023-09-08)

### New Features

 - <csr-id-397024bb744af0c2e0fc66674e55998a40c24ae4/> add `describe` feature
   That way users can more precisely decide what they want to use.
   Note that spec-parsing is so foundational that it's always included.
   Those who don't need it nor need describe don't need the crate in the
   fist place.

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 17 calendar days.
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
    - Release gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0 ([`1ff3064`](https://github.com/Byron/gitoxide/commit/1ff30641b8724efd6699d8bef5c71d28454e98b9))
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch 'optimizations' ([`6135a5e`](https://github.com/Byron/gitoxide/commit/6135a5ea8709646f01da62939a59dd3a9750e007))
    - Add `describe` feature ([`397024b`](https://github.com/Byron/gitoxide/commit/397024bb744af0c2e0fc66674e55998a40c24ae4))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'adjustments-for-cargo' ([`b7560a2`](https://github.com/Byron/gitoxide/commit/b7560a2445b62f888bf5aa2ba4c5a47ae037cb23))
    - Release gix-date v0.7.4, gix-index v0.23.0, safety bump 5 crates ([`3be2b1c`](https://github.com/Byron/gitoxide/commit/3be2b1ccfe30eeae45711c64b88efc522a2b51b7))
    - Thanks clippy ([`5044c3b`](https://github.com/Byron/gitoxide/commit/5044c3b87456cf58ebfbbd00f23c9ba671cb290c))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.20.0 (2023-08-22)

<csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/>
<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>
<csr-id-353b1a788a7c5a627ec73185f841ea4893a147a5/>
<csr-id-145125ab79526a6191a9192a6faa7fe1a8826935/>

### Chore

 - <csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/> switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance.
   It's likely that over time, these parsers will get even faster due to improvements to `winnow`.
   Thanks, Ed Page, for single-handedly performing this transition.
 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.
 - <csr-id-353b1a788a7c5a627ec73185f841ea4893a147a5/> add benchmarks to avoid parsing performance regressions
 - <csr-id-145125ab79526a6191a9192a6faa7fe1a8826935/> use `faster-hex` instead of `hex`
   The performance here certainly doesn't make a difference, but we
   try to avoid duplicate dependencies.

### New Features

 - <csr-id-f4a9a6b574ad4521d475e9088073c0f15d56d079/> add tracing support to `describt()`.

### Bug Fixes

 - <csr-id-e0432d1547e254fee81f8188ab0a5dccfb4f6336/> replace `libflate2` with `flage2` for buidling a `gz` file.
   This brings streaming support and better performance, while also
   supporting compression settings.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 14 calendar days.
 - 15 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0 ([`6c62e74`](https://github.com/Byron/gitoxide/commit/6c62e748240ac0980fc23fdf30f8477dea8b9bc3))
    - Make `gix-url` publishable by adding baseline test ([`d3746df`](https://github.com/Byron/gitoxide/commit/d3746df5dd402d8a461c2b07eaa0f8d8803fadf8))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge branch 'gix-submodule' ([`8f3f358`](https://github.com/Byron/gitoxide/commit/8f3f358800f1fe77d7ba7ebd396a90b692d3c0c1))
    - Add tracing support to `describt()`. ([`f4a9a6b`](https://github.com/Byron/gitoxide/commit/f4a9a6b574ad4521d475e9088073c0f15d56d079))
    - Switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance. ([`ef54aab`](https://github.com/Byron/gitoxide/commit/ef54aab9e5521add4154ee8d902d62612a9d8d4a))
    - Upgrade `winnow` to latest patch release ([`8c41848`](https://github.com/Byron/gitoxide/commit/8c4184817e4e4364c34badc8ff0a71c6ae952efd))
    - Add fuzz-issue for reproduction ([`510192e`](https://github.com/Byron/gitoxide/commit/510192e0e5750bdfe461d701b3e124c03f22b7d9))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Add benchmarks to avoid parsing performance regressions ([`353b1a7`](https://github.com/Byron/gitoxide/commit/353b1a788a7c5a627ec73185f841ea4893a147a5))
    - Merge branch 'faster-hex' ([`4a4fa0f`](https://github.com/Byron/gitoxide/commit/4a4fa0fcdaa6e14b51d3f03f5d7c5b65042667bf))
    - Use `faster-hex` instead of `hex` ([`145125a`](https://github.com/Byron/gitoxide/commit/145125ab79526a6191a9192a6faa7fe1a8826935))
    - Merge branch 'archive-gz' ([`c7d9129`](https://github.com/Byron/gitoxide/commit/c7d912917a2dad5c076d0bd645cfda092c66ff79))
    - Replace `libflate2` with `flage2` for buidling a `gz` file. ([`e0432d1`](https://github.com/Byron/gitoxide/commit/e0432d1547e254fee81f8188ab0a5dccfb4f6336))
</details>

## 0.19.0 (2023-08-07)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 4 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Prepare changelogs prior to release of `gix-submodule` ([`f3c4311`](https://github.com/Byron/gitoxide/commit/f3c43110e8d5f16cf87e50821044d8b3edbae235))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
    - Update `time` crate explicitly in Cargo.toml to latest version ([`e145a74`](https://github.com/Byron/gitoxide/commit/e145a7489dd5e1a7c3458428ecbd101e7b53536b))
</details>

## 0.18.1 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`0062971`](https://github.com/Byron/gitoxide/commit/00629710dffeb10fda340665530353703cf5d129))
    - Release gix-tempfile v7.0.2, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0 ([`107a64e`](https://github.com/Byron/gitoxide/commit/107a64e734580ad9e2c4142db96394529d8072df))
    - Prepare yet another changelog ([`8451aac`](https://github.com/Byron/gitoxide/commit/8451aace5161ec6f5950318d6bfd042e7c6c62cc))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.18.0 (2023-07-19)

<csr-id-7f7db9794c23b87c8ea50b7bcf38955c9d977624/>

### Chore

 - <csr-id-7f7db9794c23b87c8ea50b7bcf38955c9d977624/> curtail `bstr` features to exactly what's needed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 17 calendar days.
 - 19 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`4aca8c2`](https://github.com/Byron/gitoxide/commit/4aca8c2ae2ec588fb65ec4faa0c07c19d219569f))
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Curtail `bstr` features to exactly what's needed. ([`7f7db97`](https://github.com/Byron/gitoxide/commit/7f7db9794c23b87c8ea50b7bcf38955c9d977624))
</details>

## 0.17.0 (2023-06-29)

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

## 0.16.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 10 calendar days.
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
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
</details>

## 0.15.2 (2023-06-10)

<csr-id-4deea8a9cf5a9e77ec138311267c9853172db14c/>

### Other

 - <csr-id-4deea8a9cf5a9e77ec138311267c9853172db14c/> Add information on how to turn `LazyCommit` into commit.
   This is to more easily support custm graph walks later.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
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
    - Adapt to changes in `gix-revwalk` ([`f7d95d1`](https://github.com/Byron/gitoxide/commit/f7d95d189af1422a7ba48db1857452e32e1d9db9))
    - Add new `gix-revwalk` crate for support types related to revision walking. ([`13ce887`](https://github.com/Byron/gitoxide/commit/13ce887682f5c31d1f78a63613ca97b811e4ffba))
    - Add information on how to turn `LazyCommit` into commit. ([`4deea8a`](https://github.com/Byron/gitoxide/commit/4deea8a9cf5a9e77ec138311267c9853172db14c))
</details>

## 0.15.1 (2023-06-06)

### Bug Fixes

 - <csr-id-fd2593cc1e5824687f3bcc4be4badc3d7920e5fc/> avoid duplicate error message
   "There was an error looking up a commit" could be printed twice in error stacks
   due to a copy-paste error.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-revision v0.15.1, gix v0.45.1 ([`11766a0`](https://github.com/Byron/gitoxide/commit/11766a0a82754fee9918ccdb8eaf92af6d2561ba))
    - Merge branch 'adjustments-for-cargo' ([`04f011c`](https://github.com/Byron/gitoxide/commit/04f011c3c3e49e87a3b868d4bf6e77a361b96da8))
    - Avoid duplicate error message ([`fd2593c`](https://github.com/Byron/gitoxide/commit/fd2593cc1e5824687f3bcc4be4badc3d7920e5fc))
</details>

## 0.15.0 (2023-06-06)

<csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/>

### Chore

 - <csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/> inline format args

### New Features

 - <csr-id-1bd93bedd2f184510239c50c345d3dbc41d7d13b/> allow graph sharing by unifying `Flags` type.
   This makes the graph used in `gix-negotiate` shareable by callers, which can
   do their own traversal and store their own flags. The knowlege of this traversal
   can be kept using such shared flags, like the `PARSED` bit which should be set whenever
   parents are traversed.
   
   That way we are able to emulate the algorithms git uses perfectly, as we keep exactly the
   same state.
 - <csr-id-9ab205102eacaf0758c143941f43831a481a1f06/> various improvements to the API
   * make `CommitterTimestamp` available as type, making the code using it more descriptive.
* add `new()` to `PriorityQueue`
* add `Graph::try_lookup_and_insert_default()`
* add `Debug` impl for `Graph`

### New Features (BREAKING)

 - <csr-id-11ad8a890a6233befb5d2b6b41caadbcb296c3f5/> Add version of Graph that handles fully-parsed commits
   This renames `graph::Commit` to `graph::LazyCommit` to make space for `graph::Commit` to be a fully owned.
   `LazyCommit::to_owned()` was added to obtain fully owned `Commit` instances.
   Rename `Graph::try_lookup_and_insert()` to `Graph::try_lookup_or_insert()` and
   `Graph::try_lookup_and_insert_default()` to `Graph::try_lookup_or_insert_default()`
   
   Additionally, add the `peek()` and `iter_unordered()` method to the `PriorityQueue`, along with an implementation for `Clone`
   Rename `PriorityQueue::iter_random()` to `::iter_unordered()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 13 calendar days.
 - 18 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'integrate-gix-negotiate' ([`ae845de`](https://github.com/Byron/gitoxide/commit/ae845dea6cee6523c88a23d7a14293589cf8092f))
    - Allow graph sharing by unifying `Flags` type. ([`1bd93be`](https://github.com/Byron/gitoxide/commit/1bd93bedd2f184510239c50c345d3dbc41d7d13b))
    - Add version of Graph that handles fully-parsed commits ([`11ad8a8`](https://github.com/Byron/gitoxide/commit/11ad8a890a6233befb5d2b6b41caadbcb296c3f5))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge pull request #864 from nyurik/lint-fmt ([`279dc09`](https://github.com/Byron/gitoxide/commit/279dc09446f41d7f1d76350fbfafb444e53cd7da))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Inline format args ([`dbc6cbb`](https://github.com/Byron/gitoxide/commit/dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'consecutive-negotiation' ([`97b3f7e`](https://github.com/Byron/gitoxide/commit/97b3f7e2eaddea20c98f2f7ab6a0d2e2117b0793))
    - Various improvements to the API ([`9ab2051`](https://github.com/Byron/gitoxide/commit/9ab205102eacaf0758c143941f43831a481a1f06))
</details>

## 0.14.0 (2023-05-19)

### New Features

 - <csr-id-59ce4c606f8ccd9b6a16da2025e6746984d32fd6/> A Graph for quick access to commits and for associating state with them.
   This data structure should be used whenever stateful traversal is required, usually
   by associating information with each commit to remember what was seen and what wasn't.
 - <csr-id-dde8c3aca545ba20cd5752f02283b98647fd3970/> A PriorityQueue that is useful for graph traversal.

### New Features (BREAKING)

 - <csr-id-ed258da9015d2d68734aeac485dd009760fc4da4/> `describe` usees commitgraph.
   With it it can leverage the commitgraph data structure would would be more prominent
   on server-side applications, presumably.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 22 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-commitgraph v0.15.0, gix-revision v0.14.0, gix-negotiate v0.1.0, safety bump 7 crates ([`92832ca`](https://github.com/Byron/gitoxide/commit/92832ca2899cd2f222f4c7b1cc9e766178f55806))
    - Merge branch 'consecutive-negotiation' ([`4507f94`](https://github.com/Byron/gitoxide/commit/4507f94984c811ea098e43472e5f54ec4dbb90c1))
    - `describe` usees commitgraph. ([`ed258da`](https://github.com/Byron/gitoxide/commit/ed258da9015d2d68734aeac485dd009760fc4da4))
    - A Graph for quick access to commits and for associating state with them. ([`59ce4c6`](https://github.com/Byron/gitoxide/commit/59ce4c606f8ccd9b6a16da2025e6746984d32fd6))
    - A PriorityQueue that is useful for graph traversal. ([`dde8c3a`](https://github.com/Byron/gitoxide/commit/dde8c3aca545ba20cd5752f02283b98647fd3970))
    - Make clear that we do handle shallow repos, refactor tests ([`fc423e4`](https://github.com/Byron/gitoxide/commit/fc423e470de50491a725d4802066d26c05bd2b2a))
    - Release gix-object v0.29.2 ([`4f879bf`](https://github.com/Byron/gitoxide/commit/4f879bf35653bdc8f9729d524c6e8e1fb3c6886b))
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

 - 11 commits contributed to the release over the course of 14 calendar days.
 - 25 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`d7173b2`](https://github.com/Byron/gitoxide/commit/d7173b2d2cb79685fdf7f618c31c576db24fa648))
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`e4df557`](https://github.com/Byron/gitoxide/commit/e4df5574c0813a0236319fa6e8b3b41bab179fc8))
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'fix-823' ([`6ebd61e`](https://github.com/Byron/gitoxide/commit/6ebd61e548a36a04e413ac725a03e607a3588334))
    - Thanks clippy ([`14e64e7`](https://github.com/Byron/gitoxide/commit/14e64e74649cfb1f2f99da87015939af98fae5c8))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
</details>

## 0.12.2 (2023-04-01)

<csr-id-1c27e7a3745b156ea953e430f726576389fad5f2/>

### Bug Fixes

 - <csr-id-cfaab7ffc67d33224afe4b92e42059c0bb88ea02/> Parse revisions with `@` in their name.
   Previously these would cause a parse error due to confusing `@` with
   the short form of `HEAD`.
   
   Merge branch 'fix-rev-parse-with-at'

### Other

 - <csr-id-1c27e7a3745b156ea953e430f726576389fad5f2/> Parse revisions with the @ character
   Fixes https://github.com/Byron/gitoxide/issues/802

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#802](https://github.com/Byron/gitoxide/issues/802)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#802](https://github.com/Byron/gitoxide/issues/802)**
    - Parse revisions with `@` in their name. ([`cfaab7f`](https://github.com/Byron/gitoxide/commit/cfaab7ffc67d33224afe4b92e42059c0bb88ea02))
 * **Uncategorized**
    - Release gix-revision v0.12.2 ([`ec64a88`](https://github.com/Byron/gitoxide/commit/ec64a88690243a210efee6d5ae5164723e13f734))
    - Refactor ([`c0905ce`](https://github.com/Byron/gitoxide/commit/c0905ce74f1bef4c42c9729e2bcf267d7aa6af5e))
    - Parse revisions with the @ character ([`1c27e7a`](https://github.com/Byron/gitoxide/commit/1c27e7a3745b156ea953e430f726576389fad5f2))
</details>

## 0.12.1 (2023-03-26)

A maintenance release without any user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 3 calendar days.
 - 21 days passed between releases.
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

## 0.12.0 (2023-03-04)

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

## 0.11.0 (2023-03-01)

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

## 0.10.4 (2023-02-20)

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

## 0.10.3 (2023-02-17)

<csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

A maintenance release without user-facing changes.

### Reverted (BREAKING)

 - <csr-id-2761466ef6734ad6484548d7a93e52db3a230864/> hash_hasher re-export in favor of using `git-hashtable`.
   Due to the importance of best-suited data structures for maximizing
   performance we need to take control over them. This is best done using
   a dedicated crate that can cater to our very needs. That very crate is
   named `git-hashtable`.

### Refactor (BREAKING)

 - <csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/> Make `describe::Format` more consistent with other builder APIs
   Configuration methods now take an argument which makes it more
   straightforward to use for most.

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
 - <csr-id-42aea42c1f6c2a9681688825a9e31966bca1896c/> More intuitive variants for `Spec`.
 - <csr-id-487941ce557182c7ad02958e011959acb2dd5607/> rename various `Kind` variants to be more descrptive.
 - <csr-id-baf34c486f54e4699f88b06a0f8cbb10f0582bd0/> Rename `Kind::Single` to `Include` and add `Exclude` kind.
   So far I got ranges pretty wrong and was degenerating the `^rev` case
   due to misinterpretation of the docs.
   This summary corrected that: https://git-scm.com/docs/git-rev-parse#_revision_range_summary

### Bug Fixes

 - <csr-id-786f6dc5c1f765b9598cd55ca8fb1714ad177e46/> prevent panics from dates which cannot be represented by the `time` crate
 - <csr-id-4788270853d42be8405465a6b9b612783ae9ef6e/> decscribe() won't abort before the first name check if max-candidates is 0
   A test was missing too, which is now fixed.
 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-df62f5081291f65f994b2aa66f0599f47eea8d4d/> `describe()` aborts search early if there is no input name in the name map.
 - <csr-id-ca6651234a8c0d4718554323b197a49266b60a61/> revision describe can now short-cut what effectively is only a name-to-id lookup
   This makes situations easier where `max-candidates` is provided by the user or by
   configuration.
 - <csr-id-36c70e1f4ce07bf69d7064de1b6f0516d13d8acf/> `Spec` with `Display` implementation to reproduce itself.
   That way it can be parsed back perfectly after displaying itself, and
   will work normally when used in backticks in the shell for simple
   include patterns.
 - <csr-id-5038ffab6a0f83e0566f99e3c92ae2dea266e10b/> Add `Spec` data strcuture to fully represent a revision specification
 - <csr-id-4bb200300b1665cab49b780ae13c277630b70f51/> Add support for `r1^@`
 - <csr-id-7e5d31cb253f994ef19b15978c5df0f3a7ccebb1/> Add support for `r1^!`
 - <csr-id-fa1615da63594acbe92c3c4a13e2aeb7c1ee1d94/> support for `<rev>^-<n>` and `<rev>^-`

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 328 commits contributed to the release over the course of 377 calendar days.
 - 22 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 11 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364), [#384](https://github.com/Byron/gitoxide/issues/384), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#503](https://github.com/Byron/gitoxide/issues/503), [#691](https://github.com/Byron/gitoxide/issues/691), [#706](https://github.com/Byron/gitoxide/issues/706), [#720](https://github.com/Byron/gitoxide/issues/720)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Support for in truncated history in git-describe ([`99365f2`](https://github.com/Byron/gitoxide/commit/99365f221065ebc315ac80940ad72cae253743bc))
    - Fix git-revision dependencies ([`c336b03`](https://github.com/Byron/gitoxide/commit/c336b033ae8d94d859a04f0a19f82aa5c4d760e0))
    - Fix ordering of commits to actually be by commit-time, then topo-time ([`8286eac`](https://github.com/Byron/gitoxide/commit/8286eacfb791bac3449f84c9a2990aa13fba5b81))
    - Support for the --max-candidates flag ([`b9e6754`](https://github.com/Byron/gitoxide/commit/b9e67540801f2630be8aa1acbfddfec4202360ac))
    - Reduce amount of max candidates, add --debug flag ([`c8c13e3`](https://github.com/Byron/gitoxide/commit/c8c13e398671a21e96282547fc0e3bd445627e2f))
    - Use hashed-hasher for an eek of performance ([`324a839`](https://github.com/Byron/gitoxide/commit/324a839e6c72174f08779a97fa12cc313e2afac2))
    - Early-abort if all work is done during traversal ([`5b2aa70`](https://github.com/Byron/gitoxide/commit/5b2aa7015f4adc7cedd8f5b2715d611c2df02d98))
    - Make `describe::Format` more consistent with other builder APIs ([`0a7776b`](https://github.com/Byron/gitoxide/commit/0a7776b8cce4c40c391f46542f6e7ba6830d6fc0))
    - All documentation for the git-revision crate ([`8e0fb0a`](https://github.com/Byron/gitoxide/commit/8e0fb0a49630a1e3a67f174df4a22fdf224171c3))
    - Support for 'first-parent' traversal ([`52eae32`](https://github.com/Byron/gitoxide/commit/52eae32a5393113595cc8970528c8e78d6ce0525))
    - Support for fallbacks if no candidate available ([`39708a7`](https://github.com/Byron/gitoxide/commit/39708a7a53e8bd82a769a90049b1e706e021b7e1))
    - Describe-format with support for 'always' display style ([`79f386d`](https://github.com/Byron/gitoxide/commit/79f386d6bcd65b30b319c6113dd3070c940cfebd))
    - Finish depth computation works! ([`2e80e36`](https://github.com/Byron/gitoxide/commit/2e80e365000f924be84c9c60820758f4a0661c8d))
    - Prepare for finish-computation impl ([`9e10c7a`](https://github.com/Byron/gitoxide/commit/9e10c7a5d1873d618cc268e59681f230c6338df8))
    - Prepare test for 'gave_up_on' to motivate implementing finish_computation() ([`966ec3f`](https://github.com/Byron/gitoxide/commit/966ec3fc2246f44a67d2b24d98d14e491767f162))
    - Use thiserror instead of quickerror ([`7dcd2a5`](https://github.com/Byron/gitoxide/commit/7dcd2a5a65d1ac7d4370198951a495f2e00fccfe))
    - Use quickerror to handle all error branches ([`1243417`](https://github.com/Byron/gitoxide/commit/12434170130c716dbd9daceb3f0510fe63d342ce))
    - Some TODOs to not forget where to continue ([`84c0f15`](https://github.com/Byron/gitoxide/commit/84c0f1576cd295b014fc1bf6907e4b0674444b33))
    - Git-describe complete formatting ([`eefa6c5`](https://github.com/Byron/gitoxide/commit/eefa6c51da2bafb6a6bcfb1a2fdb785b73cf919c))
    - Frame for testing describe(), first sketch of signature with return value ([`5841f47`](https://github.com/Byron/gitoxide/commit/5841f473c01ebc667922f654885a14dc289d9844))
    - First failing test for describe() ([`23b1973`](https://github.com/Byron/gitoxide/commit/23b1973997cd68e94396c9f0ea21d7ae2138877a))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
    - Sort parents by most recent to find recent tags first ([`d240740`](https://github.com/Byron/gitoxide/commit/d240740cd24bdd8ded1d9048e2861b88476dbbe1))
    - Refactor; first green tests ([`92a37ed`](https://github.com/Byron/gitoxide/commit/92a37edbc419a4b95cac62aae2627bed9ec2eaad))
    - No need for ordering by date, keep it simple ([`02909ea`](https://github.com/Byron/gitoxide/commit/02909ea7f39bd3fe0fdd361478fc665664d09377))
    - A step closer to the first successful test ([`710d46b`](https://github.com/Byron/gitoxide/commit/710d46beefc00f59f2d841170ddf46a410af7e85))
    - A step towards traversing the graph ([`48cba41`](https://github.com/Byron/gitoxide/commit/48cba41eb623be4e7d4a67d8f5a24940b5d82324))
    - Refactor ([`e22e2dd`](https://github.com/Byron/gitoxide/commit/e22e2dd5b25913cdb15b09e97897e652e50a67d9))
    - The trivial part of the actual implementation ([`92a67a6`](https://github.com/Byron/gitoxide/commit/92a67a6eb58f1e31181fc10c9fcf34b56313058f))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - More speedy access to author/committer ([`6129607`](https://github.com/Byron/gitoxide/commit/61296077cebaaf2eb939fa6082121304bc6cf39b))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - Add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - Auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - Remove unused type ([`ad3475d`](https://github.com/Byron/gitoxide/commit/ad3475d473109649eb904786db7847a4e61d0e89))
    - Better docs for `Spec` and `spec::Kind` ([`6b76c06`](https://github.com/Byron/gitoxide/commit/6b76c06c1e9e2317f6ee1ff26c3cc57c46ec0b69))
    - More intuitive variants for `Spec`. ([`42aea42`](https://github.com/Byron/gitoxide/commit/42aea42c1f6c2a9681688825a9e31966bca1896c))
    - `Spec` with `Display` implementation to reproduce itself. ([`36c70e1`](https://github.com/Byron/gitoxide/commit/36c70e1f4ce07bf69d7064de1b6f0516d13d8acf))
    - More fuzz success ([`f239796`](https://github.com/Byron/gitoxide/commit/f239796aaffce59eb30527dc3635356ca0bab699))
    - Fix panics discovered by fuzzer input ([`0f9e959`](https://github.com/Byron/gitoxide/commit/0f9e959a98d7a15ad2b0eeeea8e21bde89ed6a42))
    - Add fuzz target ([`54108f4`](https://github.com/Byron/gitoxide/commit/54108f4e00155e96a450daace6721f174743026c))
    - Add support for `r1^@` ([`4bb2003`](https://github.com/Byron/gitoxide/commit/4bb200300b1665cab49b780ae13c277630b70f51))
    - Add support for `r1^!` ([`7e5d31c`](https://github.com/Byron/gitoxide/commit/7e5d31cb253f994ef19b15978c5df0f3a7ccebb1))
    - Refactor ([`dd1a208`](https://github.com/Byron/gitoxide/commit/dd1a20824c43ab55cd8ab260a2fa381b276146f1))
    - The first test for @^! syntax ([`b97677c`](https://github.com/Byron/gitoxide/commit/b97677cecb5efa01445769ba10835ba4d8d263e5))
    - Rename various `Kind` variants to be more descrptive. ([`487941c`](https://github.com/Byron/gitoxide/commit/487941ce557182c7ad02958e011959acb2dd5607))
    - Add all remainiing rev-spec kinds. ([`fcc737d`](https://github.com/Byron/gitoxide/commit/fcc737dbca587747bb9ba1d4b3376b5e455177c9))
    - Assure parsing ends after special syntax sugar ([`661bf29`](https://github.com/Byron/gitoxide/commit/661bf2992baf184224c16ca80172a132bee9129a))
    - Support for `<rev>^-<n>` and `<rev>^-` ([`fa1615d`](https://github.com/Byron/gitoxide/commit/fa1615da63594acbe92c3c4a13e2aeb7c1ee1d94))
    - A way to intercept which ref or prefix was set ([`b7a823b`](https://github.com/Byron/gitoxide/commit/b7a823b246b6c10c5a191bde22a88678909ff4fd))
    - First steps toward implementing ^-n ([`4b105f8`](https://github.com/Byron/gitoxide/commit/4b105f88a1429108653238e7407fd3829af939c5))
    - Tests for `r1^-`  and `r1^-n` syntactic sugar ([`5d983c6`](https://github.com/Byron/gitoxide/commit/5d983c631172a87fba646d62cc102a80ab7da17f))
    - Adjust RevSpec::range() to match changes in `git-revision` ([`05ea453`](https://github.com/Byron/gitoxide/commit/05ea45337e85583db5e57f14e995be49ba888ee1))
    - Omitted revisions after or before ../... are automatically defaulted to `HEAD`. ([`d6f481d`](https://github.com/Byron/gitoxide/commit/d6f481d0eee39d5b7e8ad7885f52b07ea876388e))
    - Make it possible to see the ordering of calls ([`b04614c`](https://github.com/Byron/gitoxide/commit/b04614c8bdf85c4f8025daeba6d6b0794699104b))
    - Rename `Kind::Single` to `Include` and add `Exclude` kind. ([`baf34c4`](https://github.com/Byron/gitoxide/commit/baf34c486f54e4699f88b06a0f8cbb10f0582bd0))
    - Provide better hints for parsing describe output ([`fb0b8ca`](https://github.com/Byron/gitoxide/commit/fb0b8ca6dfde391c28c83494e7280b2ea7e933da))
    - Improve describe hinting to allow hinting with describe-anchors as well ([`d993992`](https://github.com/Byron/gitoxide/commit/d99399287966ba2adf143222c3bd9ccdb4d135f9))
    - Support disambiguation of describe prefixes ([`637dcb0`](https://github.com/Byron/gitoxide/commit/637dcb09771c8df83436dc48d6a72804b400c5e1))
    - First implementation of object peeling ([`b1ef03a`](https://github.com/Byron/gitoxide/commit/b1ef03abc8342adb4a0b67d7c86136720ee600e2))
    - Explicitly support leading `..` and `...` ([`723e803`](https://github.com/Byron/gitoxide/commit/723e8034eba511e5d98d559949ef6552a7ac7d27))
    - Support for explaining all navitation ([`ace9c89`](https://github.com/Byron/gitoxide/commit/ace9c8953bebc4a808c639e365010ed53c031622))
    - Handle lonely tilde gracefully ([`6fb834e`](https://github.com/Byron/gitoxide/commit/6fb834e06639febbe67a46e702cd523c4e7bd2a7))
    - Refactor ([`1a15e12`](https://github.com/Byron/gitoxide/commit/1a15e120a75d29b3d3f7615af1a66a033dfd3c8b))
    - Docs ([`42969f8`](https://github.com/Byron/gitoxide/commit/42969f8a53e3210af179911d655646915046bcb8))
    - Top-level regex handling ([`f9d6f9e`](https://github.com/Byron/gitoxide/commit/f9d6f9e84b852141aed8366044692af3a8344242))
    - Support for index lookups by paths and stage ([`ea22d3e`](https://github.com/Byron/gitoxide/commit/ea22d3e7c134b9517079f865e9f6848aa27f1a8b))
    - All tests relevant for top-level colon parsing ([`cee04e1`](https://github.com/Byron/gitoxide/commit/cee04e1268ad3d3fcc3f0c45efb1415a30fb9e80))
    - Implement :<path> parsing ([`74e7a46`](https://github.com/Byron/gitoxide/commit/74e7a46199d3ae13d8bc3616d285c238942c2cad))
    - Tests for path parsing ([`d51e438`](https://github.com/Byron/gitoxide/commit/d51e438041a243a9827fe638e1e6330835706446))
    - More thorough tests using more complex specs ([`beb6e25`](https://github.com/Byron/gitoxide/commit/beb6e25a3a77df3532154d62911148302e639e37))
    - Implement tilde handling ([`e8a16c9`](https://github.com/Byron/gitoxide/commit/e8a16c964ddc994d32e8a122278f40700ad90cbc))
    - Greatly improve brace handling ([`546f4df`](https://github.com/Byron/gitoxide/commit/546f4df8d8adcfc86c435a7d408307e5de8762e4))
    - More testing of escaping ([`f3eaff6`](https://github.com/Byron/gitoxide/commit/f3eaff631a88994a69437e67682680e14505f3a8))
    - Prepare for being able to escape backslashes properly ([`840d9d0`](https://github.com/Byron/gitoxide/commit/840d9d0702f835f6b92d04122c8e9a9b4f21c9d1))
    - More specific backslash testing ([`a958edd`](https://github.com/Byron/gitoxide/commit/a958eddc2920cc0512ef1f987c31957fbefa1161))
    - More regex error handling ([`edd36ba`](https://github.com/Byron/gitoxide/commit/edd36baad610d32aeb17ab34448f1b4a5b253732))
    - Handle braces within braces and support escaping them ([`8c5d87b`](https://github.com/Byron/gitoxide/commit/8c5d87bdf886727b8d0f013fc2ee497140032644))
    - Basic regex parsing ([`1caeae9`](https://github.com/Byron/gitoxide/commit/1caeae95004ed4ef19a9c587744fe2b6d972c61a))
    - Fix regex API and first ignored test ([`7a3a5fa`](https://github.com/Byron/gitoxide/commit/7a3a5fa740751f024b88a92deb3ffe624842509b))
    - A sketch of the regex parsing API for the delegate ([`18d9331`](https://github.com/Byron/gitoxide/commit/18d9331745bdebb077730f79132c76a12e9e7e24))
    - Provide a marker for the delegate to know parsing is done ([`159a482`](https://github.com/Byron/gitoxide/commit/159a48268ee1e5d53adafbf36aa6e5fdf2886323))
    - Refactor ([`6638040`](https://github.com/Byron/gitoxide/commit/66380409611a06c56800454813eb018d4938ef32))
    - Parseing of 'follow tags recursively' ([`f11916a`](https://github.com/Byron/gitoxide/commit/f11916a78c3747ef6e52b9cd48b3235608a2c598))
    - Parsing of `^{commit}` etc. ([`4d2dd56`](https://github.com/Byron/gitoxide/commit/4d2dd569c1296a2f906da6c30c591a966fcc5716))
    - Refactor ([`a52244b`](https://github.com/Byron/gitoxide/commit/a52244b75bdaf10716fc788c8ef30615318d4606))
    - Proper stacking/consumption of navigation items ([`76f7c4d`](https://github.com/Byron/gitoxide/commit/76f7c4de4b781f59cfd95b04ff8342cab0fe2dd5))
    - Refactor ([`6f00e33`](https://github.com/Byron/gitoxide/commit/6f00e33781e5db7ff7d2c4290fb7f57d1db147b1))
    - Navigation doesn't stack yet ([`d83937b`](https://github.com/Byron/gitoxide/commit/d83937b16640c9021a16abab6a1c89dbbca10c5c))
    - Handle special case `@^0` ([`fa7790b`](https://github.com/Byron/gitoxide/commit/fa7790bc5a2385351e0c61fa6ea8878317ce1fcc))
    - Basic caret parsing ([`c064135`](https://github.com/Byron/gitoxide/commit/c0641354e43a33a851339fd9871d8eec1abb93d8))
    - Refactor ([`9b0e2a4`](https://github.com/Byron/gitoxide/commit/9b0e2a4c9201d7c1dd65377fbc982e44b1c33886))
    - Reflog lookup by date is complete ([`b3d009e`](https://github.com/Byron/gitoxide/commit/b3d009e80e3e81afd3d095fa2d7b5fc737d585c7))
    - Prepare for date based reflog lookups. ([`2267b2b`](https://github.com/Byron/gitoxide/commit/2267b2b7c31f6ee9995126a0d4783699166a6a3c))
    - Sibling branch support ([`0d3fb7a`](https://github.com/Byron/gitoxide/commit/0d3fb7a880ffbb6156bfb1d0b34f9679a6c6957f))
    - Refname reflog entries ([`b50d099`](https://github.com/Byron/gitoxide/commit/b50d09903932961c62fa57464aef842766bbbbcb))
    - Allow parsing `@{-n}` ([`faa9914`](https://github.com/Byron/gitoxide/commit/faa9914731d5202e8f162eb6c09cdf8680de6d18))
    - Refactor ([`a5f8f58`](https://github.com/Byron/gitoxide/commit/a5f8f5806edb0be7fe97ad65dde8c37d0a9c198f))
    - Basic number parsing for '@' navigation ([`3fedcc0`](https://github.com/Byron/gitoxide/commit/3fedcc0afad1fe4c5cf6ef487904b0b60dc19540))
    - Refactor ([`bff11a0`](https://github.com/Byron/gitoxide/commit/bff11a066f73b43045064cd9d6ca0ac09468e8f3))
    - More information on how anchors work ([`d82b21f`](https://github.com/Byron/gitoxide/commit/d82b21f2cd4f863a9d3d39d90f233fa171f52067))
    - Show that we can already parse ranged rev-specs better than git ([`418360c`](https://github.com/Byron/gitoxide/commit/418360c23b9fcf6e57fdaa2e1ea732dc6256dbbf))
    - Basic brace parsing ([`43e4cc1`](https://github.com/Byron/gitoxide/commit/43e4cc15c7115dd40238051274f50fe10907c24e))
    - Refactor ([`ad4d8af`](https://github.com/Byron/gitoxide/commit/ad4d8afb3036b4f626f09fb26ac78a426d7acc2d))
    - Prevent double-kind calls on parser level ([`d6781da`](https://github.com/Byron/gitoxide/commit/d6781da221602c272a26ac4f45a54f77ddd340bd))
    - Refactor ([`c3b03a2`](https://github.com/Byron/gitoxide/commit/c3b03a237f30091558ddd0325279953fced16131))
    - Refactor ([`b2c80ee`](https://github.com/Byron/gitoxide/commit/b2c80ee4c78a45ac2d95b69d8cbdccf349b95f3c))
    - Also handle short decribe output with dirty suffix ([`826f964`](https://github.com/Byron/gitoxide/commit/826f96416d3eb59f93380b4c12c92844d9fd690e))
    - Finalize git-describe parsing ([`e1e369f`](https://github.com/Byron/gitoxide/commit/e1e369f0c1a36805d50826d6b48d2dc62195f8bd))
    - Tests for parsing describe output ([`5be4ad8`](https://github.com/Byron/gitoxide/commit/5be4ad8ac40f984e88acc64fbf77f221b0902a6a))
    - Refactor ([`4f53dc3`](https://github.com/Byron/gitoxide/commit/4f53dc304abf89e8b6cafaafbcec99264ea67a95))
    - More varied range testing ([`bb0a554`](https://github.com/Byron/gitoxide/commit/bb0a554efd1b68298a23bcd2e29dc60da7a127c5))
    - Refactor ([`2e49831`](https://github.com/Byron/gitoxide/commit/2e498317e6637ac57de21fee8bf905daf1cc54bf))
    - Support for hex-lookups by prefix ([`16945ed`](https://github.com/Byron/gitoxide/commit/16945edd1e544caf34ffa318bc59eea635e8b060))
    - Refactor ([`db97a2e`](https://github.com/Byron/gitoxide/commit/db97a2ed20ab13786b30e7ad17a1b24eaeb34648))
    - Half-decent parsing of ref-names with preparation for parenthesis handling ([`9866986`](https://github.com/Byron/gitoxide/commit/9866986de74f2aaa6471cfb2ec8ea7e4572b3a09))
    - Tiny steps towards understanding rev-parsing better ([`13c07f4`](https://github.com/Byron/gitoxide/commit/13c07f4ef84c5e03e08d04259eeede5e4d487476))
    - Decide to not implement regex support (yet) ([`d6a4838`](https://github.com/Byron/gitoxide/commit/d6a4838dbb91d43f84e319986c027e9cabf536b2))
    - Allow delegates to refuse spec kind changes ([`2d9465f`](https://github.com/Byron/gitoxide/commit/2d9465fe01021bdcc8ba0907a5847e970c3cea12))
    - Refactor ([`d16a4e8`](https://github.com/Byron/gitoxide/commit/d16a4e8f75bac5df6a4e96a2bd93d256587457b3))
    - Refactor ([`e059bd3`](https://github.com/Byron/gitoxide/commit/e059bd33647a2b35af241a1f88cb61dc5176b55d))
    - Support for range parsing with range in the middle ([`5ada481`](https://github.com/Byron/gitoxide/commit/5ada481c3756e1717189b478fc458322c3acc7ac))
    - Basic range parsing ([`0c1c48c`](https://github.com/Byron/gitoxide/commit/0c1c48c5b393eeb534d50bf4048fe9c049297f00))
    - Parse initial carets ([`8573c8e`](https://github.com/Byron/gitoxide/commit/8573c8e3d6f11f015f7e586632a637269e70395b))
    - Some more thought about whitespace and empty input ([`7182d88`](https://github.com/Byron/gitoxide/commit/7182d88e245f3bb8740cab1058acb7c9a1d6d461))
    - Refactor ([`91e2c43`](https://github.com/Byron/gitoxide/commit/91e2c43c20c0d6ff4fae9669bfca4fcfe03c37a0))
    - Prepare range parsing ([`5bd4863`](https://github.com/Byron/gitoxide/commit/5bd4863ced766d71432e252c344a424a2fd1a4fd))
    - Refactor ([`efc05e1`](https://github.com/Byron/gitoxide/commit/efc05e11fa2ec11952b06080ba76387a4c11c3b4))
    - A basis for 'pure' parsing of rev-specs ([`29ab704`](https://github.com/Byron/gitoxide/commit/29ab7049fd180fac2e443a99908db066c67938db))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - Adjust to deal with changes to git-repository ([`b99b6bf`](https://github.com/Byron/gitoxide/commit/b99b6bfea47a4485496c2eb565693a6a53efe166))
    - Add fuzz target and basic docs on how to run it ([`febf070`](https://github.com/Byron/gitoxide/commit/febf0706b83b36a71efbe669ee760c2d4ef14b72))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#503](https://github.com/Byron/gitoxide/issues/503)**
    - Prepare changelog ([`3c99e7f`](https://github.com/Byron/gitoxide/commit/3c99e7f02ada72a171856ffc5b870da83fffc703))
    - Decscribe() won't abort before the first name check if max-candidates is 0 ([`4788270`](https://github.com/Byron/gitoxide/commit/4788270853d42be8405465a6b9b612783ae9ef6e))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#706](https://github.com/Byron/gitoxide/issues/706)**
    - Improve test coverage related to freestanding 'kind' markers ([`6a70f0b`](https://github.com/Byron/gitoxide/commit/6a70f0b8dbc29d7ae7f156ee0be558d67e2d155c))
 * **[#720](https://github.com/Byron/gitoxide/issues/720)**
    - Prevent panics from dates which cannot be represented by the `time` crate ([`786f6dc`](https://github.com/Byron/gitoxide/commit/786f6dc5c1f765b9598cd55ca8fb1714ad177e46))
 * **Uncategorized**
    - Release gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`7fc00f8`](https://github.com/Byron/gitoxide/commit/7fc00f87d74aedf631ce4032be1cdfe1804c7e7d))
    - Release gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`59e9fac`](https://github.com/Byron/gitoxide/commit/59e9fac67d1b353e124300435b55f6b5468d7deb))
    - Release gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`48f5bd2`](https://github.com/Byron/gitoxide/commit/48f5bd2014fa3dda6fbd60d091065c5537f69453))
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
    - Rename `git-revision` to `gix-revision` ([`3308a45`](https://github.com/Byron/gitoxide/commit/3308a45f4f9c6d48f646718e7ba0cebb49d9b17b))
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
    - Release git-config v0.16.1, git-revision v0.10.3, gix v0.35.0 ([`74390ba`](https://github.com/Byron/gitoxide/commit/74390baf9d177a1abe3c7c35f1d9bc67faba1e97))
    - Prepare changelogs prior to release ([`446f866`](https://github.com/Byron/gitoxide/commit/446f866d146e255ab8302b89f87bf28f2c5f3733))
    - Merge branch 'rename-crates' ([`6461c3d`](https://github.com/Byron/gitoxide/commit/6461c3da4d6daee857606d94294c3f87fc36965a))
    - Rename `git-repository` to `gix` ([`7bed2a9`](https://github.com/Byron/gitoxide/commit/7bed2a96604397fa990f427b1a970ddeb6f09f1c))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Make fmt ([`e22080e`](https://github.com/Byron/gitoxide/commit/e22080e4a29d0bad15a99d565a5e3e304a8743ec))
    - Optimize usage of `hex_to_id()` ([`6fa950d`](https://github.com/Byron/gitoxide/commit/6fa950d0ab1991a5577c06385169be1b390dd88a))
    - Merge branch 'main' into break_cycel2 ([`e67307a`](https://github.com/Byron/gitoxide/commit/e67307aa9b1b81957abe0d5bae4c0e1008b1c1d7))
    - Merge branch 'fix-706' ([`ab0bc98`](https://github.com/Byron/gitoxide/commit/ab0bc987e3647de56db9f7b4fc7bda6e76fc5f75))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Release git-ref v0.23.0, git-config v0.15.0, git-command v0.2.2, git-diff v0.26.0, git-discover v0.12.0, git-mailmap v0.9.0, git-pack v0.30.0, git-odb v0.40.0, git-transport v0.25.2, git-protocol v0.26.1, git-revision v0.10.0, git-refspec v0.7.0, git-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - Prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
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
    - Hash_hasher re-export in favor of using `git-hashtable`. ([`2761466`](https://github.com/Byron/gitoxide/commit/2761466ef6734ad6484548d7a93e52db3a230864))
    - Use newly added git-hashtable ([`50cb436`](https://github.com/Byron/gitoxide/commit/50cb4362010e1a5799fe782df36ac5fcdb48dd8a))
    - Switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'git_date_parse' ([`75591fb`](https://github.com/Byron/gitoxide/commit/75591fb108ce440ba2f920bebf99158b407e3046))
    - Refactor ([`e1a1406`](https://github.com/Byron/gitoxide/commit/e1a1406183ae4feadad7a91925144e62cd1592c3))
    - Refactor  - don't degenerate error ([`976b31f`](https://github.com/Byron/gitoxide/commit/976b31f81c830facf6386ad8ae43867c57af77e2))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - `parse` is pure function. ([`9ad1a5f`](https://github.com/Byron/gitoxide/commit/9ad1a5fa2ce54e978396ff3eaa7061a8edd10d4a))
    - `parse()` returns Result. ([`206f392`](https://github.com/Byron/gitoxide/commit/206f3923f5da2e9e26677e917550e6e5baa2913a))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Merge branch 'fix-ci-installation' ([`9245083`](https://github.com/Byron/gitoxide/commit/92450839621a4d99cb22d08cbf9f9a89ff6b9e3f))
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - Update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - Adjust to new version of git-date ([`b3fe26b`](https://github.com/Byron/gitoxide/commit/b3fe26bf03db7e1babb5ffbc89d71bf9614e3df3))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
    - `describe()` aborts search early if there is no input name in the name map. ([`df62f50`](https://github.com/Byron/gitoxide/commit/df62f5081291f65f994b2aa66f0599f47eea8d4d))
    - Release git-date v0.0.4, git-actor v0.11.2, git-revision v0.4.1, git-repository v0.21.1 ([`2f9dc84`](https://github.com/Byron/gitoxide/commit/2f9dc847e0d54f4181ce35ddadd9286ba80ca01f))
    - Prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Revision describe can now short-cut what effectively is only a name-to-id lookup ([`ca66512`](https://github.com/Byron/gitoxide/commit/ca6651234a8c0d4718554323b197a49266b60a61))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Adjust `git_date::parsea(str)` to use a str ([`0f8680a`](https://github.com/Byron/gitoxide/commit/0f8680a60913556b7fbd7543fda6a409ac05b121))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - Thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - Raise `git-revision` to the status of 'usable' ([`09eb1a6`](https://github.com/Byron/gitoxide/commit/09eb1a6e1eb5888b66b211500c73d72951058685))
    - Merge branch 'parse-refspec' ([`2ba338e`](https://github.com/Byron/gitoxide/commit/2ba338e28eb45d4d3215dd6ff9882611880d4cd9))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Add `Spec` data strcuture to fully represent a revision specification ([`5038ffa`](https://github.com/Byron/gitoxide/commit/5038ffab6a0f83e0566f99e3c92ae2dea266e10b))
    - Thanks clippy ([`ca82265`](https://github.com/Byron/gitoxide/commit/ca82265abfcce644265af64afc499d2de88c3cba))
    - Thanks clippy ([`19db44a`](https://github.com/Byron/gitoxide/commit/19db44a97d42f4fa77c681263cf509ee91f8fa6c))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - Assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'revspec-parsing' ([`a2c8969`](https://github.com/Byron/gitoxide/commit/a2c8969ba821fd387c39b14248074767f54749c8))
    - Thanks clippy ([`1bbd3f4`](https://github.com/Byron/gitoxide/commit/1bbd3f471d78e53a76b3e708c755fc9d72fc28fe))
    - Thanks clippy ([`b93fa40`](https://github.com/Byron/gitoxide/commit/b93fa40a9abcfb7390276e4254f696c0cac2abb1))
    - Thanks clippy ([`6dc9c44`](https://github.com/Byron/gitoxide/commit/6dc9c44fb2770d93badb8e1d506b7601107ea586))
    - Thanks clippy ([`ec0ff74`](https://github.com/Byron/gitoxide/commit/ec0ff7404ba7df80bf98fd6d28b13426c2e3ee6c))
    - Thanks clippy ([`1b40259`](https://github.com/Byron/gitoxide/commit/1b402596bb581ea84b285282a44bf81752c14bba))
    - Thanks clippy ([`6d08d5f`](https://github.com/Byron/gitoxide/commit/6d08d5f518a94426420c973b8e6e561ef558627c))
    - Thanks clippy ([`1f0545f`](https://github.com/Byron/gitoxide/commit/1f0545f3169824f4953727f7319324b60baaf92f))
    - Thanks clippy ([`2bc1acc`](https://github.com/Byron/gitoxide/commit/2bc1acc1816ef95b60c0192ef8d956558ff58bb9))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - Release git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0 ([`f041c00`](https://github.com/Byron/gitoxide/commit/f041c00a7df2455ca52fac7b83af1e9f335f5688))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - Thanks clippy ([`4d4fda6`](https://github.com/Byron/gitoxide/commit/4d4fda68c67eb02ce2055707bc62a577ad3d7b78))
    - Thanks clippy ([`f2faa00`](https://github.com/Byron/gitoxide/commit/f2faa001ed2c8e96e25dbd56544320055f8dbe1b))
    - Thanks clippy ([`9f18dca`](https://github.com/Byron/gitoxide/commit/9f18dca5dfde3f24ce2e81d60beb343aa85d9cd6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Remove serde support for describe types due to warning ([`2ba33c8`](https://github.com/Byron/gitoxide/commit/2ba33c89e723c7ec44ff8b5597718ee7792f462d))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - Thanks clippy ([`2c8a504`](https://github.com/Byron/gitoxide/commit/2c8a504c2b1a8309c3176e8c829e129c8dd39f80))
    - INTERMEDIATE RESET ME ([`a4de008`](https://github.com/Byron/gitoxide/commit/a4de008b88f892e95bf6da36d09b27190e9c5ede))
    - Thanks clippy ([`f1ef59d`](https://github.com/Byron/gitoxide/commit/f1ef59d8129231554158fc51ab967b4f857c5e12))
    - Release git-revision v0.0.0 ([`8e434d8`](https://github.com/Byron/gitoxide/commit/8e434d8d0046e4479f0a575247ce3c9cce7e1f77))
    - Rename git-rev to git-revision ([`2e939c9`](https://github.com/Byron/gitoxide/commit/2e939c973ab3635a946317af08f37c4e23450f18))
</details>

## 0.10.2 (2023-02-09)

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-786f6dc5c1f765b9598cd55ca8fb1714ad177e46/> prevent panics from dates which cannot be represented by the `time` crate

## 0.10.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.10.0 (2023-01-09)

A maintenance release without user-facing changes.

## 0.9.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.8.0 (2022-12-19)

### Reverted (BREAKING)

 - <csr-id-2761466ef6734ad6484548d7a93e52db3a230864/> hash_hasher re-export in favor of using `gix-hashtable`.
   Due to the importance of best-suited data structures for maximizing
   performance we need to take control over them. This is best done using
   a dedicated crate that can cater to our very needs. That very crate is
   named `gix-hashtable`.

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

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.4 (2022-08-27)

### Bug Fixes

 - <csr-id-4788270853d42be8405465a6b9b612783ae9ef6e/> decscribe() won't abort before the first name check if max-candidates is 0
   A test was missing too, which is now fixed.

## 0.4.3 (2022-08-24)

A maintenance release without user facing changes.

## 0.4.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-df62f5081291f65f994b2aa66f0599f47eea8d4d/> `describe()` aborts search early if there is no input name in the name map.

## 0.4.1 (2022-08-19)

### New Features

 - <csr-id-ca6651234a8c0d4718554323b197a49266b60a61/> revision describe can now short-cut what effectively is only a name-to-id lookup
   This makes situations easier where `max-candidates` is provided by the user or by
   configuration.

## 0.4.0 (2022-08-17)

### New Features

 - <csr-id-36c70e1f4ce07bf69d7064de1b6f0516d13d8acf/> `Spec` with `Display` implementation to reproduce itself.
   That way it can be parsed back perfectly after displaying itself, and
   will work normally when used in backticks in the shell for simple
   include patterns.
 - <csr-id-5038ffab6a0f83e0566f99e3c92ae2dea266e10b/> Add `Spec` data strcuture to fully represent a revision specification
 - <csr-id-4bb200300b1665cab49b780ae13c277630b70f51/> Add support for `r1^@`
 - <csr-id-7e5d31cb253f994ef19b15978c5df0f3a7ccebb1/> Add support for `r1^!`
 - <csr-id-fa1615da63594acbe92c3c4a13e2aeb7c1ee1d94/> support for `<rev>^-<n>` and `<rev>^-`

### Changed (BREAKING)

 - <csr-id-42aea42c1f6c2a9681688825a9e31966bca1896c/> More intuitive variants for `Spec`.
 - <csr-id-487941ce557182c7ad02958e011959acb2dd5607/> rename various `Kind` variants to be more descrptive.
 - <csr-id-baf34c486f54e4699f88b06a0f8cbb10f0582bd0/> Rename `Kind::Single` to `Include` and add `Exclude` kind.
   So far I got ranges pretty wrong and was degenerating the `^rev` case
   due to misinterpretation of the docs.
   This summary corrected that: https://git-scm.com/docs/git-rev-parse#_revision_range_summary

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.2.1 (2022-06-13)

### New Features

- support for parsing `revspec`s on a low level, meaning that the ground work for actually resolving them is done.

## 0.2.0 (2022-05-18)

### Bug Fixes

 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

## 0.1.0 (2022-04-05)

<csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/>

### Refactor (BREAKING)

 - <csr-id-0a7776b8cce4c40c391f46542f6e7ba6830d6fc0/> Make `describe::Format` more consistent with other builder APIs
   Configuration methods now take an argument which makes it more
   straightforward to use for most.

## 0.0.0 (2022-02-05)

Reserve the name for a necessary crate of the `gitoxide` project.

