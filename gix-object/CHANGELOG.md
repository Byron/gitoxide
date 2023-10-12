# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.38.0 (2023-10-12)

### New Features

 - <csr-id-5bb55decbab55f0480165a756893b0af111e485c/> add `compute_stream_hash` method as lower-level way of hashing objects.

### Bug Fixes (BREAKING)

 - <csr-id-9283a9d2ec460c1380cf6c40d876f477cb552826/> `encode::loose_header()` now supports large objects even on 32 bit systems.
   Previously, larger than 4GB files wouldn't be supported, which causes problems when
   genrating hashes even when streaming data.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 8 calendar days.
 - 17 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs prior to release ([`1347a54`](https://github.com/Byron/gitoxide/commit/1347a54f84599d8f0aa935d6e64b16c2298d25cf))
    - Merge branch 'improvements' ([`429e7b2`](https://github.com/Byron/gitoxide/commit/429e7b25f93c8a7947db19bafa74babf199a1aa6))
    - Add `compute_stream_hash` method as lower-level way of hashing objects. ([`5bb55de`](https://github.com/Byron/gitoxide/commit/5bb55decbab55f0480165a756893b0af111e485c))
    - Fix docs ([`995bc84`](https://github.com/Byron/gitoxide/commit/995bc840664cbd4aeb7f95592e3125dee63bdcd4))
    - Merge branch 'reset' ([`b842691`](https://github.com/Byron/gitoxide/commit/b8426919a491dc3a7df01ee3f258fc0d8a3a327c))
    - `encode::loose_header()` now supports large objects even on 32 bit systems. ([`9283a9d`](https://github.com/Byron/gitoxide/commit/9283a9d2ec460c1380cf6c40d876f477cb552826))
</details>

## 0.37.0 (2023-09-24)

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

## 0.36.0 (2023-09-08)

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

## 0.35.0 (2023-08-22)

<csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/>
<csr-id-353b1a788a7c5a627ec73185f841ea4893a147a5/>
<csr-id-7649b185c93877e8c8b3cebedf685be8e94a0d9d/>

### Chore

 - <csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/> switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance.
   It's likely that over time, these parsers will get even faster due to improvements to `winnow`.
   Thanks, Ed Page, for single-handedly performing this transition.
 - <csr-id-353b1a788a7c5a627ec73185f841ea4893a147a5/> add benchmarks to avoid parsing performance regressions
 - <csr-id-7649b185c93877e8c8b3cebedf685be8e94a0d9d/> remove unused dependency: 'hex'

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 13 calendar days.
 - 15 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Just fmt ([`0d258f4`](https://github.com/Byron/gitoxide/commit/0d258f40afcd848509e2b0c7c264e9f346ed1726))
    - Switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance. ([`ef54aab`](https://github.com/Byron/gitoxide/commit/ef54aab9e5521add4154ee8d902d62612a9d8d4a))
    - Refactor and fixes ([`02587fc`](https://github.com/Byron/gitoxide/commit/02587fc879c54b2b3e62ffbe1ab4c29591ea0d80))
    - Upgrade `winnow` to latest patch release ([`8c41848`](https://github.com/Byron/gitoxide/commit/8c4184817e4e4364c34badc8ff0a71c6ae952efd))
    - Remove From<winnow> for (small) step towards stable API ([`be1f4b5`](https://github.com/Byron/gitoxide/commit/be1f4b5d2158fc73d926394d22d62bb597c5e6ce))
    - Make clippy happy ([`62f7bc6`](https://github.com/Byron/gitoxide/commit/62f7bc65c1eef812baa2ef2e5b406762e117b66a))
    - Switch off deprecated VerboseError ([`93fc441`](https://github.com/Byron/gitoxide/commit/93fc441c6c82086e125ace5bdda3b9403d271618))
    - Switch errors to StrContext ([`df226dd`](https://github.com/Byron/gitoxide/commit/df226dd31df2c591c6470ed70098202112e13dae))
    - Minor cleanup possible with 0.5 ([`a07590c`](https://github.com/Byron/gitoxide/commit/a07590cb46423cb0422c18b9fc04b153c0fd53b1))
    - Upgrade to Winnow 0.5 ([`3f8c91f`](https://github.com/Byron/gitoxide/commit/3f8c91fa463fbb53d54b2bf359e0dee7387afa00))
    - Simplify parsers ([`12f03db`](https://github.com/Byron/gitoxide/commit/12f03db6475b92f492f5a14bda472c139c3511e0))
    - Resolve 0.4 not-quite deprecations ([`f0cbf81`](https://github.com/Byron/gitoxide/commit/f0cbf81a346e087a622b0e2a6a37593861d0010f))
    - Resolve 0.4 deprecations ([`9ed7df0`](https://github.com/Byron/gitoxide/commit/9ed7df0a17deed08759dc29fc0089cdea100e433))
    - Upgrade to Winnow 0.4 ([`86ea47f`](https://github.com/Byron/gitoxide/commit/86ea47f28079c51f874b0d662867040b92f88d14))
    - Parse explicitly in prep for 0.4 ([`b3f0418`](https://github.com/Byron/gitoxide/commit/b3f041829881e881ad4eeeacaeea31064c523340))
    - Resolve remaining winnow 0.3 deprecations ([`fee441d`](https://github.com/Byron/gitoxide/commit/fee441da875d52b1a0cb557d2fa58cee9c29e16a))
    - Prefer Parser inherent parsers ([`b37a909`](https://github.com/Byron/gitoxide/commit/b37a909a5c344201a985262351e0fb67757572a4))
    - Prefer built-in Winnow parsers ([`ac0e81c`](https://github.com/Byron/gitoxide/commit/ac0e81c41f8c8a33ede9a0d8b7bffcd04bb97dc3))
    - Simplify winnow ErrMode construction ([`86d7fd1`](https://github.com/Byron/gitoxide/commit/86d7fd18487626d30f6d5478864819a3d7428085))
    - Switch gix to winnow 0.3 ([`ee75de1`](https://github.com/Byron/gitoxide/commit/ee75de1e6035305fc23bdef2522ae5081272ac82))
    - Add benchmarks to avoid parsing performance regressions ([`353b1a7`](https://github.com/Byron/gitoxide/commit/353b1a788a7c5a627ec73185f841ea4893a147a5))
    - Merge branch 'faster-hex' ([`4a4fa0f`](https://github.com/Byron/gitoxide/commit/4a4fa0fcdaa6e14b51d3f03f5d7c5b65042667bf))
    - Remove unused dependency: 'hex' ([`7649b18`](https://github.com/Byron/gitoxide/commit/7649b185c93877e8c8b3cebedf685be8e94a0d9d))
    - Merge branch 'extract-signatures' ([`b37affe`](https://github.com/Byron/gitoxide/commit/b37affefecfb30a94431cd21dae6659004ca6244))
    - Refactor ([`fb95ead`](https://github.com/Byron/gitoxide/commit/fb95eadb2da12c794d41b1f6212eba60ff3dd2dc))
    - Add commit signature extraction ([`2f9c0dd`](https://github.com/Byron/gitoxide/commit/2f9c0ddb7c2191789b4c0a5e8f2bf9253ac58606))
</details>

## 0.34.0 (2023-08-07)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
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
</details>

## 0.33.2 (2023-08-02)

<csr-id-7f7353e4525adc308e97b82d27bd301e48da6016/>

### Other

 - <csr-id-7f7353e4525adc308e97b82d27bd301e48da6016/> fix test for struct size for 32-bit architectures
   The size of gix_object::Data is 24 bytes only on 64-bit architectures,
   on 32-bit architectures it's exactly half that (12 bytes).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 11 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-actor v0.24.2, gix-object v0.33.2, gix-ref v0.33.3, gix-config v0.26.2, gix-prompt v0.5.5, gix-odb v0.50.2, gix-transport v0.34.2, gix-protocol v0.37.0, gix-worktree v0.23.1, gix v0.51.0, safety bump 3 crates ([`231ac1c`](https://github.com/Byron/gitoxide/commit/231ac1c6ad5ca9a84dbeb0dee14bfbf2fef1ae1e))
    - Prepare additional changelogs ([`db63815`](https://github.com/Byron/gitoxide/commit/db6381522395a0de047118e81df5cd3cbeb862b9))
    - Merge branch 'decathorpe/main' ([`422747d`](https://github.com/Byron/gitoxide/commit/422747d35724a56dade73c8d68b8ebcbaa4b9b25))
    - Prepare changelogs ([`e4d2890`](https://github.com/Byron/gitoxide/commit/e4d2890a85bf60e9cdb4016dddfab3c4dccbe75e))
    - Merge branch 'fixes-and-improvements' ([`f8b1f55`](https://github.com/Byron/gitoxide/commit/f8b1f553371f25b1bea6bce7cbb2ff1f01194856))
    - Fix test for struct size for 32-bit architectures ([`7f7353e`](https://github.com/Byron/gitoxide/commit/7f7353e4525adc308e97b82d27bd301e48da6016))
    - Add another very special commit for parsing tests ([`f852243`](https://github.com/Byron/gitoxide/commit/f852243831474a3fd311a0507d11fd43e0f6e4d3))
</details>

## 0.33.1 (2023-07-22)

### New Features

 - <csr-id-424d347bb2466ecdf8cfc9e0da779d604d1f9e4e/> `TreeRef::bisect_entry()` to correctly find an entry by name
 - <csr-id-437fedc59c2bad57beb3cf54e4c71a470bd22ee2/> `Copy` for `tree::EntryRef`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Prepare more changelogs ([`c4cc5f2`](https://github.com/Byron/gitoxide/commit/c4cc5f261d29f712a101033a18293a97a9d4ae85))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Merge branch 'improvements-for-crates-index' ([`7734736`](https://github.com/Byron/gitoxide/commit/7734736ce7e590b2ad60f3c1d8289854931d155c))
    - `TreeRef::bisect_entry()` to correctly find an entry by name ([`424d347`](https://github.com/Byron/gitoxide/commit/424d347bb2466ecdf8cfc9e0da779d604d1f9e4e))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Merge branch 'improvements-for-crates-index' ([`3f914e8`](https://github.com/Byron/gitoxide/commit/3f914e8840afc59441c3c463bdc89c53136d583e))
    - `Copy` for `tree::EntryRef` ([`437fedc`](https://github.com/Byron/gitoxide/commit/437fedc59c2bad57beb3cf54e4c71a470bd22ee2))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.33.0 (2023-07-19)

### New Features

 - <csr-id-695de5679904776798b4d8b9dbee771306ff39ba/> Add classifier methods to `Kind` for easier object classification.
   This allows to avoid having to think about where to right type lives
   for comparisons or matches in simple cases that are about one specific object type.
 - <csr-id-437fedc59c2bad57beb3cf54e4c71a470bd22ee2/> `Copy` for `tree::EntryRef`
 - <csr-id-424d347bb2466ecdf8cfc9e0da779d604d1f9e4e/> `TreeRef::bisect_entry()` to correctly find an entry by name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
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
    - Merge branch 'adjustments-for-crates-index' ([`b82868d`](https://github.com/Byron/gitoxide/commit/b82868d5688d8d4849c47ed3d209a96ee59e69b3))
    - Add classifier methods to `Kind` for easier object classification. ([`695de56`](https://github.com/Byron/gitoxide/commit/695de5679904776798b4d8b9dbee771306ff39ba))
</details>

## 0.32.0 (2023-06-29)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Prepare changelogs prior to release ([`00f96fb`](https://github.com/Byron/gitoxide/commit/00f96fb3110a8f81a1bd0d74c757c15b8773c6f6))
    - Merge branch 'i64-times' ([`b407461`](https://github.com/Byron/gitoxide/commit/b407461d8991db67a5bdb2ab13f518f78a85ed40))
    - Add a test to see what happens if negative dates are used in commits ([`57a5cd1`](https://github.com/Byron/gitoxide/commit/57a5cd1ca2f8153568c366cd1709be7d4ebec972))
</details>

## 0.31.0 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 11 calendar days.
 - 15 days passed between releases.
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
    - Merge branch 'future-dates' ([`8d2e6a9`](https://github.com/Byron/gitoxide/commit/8d2e6a91ac92a033e9e3daad5cffa90263075536))
    - Adapt to changes in `gix-actor` ([`4a80e86`](https://github.com/Byron/gitoxide/commit/4a80e868f9530896616e649838e9be64b6d10036))
    - Adapt to changes in `gix-date` ([`d575336`](https://github.com/Byron/gitoxide/commit/d575336c26e6026e463cd06d88266bb2bdd3e162))
</details>

## 0.30.0 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 12 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - `just fmt` ([`ffc1276`](https://github.com/Byron/gitoxide/commit/ffc1276e0c991ac33ce842f5dca0b45ac69680c0))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge pull request #878 from blinxen/main ([`67da689`](https://github.com/Byron/gitoxide/commit/67da6894c8d8a24b982c732a1753a3e0a3300cc3))
    - Include missing changelog file in some crates ([`0269eed`](https://github.com/Byron/gitoxide/commit/0269eedc08c21589b5381d9b7d7fcc7004160bf8))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Minor fixes ([`89a8cfe`](https://github.com/Byron/gitoxide/commit/89a8cfe40e5c3a9d4a4181fa055e3f4a529a8081))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Auto-fix as many 'range-plus-one' lints as possible ([`4795fcf`](https://github.com/Byron/gitoxide/commit/4795fcf6adb06b792592a1b11a3f071e9d47b1ac))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include custom clippy settings ([`b057500`](https://github.com/Byron/gitoxide/commit/b057500dd3e6b75be3ebcd258cda0b946bedd9e1))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
</details>

## 0.29.2 (2023-05-11)

### Bug Fixes

 - <csr-id-ca37915fbb0c43c7f01ae600d1da63d2f21eeb63/> sorting of tree entries is now according to specificiation.
   This means tree entries are compared as if they had a / appended to it.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 15 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.29.2 ([`4f879bf`](https://github.com/Byron/gitoxide/commit/4f879bf35653bdc8f9729d524c6e8e1fb3c6886b))
    - Merge branch 'fix-848' ([`e3b146f`](https://github.com/Byron/gitoxide/commit/e3b146ffdd6a1150e6deb6ddd267b81fd3c217b2))
    - Sorting of tree entries is now according to specificiation. ([`ca37915`](https://github.com/Byron/gitoxide/commit/ca37915fbb0c43c7f01ae600d1da63d2f21eeb63))
    - Refactor ([`d92a588`](https://github.com/Byron/gitoxide/commit/d92a588249f3f507f0b3fc05543f819c51e49c5a))
</details>

## 0.29.1 (2023-04-26)

### New Features

 - <csr-id-ebf3fd49ff2a53c99f818f4e87fbfed57db696bb/> add `encode::compute_hash()` for an easy way to obtain object hashes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'fix-819' ([`69faad0`](https://github.com/Byron/gitoxide/commit/69faad0d7cc100de54d757d42acc152a22edc022))
    - Add `encode::compute_hash()` for an easy way to obtain object hashes. ([`ebf3fd4`](https://github.com/Byron/gitoxide/commit/ebf3fd49ff2a53c99f818f4e87fbfed57db696bb))
</details>

## 0.29.0 (2023-04-19)

### New Features (BREAKING)

 - <csr-id-b83ee366a3c65c717beb587ad809268f1c54b8ad/> Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability.
   With it it's possible to not automatically declare all optional dependencies externally visible
   features, and thus re-use feature names that oterwise are also a crate name.
   
   Previously I thought that `serde1` is for future-proofing and supporting multiple serde versions
   at the same time. However, it's most definitely a burden I wouldn't want anyway, so using
   `serde` seems to be the way to go into the future.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 45 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#814](https://github.com/Byron/gitoxide/issues/814)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#814](https://github.com/Byron/gitoxide/issues/814)**
    - Rename `serde1` cargo feature to `serde` and use the weak-deps cargo capability. ([`b83ee36`](https://github.com/Byron/gitoxide/commit/b83ee366a3c65c717beb587ad809268f1c54b8ad))
 * **Uncategorized**
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`8dbd0a6`](https://github.com/Byron/gitoxide/commit/8dbd0a60557a85acfa231800a058cbac0271a8cf))
    - Prepare changelog prior to release ([`7f06458`](https://github.com/Byron/gitoxide/commit/7f064583bd0e1b078df89a7750f5a25deb70f516))
    - Merge branch 'main' into dev ([`cdef398`](https://github.com/Byron/gitoxide/commit/cdef398c4a3bd01baf0be2c27a3f77a400172b0d))
    - Rename the serde1 feature to serde ([`19338d9`](https://github.com/Byron/gitoxide/commit/19338d934b6712b7d6bd3fa3b2e4189bf7e6c8a1))
    - Release gix-hash v0.10.4, gix-hashtable v0.1.3 ([`b574a39`](https://github.com/Byron/gitoxide/commit/b574a3904203762a6b9e475e16a7c358d7616599))
</details>

## 0.28.0 (2023-03-04)

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

## 0.27.0 (2023-03-01)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.27.0, gix-actor v0.18.0, gix-quote v0.4.3, gix-attributes v0.9.0, gix-object v0.27.0, gix-ref v0.25.0, gix-config v0.17.0, gix-url v0.14.0, gix-credentials v0.10.0, gix-diff v0.27.0, gix-discover v0.14.0, gix-hashtable v0.1.2, gix-bitmap v0.2.2, gix-traverse v0.23.0, gix-index v0.13.0, gix-mailmap v0.10.0, gix-pack v0.31.0, gix-odb v0.41.0, gix-transport v0.26.0, gix-protocol v0.27.0, gix-revision v0.11.0, gix-refspec v0.8.0, gix-worktree v0.13.0, gix v0.38.0 ([`e6cc618`](https://github.com/Byron/gitoxide/commit/e6cc6184a7a49dbc2503c1c1bdd3688ca5cec5fe))
    - Adjust manifests prior to release ([`addd789`](https://github.com/Byron/gitoxide/commit/addd78958fdd1e54eb702854e96079539d01965a))
    - Prepare changelogs prior to release ([`94c99c7`](https://github.com/Byron/gitoxide/commit/94c99c71520f33269cc8dbc26f82a74747cc7e16))
</details>

## 0.26.4 (2023-02-24)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.4, gix-diff v0.26.3, gix v0.37.2, gix-commitgraph v0.13.1, gitoxide-core v0.25.0, gitoxide v0.23.0 ([`9982949`](https://github.com/Byron/gitoxide/commit/9982949cab401501d5ce3cba4e2ba900bc249c53))
    - Prepare changelog for release ([`13a1ec1`](https://github.com/Byron/gitoxide/commit/13a1ec1803d677c2e94f3ea0461118c2426f8071))
    - Merge branch 'rename-tracking' ([`550144a`](https://github.com/Byron/gitoxide/commit/550144a5fd37d501d86f4b1c4db2948d951d1c93))
    - Document why EntryRef has a different comparison method. ([`c9ba1f7`](https://github.com/Byron/gitoxide/commit/c9ba1f7e5bd4a24cd1e32b126955b03957f01845))
</details>

## 0.26.3 (2023-02-20)

### New Features

 - <csr-id-c882ff0b2cbf3484520432593968716493ffdaa1/> add `tree::EntryMode::is_blob_or_link()`.
   Returns true if the entry points to a blob or a symlink.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-object v0.26.3, gix-diff v0.26.2, gix-traverse v0.22.2, gix v0.37.0, safety bump 3 crates ([`8b3e42f`](https://github.com/Byron/gitoxide/commit/8b3e42f69fe97fe5083eb845c323f10d7ac087b2))
    - Merge branch 'rename-tracking' ([`35415c5`](https://github.com/Byron/gitoxide/commit/35415c5061bf5ea90a04db80d06ac3622d0b0f1a))
    - Add `tree::EntryMode::is_blob_or_link()`. ([`c882ff0`](https://github.com/Byron/gitoxide/commit/c882ff0b2cbf3484520432593968716493ffdaa1))
</details>

## 0.26.2 (2023-02-20)

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

## 0.26.1 (2023-02-17)

<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-598698b88c194bc0e6ef69539f9fa7246ebfab70/>
<csr-id-9c8a36f35b1e8dad66befb102e5a8670b4a6c2b2/>
<csr-id-5d57c1f7e3b9a84f7b46a4378015572155f3104b/>
<csr-id-3b9d66c932075feb08cdf2967f7698daef9fd3ff/>
<csr-id-ec3be19c8d007565b814b4757f17811ec0e9de2c/>
<csr-id-899c57927ce4ba2e1b5d8182f9e731c2a9094cba/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-322b2901382192d3b4ac0fbae32381a9abb49fa9/>

### Refactor (BREAKING)

 - <csr-id-2290d006705ff47ad780b009fe58ee422b3285af/> move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.
 - <csr-id-598698b88c194bc0e6ef69539f9fa7246ebfab70/> move loose header manipulation from git-pack to git-object

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

### Chore (BREAKING)

 - <csr-id-9c8a36f35b1e8dad66befb102e5a8670b4a6c2b2/> remove quick-error in favor of thiserror
   Some errors change shape which makes this a breaking change.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-fda2a8d2f5f8b7d80b4cc0177d08d6a061f1b8f1/> rename `commit::ref_iter::Token::into_id()` to `*::try_into_id()`
   This makes the method more consistent.

### Refactor

 - <csr-id-5d57c1f7e3b9a84f7b46a4378015572155f3104b/> Use borrowed::Id in trees for full type safety
 - <csr-id-3b9d66c932075feb08cdf2967f7698daef9fd3ff/> make reusing round-trip code easier
 - <csr-id-ec3be19c8d007565b814b4757f17811ec0e9de2c/> Prefer integration level tests, but use unit-tests where appropriate

### Performance

 - <csr-id-83d591d536d1a43e8523082824ec0b95cca55d34/> parse entry mode into number instead of comparing it to byte strings
 - <csr-id-f9232acf8e52f8cd95520d122469e136eb07b39f/> ObjectID specific hashers, using the fact that object ids are hashes

### Other

 - <csr-id-899c57927ce4ba2e1b5d8182f9e731c2a9094cba/> describe variants

### Bug Fixes

 - <csr-id-2df3c7395a791c751b22187a936facbce9f3f168/> don't panic if broken loose objects are encountered.
   Previously a loose object could advertise any object size and cause a
   panic if it was shorter than advertised.
   Now an error will be returned.
 - <csr-id-761695cd10222eccb720812fa41d5295d506fce9/> improve error messages related to object decoding

### New Features

 - <csr-id-14d613303b8663daac1ab1eed8413266594050c3/> `Object::try_into_blob_ref()` as alternative to consuming `*::try_into_blob()`
 - <csr-id-5095df949d30b57b09bd68355743d255a897365f/> add `tree::EntryMode::is_blob()` to quickly determine if an entry is a blob.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-4f4bba23a13f11050e35887da004e48148d68aea/> `tree::EntryMode::as_str()` to display itself as a string.
 - <csr-id-0d22ab459ce14bc57549270142595d8ebd98ea41/> `TagRefIter::tagger()`.
   Additionally ergonomics have been improved as the iterator is now
   `Copy`, similarly to the other iterators.
 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-8f1b42b193098f6e116b3522a3df08d5d1b42a0a/> `CommitRef::time()`
   A shortcut to allow for fluid retrival of the `committer.time`.
 - <csr-id-c51a9252f8a038bf995d50cdbfc8356362020bbc/> `CommitRefIter::(author|committer)()`, better usability
   The methods returning an iterator are now consuming, which allows
   them to be nested by callers.
 - <csr-id-91065cd790f559f4a661ecd9c2d2f649922682e3/> `CommitRefIter::parent_ids()`
   Allocation-free parent retrieval.
 - <csr-id-5c573bf9abfc10227d5a900100e84d7f8f6c0504/> in-manifest and in-lib feature documentation
 - <csr-id-eb36a3dda83a46ad59078a904f4e277f298a24e1/> Add sorting mode to ancestor traversal  #270
 - <csr-id-a0bb6529216160d14e67b97206a51140099fd245/> add `Data` object
   This is typed data baked by a slice for conversion into parsed ObjectRef's
   for example.
   
   This is usually the result of a `Find` operation on an object database.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-322b2901382192d3b4ac0fbae32381a9abb49fa9/> upgrade dependencies

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 574 commits contributed to the release over the course of 977 calendar days.
 - 30 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 20 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#222](https://github.com/Byron/gitoxide/issues/222), [#250](https://github.com/Byron/gitoxide/issues/250), [#254](https://github.com/Byron/gitoxide/issues/254), [#259](https://github.com/Byron/gitoxide/issues/259), [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#293](https://github.com/Byron/gitoxide/issues/293), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#364](https://github.com/Byron/gitoxide/issues/364), [#389](https://github.com/Byron/gitoxide/issues/389), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#63](https://github.com/Byron/gitoxide/issues/63), [#67](https://github.com/Byron/gitoxide/issues/67), [#691](https://github.com/Byron/gitoxide/issues/691), [#79](https://github.com/Byron/gitoxide/issues/79), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 17 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - Deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - Regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - Greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Feat: `BodyRef::without_trailer()` for more obvious access than `*body` or `body.as_ref()` ([`f0ea526`](https://github.com/Byron/gitoxide/commit/f0ea526775793c9104e4ae27dd5d92b5a1202c5f))
    - Refactor ([`ef3fc6d`](https://github.com/Byron/gitoxide/commit/ef3fc6d92c1d751d0032e072834f41d37cbb9200))
    - Feat: `CommitRef::message_trailers()` as shortcut… ([`5324391`](https://github.com/Byron/gitoxide/commit/5324391f581c5ad2c09604f0beeac7df852bfb33))
    - More tests for trailers iterator ([`c3b0161`](https://github.com/Byron/gitoxide/commit/c3b0161eb76aaf806a7d82232ec7ac1a304052a3))
    - Feat: `BodyRef::trailers()` allows iterating trailer tokens and values ([`175e1cb`](https://github.com/Byron/gitoxide/commit/175e1cbdfebfc6f01784fffdaf0859cd6c23377e))
    - Some tests and sketch for BodyRef parsing ([`3953c24`](https://github.com/Byron/gitoxide/commit/3953c245461941c636ce5d755e6a469f7fa3eabe))
    - Feat: CommitRef::summary() and `MessageRef::body()` methods ([`1714d05`](https://github.com/Byron/gitoxide/commit/1714d05df812aa373da485492b342e58e9e7c17d))
    - Refactor ([`7055dc8`](https://github.com/Byron/gitoxide/commit/7055dc81e9db448da89ab2ee0ba2ffe07cd00cc2))
    - Another test for footer separation, simple version ([`b439186`](https://github.com/Byron/gitoxide/commit/b4391862b67a09330476a82d520bfc3a698a4fbe))
    - Return to safety ([`35313b9`](https://github.com/Byron/gitoxide/commit/35313b9f7c92edd1afdbe22d1f592baabc0abc9c))
    - Omg nom parsing works… ([`cd11704`](https://github.com/Byron/gitoxide/commit/cd11704dd0d469cd23d7ee6905d6219b26ba4563))
    - FAIL: not really successful to continue down the 'fold' road ([`d9afc22`](https://github.com/Byron/gitoxide/commit/d9afc22f161fb60195571be09d2d816d67050575))
    - Three tests failing with nom ([`13646e8`](https://github.com/Byron/gitoxide/commit/13646e8dfe97d8503d0cef935c4c303f82271aa4))
    - Revert " FAIL: try to use nom-way of the previous body parsing…" ([`d1e6f62`](https://github.com/Byron/gitoxide/commit/d1e6f621c2898ad9f03b2ee712019e6a10b44035))
    - FAIL: try to use nom-way of the previous body parsing… ([`909f668`](https://github.com/Byron/gitoxide/commit/909f6682ac1de6be0eb8b66015b3f250daca17cd))
    - Sketch nom version of the message parser… ([`1ec47de`](https://github.com/Byron/gitoxide/commit/1ec47ded5793cac1f2633262d59bfbae4a0e14be))
    - Fix build ([`d0a956f`](https://github.com/Byron/gitoxide/commit/d0a956fdb5a822dbd116792bfbe70d1532a95ec9))
    - Refactor!: Use git_object::commit::MessageRef::summary()… ([`13e7c3a`](https://github.com/Byron/gitoxide/commit/13e7c3ad5e079fe778d07d115c9e41c4c6eb038f))
    - Feat(commit): A summary for commit messages suitable for logs ([`cd3fc99`](https://github.com/Byron/gitoxide/commit/cd3fc99968baa827302aa9c4f5d181bc9c4f9084))
    - More message parsing tests with windows line separators ([`001e8c2`](https://github.com/Byron/gitoxide/commit/001e8c2a4ede5fc025572a4c39a771cc854f8b18))
    - A manual message parse impl and more tests ([`f4b8a0d`](https://github.com/Byron/gitoxide/commit/f4b8a0da787f9a16eebd2a36b342f5a2a66edabd))
    - More message parsing tests, now with legit failure… ([`625be8d`](https://github.com/Byron/gitoxide/commit/625be8dbd4204ea1a7131ade9b17f63dcc7e30d7))
    - Feat(commit): Add `message()` method and `MessageRef` type… ([`6150b2d`](https://github.com/Byron/gitoxide/commit/6150b2db18034d5912029deac51d1affb38ae7b2))
 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - Update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - Stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **[#250](https://github.com/Byron/gitoxide/issues/250)**
    - Move loose header manipulation from git-pack to git-object ([`598698b`](https://github.com/Byron/gitoxide/commit/598698b88c194bc0e6ef69539f9fa7246ebfab70))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - Btree/hashmap free lookup of packs in store, keeping things more bundled ([`a88981b`](https://github.com/Byron/gitoxide/commit/a88981b6f38b86624588f0c8ff200d17f38d0263))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - Upgrade dependencies ([`322b290`](https://github.com/Byron/gitoxide/commit/322b2901382192d3b4ac0fbae32381a9abb49fa9))
    - Fix docs ([`1bb4253`](https://github.com/Byron/gitoxide/commit/1bb425347e4b502e1c048908cd5f3641d2b16896))
    - Move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
    - Add `Data` object ([`a0bb652`](https://github.com/Byron/gitoxide/commit/a0bb6529216160d14e67b97206a51140099fd245))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Adapt to changes in git-hash ([`5eb0230`](https://github.com/Byron/gitoxide/commit/5eb0230b58c25c0aa744eee0bd878dd91410dbe1))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Fix tree-entry-ordering implementation ([`ea169a6`](https://github.com/Byron/gitoxide/commit/ea169a6dbbba30d9464570cb86e5c990fcaf9ae8))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - In-manifest and in-lib feature documentation ([`5c573bf`](https://github.com/Byron/gitoxide/commit/5c573bf9abfc10227d5a900100e84d7f8f6c0504))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Support for trimming of whitespace around name and email ([`a39bf71`](https://github.com/Byron/gitoxide/commit/a39bf71531ee0a6c8db082758d3212c805ce2bf0))
    - `CommitRef::time()` ([`8f1b42b`](https://github.com/Byron/gitoxide/commit/8f1b42b193098f6e116b3522a3df08d5d1b42a0a))
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
    - More speedy access to author/committer ([`6129607`](https://github.com/Byron/gitoxide/commit/61296077cebaaf2eb939fa6082121304bc6cf39b))
    - `CommitRefIter::(author|committer)()`, better usability ([`c51a925`](https://github.com/Byron/gitoxide/commit/c51a9252f8a038bf995d50cdbfc8356362020bbc))
    - `CommitRefIter::parent_ids()` ([`91065cd`](https://github.com/Byron/gitoxide/commit/91065cd790f559f4a661ecd9c2d2f649922682e3))
 * **[#389](https://github.com/Byron/gitoxide/issues/389)**
    - `TagRefIter::tagger()`. ([`0d22ab4`](https://github.com/Byron/gitoxide/commit/0d22ab459ce14bc57549270142595d8ebd98ea41))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - `tree::EntryMode::as_str()` to display itself as a string. ([`4f4bba2`](https://github.com/Byron/gitoxide/commit/4f4bba23a13f11050e35887da004e48148d68aea))
    - Improve error messages related to object decoding ([`761695c`](https://github.com/Byron/gitoxide/commit/761695cd10222eccb720812fa41d5295d506fce9))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - Fix docs ([`dad9cbe`](https://github.com/Byron/gitoxide/commit/dad9cbeb853c0cc5128360b05c04b5a3da7ec75e))
    - Remove quick-error in favor of thiserror ([`9c8a36f`](https://github.com/Byron/gitoxide/commit/9c8a36f35b1e8dad66befb102e5a8670b4a6c2b2))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com/Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - Refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com/Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com/Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
    - Remove re-export of git_object::borrowed::Id ([`a3f2816`](https://github.com/Byron/gitoxide/commit/a3f28169c1268c1129852f279631d5a7f7540cdf))
    - Move git-hash::owned::Id into git-hash::Id ([`fdbe704`](https://github.com/Byron/gitoxide/commit/fdbe704b6c9ace2b8f629f681a0580b24749a238))
    - Rename `git_hash::*::Digest` to `Id` ([`188d90a`](https://github.com/Byron/gitoxide/commit/188d90ad463d342d715af701b03f0ed392c977fc))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - Describe variants ([`899c579`](https://github.com/Byron/gitoxide/commit/899c57927ce4ba2e1b5d8182f9e731c2a9094cba))
    - Parse entry mode into number instead of comparing it to byte strings ([`83d591d`](https://github.com/Byron/gitoxide/commit/83d591d536d1a43e8523082824ec0b95cca55d34))
    - ObjectID specific hashers, using the fact that object ids are hashes ([`f9232ac`](https://github.com/Byron/gitoxide/commit/f9232acf8e52f8cd95520d122469e136eb07b39f))
    - Tree parsing now probably is twice as fast… ([`d1e2b89`](https://github.com/Byron/gitoxide/commit/d1e2b8910b454dd798be8a9a43871f0b0644d503))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#79](https://github.com/Byron/gitoxide/issues/79)**
    - Refactor; add test for empty tree iteration ([`6340296`](https://github.com/Byron/gitoxide/commit/634029682da374f912068f5c8d5ec79d4837f7ea))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - Prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Fix git-object tests ([`fb92865`](https://github.com/Byron/gitoxide/commit/fb92865fec8c76b710c432b9e2e08ff8e9538fcd))
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
    - Rename `git-object` to `gix-object` ([`2320358`](https://github.com/Byron/gitoxide/commit/23203589e8c1df2e0eae9d9c2e75b0d0b24fd9b5))
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
    - Optimize usage of `hex_to_id()` ([`6fa950d`](https://github.com/Byron/gitoxide/commit/6fa950d0ab1991a5577c06385169be1b390dd88a))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Prepare changelogs prior to release ([`d679f5b`](https://github.com/Byron/gitoxide/commit/d679f5b6f018633e858d3ebbdaf1cd5098bbc5e7))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'pierrechevalier83/main' ([`a5b1d73`](https://github.com/Byron/gitoxide/commit/a5b1d738d23d0a343bee1b72bcb72250b5fdae11))
    - Thanks clippy ([`0a6d888`](https://github.com/Byron/gitoxide/commit/0a6d8882265eda833f4bf0252dc2c656aa05ca6f))
    - Restore original representation of `Tag` at the cost of some duplication ([`dd0a23d`](https://github.com/Byron/gitoxide/commit/dd0a23d710be0eb6c7ea7f883aeb1400bcbc0709))
    - Unify pre-allocation of `parents` in `Commit` to match typical single-ancestor chains ([`7242d1f`](https://github.com/Byron/gitoxide/commit/7242d1f4acb4ac0d2585295c11f36436a1215a7a))
    - Refactor ([`c02a6bd`](https://github.com/Byron/gitoxide/commit/c02a6bdcc3669a48cd4b5b640280701fd089575d))
    - [refactor] Deduplicate Tag and TagRef ([`6003fa2`](https://github.com/Byron/gitoxide/commit/6003fa22085b5031565c51b2b5a0a9feb1579fb0))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'pierrechevalier83/main' ([`f0dfa4c`](https://github.com/Byron/gitoxide/commit/f0dfa4c17be4d4a15fb2117bfc47b2fea1bc48fb))
    - [git-object] Encode empty tags like git does ([`3bd3380`](https://github.com/Byron/gitoxide/commit/3bd33802e072b06d278b93eeae8cfaed19795d6b))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Adapt in-memory size check to Rust 1.65 and below ([`1919e8e`](https://github.com/Byron/gitoxide/commit/1919e8ec2f6bca8237a0356972b86f28c18da908))
    - `Object::try_into_blob_ref()` as alternative to consuming `*::try_into_blob()` ([`14d6133`](https://github.com/Byron/gitoxide/commit/14d613303b8663daac1ab1eed8413266594050c3))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'main' into fetch-pack ([`93917cb`](https://github.com/Byron/gitoxide/commit/93917cb6ecbb30daf3d20bb5a7c65e12211f084f))
    - Merge branch 'fix-loose-parsing' ([`1dc2a5d`](https://github.com/Byron/gitoxide/commit/1dc2a5db9efa2cfb90a8b9fbeaca5200d35daed9))
    - Don't panic if broken loose objects are encountered. ([`2df3c73`](https://github.com/Byron/gitoxide/commit/2df3c7395a791c751b22187a936facbce9f3f168))
    - Don't panic on too short loose object ([`c89c7ff`](https://github.com/Byron/gitoxide/commit/c89c7fface26660d1855915b871dbf758150ab6f))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'main' into fix-odb-race ([`30712dc`](https://github.com/Byron/gitoxide/commit/30712dc9ab952d29c528ec58bcc467b425489304))
    - Add `tree::EntryMode::is_blob()` to quickly determine if an entry is a blob. ([`5095df9`](https://github.com/Byron/gitoxide/commit/5095df949d30b57b09bd68355743d255a897365f))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Refactor ([`867fc20`](https://github.com/Byron/gitoxide/commit/867fc208e8542668adcc81f8c03dd4957e956ef7))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Erge branch 'fix-describe' ([`56d7ad7`](https://github.com/Byron/gitoxide/commit/56d7ad7a2e7994545581ad5955c25feb9cefdf4e))
    - Release git-actor v0.9.0, git-object v0.18.0 ([`ef9242b`](https://github.com/Byron/gitoxide/commit/ef9242bdb35c02afc36af7c59073d78091fbf504))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Thanks clippy ([`3079e11`](https://github.com/Byron/gitoxide/commit/3079e114dc1d2552e023aa793dc10c28258b34da))
    - Merge branch 'main' into mailmap ([`b2df941`](https://github.com/Byron/gitoxide/commit/b2df941feaf5ae9fa170fa49270189f3527f2eab))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - Adapt to breaking changes in git-actor ([`40c48c3`](https://github.com/Byron/gitoxide/commit/40c48c390eb796b427ebd516dde92e9538ce5fb7))
    - Upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Release git-actor v0.8.1 ([`08fe550`](https://github.com/Byron/gitoxide/commit/08fe5508472f2eb209db8a5fc4e4942a9d7db93d))
    - Release git-object v0.17.1, git-pack v0.16.1 ([`e959af8`](https://github.com/Byron/gitoxide/commit/e959af83fa92e8ed87edae6e2d1c6a797964c056))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Update changelogs prior to git-pack release ([`b7e3a4a`](https://github.com/Byron/gitoxide/commit/b7e3a4afdd6417a38aadad35c7f584617e7b47fa))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Merge branch 'oknozor-feat/traversal-sort-by-committer-date' ([`6add377`](https://github.com/Byron/gitoxide/commit/6add3773c64a9155c236a36bd002099c218882eb))
    - Add sorting mode to ancestor traversal  #270 ([`eb36a3d`](https://github.com/Byron/gitoxide/commit/eb36a3dda83a46ad59078a904f4e277f298a24e1))
    - Rename `commit::ref_iter::Token::into_id()` to `*::try_into_id()` ([`fda2a8d`](https://github.com/Byron/gitoxide/commit/fda2a8d2f5f8b7d80b4cc0177d08d6a061f1b8f1))
    - Thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Merge branch 'pack-consistency' ([`5982406`](https://github.com/Byron/gitoxide/commit/5982406b4e1b26fd383d9ec21a3cf652ec8ab25f))
    - Merge branch 'git-loose-objects' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-git-loose-objects ([`ee737cd`](https://github.com/Byron/gitoxide/commit/ee737cd237ad70bf9f2c5e0d3e4557909e495bca))
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Adjust changelogs prior to git-pack release ([`ac8015d`](https://github.com/Byron/gitoxide/commit/ac8015de710142c2bedd0e4188e872e0cf1ceccc))
    - Move "loose object header" ser/de to git-object ([`3d1565a`](https://github.com/Byron/gitoxide/commit/3d1565acfc336baf6487edccefd72d0226141a08))
    - Merge branch 'header-field-multi-improve' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-header-field-multi-improve ([`d88e377`](https://github.com/Byron/gitoxide/commit/d88e377c21e566bf86c274d5e87eff06100698b9))
    - Improve error handling of encode::header_field_multi_line & simplify ([`bab9fb5`](https://github.com/Byron/gitoxide/commit/bab9fb567e47bb88d27b36f6ffa95c62c14ed80a))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Thanks clippy ([`d78d382`](https://github.com/Byron/gitoxide/commit/d78d3828c7f80963c0b8803cb64e0ae5e08d0ba3))
    - Thanks clippy ([`4ea1126`](https://github.com/Byron/gitoxide/commit/4ea11264296063278977c5539e2d68367475464a))
    - Thanks clippy ([`e56af5a`](https://github.com/Byron/gitoxide/commit/e56af5a0846652f177a88771d495bff5973abde3))
    - [object #164] refactor ([`883343b`](https://github.com/Byron/gitoxide/commit/883343bbbae431cfb8ffb16f0d39838b0d7636d7))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com/Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [repository #164] generic write_object() ([`c569f83`](https://github.com/Byron/gitoxide/commit/c569f83363489dde03c8b9cd01e75d35f5e04dbc))
    - Thanks clippy ([`33a8fb3`](https://github.com/Byron/gitoxide/commit/33a8fb34708407fd6b9a9ddabeaab51409aa1b03))
    - [object #164] Allow referenced objects to be serialized as well ([`a98d298`](https://github.com/Byron/gitoxide/commit/a98d2985dae2259d72bb91a01548906862fee9f7))
    - Release git-object v0.13.1 ([`2c55ea7`](https://github.com/Byron/gitoxide/commit/2c55ea759caa1d317f008966ae388b3cf0ce0f6d))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [object #190] consistent method naming ([`c5de433`](https://github.com/Byron/gitoxide/commit/c5de433e569c2cc8e78f3f84e368a11fe95f522a))
    - [object #190] More conversion methods for Object ([`78bacf9`](https://github.com/Byron/gitoxide/commit/78bacf97d669f3adfebdb093054c162cfd5214fa))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com/Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] dissolve 'immutable' module ([`70e11c2`](https://github.com/Byron/gitoxide/commit/70e11c21b0637cd250f54381d5490e9976880ad9))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [object #177] resolve 'mutable' module ([`b201b32`](https://github.com/Byron/gitoxide/commit/b201b3260e3eec98ed71716c1aab1ba4a06ab829))
    - [object #177] refactor ([`216dd0f`](https://github.com/Byron/gitoxide/commit/216dd0f10add7a11ebdf96732ed7649d74815d64))
    - [object #177] refactor ([`472e13b`](https://github.com/Byron/gitoxide/commit/472e13b27e97a196c644d716cad1801bd62fac71))
    - [object #177] Commit::write_to migration ([`60b9365`](https://github.com/Byron/gitoxide/commit/60b936553bef3c9126d46ece9779f08b5eef9a95))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com/Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] refactor tag write_to ([`7f19559`](https://github.com/Byron/gitoxide/commit/7f1955916ae9d7e17be971170c853487e3755169))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] into_mutable() -> into_owned() ([`7e701ce`](https://github.com/Byron/gitoxide/commit/7e701ce49efe5d40327770a988aae88692d88219))
    - [object #177] fix docs ([`25d8e7b`](https://github.com/Byron/gitoxide/commit/25d8e7b1862bd05489359b162a32c6ad45ecdf9a))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com/Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] fix docs ([`07be661`](https://github.com/Byron/gitoxide/commit/07be6611d1742633815566443f71eef8b85ad5c0))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - Merge branch 'git-ref-refactor' ([`5dbf753`](https://github.com/Byron/gitoxide/commit/5dbf753ce2035ffd07e4bce7ceb3bcd4e309c16e))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com/Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #175] refactor ([`ec88c59`](https://github.com/Byron/gitoxide/commit/ec88c5905194150cc94db4d4c20e9f4e2f6595c3))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com/Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] fix docs ([`2d7956a`](https://github.com/Byron/gitoxide/commit/2d7956a22511d73b767e443dac21b60e93f286dd))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Merge branch 'Byron:main' into main ([`dc58eca`](https://github.com/Byron/gitoxide/commit/dc58eca510e5a067acdeaad4b595a34b4598a0cd))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`51d1943`](https://github.com/Byron/gitoxide/commit/51d19433e6704fabb6547a0ba1b5c32afce43d8b))
    - [repository #162] what could be a correct implementation of a tree path lookup ([`1f638ee`](https://github.com/Byron/gitoxide/commit/1f638eee0aa5f6e1cc34c5bc59a18b5f22af4cbc))
    - Release git-object v0.12.2 ([`6e58edd`](https://github.com/Byron/gitoxide/commit/6e58edde2d7c881a6b957b7efafb63e2f517c9b4))
    - [object] argh, remove these tests for now no time for this ([`13d627d`](https://github.com/Byron/gitoxide/commit/13d627d19aae3bb9d44ed0e9304e373e90f51547))
    - [object] simply exclude the feature from testing for now… ([`adba3b9`](https://github.com/Byron/gitoxide/commit/adba3b982c4b21889615afafcfcaa7ae1f91661d))
    - [object] fix magically smaller object size expectation ([`bf4d2d7`](https://github.com/Byron/gitoxide/commit/bf4d2d7c0a33a3f35646f776edce4b829f086f66))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - Release git-object v0.12.1 ([`086baa2`](https://github.com/Byron/gitoxide/commit/086baa261d0874e541e374d51d427727aa37a8ee))
    - Remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com/Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.5.0 ([`bf15c2a`](https://github.com/Byron/gitoxide/commit/bf15c2a2f285046b094093760c1969007ee75e25))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com/Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.4.0 ([`0d5c8b9`](https://github.com/Byron/gitoxide/commit/0d5c8b96dfdfb96e4fc82623f756f6c7f7046e90))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com/Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - Clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - Thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] fix build (bad find&replace) ([`467395f`](https://github.com/Byron/gitoxide/commit/467395f19ce13ff8cd62499573d3cd4cb2e7797f))
    - [ref] refactor ([`e26c72f`](https://github.com/Byron/gitoxide/commit/e26c72fb1bf9392932ffe42843f3dec52c7bbd7d))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - [ref] refactor ([`207a799`](https://github.com/Byron/gitoxide/commit/207a799c1fcf490425f2e5dcf8274da83125af6f))
    - [ref] flexible and simple support for different hash lengths ([`9c2edd5`](https://github.com/Byron/gitoxide/commit/9c2edd537fb86d2d7db874ec976d0cb1b8ec7c2e))
    - Thanks clippy ([`c437304`](https://github.com/Byron/gitoxide/commit/c43730496ac5e1f7e66ee226099a4e99e151e97d))
    - [object] Add feature toggle for verbose errors… ([`4b63d8a`](https://github.com/Byron/gitoxide/commit/4b63d8a8709a2674d287879c4d6538a74cd7869b))
    - [object] support for verbose errors for object parsing ([`8156f10`](https://github.com/Byron/gitoxide/commit/8156f1037b87424864db73888108be12dedb5169))
    - [object] refactor ([`6f63983`](https://github.com/Byron/gitoxide/commit/6f639835362224bade27dd4f45c275544a39625d))
    - [object] Generalize nom error handling and use nom-errors instead of custom ones ([`47c8a97`](https://github.com/Byron/gitoxide/commit/47c8a97194c9e401ee311234a269f8b8f3650ba1))
    - [object] remove unused dependencies ([`2f01e46`](https://github.com/Byron/gitoxide/commit/2f01e46a9b30f1231adf1e79c5923843e63cad86))
    - [object] cleanup parsing error handling by removing NomDetail ([`e91cb40`](https://github.com/Byron/gitoxide/commit/e91cb405381d84bc1021c3ac30dfe6061788f9b1))
    - [object] refactor ([`1ddb5c0`](https://github.com/Byron/gitoxide/commit/1ddb5c07b75aa2b9a9536125fbba1fc862b7fe34))
    - [object] replace custom context impl with the one by nom ([`9a6692d`](https://github.com/Byron/gitoxide/commit/9a6692d034976dbcf011b587140c7f47fbc583e2))
    - [object] refactor ([`8205429`](https://github.com/Byron/gitoxide/commit/8205429b2ac160525a7449e50edf04aaf027f12c))
    - [actor] git-object uses git-actor ([`d01dd2f`](https://github.com/Byron/gitoxide/commit/d01dd2f9e9e8e2b81cdb1131a436d32b5819b731))
    - [actor] make signature parsing public, exposing nom :/ ([`a627972`](https://github.com/Byron/gitoxide/commit/a627972ecc53d38210c826f851ea9c5fec17b9cb))
    - [refs] try to get structure in place for reflog parsing ([`727c66a`](https://github.com/Byron/gitoxide/commit/727c66a2560c00cc8e01fbe47503ffbb67147c59))
    - Thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - (cargo-release) version 0.3.0 ([`87db688`](https://github.com/Byron/gitoxide/commit/87db688f23475d7232731429d770848aea228492))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - (cargo-release) version 0.2.0 ([`1327894`](https://github.com/Byron/gitoxide/commit/132789475400abe660b30ef6d2c5ff57821dd2c4))
    - [git-object] use git-validate crate ([`4ba98e8`](https://github.com/Byron/gitoxide/commit/4ba98e824417d1c58998fabee88549700a714bcf))
    - [git-object] refactor ([`d64d326`](https://github.com/Byron/gitoxide/commit/d64d3266167ee224b651cc24c4dbc8d88c9c9876))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - Switch to latest nom ([`859e57e`](https://github.com/Byron/gitoxide/commit/859e57eae93c3490523b7ed98f7a606acbd87a2f))
    - [git-ref] clear it out and move existing functionality to git-object ([`fa548ce`](https://github.com/Byron/gitoxide/commit/fa548ce94db3dd3969add494756fcc34e48985a3))
    - (cargo-release) version 0.5.0 ([`b6b5856`](https://github.com/Byron/gitoxide/commit/b6b58560b7c3bc88e2b8b780be5ceb4cb508a346))
    - [pack-gen] refactor ([`61554e2`](https://github.com/Byron/gitoxide/commit/61554e2effcbafef9cff0b407351c2fae0d2916c))
    - [pack-gen] tag support for tree traversal ([`28ed260`](https://github.com/Byron/gitoxide/commit/28ed260a73554d261c9b00c8ae9a46e66f123e6f))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com/Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - [pack-gen] more tests for Tag iterator ([`b69d6d6`](https://github.com/Byron/gitoxide/commit/b69d6d6590ea9d8de4a50e31ab9f331a2e21faab))
    - [pack-gen] the first green test for Tag iterators ([`df5ef8a`](https://github.com/Byron/gitoxide/commit/df5ef8a53cb4007058137890b414af510025fccf))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com/Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - Allow empty trees when parsing them at once, fixes #79 ([`d34fd19`](https://github.com/Byron/gitoxide/commit/d34fd19db5b3802ae9c677a6cf481f42e8a7e073))
    - Refactor ([`9870923`](https://github.com/Byron/gitoxide/commit/9870923ce02d20beb98be5e4bb76ff8081804054))
    - [hours-demo] computation seems to work better now ([`26ecca2`](https://github.com/Byron/gitoxide/commit/26ecca2133af287ddc9146fb3a3fc73dddc945e9))
    - Refactor ([`2d00c4e`](https://github.com/Byron/gitoxide/commit/2d00c4ed6be5baa1c3389a61102e25eb7d543465))
    - [hours-demo] Maybe the pinnacle of performance… ([`f70c61a`](https://github.com/Byron/gitoxide/commit/f70c61ab56b4153030d524da69a514a667c6abb7))
    - Remove debug-assert which doesn't hold - it's OK to have empty commit messages ([`13abc2d`](https://github.com/Byron/gitoxide/commit/13abc2d70c9aa42fb76e71d44c9c711e2780a5ba))
    - And it's a wrap for git-diff docs for now ([`9e09dd5`](https://github.com/Byron/gitoxide/commit/9e09dd560a23d52d0469ce4fc13de01f7efce227))
    - [traversal] first impl based on git-odb::traver ([`76a3017`](https://github.com/Byron/gitoxide/commit/76a3017b60d41957f5fea56bf7b2b2bf41aae0d5))
    - A new crate: git-traverse ([`1a9af50`](https://github.com/Byron/gitoxide/commit/1a9af50f1fca0e7e939f339b885c66dcb95e44e5))
    - (cargo-release) version 0.8.0 ([`a1ce210`](https://github.com/Byron/gitoxide/commit/a1ce210003ff07bf11291018bb182cbc7913647b))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - [traversal] add CommitIter::tree_id() convenience method ([`6affd9d`](https://github.com/Byron/gitoxide/commit/6affd9d90d56d89774fcd4843638309a198815bf))
    - [traversal] trying to get things done with gitoxide shows some teeth… ([`3fee661`](https://github.com/Byron/gitoxide/commit/3fee661af8d67e277e8657606383a670f17e7825))
    - Refactor; better iter error handling tests ([`9fe139b`](https://github.com/Byron/gitoxide/commit/9fe139b85c350c8cbb78975a94f4548130764b1c))
    - [tree-diff] more tests for the tree iterator ([`91b5a02`](https://github.com/Byron/gitoxide/commit/91b5a029337200a2873a21696020dcda08e335cb))
    - Test error handling of commit iteration ([`fcec4b4`](https://github.com/Byron/gitoxide/commit/fcec4b43f7c1d72680431ec3522b0db94728507f))
    - Thanks clippy ([`41418ed`](https://github.com/Byron/gitoxide/commit/41418ede15be9d0d18e49c34e4482c5701851404))
    - Fix serde support for commit iter token ([`3bfcb49`](https://github.com/Byron/gitoxide/commit/3bfcb49814ed5f14dd66206a54a9b85f13edd9d9))
    - [tree-diff] all the tests for commit iter ([`7ebea87`](https://github.com/Byron/gitoxide/commit/7ebea87b91c7cae3378fa3a5780d6c58e319c006))
    - [tree-diff] more tests ([`4f81450`](https://github.com/Byron/gitoxide/commit/4f81450b13bfc14cede1bec3234d33ec0844ac3d))
    - [tree-diff] And there is a working commit iterator, needs more tests ([`d991847`](https://github.com/Byron/gitoxide/commit/d99184782e9e79b517b7703ab41fefdc2424994e))
    - [tree-diff] A complete nearly working impl of a Commit iterator ([`4711821`](https://github.com/Byron/gitoxide/commit/4711821dd54193737cff76ce904b18d29b518ac2))
    - Frame for Commit iterator ([`796b74a`](https://github.com/Byron/gitoxide/commit/796b74a09cf1b4c8c4694d877a76da94d425bdc0))
    - First failing test for commit iterator; store two parents without alloc ([`8337514`](https://github.com/Byron/gitoxide/commit/8337514378148d72dc7f6d7474d6d0b794759589))
    - [tree-diff] one more test green + refactor ([`bc5549d`](https://github.com/Byron/gitoxide/commit/bc5549db2ad16222761219d8652caf64867a889f))
    - [tree-diff] refactor into iterator based model ([`29b527a`](https://github.com/Byron/gitoxide/commit/29b527aaea101c9b4e885db1c6d3533ef2310c54))
    - [tree-diff] The least intrusive way to allow dealing with tree iterators ([`d41dd3c`](https://github.com/Byron/gitoxide/commit/d41dd3c38ee34b250a4f5de120d7ae3e04e3212d))
    - [tree-diff] prototype an immutable tree iterator to avoid entry allocs ([`f38e5cd`](https://github.com/Byron/gitoxide/commit/f38e5cdcd072873707a21f0b73c491ef1f1c7a8f))
    - [tree-diff] A step closer to handling additions in a directory ([`a11f210`](https://github.com/Byron/gitoxide/commit/a11f210bec2c6c55bcf8cebe00e116e835306360))
    - Refactor ([`a4d5f99`](https://github.com/Byron/gitoxide/commit/a4d5f99c8dc99bf814790928a3bf9649cd99486b))
    - Refactor ([`633cba7`](https://github.com/Byron/gitoxide/commit/633cba7c1ff1f63c32613bedf963d1bd89afaee1))
    - First sketch of diff API ([`fc3f2b7`](https://github.com/Byron/gitoxide/commit/fc3f2b7066538e31f8d4bb1053d70dcabd5fbab1))
    - Better ergonomics for accessing decoded objects ([`ae3eab6`](https://github.com/Byron/gitoxide/commit/ae3eab6d6e4b96e207372fa8cb82f5ac9833e5e4))
    - Thanks clippy ([`8295548`](https://github.com/Byron/gitoxide/commit/829554805520170f69cadc61e6be5e7255737cff))
    - Refactor ([`9d03843`](https://github.com/Byron/gitoxide/commit/9d03843eeae388738d70b4251166081542893749))
    - Fix debug assert, thanks gitpython ([`fe954b9`](https://github.com/Byron/gitoxide/commit/fe954b9f6d26bd8629f24a01bd2a06f9800deed0))
    - More explicit expectations towards entries in mutable Trees ([`d94f84c`](https://github.com/Byron/gitoxide/commit/d94f84ceac637d2b6495be01dfc8eeb2494922f2))
    - Refactor ([`f19ea33`](https://github.com/Byron/gitoxide/commit/f19ea33709f7c31873e46d896ed7b6d06607f1e7))
    - An even better name for decode errors ([`f270850`](https://github.com/Byron/gitoxide/commit/f270850ff92eab15258023b8e59346ec200303bd))
    - Make clear it's a decode error we are using there ([`f45cb4b`](https://github.com/Byron/gitoxide/commit/f45cb4b62122559e5701923e0a23dd5791ee2ced))
    - Rename git-object::(owned->mutable)|(borrowed|immutable) #(67) ([`91ee558`](https://github.com/Byron/gitoxide/commit/91ee55893bf4b27a47d86d51bae6f99b59b69147))
    - The first basic traversal utility #(67) ([`ea6610b`](https://github.com/Byron/gitoxide/commit/ea6610b9157d8d0dabd2ddd07c45dc6651b9cf84))
    - (cargo-release) version 0.7.0 ([`b900914`](https://github.com/Byron/gitoxide/commit/b900914a00292217ba7b9bcac260591800395287))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - Thanks clippy ([`cefbf3e`](https://github.com/Byron/gitoxide/commit/cefbf3e02edebd1875cd2762eb231e6c379b1ebb))
    - Upgrade depdendencies ([`e4a7711`](https://github.com/Byron/gitoxide/commit/e4a77112ee4f5d0ab61d0678aab8ee090335740c))
    - Improved high-level docs for git-object ([`60036f2`](https://github.com/Byron/gitoxide/commit/60036f20328600f0faaaf21ca30f1b9d0275d15f))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - First round of git-object doc proof reading ([`524ce51`](https://github.com/Byron/gitoxide/commit/524ce51eb3e606b1225a23fce62df2199799d4f3))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com/Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - `deny(missing_docs)` for git-object ([`8525684`](https://github.com/Byron/gitoxide/commit/8525684c6c69677f3e1b40a3673a817e111e9bff))
    - More docs for owned git-object ([`b79101d`](https://github.com/Byron/gitoxide/commit/b79101d714f59a42a30eb47776486a212ec0f738))
    - A few more comments in git-object ([`171d269`](https://github.com/Byron/gitoxide/commit/171d269e428f711b163f6644ebf0c44c1279d497))
    - Thanks clippy ([`ba9b3c2`](https://github.com/Byron/gitoxide/commit/ba9b3c2345887353e02fc081be80733f1c5e22d9))
    - Refactor ([`d5d7cf9`](https://github.com/Byron/gitoxide/commit/d5d7cf9d3f42d83652a7b81bc6e1ee6731396d6b))
    - More git-object docs ([`ba595f6`](https://github.com/Byron/gitoxide/commit/ba595f6d4864eafc64f31460f7192cb48abd408a))
    - More docs of git-object::owned ([`0620dce`](https://github.com/Byron/gitoxide/commit/0620dce7a3ac368354c73e3927eb96a6e4990f0c))
    - Docs for git-object::borrowed ([`68e524d`](https://github.com/Byron/gitoxide/commit/68e524d079fe9042ebba1e33457f934a64018623))
    - Docs for git-object::borrowed::commit ([`c5c1df0`](https://github.com/Byron/gitoxide/commit/c5c1df031aa391e0e65d0540f8414cbd1d1aa39f))
    - Merge branch 'commit-graph' into main ([`9cb09b2`](https://github.com/Byron/gitoxide/commit/9cb09b248796f0ff5c9d3f3e857de4731324cfd5))
    - Add and use borrowed::Id::null_sha1() ([`c717492`](https://github.com/Byron/gitoxide/commit/c717492d0038f55a6f21b48937b56a756890d214))
    - Updated `expect` message ([`e8d8d93`](https://github.com/Byron/gitoxide/commit/e8d8d9351168b5423c44626ed8ac81cf7b013a02))
    - Update error message for type name ([`92cbb13`](https://github.com/Byron/gitoxide/commit/92cbb1314657abaad560d068e7395a70769f0592))
    - Document borrowed odb objects ([`7626f7f`](https://github.com/Byron/gitoxide/commit/7626f7f3af885f1b95801f9dbc71bee0bc77400e))
    - Remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com/Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - Refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
    - (cargo-release) version 0.4.0 ([`0d7b60e`](https://github.com/Byron/gitoxide/commit/0d7b60e856325009431172e1df742a1cd2165575))
    - (cargo-release) version 0.4.0 ([`f9dd225`](https://github.com/Byron/gitoxide/commit/f9dd225afc4aafde1a8b8148943f56f2c547a9ea))
    - [clone] proper parsing of V1 refs ([`d262307`](https://github.com/Byron/gitoxide/commit/d26230727ef795a819852bc82d6c2e9956809d8c))
    - [clone] Don't expose hex-error in public interfaces anymore ([`92dab30`](https://github.com/Byron/gitoxide/commit/92dab3033890fe26fe2b901d87abe16abd065cce))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - Refactor ([`a0bebd1`](https://github.com/Byron/gitoxide/commit/a0bebd17500bccc08ed5a1c16e2ffcde89c71052))
    - Bump minor version to 0.3 ([`4351e28`](https://github.com/Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - Update to quick-error 2.0 ([`4b1b784`](https://github.com/Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - Thanks clippy ([`62d2ff3`](https://github.com/Byron/gitoxide/commit/62d2ff383c5f7fe884057c70868569a811a73e00))
    - Organize object type comparisons by probability… ([`19a5d94`](https://github.com/Byron/gitoxide/commit/19a5d9465f7962cfcc39ea31a2c84be6235e40ed))
    - Don't cause re-allocs of the compression buffer ([`2bb6fd2`](https://github.com/Byron/gitoxide/commit/2bb6fd26235825484a8f60a49455fee71f08236c))
    - Reduce memory consumption ([`6d1a7a1`](https://github.com/Byron/gitoxide/commit/6d1a7a1292e8065d0a777cb6acd34776b1e23696))
    - Also read the pack trailer during iteration ([`98a8e17`](https://github.com/Byron/gitoxide/commit/98a8e17e791b6bcd92149d7ff68cbc9d9ceee087))
    - Refactor; better tests ([`12d14bf`](https://github.com/Byron/gitoxide/commit/12d14bfe2aa089723a395287c5100aad6e838935))
    - First step towards putting the index file into position ([`d994c74`](https://github.com/Byron/gitoxide/commit/d994c74d7cd9c9c004bf27f0b2ac23558ce9c50d))
    - Improve looks of documentation ([`11a32eb`](https://github.com/Byron/gitoxide/commit/11a32ebc2209d1a05eb4c4ec5131e85dfb43d9f6))
    - Finish Sink implementation ([`84f7908`](https://github.com/Byron/gitoxide/commit/84f7908b1883ed6c484ca4e522ac530c8cc161d5))
    - Introduce hash kind, as this should be specified when writing an object ([`f5d0acf`](https://github.com/Byron/gitoxide/commit/f5d0acf61ac5dd815bc5ece4462eb9a43dd9c44a))
    - (cargo-release) version 0.2.0 ([`76fe0ab`](https://github.com/Byron/gitoxide/commit/76fe0ab5f0b58504a5ea5adb74b349b9d588e51e))
    - (cargo-release) version 0.2.0 ([`d350a13`](https://github.com/Byron/gitoxide/commit/d350a13784685ea82b84646b18736986aeb68146))
    - Beautifully implement shared extra-header access ([`920d1ac`](https://github.com/Byron/gitoxide/commit/920d1accc92d67019f0e654f8c4ab5c95d798925))
    - Roundtrip Rust repo in stress test; accept more diverse trees when parsing ([`0347cdb`](https://github.com/Byron/gitoxide/commit/0347cdbf473d80c016745ffbaf582832fe2eba2a))
    - Make sure we write out trailing newlines properly in multi-line headers! ([`7f044c3`](https://github.com/Byron/gitoxide/commit/7f044c36279aadfd7a2ebeecedd7f2c20b2b7b52))
    - Consume PGP signature in tags fully ([`ffd6c31`](https://github.com/Byron/gitoxide/commit/ffd6c31aa3adecc2dea6357373d88a495d63ba0d))
    - Support for very special tree entry mode… ([`2be2c9d`](https://github.com/Byron/gitoxide/commit/2be2c9d31563b147f0f2a5c1cd03279c79f1dd6b))
    - Make tagger signature optional ([`3358f9a`](https://github.com/Byron/gitoxide/commit/3358f9ae539c7b7878d87a209d678d2f08f94b1b))
    - Remove now unused pgp_signature field - it's in extra-headers ([`c8c937c`](https://github.com/Byron/gitoxide/commit/c8c937c505e455572544a1a9da1b991ef4662b97))
    - Proper support for extra-headers ([`d0feb2b`](https://github.com/Byron/gitoxide/commit/d0feb2b5b30f9719bf3b40ac5b74f8a5a8515bc9))
    - Abiility to read mergetags (for now only these) as extra-headers ([`bd3a2db`](https://github.com/Byron/gitoxide/commit/bd3a2db1068ce7509612ef1be0a108b7bb45ed49))
    - Switch to latest quick-error ([`9760856`](https://github.com/Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com/Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - Empty trees are allowed, and they are special, too ([`6bed200`](https://github.com/Byron/gitoxide/commit/6bed200ec1a528574edabf5783e9ebfb00add56d))
    - Refactor ([`56b66ac`](https://github.com/Byron/gitoxide/commit/56b66ac069f24635a8fa74b4b2231dfb0a82a1ef))
    - Basic top-level object round-tripping ([`e851cbe`](https://github.com/Byron/gitoxide/commit/e851cbe585525b3e50114eb8d2a0662149bf2019))
    - Refactor ([`ec5e50f`](https://github.com/Byron/gitoxide/commit/ec5e50f607d59302d6db3944f6ea7b667f820927))
    - Implement blob ([`f30caf4`](https://github.com/Byron/gitoxide/commit/f30caf4ff69fee36e65a2e404910b88e06d539bc))
    - Refactor ([`335e98a`](https://github.com/Byron/gitoxide/commit/335e98ab3a2e9c05141f1cd218197079bb51cfa5))
    - Tree roundtrip ([`8b26a0e`](https://github.com/Byron/gitoxide/commit/8b26a0e16c838270290cb3ac02b029100afe6b46))
    - Prepare for writing out owned trees ([`2b6eced`](https://github.com/Byron/gitoxide/commit/2b6eced325057a884d56ed9db755a8699cbf8d00))
    - Manual deserialize implementation, for now ([`9f46efd`](https://github.com/Byron/gitoxide/commit/9f46efd625d45a9ad947e9f7bc6f31f4426f3cfc))
    - Use borrowed::Id in trees for full type safety ([`5d57c1f`](https://github.com/Byron/gitoxide/commit/5d57c1f7e3b9a84f7b46a4378015572155f3104b))
    - Refactor ([`f7b8826`](https://github.com/Byron/gitoxide/commit/f7b8826ba144f54f3a3fe6096a5daafd29e25002))
    - Commit round-tripping works with multi-line signatures ([`b692b0a`](https://github.com/Byron/gitoxide/commit/b692b0aa3f38507697096e671c990700d25933c8))
    - First attempts to roundtrip signatures shows I parse it wrongly :D ([`1b48367`](https://github.com/Byron/gitoxide/commit/1b48367d02fde977ed4acab63e295c0c5feec357))
    - Prepare for allowing an owned, processed version of multi-line headers ([`f966e7f`](https://github.com/Byron/gitoxide/commit/f966e7f26cbbe99e5508215adaacf073e108bf48))
    - First attempt to round-trip multi-line headers ([`645ef94`](https://github.com/Byron/gitoxide/commit/645ef946653caf2eed571d83c61e8b7d7c1cf4b4))
    - Single-line header support ([`478c09e`](https://github.com/Byron/gitoxide/commit/478c09e54cc73dcc5cce7aea6bc0702882c5f882))
    - The first basic version of commit serialization ([`5319f64`](https://github.com/Byron/gitoxide/commit/5319f64036e09bce97285db19f30f988a5039761))
    - Make reusing round-trip code easier ([`3b9d66c`](https://github.com/Byron/gitoxide/commit/3b9d66c932075feb08cdf2967f7698daef9fd3ff))
    - Refactor ([`987787e`](https://github.com/Byron/gitoxide/commit/987787e3084bbfc948ed3ca464909a2912f7b653))
    - Fix tests on windows, by ignoring them ([`512ed6c`](https://github.com/Byron/gitoxide/commit/512ed6c915b3db2cd3353ea23b945652ad1bef50))
    - Use borrowed::Id everywhere ([`9f876f0`](https://github.com/Byron/gitoxide/commit/9f876f04feaa3fd3bba9729fff7667708dc0c4be))
    - Move git_object::Id into git_object::owned::Id - much better already! ([`50c7136`](https://github.com/Byron/gitoxide/commit/50c71368a69f57b0a43061df105685e992ed384a))
    - Basic integration of borrowed Id; translate between owned and borrowed ([`84ff638`](https://github.com/Byron/gitoxide/commit/84ff638a183567593ace8056de2a856304d29d1d))
    - Prepare to allow Id be owned and borrwed; abstract over hash type ([`d883c31`](https://github.com/Byron/gitoxide/commit/d883c31dd14f253a3af153616007c9231fdf265a))
    - Introduce the notion of IdRef ([`7007361`](https://github.com/Byron/gitoxide/commit/700736197b903cb6fe9ed60718e49e4be44199a7))
    - Use statically known borrowed arrays for perfect type safety! ([`3ead048`](https://github.com/Byron/gitoxide/commit/3ead048bb999e6266831df2ca6c2022013529ab2))
    - Refactor ([`766f3e4`](https://github.com/Byron/gitoxide/commit/766f3e491dc6ebcca20753cda3487545268721eb))
    - Tags can write signatures ([`a48275e`](https://github.com/Byron/gitoxide/commit/a48275e65bee7f544c19bc81307660ed4f60b8fa))
    - Tags can write a message properly ([`b590b77`](https://github.com/Byron/gitoxide/commit/b590b779a6f168db377bf5b4b796a4813bd19ccb))
    - Green tests as basic tags can now be serialied ([`62a02b4`](https://github.com/Byron/gitoxide/commit/62a02b490055d9b95a5aae3cbe1641f42ff69df8))
    - More tests for signature serialization ([`5000f30`](https://github.com/Byron/gitoxide/commit/5000f30bd0085c0afacf2c32d8a31224ec7337d0))
    - Time serialization ([`1eb1e36`](https://github.com/Byron/gitoxide/commit/1eb1e36992f9973977b4d94d55348b7a3eecd3fb))
    - Prepare writing of time as part of signature ([`f560bc5`](https://github.com/Byron/gitoxide/commit/f560bc50a2a2e4c9697c17b59ec5cf4122992fab))
    - Add new 'git-ref' crate; place ref name validation code there ([`1a0e84e`](https://github.com/Byron/gitoxide/commit/1a0e84e627b17be1b1fb53b4dc98ab78e9cfb9a7))
    - Refactor ([`b4392e8`](https://github.com/Byron/gitoxide/commit/b4392e880ed67464af9e8cfa2e343d10f23a949f))
    - Some more boilerplate to actually implement complete ref name checking ([`087857a`](https://github.com/Byron/gitoxide/commit/087857a56654537fdfb5bfa6c66745184027517f))
    - Very basic first steps of validated serialization ([`d3fd5ff`](https://github.com/Byron/gitoxide/commit/d3fd5ffe10015e2a13200a1fef5bd903532f81af))
    - It's probably OK to consume the borrowed objects when converting them to owned ([`101ddd5`](https://github.com/Byron/gitoxide/commit/101ddd586d4250aa5b3c8c6f8076456ae997faec))
    - Try basics of roundtrip without consuming the source object ([`581794e`](https://github.com/Byron/gitoxide/commit/581794efcf4577c21f2ff078ba7844a71b47c0aa))
    - Refactor ([`bca1f16`](https://github.com/Byron/gitoxide/commit/bca1f16a6f3da497e3488e333d5ebc99e39ee689))
    - First sketch of owned Tag in preparation for round-tripping ([`fa2745a`](https://github.com/Byron/gitoxide/commit/fa2745a5d5f7b6c4e02177e4080db7df6603b9fc))
    - Refactor ([`90ae25d`](https://github.com/Byron/gitoxide/commit/90ae25d39aa4540fc2785eb7cb189eee102895c0))
    - Refactor ([`256581b`](https://github.com/Byron/gitoxide/commit/256581bad6692a458b331b712d16ce2d5143cb75))
    - 'data -> 'a as it's shorter and also more idiomatic ([`71821e9`](https://github.com/Byron/gitoxide/commit/71821e938887f448f1458642eda2bac365f2aa85))
    - Refactor ([`dedd4dc`](https://github.com/Byron/gitoxide/commit/dedd4dc91c26dfef368307345bb9e8d49637207c))
    - Apply cargo-diet (better late than never :D) ([`295fc81`](https://github.com/Byron/gitoxide/commit/295fc81a2e0e31d6d83eba7e169dc4ed05038083))
    - Make crates publishable ([`5688a34`](https://github.com/Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Handle windows newlines in test suite for packs as well. ([`ebd5176`](https://github.com/Byron/gitoxide/commit/ebd517633f099582dc2633e71f7bb7890acd14d1))
    - Fixup text file tests on windows ([`2288088`](https://github.com/Byron/gitoxide/commit/22880881d5e442acdeb8dd0cfb5ecc4f62783951))
    - Add metadata to allow docs.rs build all featueres ([`10f9386`](https://github.com/Byron/gitoxide/commit/10f9386a12decc1f13999aee72be484c8f6d48ce))
    - Git-odb with serde support ([`0da930c`](https://github.com/Byron/gitoxide/commit/0da930cf23f215cc1e2bda8f7340a5d69370735a))
    - Cut back on onnecessary annnotations: serde(borrow) ([`759915c`](https://github.com/Byron/gitoxide/commit/759915c75e473f65d35ba926aaca8303e5a77f9a))
    - Serde support for all git-object types, incl. test ([`1ae8f9c`](https://github.com/Byron/gitoxide/commit/1ae8f9c8b6b699c3f4928905358f42187bef07a7))
    - Learn from the best: with-serde -> serde1 ([`d651c21`](https://github.com/Byron/gitoxide/commit/d651c218bfb7af5fc67ca4b9763703fb29788017))
    - Commit to using bstr whenever something is not data bytes; remove miniserde ([`3183d1b`](https://github.com/Byron/gitoxide/commit/3183d1b02c2d7bb3c750f8472c29bb161641ca7f))
    - Prepare centralization of bstr as optional component ([`aa857d9`](https://github.com/Byron/gitoxide/commit/aa857d9df32dfc75f151154ca430ddfee907deed))
    - Add support for miniserde ([`f806647`](https://github.com/Byron/gitoxide/commit/f80664769e4fdbab3d1ffa56510ba87e570ae9b0))
    - First gentle test of adding serde support selectively. ([`78d9bc0`](https://github.com/Byron/gitoxide/commit/78d9bc0f54504dc809651aeb0d24e7ac6a3f0bb3))
    - Allow for more screen space when formatting ([`6794300`](https://github.com/Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - Pack offset by index ([`69e35b1`](https://github.com/Byron/gitoxide/commit/69e35b1d8f24f366d675484a1bddbebd37b72e22))
    - Test V1 lookup ([`e9c7127`](https://github.com/Byron/gitoxide/commit/e9c71271fa51d5420fcb205d2d3deb6d012f0d41))
    - Validate sha1 of pack objects, some work, some don't for some reason… ([`aa8799a`](https://github.com/Byron/gitoxide/commit/aa8799a01b92c3c3b7d4347f745921bbb685c454))
    - Capability to write loose object headers, fast ([`de0aeff`](https://github.com/Byron/gitoxide/commit/de0aeff518ebd218b73bf472558f278f6bcdc17c))
    - Simplify folder names ([`36fde1f`](https://github.com/Byron/gitoxide/commit/36fde1f90e9034060b5ede8a923365474659085e))
    - Implement object Id as newtype for more convenenience and… ([`b7a4d17`](https://github.com/Byron/gitoxide/commit/b7a4d17d886d555b080e4182d49dc648e201ad13))
    - Rename grit to 'gitoxide', CLI name is 'gio' ([`9d6007f`](https://github.com/Byron/gitoxide/commit/9d6007f83b3b018d736d58aa0722b83b9cffb228))
    - Fix clippy ([`a9c5da7`](https://github.com/Byron/gitoxide/commit/a9c5da7132eeaa6806b8190985a7aa25f9ef89d8))
    - More convenient access to our four object types ([`ecda6d2`](https://github.com/Byron/gitoxide/commit/ecda6d23561dc176f7d7ad2565da8105efac614f))
    - Even better trait derives ([`e78f9f6`](https://github.com/Byron/gitoxide/commit/e78f9f64c8d52402197b1f946bcd32f0d83e6c7d))
    - Better trait support for basic types ([`6617386`](https://github.com/Byron/gitoxide/commit/6617386e37b69f6e036ab27280c946e271c99540))
    - Memory size checks for objects ([`ab51616`](https://github.com/Byron/gitoxide/commit/ab51616bb250a62b5367e861c25c1d90ec60f720))
    - Make single-field objects blob and tree more explicit ([`1aef68f`](https://github.com/Byron/gitoxide/commit/1aef68f7e979324eb94966d44c160ffe537ee4a8))
    - Add Blob type to parsed objects ([`d3e8e4b`](https://github.com/Byron/gitoxide/commit/d3e8e4b24ecda84665b994ccad768774efdcdc90))
    - Fix imports ([`10f2967`](https://github.com/Byron/gitoxide/commit/10f29675442c76b38e0a8deb757930a13af3a3bb))
    - Try pub use with rename. Not bad in the docs, but maybe a bit confusing ([`526f3f8`](https://github.com/Byron/gitoxide/commit/526f3f8d3ca9fe9672b0518f1bc3b921f695c0d8))
    - Refactor ([`2ffd7fa`](https://github.com/Byron/gitoxide/commit/2ffd7fa6c4e5a88042b7ee1d56ec8d8515f0991f))
    - Refacto ([`ffc0089`](https://github.com/Byron/gitoxide/commit/ffc0089fd7f4ffd3eeb0d0411b6857a28b388001))
    - Refactor ([`b9a1647`](https://github.com/Byron/gitoxide/commit/b9a16473ed028abc59fc5126db9530f2107498d8))
    - Test for parsing trees from loose dbs ([`4f48249`](https://github.com/Byron/gitoxide/commit/4f4824971d62d165fd4c2bea869fd88986dc259f))
    - Refactor ([`9f9ccad`](https://github.com/Byron/gitoxide/commit/9f9ccad37fea96954a2df9e314b6c154466dc0ca))
    - Move git-object tests to top-level for separation and cleanness ([`df42a01`](https://github.com/Byron/gitoxide/commit/df42a012bcc489b78320126e51b1f121abe7c018))
    - Prefer integration level tests, but use unit-tests where appropriate ([`ec3be19`](https://github.com/Byron/gitoxide/commit/ec3be19c8d007565b814b4757f17811ec0e9de2c))
    - Run previously unused method of Tree ([`0d159c2`](https://github.com/Byron/gitoxide/commit/0d159c2b76f2a8fc3c391fd990d8e7a4eeffc913))
    - Actually use the Tree object ([`635e735`](https://github.com/Byron/gitoxide/commit/635e735419af906579de681dbc27b36fd826406d))
    - Handle commits without newlines; make tag newlines optional ([`c0b54be`](https://github.com/Byron/gitoxide/commit/c0b54bef5a2bcfce9b6deb90cdd27c7e0cc85810))
    - Handle tags without newline; document fixture processing step ([`344a562`](https://github.com/Byron/gitoxide/commit/344a5622953047e6f2a543bfb0355fb060a4b1e8))
    - Don't assume newlines in trees anymore ([`45d7c36`](https://github.com/Byron/gitoxide/commit/45d7c365072a9bada3a6e0b77ced7669fe5533a3))
    - Found huge issue with newlines polluting fixtures. ([`f182d22`](https://github.com/Byron/gitoxide/commit/f182d22caf1dd9c262cdca6a1834478556a74f31))
    - First tree implementation, which seems to work well ([`9694fcb`](https://github.com/Byron/gitoxide/commit/9694fcbeb7bea6ebf814119ba5757110ae33bc55))
    - Boilerplate for tree parsing ([`48c4c07`](https://github.com/Byron/gitoxide/commit/48c4c07098d807b3d62e540e06459c66fef94355))
    - Refactor ([`d48cafa`](https://github.com/Byron/gitoxide/commit/d48cafa7edc4c6d977c396df4a26d235a3bd662c))
    - Add conversion traits for Object<->Tag|Commit ([`7dcbd5d`](https://github.com/Byron/gitoxide/commit/7dcbd5dc764a07685a66594e3ae5514a9df83082))
    - Make Commit available in borrowed object ([`b2d1b5d`](https://github.com/Byron/gitoxide/commit/b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8))
    - Use smallvec to save memory in the common case (single parent) ([`263835b`](https://github.com/Byron/gitoxide/commit/263835b7e14e94bfb641067e8188e23d81bc9cac))
    - More tests ([`56248fe`](https://github.com/Byron/gitoxide/commit/56248fe9a351572478cecda8520c25ec25664bc3))
    - Now gpg-signature parsing works correctly - thanks to peek(…) ([`7078dac`](https://github.com/Byron/gitoxide/commit/7078dac0fc27594c63cd9550c8b8b4ac7a52a627))
    - First somewhat working version of single/multi-line signature parsing ([`dab5c65`](https://github.com/Byron/gitoxide/commit/dab5c6581dc218ee9a7f049c5499975f762d81cf))
    - Support single-line gpg signatures ([`71330b5`](https://github.com/Byron/gitoxide/commit/71330b526614a78e20e739aa8b1cd31b5cf2ce0e))
    - Support for commit encoding field ([`40bffe9`](https://github.com/Byron/gitoxide/commit/40bffe9b36f5afcb9b3f147d47b94b5e82acaae8))
    - More commit tests, next up: encoding ([`ca4d3aa`](https://github.com/Byron/gitoxide/commit/ca4d3aad8f91189890b8445680406fddb6544af4))
    - First successful parsing of commit ([`b44765a`](https://github.com/Byron/gitoxide/commit/b44765ad08f53a7062def35ecb7fe7624827df85))
    - Parse BStr versions of hex-shas directly ([`e3a2b77`](https://github.com/Byron/gitoxide/commit/e3a2b7782fa48f474c2e1d51a6b8c2ea2c561549))
    - Parse parents ([`696e0a3`](https://github.com/Byron/gitoxide/commit/696e0a3c48e72373cb540d16b640ddb6fc2a2dcf))
    - Use BStr instead of Id to avoid parsing into something we might not use/need ([`7c97471`](https://github.com/Byron/gitoxide/commit/7c97471c34362c9d3d56ccada252d3058aea6697))
    - Factor out hex sha parsing ([`d650dd2`](https://github.com/Byron/gitoxide/commit/d650dd26a168ab5a8d679dfb4b93a7f2863a20f0))
    - Refactor ([`0104f4c`](https://github.com/Byron/gitoxide/commit/0104f4c8a8449c2549bfcfeacfeb20f14b2ddc2d))
    - First stab at factoring header parsing into sub-parser ([`6f6ee8f`](https://github.com/Byron/gitoxide/commit/6f6ee8f721df9f3caf4db54346e7653f341552e3))
    - First fixtures for commit parsing ([`551f2d1`](https://github.com/Byron/gitoxide/commit/551f2d1f8e32e7e64a0d19e9e7d3b3ea63e9b449))
    - Avoid unnecessary allocation when creating SHA1 paths in loose ODB ([`09d8d3a`](https://github.com/Byron/gitoxide/commit/09d8d3a12e161a7f6afb522dbe8900a9c09bce06))
    - Document existing use of unsafe, deny everywhere else ([`41f4bce`](https://github.com/Byron/gitoxide/commit/41f4bce9d9a492f8e20a6eb5b3eaf5adc6d78329))
    - Cleanup integer parsing in loose object database ([`ecdce1a`](https://github.com/Byron/gitoxide/commit/ecdce1a05d8c732afd53c6da6067bf591f96fa6a))
    - Add remaining tag tests, along with some fixes ([`06e22fb`](https://github.com/Byron/gitoxide/commit/06e22fb8aea281676e53f786ddb808dd77d3b702))
    - Use bstr were possible ([`01dd4e2`](https://github.com/Byron/gitoxide/commit/01dd4e2a978a9f5bd773dae6da7aa4a5ac1cdbbc))
    - The defining property is actually that the object is borrowing data ([`e0125fd`](https://github.com/Byron/gitoxide/commit/e0125fdb0a41ed139364084f6d679932f08b7b4f))
    - Refactor ([`683360a`](https://github.com/Byron/gitoxide/commit/683360a6932f7d5e216dc0fdafa5890c6708d1e8))
    - Move all tests into the top-level for nicer names basically :D ([`598901a`](https://github.com/Byron/gitoxide/commit/598901a768fec768b2519e7925ac623cb66582d6))
    - Refactor ([`0f01e9f`](https://github.com/Byron/gitoxide/commit/0f01e9fff39fb7f1234f57c6689c0e114d9aece7))
    - Refactor ([`87bbea4`](https://github.com/Byron/gitoxide/commit/87bbea48d247b7938e74672e1a5cb1b8b5cc6a9f))
    - Refactor; add more signature parsing tests ([`ba9c7de`](https://github.com/Byron/gitoxide/commit/ba9c7de7ca93ac42d3c57315d743b321f8f9e3b5))
    - Cleanup; all tests work! ([`7c96603`](https://github.com/Byron/gitoxide/commit/7c9660354484869681356a8c4ef8057313e864f2))
    - Fix whitespace ([`ebaaa00`](https://github.com/Byron/gitoxide/commit/ebaaa00d9508141746a7c20e5d25d877f38733e9))
    - First version of tag message parsing - it's actually changed now ([`74b2328`](https://github.com/Byron/gitoxide/commit/74b2328fcbbcffab9981c23e903c4f4c5d085aff))
    - Implement parse_signature with nom, starting to like it ([`ebdf205`](https://github.com/Byron/gitoxide/commit/ebdf205038b66108c0331aa590388431427493b7))
    - First part of parsing tagger signatures ([`5b43270`](https://github.com/Byron/gitoxide/commit/5b432703cf1c44bbf29e8bf89b297ea29c959be6))
    - Generalize with Boxed error cause ([`824cd2c`](https://github.com/Byron/gitoxide/commit/824cd2cfbfaef605e953f0af193a036ef74bcac7))
    - First seemingly awkward way of not discarding too much error information… ([`6f9a636`](https://github.com/Byron/gitoxide/commit/6f9a636da5c2f33a612395a25e8b92e557d06e83))
    - Refactor ([`fb287af`](https://github.com/Byron/gitoxide/commit/fb287af33fcb75c01ac25dd484f529cbb49f3e6f))
    - The first sketch of parsing a tag with Nom and half-decent errors ([`4498dff`](https://github.com/Byron/gitoxide/commit/4498dff1cf63abe53ae17b59d3658ab52235630d))
    - Use git-object in git-odb ([`07f7c31`](https://github.com/Byron/gitoxide/commit/07f7c318d55603e3731f08cb04d3da8ac2fcfea6))
    - Move all object related code into own crate… ([`605ef20`](https://github.com/Byron/gitoxide/commit/605ef20ec5ccf66e4f42df6d0140e4087aa13053))
</details>

## 0.26.0 (2023-01-06)

A maintenance release without user-facing changes.

## 0.25.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.24.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.23.0 (2022-11-21)

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

## 0.22.1 (2022-11-06)

### New Features

 - <csr-id-14d613303b8663daac1ab1eed8413266594050c3/> `Object::try_into_blob_ref()` as alternative to consuming `*::try_into_blob()`

## 0.22.0 (2022-10-10)

### New Features

 - <csr-id-5095df949d30b57b09bd68355743d255a897365f/> add `tree::EntryMode::is_blob()` to quickly determine if an entry is a blob.

### Bug Fixes

 - <csr-id-2df3c7395a791c751b22187a936facbce9f3f168/> don't panic if broken loose objects are encountered.
   Previously a loose object could advertise any object size and cause a
   panic if it was shorter than advertised.
   Now an error will be returned.

## 0.21.0 (2022-09-20)

<csr-id-9c8a36f35b1e8dad66befb102e5a8670b4a6c2b2/>

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Chore (BREAKING)

 - <csr-id-9c8a36f35b1e8dad66befb102e5a8670b4a6c2b2/> remove quick-error in favor of thiserror
   Some errors change shape which makes this a breaking change.

## 0.20.3 (2022-08-28)

Maintenance release without user-facing changes.

## 0.20.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.20.1 (2022-08-17)

### New Features

 - <csr-id-4f4bba23a13f11050e35887da004e48148d68aea/> `tree::EntryMode::as_str()` to display itself as a string.

## 0.20.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Bug Fixes

 - <csr-id-761695cd10222eccb720812fa41d5295d506fce9/> improve error messages related to object decoding

## 0.19.0 (2022-05-18)

### New Features

 - <csr-id-0d22ab459ce14bc57549270142595d8ebd98ea41/> `TagRefIter::tagger()`.
   Additionally ergonomics have been improved as the iterator is now
   `Copy`, similarly to the other iterators.

## 0.18.0 (2022-04-03)

### New Features

 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-8f1b42b193098f6e116b3522a3df08d5d1b42a0a/> `CommitRef::time()`
   A shortcut to allow for fluid retrieval of the `committer.time`.
 - <csr-id-c51a9252f8a038bf995d50cdbfc8356362020bbc/> `CommitRefIter::(author|committer)()`, better usability
   The methods returning an iterator are now consuming, which allows
   them to be nested by callers.
 - <csr-id-91065cd790f559f4a661ecd9c2d2f649922682e3/> `CommitRefIter::parent_ids()`
   Allocation-free parent retrieval.
 - <csr-id-5c573bf9abfc10227d5a900100e84d7f8f6c0504/> in-manifest and in-lib feature documentation

## 0.17.1 (2022-02-01)

A automated maintenance release without impact to the public API.

### Fixes

 - Corrected the tree-entry ordering implementation.

## 0.17.0 (2022-01-23)

<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-322b2901382192d3b4ac0fbae32381a9abb49fa9/>

### Chore

 - <csr-id-322b2901382192d3b4ac0fbae32381a9abb49fa9/> upgrade dependencies

### New Features

 - <csr-id-eb36a3dda83a46ad59078a904f4e277f298a24e1/> Add sorting mode to ancestor traversal  #270
 - <csr-id-a0bb6529216160d14e67b97206a51140099fd245/> add `Data` object
   This is typed data baked by a slice for conversion into parsed ObjectRef's
   for example.
   
   This is usually the result of a `Find` operation on an object database.

### Changed (BREAKING)

 - <csr-id-fda2a8d2f5f8b7d80b4cc0177d08d6a061f1b8f1/> rename `commit::ref_iter::Token::into_id()` to `*::try_into_id()`
   This makes the method more consistent.
 - move gix_pack::data::Object to gix_object::Data, massively alter gix_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

## 0.16.0 (2021-11-29)

<csr-id-598698b88c194bc0e6ef69539f9fa7246ebfab70/>

Maintenance release due, which isn't really required but one now has to be careful what's committed at once.

## 0.15.1 (2021-11-16)

A maintenance release triggered by changes to gix-pack and changelog rewrites.

## v0.15.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `gix-hash`.

## v0.14.1 (2021-10-15)

<csr-id-899c57927ce4ba2e1b5d8182f9e731c2a9094cba/>

This release greatly improves performance when decoding trees, a critical requirement when building packs
or generally trying to figure out what changed between commits.

### Other

 - <csr-id-899c57927ce4ba2e1b5d8182f9e731c2a9094cba/> describe variants

### Performance

<csr-id-f9232acf8e52f8cd95520d122469e136eb07b39f/>

 - Provide a new fast parser for tree objects which is used by the tree entry iterator.
 - <csr-id-83d591d536d1a43e8523082824ec0b95cca55d34/> parse entry mode into number instead of comparing it to byte strings

## v0.14.0 (2021-09-08)

## v0.13.1 (2021-09-07)

## v0.13.0 (2021-08-27)

## v0.12.2 (2021-08-17)

## v0.12.1 (2021-08-13)

## v0.12.0 (2021-08-12)

## v0.11.0 (2021-08-11)

## v0.10.0 (2021-08-10)

## v0.9.0 (2021-05-09)

## v0.8.0 (2021-04-30)

## v0.7.0 (2021-04-08)

## v0.6.0 (2020-12-16)

## v0.5.0 (2020-12-15)

## v0.4.0 (2020-09-12)

## v0.3.0 (2020-08-12)

<csr-id-5d57c1f7e3b9a84f7b46a4378015572155f3104b/>
<csr-id-3b9d66c932075feb08cdf2967f7698daef9fd3ff/>

### Refactor

 - <csr-id-5d57c1f7e3b9a84f7b46a4378015572155f3104b/> Use borrowed::Id in trees for full type safety
 - <csr-id-3b9d66c932075feb08cdf2967f7698daef9fd3ff/> make reusing round-trip code easier

## v0.1.0 (2020-07-12)

<csr-id-ec3be19c8d007565b814b4757f17811ec0e9de2c/>

### Refactor

 - <csr-id-ec3be19c8d007565b814b4757f17811ec0e9de2c/> Prefer integration level tests, but use unit-tests where appropriate

