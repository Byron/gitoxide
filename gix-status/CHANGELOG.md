# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.23.1 (2023-08-02)

### Bug Fixes

 - <csr-id-82ae37d70dc244cdf705d20c617d4b0e6bf3cdbf/> don't panic during checkouts when submodules or sparse directories are encountered.
   Now we trace instead.

## 0.23.0 (2023-07-22)

### Bug Fixes (BREAKING)

 - <csr-id-68bd71cc0c47b0d86c7cb8fb9fe73a03cf8b52f6/> make it possible to pass information about the directory status when matching attributes.
   This is significant for archiving operations, even though it's not important when matching attributes
   otherwise.

## 0.22.0 (2023-07-19)

### Bug Fixes (BREAKING)

 - <csr-id-68bd71cc0c47b0d86c7cb8fb9fe73a03cf8b52f6/> make it possible to pass information about the directory status when matching attributes.
   This is significant for archiving operations, even though it's not important when matching attributes
   otherwise.

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

## 0.21.1 (2023-06-29)

A maintenance release without user-facing changes.

## 0.21.0 (2023-06-29)

A maintenance release without user-facing changes.

## 0.20.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

## 0.19.0 (2023-06-10)

A maintenance release without user-facing changes.

## 0.18.0 (2023-06-06)

### Bug Fixes

 - <csr-id-c86ca69b57cda379fdfe1b4a7af8fabbdfcec28d/> disallow reading macros when they are not global.

## 0.17.1 (2023-04-29)

A maintenance release without user-facing changes.

## 0.17.0 (2023-04-27)

A maintenance release without user-facing changes.

## 0.16.0 (2023-04-26)

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

## 0.15.2 (2023-03-30)

### Documentation

 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

## 0.15.1 (2023-03-26)

A maintenance release without any user-facing changes.

## 0.15.0 (2023-03-10)

A maintenance release without user-facing changes.

## 0.14.0 (2023-03-04)

A maintenance release without user-facing changes.

## 0.13.0 (2023-03-01)

A maintenance release without user-facing changes.

## 0.12.3 (2023-02-20)

### Bug Fixes

 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

## 0.12.2 (2023-02-17)

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

 - <csr-id-d7ee622cfa9fc3cf7a97b823dd70f0aa1355365e/> Simplify `Cache` by removing its lifetime.
   The lifetime was more of a premature optimization that makes actually using
   the cache much harder than it needs to be.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

## 0.12.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.12.0 (2023-01-09)

A maintenance release without user-facing changes.

## 0.11.0 (2022-12-30)

### Changed (BREAKING)

 - <csr-id-d7ee622cfa9fc3cf7a97b823dd70f0aa1355365e/> Simplify `Cache` by removing its lifetime.
   The lifetime was more of a premature optimization that makes actually using
   the cache much harder than it needs to be.

## 0.10.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.9.0 (2022-11-21)

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

## 0.8.0 (2022-11-17)

A maintenance release without user-facing changes.

## 0.7.0 (2022-11-06)

A maintenance release without user-facing changes.

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.3 (2022-08-27)

Maintenance release without user-facing changes.

## 0.4.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.4.1 (2022-08-17)

A maintenance release without user facing changes.

## 0.4.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.3.0 (2022-06-13)

A maintenance release without user-facing changes.

## 0.2.0 (2023-10-12)

A maintenance release without documented changes.

### New Features (BREAKING)

 - <csr-id-60c948f55ec432ab40b826a9ce8cb3d8fe15a543/> replace `conflict` marker with detailed decoding of stages.
   We also adjust the returned data structure to allow the input to be immutable,
   which delegates entry updates to the caller.
   
   This also paves the way for rename tracking, which requires free access to entries
   for searching renames among the added and removed items, and/or copies among the added ones.
 - <csr-id-b55a8d58b8bd9e1ba2f9049668c166e75fb0a360/> add entries-relative index to each change.
   That way it's possible to lookup other, surrounding entries in case
   of conflicts or easily find entries that didn't change.
 - <csr-id-0d01eb28ebb2305873726ba1892204fd151f4c4f/> provide statistics at the end of a index status operation
 - <csr-id-53de126ea571ef9ed911e66c26a4c36cfdc7e0dd/> add support for submodule status
   Previously, submodules where ignored. Now they are treated correctly
   as 'directory' which is compared to what's in the worktree.
   
   We also simplify blob handling.

### New Features

 - <csr-id-de66b4c26a937a4f6462dff5ec58275dae01813a/> `status` now supports filters.
   This is important as it allows to streaming-read from the worktree and
   correctly change, for example, `git-lfs` files back into their manifests,
   and to arrive at the correct hash.
 - <csr-id-0e10b62557fbd5b33a4aebab24e442e23304ac0a/> a way for `status` to stop early.
   That way, 'is_dirty()` scenarios can be done without wasting too much time.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 8 calendar days.
 - 17 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.13.1, gix-features v0.36.0, gix-actor v0.28.0, gix-object v0.38.0, gix-glob v0.14.0, gix-attributes v0.20.0, gix-command v0.2.10, gix-filter v0.6.0, gix-fs v0.8.0, gix-commitgraph v0.22.0, gix-revwalk v0.9.0, gix-traverse v0.34.0, gix-worktree-stream v0.6.0, gix-archive v0.6.0, gix-tempfile v11.0.0, gix-lock v11.0.0, gix-ref v0.38.0, gix-config v0.31.0, gix-url v0.25.0, gix-credentials v0.21.0, gix-diff v0.37.0, gix-discover v0.26.0, gix-ignore v0.9.0, gix-index v0.26.0, gix-mailmap v0.20.0, gix-negotiate v0.9.0, gix-pack v0.44.0, gix-odb v0.54.0, gix-pathspec v0.4.0, gix-packetline v0.16.7, gix-transport v0.37.0, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0, safety bump 37 crates ([`68e5432`](https://github.com/Byron/gitoxide/commit/68e54326e527a55dd5b5079921fc251615833040))
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Merge branch 'improvements' ([`429e7b2`](https://github.com/Byron/gitoxide/commit/429e7b25f93c8a7947db19bafa74babf199a1aa6))
    - Adapt to changes in `gix-object` ([`f712aeb`](https://github.com/Byron/gitoxide/commit/f712aeb1fe14ab60c58da5317410e397115f8d35))
    - Merge branch 'reset' ([`b842691`](https://github.com/Byron/gitoxide/commit/b8426919a491dc3a7df01ee3f258fc0d8a3a327c))
    - Replace `conflict` marker with detailed decoding of stages. ([`60c948f`](https://github.com/Byron/gitoxide/commit/60c948f55ec432ab40b826a9ce8cb3d8fe15a543))
    - Add entries-relative index to each change. ([`b55a8d5`](https://github.com/Byron/gitoxide/commit/b55a8d58b8bd9e1ba2f9049668c166e75fb0a360))
    - `status` now supports filters. ([`de66b4c`](https://github.com/Byron/gitoxide/commit/de66b4c26a937a4f6462dff5ec58275dae01813a))
    - A way for `status` to stop early. ([`0e10b62`](https://github.com/Byron/gitoxide/commit/0e10b62557fbd5b33a4aebab24e442e23304ac0a))
    - Provide statistics at the end of a index status operation ([`0d01eb2`](https://github.com/Byron/gitoxide/commit/0d01eb28ebb2305873726ba1892204fd151f4c4f))
    - Add support for submodule status ([`53de126`](https://github.com/Byron/gitoxide/commit/53de126ea571ef9ed911e66c26a4c36cfdc7e0dd))
    - Add symlink checking for `gix status` ([`c044919`](https://github.com/Byron/gitoxide/commit/c044919383b14f9355cf279add64297c2acedeed))
</details>

## 0.1.0 (2023-09-24)

<csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/>

An initial release with the ability to checkout indices with simple files only.

### Chore

 - <csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/> split tests off into their own crate to allow feature toggles.
   That way we can test with the `parallel` feature and won't have to
   create bogus feature toggles that are only used for testing, yet visbible
   to users.

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### New Features (BREAKING)

 - <csr-id-2d011253c64aaeede546a2e4cdd143142689044b/> various improvements

### Bug Fixes

 - <csr-id-93feea269eebd114e866e6f29f4a73c0096df9e0/> split tests off into their own crate to allow feature toggles.
   That way we can test with the `parallel` feature and won't have to
   create bogus feature toggles that are only used for testing, yet visbible
   to users.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 35 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Fix gix-status dev dependencies - can't use versions ([`f692809`](https://github.com/Byron/gitoxide/commit/f69280929c4ebc3a7677dc3039a61fee6bcfe428))
    - Prepare changelogs prior to release ([`8a60d5b`](https://github.com/Byron/gitoxide/commit/8a60d5b80877c213c3b646d3061e8a33e0e433ec))
    - Merge branch 'reset' ([`54a8495`](https://github.com/Byron/gitoxide/commit/54a849545140f7f1c0c7564c418071c0a76a34e7))
    - Various improvements ([`2d01125`](https://github.com/Byron/gitoxide/commit/2d011253c64aaeede546a2e4cdd143142689044b))
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'adjustments-for-cargo' ([`b7560a2`](https://github.com/Byron/gitoxide/commit/b7560a2445b62f888bf5aa2ba4c5a47ae037cb23))
    - Release gix-index v0.23.1 ([`11b9c71`](https://github.com/Byron/gitoxide/commit/11b9c71311df978ebb20cca0d765cf249c8eedcf))
    - Release gix-date v0.7.4, gix-index v0.23.0, safety bump 5 crates ([`3be2b1c`](https://github.com/Byron/gitoxide/commit/3be2b1ccfe30eeae45711c64b88efc522a2b51b7))
    - Merge branch 'fixes' ([`4bfd1cc`](https://github.com/Byron/gitoxide/commit/4bfd1cc8f7922a8c4de6b9d078d54b93e78f51ff))
    - Adapt to changes in `gix-index` and pass skip-hash through for performance.. ([`713cd59`](https://github.com/Byron/gitoxide/commit/713cd59f0b1eff6397b80f1e1fceec278532fd59))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Merge branch 'gix-submodule' ([`8f3f358`](https://github.com/Byron/gitoxide/commit/8f3f358800f1fe77d7ba7ebd396a90b692d3c0c1))
    - More cleanup of test crates ([`73c685a`](https://github.com/Byron/gitoxide/commit/73c685a67debcfa26a940f37bbca69cb3a4af57e))
    - Split tests off into their own crate to allow feature toggles. ([`93feea2`](https://github.com/Byron/gitoxide/commit/93feea269eebd114e866e6f29f4a73c0096df9e0))
    - Merge branch 'worktree-organization' ([`8d0d8e0`](https://github.com/Byron/gitoxide/commit/8d0d8e005d7f11924a6717954d892aae5cec45e7))
    - Adapt to changes in `gix-worktree` ([`e5717e1`](https://github.com/Byron/gitoxide/commit/e5717e1d12c49285d31a90b03b7f8e9cbc6c1108))
    - Create new `gix-status` crate to capture `git-status` like functionality ([`be9af32`](https://github.com/Byron/gitoxide/commit/be9af327c75d693658a2427ee9a711e631a8da7d))
</details>

## 0.0.0 (2022-01-08)

Reserve the name for a necessary crate of the `gitoxide` project.

