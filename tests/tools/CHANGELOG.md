# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.15.0 (2024-06-23)

Now by default, `tar` files will be written which works better when checking them into
Git. Those who need the previous behaviour, can use the `xz` feature instead.

### New Features (BREAKING)

 - <csr-id-55382c0aa6f04a3bb689299c613df2a39f261289/> make `xz2` optional to write uncompressed tar files by default.
   Previously, compression was beneficial due to storage in `git-lfs`.
   Now, storing (mostly) non-binary files is actually better moving forward.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update changelog prior to `gix-testtools` release ([`88eb841`](https://github.com/Byron/gitoxide/commit/88eb841b7e3991b6f228390433f58333c6d4b85f))
    - Merge branch 'tar-only' ([`1dfa90d`](https://github.com/Byron/gitoxide/commit/1dfa90d641306b4099a6ecd52e2056b231467807))
    - Make `xz2` optional to write uncompressed tar files by default. ([`55382c0`](https://github.com/Byron/gitoxide/commit/55382c0aa6f04a3bb689299c613df2a39f261289))
</details>

## 0.14.0 (2024-06-22)

A maintenance release with updated dependencies, and possibly minor improvements.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 175 calendar days.
 - 176 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-testtools v0.14.0 ([`f6eaba3`](https://github.com/Byron/gitoxide/commit/f6eaba3a465720819f2ef7844648699195dd8341))
    - Prepare changelog prior to release of `gix-testtools` ([`ae012ba`](https://github.com/Byron/gitoxide/commit/ae012ba8627a8ea2f0145c53c42972db1d875662))
    - Merge branch 'gix-testtools-license-file' ([`c50af65`](https://github.com/Byron/gitoxide/commit/c50af65bf84963cb430538915f8eca9bd4481012))
    - Update `gix-testtools` dependencies to the latest version. ([`ddaacda`](https://github.com/Byron/gitoxide/commit/ddaacda6667092ccea100180f70d2ccb9bb79dbc))
    - Add include directive to test-tools, to avoid CHANGELOG.md ([`c668bdd`](https://github.com/Byron/gitoxide/commit/c668bdd0b29f513df75c96f09c84abac612b8e6a))
    - Fix missing license files in the released gix-testtools crate ([`ef73ed4`](https://github.com/Byron/gitoxide/commit/ef73ed4644855ba53cc2b73674ee8df51ae7c053))
    - Merge branch 'main' into config-key-take-2 ([`9fa1054`](https://github.com/Byron/gitoxide/commit/9fa1054a01071180d7b08c8c2b5bd61e9d0d32da))
    - Release gix-fs v0.11.1, gix-glob v0.16.3 ([`2cefe77`](https://github.com/Byron/gitoxide/commit/2cefe77203131878d0d8f5346f20f0e25b76cbea))
    - Release gix-features v0.38.2, gix-actor v0.31.2, gix-validate v0.8.5, gix-object v0.42.2, gix-command v0.3.7, gix-filter v0.11.2, gix-fs v0.11.0, gix-revwalk v0.13.1, gix-traverse v0.39.1, gix-worktree-stream v0.13.0, gix-archive v0.13.0, gix-tempfile v14.0.0, gix-lock v14.0.0, gix-ref v0.44.0, gix-config v0.37.0, gix-prompt v0.8.5, gix-index v0.33.0, gix-worktree v0.34.0, gix-diff v0.44.0, gix-discover v0.32.0, gix-pathspec v0.7.5, gix-dir v0.5.0, gix-macros v0.1.5, gix-mailmap v0.23.1, gix-negotiate v0.13.1, gix-pack v0.51.0, gix-odb v0.61.0, gix-transport v0.42.1, gix-protocol v0.45.1, gix-revision v0.27.1, gix-status v0.10.0, gix-submodule v0.11.0, gix-worktree-state v0.11.0, gix v0.63.0, gitoxide-core v0.38.0, gitoxide v0.36.0, safety bump 19 crates ([`4f98e94`](https://github.com/Byron/gitoxide/commit/4f98e94e0e8b79ed2899b35bef40f3c30b3025b0))
    - Merge pull request #1365 from EliahKagan/no-fixture-lfs ([`c4e0a82`](https://github.com/Byron/gitoxide/commit/c4e0a82ec9948ed0d9876a50b156d1ba30e2ad77))
    - Remove LFS CI step and modify docs/comments ([`3283445`](https://github.com/Byron/gitoxide/commit/3283445424566579e2c2b31731f82d3f948a5c48))
    - Release gix-trace v0.1.9, gix-utils v0.1.12, gix-packetline-blocking v0.17.4, gix-filter v0.11.1, gix-fs v0.10.2, gix-traverse v0.39.0, gix-worktree-stream v0.12.0, gix-archive v0.12.0, gix-config v0.36.1, gix-url v0.27.3, gix-index v0.32.0, gix-worktree v0.33.0, gix-diff v0.43.0, gix-pathspec v0.7.3, gix-dir v0.4.0, gix-pack v0.50.0, gix-odb v0.60.0, gix-transport v0.42.0, gix-protocol v0.45.0, gix-status v0.9.0, gix-worktree-state v0.10.0, gix v0.62.0, gix-fsck v0.4.0, gitoxide-core v0.37.0, gitoxide v0.35.0, safety bump 14 crates ([`095c673`](https://github.com/Byron/gitoxide/commit/095c6739b2722a8b9af90776b435ef2da454c0e6))
    - Release gix-date v0.8.5, gix-hash v0.14.2, gix-trace v0.1.8, gix-utils v0.1.11, gix-features v0.38.1, gix-actor v0.31.0, gix-validate v0.8.4, gix-object v0.42.0, gix-path v0.10.7, gix-glob v0.16.2, gix-quote v0.4.12, gix-attributes v0.22.2, gix-command v0.3.6, gix-filter v0.11.0, gix-fs v0.10.1, gix-chunk v0.4.8, gix-commitgraph v0.24.2, gix-hashtable v0.5.2, gix-revwalk v0.13.0, gix-traverse v0.38.0, gix-worktree-stream v0.11.0, gix-archive v0.11.0, gix-config-value v0.14.6, gix-tempfile v13.1.1, gix-lock v13.1.1, gix-ref v0.43.0, gix-sec v0.10.6, gix-config v0.36.0, gix-prompt v0.8.4, gix-url v0.27.2, gix-credentials v0.24.2, gix-ignore v0.11.2, gix-bitmap v0.2.11, gix-index v0.31.0, gix-worktree v0.32.0, gix-diff v0.42.0, gix-discover v0.31.0, gix-pathspec v0.7.1, gix-dir v0.2.0, gix-macros v0.1.4, gix-mailmap v0.23.0, gix-negotiate v0.13.0, gix-pack v0.49.0, gix-odb v0.59.0, gix-packetline v0.17.4, gix-transport v0.41.2, gix-protocol v0.44.2, gix-revision v0.27.0, gix-refspec v0.23.0, gix-status v0.7.0, gix-submodule v0.10.0, gix-worktree-state v0.9.0, gix v0.60.0, safety bump 26 crates ([`b050327`](https://github.com/Byron/gitoxide/commit/b050327e76f234b19be921b78b7b28e034319fdb))
    - Merge pull request #1290 from epage/winnow ([`a663e9f`](https://github.com/Byron/gitoxide/commit/a663e9fcdb5a3aedc9200da77ebae17d5c3e7135))
    - Update winnow to 0.6 ([`e175b20`](https://github.com/Byron/gitoxide/commit/e175b20d431faa6859fbcc52f78400e50f91cad1))
    - Use winnow BStr ([`47d0374`](https://github.com/Byron/gitoxide/commit/47d0374e86cab4d498d955ac73bd7468cd5fcda9))
    - Update winnow to 0.5.40 ([`516e105`](https://github.com/Byron/gitoxide/commit/516e105db5f22e1483b4b8a886cc4f3929ad7f6a))
    - Merge pull request #1267 from epage/winnow ([`69cb78b`](https://github.com/Byron/gitoxide/commit/69cb78bd865a372c580b386766d7b61e5ca9303a))
    - Update from winnow 0.5.31 to 0.5.36 ([`9470554`](https://github.com/Byron/gitoxide/commit/94705546cf0e4c8e38bcc96999cfa79cd8ee1acd))
    - Release gix-utils v0.1.9, gix-features v0.38.0, gix-actor v0.30.0, gix-object v0.41.0, gix-path v0.10.4, gix-glob v0.16.0, gix-attributes v0.22.0, gix-command v0.3.3, gix-packetline-blocking v0.17.3, gix-filter v0.9.0, gix-fs v0.10.0, gix-commitgraph v0.24.0, gix-revwalk v0.12.0, gix-traverse v0.37.0, gix-worktree-stream v0.9.0, gix-archive v0.9.0, gix-config-value v0.14.4, gix-tempfile v13.0.0, gix-lock v13.0.0, gix-ref v0.41.0, gix-sec v0.10.4, gix-config v0.34.0, gix-url v0.27.0, gix-credentials v0.24.0, gix-ignore v0.11.0, gix-index v0.29.0, gix-worktree v0.30.0, gix-diff v0.40.0, gix-discover v0.29.0, gix-mailmap v0.22.0, gix-negotiate v0.12.0, gix-pack v0.47.0, gix-odb v0.57.0, gix-pathspec v0.6.0, gix-packetline v0.17.3, gix-transport v0.41.0, gix-protocol v0.44.0, gix-revision v0.26.0, gix-refspec v0.22.0, gix-status v0.5.0, gix-submodule v0.8.0, gix-worktree-state v0.7.0, gix v0.58.0, safety bump 39 crates ([`eb6aa8f`](https://github.com/Byron/gitoxide/commit/eb6aa8f502314f886fc4ea3d52ab220763968208))
    - Release gix-date v0.8.3, gix-hash v0.14.1, gix-trace v0.1.6, gix-features v0.37.1, gix-actor v0.29.1, gix-validate v0.8.3, gix-object v0.40.1, gix-path v0.10.3, gix-glob v0.15.1, gix-quote v0.4.10, gix-attributes v0.21.1, gix-command v0.3.2, gix-packetline-blocking v0.17.2, gix-utils v0.1.8, gix-filter v0.8.1, gix-fs v0.9.1, gix-chunk v0.4.7, gix-commitgraph v0.23.1, gix-hashtable v0.5.1, gix-revwalk v0.11.1, gix-traverse v0.36.1, gix-worktree-stream v0.8.1, gix-archive v0.8.1, gix-config-value v0.14.3, gix-tempfile v12.0.1, gix-lock v12.0.1, gix-ref v0.40.1, gix-sec v0.10.3, gix-config v0.33.1, gix-prompt v0.8.2, gix-url v0.26.1, gix-credentials v0.23.1, gix-ignore v0.10.1, gix-bitmap v0.2.10, gix-index v0.28.1, gix-worktree v0.29.1, gix-diff v0.39.1, gix-discover v0.28.1, gix-macros v0.1.3, gix-mailmap v0.21.1, gix-negotiate v0.11.1, gix-pack v0.46.1, gix-odb v0.56.1, gix-pathspec v0.5.1, gix-packetline v0.17.2, gix-transport v0.40.1, gix-protocol v0.43.1, gix-revision v0.25.1, gix-refspec v0.21.1, gix-status v0.4.1, gix-submodule v0.7.1, gix-worktree-state v0.6.1, gix v0.57.1 ([`972241f`](https://github.com/Byron/gitoxide/commit/972241f1904944e8b6e84c6aa1649a49be7a85c3))
    - Fixup `gix-testtools` manifest to allow releasing all other crates. ([`9587972`](https://github.com/Byron/gitoxide/commit/95879729c411337cb5f6f5fd699b8a6d61e83a78))
</details>

## 0.13.0 (2023-12-29)

<csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/>
<csr-id-7f7db9794c23b87c8ea50b7bcf38955c9d977624/>
<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-ef54aab9e5521add4154ee8d902d62612a9d8d4a/> switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance.
   It's likely that over time, these parsers will get even faster due to improvements to `winnow`.
   Thanks, Ed Page, for single-handedly performing this transition.
 - <csr-id-7f7db9794c23b87c8ea50b7bcf38955c9d977624/> curtail `bstr` features to exactly what's needed.
 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### New Features

 - <csr-id-06d4682c9fed696fee09234223814016a6453a6d/> write informative message if archives are ignored due to script change.

### Bug Fixes

 - <csr-id-11cb4317c75864bf310f4964edba7cf487a604f9/> better debug output when fixture script script fails
 - <csr-id-ed1407c85525a524bcfa0a4a021a22de339e6149/> don't let scripts run on (potentially) partially extracted archives and don't create archives on windows
   This could cause unrelated failures which didn't help debugging at all.
   Also improve error messages when trying to delete a stale/incorrect script
   output directory fails.
   This can easily happen on windows when symlinks are involved.
   
   We also stop creating archives on windows as the archive-metadata can't be deleted for some
   reason, which means it stays in the created directory and may cause script failures.
   This seems acceptable as windows users simply won't be able to create their own archives for accelration,
   but could probably use WSL for it.

### Bug Fixes (BREAKING)

 - <csr-id-2189cee47f99350b368390eaa2a01961bb77c250/> rename `GITOXIDE_*` environment variables to `GIX_#`
 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 50 commits contributed to the release over the course of 218 calendar days.
 - 243 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#960](https://github.com/Byron/gitoxide/issues/960)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#960](https://github.com/Byron/gitoxide/issues/960)**
    - Try to prevent 'git' reading user and system configuration ([`33be0e0`](https://github.com/Byron/gitoxide/commit/33be0e032e4802b0d8e261cccf19ecb9244ffb36))
 * **Uncategorized**
    - Release gix-testtools v0.13.0 ([`edb12ff`](https://github.com/Byron/gitoxide/commit/edb12ff7216fdc9b0cf9960949b5794e6fa3cd08))
    - Set `gix-testtools` package versions so that it can be released ([`793c2af`](https://github.com/Byron/gitoxide/commit/793c2afbb5e4a5c817b06c15103310781769abfd))
    - Release gix-testtools v0.13.0 ([`7fe620d`](https://github.com/Byron/gitoxide/commit/7fe620d6135e7619c5484aeca7ca8823bee585e1))
    - Release gix-date v0.8.2, gix-hash v0.14.0, gix-trace v0.1.5, gix-features v0.37.0, gix-actor v0.29.0, gix-validate v0.8.2, gix-object v0.40.0, gix-path v0.10.2, gix-glob v0.15.0, gix-quote v0.4.9, gix-attributes v0.21.0, gix-command v0.3.1, gix-packetline-blocking v0.17.1, gix-utils v0.1.7, gix-filter v0.8.0, gix-fs v0.9.0, gix-chunk v0.4.6, gix-commitgraph v0.23.0, gix-hashtable v0.5.0, gix-revwalk v0.11.0, gix-traverse v0.36.0, gix-worktree-stream v0.8.0, gix-archive v0.8.0, gix-config-value v0.14.2, gix-tempfile v12.0.0, gix-lock v12.0.0, gix-ref v0.40.0, gix-sec v0.10.2, gix-config v0.33.0, gix-prompt v0.8.1, gix-url v0.26.0, gix-credentials v0.23.0, gix-ignore v0.10.0, gix-bitmap v0.2.9, gix-index v0.28.0, gix-worktree v0.29.0, gix-diff v0.39.0, gix-discover v0.28.0, gix-macros v0.1.2, gix-mailmap v0.21.0, gix-negotiate v0.11.0, gix-pack v0.46.0, gix-odb v0.56.0, gix-pathspec v0.5.0, gix-packetline v0.17.1, gix-transport v0.40.0, gix-protocol v0.43.0, gix-revision v0.25.0, gix-refspec v0.21.0, gix-status v0.4.0, gix-submodule v0.7.0, gix-worktree-state v0.6.0, gix v0.57.0, gix-fsck v0.2.0, gitoxide-core v0.35.0, gitoxide v0.33.0, safety bump 40 crates ([`e1aae19`](https://github.com/Byron/gitoxide/commit/e1aae191d7421c748913c92e2c5883274331dd20))
    - Merge branch 'maintenance' ([`4454c9d`](https://github.com/Byron/gitoxide/commit/4454c9d66c32a1de75a66639016c73edbda3bd34))
    - Upgrade testtools dependencies to latest feasible version ([`29c5904`](https://github.com/Byron/gitoxide/commit/29c59045425dcfc23392e0b766d6d6ca399b00a4))
    - Merge branch 'main' into fix-1183 ([`1691ba6`](https://github.com/Byron/gitoxide/commit/1691ba669537f4a39ebb0891747dc509a6aedbef))
    - Merge branch 'archive-handling' ([`7549559`](https://github.com/Byron/gitoxide/commit/7549559fcbf42249939f41fd7aa34b4449eb1fec))
    - Write informative message if archives are ignored due to script change. ([`06d4682`](https://github.com/Byron/gitoxide/commit/06d4682c9fed696fee09234223814016a6453a6d))
    - Release gix-date v0.8.1, gix-hash v0.13.2, gix-trace v0.1.4, gix-features v0.36.1, gix-actor v0.28.1, gix-validate v0.8.1, gix-object v0.39.0, gix-path v0.10.1, gix-glob v0.14.1, gix-quote v0.4.8, gix-attributes v0.20.1, gix-command v0.3.0, gix-packetline-blocking v0.17.0, gix-utils v0.1.6, gix-filter v0.7.0, gix-fs v0.8.1, gix-chunk v0.4.5, gix-commitgraph v0.22.1, gix-hashtable v0.4.1, gix-revwalk v0.10.0, gix-traverse v0.35.0, gix-worktree-stream v0.7.0, gix-archive v0.7.0, gix-config-value v0.14.1, gix-tempfile v11.0.1, gix-lock v11.0.1, gix-ref v0.39.0, gix-sec v0.10.1, gix-config v0.32.0, gix-prompt v0.8.0, gix-url v0.25.2, gix-credentials v0.22.0, gix-ignore v0.9.1, gix-bitmap v0.2.8, gix-index v0.27.0, gix-worktree v0.28.0, gix-diff v0.38.0, gix-discover v0.27.0, gix-macros v0.1.1, gix-mailmap v0.20.1, gix-negotiate v0.10.0, gix-pack v0.45.0, gix-odb v0.55.0, gix-pathspec v0.4.1, gix-packetline v0.17.0, gix-transport v0.39.0, gix-protocol v0.42.0, gix-revision v0.24.0, gix-refspec v0.20.0, gix-status v0.3.0, gix-submodule v0.6.0, gix-worktree-state v0.5.0, gix v0.56.0, gix-fsck v0.1.0, gitoxide-core v0.34.0, gitoxide v0.32.0, safety bump 27 crates ([`55d386a`](https://github.com/Byron/gitoxide/commit/55d386a2448aba1dd22c73fb63b3fd5b3a8401c9))
    - Merge branch 'adjustments-for-cargo' ([`8156340`](https://github.com/Byron/gitoxide/commit/8156340724b1b7cb15824f88c75f6ddd7302cff5))
    - Rename `GITOXIDE_*` environment variables to `GIX_#` ([`2189cee`](https://github.com/Byron/gitoxide/commit/2189cee47f99350b368390eaa2a01961bb77c250))
    - Upgrade to `winnow` 0.5.24 ([`abcfb65`](https://github.com/Byron/gitoxide/commit/abcfb659786425ec09eff6b644cd2ad36b7d6bc4))
    - Release gix-hash v0.13.1, gix-features v0.36.0, gix-actor v0.28.0, gix-object v0.38.0, gix-glob v0.14.0, gix-attributes v0.20.0, gix-command v0.2.10, gix-filter v0.6.0, gix-fs v0.8.0, gix-commitgraph v0.22.0, gix-revwalk v0.9.0, gix-traverse v0.34.0, gix-worktree-stream v0.6.0, gix-archive v0.6.0, gix-tempfile v11.0.0, gix-lock v11.0.0, gix-ref v0.38.0, gix-config v0.31.0, gix-url v0.25.0, gix-credentials v0.21.0, gix-diff v0.37.0, gix-discover v0.26.0, gix-ignore v0.9.0, gix-index v0.26.0, gix-mailmap v0.20.0, gix-negotiate v0.9.0, gix-pack v0.44.0, gix-odb v0.54.0, gix-pathspec v0.4.0, gix-packetline v0.16.7, gix-transport v0.37.0, gix-protocol v0.41.0, gix-revision v0.23.0, gix-refspec v0.19.0, gix-worktree v0.27.0, gix-status v0.2.0, gix-submodule v0.5.0, gix-worktree-state v0.4.0, gix v0.55.0, safety bump 37 crates ([`68e5432`](https://github.com/Byron/gitoxide/commit/68e54326e527a55dd5b5079921fc251615833040))
    - Release gix-features v0.35.0, gix-actor v0.27.0, gix-object v0.37.0, gix-glob v0.13.0, gix-attributes v0.19.0, gix-filter v0.5.0, gix-fs v0.7.0, gix-commitgraph v0.21.0, gix-revwalk v0.8.0, gix-traverse v0.33.0, gix-worktree-stream v0.5.0, gix-archive v0.5.0, gix-tempfile v10.0.0, gix-lock v10.0.0, gix-ref v0.37.0, gix-config v0.30.0, gix-url v0.24.0, gix-credentials v0.20.0, gix-diff v0.36.0, gix-discover v0.25.0, gix-ignore v0.8.0, gix-index v0.25.0, gix-mailmap v0.19.0, gix-negotiate v0.8.0, gix-pack v0.43.0, gix-odb v0.53.0, gix-pathspec v0.3.0, gix-transport v0.37.0, gix-protocol v0.40.0, gix-revision v0.22.0, gix-refspec v0.18.0, gix-status v0.1.0, gix-submodule v0.4.0, gix-worktree v0.26.0, gix-worktree-state v0.3.0, gix v0.54.0, gitoxide-core v0.32.0, gitoxide v0.30.0, safety bump 37 crates ([`7891fb1`](https://github.com/Byron/gitoxide/commit/7891fb17348ec2f4c997665f9a25be36e2713da4))
    - Release gix-date v0.8.0, gix-hash v0.13.0, gix-features v0.34.0, gix-actor v0.26.0, gix-object v0.36.0, gix-path v0.10.0, gix-glob v0.12.0, gix-attributes v0.18.0, gix-packetline-blocking v0.16.6, gix-filter v0.4.0, gix-fs v0.6.0, gix-commitgraph v0.20.0, gix-hashtable v0.4.0, gix-revwalk v0.7.0, gix-traverse v0.32.0, gix-worktree-stream v0.4.0, gix-archive v0.4.0, gix-config-value v0.14.0, gix-tempfile v9.0.0, gix-lock v9.0.0, gix-ref v0.36.0, gix-sec v0.10.0, gix-config v0.29.0, gix-prompt v0.7.0, gix-url v0.23.0, gix-credentials v0.19.0, gix-diff v0.35.0, gix-discover v0.24.0, gix-ignore v0.7.0, gix-index v0.24.0, gix-macros v0.1.0, gix-mailmap v0.18.0, gix-negotiate v0.7.0, gix-pack v0.42.0, gix-odb v0.52.0, gix-pathspec v0.2.0, gix-packetline v0.16.6, gix-transport v0.36.0, gix-protocol v0.39.0, gix-revision v0.21.0, gix-refspec v0.17.0, gix-submodule v0.3.0, gix-worktree v0.25.0, gix-worktree-state v0.2.0, gix v0.53.0, safety bump 39 crates ([`8bd0456`](https://github.com/Byron/gitoxide/commit/8bd045676bb2cdc02624ab93e73ff8518064ca38))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Switch `nom` to `winnow` in remaining uses in `gix-object`, `gix-ref`, and `gix-actor` for ~20% more performance. ([`ef54aab`](https://github.com/Byron/gitoxide/commit/ef54aab9e5521add4154ee8d902d62612a9d8d4a))
    - Refactor and fixes ([`02587fc`](https://github.com/Byron/gitoxide/commit/02587fc879c54b2b3e62ffbe1ab4c29591ea0d80))
    - Upgrade `winnow` to latest patch release ([`8c41848`](https://github.com/Byron/gitoxide/commit/8c4184817e4e4364c34badc8ff0a71c6ae952efd))
    - Switch errors to StrContext ([`df226dd`](https://github.com/Byron/gitoxide/commit/df226dd31df2c591c6470ed70098202112e13dae))
    - Show more error details in parse tests failures ([`266864f`](https://github.com/Byron/gitoxide/commit/266864f35dc9ee96b81d22281c8f267fd7c059a4))
    - Minor cleanup possible with 0.5 ([`a07590c`](https://github.com/Byron/gitoxide/commit/a07590cb46423cb0422c18b9fc04b153c0fd53b1))
    - Upgrade to Winnow 0.5 ([`3f8c91f`](https://github.com/Byron/gitoxide/commit/3f8c91fa463fbb53d54b2bf359e0dee7387afa00))
    - Upgrade to Winnow 0.4 ([`86ea47f`](https://github.com/Byron/gitoxide/commit/86ea47f28079c51f874b0d662867040b92f88d14))
    - Resolve remaining winnow 0.3 deprecations ([`fee441d`](https://github.com/Byron/gitoxide/commit/fee441da875d52b1a0cb557d2fa58cee9c29e16a))
    - Switch gix to winnow 0.3 ([`ee75de1`](https://github.com/Byron/gitoxide/commit/ee75de1e6035305fc23bdef2522ae5081272ac82))
    - Merge branch 'limit-git' ([`68d9e80`](https://github.com/Byron/gitoxide/commit/68d9e809d4e746fd7beaddeabd3313d59a4cbdfd))
    - Merge branch 'dev-on-linux' ([`6b4a303`](https://github.com/Byron/gitoxide/commit/6b4a30330fe49fc97daa73f55bf56580cc0597aa))
    - Better debug output when fixture script script fails ([`11cb431`](https://github.com/Byron/gitoxide/commit/11cb4317c75864bf310f4964edba7cf487a604f9))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Adjust package versions (by cargo-smart-release) ([`c70e54f`](https://github.com/Byron/gitoxide/commit/c70e54f163c312c87753a506eeaad462e8579bfb))
    - Merge branch 'integrate-filtering' ([`b19a56d`](https://github.com/Byron/gitoxide/commit/b19a56dcfa9bea86332a84aa4e8fad445e7d1724))
    - Don't let scripts run on (potentially) partially extracted archives and don't create archives on windows ([`ed1407c`](https://github.com/Byron/gitoxide/commit/ed1407c85525a524bcfa0a4a021a22de339e6149))
    - Curtail `bstr` features to exactly what's needed. ([`7f7db97`](https://github.com/Byron/gitoxide/commit/7f7db9794c23b87c8ea50b7bcf38955c9d977624))
    - Upgrade memmap2 and fastrand dependencies ([`6fc7497`](https://github.com/Byron/gitoxide/commit/6fc74971ac6838cbfd9c869ba3746713001d7a38))
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Allow gix-testtools to refer to the local crates that don't cause cycles. ([`082a6fc`](https://github.com/Byron/gitoxide/commit/082a6fc65ae08ea0fda11a3340941d58ead4036a))
    - Merge branch 'fix-docs' ([`420553a`](https://github.com/Byron/gitoxide/commit/420553a10d780e0b2dc466cac120989298a5f187))
    - Cleaning up documentation ([`2578e57`](https://github.com/Byron/gitoxide/commit/2578e576bfa365d194a23a1fb0bf09be230873de))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`2087032`](https://github.com/Byron/gitoxide/commit/2087032b5956dcd82bce6ac57e530e8724b57f17))
    - Auto-fix clippy to remove explicit iter looping ([`3eff567`](https://github.com/Byron/gitoxide/commit/3eff567c683b5c650c14792b68968cbdbc90ec5c))
</details>

## 0.12.0 (2023-04-29)

<csr-id-b973f19274bb2d8218e5ff63ce0a81f34985f54c/>

### Chore

 - <csr-id-b973f19274bb2d8218e5ff63ce0a81f34985f54c/> upgrade dependencies

### Documentation

 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 61 calendar days.
 - 68 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-discover v0.18.1, gix-worktree v0.17.1, gix-testtools v0.12.0 ([`f7b6c6f`](https://github.com/Byron/gitoxide/commit/f7b6c6f27c090cbc584fbd3f5403da5ac1a9ff02))
    - Prepare changelogs prior to release of gix-testtools ([`fc45f1b`](https://github.com/Byron/gitoxide/commit/fc45f1b417bf545d4a0a105c40b37f92c24decad))
    - Upgrade dependencies ([`b973f19`](https://github.com/Byron/gitoxide/commit/b973f19274bb2d8218e5ff63ce0a81f34985f54c))
    - Accept paths in scripted_fixture_writable ([`efcbf0d`](https://github.com/Byron/gitoxide/commit/efcbf0d1cb1c9d77eaf04fbcf6e86dc101c886d2))
    - Add note about shortcomings of `Creation::CopyFromReadOnly` mode. ([`b2e3223`](https://github.com/Byron/gitoxide/commit/b2e322332ac017824e90d260d7041504c38ab57f))
    - Fix minor typos ([`cc48c35`](https://github.com/Byron/gitoxide/commit/cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5))
    - Prepare for git-tempfile release ([`56c005b`](https://github.com/Byron/gitoxide/commit/56c005b13c44376f71e61781e73c0bf93416d0e4))
</details>

## 0.11.0 (2023-02-20)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>
<csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/>
<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-fb68bffcb26582d508db946f72234bfd847a3a11/> `set_current_dir()` to change the CWD and reset it to previous version on drop.
 - <csr-id-15ecd841cfe7c77bbdfdfa232dd51a44c4940bbc/> allow execution of scripts without 'bash'.
   This works by trying to execute the file directly, and on failure, use 'bash'
   as interpreter.
   
   That way we are finally able to support a wider variety of fixture generators
   and make the crate more useful to a wieder audience.
 - <csr-id-221f1374aa004a76693cfb1529daab930a5a9dd7/> `spawn_git_daemon()` to spawn a git daemon hosting a working directoy…
   …with support for multiple at a time thanks to port selection (allowing
   tests to run in parallel) as well as auto-kill on drop.
 - <csr-id-67777a81f8d9d0335475e4fe4cbf770c328bd24f/> increase the waiting time on MacOS for file base locks
   It appears that these workers run into timeouts more and more,
   they got slower or maybe there are just more tests.
 - <csr-id-09da4c5eeff5c6657beb9c53c168f90e74d6f758/> add `Env::unset()` for convenience
 - <csr-id-231785644194cd3be0b0dab06224a39ecf0ed714/> Provide `GIT_VERSION` information along with a way to skip a test based on it.
 - <csr-id-654b521323a5822cbb86e57bee159d90576fa5ff/> expose `on_ci` in the top-level.
 - <csr-id-449b6c1555fc2832c712ba51cd41ab9ed79e0b15/> Allow to re-execute scripts into temp directories.
   This is important in cases where the files created by the script
   contain absolute mentions of locations. That way, when copying
   files over, the test might accidentally return to the original
   read-only location, and write into it making future test runs fail.
 - <csr-id-f1635c3ee36678cff9f26135946c281bf4a75331/> publicly accessible `Result` type

### Bug Fixes

 - <csr-id-00286c9cf63b5eba9534ef7639805545ec40eb03/> `gix-testtools` use the latest dependencies
   These should compile properly.
 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-761b7d71977a5aa4876010faa61ab88f0dba6eab/> don't overwrite unexpanded `git-lfs` pointer files.
   It's possible for those with incomplete `git-lfs` installations
   (and many more situations) to end up in a spot where pointer files
   aren't expanded. If we overwrite the with archives, files look
   changed which can be confusing and lead to even bigger messes
   to happen.
   
   Now we don't overwrite those files anyomre.
 - <csr-id-1ce3190000f6211ce31468c7603d491bb5b90293/> Disable tag.gpgSign in test scripts
   This is done for the same reason that commit.gpgsign is disabled for test
   scripts. It prevents test failures if the user has tag.gpgsign enabled in
   their global git config when invoking tests.
 - <csr-id-cba9edeb403aae4d77087de4167cbabe72525d92/> Allow multiple scripts to run at the same time, if they are not the same.
   Previously, per integration test and thus per crate, one would
   effectively only be able to run a single script at a time because of the
   global identity lock. This was required previously before the additional
   file based lock, per script name, was introduced.
   
   This is now fixed by dropping the lock after the script identity was
   obtained.
 - <csr-id-004dab17deab4c360adb5ac428f6b4951c974fe3/> `_with_args(…)` functions now allow non-static strings

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

### Changed (BREAKING)

 - <csr-id-dbf6c8c87cdca8169ac01aa89aefe56a33215142/> rename `scripted_fixture_*` to not contain 'repo' in the name.
   Further make clear in the documentation that `bash` is used to execute
   the fixture scripts, previously it wasn't even implied and got
   lost in history.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features (BREAKING)

 - <csr-id-21bd6075ca36ca49b3c85d0431ec11f68e6e9f9c/> remove `hex_to_id()` and add various `*_standalone()` versions of existing methods.
   With these it's possible to handle fully standalone (as in with their own Cargo.toml) integration
   tests which are needed to resolve cyclic dependencies.
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

 - 247 commits contributed to the release over the course of 938 calendar days.
 - 23 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 20 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#266](https://github.com/Byron/gitoxide/issues/266), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366), [#382](https://github.com/Byron/gitoxide/issues/382), [#384](https://github.com/Byron/gitoxide/issues/384), [#391](https://github.com/Byron/gitoxide/issues/391), [#393](https://github.com/Byron/gitoxide/issues/393), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#488](https://github.com/Byron/gitoxide/issues/488), [#509](https://github.com/Byron/gitoxide/issues/509), [#607](https://github.com/Byron/gitoxide/issues/607), [#650](https://github.com/Byron/gitoxide/issues/650), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 11 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix windows tests by transforming line endings ([`e276d77`](https://github.com/Byron/gitoxide/commit/e276d777eb7a88dc424badbf88a929b5f567e5de))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - A failing test to show the handle-stability doesn't quite work yet ([`5562e88`](https://github.com/Byron/gitoxide/commit/5562e8888cd8ac8fc3d89a41f8e8cc5cec7b8ca6))
    - Refactor ([`c499843`](https://github.com/Byron/gitoxide/commit/c499843485a8af102cb4d3594c4e6014976c5aa0))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - Use parking_lot mutex to avoid poison errors ([`d8ca74f`](https://github.com/Byron/gitoxide/commit/d8ca74f358e802916353f545b90127f9a7bb5137))
    - Base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
    - Upgrade parking_lot and cargo_toml ([`f95c1a0`](https://github.com/Byron/gitoxide/commit/f95c1a0d9c19bcc6feb9b8739a09d86f9970a0e0))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Allow to re-execute scripts into temp directories. ([`449b6c1`](https://github.com/Byron/gitoxide/commit/449b6c1555fc2832c712ba51cd41ab9ed79e0b15))
    - Don't print archive message if archive is excluded ([`c6bd30e`](https://github.com/Byron/gitoxide/commit/c6bd30e81997931d1f65a62924d20fe5e74b8521))
    - Support unique directories for different platforms ([`0b385b3`](https://github.com/Byron/gitoxide/commit/0b385b31cf95d500f9ec2d05be0894956e40e4a1))
    - Use git exclude information to determine if archives should be generated ([`4a3dccc`](https://github.com/Byron/gitoxide/commit/4a3dccc7fc7a5e190d88af8c7eb0713edbada55f))
    - Add TODO ([`778fd77`](https://github.com/Byron/gitoxide/commit/778fd7703920e6a2693beb59aad611f3c9fab106))
    - Publicly accessible `Result` type ([`f1635c3`](https://github.com/Byron/gitoxide/commit/f1635c3ee36678cff9f26135946c281bf4a75331))
    - Refactor ([`9ea1e44`](https://github.com/Byron/gitoxide/commit/9ea1e4474a3ce803da7a56e1fc1748f65c11a876))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Expose `on_ci` in the top-level. ([`654b521`](https://github.com/Byron/gitoxide/commit/654b521323a5822cbb86e57bee159d90576fa5ff))
    - Move `Env` test utility into `git-testtools` ([`bd3f4d0`](https://github.com/Byron/gitoxide/commit/bd3f4d014dd7df7a1e25defa8eea7253eec1560a))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - Add test-tools changelog prior to release ([`1ebc16a`](https://github.com/Byron/gitoxide/commit/1ebc16a6ac9ef188c188a52737820773aa949cee))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - Quickfix for unintentionally using 'unicode' feature of bytecode ([`fb5593a`](https://github.com/Byron/gitoxide/commit/fb5593a7272498ae042b6c8c7605faa3d253fa10))
 * **[#382](https://github.com/Byron/gitoxide/issues/382)**
    - Simplify state tests ([`fc61c0d`](https://github.com/Byron/gitoxide/commit/fc61c0d4f7cb3cd9073418e4d8edc55cd14f5fb3))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - Enforce signal handler setup to cleanup tempfiles on abort ([`1caf3ae`](https://github.com/Byron/gitoxide/commit/1caf3ae2cabee776dc45a687f00ce386c27ab87d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - Provide some more information when using archives; debug windows more ([`4f5b1fd`](https://github.com/Byron/gitoxide/commit/4f5b1fd5e6440208c460388a9d69d664d6d8d0d7))
    - Protect test generation from multi-process races ([`1aec924`](https://github.com/Byron/gitoxide/commit/1aec924f009fd16b953cd1313b9408558b1c7aeb))
    - Definitely don't follow symlnks ([`1343448`](https://github.com/Byron/gitoxide/commit/13434481c44efbc170cb74dd9057807c3ee58e01))
    - Make sure existing files aren't written into ([`9b5a8a2`](https://github.com/Byron/gitoxide/commit/9b5a8a243d49b6567d1db31050d3bf3123dd54d3))
    - Extraction of tar archives with identity check ([`07c1f07`](https://github.com/Byron/gitoxide/commit/07c1f0752fefbd3e49ef414bced2ca6bbc844448))
    - Assure there are no archive file-name clashes across crates ([`c30bebf`](https://github.com/Byron/gitoxide/commit/c30bebf4f0272fe728e18b1932e419128f63ed44))
    - Actual compression of archives ([`5dd3d82`](https://github.com/Byron/gitoxide/commit/5dd3d82aa68c9024cd1742043a3c56cd6b0665fd))
    - Simple creation of test-archives ([`f1e107a`](https://github.com/Byron/gitoxide/commit/f1e107aa864107e02309b15b41da8d8f962e19a6))
    - Make sure archives are handled by git-lfs ([`f744a6c`](https://github.com/Byron/gitoxide/commit/f744a6cc8b453ea349664540af4be0566e376528))
    - Frame for extracting and generating archives ([`92c7044`](https://github.com/Byron/gitoxide/commit/92c7044cfbc3054b237ea7c79da981bb91908812))
    - Further partition generated test directories by script name ([`e141ddb`](https://github.com/Byron/gitoxide/commit/e141ddbdd2e0677e921856b30096733530fde569))
    - Auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#391](https://github.com/Byron/gitoxide/issues/391)**
    - Also write a failure marker if archive creation failed ([`7f88c7f`](https://github.com/Byron/gitoxide/commit/7f88c7f9d908df39ad4e710402783dca35eb758f))
    - Auto-clean test fixtures on re-run if they failed previously ([`3617ff4`](https://github.com/Byron/gitoxide/commit/3617ff411224a691057eb1c39c4144b932b33f51))
 * **[#393](https://github.com/Byron/gitoxide/issues/393)**
    - Add support for disabling archive usage ([`624ad2e`](https://github.com/Byron/gitoxide/commit/624ad2ef42172556efe942129f6f46dd627250d5))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Add `Env::unset()` for convenience ([`09da4c5`](https://github.com/Byron/gitoxide/commit/09da4c5eeff5c6657beb9c53c168f90e74d6f758))
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - Allow multiple scripts to run at the same time, if they are not the same. ([`cba9ede`](https://github.com/Byron/gitoxide/commit/cba9edeb403aae4d77087de4167cbabe72525d92))
    - Make tests more robust; fix windows tests ([`1983fbc`](https://github.com/Byron/gitoxide/commit/1983fbc39be3da5598cf3af6fb97f6ea0bc3ec6b))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#488](https://github.com/Byron/gitoxide/issues/488)**
    - Provide `GIT_VERSION` information along with a way to skip a test based on it. ([`2317856`](https://github.com/Byron/gitoxide/commit/231785644194cd3be0b0dab06224a39ecf0ed714))
 * **[#509](https://github.com/Byron/gitoxide/issues/509)**
    - Some unit tests for the time when something truly unparseable shows up ([`94fc0d6`](https://github.com/Byron/gitoxide/commit/94fc0d60d21c22a0d36f5de986cd9443755141bf))
    - Be more verbose when git version parsing fails ([`9c2f1b5`](https://github.com/Byron/gitoxide/commit/9c2f1b5d03fbcf5dd08c2469ea17da426ea6670c))
 * **[#607](https://github.com/Byron/gitoxide/issues/607)**
    - Don't overwrite unexpanded `git-lfs` pointer files. ([`761b7d7`](https://github.com/Byron/gitoxide/commit/761b7d71977a5aa4876010faa61ab88f0dba6eab))
    - Improve documentation to inform about the need for `git-lfs`. ([`519db50`](https://github.com/Byron/gitoxide/commit/519db50eac6576906f266a6f0b980f88098e3f9f))
 * **[#650](https://github.com/Byron/gitoxide/issues/650)**
    - Allow execution of scripts without 'bash'. ([`15ecd84`](https://github.com/Byron/gitoxide/commit/15ecd841cfe7c77bbdfdfa232dd51a44c4940bbc))
    - Rename `scripted_fixture_*` to not contain 'repo' in the name. ([`dbf6c8c`](https://github.com/Byron/gitoxide/commit/dbf6c8c87cdca8169ac01aa89aefe56a33215142))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - `_with_args(…)` functions now allow non-static strings ([`004dab1`](https://github.com/Byron/gitoxide/commit/004dab17deab4c360adb5ac428f6b4951c974fe3))
 * **Uncategorized**
    - Release gix-testtools v0.11.0 ([`dfe2402`](https://github.com/Byron/gitoxide/commit/dfe24026f9b1d85b8ab01e69dfec6a4188091850))
    - `gix-testtools` use the latest dependencies ([`00286c9`](https://github.com/Byron/gitoxide/commit/00286c9cf63b5eba9534ef7639805545ec40eb03))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Note that crates have been renamed from `git-*` to `gix-*`. ([`e14dc7d`](https://github.com/Byron/gitoxide/commit/e14dc7d475373d2c266e84ff8f1826c68a34ab92))
    - Rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - Adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - Adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
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
    - Adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - Adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
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
    - `set_current_dir()` to change the CWD and reset it to previous version on drop. ([`fb68bff`](https://github.com/Byron/gitoxide/commit/fb68bffcb26582d508db946f72234bfd847a3a11))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Remove `hex_to_id()` and add various `*_standalone()` versions of existing methods. ([`21bd607`](https://github.com/Byron/gitoxide/commit/21bd6075ca36ca49b3c85d0431ec11f68e6e9f9c))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - Thanks clippy ([`b34c9fe`](https://github.com/Byron/gitoxide/commit/b34c9fe58223862712eacc1cb7353e497a4b9778))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - Adapt to changes in `git-worktree` ([`5a97bb5`](https://github.com/Byron/gitoxide/commit/5a97bb5365573895500f0adeb73c482b797051c4))
    - Merge branch 'adjustments-for-cargo' ([`f8c562a`](https://github.com/Byron/gitoxide/commit/f8c562a559e6dc3377583cc7200585dad7c3d481))
    - Release git-testtools v0.10.0 ([`926ba5b`](https://github.com/Byron/gitoxide/commit/926ba5bf1a5f1b665e0791d12496b8a88bf60be5))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Merge branch 'git-lfs-improvements' ([`4c1685b`](https://github.com/Byron/gitoxide/commit/4c1685b971bb18117897a2c958ac2434bcb4f9e8))
    - Merge branch 'jpgrayson/main' ([`b242853`](https://github.com/Byron/gitoxide/commit/b242853abd790e5234b2f18b4aaeddb8f6f4d36f))
    - Disable tag.gpgSign in test scripts ([`1ce3190`](https://github.com/Byron/gitoxide/commit/1ce3190000f6211ce31468c7603d491bb5b90293))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - Merge branch 'main' into http-config ([`f4ff821`](https://github.com/Byron/gitoxide/commit/f4ff821fd4233dd1dc1a449af4d4600becf3b4ac))
    - Merge branch 'async-fetch' ([`0c9c48b`](https://github.com/Byron/gitoxide/commit/0c9c48b3b91a1396eb1796f288a2cb10380d1f14))
    - Let's be very conservative regarding maximum lock times ([`ba83945`](https://github.com/Byron/gitoxide/commit/ba83945bf885fd14b841323655991554af8b33d1))
    - This should work on windows (when launching the git-daemon) ([`52f4095`](https://github.com/Byron/gitoxide/commit/52f4095812f311abeb0184bfb70b133de64a6b62))
    - Make sure we can shut-down the daemon by starting it directly ([`4924b33`](https://github.com/Byron/gitoxide/commit/4924b33b40fa874ec3cc22476680ffce3eb30c84))
    - `spawn_git_daemon()` to spawn a git daemon hosting a working directoy… ([`221f137`](https://github.com/Byron/gitoxide/commit/221f1374aa004a76693cfb1529daab930a5a9dd7))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - Thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Assure the 'file' protocol is always allowed ([`7086101`](https://github.com/Byron/gitoxide/commit/7086101d3950b3e5ecb143b78185f2988cfb8fe8))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - Merge branch 'main' into fetch-pack ([`93917cb`](https://github.com/Byron/gitoxide/commit/93917cb6ecbb30daf3d20bb5a7c65e12211f084f))
    - Increase the waiting time on MacOS for file base locks ([`67777a8`](https://github.com/Byron/gitoxide/commit/67777a81f8d9d0335475e4fe4cbf770c328bd24f))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`2571831`](https://github.com/Byron/gitoxide/commit/2571831e5939bf4ea6f19537b0c1ccd71dc99088))
    - Merge branch 'main' into filter-refs-by-spec ([`56ba481`](https://github.com/Byron/gitoxide/commit/56ba481f4c48f74f10397feb1b6dc3d7dd3704fb))
    - Merge branch 'joelparkerhenderson/main' ([`239cb8a`](https://github.com/Byron/gitoxide/commit/239cb8a7c25f89ad087f201982585ab4c904c77b))
    - Fix format ([`1b00ab1`](https://github.com/Byron/gitoxide/commit/1b00ab1d2a38e0ee33570714760a21cc8ca3785e))
    - Fix git_version_from_bytes to handle trailing newline ([`14e4e66`](https://github.com/Byron/gitoxide/commit/14e4e66fe064114f3d9f1dc07ce34497abf8374e))
    - Merge branch 'main' into filter-refs-by-spec ([`a36c05d`](https://github.com/Byron/gitoxide/commit/a36c05d281269f3f8b297e7adc463bfb3c306663))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-worktree v0.4.3, git-testtools v0.8.0 ([`b2e4bf2`](https://github.com/Byron/gitoxide/commit/b2e4bf2c11ff2c3c32efcb91837fb5677714bdf9))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Thanks clippy ([`9aa8277`](https://github.com/Byron/gitoxide/commit/9aa827785c25e63dd1b351a7cc553f140fb93c2e))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Fix CI for good ([`e0c0b8c`](https://github.com/Byron/gitoxide/commit/e0c0b8c7c1898b2bc11a915e8e4fb8426295ccbb))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Add docs related to archives. ([`f409a2a`](https://github.com/Byron/gitoxide/commit/f409a2ae88f2b0d80c7d160563c07935993203a6))
    - Add documentation to test-tools. ([`074b283`](https://github.com/Byron/gitoxide/commit/074b2833d15c8483bd89e4bde4486c0c7df14637))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - Thanks clippy ([`49f5a54`](https://github.com/Byron/gitoxide/commit/49f5a5415c119267ea37e20fb198df80f621cbde))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`6cd365e`](https://github.com/Byron/gitoxide/commit/6cd365e2cf6851f5cdecc22f3b1667440ad011b0))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Thanks clippy ([`60cf67c`](https://github.com/Byron/gitoxide/commit/60cf67cb081b91932d9943b9c525cac2c0cf0782))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Set the time to wait for lock to longest expected runtime of fixture scripts ([`eea3988`](https://github.com/Byron/gitoxide/commit/eea3988462a61e8a64d646a15d062d13fdbfb615))
    - More robust archive creation on windows ([`e7b2d8f`](https://github.com/Byron/gitoxide/commit/e7b2d8f446b41b26b518abf7d1b048605ef2bbe8))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Thanks clippy ([`658862e`](https://github.com/Byron/gitoxide/commit/658862eeb042073632f5a3f203e264a47151d454))
    - Thanks clippy ([`c8d218c`](https://github.com/Byron/gitoxide/commit/c8d218c6399f52fb1a57eca22005196d1c686774))
    - Release git-testtools v0.6.0 ([`45386a0`](https://github.com/Byron/gitoxide/commit/45386a0b135656681dbdf8c47ad888b50e68f151))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Thanks clippy ([`1038dab`](https://github.com/Byron/gitoxide/commit/1038dab842b32ec1359a53236b241a91427ccb65))
    - Add `fixture_bytes` to test tools ([`85e3820`](https://github.com/Byron/gitoxide/commit/85e3820caa106a32c3406fd1e9e4c67fb0033bc5))
    - Commit to using 'unicode' feature of bstr as git-object wants it too ([`471fa62`](https://github.com/Byron/gitoxide/commit/471fa62b142ba744541d7472464d62826f5c6b93))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Ensure tests use 'merge.ff false' and recreate fixtures on each run ([`1d5ab44`](https://github.com/Byron/gitoxide/commit/1d5ab44145ccbc2064ee8cc7acebb62db82c45aa))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - Merge branch 'Byron:main' into main ([`dc58eca`](https://github.com/Byron/gitoxide/commit/dc58eca510e5a067acdeaad4b595a34b4598a0cd))
    - Release git-testtools v0.5.0 ([`86e0a92`](https://github.com/Byron/gitoxide/commit/86e0a92c7dc3b69a766aeac1b675b148d61a7ec5))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - [pack] refactor ([`9ee1e22`](https://github.com/Byron/gitoxide/commit/9ee1e22fa5c5d97ff626f0dfc44706272433bfef))
    - [ref] packed refs header line parsing ([`fde5543`](https://github.com/Byron/gitoxide/commit/fde5543ad22395e27266db02a5442a33d16e68c5))
    - [tools] fix create writable fixture ([`bf7783d`](https://github.com/Byron/gitoxide/commit/bf7783dd9ccc9ac433b978b9dded0d38f7351494))
    - [ref] on the way towards realistic transactions… ([`c808cb1`](https://github.com/Byron/gitoxide/commit/c808cb17b2fea12e018fabb789862e9b7703e49b))
    - [ref] on the way to setup the first transaction test ([`29c0b51`](https://github.com/Byron/gitoxide/commit/29c0b51625e2c7e3a8d60075bb925126a024dc83))
    - Bump once_cell from 1.7.2 to 1.8.0 ([`bd323d9`](https://github.com/Byron/gitoxide/commit/bd323d911b6becf8b379343c6ef56ec46e28fa28))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - Manually fix crc in tooling ([`48fa9bc`](https://github.com/Byron/gitoxide/commit/48fa9bc80876a0186f43add6c6d3477385241f5e))
    - Bump crc from 1.8.1 to 2.0.0 ([`07f08ac`](https://github.com/Byron/gitoxide/commit/07f08ac1ea04ec278993ad1a5fc1d4f243bf8eb7))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
    - Prepare test utilities for release… ([`d35e654`](https://github.com/Byron/gitoxide/commit/d35e654747f96cec93bdecd1314ce325129cbc44))
    - [tree-diff] Beginning of more nested test-suite… ([`b8a90e7`](https://github.com/Byron/gitoxide/commit/b8a90e7c9347b0eefdbef6f4c724cc0561cd79c9))
    - Fix debug assert, thanks gitpython ([`fe954b9`](https://github.com/Byron/gitoxide/commit/fe954b9f6d26bd8629f24a01bd2a06f9800deed0))
    - Revert "FAIL: try to disable GPG signing with environment variables…" ([`e326352`](https://github.com/Byron/gitoxide/commit/e326352eec7bd1aae13f770328979e5730ffc32b))
    - Try to disable GPG signing with environment variables… ([`29bf8ca`](https://github.com/Byron/gitoxide/commit/29bf8ca8399b6d4941aa242b9f08c74e59a179bb))
    - Thanks, cargo audit ([`4f293f5`](https://github.com/Byron/gitoxide/commit/4f293f5036c44a69ccacf102d35202adad83bbe0))
    - Thanks clippy ([`002792a`](https://github.com/Byron/gitoxide/commit/002792a8bc2512c92c16fd28662c26c9b3a12572))
    - Set environment in testtools to freeze repositories generation scripts ([`eaad3ab`](https://github.com/Byron/gitoxide/commit/eaad3ab69338115439a553ba1062160dc3a08082))
    - Faster repeated tests if fixtures don't change ([`792277f`](https://github.com/Byron/gitoxide/commit/792277f241446086dd6c9b78f688363d4e66e5a7))
    - Allow the use of shared test utilities across crates ([`b117626`](https://github.com/Byron/gitoxide/commit/b117626df6da714c24d2b7914301678e89d2d0cb))
    - The first test with the new and nice and cheap journey test tool ([`d3c99e1`](https://github.com/Byron/gitoxide/commit/d3c99e1cf3125ab107e12718b39ac9b7c9a9165c))
</details>

## 0.10.0 (2022-12-28)

### New Features

 - <csr-id-15ecd841cfe7c77bbdfdfa232dd51a44c4940bbc/> allow execution of scripts without 'bash'.
   This works by trying to execute the file directly, and on failure, use 'bash'
   as interpreter.
   
   That way we are finally able to support a wider variety of fixture generators
   and make the crate more useful to a wieder audience.
 - <csr-id-221f1374aa004a76693cfb1529daab930a5a9dd7/> `spawn_git_daemon()` to spawn a git daemon hosting a working directoy…
   …with support for multiple at a time thanks to port selection (allowing
   tests to run in parallel) as well as auto-kill on drop.
 - <csr-id-67777a81f8d9d0335475e4fe4cbf770c328bd24f/> increase the waiting time on MacOS for file base locks
   It appears that these workers run into timeouts more and more,
   they got slower or maybe there are just more tests.
 - <csr-id-09da4c5eeff5c6657beb9c53c168f90e74d6f758/> add `Env::unset()` for convenience

### Bug Fixes

 - <csr-id-761b7d71977a5aa4876010faa61ab88f0dba6eab/> don't overwrite unexpanded `gix-lfs` pointer files.
   It's possible for those with incomplete `gix-lfs` installations
   (and many more situations) to end up in a spot where pointer files
   aren't expanded. If we overwrite the with archives, files look
   changed which can be confusing and lead to even bigger messes
   to happen.
   
   Now we don't overwrite those files anyomre.
 - <csr-id-1ce3190000f6211ce31468c7603d491bb5b90293/> Disable tag.gpgSign in test scripts
   This is done for the same reason that commit.gpgsign is disabled for test
   scripts. It prevents test failures if the user has tag.gpgsign enabled in
   their global git config when invoking tests.

### Changed (BREAKING)

 - <csr-id-dbf6c8c87cdca8169ac01aa89aefe56a33215142/> rename `scripted_fixture_*` to not contain 'repo' in the name.
   Further make clear in the documentation that `bash` is used to execute
   the fixture scripts, previously it wasn't even implied and got
   lost in history.

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

## 0.9.0 (2022-09-20)

### Bug Fixes

 - <csr-id-cba9edeb403aae4d77087de4167cbabe72525d92/> Allow multiple scripts to run at the same time, if they are not the same.
   Previously, per integration test and thus per crate, one would
   effectively only be able to run a single script at a time because of the
   global identity lock. This was required previously before the additional
   file based lock, per script name, was introduced.
   
   This is now fixed by dropping the lock after the script identity was
   obtained.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.8.0 (2022-08-27)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-231785644194cd3be0b0dab06224a39ecf0ed714/> Provide `GIT_VERSION` information along with a way to skip a test based on it.
 - <csr-id-654b521323a5822cbb86e57bee159d90576fa5ff/> expose `on_ci` in the top-level.
 - <csr-id-449b6c1555fc2832c712ba51cd41ab9ed79e0b15/> Allow to re-execute scripts into temp directories.
   This is important in cases where the files created by the script
   contain absolute mentions of locations. That way, when copying
   files over, the test might accidentally return to the original
   read-only location, and write into it making future test runs fail.
 - <csr-id-f1635c3ee36678cff9f26135946c281bf4a75331/> publicly accessible `Result` type

### Bug Fixes

 - <csr-id-004dab17deab4c360adb5ac428f6b4951c974fe3/> `_with_args(…)` functions now allow non-static strings

## v0.6.0 (2022-04-04)

<csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/>

### Test

 - <csr-id-1d5ab44145ccbc2064ee8cc7acebb62db82c45aa/> ensure tests use 'merge.ff false' and recreate fixtures on each run

## v0.5.0 (2021-08-24)

## v0.4.0 (2021-08-11)

## v0.3.0 (2021-06-07)

## v0.1.0 (2021-04-30)

<csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/>

### Other

 - <csr-id-29bf8ca8399b6d4941aa242b9f08c74e59a179bb/> try to disable GPG signing with environment variables…
   …but it's not picked up at all even though it's definitely present.

