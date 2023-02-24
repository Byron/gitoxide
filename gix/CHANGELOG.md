# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.37.2 (2023-02-24)

### Bug Fixes

 - <csr-id-1d3d22d45e70222c12fcf5a82063ceb9321a0129/> reproduce a diff issue and fix it
   Diffs could be quite wrong and this is a small repro along with the fix.

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
    - Fix new diff tests on windows ([`b1ec1b7`](https://github.com/Byron/gitoxide/commit/b1ec1b776696b4b1d73e3dd26cbaf33260367855))
    - Prepare changelog for release ([`13a1ec1`](https://github.com/Byron/gitoxide/commit/13a1ec1803d677c2e94f3ea0461118c2426f8071))
    - Merge branch 'rename-tracking' ([`550144a`](https://github.com/Byron/gitoxide/commit/550144a5fd37d501d86f4b1c4db2948d951d1c93))
    - Reproduce a diff issue and fix it ([`1d3d22d`](https://github.com/Byron/gitoxide/commit/1d3d22d45e70222c12fcf5a82063ceb9321a0129))
</details>

## 0.37.1 (2023-02-21)

A maintenance release to restore MSRV (1.64) support.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-config v0.16.3, gix v0.37.1 ([`a3c283f`](https://github.com/Byron/gitoxide/commit/a3c283ff0e3f21cedb3ba7cd464fdfa0f5133af0))
    - Prepare changelogs prior to release ([`362d659`](https://github.com/Byron/gitoxide/commit/362d659f946ca1ff2cbf915766113a34a9df97b3))
    - Restore msrv compatibility by removing sole `if let ... else` ([`9160659`](https://github.com/Byron/gitoxide/commit/91606597b714a6e9b3a2c071bdb08baeacd6056b))
</details>

## 0.37.0 (2023-02-20)

### Bug Fixes

 - <csr-id-d3b974000133caa0ea107cb4724b950eda91d69b/> `Repository::object_cache_size()` now unsets the cache if `Some(0)` is passed.
   Previously it would fail.

### New Features (BREAKING)

 - <csr-id-ed87f4c7c2799625bc6c7109368687908f0fb6f0/> `object::tree::diff::Platform::track_rewrites(...)`
   The invocation of `object::tree::diff::Platform::track_rewrites(Rewrites { percentage: None, ..Default::default() })`
   is now able to explicitly configure perfect rename tracking without percentage of equivalence.
   
   By setting `percentage = Some(<fraction>)` one can set how similar both files should be to be considered related.
   
   The same can be configured for copy-tracking, which also includes something like `--find-copies-harder`.
   
   Note that by default, renames are considered if a file looks 50% similar, and copies tracking is
   using the same convention.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.3, gix-diff v0.26.2, gix-traverse v0.22.2, gix v0.37.0, safety bump 3 crates ([`8b3e42f`](https://github.com/Byron/gitoxide/commit/8b3e42f69fe97fe5083eb845c323f10d7ac087b2))
    - `Repository::object_cache_size()` now unsets the cache if `Some(0)` is passed. ([`d3b9740`](https://github.com/Byron/gitoxide/commit/d3b974000133caa0ea107cb4724b950eda91d69b))
    - Merge branch 'rename-tracking' ([`35415c5`](https://github.com/Byron/gitoxide/commit/35415c5061bf5ea90a04db80d06ac3622d0b0f1a))
    - `object::tree::diff::Platform::track_rewrites(...)` ([`ed87f4c`](https://github.com/Byron/gitoxide/commit/ed87f4c7c2799625bc6c7109368687908f0fb6f0))
</details>

## 0.36.1 (2023-02-20)

### Bug Fixes

 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.36.1 ([`fac6bce`](https://github.com/Byron/gitoxide/commit/fac6bce2f9942d7f333f66a92374d5400a00b0a5))
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`9604783`](https://github.com/Byron/gitoxide/commit/96047839a20a657a559376b0b14c65aeab96acbd))
    - Compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
    - Release gix-glob v0.5.4 ([`c56d336`](https://github.com/Byron/gitoxide/commit/c56d3365fde21120cf6101cf34f8b5669804977c))
    - Release gix-transport v0.25.5 ([`f872ba8`](https://github.com/Byron/gitoxide/commit/f872ba8271a5d632acc071e7a857ef19f7cf5610))
</details>

## 0.36.0 (2023-02-17)

### New Features

 - <csr-id-4f49992fae2bc60b22644e86808d61afe557f192/> cloning repositories doesn't require a committer anymore.
   This is similar to what git does and probably a decent convenience to have.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#737](https://github.com/Byron/gitoxide/issues/737)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#737](https://github.com/Byron/gitoxide/issues/737)**
    - Cloning repositories doesn't require a committer anymore. ([`4f49992`](https://github.com/Byron/gitoxide/commit/4f49992fae2bc60b22644e86808d61afe557f192))
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
    - Release git-date v0.4.3, git-hash v0.10.3, git-features v0.26.5, git-actor v0.17.2, git-glob v0.5.4, git-path v0.7.2, git-quote v0.4.2, git-attributes v0.8.3, git-bitmap v0.2.2, git-chunk v0.4.2, git-command v0.2.4, git-commitgraph v0.13.1, git-config-value v0.10.2, git-tempfile v3.0.3, git-lock v3.0.3, git-validate v0.7.3, git-object v0.26.2, git-ref v0.24.1, git-sec v0.6.3, git-config v0.16.2, git-prompt v0.3.3, git-url v0.13.3, git-credentials v0.9.2, git-diff v0.26.2, git-discover v0.13.1, git-fetchhead v0.1.0, git-filter v0.1.0, git-hashtable v0.1.2, git-traverse v0.22.2, git-index v0.12.4, git-lfs v0.1.0, git-mailmap v0.9.3, git-note v0.1.0, git-pack v0.31.0, git-odb v0.41.0, git-packetline v0.14.3, git-pathspec v0.1.0, git-transport v0.25.5, git-protocol v0.26.4, git-rebase v0.1.0, git-revision v0.10.4, git-refspec v0.7.3, git-sequencer v0.1.0, git-submodule v0.1.0, git-tix v0.1.0, git-tui v0.1.0, git-worktree v0.12.3, safety bump 2 crates ([`90035a3`](https://github.com/Byron/gitoxide/commit/90035a332d0ba67584558db3605500fbcb424ddd))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - Adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - Adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - Adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - Adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - Adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - Adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - Adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
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
    - Fix `gix` changelog (find-replace issue) to indicate renaming from `git-repository` ([`f86b780`](https://github.com/Byron/gitoxide/commit/f86b7803e85839450ed2eeef57fe738da1e3ec87))
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
</details>

## 0.35.0 (2023-02-13)

This is the last release under this name and merely a notice to inform that `git-repository` from now on is `gix`.

Furthermore, all `git-*` crates belonging to the `gitoxide` project will be renamed to `gix-*`.

### Changed (BREAKING)

 - <csr-id-1408482fd21be7487b46753bb54a018c7a164f34/> a note of the pending rename of `git-repository` to `gix`

### New Features

 - <csr-id-069eb6c3f0844b43873ae1bd536e2bca53ff5c8a/> tree diffs with simple rename and copy tracking in cases where there is no additional modification.
   As the fastest way of rename tracking, we now offer support for tracking renames and copies,
   that is a file was renamed or copied without modification.
 - <csr-id-f6ed34aa254d34e596ad027c33f78404a630ff76/> Add `diff.renames` and `diff.renameLimit` keys to config tree.
   In preparation for the implementation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.16.1, git-revision v0.10.3, gix v0.35.0 ([`74390ba`](https://github.com/Byron/gitoxide/commit/74390baf9d177a1abe3c7c35f1d9bc67faba1e97))
    - Show more debugging information if unreachable code is reached. ([`66f5341`](https://github.com/Byron/gitoxide/commit/66f53414efef6cfd6d03f830520964c9bdd23634))
    - Prepare changelogs prior to release ([`446f866`](https://github.com/Byron/gitoxide/commit/446f866d146e255ab8302b89f87bf28f2c5f3733))
    - Merge branch 'rename-crates' ([`6461c3d`](https://github.com/Byron/gitoxide/commit/6461c3da4d6daee857606d94294c3f87fc36965a))
    - Rename `git-repository` to `gix` ([`7bed2a9`](https://github.com/Byron/gitoxide/commit/7bed2a96604397fa990f427b1a970ddeb6f09f1c))
</details>

## 0.34.0 (2023-02-09)

<csr-id-a01f5d72346c36fdcb77af095273da6f4ab86e21/>

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-297d59e8396fbe2e5a2224a8524fa0778e786773/> add `env::collate::fetch::Error` - a combined error type with its own API.
   This error API allows to look at all the steps it takes to perform an operation and
   gather insights from it which require understanding a lot about the semantics of
   the contained errors.
 - <csr-id-d792ea543246632bf1ca8d0e1d239bbe7f07e219/> use enumerations to advertise progress ids publicly.
   Previously these were an implementation detail which also means they
   couldn't be relied upon.
   
   Thanks to an intermediate enumeration, they become part of the public API
   and their actual value is not exposed.
 - <csr-id-5dc408f726d6f0f480438348eb5d713776329710/> read shared indices by dissolving them into the current one.
   This allows the 'link' extension to be processed correctly, even though it
   won't be maintained. When written back, the 'link' extension will be removed
   automatically.

### Bug Fixes

 - <csr-id-5d3a3a245968d5ad8c29ea11a99b4896d1b41191/> don't panic, but error when parsing the rev-specs `^`, `..`, `...`.

### Chore (BREAKING)

 - <csr-id-a01f5d72346c36fdcb77af095273da6f4ab86e21/> adjust to changes in `gitoxide` for clap upgrade to 4.1

### New Features (BREAKING)

 - <csr-id-2faad43d11283ff06381c51d2466307cfb8736ff/> transfer knowledge about configuration and its usage into the type system.
   That way it's possible to use configuration overrides, even though ultimately being strings,
   in a type-safe manner and leverage code-completion while at it.
   
   In that process, we also change `Repository::(committer|Author)()` to return
   `Option<Result<...>>` to be able to account for date parse errors.

## 0.33.0 (2023-01-10)

<csr-id-dd7f3bf19cce0d214924fa86aeb4c5823f5bcc02/>

### Chore (BREAKING)

 - <csr-id-dd7f3bf19cce0d214924fa86aeb4c5823f5bcc02/> upgrade MSRV to v1.64 (possible due to `windows` upgrade)

## 0.32.0 (2023-01-09)

<csr-id-80dcb406c5f588122531da115398094de3c3af79/>

### Bug Fixes

 - <csr-id-a05b1c4d82bc6c7758989a3bbe326ea610903820/> default author and committer time
   When needing to fallback to a default author or committer signature, the
   time from GIT_AUTHOR_DATE should only be used for the author and
   GIT_COMMITTER_DATE should only be used for the committer and not
   intermixed. This change enforces that constraint.
 - <csr-id-ec7bf71b60f8c1e7529d610557c0305d624c1253/> signature name and email resolution
   The name and email for the author and/or committer may come from different
   config files. For example, user.name may be set in the global config and
   user.email may come from the repository local config.
   
   This case was broken due to Personas.from_config_and_env() only looking in
   the last config section containing, for example, a "user" section. Thus if
   the user.name and user.email are split across multiple sections (i.e.
   originating from separate config files), the fallback name and email
   ("gitoxide" and "gitoxide@localhost") would be used.
   
   The solution is to use gix_config::File::string() to lookup the name and
   email separately. The string() method correctly resolves the value by
   looking through all sections from all files in the correct order.

### Other

 - <csr-id-80dcb406c5f588122531da115398094de3c3af79/> name and email from different config sections
   The user.name, user.email, author.name, author.email, committer.name, and
   committer.email configuration may come from different sections from
   different config files. This new test exercises a couple of scenarios that
   are currently broken.

### Reverted (BREAKING)

 - <csr-id-87abb51596bd0a5a6b552a5de98a920d6c797e3c/> `committer_or_default()`, `author_or_default()` and `user_default()`.
   This means that all methods that previously succeeded by adding a default
   will now fail.
   
   This is preferable over 'doing something' and also admits that gits
   guesswork that tries to find user information by querying the system
   is nothing we want to repeat.

## 0.31.0 (2022-12-30)

<csr-id-9fabfc50007603f9c1f7e70b5bb79a39726b12af/>
<csr-id-91720798889ee7eb26da03a9e732caedda83b3e3/>

### New Features

 - <csr-id-d48b9a7ae2d51676c7549377bcb0b9d3baa83681/> fetching `ssh` urls can ask for a different username.
   If authentication fails, the user will be queried for a different username
   to try authentication via ssh mechanisms again.
 - <csr-id-61d89f586a0ad913fc2f502520282520a5e1fd15/> collect ssh-specific options to control how the ssh program is invoked.
   These are passed through when creating the ssh transport.

### Other

 - <csr-id-9fabfc50007603f9c1f7e70b5bb79a39726b12af/> explain how it's possible to deal with the first commit when comparing trees
   The reason the other tree isn't an option is that it's a special case that can more easily be handled
   with an `.unwrap_or_else(|| repo.empty_tree())` (or similar) for those who need it.
   
   Making the empty tree explicit also helps to deal with diffs from the empty tree (which can't be `Option<Tree>`)
   to the first tree of the first commit.

### Chore (BREAKING)

 - <csr-id-91720798889ee7eb26da03a9e732caedda83b3e3/> upgrade to prodash v23

## 0.30.2 (2022-12-26)

<csr-id-114f184855b6177aa1f0dbf6e6589f23deb5ffe6/>

### New Features

 - <csr-id-38ae61a805bd8cca5df8d1c1dcf3a8a0f9c85f5a/> make more HTTP options available
   - `http.schannelCheckRevoke`

### Other

 - <csr-id-114f184855b6177aa1f0dbf6e6589f23deb5ffe6/> provide a repository clone example

## 0.30.1 (2022-12-22)

### New Features

 - <csr-id-ca84c87734804cbfc65e311b89ff6ccfc236149c/> `open::Options::open_path_as_is()` allows to avoid 'smart opening' to try the path verbatim.
   The path to git repositories is well-known as they either end in `.git` or `.../.git`.
   If this is not the case, by default we append `/.git` to the path.
   
   With this new option enabled, no path transformations apply to open the given path as is,
   which is preferable if you know it's a non-standard git repository folder name.

## 0.30.0 (2022-12-19)

<csr-id-fceee748c114b2d0760074e911e533cd020f6996/>

### Changed

 - <csr-id-a4ac9cf3e667a3059e33aac8188150529578622d/> represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration.
   That way it becomes more obvious where values are coming from.

### New Features

 - <csr-id-1683a848459cae2b9182b365e3e22b0e8ba73534/> expose `gix-features` crate at root under `features`.
   That way application developers can use more of the utilities
   that power most of the `gitoxide` plumbing crates.
 - <csr-id-90ef6fc36b440cc4baf3fde4a30060f1b4a0c8cf/> `Remote` knows about its `tagOpt` configuration.
   That way it's clear if it should or shouldn't fetch included/reachable
   tags automatically.
   
   The default setting for this is to include tags, similar to `git`.
   
   The `fetch_tags()` accessor allows to query this information, and the
   `with_fetch_tags()` builder method allows to set the value comfortably
   right after creating the `Remote` instance.
   
   The `tagOpt` key will also be written as part of the remote's git
   configuration.
   
   Clone operations can set the `Tags` setting when configuring the
   remote in a callback.
   
   This also comes with a fix to assure that ref-updates aren't skipped
   just because there was no pack to receive. That way, locally missing
   refs or tags will automatically be put back.
 - <csr-id-28e48083052216ddf1fd1f187cc22d506d3d9f86/> network related Error type support `is_spurious()` method.
   That way the caller can determine more easily if it makes sense
   to try again.
 - <csr-id-457c2e081b1aa5dfaab3f663b9aba66c16369939/> Make `prodash::tree` available as `progress::tree`.
 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.
 - <csr-id-fc64693d5af0fda402c560d10d15652c24d14219/> add `permissions::Environment::http_transport`.
   That way it's possible to deny using environment variables that affect
   the HTTP transport, like setting the proxy.
 - <csr-id-0ce29a965cf16273cf74bd22e40f464e322e2f62/> `open::Options::modify()` as general pattern to allow builder methods usage in `&mut self`.
   That way it's easier to configure both the `full` and the `partial` trust instances
   of discovery options.
 - <csr-id-8482f90d0a2b61259cd51ca4f40ce322e388cb34/> Add `Repository::commit_as(committer, author, …)` convenience method.
   That way it's, very much beyond convenience, possible to set the time
   of a commit.
   
   Many thanks to @epage for the suggestion.
 - <csr-id-c8835c6edae784c9ffcb69a674c0a6545dbb2af3/> upgrade to `prodash 21.1` and add `Ids` to all progress instances.
   That way callers can identify progress they are interested in, say, for
   selective visualizations.

### Bug Fixes

 - <csr-id-d659bda2e1b0fcab529df7af6467f063ae5d0dd7/> provide a clearer error message when trying to open a git repository that isn't one.
 - <csr-id-ff0332e815c228cc5cdfe58c3598ad261bb2879e/> http transports can now reuse a connection.
   This makes connections more efficient generally and `cargo` relies
   on that behaviour in their tests as well.
 - <csr-id-9079b9d2e5f7cc133c6f2b2c2e64245b150c7d74/> allow to open a `Repository` from if 'config' file is missing.
   In this case, treat it similar to having an empty repository configuration
   file and assume defaults everywhere.
 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning
 - <csr-id-b77fc86ab580dd81b08022996f07cc7925104e77/> `tree::diff::Platform::for_each_to_obtain_tree()` now properly surfaces user provided errors.
   Previously it would squelch them unintentionally.
   
   First discovered via https://github.com/Byron/crates-index-diff-rs/issues/35.
 - <csr-id-5386eed6a13a32a850c59706b15d8988c67733ce/> when fetching from file://, don't upset windows by trying `d:/foo`, use `d:\\foo` instead.
 - <csr-id-363ac7a74ec841505b5fc7cc1b8fae11c0a63ea9/> `config::CommitAutoRollback` now implements `DerefMut`.

### Changed (BREAKING)

 - <csr-id-3c84cebc5997356ff5f531c6cc9567bdd9b83eb5/> default features are set to `max-performance-safe` to assure compatibility.
   Previously the `max-performance` setting might have caused issues during compilation
   or issues at runtime if libraries like `git2` are used in the same binary, and the
   new default feature settings maximizes compatibility so this won't happen.
   
   For best performance, however, one will have to activate the `max-performance`
   feature on top of that.
 - <csr-id-5fe6aa3f3f2f81d84f0d96e874e68a8bf4de1db1/> environment variable permissions are per topic.
   Now `Permissions` for environment variables are so that they
   are by topic instead of by prefix, by default. That way
   it's easier to allow or deny particular sets of related
   environment variables.
   
   The catch-all permissions by prefix are still present for all
   other variables that aren't contained in one particular topic.
 - <csr-id-49f39d6bb487c0254176a5082f2c7851b83952a1/> `open::ReplacementObjects` is removed in favor of two custom git-configuration flags.
   Now it's possible to map the environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS`
   to custom git configuration keys which can also be set, namely `gitoxide.odb.replaceObjectsRefBase`
   and `gitoxide.odb.noReplaceObjects`.
   
   Along with the possibility of disabling the usage of `GIT_` prefixed environment variables one
   reaches the previous level of control without making object replacement a special case.

### New Features (BREAKING)

 - <csr-id-f8a2bfb93dadbc64393135e0a447f3d76628509c/> `interrupts::init_handler()` can now be undone.
   This can be done by calling `deregister()` or `auto_deregister()` on the return value
   of `interrupts::init_handler(…)`.
   
   That way it's possible to temporarily do interrupt handling only while some methods
   that require it are running.
 - <csr-id-becbd8d896a1663f1607be4e86e632773e926f1f/> represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration.
   That way there is a unified system for how to set values, which may be overridable by configuration
   variables or not.
   
   With this changes, the explicit application of environment variables for setting the cache
   isn't required anymore as everything happens using git-configuration, and automatically,
   while providing full control like before.
 - <csr-id-f16e36168cc93768ba5d53c9848ff2e8432d06b1/> remove `SnapshotMut::apply_cli_overrides()` in favor of `open::Options::cli_overrides()`.
 - <csr-id-84d594caf3f04f1ce337e455343278a6f4674957/> more type-safety for remote names by making clear they can be named after URLs.

### Other (BREAKING)

 - <csr-id-fceee748c114b2d0760074e911e533cd020f6996/> `Remote::with_refspec()` to `Remote::with_refspecs()` to allow adding more than one refspec as part of the builder.

## 0.29.0 (2022-11-21)

<csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/>

### New Features

<csr-id-ff9e1571b558475e727dc6ba11dab24ef15fb6f4/>

 - <csr-id-3ddbd2de369b521fa3f21935f10fe9c248840893/> Make `reqwest` TLS backend configuration easy.
   We provide the choice of `native-tls` or `rust-tls`. If none is
   provided, the user can configure on their on similar to how it's done
   in `gix`.
   
   Please note that a choice now has to be made or HTTPS will not be
   available, so use one of…
   
   * blocking-http-transport-reqwest-rust-tls
* blocking-http-transport-reqwest-native-tls

### Bug Fixes

 - <csr-id-c6a690219915b2b401d2d11f61db35b2931e5b3a/> `gix::Commit::describe()` chooses tag names (more) correctly.
   Previously, if there were multiple choices for tags on the same commit,
   `git describe` would disagree with `gitoxide` due to different
   prioritization of names.
   
   This has now been fixed.
 - <csr-id-84ed89c3bf6692f18c4bb97173527de1bcba7ac6/> also sort entries lexicographically

### Other

 - <csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/> Set GIT_EDITOR in make_rebase_i_repo.sh
   If the user has core.editor set in their global git config, then that value
   takes precedence over the EDITOR environment variable. The GIT_EDITOR
   environment variable, however, has higher precedence than core.editor. For
   this test, using GIT_EDITOR ensures that the desired sed command line is
   used.

### New Features (BREAKING)

 - <csr-id-bc2a399f2fbb69d23b0b05e8dfb95f3c64ff93b9/> rename `blocking-http-transport` feature to `blocking-http-transport-curl`; add `blocking-http-transport-reqwest`.
   With the new and relatively immature second tier http backend we pave
   the way to support builds without the use of open-ssl and probably many
   other C libraries.
   
   Note that it's early and not `http` configuration option is implemented
   yet.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.28.0 (2022-11-17)

<csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/>

### New Features

 - <csr-id-58e14884b1d025651f874d899cb2d627c4a2afbf/> `Id` implements `std::fmt::Display`
 - <csr-id-25f7aabe38267b6b6c0547806028b2becb806416/> `Remote::repo()` to obtain the underlying repository.
   For convenience.
 - <csr-id-709a73229b7cde56ddffa099158661632c606468/> Support for user-customizable user agent strings.
   Doable by setting the `gitoxide.userAgent` variable.
 - <csr-id-e60d07997989993216c2bd93efeb6f1b48da0a87/> add `env::agent()` for obtaining the default client agent string.

### Other

 - <csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/> try to apply maybe-async in a place where it's probably not possible.
   The goal is to re-use the existing tests, but right now they only
   compile in async mode as the `maybe-async` crates needs
   a feature to be set. Doing so is hard(er) if it's not already used
   in the main crate, which we do not and will do our best to avoid.

### New Features (BREAKING)

 - <csr-id-db9040f0bb3a16879c8da0252a77df80bd417387/> add `remote::Connection::with_transport_config()`, change the way `*::transport_mut()` is used.
   Previously `transport_mut()` was supposed to be used for calling
   `configure()`, but that doesn't work anymore as `configure()` can
   only effectively be called once the initialization of the Connection
   is complete, as it may depend on the Remote name AND the credential
   provider for proxy auth credential acquisition.
   
   Thus we allow callers to set the transport options they need in advance
   for it to be used when needed.

## 0.27.0 (2022-11-08)

### Changed (BREAKING)

 - <csr-id-c50868c7ed7309515b4f0a188213d332d57dd146/> Move `object::tree::diff::change::DiffPlatform` to `object::blob::diff::Platform`.
 - <csr-id-4ee32713093c2e41a12d148c6030add1df6aa966/> new `DiffPlatform::counts()`, open `DiffPlatform` for use of `gix-diff::blob::*`.

## 0.26.0 (2022-11-06)

<csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/>

### New Features

 - <csr-id-b1edb9e3537df86669714f03666f4a88e0ac3709/> diff algorithm is controlled by git configuration `diff.algorithm`
 - <csr-id-072f5bc9c91c4c09bd6a73f9d7ac672805cae533/> Query of `core.logAllRefUpdates` is now fallible.
   This is the same behaviour as shown by `git`, which requires valid
   values or aborts.
 - <csr-id-2eaf69e5f8f8da10e5af85cb9f0c39577ad1707f/> automatically handle `.keep` files after writing a pack bundle to disk.
   The logic implemented here tries to do the right thing, that is when
   we have reason to believe that the objects in the pack are linked up
   with a ref, we delete the keep file automatically.
   
   However, if there was no local ref edit as the ref specs didn't contain
   local destinations, or if the pack was empty, then keep the .keep file
   and leave it to the caller to handle.
 - <csr-id-8b9fbd4e9ed7be37976c7203cd9a89c6116a6d3d/> Use `core.askpass` when building the credential helper.
   Previously it would only consider the environment variable, which can
   still override the provided git-configuration at core.askpass .
 - <csr-id-a9d14492322785a14f4ecb5b0d3dbdc87e56f8c5/> `remote::fetch::Prepare::handshake_outcome()` to obtain server information right after listing refs.
 - <csr-id-0b5c53ec43bdb58b2b7cf46e453ddf858770a95a/> `open::Options::config_overrides()` for early configuration; support for `init.defaultBranch`.

### Bug Fixes

 - <csr-id-f869b224170b0c49a0e4d89e88bfbf5caedaa725/> don't allow non-bare repositories to be initialized into non-empty directories.
 - <csr-id-91baefad02a0d52c745106359da3693d06aace46/> `init_bare()` now creates the destination directory if it doesn't exist.
 - <csr-id-5c11b84f4e74e3eefdd0f5804976ebfc505e0f2f/> build correct path for `$HOME/.config/…` files.
   The special per-user `ignore` and `attributes` files can also be
   defaulted if some environment variables are set and may be accessed.
   
   Previously the default for `$HOME` was incorrect, as it was missing the
   intermediate `.config/` directory. This is now present to build paths
   exactly like git.
 - <csr-id-275e80f3d602b63ef91efe31a92b4aafb2eeca44/> ref-map filtering now uses correct prefixes.
   Previously specs could get filtered out server-side as a matching prefix
   was entirely missing.

### Changed (BREAKING)

 - <csr-id-449ff066d2b5dd423c639618193dd9e54d03c1f8/> `Repository::branch_remote_name()` returns `reference::remote::Name`.
   That way it's made clear the remote can also be a URL, while rejecting
   illformed UTF8. The latter isn't valid for remote names anyway as these
   only support a very limited character set.
   
   Note that this error currently is degenerated, making it appear if the
   remote name doesn't exists if illformed UTF-8 is found in what appears
   to be a symbolic ref.
 - <csr-id-71f15fc46fbaea455cf84a2b4cfe3e680047d790/> be specific about the kind of `diff::Error`, moving it to `diff::for_each::Error`.

### New Features (BREAKING)

 - <csr-id-7413a284eb7754e63ba45d0f526347b9f79b557d/> `Tree::lookup_entry*()` returns attached `Entry` type.
   That way chaining gets even easier.

### Bug Fixes (BREAKING)

 - <csr-id-2bece79285e244a7029f9393dafc990e39420e2d/> `create::into(…)` takes `create::Kind` to determine if it's bare or not.
   First of all, `bare` is not an option anymore, but a parameter as
   it can't be defaulted.
   Other public signatures change as well to accommodate for it.

### Other (BREAKING)

 - <csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/> `DiffPlatform::text()` to `*::lines()`.
   This is more specific as one could also do character changes in a single
   line, and it adapts the signature to the new `imra-diff` powered
   mechanism, for a 2x speed boost.

## 0.25.0 (2022-10-10)

<csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/>

### New Features

 - <csr-id-22d3b37ea6239170a478b859361a7d1d7ba01a9a/> `Url::try_from(path: &std::path::Path)` for more convenient instantiation.
 - <csr-id-31a7089f2583832727e2175ada6fb5c30c3beebe/> make some private methods public to give callers more flexibility.
   This allows to implement the fetch-negotiation part oneself and break
   free from constraints of the delegate.
 - <csr-id-4367994a8a7476eb44e1309e833e345fdb78f246/> add `config::SnapshotMut::commit()` to make clear it's transactional.
 - <csr-id-d2bea003230078ffb4e6cd80d1b01c3995435a34/> add `config::SnapshotMut::forget()` to forget all changes before applying them.
   The documentation was update to make clear when the changes are applied.
 - <csr-id-4b1e3b3d91c51da3dbea9191e60f959a1266cf47/> add `Repository::find_default_remote()` which works on detached heads as well.
 - <csr-id-25f06400c49ddd1688fd76f9285542b121b223b4/> `Remote::rem_map()` now specifies ref-prefixes to the remote.
   This can greatly reduce the amount of refs sent.

### Other

 - <csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/> try to make the transport configurable after being boxed, but…
   …that would force it to be 'static, which is something we explicitly
   cannot have. We need references to be contained within, if I remember
   correctly.

### Changed (BREAKING)

 - <csr-id-e88de0f948325773db1925b07aa878e1dbb76bad/> All methods editing references don't take the author as parameter anymore.
   Instead, these are taken from the git configuration and can be
   configured on the fly with temporarily altered configuration.
 - <csr-id-3a0fb1b45c757add49677450836c0aaf6179a2b5/> remote `lock_mode` from all methods dealing with reference edits.
   It is now read from `core.filesRefLockTimeout` accordingly.

### New Features (BREAKING)

 - <csr-id-3b29fc18672c0176684c797a0f16f85d09369bf8/> make jwalk fully optional
 - <csr-id-78ad3df64f2c016ba17b158bd9ab1d2341aab399/> add `fetch::Transport::configure` to generically configure any transport.

## 0.24.0 (2022-09-20)

<csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/>
<csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/>

### New Features

 - <csr-id-0871a96b9cc84d7a496d39393e081999c0a3fe17/> `Object::peel_to_tree()` as convenience method.
   It's very common to try to work with trees, so let's make that easier.
 - <csr-id-1027be960852618915014f9ba6e6632bd4999b0e/> `interrupt::Iter` now allows accessing the inner iterator without consumption.
   This is useful if these provide additional out-of-band information.
 - <csr-id-8c2e5c60f9f5f8d0859ecd84c17af20e88812512/> Once a change is obtained, it's easy to obtain changes line by line.
 - <csr-id-7e96d1841989b37133cddf334724a2d6df557e69/> obtain a refmap after listing refs via `remote::Connection::list_refs_to_map()`.
   With it it's possible to establish a relationship between what's about
   to be fetched to local tracking branches as established by refspecs for
   fetching.
 - <csr-id-d51e7c901fe5ed60d5dd56006c5faedb71cad537/> Add `permissions::Config::git_binary` field
   When true, default false, inject the git installation configuration file
   if present at the cost of one `git config` invocation.
   
   Note that we rely on the underlying `gix-config` crate to not load
   duplicate files.
   
   We also currently lie about the scope which is actually unclear - have
   seen 'unknown' or normal scopes like `system`.
 - <csr-id-1c13f1125664fbcc276a1ca440d168d07d0bf493/> add `prompt` to top level forwarding #450)

### Bug Fixes

 - <csr-id-ae3866065c9c3c6d01709f8dde1cea1ae1345779/> rev-spec parsing can now handle the empty tree as full hex hash.
   Even though the empty-tree object can be found when searched via
   `Repository::find_object()`, previously it was not locatable when
   used during rev-spec parsing.
 - <csr-id-74ede2031d1beedf11f1cdf006fff71e597a2cb5/> `Reference::remote()` can produce remotes for URLs

### Refactor

 - <csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/> use specific error type for `rev_parse_single()`

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features (BREAKING)

 - <csr-id-2992b1ba4e7bbeab26f41175cd31fd664abf2a11/> Add reference remote name type to make usage of `remote_name()` result clear

### Other (BREAKING)

 - <csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/> `Tree::lookup_path()` -> `Tree::lookup_entry()`.

## 0.23.1 (2022-09-01)

### Changed (BREAKING)

 - <csr-id-36d8c57824a2b921012439105e49261fac66c955/> Remove 'unstable' feature.
   It's not worth maintaining it especially since everything is in
   pre-release mode right now.
   
   It might be something to re-introduce after go-live.

### Bug Fixes

 - <csr-id-d18e76cfb512ef7fe5bfee6e87726372c6a4a8b6/> `max-performance-safe` mode does not include zlib-ng adjustments anymore.
   git2 cannot handle this and fails to fetch packs after a couple of
   seconds.
   
   It's unclear what is causing this except that git2 doesn't like libz
   with zlibng support enabled, which happens if git2 in the
   same tree is with us.
 - Transitively through a kindly contributed fix in the `gix-discover` crate, `Repository` can now be opened on `exFat` volumes.

## 0.23.0 (2022-08-28)

### New Features

 - <csr-id-70aa850591de268488ae9bf2d3839a5c9c543c35/> The empty tree can always be returned by `Repository::(try_)find_object()`
   This matches the behaviour of git and libgit2.
   We consciously chose to only do this on the highest level, allowing lower
   levels to determine if the object exists or not.
 - <csr-id-8d0786646e17a82d20ca6b2799b54f6349d389f4/> Make `find::object::*::Error` publicly available.
 - <csr-id-2d0b63997b276a53b3cf8f09fac51f8e3f044bcd/> Add `Reference::delete()` for simple reference deletion
 - <csr-id-9170562059c3eaa529850025ef35ac5ffffc0fdf/> `Reference::set_target_id()` to easily set the target id of a reference
 - <csr-id-950da602925e6376b08640ed3ebfdf407394db34/> `Reference::head_ref()` to quickly access the reference the head points to.

### Bug Fixes

 - <csr-id-2834311b4f262c57e76627addaa4932196fd26b3/> `Commit::tree_id()` now returns a connected id

### New Features (BREAKING)

 - <csr-id-e090f843f5cffc8e8e47a8cac2e6fb98e4c47771/> `gix-diff` is now included by default as part of core functionality

## 0.22.1 (2022-08-24)

A maintenance release without user facing changes.

## 0.22.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-c28bcec19b5526acf888f928e6ddc4671873368e/> support avoiding usage of `fast-sha1` in gix-features separately.
   That way one has an angle on compile failures in client libraries,
   see https://github.com/o2sh/onefetch/pull/752 for motivation.
 - <csr-id-4f87a0672f7288486a9b6403c0bb07a6279d2cfe/> `Repository::write_blob[_stream]()` to more easily write blobs.
   That way, one won't have to use the underlying `objects` database but
   can remain in the land of `Repository` enabled types for longer.
 - <csr-id-d35cd2a12c6db3d655ce10cad5c027bde99e19b4/> `SnapshotMut::apply_cli_overrides()` to make it easy to support things like `-c`
 - <csr-id-2a839f3209f3bd35e0c0f7edff664cc953059f65/> `Repository::config_snapshot_mut()` to mutate configuration values in memory.
   It's a first step towards writing changes back to disk, which can work
   already, but probably wouldn't as we currently don't localize changes
   to only one section type, i.e. Api, but instead may change values
   from other sections.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Bug Fixes

 - <csr-id-ff71730b4e3635533d9969d9dd44c0f3c75c6648/> Don't fail worktree tests if the system's git version is not supporting required features

## 0.21.1 (2022-08-19)

A maintenance release that speeds up `commit.describe()` performance if `max_candidates()` is 0.

## 0.21.0 (2022-08-17)

<csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/>

### Changed

 - <csr-id-0235111a4fcc40c7b57d973bfce27a66eddea901/> Invert behaviour to `open::Options::strict_config()`, with lenient being the default.
   This means API users will get libgit2 behaviour but commands like `gix` can
   change options to emulate `git` behaviour.

### New Features

 - <csr-id-a01525d159a33d6ad60a5324f2e9abbbe17fcfad/> `Kind` can now represent submodules.
   This should complete the list of git repository types and flavors.
 - <csr-id-5dac021bbbc5621167e7f49d62b11f68f76e42b6/> `open()` and `discover()` support opening submodules.
   This includes submodule checkouts as well as their original module git
   directories.
 - <csr-id-067c3342f3564dd7f152a720e93e3aa590ae6524/> `open::Options::lenient_config(…)` to default otherwise invalid configuration values where possible
   Originally required by https://github.com/starship/starship/issues/4266 .
 - <csr-id-0bf8371706d319681c3f794af5cd13f2f50a27d0/> support core.worktree option
 - <csr-id-b47bbb787ef2e31dd2612a56f9e7759ef8a188b8/> display for `object::tree::EntryRef`
 - <csr-id-727768a49c41165b03ddcdbc71ca88b66c330f32/> `Head::prior_checked_out_branches()`
 - <csr-id-ffe72918baf5c4c9f0f3709f75f068a663778588/> `Repository::index()` and `Worktree::index()`.
   These methods provide a possibly updated shared index.
 - <csr-id-47619f7c06a49dcf60a30e1a43a5352914183092/> add `Repository::object_cache_size_if_unset()`
 - <csr-id-d2611cee61841bc7bd978bef5af7dc66154248a2/> `Commit::message_raw_sloppy()` to provide yet another way to obtain a commit message.
 - <csr-id-906c95845fa4aa2d4390c522bb566a188b8c0e78/> add `rev_spec::parse::ObjectKindHint` to support `core.disambiguate`.
   The latter is seemingly undocumented in the typical place, gix-config.
 - <csr-id-ef187f0180d89544d9015cbc2bc03d8cb51f4615/> `Remote::with_refspec()` to add new unique refspecs
 - <csr-id-d51ba42c643d8ee03a3c6b648f8524ff04827170/> `Remote::push_url()` to set it after the fact
 - <csr-id-9b07b91ad065836e7473df6635025659af2865ee/> `Repository::remote_at(…)` to create an unnamed remote
 - <csr-id-a67fc26b80e5d1183ddc5c6598396214f3e19945/> more conversions for `TryFrom`: `String` and `&str`
 - <csr-id-7a512ecdf236afc0b3d562d60fa81ab62c00cd9d/> `Head::into_remote()` to try really hard to find the correct remote
 - <csr-id-f392f26bec6069ac43ecd461b4f50e0def8b8972/> `Repository::remote_default_name()` to obtain the repo-wide remote for a a direction.
 - <csr-id-f47464f64f7c21500a24f563b25a8fc070c41778/> `Repository::branch_names()` to obtain branch names for which configuration exists.

### Bug Fixes

 - <csr-id-be6114e7c4ac48467db6acb2180b443dc9f59f32/> assure permissions per trust level are properly inherited into `open::Options`.
 - <csr-id-270242c707bd086b7746ee45b55791587f1484b1/> provide additional explanation about when to use `open::Options::with()`

### Refactor

 - <csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/> embrace `revision` module and move `rev_walk` there.
   Let's embrace the idea of structured modules and platforms in the right
   spot in the module hierarchy instead of forcing known names on it that
   over-simplify.

### Changed (BREAKING)

 - <csr-id-0deda0df55c11647f51374ed5a8bf11c932e2bae/> remove `permissions::Config::strict()` as they were unused internally.
   Furthermore, they were allowing everything as before so better not to
   have it.
 - <csr-id-1c12d49eefa6d79ef50b2960f41b29de680ac8eb/> rename `Repository::load_mailmap*` to `Repository::open_mailmap*`.
   For consistency with other similar methods.
 - <csr-id-ea35183b53f2ff71bdf2270ac4f7470a85d7756f/> remove `Repository::load_index()` in favor of `repo.worktree().open_index()`.
 - <csr-id-4fd096840ba27da6ce86678a85ede33e3be974ff/> `gix_revision` is now available in `revision::plumbing`.
   That way it won't clash with the higher-level constructs on top of it
   which use the same names.
 - <csr-id-2424957cff75daacf6f6f14f74b9639f6875c4fb/> Turn `id::Ancestors` into general-purpose `RevWalk`.
 - <csr-id-1df379ab0046887a330c0a670ad0414e79cfae7b/> remove `Permissions::git_dir` field entirely.
   It was meant to help dealing with bailing out if the git dir isn't
   fully trusted, but the way this was done was over-engineered especially
   since the read-only permission level wasn't implemented at all.
   
   That function is now performed by a new flag, the `bail_on_untrusted`
   which is off by default.
 - <csr-id-5ab81ece15ec802ad4328ce31304233bd25b2929/> rename `Repository::remote_ref()` to `::branch_remote_ref()`

### New Features (BREAKING)

 - <csr-id-e2aff28e818951785d933f4b55b2f1b882729cb6/> `Repository::rev_parse()` returns a `RevSpec`.
   This lays the foundation for actually handling rev-specs faithfully.
   Previous users should use `rev_parse().single()` to obtain a single
   object id which was the only supported usecase previously.

### Bug Fixes (BREAKING)

 - <csr-id-c68b125a46f666700cdbda6f8cd39a044f4feb1b/> Don't panic for `@{1}` in new repos; rename `Head::into_referent()` to `::try_into_referent()`
   The signature change will prevent such issues in the future as one
   cannot simply ignore new repositories.

## 0.20.0 (2022-07-22)

### New Features

 - <csr-id-1b765ec6ae70d1f4cc5a885b3c68d6f3335ba827/> respect `safe.directory`.
   In practice, this code will rarely be hit as it would require very
   strict settings that forbid any operation within a non-owned git
   directory.
 - <csr-id-840d9a3018d11146bb8e80fc92693c65eb534d91/> permissions for configuration.
   It provides fine-grained control over what sources to load.
 - <csr-id-657080829867d9dcb0c9b9cb6c1c8126c4df3783/> `gix-config` is now accessible in `git-repository::config`.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.
 - <csr-id-ebedd03e119aa5d46da07e577bfccad621eaecb5/> repository now initializes global configuration files and resolves includes
 - <csr-id-de8572ff2ced9422832e1ba433955c33f0994675/> resolve includes in local repository configuration
 - <csr-id-d5a48b82230b047434610550aacd2dc741b4b5f0/> `config::Snapshot::trusted_path()` to obtain trustworthy paths.
   We also apply trust-based config query during initialization to assure
   we don't use paths which aren't owned by the current user.
 - <csr-id-5f9bfa89ceb61f484be80575b0379bbf9d7a36b3/> `Repository::config_snapshot()` to access configuration values.
 - <csr-id-7f67b23b9462b805591b1fe5a8406f8d7404f372/> Use `gix-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`.
 - <csr-id-e263e13d312e41aa1481d104fa79ede509fbe1c5/> respect `core.logallrefupdates` configuration setting.

### Changed (BREAKING)

 - <csr-id-68f4bc2570d455c762da7e3d675b9b507cec69bb/> Make `SignatureRef<'_>` mandatory for editing reference changelogs.
   If defaults are desired, these can be set by the caller.
 - <csr-id-f932cea68ece997f711add3368db53aeb8cdf064/> `Repository::committer()` now returns an `Option`, see `::committer_or_default()` for a method that doesn't.
 - <csr-id-89a41bf2b37db29b9983b4e5492cfd67ed490b23/> remove local-time-support feature toggle.
   We treat local time as default feature without a lot of fuzz, and
   will eventually document that definitive support needs a compile
   time switch in the compiler (`--cfg unsound_local_offset` or something).
   
   One day it will perish. Failure is possible anyway and we will write
   code to deal with it while minimizing the amount of system time
   fetches when asking for the current local time.
 - <csr-id-6f4eea936d64fb9827277c160f989168e7b1dba2/> Associate `file::Metadata` with each `File`.
   This is the first step towards knowing more about the source of each
   value to filter them based on some properties.
   
   This breaks various methods handling the instantiation of configuration
   files as `file::Metadata` typically has to be provided by the caller
   now or be associated with each path to read configuration from.

### New Features (BREAKING)

 - <csr-id-d003c0f139d61e3bd998a0283a9c7af25a60db02/> Support for `lossy` load mode.
   There is a lot of breaking changes as `file::from_paths::Options` now
   became `file::init::Options`, and the same goes for the error type.
 - <csr-id-311d4b447daf8d4364670382a20901468748d34d/> change mostly internal uses of [u8] to BString/BStr

## 0.19.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.18.1 (2022-05-23)

### New Features

 - <csr-id-c78baecbb37fd92a0a86231810c9e35e9a4c21cd/> `Debug` for `Reference`.

## 0.18.0 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

## 0.17.0 (2022-05-18)

<csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/>
<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>
<csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/>

### New Features

 - <csr-id-45920da7c8c5618c6e7258de08dbd633a637d017/> Add `Repository::head_name()`.
   A convenient way to obtain the name of a head, if not detached.

### Bug Fixes

 - <csr-id-a1680b44ef568317465d2971da6e0930f9885530/> `Commit::describe()` now returns annotated tags before lightweight ones and prefers more recent ones as well
 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Other

 - <csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/> allow reading information about remote branch
 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> `path::discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absolutize' the path later (std::path::absolute()) is
   on the way for that.
 - <csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/> remove unused variant

### Changed (BREAKING)

 - <csr-id-80e8fd4a5944890f43f3d888b7a73bb26351b195/> integrate trust model into repository discovery
   That way it's possible to ignore repositories which effectively
   aren't owned by the current user, or to not ignore them (default)
   but assign tighter permissions to the repository.
 - <csr-id-2e39b0ede98826e6f85c56fef77ac65a5b7e7ac2/> `path::discover::existing()` -> `path::discover()`
 - <csr-id-38dfdcf80f9b7368ccaa10f4b78b2129849848d0/> remove `values::*Error` in favor of `value::parse::Error`.
   This makes it easier to work with errors in practice, we are either
   interested in the value that failed to parse to try something else
   or want a nice user message.
   
   Having one decode error type facilitates that.

### New Features (BREAKING)

 - <csr-id-32dc1829a5661f66396d109c8d0a8eaae6b1f532/> use `gix-credentials` in `gix-protocol`

## 0.16.0 (2022-04-05)

### New Features

 - <csr-id-47556f6815148ed960a727fd122f7162345544c3/> auto-calculation of a good hex-len, like what git does
   If the `core.abbrev` value isn't set or is set to `auto`.
 - <csr-id-654f4afb794a370b7cd9d9502ff6d0c3378ec417/> `Commit::describe()`
   A way to fluidly configure a `git describe` operation and run it.
   
   Along that, a new `Tag` top-level object was added as well to provide
   convenient access to otherwise lower-level objects. It's not strictly
   required for our implementation here but it's needed for a symmetric
   API.

## 0.15.0 (2022-04-03)

<csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/>
<csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/>
<csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/>

### New Features

 - <csr-id-1322dbf6827ea5cc1d71175afb669e01fb1242ef/> support for object replacement
   The Repository now respects replacement refs created by `git replace`
   and picks up environment variables for its configuration as well.
   
   Which environment variables are used is fully configurable.
 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-00578040a699e1939b3d3813616d3cc4e1d8669e/> `Repository::head_commit()`
   A shortcut to get to the commit much faster.
 - <csr-id-def80df2e165b74f4b053e4030f563902b7d34a4/> `ein tool estimate-hours` now supports mailmaps
 - <csr-id-f0d8a49587c08713350252e1701a45bb308b6f9d/> `Repository::head_id()`
   A long-needed shortcut.
 - <csr-id-d2388d8d80f379eccc9ee84ebe07acd67d154630/> `gix repository mailmap entries`
 - <csr-id-e3bc1b410409a9e27894a5cac48b06d8c3295e36/> unstable mailmap module
 - <csr-id-1be00cf9e00ce9428ffddb2c79b2373926069b13/> `Commit::short_id()`
 - <csr-id-c7dff9e8b695d298a3fb21f19f51752a885a5ce3/> in-manifest and in-lib documentation of feature toggles
 - <csr-id-9f5663ed83d83c7335b346313837d4cada9cd846/> `easy::Commit::time()` to access the committers time conveniently.
 - <csr-id-7c88b62e439af7a60ddb68fb6737cb3b1cebf00d/> easy::Head::name() to learn about the name of the HEAD ref
   It's mainly for completeness to provide people with with a `FullNameRef`
   of HEAD.
 - <csr-id-3b0913a2e6695e4e9e94341ef48d2ba3b4a518e6/> `easy::Head::peel_to_commit_in_place()`
   It allows to quickly get a commit from the head, something most people
   want when getting started with any kind of tool.
 - <csr-id-1c22d76c26464db4a185e19bb6c1f9a17fa19bc9/> `Repository::load_index()`
   This method makes the index of the default workspace available.

### Bug Fixes

 - <csr-id-c329dd75420f82d506fd415cd377f7df6c6ccbad/> Properly classify worktrees as non-bare, helps with `ein t find`
   They use git-files which point to the actual repository data.

### Changed (BREAKING)

 - <csr-id-a8b6589a7c645f323f95da6cb94321fc967e9b06/> Easier access to local and remote branches

### New Features (BREAKING)

 - <csr-id-8945d95f7fa88562d37ff67ac6e38bead73dd2df/> `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError`
 - <csr-id-813a3bea88cdbe1fd9b0a8070efeee2a44f7823e/> Let 'easy::Object::try_into_…()` return `try_into::Error`.
   That way, the typical usage of `try_into_commit()?` will not result
   in a strange error about `Object` not being convertible into some
   error. We think having a real error there is the least surprising.

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

### Other (BREAKING)

 - <csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/> `Id::prefix` -> `Id::shorten()`
   It's definitely more intuitive that way.

### Refactor (BREAKING)

 - <csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/> dissolve 'easy' module by moving everything one level up
 - <csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/> clarify different repository types much better

## 0.14.0 (2022-01-23)

<csr-id-7a91212631219e94b9454d2874b53f3ecc1db77e/>
<csr-id-b2cc0c63570d45de032d63e62d94c3344783440e/>
<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>

### New Features

 - <csr-id-667485e133ca29fcc6914a7142cf953564b5fce3/> Add `easy::Tree::traverse()` platform
 - <csr-id-8f650c089c88698483f778aa5c0070f606b94f09/> Add `easy::Commit` object
   It allows to more conveniently access commit information.
 - <csr-id-0ae2a8da010d848d98bef47ac923ae1d770091ff/> `easy::Oid::ancestors()` now supports `sorting()` and iteration by first commit only
   Especially the sorting is useful to avoid having to sort commits by
   hand after collecting them.
 - <csr-id-bc77534f9c385046f6c9adb994b1443307afda46/> Use GITOXIDE_OBJECT_CACHE_MEMORY to control how much object cache is used
   Note that this is mostly for debugging or quickly seeing if object
   caches help with certain operations.
   
   Ideally the implementation knows themselves and sets up caches
   accordingly, probably after trying it with these environment variables.

### Changed (BREAKING)

 - <csr-id-6e3a745dfada66a2fcac256dae0ac63959e74d08/> rename `easy::Object` methods returning `Ref` objects to have `ref` in their name
   That way, it's more clear that the `Ref` versions are low-level ones
   whereas the `into_` ones are higher-level ones that are part of the
   `easy` suite.
 - <csr-id-b6730979808ce28b98c65888a349f1e3d0ea1b9a/> Rename `OwnedObject` to `DetachedObject`
   The latter more clearly indicates what the difference is to
   `Object` (which is attached and carries a lifetime)
 - <csr-id-c4184f3c31ffc4597bd089e8140653906a6594d8/> Remove easy::borrow::Error entirely; support for multiple objects per handle
   This massive simplification finally allows any amounts of objects to be
   created while adding support for reusing their data buffers thanks
   to a simple free-list stored with the handle.
 - <csr-id-880b56426859306aa30038ff35e2ad14607e9e90/> rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef`
 - <csr-id-f9c0493460ab7c664aaa231ffcf7dfd56076c920/> use `gix_odb::Find*` traits in prelude, instead of `gix_pack::Find*`
   These are higher-level and generally more desirable.
   The Find traits in `gix-pack` are more useful internally when packs
   have to be handled directly, for example when generating packs.
 - <csr-id-83d7b31e7dd6d09eea79fc3c68620d099459132f/> rename easy::State to easy::Handle
   As the first step to remove the 'Easy' abstraction.
 - <csr-id-5e7aa1689f5d7ea5b510611a3ca0868828226291/> fully rely on OdbHandle in repository State
 - <csr-id-57de915886b76f80b3641def0ccf4fd79e334fc8/> Rename `Repository::odb` to` Repository::objects`
   This way it's more inline with `Repository::refs`.
 - <csr-id-93db4a5e70456d2c33ea010e3c86e5f26eb1bcc0/> remove Repository::refresh_object_database()
   With the linked DB this is simply not possible anymore and we expect
   these updates to happen automatically in future for greater convenience.
   
   For now, in order to refresh a repository, one has to reopen it.
 - <csr-id-580e96c1b2d9782a2e8cf9d1123f6d53a5376a3d/> Rename `Handle` to `Cache`
   Because this is exactly what it is effectively.
   Also add some basic instantiation for the new object store.
 - remove borrowing Repo as possible failure cause
   The `easy::Handle` is now a full (but shared) clone of the original
   Repository with additional thread-local state, hence there is no more
   need for a way to access the original repository.
 - remove Easy… abstraction in favor of Handle
   This great reduction of complexity allows for being multi-threading
   capable by default with the option to turn that off at compile time.
   
   All `to|into_easy…()` methods are removed in favor of `to_easy()`
   along with the removal of all `Easy` types in favor of the single
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - move gix_pack::data::Object to gix_object::Data, massively alter gix_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

## 0.13.0 (2021-11-29)

<csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/>
<csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/>

With changes to `gix-ref`, what follows is all the adjustments made to simplify the `gix` implementation.

### Changed (BREAKING)

 - <csr-id-5d498a33236391d8e456f267b1bf6af24de66f11/> file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively
   This way, it's possible to keep shared ownership of the packed buffer
   while allowing the exact same iterator machinery to work as before.
 - <csr-id-15d429bb50602363292453606902bdce5042d9a5/> file::Store::(try_)find(…, packed) was removed
   The packed buffer is now handled internally while loading it on demand.
   When compiled with `gix-features/parallel` the `file::Store` remains
   send and sync.
   
   The packed refs buffer is shared across clones and it's recommended
   to clone one `file::Store` instance per thread, each of which can
   use its own namespace.
 - <csr-id-95247322a8191edfa7fac9c5aa72b40239f3aa88/> move `gix_ref::file::WriteRefLog` to `gix_ref::store::WriteRefLog`

### Bug Fixes (BREAKING)

 - <csr-id-fc8e85cd71d4f16bc8daad0b790d875045faefff/> ref namespaces are now thread-local
   Previously these were shared in the shared Repo instance, which makes
   threaded applications impossible to remain deterministic across multiple
   connections.
   
   Now they are local to the thread, which allowed some methods to remove
   their Result<> as they cannot fail anymore, the reason for this being
   a breaking change.

### Other (BREAKING)

 - <csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/> Reference::logs() -> Reference::log_iter()
   The latter now returns a standard Platform to iterate over all
   reflog entries from oldest to newest or vice versa.

### Refactor (BREAKING)

 - <csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/> `file::Store::base` is now `file::Store::base()` and read-only
   That way, file databases can't be repositioned anymore, it's recommended
   to recreate it if that's desired.

## 0.12.0 (2021-11-16)

### New Features

 - <csr-id-b7aab9efd42975e8f2dcb5c97e51495996175702/> Allow `PartialNameRef` to be created from owned items

### Changed (BREAKING)

 - <csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/> Rename gix->ein and gixp->gix

## v0.11.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `gix-hash`.

## v0.10.0 (2021-10-15)

<csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/>
<csr-id-61793ff42f5c2f9ddf302901adea2dac6149eac8/>
<csr-id-0cd585e20a5abd323a34ec32d92fbd48531b3b18/>
<csr-id-89f15051763a03627f332c46beedfc53b8b9b15b/>
<csr-id-f644d0ede7a2e8d344a81c7003c3877eed64a6b0/>
<csr-id-ac3b9efb7b90958274ce55800959d930f8641115/>
<csr-id-03fe8a7ebd34608d725d4585da5c1630123762ec/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>
<csr-id-b209da29f361512ba757febf56bc1aca039f2a41/>
<csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/>
<csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/>
<csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/>
<csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/>
<csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/>
<csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/>
<csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/>
<csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/>
<csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/>
<csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/>
<csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/>
<csr-id-41afad3386461b658ee859225785b6de86d13cfb/>
<csr-id-f582439a3efe5c234f54c488792395e9de09a032/>
<csr-id-42080aefe3b286afb58235c1c22491579ab73919/>
<csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/>
<csr-id-e7c061b10c263001eb4abf03098d6694b770f828/>
<csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/>
<csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/>
<csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/>
<csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/>
<csr-id-06996e032b1e451a674395ebaca94434fac46f05/>
<csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/>
<csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/>
<csr-id-debe0094826f83839f907523715def929133fd58/>
<csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/>
<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

### New Features

 - <csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/> Make `gix_url::Url` available under `gix::Url`
 - <csr-id-80b8331092f4856f52afa1d85fa375ae688bdd28/> add easy::ext::ObjectAccessExt::tag(…) to create tag objects
   It's a quick sketch on how tag object creation could work.
   
   Note the duplication the method name using traits, which seems like a good solution
   to the problem of differentiating tag objects and tag references while
   keeping the method name short.
   
   Most will only ever need one, right?
   
   Even in my example that's not the case, so maybe we have to rename it.
 - <csr-id-0ebfeb614264ca06ab763189e55e6c016c9997af/> Make `gix_url::Url` available under `gix::Url`

### BREAKING Changes

 - Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.
 - Avoid duplicate module paths in 'tree' and 'commit'
 - rename ObjectIdExt::ancestors_iter() to *::ancestors()
 - rename `easy::Object::to_(commit|tag)_iter()`…
   …to  `easy::Object::try_to_(commit|tag)_iter()` for consistency.
 - rename `*::State` into `*::Platform`
 - various small API changes
 - move easy::head::peel::Error -> easy::head::peel::to_id::Error
 - rename path::is_git to path::is
 - rename easy::reference::log::State to easy::reference::Logs

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/> improved changelog…
   …akin to 'Keep a changelog'.

### Refactor

 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files
 - <csr-id-b209da29f361512ba757febf56bc1aca039f2a41/> use new gix_pack::cache::Object trait
 - <csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/> remove object cache impl which now lives in gix-pack

### Other

 - <csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/> :remote_url() is now optional
   Otherwise it wouldn't work on repos that don't have a remote set yet.
   Instead of failing, we don't create links.
 - <csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/> try to create persistent Easy iterator, but can't make it Send…
   …which is fair as it contains borrowed RefCells, which really would have
   to be owned to work for this, which would in turn require the Ancestor's
   struct to be kind of self-referential
 - <csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/> path::is
 - <csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/> path::discover
 - <csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/> top-level of 'path' module
 - <csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/> object_id
 - <csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/> repository
 - <csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/> ext::tree
 - <csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/> easy::object::peel
 - <csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/> easy::object::errors
 - <csr-id-41afad3386461b658ee859225785b6de86d13cfb/> a seemingly slow version of path lookup, but…
   …in debug mode it's faster than the fast path, despite doing more
   and being the same when it comes to searching path components.
 - <csr-id-f582439a3efe5c234f54c488792395e9de09a032/> easy::object, sans a few child-modules
 - <csr-id-42080aefe3b286afb58235c1c22491579ab73919/> update 'platform' information to reflect the current usage
 - <csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/> configure caches with env vars using `apply_environment()`
 - <csr-id-e7c061b10c263001eb4abf03098d6694b770f828/> refactor
 - <csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/> set package cache via RepositoryAccessExt
 - <csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/> Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size…
   …which can mean another considerable speed-up for many workloads, but
   usually needs some knowledge about the application, repos, and should
   thus be with the user.
 - <csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/> allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE
 - <csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/> prepare for configurable pack cache
 - <csr-id-06996e032b1e451a674395ebaca94434fac46f05/> object-cache to allow for a speed boost…
   …by avoiding duplicate accesses to hit the object database.
   However, the cost for the cache are relatively high and involve some
   memory copying, so hit rates of about 50% is certainly what is needed
   to get any speed boost at all.
 - <csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/> build commit history for later use in changelog generation
 - <csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/> Allow object access during commit ancestor traversal…
   …by getting only a temporary handle to the pack-cache. The cost of this
   should be negligible compared to the cost of object decoding.
 - <csr-id-debe0094826f83839f907523715def929133fd58/> sketch history acquisition
 - <csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/> add 'Head::peeled()' method

### Changed (BREAKING)

 - <csr-id-c3385cd144298eb9f06d7751d180e26da7b4d338/> `easy::Object::try_to_commit()` now returns `Result<CommitRef>`…
   …without the nested `Option`, folding the type mismatch into a specific
   `conversion::Error` instead.
 - <csr-id-e59f901f47fb0180211494a1591aed62b856406a/> rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()`
   This one also contains the first and probably only test for tag object
   creation.

## v0.9.1 (2021-09-10)

<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/> improved changelog…
   …akin to 'Keep a changelog'.

## v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport

### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

## v0.8.2 (2021-09-07)

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies

## v0.7.2 (2021-08-17)

## v0.7.1 (2021-08-13)

## v0.7.0 (2021-08-10)

## v0.6.0 (2021-05-28)

## v0.5.0 (2021-04-08)

## v0.4.0 (2020-09-12)

## v0.3.0 (2020-08-12)

## v0.1.0 (2020-07-12)

## 0.0.0 (2023-02-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix v0.0.0 ([`8bce6d5`](https://github.com/Byron/gitoxide/commit/8bce6d5cba12630bf4d12ed92f572a379d945329))
    - Add `gix` crate to reserve name ([`5104a78`](https://github.com/Byron/gitoxide/commit/5104a787127bf0b1f9b65f371b7c5b79f491e396))
</details>

