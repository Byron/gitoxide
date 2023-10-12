# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.27.0 (2023-10-12)

### New Features (BREAKING)

 - <csr-id-f1597756e5f6294f4757d90484f63e8d68bc05fa/> add `Stack::from_state_and_ignore_case()`.
   It adds `Stack::from_state_and_ignore_case()` as utility to more easily instantiate
   a stack the is configured correctly.
   This also removes the `stack::State::for_status()` method as it's not actually
   suitable for status retrieval per se.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.13.1, gix-features v0.36.0, gix-actor v0.28.0, gix-object v0.38.0, gix-glob v0.14.0, gix-attributes v0.20.0, gix-command v0.2.10, gix-filter v0.6.0, gix-fs v0.8.0, gix-commitgraph v0.22.0, gix-revwalk v0.9.0, gix-traverse v0.34.0, gix-worktree-stream v0.6.0, gix-archive v0.6.0, gix-tempfile v11.0.0, gix-lock v11.0.0, gix-ref v0.38.0, gix-config v0.31.0, gix-url v0.25.0, gix-credentials v0.21.0, gix-diff v0.37.0, gix-discover v0.26.0, gix-ignore v0.9.0, gix-index v0.26.0, gix-mailmap v0.20.0, gix-negotiate v0.9.0, gix-pack v0.44.0, gix-odb v0.54.0, gix-pathspec v0.4.0, gix-packetline v0.16.7, gix-transport v0.37.0, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0, safety bump 37 crates ([`68e5432`](https://github.com/Byron/gitoxide/commit/68e54326e527a55dd5b5079921fc251615833040))
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Merge branch 'reset' ([`b842691`](https://github.com/Byron/gitoxide/commit/b8426919a491dc3a7df01ee3f258fc0d8a3a327c))
    - Add `Stack::from_state_and_ignore_case()`. ([`f159775`](https://github.com/Byron/gitoxide/commit/f1597756e5f6294f4757d90484f63e8d68bc05fa))
</details>

## 0.26.0 (2023-09-24)

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

## 0.25.0 (2023-09-08)

### New Features

 - <csr-id-477a1d9089d831cd9ed81f109fc2c89333026965/> add `attributes` feature to allow ignore-only stacks.

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0 ([`1ff3064`](https://github.com/Byron/gitoxide/commit/1ff30641b8724efd6699d8bef5c71d28454e98b9))
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch 'optimizations' ([`6135a5e`](https://github.com/Byron/gitoxide/commit/6135a5ea8709646f01da62939a59dd3a9750e007))
    - Add `attributes` feature to allow ignore-only stacks. ([`477a1d9`](https://github.com/Byron/gitoxide/commit/477a1d9089d831cd9ed81f109fc2c89333026965))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'adjustments-for-cargo' ([`b7560a2`](https://github.com/Byron/gitoxide/commit/b7560a2445b62f888bf5aa2ba4c5a47ae037cb23))
    - Release gix-index v0.23.1 ([`11b9c71`](https://github.com/Byron/gitoxide/commit/11b9c71311df978ebb20cca0d765cf249c8eedcf))
    - Release gix-date v0.7.4, gix-index v0.23.0, safety bump 5 crates ([`3be2b1c`](https://github.com/Byron/gitoxide/commit/3be2b1ccfe30eeae45711c64b88efc522a2b51b7))
    - Merge branch 'fixes' ([`4bfd1cc`](https://github.com/Byron/gitoxide/commit/4bfd1cc8f7922a8c4de6b9d078d54b93e78f51ff))
    - Adapt to changes in `gix-index` and pass skip-hash through for performance.. ([`713cd59`](https://github.com/Byron/gitoxide/commit/713cd59f0b1eff6397b80f1e1fceec278532fd59))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.24.0 (2023-08-22)

<csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/>
<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>
<csr-id-1958dffc164e7b60ddc2eb308ed6da74a80559df/>

### Chore

 - <csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/> split tests off into their own crate to allow feature toggles.
   That way we can test with the `parallel` feature and won't have to
   create bogus feature toggles that are only used for testing, yet visbible
   to users.
 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### New Features

 - <csr-id-a7c088015282533f6afbca28ee64cceeca75be35/> add `cache::state::attributes|ignore::Source::adjust_for_bare()` to use non-worktree versions of source easily.
 - <csr-id-9a4f81952c5361a0001ad4b7f7647b2539edeab1/> `Cache::set_case()` to allow changing the case for each match.
   This makes it possible to adjust case-sensitivity in case of pathspecs for instance,
   which require this to be adjustable.

### Changed (BREAKING)

 - <csr-id-5d5f2866e512a4ddad7cb4606913026d2fe62840/> rename `Cache` to `Stack` because it's more fitting.

### Chore (BREAKING)

 - <csr-id-1958dffc164e7b60ddc2eb308ed6da74a80559df/> remove `checkout` and `status` functionality so only `Cache` remains.
   `gix-worktree` is now a base-crate for use by derived crates that provide additional
   functionality. Shared types or utilities go here.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 18 calendar days.
 - 19 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0 ([`6c62e74`](https://github.com/Byron/gitoxide/commit/6c62e748240ac0980fc23fdf30f8477dea8b9bc3))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Merge branch 'gix-submodule' ([`8f3f358`](https://github.com/Byron/gitoxide/commit/8f3f358800f1fe77d7ba7ebd396a90b692d3c0c1))
    - More cleanup of test crates ([`73c685a`](https://github.com/Byron/gitoxide/commit/73c685a67debcfa26a940f37bbca69cb3a4af57e))
    - Split tests off into their own crate to allow feature toggles. ([`93feea2`](https://github.com/Byron/gitoxide/commit/93feea269eebd114e866e6f29f4a73c0096df9e0))
    - Merge branch 'submodule-in-gix' ([`36f7b78`](https://github.com/Byron/gitoxide/commit/36f7b783c67b8a087076a130f5ee9b90b23bc3cc))
    - Add `cache::state::attributes|ignore::Source::adjust_for_bare()` to use non-worktree versions of source easily. ([`a7c0880`](https://github.com/Byron/gitoxide/commit/a7c088015282533f6afbca28ee64cceeca75be35))
    - Merge branch 'worktree-organization' ([`8d0d8e0`](https://github.com/Byron/gitoxide/commit/8d0d8e005d7f11924a6717954d892aae5cec45e7))
    - Adapt to changes in `gix-worktree` ([`e5717e1`](https://github.com/Byron/gitoxide/commit/e5717e1d12c49285d31a90b03b7f8e9cbc6c1108))
    - Rename `Cache` to `Stack` because it's more fitting. ([`5d5f286`](https://github.com/Byron/gitoxide/commit/5d5f2866e512a4ddad7cb4606913026d2fe62840))
    - Remove `checkout` and `status` functionality so only `Cache` remains. ([`1958dff`](https://github.com/Byron/gitoxide/commit/1958dffc164e7b60ddc2eb308ed6da74a80559df))
    - Create new `gix-status` crate to capture `git-status` like functionality ([`be9af32`](https://github.com/Byron/gitoxide/commit/be9af327c75d693658a2427ee9a711e631a8da7d))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Merge branch 'submodule-active' ([`a3afaa4`](https://github.com/Byron/gitoxide/commit/a3afaa42741616a0f1abeef9b54557e7c2b800cb))
    - `Cache::set_case()` to allow changing the case for each match. ([`9a4f819`](https://github.com/Byron/gitoxide/commit/9a4f81952c5361a0001ad4b7f7647b2539edeab1))
    - Release gix-glob v0.10.2, gix-date v0.7.2, gix-validate v0.8.0, gix-object v0.34.0, gix-ref v0.34.0, gix-config v0.27.0, gix-commitgraph v0.18.2, gix-revwalk v0.5.0, gix-revision v0.19.0, gix-refspec v0.15.0, gix-submodule v0.1.0, safety bump 18 crates ([`4604f83`](https://github.com/Byron/gitoxide/commit/4604f83ef238dc07c85aaeae097399b67f3cfd0c))
    - Merge branch 'limit-git' ([`68d9e80`](https://github.com/Byron/gitoxide/commit/68d9e809d4e746fd7beaddeabd3313d59a4cbdfd))
    - Improve tests (and maybe more) to assure parallel worktree tests are deterministic ([`4ed84a6`](https://github.com/Byron/gitoxide/commit/4ed84a62ccc335be14b928e57b4880954bdf14a3))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Fix various tests to run properly on linux ([`ef8ccd9`](https://github.com/Byron/gitoxide/commit/ef8ccd9d16143d37155d063747c69cade80f162d))
</details>

## v0.23.1 (2023-08-02)

### Bug Fixes

 - <csr-id-82ae37d70dc244cdf705d20c617d4b0e6bf3cdbf/> don't panic during checkouts when submodules or sparse directories are encountered.
   Now we trace instead.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-actor v0.24.2, gix-object v0.33.2, gix-ref v0.33.3, gix-config v0.26.2, gix-prompt v0.5.5, gix-odb v0.50.2, gix-transport v0.34.2, gix-protocol v0.37.0, gix-worktree v0.23.1, gix v0.51.0, safety bump 3 crates ([`231ac1c`](https://github.com/Byron/gitoxide/commit/231ac1c6ad5ca9a84dbeb0dee14bfbf2fef1ae1e))
    - Prepare additional changelogs ([`db63815`](https://github.com/Byron/gitoxide/commit/db6381522395a0de047118e81df5cd3cbeb862b9))
    - Prepare changelogs ([`e4d2890`](https://github.com/Byron/gitoxide/commit/e4d2890a85bf60e9cdb4016dddfab3c4dccbe75e))
    - Merge branch 'EmbarkStudios/main' ([`7611fa4`](https://github.com/Byron/gitoxide/commit/7611fa4872c3fcbe2bf7b20fcb4033df2493d256))
    - Don't panic during checkouts when submodules or sparse directories are encountered. ([`82ae37d`](https://github.com/Byron/gitoxide/commit/82ae37d70dc244cdf705d20c617d4b0e6bf3cdbf))
    - Remove todo for submodule checkout ([`a08d851`](https://github.com/Byron/gitoxide/commit/a08d851abd4965ac712db2c9e0263c9c137f9d80))
</details>

## v0.23.0 (2023-07-22)

### Bug Fixes (BREAKING)

 - <csr-id-68bd71cc0c47b0d86c7cb8fb9fe73a03cf8b52f6/> make it possible to pass information about the directory status when matching attributes.
   This is significant for archiving operations, even though it's not important when matching attributes
   otherwise.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'gix-archive' ([`1dda48b`](https://github.com/Byron/gitoxide/commit/1dda48ba2fccb93ebac00fe3460e923af43c86ce))
    - Make it possible to pass information about the directory status when matching attributes. ([`68bd71c`](https://github.com/Byron/gitoxide/commit/68bd71cc0c47b0d86c7cb8fb9fe73a03cf8b52f6))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## v0.22.0 (2023-07-19)

### New Features

 - <csr-id-9cd256e91c6c6800b5f6c673285bf08e566f068f/> add `Cache::attributes_metadata()`.
   A function to obtain the metadata-collection which allows to initialize attribute match
   initialization in code that can't use the `gix_worktree` crate dependency.

### New Features (BREAKING)

 - <csr-id-29a64c289946301d5e502ee956f3606280409faf/> make it possible to use filter pipeline with streaming for checkouts.
   This is done by providing a `gix_filter::Pipeline` to `checkout::Options` to control
   how filters are applied.
 - <csr-id-9c936755833b10989124f2cce1b675ae7e78af64/> add `cache::State::Attributes` to only load attributes.
   Even though technically, this isn't really needed, it's required if one wants
   to support things that git doesn't usually do, like providing conversion of worktree
   files without a worktree, which can be happening whith `gix-archive` for example.
   
   As part of this change, `cache::State::id_mappings_from_index()` looses its `ignore` parameter
   as it wasn't required in the first place.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 12 calendar days.
 - 19 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`4aca8c2`](https://github.com/Byron/gitoxide/commit/4aca8c2ae2ec588fb65ec4faa0c07c19d219569f))
    - Release gix-features v0.32.0, gix-actor v0.24.0, gix-glob v0.10.0, gix-attributes v0.15.0, gix-commitgraph v0.18.0, gix-config-value v0.12.4, gix-fs v0.4.0, gix-object v0.33.0, gix-ref v0.33.0, gix-config v0.26.0, gix-command v0.2.7, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-filter v0.1.0, gix-ignore v0.5.0, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.0 ([`68ae3ff`](https://github.com/Byron/gitoxide/commit/68ae3ff9d642ec56f088a6a682a073dc16f4e8ca))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Prepare changelogs prior to release ([`e4dded0`](https://github.com/Byron/gitoxide/commit/e4dded05138562f9737a7dcfb60570c55769486d))
    - Just fmt ([`a063c62`](https://github.com/Byron/gitoxide/commit/a063c62e3a30006d837b267e2ce74e70e48b4fb6))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Make it possible to use filter pipeline with streaming for checkouts. ([`29a64c2`](https://github.com/Byron/gitoxide/commit/29a64c289946301d5e502ee956f3606280409faf))
    - Assure we run all tests in nextest ([`3821089`](https://github.com/Byron/gitoxide/commit/3821089b6b02c933770705b19fc3126d61beb5a7))
    - Add `cache::State::Attributes` to only load attributes. ([`9c93675`](https://github.com/Byron/gitoxide/commit/9c936755833b10989124f2cce1b675ae7e78af64))
    - Add `Cache::attributes_metadata()`. ([`9cd256e`](https://github.com/Byron/gitoxide/commit/9cd256e91c6c6800b5f6c673285bf08e566f068f))
</details>

## v0.21.1 (2023-06-29)

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

## v0.21.0 (2023-06-29)

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

## v0.20.0 (2023-06-22)

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

## v0.19.0 (2023-06-10)

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
    - Release gix-attributes v0.13.1, gix-diff v0.30.1, gix-revwalk v0.1.0, gix-traverse v0.27.0, gix-index v0.18.0, gix-revision v0.15.2, gix-negotiate v0.2.1, gix-pack v0.37.0, gix-odb v0.47.0, gix-protocol v0.33.2, gix-worktree v0.19.0, gix v0.46.0, safety bump 7 crates ([`2560a2c`](https://github.com/Byron/gitoxide/commit/2560a2cc3e1d8c60cd812e15696fa4761d036e19))
    - Prepare changelogs prior to release ([`298f3d7`](https://github.com/Byron/gitoxide/commit/298f3d7359c5b183314d8c584e45dcdd559d88b3))
</details>

## v0.18.0 (2023-06-06)

### Bug Fixes

 - <csr-id-c86ca69b57cda379fdfe1b4a7af8fabbdfcec28d/> disallow reading macros when they are not global.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 27 calendar days.
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
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'fix-851' ([`2f275d5`](https://github.com/Byron/gitoxide/commit/2f275d5d3cb49b3b8ba53b30e4b4386fac32662b))
    - Adjust to changes in `gix-pack` ([`215889c`](https://github.com/Byron/gitoxide/commit/215889ceb976a59368c132aabfffb71a6a2ac9f8))
    - Release gix-object v0.29.2 ([`4f879bf`](https://github.com/Byron/gitoxide/commit/4f879bf35653bdc8f9729d524c6e8e1fb3c6886b))
    - Merge branch 'gix-attributes-validate' ([`a849da8`](https://github.com/Byron/gitoxide/commit/a849da8e35ca14fef9a2431fe1bb1c05b249680e))
    - Adjust to changes in `gix-ignore` ([`977447f`](https://github.com/Byron/gitoxide/commit/977447f7255ffa6a36ab1fdaaa8532b60cd6c3a5))
    - Disallow reading macros when they are not global. ([`c86ca69`](https://github.com/Byron/gitoxide/commit/c86ca69b57cda379fdfe1b4a7af8fabbdfcec28d))
</details>

## v0.17.1 (2023-04-29)

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
    - Release gix-index v0.16.1 ([`08c6f9d`](https://github.com/Byron/gitoxide/commit/08c6f9de95c65ff05db4ce6a5593127c4280b2ef))
</details>

## v0.17.0 (2023-04-27)

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

## v0.16.0 (2023-04-26)

### New Features

 - <csr-id-c402891d269a77913be39a92b1fc7fccba509557/> `cache::state::ignore::Source` to specify where to read `.gitignore` files from.
   This allows better tuning and makes it more versatile for usage in any application, not
   just `git`.
 - <csr-id-745fc37b87d64e4fd821a692b223eda4f5df81ce/> provide statistics for cache operations, and turn debug API into better public API for `Cache`.
   That way it's a bit clearer what it is doing and does away with
   some rather dubious test code.
 - <csr-id-0a8e50f64a8a730fbbd14b465e08d96ebfcf697d/> diff between worktree and index

### Bug Fixes

 - <csr-id-27157ae7b3bec4e3117645a6456ff8cc28755f81/> `cache::Ignore` assures that case-sensitivity is handled similarly to git.
   Previously directory excludes like `dir/` could (possibly) yield different results compared to git.
   This is an opportunitstic change as it wasn't possible to trigger the wanted behaviour in a test
   related to directory-specific matching. It did trigger, however, when matching normal patterns
   which indicated there was indeed a bug.

### New Features (BREAKING)

 - <csr-id-af9ca15622be650b0fb65acf1b501918df880fd7/> support to obtain `Attributes` using the `Cache` type.
 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.
 - <csr-id-b645d28f9641c6b4022e1e37ad9fe528922ec747/> remove types that are now available in `gix-os`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 14 calendar days.
 - 27 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-worktree v0.16.0, gix v0.44.0 ([`4527fb8`](https://github.com/Byron/gitoxide/commit/4527fb8e0fe0786a70ccfd8c3f5c5e79e8867944))
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`d7173b2`](https://github.com/Byron/gitoxide/commit/d7173b2d2cb79685fdf7f618c31c576db24fa648))
    - Release gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0 ([`e4df557`](https://github.com/Byron/gitoxide/commit/e4df5574c0813a0236319fa6e8b3b41bab179fc8))
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'index-entries-attrs' ([`f37a930`](https://github.com/Byron/gitoxide/commit/f37a930aefa27e67f0b693ba9669cc26d49044fa))
    - Remove ignored archive to assure tests run as intended ([`df28b7d`](https://github.com/Byron/gitoxide/commit/df28b7d3fb7f114c862ba5559ab3974ce752cb65))
    - `cache::state::ignore::Source` to specify where to read `.gitignore` files from. ([`c402891`](https://github.com/Byron/gitoxide/commit/c402891d269a77913be39a92b1fc7fccba509557))
    - Provide statistics for cache operations, and turn debug API into better public API for `Cache`. ([`745fc37`](https://github.com/Byron/gitoxide/commit/745fc37b87d64e4fd821a692b223eda4f5df81ce))
    - Remove duplicate usage of `case` in `cache::state::Attributes|Ignore` types. ([`21b4e67`](https://github.com/Byron/gitoxide/commit/21b4e676ab094850ba808e085e2e1c3b1eb2eb61))
    - Merge branch 'attributes-cache' ([`3456c84`](https://github.com/Byron/gitoxide/commit/3456c845dfeedd2fa96b4313b1a84c8cbe9433c5))
    - Support to obtain `Attributes` using the `Cache` type. ([`af9ca15`](https://github.com/Byron/gitoxide/commit/af9ca15622be650b0fb65acf1b501918df880fd7))
    - `cache::Ignore` assures that case-sensitivity is handled similarly to git. ([`27157ae`](https://github.com/Byron/gitoxide/commit/27157ae7b3bec4e3117645a6456ff8cc28755f81))
    - Merge branch 'fix-819' ([`69faad0`](https://github.com/Byron/gitoxide/commit/69faad0d7cc100de54d757d42acc152a22edc022))
    - Adjust to improvements in `gix-object` ([`efeb29d`](https://github.com/Byron/gitoxide/commit/efeb29daaf4199748702f42966470a41ded3c6de))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Make fmt ([`5d2b5d0`](https://github.com/Byron/gitoxide/commit/5d2b5d02c3869e07dc2507a8f2519ee1df633df7))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Minor adjustments to the worktree structure. ([`8920229`](https://github.com/Byron/gitoxide/commit/89202296f63dacedfd396aefe25e686b4d426b2a))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Create new `gix-fs` crate to consolidate all filesystem utilities ([`f8cc33c`](https://github.com/Byron/gitoxide/commit/f8cc33cb372dd2b4bbe4a09cf4f64916681ab1dd))
    - Add a test to assure we can detect conflicts ([`055611c`](https://github.com/Byron/gitoxide/commit/055611c6eee840f6ea870418448733bb491437aa))
    - Adjust to changes in `gix-index` ([`cae539b`](https://github.com/Byron/gitoxide/commit/cae539b87164fabd258255a5d217decf3182d81f))
    - Refactor ([`691758a`](https://github.com/Byron/gitoxide/commit/691758a4491f8430b61e418dad33d8d901f89361))
    - Make it compile ([`6acc5f1`](https://github.com/Byron/gitoxide/commit/6acc5f10de5d21115772def2b86bc5d3d8ddf662))
    - Merge branch 'main' into dev ([`23ee47f`](https://github.com/Byron/gitoxide/commit/23ee47fb24c197f8437bd426544b2aa74e005bdc))
    - Merge branch 'worktree-stack' ([`3d47919`](https://github.com/Byron/gitoxide/commit/3d47919c1a2f83fc7c1fd7ae590d098057a22626))
    - First steps of attribute cache initialization. ([`e24b677`](https://github.com/Byron/gitoxide/commit/e24b6773b0626acdd156b2ff205532df530cbcd3))
    - Adjust to changes in `gix-attributes` ([`1755c81`](https://github.com/Byron/gitoxide/commit/1755c81f64ce8a68807c2026eeae13dc46021db1))
    - Remove types that are now available in `gix-os` ([`b645d28`](https://github.com/Byron/gitoxide/commit/b645d28f9641c6b4022e1e37ad9fe528922ec747))
    - Add test for racy git detection ([`c5adbe1`](https://github.com/Byron/gitoxide/commit/c5adbe1849aa776055a1de56b12528b032fee0ff))
    - Remove unused config option ([`55d8902`](https://github.com/Byron/gitoxide/commit/55d890271ee0a6f2bc25efa7378cce1a76235aa5))
    - Ensure stable sort order for deterministic tests ([`0582ec5`](https://github.com/Byron/gitoxide/commit/0582ec534d10dad3da0da3ea139c3200083eb86e))
    - Improve terminology and documentation ([`8b2bcdc`](https://github.com/Byron/gitoxide/commit/8b2bcdccb2351ab533d41159c956a1f157f23eea))
    - Clean up status::Diff implementations ([`8df154b`](https://github.com/Byron/gitoxide/commit/8df154b5b9cb9a614063498ce64658907955f94f))
    - Use existing concurrency primitive in_parallel ([`c5f3fc8`](https://github.com/Byron/gitoxide/commit/c5f3fc8b5875745eb50bd80005b43a66cf255acb))
    - Parallel status check ([`d7f250d`](https://github.com/Byron/gitoxide/commit/d7f250ddbd53a994a17db41f86cc780b45e9ee5a))
    - Streamline status API ([`0f747f3`](https://github.com/Byron/gitoxide/commit/0f747f303089fd862c24d4ad93b75d3064c9328b))
    - Centralize index entry Stat creation/comparison ([`870bdb2`](https://github.com/Byron/gitoxide/commit/870bdb2f3957e0f5690679e2aeb6752cd0b8d93e))
    - Change index/worktree diff to a visitor based API ([`1c43c75`](https://github.com/Byron/gitoxide/commit/1c43c75966994ada6b84cf0d69cd13fb7cebf7cd))
    - Refactor gix_worktree::read module ([`16eab81`](https://github.com/Byron/gitoxide/commit/16eab8166ea03489c3ac5537365d69626b419bfe))
    - Refactor ([`31ddda2`](https://github.com/Byron/gitoxide/commit/31ddda2ae47f68c9a487d12c1f7ffa1d14de13d1))
    - Diff between worktree and index ([`0a8e50f`](https://github.com/Byron/gitoxide/commit/0a8e50f64a8a730fbbd14b465e08d96ebfcf697d))
    - Add function to read blob from worktree ([`38e228c`](https://github.com/Byron/gitoxide/commit/38e228c61be79e2c03457a69242b2abd523f755b))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
</details>

## v0.15.2 (2023-03-30)

### Documentation

 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.7.3, gix-config-value v0.10.2, gix-config v0.20.1, gix-discover v0.16.2, gix-index v0.15.1, gix-odb v0.43.1, gix-packetline v0.15.1, gix-protocol v0.30.2, gix-worktree v0.15.2, gix v0.43.1 ([`38eed1d`](https://github.com/Byron/gitoxide/commit/38eed1d06e7cbb8fbcd54b2cad3163ca45e0baf1))
    - Fix minor typos ([`cc48c35`](https://github.com/Byron/gitoxide/commit/cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5))
</details>

## v0.15.1 (2023-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 15 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-prompt v0.3.3, gix-diff v0.28.1, gix-discover v0.16.1, gix-pack v0.33.2, gix-transport v0.29.1, gix-protocol v0.30.1, gix-revision v0.12.1, gix-worktree v0.15.1, gix v0.43.0, safety bump gix v0.43.0 ([`5dc1f9f`](https://github.com/Byron/gitoxide/commit/5dc1f9f2bcb8b3e147115fcb6f76558e8f48ffef))
    - Prepare changelogs prior to release ([`3016a28`](https://github.com/Byron/gitoxide/commit/3016a285f566bdfe7de2774fa6f2254c1b1a2c51))
    - Correct more typos with `typos` tool. ([`2321eb9`](https://github.com/Byron/gitoxide/commit/2321eb971c2b89551506e2016a3495fafd15b47d))
    - Merge branch 'fix-cred-helper' ([`01277a6`](https://github.com/Byron/gitoxide/commit/01277a681e4997896e04567490c572b5af606f35))
</details>

## v0.15.0 (2023-03-10)

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
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`29a0870`](https://github.com/Byron/gitoxide/commit/29a087043d1feb2f127b065341c8028d0bd0301e))
    - Prepare changelogs prior to release ([`e06f5f5`](https://github.com/Byron/gitoxide/commit/e06f5f523e83f4da390eddbebcb9a2d58674587b))
</details>

## v0.14.0 (2023-03-04)

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

## v0.13.0 (2023-03-01)

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

## v0.12.3 (2023-02-20)

### Bug Fixes

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

## v0.12.2 (2023-02-17)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Changed (BREAKING)

 - <csr-id-d7ee622cfa9fc3cf7a97b823dd70f0aa1355365e/> Simplify `Cache` by removing its lifetime.
   The lifetime was more of a premature optimization that makes actually using
   the cache much harder than it needs to be.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 340 commits contributed to the release over the course of 405 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 11 unique issues were worked on: [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366), [#384](https://github.com/Byron/gitoxide/issues/384), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 11 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Update changelog ([`b3ee7c6`](https://github.com/Byron/gitoxide/commit/b3ee7c6f7553de6bff4934bbdf38f6c6ea2cf349))
    - Preempt the eventual need for a worktree implementation ([`bce67d8`](https://github.com/Byron/gitoxide/commit/bce67d8ec58f78a1fce1c76f7b93d9650f9f550e))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Status quo test that shows gitoxide has the same limitation as git ([`5f6c2fb`](https://github.com/Byron/gitoxide/commit/5f6c2fb7787e674aa05af6185e665d6a33860f02))
    - Refactor ([`36fa167`](https://github.com/Byron/gitoxide/commit/36fa16761bd59d9c314e29b1b0911608ae409c1f))
    - Improve how directory excludes are handled ([`bea5ea5`](https://github.com/Byron/gitoxide/commit/bea5ea5cb3d304e73260fc1139b8fdc1acc139d7))
    - Fix inverted logic for matching non-negative pattern in `is_excluded()` ([`6d5784f`](https://github.com/Byron/gitoxide/commit/6d5784fc961c08fda7affffa4601baaea0000b98))
    - Reorganize types to properly represent worktrees in their various 'states' ([`b46bff5`](https://github.com/Byron/gitoxide/commit/b46bff58e40bb9805af7ee7f96272f0dc19c0ac7))
    - A sketch for worktree state ([`55e17a4`](https://github.com/Byron/gitoxide/commit/55e17a402c70be64609f0ffa98d1eaeee5146439))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Revert "Turn attribute files into a Cow to support other usecases…" ([`ed7f223`](https://github.com/Byron/gitoxide/commit/ed7f223b1bee688dbd257a59f3317f39bf5eb2cd))
    - Turn attribute files into a Cow to support other usecases… ([`d0c8407`](https://github.com/Byron/gitoxide/commit/d0c84079bdd4bb7746f47f132868ed4743f5dda0))
    - Make use of new git-glob::Pattern::to_string() feature ([`d29932d`](https://github.com/Byron/gitoxide/commit/d29932dc579f0579990bca1dcfc656ac020be50e))
    - Some tests to check pattern negation ([`2672a25`](https://github.com/Byron/gitoxide/commit/2672a25dae546f85807a7e5ec1939240221a5a14))
    - Test for case-sensitivity as well ([`120675d`](https://github.com/Byron/gitoxide/commit/120675db0508a6bb9d1e0eca45edf3f15632cd2f))
    - The stack now allows to change a non-dir into a dir ([`6793bab`](https://github.com/Byron/gitoxide/commit/6793bab687bf492da545981e0116322dab4455cb))
    - Allow check-ignore style queries with API that doesn't remove trailing slashes ([`e68cd69`](https://github.com/Byron/gitoxide/commit/e68cd692b5230592ca2ca17418d9b9fda9f3e317))
    - More tests and fixes to assure directory logic in stack works ([`2010ddd`](https://github.com/Byron/gitoxide/commit/2010dddf244335f3967d0debb5d8e0f3ffdac6a7))
    - Improved testing… ([`e191b72`](https://github.com/Byron/gitoxide/commit/e191b7220c5286bb0d0038398810ae344de626d3))
    - Refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - Don't hardcode case in state::Ignore ([`a6532e7`](https://github.com/Byron/gitoxide/commit/a6532e7fd94757dc5b4b231b63cb2cbcacf1e602))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - Adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Adjustments to go along with changes in git-features ([`c55cac6`](https://github.com/Byron/gitoxide/commit/c55cac6a1ada77619bb5723717a5a6d757499fa9))
    - Refactor ([`8345b7c`](https://github.com/Byron/gitoxide/commit/8345b7caa0cc1cd8489e41822eea89da4c539e6d))
    - Customize stack operation to support the notion of directories ([`2659816`](https://github.com/Byron/gitoxide/commit/26598163ce0a029e7eb92d862f899bdaadad3e90))
    - And finally, we can read ignore files from the index, too ([`910d500`](https://github.com/Byron/gitoxide/commit/910d5000d479939c14e330b6f1a12d50dd57cdd6))
    - Wire everything up to have all data where it needs to be, but… ([`34d0d5c`](https://github.com/Byron/gitoxide/commit/34d0d5c5bedae5ed069fd147c19cfb7414b66fb5))
    - Refactor ([`883d78d`](https://github.com/Byron/gitoxide/commit/883d78d3d17cae1b3bdd9801abb3ee6f9452c1a0))
    - Fix MSRV ([`63f0839`](https://github.com/Byron/gitoxide/commit/63f08391af5da3901190797532566758e3dff9e3))
    - Support for shared attribute file names ([`e4044a4`](https://github.com/Byron/gitoxide/commit/e4044a48c606497e5de0fd711c7a5ce7afc44117))
    - Use a separate path mapping to enable clone-avoidance ([`e525b5e`](https://github.com/Byron/gitoxide/commit/e525b5e5138ec0050f1ff178b5985cc7ce440b3a))
    - Fix borrow check issues the fast way, but… ([`514e2f4`](https://github.com/Byron/gitoxide/commit/514e2f424fa4976693393c6d0911b724f94b1c70))
    - Try to keep borrows to path backing alive but… ([`4234b84`](https://github.com/Byron/gitoxide/commit/4234b8497e3819eaae66f4c0462b5fc29509d675))
    - Refactor ([`b14904b`](https://github.com/Byron/gitoxide/commit/b14904b54587f99f8741fa59eda6c2b9db98fff7))
    - Doing things directly works fortunately ([`6f74f85`](https://github.com/Byron/gitoxide/commit/6f74f8516ba73c35b1b327aae491f70f83eefafd))
    - An attempt to build a lookup table of attribute files, but… ([`9841efb`](https://github.com/Byron/gitoxide/commit/9841efb566748dae6c79c5990c4fd1ecbc468aef))
    - Refactor ([`475aa6a`](https://github.com/Byron/gitoxide/commit/475aa6a3e08f63df627a0988cd16c20494960c79))
    - Make .gitignore name overridable ([`155bb82`](https://github.com/Byron/gitoxide/commit/155bb820be03d4ac210b6ae4a76ecfb33445271e))
    - A test to check skip-worktree special case with ignore files ([`dec9f33`](https://github.com/Byron/gitoxide/commit/dec9f332ecd2eaf7bad8ce0f94194d68624d9ac7))
    - A baseline test that indicates how excludes aren't using data from the index initially ([`e58b771`](https://github.com/Byron/gitoxide/commit/e58b771cd514024e63c1ab7af7c0d0abad00797d))
    - First primitive ignore pattern test works ([`0424136`](https://github.com/Byron/gitoxide/commit/04241367e8ce99ce6c7583d5dac4955fad3d6542))
    - Refactor to make push/pop with mutable state work; prepare to read .gitignore files ([`8d1000b`](https://github.com/Byron/gitoxide/commit/8d1000b30257675564195202b15dca1ab1538227))
    - Add baseline test to motivate implementing ignore file stack ([`ce40add`](https://github.com/Byron/gitoxide/commit/ce40add21add518374d9ff6d40fe488e2f29ce6d))
    - Re-export `git-glob` as its `Case` type is part of the public API ([`4b72045`](https://github.com/Byron/gitoxide/commit/4b7204516a7c61162a2940eb66e8a7c64bf78ce7))
    - Sketch state for handling attributes as well ([`d87d62d`](https://github.com/Byron/gitoxide/commit/d87d62db5cf327397390ec7888c1d1155619ba38))
    - Sketch state for handling excludes ([`eb525f7`](https://github.com/Byron/gitoxide/commit/eb525f76134a2ffd770848941c976ec456fcc296))
    - Sketch how attribute globals could be used in worktrees ([`97ee03d`](https://github.com/Byron/gitoxide/commit/97ee03d5e4703b583dd5bb741dbf43f310404882))
    - Adjustments to support lower MSRV ([`16a0973`](https://github.com/Byron/gitoxide/commit/16a09737f0e81654cc7a5bbc9043385528524ca5))
    - Remove `git-dir` for `checkout()` as it's something to be dealt with elsewhere ([`f7996b8`](https://github.com/Byron/gitoxide/commit/f7996b8f6a877275b8725804c558b51732e8b469))
    - An idea on how to test excludes, but… ([`9c036e8`](https://github.com/Byron/gitoxide/commit/9c036e81b3abcd5dcde2b023459a15cbd281824d))
    - Make attributes and ignore configuration possible, but… ([`8a75fd7`](https://github.com/Byron/gitoxide/commit/8a75fd745a194786f0da7c1fd660211446ea51f7))
    - Refactor ([`80af734`](https://github.com/Byron/gitoxide/commit/80af734a5ddfd0785ec946a3609887b5d503d03d))
    - Provide a platform for multiple queries at a dir cache level ([`48be382`](https://github.com/Byron/gitoxide/commit/48be3828ea07124c4d21cb10121780f596116bcb))
    - Be explicit about the cache-modes that actually happen ([`dc12f88`](https://github.com/Byron/gitoxide/commit/dc12f88a5d2e54e2ff987127bab37e5bd7ce314a))
    - Refactor ([`5d30018`](https://github.com/Byron/gitoxide/commit/5d300181c0696430c75bec7070da35cb308a1b9a))
    - Refactor ([`fe46078`](https://github.com/Byron/gitoxide/commit/fe46078dd9496744b048165fba548df5c3f76991))
    - Port PathCache over to `Stack` ([`ebfea8d`](https://github.com/Byron/gitoxide/commit/ebfea8d4be1afb3bd47bcffbaf5d705bed2d1ed6))
    - A sketch for a generalized version of a path stack ([`0d3ba1a`](https://github.com/Byron/gitoxide/commit/0d3ba1a02f076d32334d85f68d99e6b8033844ad))
    - Refactor ([`fe6641c`](https://github.com/Byron/gitoxide/commit/fe6641c86704df67b020510700e9c087fff5a52c))
    - Refactor ([`f86eacc`](https://github.com/Byron/gitoxide/commit/f86eacc5cfaf6d88ead4f8dbd65989d32674c213))
    - Use io-close instead of close-file - works ([`279461b`](https://github.com/Byron/gitoxide/commit/279461ba1741ace0399127ca9089230082bbf3e0))
    - Better error handling on close ([`a28c9b3`](https://github.com/Byron/gitoxide/commit/a28c9b32466a431450a504e313d2e49926e36a98))
    - Try close_file crate and see tests fail for some reason ([`c7e1400`](https://github.com/Byron/gitoxide/commit/c7e140094a3a5947cf59107d5a621245ea2ecbeb))
    - More multi-threaded test stability ([`be5a19e`](https://github.com/Byron/gitoxide/commit/be5a19e0eb2e895d03b80afc24c7b8d2d436458d))
    - Avoid racyness in worktree tests ([`c8a1319`](https://github.com/Byron/gitoxide/commit/c8a13198a12939befa473b30131e5a763c6fc28c))
    - Stabilize assertions in parallel mode ([`21d6f88`](https://github.com/Byron/gitoxide/commit/21d6f880293de4e8ffc6a8472eb1b54d8b1b105a))
    - A reducer which produces progress reporting each time it feeds ([`e83079d`](https://github.com/Byron/gitoxide/commit/e83079d219c96692725ab8af1c0e656cb331ecd8))
    - Call chunk processing in threaded processor ([`6bfd865`](https://github.com/Byron/gitoxide/commit/6bfd865a0578eeacd8d19eaa89d8914ac947c62a))
    - Conversions from Rc to arc for Handle ([`c19331e`](https://github.com/Byron/gitoxide/commit/c19331e001e587e4fca74f3e9fec28a7df922c0a))
    - Basic parallelization, without proper reducer, just so it compiles ([`5f29c0f`](https://github.com/Byron/gitoxide/commit/5f29c0f66d0aa6c045bfdf6f39a806ce8c4a5100))
    - Decouple amount of bytes written from progress ([`9ecdade`](https://github.com/Byron/gitoxide/commit/9ecdade0f117b966c98f48d1879bdba21ccaafd7))
    - Parallel and non-parallel tests ([`1cd7eb3`](https://github.com/Byron/gitoxide/commit/1cd7eb3f720e8b66792c942a99d7d9d85069ec03))
    - Switch index checkout to chunk-based operation ([`e5f6943`](https://github.com/Byron/gitoxide/commit/e5f69433e4a6cc7866b666e0baccfa32efb92a7f))
    - Proper handling of interruptions during checkout ([`7575a58`](https://github.com/Byron/gitoxide/commit/7575a5854ebe61a5941177efb470143192223ef3))
    - Add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - Refactor ([`542f49b`](https://github.com/Byron/gitoxide/commit/542f49beb811f7f9bf9dff3cd19694498f6cf9e2))
    - Refactor ([`c3c31af`](https://github.com/Byron/gitoxide/commit/c3c31afb9dee5040abef7a8d6f8e1e2cba29e2d7))
    - Fix windows test expecations for good ([`81bcb8d`](https://github.com/Byron/gitoxide/commit/81bcb8d281099e952a5e3c075d9578f15f2f2a0d))
    - Try to fix windows once again ([`ff95265`](https://github.com/Byron/gitoxide/commit/ff95265a35fb9f340c3a9fa78f8beba24d6734ff))
    - Some more debugging on windows ([`0c18443`](https://github.com/Byron/gitoxide/commit/0c18443f5195e10c99504c4f527c1882fcf84e45))
    - Debug mode for windows ([`8f3bc5a`](https://github.com/Byron/gitoxide/commit/8f3bc5a3195770753b0b6445259ce20ab609b393))
    - See if we can remove symlinks this way on windows ([`0bc9489`](https://github.com/Byron/gitoxide/commit/0bc94891c92f324d3940e064e8918b117db4641d))
    - Delete directories recursively on overwrite-existing ([`ea561e6`](https://github.com/Byron/gitoxide/commit/ea561e6f7d398991f214957dbd92e1b6a81e9ab0))
    - Better symlink checking on ubuntu ([`facad25`](https://github.com/Byron/gitoxide/commit/facad25c08b82a975eda70493d4818ca7c560aa8))
    - Overwrite-existing support with tests ([`49d1d34`](https://github.com/Byron/gitoxide/commit/49d1d34dff76d8b1e5e7fa9d08e6ead4e8bca018))
    - Fix dir-cache to properly handle its valiity which fixes test ([`52c0058`](https://github.com/Byron/gitoxide/commit/52c0058531df1a0f3fc755c5c51e71d34841cb77))
    - Delayed symlink creation for everyone, but… ([`ab5cd3d`](https://github.com/Byron/gitoxide/commit/ab5cd3d383c3c6cb31a7b8d387daedacb9e3838f))
    - Delayed symlink creation for windows, but… ([`77b053d`](https://github.com/Byron/gitoxide/commit/77b053dfd38e30a8ab397704059283a4766b9601))
    - Prepare for first overwrite test… ([`cd6e086`](https://github.com/Byron/gitoxide/commit/cd6e08644df3a2b52aa70a2f37e988ec10b280f0))
    - Fix case-insensitive tests ([`ccd25cb`](https://github.com/Byron/gitoxide/commit/ccd25cb5929554c69ea1250c6d2762fdd6ef5bbd))
    - Allow symlinks to dirs to be returned, too ([`d3d7a7c`](https://github.com/Byron/gitoxide/commit/d3d7a7c3c67868ba0fda6b04e6874aa2f91f638b))
    - Try to fix tests on linux ([`9f9d36d`](https://github.com/Byron/gitoxide/commit/9f9d36d7d7bba443fba5917e9920911596fd64f6))
    - A stab at making file writes safer… ([`805c0da`](https://github.com/Byron/gitoxide/commit/805c0da62204b8c4675c9c098e10eb0fe2bc12a9))
    - Mior refactor and notes towards parallelization ([`99de1ef`](https://github.com/Byron/gitoxide/commit/99de1ef494719cb4d46e3414474e619225fe7bd4))
    - Return proper errors during checkout object lookup ([`f9beac0`](https://github.com/Byron/gitoxide/commit/f9beac0471a38cb4c3b070ecb576ed1a39456bd6))
    - Switch worktree to thiserror ([`bacc654`](https://github.com/Byron/gitoxide/commit/bacc65481d4ff5ecfbdf3755383b60f354deaf47))
    - Sub-command to print multi-index entries ([`6c10e09`](https://github.com/Byron/gitoxide/commit/6c10e097a432d81b930008abc00c6821ed7ac9be))
    - Bring back more detailed errors in case of keep-going ([`8198817`](https://github.com/Byron/gitoxide/commit/8198817507a5e9c6e6fb847a45ac47bd38de68f6))
    - Use progress to print errors right when they happen ([`af03686`](https://github.com/Byron/gitoxide/commit/af03686b5abf9548300a83329500b27acd66e16a))
    - Implement 'keep-going' for index checkout ([`ecebc55`](https://github.com/Byron/gitoxide/commit/ecebc55f8321c67f57111f8d0002e75388dd3734))
    - Support for forceful removal of symlinks or files during dir creation ([`749c310`](https://github.com/Byron/gitoxide/commit/749c3100d785f7ac373bdb109fda21f2ac62d5c0))
    - Forbid symlinks and files in the path ([`de58f50`](https://github.com/Byron/gitoxide/commit/de58f50748bd70e39d29e503a7f4b1e6c9b20093))
    - Avoid popping the entire cached path ([`a3501df`](https://github.com/Byron/gitoxide/commit/a3501df6eb8d2fd3176434c80c443316e91dabb6))
    - Basic impl of the dir cache which already avoids unnecessary allocations ([`cb36d56`](https://github.com/Byron/gitoxide/commit/cb36d5691294971e1b0e097ed11908768283731a))
    - Sketch out dir cache and realize that git uses chdir ([`f4621cc`](https://github.com/Byron/gitoxide/commit/f4621cc4dd48fcd4b1aba294c811bc92f2715981))
    - Allow writing empty files during checkout but also query the odb ([`5388d80`](https://github.com/Byron/gitoxide/commit/5388d8091ef02cf927478a1492847ae1666040d4))
    - Basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - Basic progress reporting for checkout ([`039e822`](https://github.com/Byron/gitoxide/commit/039e822bb4e56e49432db5c53081e0eb39588d66))
    - Support for unicode-precomposition for gix apps ([`e90c123`](https://github.com/Byron/gitoxide/commit/e90c123675a98ab62fc6bb22019f889cee8b7301))
    - Fix symlink creation on windows, hopefully ([`4b1650b`](https://github.com/Byron/gitoxide/commit/4b1650ba1988f52a7a91ce4f5327eca350f32520))
    - Gather more information about test failure on windows ([`be5e3fb`](https://github.com/Byron/gitoxide/commit/be5e3fb3a19f86e37244b17055bf31cc455e78e8))
    - Hopefully fix symlink creation on windows ([`acb8acd`](https://github.com/Byron/gitoxide/commit/acb8acd905c4a7ec0fbc831b159f626962c0a37d))
    - Refactor ([`48dc401`](https://github.com/Byron/gitoxide/commit/48dc40195fd3d41d1fa5cd6326422ae18266dd7d))
    - Also validate symlink collisions ([`322c316`](https://github.com/Byron/gitoxide/commit/322c3161947cd5c10e3122c097d5a888726d42c1))
    - Fix compile warnings ([`58145bc`](https://github.com/Byron/gitoxide/commit/58145bc0fc329c370638a336215679fa727a9f0f))
    - Try to fix windows ([`5c1e727`](https://github.com/Byron/gitoxide/commit/5c1e727a1af4b9a0b5b7dcfca0d1ef5a533a66b6))
    - Finally an understanding on collision checking ([`0454e4a`](https://github.com/Byron/gitoxide/commit/0454e4a6f039541255728c4c8e076578236f0d86))
    - Add check_stat and trust_ctime options to index checkout ([`1a502c7`](https://github.com/Byron/gitoxide/commit/1a502c7e456a191d8639b799648ea33eb5a7dac2))
    - Validate that colliding files are checked out ([`09fecd9`](https://github.com/Byron/gitoxide/commit/09fecd9687cf3271f7138bca9214ba99c17b5ef7))
    - Support for executable bit check ([`267e3a7`](https://github.com/Byron/gitoxide/commit/267e3a7f4718c8f724e3e4488dd24dcebfc69413))
    - Probe precompose unicode ([`0c1c006`](https://github.com/Byron/gitoxide/commit/0c1c00689000dfc943ed25cd52eac42e3642a78c))
    - Refactor ([`fc816bd`](https://github.com/Byron/gitoxide/commit/fc816bd12f142d1df4d10429ee5b56e9eb5fbf4d))
    - Determine filesystem case ([`f8e1de0`](https://github.com/Byron/gitoxide/commit/f8e1de0dc031ad73084b2da6a6d39960b9b78b4b))
    - Basic test for filesystem probing ([`adbed12`](https://github.com/Byron/gitoxide/commit/adbed121f969a05b622d0325b434b3c6d44ae248))
    - Symlink probing ([`1bfbf1d`](https://github.com/Byron/gitoxide/commit/1bfbf1d120e31474367cd2008e1715c50af19071))
    - Make clear that we are currently only dealing with checkout during clone ([`178beb4`](https://github.com/Byron/gitoxide/commit/178beb42eaf1112143299eafa7fc93106eb9fc5b))
    - Refactor for checkout to use fs::Context ([`8914fcc`](https://github.com/Byron/gitoxide/commit/8914fcc114cdf920f2f4162e71d4d390007f6f3b))
    - Document-features support for git-index and git-worktree ([`1367cf5`](https://github.com/Byron/gitoxide/commit/1367cf5bc5908639e67e12f78f57835c5fd68a90))
    - Support for 'serde1' feature in git-worktree ([`f11929c`](https://github.com/Byron/gitoxide/commit/f11929c9652b2f414029f2ad02dacee238a138d1))
    - Sketch filesystem context, without probing for now ([`de3749e`](https://github.com/Byron/gitoxide/commit/de3749e1426d48a1d31a0ddc1fddfdb394a01078))
    - Refactor ([`004394a`](https://github.com/Byron/gitoxide/commit/004394ad04a965b631c5d75a7eced632540d9e1e))
    - Restructure tests ([`831c429`](https://github.com/Byron/gitoxide/commit/831c4294c87aae0594e1238177dd71efb997cbde))
    - Make fmt ([`636fa8a`](https://github.com/Byron/gitoxide/commit/636fa8a97ce56982c76dffc64ee084e31d39afad))
    - Strucural refactor ([`cdca1df`](https://github.com/Byron/gitoxide/commit/cdca1dfec590d24dd42f34294e21f4bdf61d36ad))
    - Allow mutation of entries during iteration, while obtaining their path ([`d0c4563`](https://github.com/Byron/gitoxide/commit/d0c4563f71ea18aaf8ae21dd8646ab256a550594))
    - Refactor ([`72af261`](https://github.com/Byron/gitoxide/commit/72af261603ee38651e15015547871d0510ce6370))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Fix build ([`f6d9693`](https://github.com/Byron/gitoxide/commit/f6d969370b8ef05b3b29983dcd9f6fa11d6225f2))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - The first possibly working version of loading a mailmap with multiple sources ([`98d745e`](https://github.com/Byron/gitoxide/commit/98d745e8080975a91cff1ce75e187258c851d3f4))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - Prevent line-ending conversions for shell scripts on windows ([`96bb4d4`](https://github.com/Byron/gitoxide/commit/96bb4d460db420e18dfd0f925109c740e971820d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - Add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - Auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Make it possible to clone empty remote repositories ([`e97eeda`](https://github.com/Byron/gitoxide/commit/e97eeda45c9cc0736273c735a9959ac1ff29fc9d))
    - Refactor ([`dd3b336`](https://github.com/Byron/gitoxide/commit/dd3b336656f5282f386c1cba0974abd2b09d81e9))
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
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
    - Adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - Adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - Adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Rename `git-worktree` to `gix-worktree` ([`2b59ff2`](https://github.com/Byron/gitoxide/commit/2b59ff25fd095cac93f7d607594f68a5e329861e))
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
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
    - Release git-worktree v0.12.2 ([`0780f35`](https://github.com/Byron/gitoxide/commit/0780f354aaa2fa16200b33b0f76c0cec21e79cfc))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Make fmt ([`e22080e`](https://github.com/Byron/gitoxide/commit/e22080e4a29d0bad15a99d565a5e3e304a8743ec))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Release git-ref v0.23.0, git-config v0.15.0, git-command v0.2.2, git-diff v0.26.0, git-discover v0.12.0, git-mailmap v0.9.0, git-pack v0.30.0, git-odb v0.40.0, git-transport v0.25.2, git-protocol v0.26.1, git-revision v0.10.0, git-refspec v0.7.0, git-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - Prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - Release git-index v0.12.1 ([`8aa5c1d`](https://github.com/Byron/gitoxide/commit/8aa5c1db9e342cc49dfa588d5b4b9f893067dbf7))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Simplify `Cache` by removing its lifetime. ([`d7ee622`](https://github.com/Byron/gitoxide/commit/d7ee622cfa9fc3cf7a97b823dd70f0aa1355365e))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - Prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Improved working of docs ([`1ef704e`](https://github.com/Byron/gitoxide/commit/1ef704e7daa83e298d99d94a3493296739338110))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - Merge branch 'fix-gix-index-from-tree' ([`da5f63c`](https://github.com/Byron/gitoxide/commit/da5f63cbc7506990f46d310f8064678decb86928))
    - Fix build ([`bb81abe`](https://github.com/Byron/gitoxide/commit/bb81abece95f466c38567d7488c0e6ae1a22e06b))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-worktree v0.4.3, git-testtools v0.8.0 ([`b2e4bf2`](https://github.com/Byron/gitoxide/commit/b2e4bf2c11ff2c3c32efcb91837fb5677714bdf9))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-worktree v0.4.2, git-repository v0.22.0 ([`2f0f530`](https://github.com/Byron/gitoxide/commit/2f0f530fb1d644bc0694e04f3c9309149f526e42))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-worktree v0.4.1, git-repository v0.21.0 ([`ee383f3`](https://github.com/Byron/gitoxide/commit/ee383f347371007f1c4d3a2a98c5511d7e0793a8))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
    - Release git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`d4df661`](https://github.com/Byron/gitoxide/commit/d4df661dbf60dad75d07002ef9979cabe8a86935))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Make it harder to forget documentation in git-worktree ([`15d87ee`](https://github.com/Byron/gitoxide/commit/15d87ee99ef269985e8f378bb2ab9c8931e8fd7d))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-worktree v0.3.0, git-repository v0.19.0 ([`0d8e856`](https://github.com/Byron/gitoxide/commit/0d8e8566dc5c6955487d67e235f86fbc75a3a88a))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`3f71246`](https://github.com/Byron/gitoxide/commit/3f7124616ab9752007b8cf03e1c6a3a796ffee0b))
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`5845934`](https://github.com/Byron/gitoxide/commit/584593448b560afdd60dbdbdff901d267082765e))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Thanks clippy ([`aeebc5f`](https://github.com/Byron/gitoxide/commit/aeebc5fe743faa7d436b1d0a30d60aafbbaeeb3d))
    - Thanks clippy ([`b199367`](https://github.com/Byron/gitoxide/commit/b1993672f5a7c516611814fd7c5d6bf796419082))
    - Merge branch 'main' into worktree-stack ([`8674c11`](https://github.com/Byron/gitoxide/commit/8674c11973e5282d087e35a71c70e418b6cc75be))
    - Fix release build ([`f7c1920`](https://github.com/Byron/gitoxide/commit/f7c1920214ebfc38676d1d53cc064b0f3d8ece4e))
    - Fix release build ([`2705679`](https://github.com/Byron/gitoxide/commit/2705679ddf7e5fe12e93ad214c15d5006c073818))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - Thanks clippy ([`1d365d2`](https://github.com/Byron/gitoxide/commit/1d365d2c6fe19ac8e27c60e3d2596a583a183728))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Thanks clippy ([`07a4094`](https://github.com/Byron/gitoxide/commit/07a4094965ac1b4eb223da8e5ca5cc4a86c5f596))
    - Thanks clippy ([`0e2a243`](https://github.com/Byron/gitoxide/commit/0e2a2438da35c0abb412682b103e5be171b1c3ad))
    - Thanks clippy ([`3229240`](https://github.com/Byron/gitoxide/commit/322924037a1710f35e4134e5a35c82b3d4266a1f))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - Thanks clippy ([`a8e9497`](https://github.com/Byron/gitoxide/commit/a8e9497caebf1c0e9faac537717cd86378f1acf6))
    - Thanks clippy ([`e04cba8`](https://github.com/Byron/gitoxide/commit/e04cba8837340d1ca0f102a340e52e8610fb0750))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Refactored code and tests ([`a4b880c`](https://github.com/Byron/gitoxide/commit/a4b880cf17665b61e3f7f193de57704b1db5318f))
    - Refactored tests ([`25a9dc1`](https://github.com/Byron/gitoxide/commit/25a9dc16dbb26e9aa0f3379b2af53cc0baa96663))
    - Reduce io calls ([`e838eaa`](https://github.com/Byron/gitoxide/commit/e838eaa5721d8b1b13155aa81234c9c44d9b15fe))
    - Refactor errors and remove unwraps ([`eaee855`](https://github.com/Byron/gitoxide/commit/eaee85595dc658549e62e3292b025ec016e70abd))
    - Implemented git-worktree ([`4177d72`](https://github.com/Byron/gitoxide/commit/4177d72c95bd94cf6a49e917dc21918044e8250b))
    - Release git-worktree v0.0.0 ([`ddb1bf4`](https://github.com/Byron/gitoxide/commit/ddb1bf49e3b5b663fcf166d8cbce416e78d9fc18))
</details>

