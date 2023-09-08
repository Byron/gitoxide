# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.0 (2023-09-08)

### Bug Fixes (BREAKING)

 - <csr-id-072ee32f693a31161cd6a843da6582d13efbb20b/> use `dyn` trait where possible.
   This reduces compile time due to avoiding duplication.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 17 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelogs for release ([`375db06`](https://github.com/Byron/gitoxide/commit/375db06a8442378c3f7a922fae38e2a6694d9d04))
    - Merge branch `dyn`ification ([`f658fcc`](https://github.com/Byron/gitoxide/commit/f658fcc52dc2200ae34ca53dc10be97fb9012057))
    - Use `dyn` trait where possible. ([`072ee32`](https://github.com/Byron/gitoxide/commit/072ee32f693a31161cd6a843da6582d13efbb20b))
    - Merge branch 'gix-submodule' ([`363ee77`](https://github.com/Byron/gitoxide/commit/363ee77400805f473c9ad66eadad9214e7ab66f4))
</details>

## 0.9.0 (2023-08-22)

<csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/>
<csr-id-5b5983a9686e9fe61a29e9e1b9e905cd4dbd296a/>

### Chore

 - <csr-id-229bd4899213f749a7cc124aa2b82a1368fba40f/> don't call crate 'WIP' in manifest anymore.

### Other

 - <csr-id-5b5983a9686e9fe61a29e9e1b9e905cd4dbd296a/> make clear that `normalize()` does not touch duplicate path separators nor single `.`.

### New Features (BREAKING)

 - <csr-id-df83d746ff44ae192b7c69356624ff8b4cc61dcd/> remove `Spec` type in favor of `gix-pathspec::Pattern`.
   The latter isn't included to keep concerns and crates separate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 12 calendar days.
 - 30 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.7.3, gix-hash v0.12.0, gix-features v0.33.0, gix-actor v0.25.0, gix-object v0.35.0, gix-path v0.9.0, gix-glob v0.11.0, gix-quote v0.4.7, gix-attributes v0.17.0, gix-command v0.2.9, gix-packetline-blocking v0.16.5, gix-filter v0.3.0, gix-fs v0.5.0, gix-commitgraph v0.19.0, gix-hashtable v0.3.0, gix-revwalk v0.6.0, gix-traverse v0.31.0, gix-worktree-stream v0.3.0, gix-archive v0.3.0, gix-config-value v0.13.0, gix-tempfile v8.0.0, gix-lock v8.0.0, gix-ref v0.35.0, gix-sec v0.9.0, gix-config v0.28.0, gix-prompt v0.6.0, gix-url v0.22.0, gix-credentials v0.18.0, gix-diff v0.34.0, gix-discover v0.23.0, gix-ignore v0.6.0, gix-bitmap v0.2.7, gix-index v0.22.0, gix-mailmap v0.17.0, gix-negotiate v0.6.0, gix-pack v0.41.0, gix-odb v0.51.0, gix-pathspec v0.1.0, gix-packetline v0.16.5, gix-transport v0.35.0, gix-protocol v0.38.0, gix-revision v0.20.0, gix-refspec v0.16.0, gix-submodule v0.2.0, gix-worktree v0.24.0, gix-worktree-state v0.1.0, gix v0.52.0, gitoxide-core v0.31.0, gitoxide v0.29.0, safety bump 41 crates ([`30b2761`](https://github.com/Byron/gitoxide/commit/30b27615047692d3ced1b2d9c2ac15a80f79fbee))
    - Update changelogs prior to release ([`f23ea88`](https://github.com/Byron/gitoxide/commit/f23ea8828f2d9ba7559973daca388c9591bcc5fc))
    - Don't call crate 'WIP' in manifest anymore. ([`229bd48`](https://github.com/Byron/gitoxide/commit/229bd4899213f749a7cc124aa2b82a1368fba40f))
    - Merge branch 'pathspec-matching' ([`9f4dfe0`](https://github.com/Byron/gitoxide/commit/9f4dfe0f0b948280692916b596923959ea2fd9da))
    - Remove `Spec` type in favor of `gix-pathspec::Pattern`. ([`df83d74`](https://github.com/Byron/gitoxide/commit/df83d746ff44ae192b7c69356624ff8b4cc61dcd))
    - Make clear that `normalize()` does not touch duplicate path separators nor single `.`. ([`5b5983a`](https://github.com/Byron/gitoxide/commit/5b5983a9686e9fe61a29e9e1b9e905cd4dbd296a))
</details>

## 0.8.4 (2023-07-22)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 23 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`16295b5`](https://github.com/Byron/gitoxide/commit/16295b58e2581d2e8b8b762816f52baabe871c75))
    - Release gix-date v0.7.1, gix-hash v0.11.4, gix-trace v0.1.3, gix-features v0.32.0, gix-actor v0.24.0, gix-validate v0.7.7, gix-object v0.33.0, gix-path v0.8.4, gix-glob v0.10.0, gix-quote v0.4.6, gix-attributes v0.15.0, gix-command v0.2.7, gix-packetline-blocking v0.16.3, gix-filter v0.1.0, gix-fs v0.4.0, gix-chunk v0.4.4, gix-commitgraph v0.18.0, gix-hashtable v0.2.4, gix-revwalk v0.4.0, gix-traverse v0.30.0, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.4, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.0, gix-sec v0.8.4, gix-prompt v0.5.3, gix-url v0.21.0, gix-credentials v0.17.0, gix-diff v0.33.0, gix-discover v0.22.0, gix-ignore v0.5.0, gix-bitmap v0.2.6, gix-index v0.21.0, gix-mailmap v0.16.0, gix-negotiate v0.5.0, gix-pack v0.40.0, gix-odb v0.50.0, gix-packetline v0.16.4, gix-transport v0.34.0, gix-protocol v0.36.0, gix-revision v0.18.0, gix-refspec v0.14.0, gix-worktree v0.22.0, gix v0.49.1 ([`5cb3589`](https://github.com/Byron/gitoxide/commit/5cb3589b74fc5376e02cbfe151e71344e1c417fe))
    - Update changelogs prior to release ([`2fc66b5`](https://github.com/Byron/gitoxide/commit/2fc66b55097ed494b72d1af939ba5561f71fde97))
    - Update license field following SPDX 2.1 license expression standard ([`9064ea3`](https://github.com/Byron/gitoxide/commit/9064ea31fae4dc59a56bdd3a06c0ddc990ee689e))
</details>

## 0.8.3 (2023-06-29)

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
    - Release gix-features v0.31.1, gix-path v0.8.3, gix v0.48.0 ([`9ca3464`](https://github.com/Byron/gitoxide/commit/9ca346462806671fbc49643a87cea25ab0da3be8))
    - Prepare changelogs once more ([`4bf355a`](https://github.com/Byron/gitoxide/commit/4bf355a8c6a7dbcdb49105af3208d56a0ed8628d))
    - Adjust `gix-trace` to the latest version. ([`353df4b`](https://github.com/Byron/gitoxide/commit/353df4bf59c7aa98da48bcdcc299f947d9449f55))
</details>

## 0.8.2 (2023-06-22)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint

### New Features

 - <csr-id-3cffa268460eb2d41bd6a30d45778b88db4ec602/> provide basic `tracing` spans for common operations.
   This is just the beginning and more crates will integrate with it over time.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 10 calendar days.
 - 15 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`ea9f942`](https://github.com/Byron/gitoxide/commit/ea9f9424e777f10da0e33bb9ffbbefd01c4c5a74))
    - Prepare changelogs prior to release ([`18b0a37`](https://github.com/Byron/gitoxide/commit/18b0a371941aa2d4d62512437d5daa351ba99ffd))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Release gix-trace v0.1.0 ([`2ab69c6`](https://github.com/Byron/gitoxide/commit/2ab69c6fd142fcbbf6df3f981b1550b0c8167a04))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Provide basic `tracing` spans for common operations. ([`3cffa26`](https://github.com/Byron/gitoxide/commit/3cffa268460eb2d41bd6a30d45778b88db4ec602))
    - Merge branch 'help-874-redundant-closures' ([`fe59956`](https://github.com/Byron/gitoxide/commit/fe59956ad667303a923d7cfd9ffd72283df41d78))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`bcad5c2`](https://github.com/Byron/gitoxide/commit/bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d))
</details>

## 0.8.1 (2023-06-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 12 calendar days.
 - 40 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-date v0.5.1, gix-hash v0.11.2, gix-features v0.30.0, gix-actor v0.21.0, gix-path v0.8.1, gix-glob v0.8.0, gix-quote v0.4.4, gix-attributes v0.13.0, gix-chunk v0.4.2, gix-commitgraph v0.16.0, gix-config-value v0.12.1, gix-fs v0.2.0, gix-tempfile v6.0.0, gix-utils v0.1.2, gix-lock v6.0.0, gix-validate v0.7.5, gix-object v0.30.0, gix-ref v0.30.0, gix-sec v0.8.1, gix-config v0.23.0, gix-command v0.2.5, gix-prompt v0.5.1, gix-url v0.19.0, gix-credentials v0.15.0, gix-diff v0.30.0, gix-discover v0.19.0, gix-hashtable v0.2.1, gix-ignore v0.3.0, gix-bitmap v0.2.4, gix-traverse v0.26.0, gix-index v0.17.0, gix-mailmap v0.13.0, gix-revision v0.15.0, gix-negotiate v0.2.0, gix-pack v0.36.0, gix-odb v0.46.0, gix-packetline v0.16.2, gix-transport v0.32.0, gix-protocol v0.33.0, gix-refspec v0.11.0, gix-worktree v0.18.0, gix v0.45.0, safety bump 29 crates ([`9a9fa96`](https://github.com/Byron/gitoxide/commit/9a9fa96fa8a722bddc5c3b2270b0edf8f6615141))
    - Prepare changelogs prior to release ([`8f15cec`](https://github.com/Byron/gitoxide/commit/8f15cec1ec7d5a9d56bb158f155011ef2bb3539b))
    - Merge branch 'auto-clippy' ([`dbf8aa1`](https://github.com/Byron/gitoxide/commit/dbf8aa19d19109195d0274928eae4b94f248cd88))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include custom clippy settings ([`b057500`](https://github.com/Byron/gitoxide/commit/b057500dd3e6b75be3ebcd258cda0b946bedd9e1))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
</details>

## 0.8.0 (2023-04-27)

A maintenance release without user-facing changes.

### Changed (BREAKING)

 - <csr-id-94564df0b5edcb41f1088c6db04add851c66b2af/> Moved `home_dir()` to `env::home_dir()` and `env_var()` to `env::var()`.
   Please note that this change was previously and erroneously not declared as breaking.

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
    - Moved `home_dir()` to `env::home_dir()` and `env_var()` to `env::var()`. ([`94564df`](https://github.com/Byron/gitoxide/commit/94564df0b5edcb41f1088c6db04add851c66b2af))
</details>

## 0.7.4 (2023-04-26)

**YANKED** due to accidental breaking changes.

### New Features

 - <csr-id-1e73f3c319caec0298cdd8c34f29ff78ff4060ab/> add `join_bstr_unix_pathsep(base, component)`.
   It's useful to have to avoid certain conversions to happen otherwise.
 - <csr-id-0d340f4fdeff1576460d43ca2210b11f0641c5dd/> add `xdg_config_home()`, installation_config` and `installation_config_prefix()` functions.

### Bug Fixes

 - <csr-id-bd1ae0db32191fb96d6b2a55b9fcb635d1652c1f/> `join_bstr_unix_pathsep()` works more suitably if base path is empty.
 - <csr-id-13edfe96696636c30040ec81ebc4e235ac689429/> use `home` in `env::home_dir()`
 - <csr-id-ec049fec7f789f6b55ca18a691388c44778603d4/> use `home` in `env::home_dir()`.
   This way it should work better on windows as it now uses the home_dir implementation
   of a crate used by `cargo`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 13 calendar days.
 - 27 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.11.1, gix-path v0.7.4, gix-glob v0.6.0, gix-attributes v0.11.0, gix-config-value v0.11.0, gix-fs v0.1.1, gix-tempfile v5.0.3, gix-utils v0.1.1, gix-lock v5.0.1, gix-object v0.29.1, gix-ref v0.28.0, gix-sec v0.7.0, gix-config v0.21.0, gix-prompt v0.4.0, gix-url v0.17.0, gix-credentials v0.13.0, gix-diff v0.29.0, gix-discover v0.17.0, gix-hashtable v0.2.0, gix-ignore v0.1.0, gix-bitmap v0.2.3, gix-traverse v0.25.0, gix-index v0.16.0, gix-mailmap v0.12.0, gix-pack v0.34.0, gix-odb v0.44.0, gix-packetline v0.16.0, gix-transport v0.30.0, gix-protocol v0.31.0, gix-revision v0.13.0, gix-refspec v0.10.0, gix-worktree v0.16.0, gix v0.44.0, safety bump 7 crates ([`91134a1`](https://github.com/Byron/gitoxide/commit/91134a11c8ba0e942f692488ec9bce9fa1086324))
    - Prepare changelogs prior to release ([`30a1a71`](https://github.com/Byron/gitoxide/commit/30a1a71f36f24faac0e0b362ffdfedea7f9cdbf1))
    - Merge branch 'index-entries-attrs' ([`f37a930`](https://github.com/Byron/gitoxide/commit/f37a930aefa27e67f0b693ba9669cc26d49044fa))
    - `join_bstr_unix_pathsep()` works more suitably if base path is empty. ([`bd1ae0d`](https://github.com/Byron/gitoxide/commit/bd1ae0db32191fb96d6b2a55b9fcb635d1652c1f))
    - Merge branch 'attributes-cache' ([`3456c84`](https://github.com/Byron/gitoxide/commit/3456c845dfeedd2fa96b4313b1a84c8cbe9433c5))
    - Add `join_bstr_unix_pathsep(base, component)`. ([`1e73f3c`](https://github.com/Byron/gitoxide/commit/1e73f3c319caec0298cdd8c34f29ff78ff4060ab))
    - Merge branch 'utkarshgupta137/main' ([`74cb5ee`](https://github.com/Byron/gitoxide/commit/74cb5ee03d7a5fbba312c0a5c782489a6fc039a7))
    - Use `home` in `env::home_dir()` ([`13edfe9`](https://github.com/Byron/gitoxide/commit/13edfe96696636c30040ec81ebc4e235ac689429))
    - Revert "fix: use `home` in `env::home_dir()`." ([`222ece2`](https://github.com/Byron/gitoxide/commit/222ece2a2fb639cbcdec639b06fa1bc2cc36c972))
    - Merge branch 'utkarshgupta137/main' ([`5092c59`](https://github.com/Byron/gitoxide/commit/5092c59084250af45e5c81d1ec5219428249974d))
    - Use `home` in `env::home_dir()`. ([`ec049fe`](https://github.com/Byron/gitoxide/commit/ec049fec7f789f6b55ca18a691388c44778603d4))
    - Make fmt ([`5d2b5d0`](https://github.com/Byron/gitoxide/commit/5d2b5d02c3869e07dc2507a8f2519ee1df633df7))
    - Merge branch 'main' into dev ([`23ee47f`](https://github.com/Byron/gitoxide/commit/23ee47fb24c197f8437bd426544b2aa74e005bdc))
    - Merge branch 'worktree-stack' ([`3d47919`](https://github.com/Byron/gitoxide/commit/3d47919c1a2f83fc7c1fd7ae590d098057a22626))
    - Add `xdg_config_home()`, installation_config` and `installation_config_prefix()` functions. ([`0d340f4`](https://github.com/Byron/gitoxide/commit/0d340f4fdeff1576460d43ca2210b11f0641c5dd))
</details>

## 0.7.3 (2023-03-30)

### Bug Fixes

 - <csr-id-d1bd513f27e17787eb223f7b0521f954c518153e/> $HOME detection on windows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 37 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-path v0.7.3, gix-config-value v0.10.2, gix-config v0.20.1, gix-discover v0.16.2, gix-index v0.15.1, gix-odb v0.43.1, gix-packetline v0.15.1, gix-protocol v0.30.2, gix-worktree v0.15.2, gix v0.43.1 ([`38eed1d`](https://github.com/Byron/gitoxide/commit/38eed1d06e7cbb8fbcd54b2cad3163ca45e0baf1))
    - Merge branch 'pascalkuthe/main' ([`d47cebe`](https://github.com/Byron/gitoxide/commit/d47cebe3b23080c45829cb307b867220e3af20db))
    - Refactor ([`d1e5e12`](https://github.com/Byron/gitoxide/commit/d1e5e12d54f79c030325860838c1cfadac1a7ac5))
    - $HOME detection on windows ([`d1bd513`](https://github.com/Byron/gitoxide/commit/d1bd513f27e17787eb223f7b0521f954c518153e))
</details>

## 0.7.2 (2023-02-20)

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

## 0.7.1 (2023-02-17)

<csr-id-37cab07f283a368f323604372c84475d73d6c258/>
<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>
<csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/>
<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Refactor (BREAKING)

 - <csr-id-37cab07f283a368f323604372c84475d73d6c258/> Rename absolutize() to normalize()
   The name absolutize implies strongly that the returned path will be
   absolute, but the function only converts relative paths to absolute under a
   few specific circumstances.
   
   The new name, normalize(), is inspired by Python's os.path.normpath(),
   Java's java.nio.file.Path.normalize(), Node's Path.normalize(), and maybe
   some others which have similar semantics to this function.
 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

### Bug Fixes (BREAKING)

 - <csr-id-7dbab1c62c49822983c59be0443478f7b4fecbca/> `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>`
   Previously the function was willing to return an empty path despite it
   being invalid. With the `current_dir` being required, this won't be the
   case anymore and will yield logically consistent results in all cases.
   
   This forces the caller to deal with the relative path being invalid
   or crafted to produce some other path, maybe to bypass sanity checks.
 - <csr-id-c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce/> Remove `git-config` test utilities from `git-path`.

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
 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-dfa1e05d3c983f1e8b1cb3b80d03608341187883/> `realpath()` handles `cwd` internally
   This makes for more convenient usage in the common case.

### Refactor

 - <csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/> rename tests/convert/normalize.rs
   This renames the test module to match the new function name.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absoluteize' the path later (std::path::absolute()) is
   on the way for that.

### Bug Fixes

 - <csr-id-745d92636f8a3436ded0c9da21beb92182341998/> `.` substitution is only done if the input was relative.
   Previously it was possible to have `/a/b/../b` and a CWD of `/a/b`
   replaced with `.` even though that clearly isn't what the user provided.
   
   Now the `.` resubstitution only happens when it's in the interest
   of the caller.
 - <csr-id-92d5d133e17c6b79400ec57b55ccd5337f3796b7/> `normalize()` would fail to interpret `../` correctly and end up in an invalid path.
   This is now fixed and should never happen again thanks to the addition
   of a missing test.
 - <csr-id-9171adb796b38b08cae9bdd375b16a59a8166a1c/> Handle `.` specifically in `absolutize()`.
   Previously, absolutizing `./../../` would lead to one path component
   of the `../` to be ignored as `.` was popped successfully, not realizing
   that it is a no-op.
   
   This could lead to problems with repository discovery if `.` was passed.

### New Features

 - <csr-id-25e795f4fe858d646ae7a3c4706e14a3837c3e66/> Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`.
 - <csr-id-523418f69030faa0add6472b14333e9aafc69f56/> add support for `wasi`
   This allows path conversions there to be just as efficient as on unix.
   
   This was adopted from [a PR in the
   hexlix-editor](https://github.com/helix-editor/helix/pull/3890/files#diff-504515b66023120e75a921cd56a932aed76c0ff62593fbb69d92e0ef65089501R47).
 - <csr-id-f58a043273b8e15afd01aac71f33652783baf462/> add `is_absolute()` for git-style absolute checks
   This essentially means that starting slashes are always absolute, even
   on windows.
 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 189 commits contributed to the release over the course of 296 calendar days.
 - 20 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#422](https://github.com/Byron/gitoxide/issues/422), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - `absolutize_*(dir)` is now `absolutize(dir, Option<cwd>)` ([`de87657`](https://github.com/Byron/gitoxide/commit/de87657194ad976cc73ebcc13c231537b35b4195))
    - More robust absolutize-paths implementation ([`4800ebe`](https://github.com/Byron/gitoxide/commit/4800ebec42f9bb6298cb5b2efdab71d6baf3b1ba))
    - Add `absolutize_components()` ([`35f146a`](https://github.com/Byron/gitoxide/commit/35f146a8573dcc9a1de3230373c0cf0794c6b897))
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - Refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - Various name changes for more convenient API ([`5480159`](https://github.com/Byron/gitoxide/commit/54801592488416ef2bb0f34c5061b62189c35c5e))
    - Use bstr intead of [u8] ([`9380e99`](https://github.com/Byron/gitoxide/commit/9380e9990065897e318b040f49b3c9a6de8bebb1))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Copy all existing functions from git-features::path to git-path:: ([`725e198`](https://github.com/Byron/gitoxide/commit/725e1985dc521d01ff9e1e89b6468ef62fc09656))
    - Add empty git-path crate ([`8d13f81`](https://github.com/Byron/gitoxide/commit/8d13f81068b4663d322002a9617d39b307b63469))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - `realpath()` handles `cwd` internally ([`dfa1e05`](https://github.com/Byron/gitoxide/commit/dfa1e05d3c983f1e8b1cb3b80d03608341187883))
 * **[#422](https://github.com/Byron/gitoxide/issues/422)**
    - Prepare changelog ([`de2d587`](https://github.com/Byron/gitoxide/commit/de2d5874b8d75c53165a9fc3ed35e2b37142bf52))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - Add `is_absolute()` for git-style absolute checks ([`f58a043`](https://github.com/Byron/gitoxide/commit/f58a043273b8e15afd01aac71f33652783baf462))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - Update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
    - Add support for `wasi` ([`523418f`](https://github.com/Byron/gitoxide/commit/523418f69030faa0add6472b14333e9aafc69f56))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
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
    - Adjust to renaming of `git-pathspec` to `gix-pathspec` ([`37f7c6b`](https://github.com/Byron/gitoxide/commit/37f7c6b9070e118604aa3fc0b38530699dcfec6e))
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
    - Rename `git-path` to `gix-path` ([`9fe8e83`](https://github.com/Byron/gitoxide/commit/9fe8e8389c0ba677f31356c26a375e694e4d1f64))
    - Adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Merge branch 'git-pack-wasm' ([`4bc19d1`](https://github.com/Byron/gitoxide/commit/4bc19d104233a3e3d3d2768c0e9b9ad027cc34c0))
    - CI validates WASM support ([`0d4b804`](https://github.com/Byron/gitoxide/commit/0d4b804171acd307bdac6eecd3b49bd8b2fb2968))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - Prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - Fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - Thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - Prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - Thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - Make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'discovery-fix' ([`689752e`](https://github.com/Byron/gitoxide/commit/689752e67e8895b3d40d335e5778d8a90ec89c4c))
    - `.` substitution is only done if the input was relative. ([`745d926`](https://github.com/Byron/gitoxide/commit/745d92636f8a3436ded0c9da21beb92182341998))
    - `normalize()` would fail to interpret `../` correctly and end up in an invalid path. ([`92d5d13`](https://github.com/Byron/gitoxide/commit/92d5d133e17c6b79400ec57b55ccd5337f3796b7))
    - Merge branch 'path-normalize' ([`805329a`](https://github.com/Byron/gitoxide/commit/805329a0a5f6543bbc1d5885977b47bf7baa7f08))
    - Rename tests/convert/normalize.rs ([`8ab47bb`](https://github.com/Byron/gitoxide/commit/8ab47bbdac44c0fa738215d3cc457eb3b6f30504))
    - Rename absolutize() to normalize() ([`37cab07`](https://github.com/Byron/gitoxide/commit/37cab07f283a368f323604372c84475d73d6c258))
    - Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`. ([`25e795f`](https://github.com/Byron/gitoxide/commit/25e795f4fe858d646ae7a3c4706e14a3837c3e66))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - Prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'discover-rel-path' ([`5f908fb`](https://github.com/Byron/gitoxide/commit/5f908fb86857d565715b9b0b8b453b29273fb022))
    - Improve documentation to clarify intent ([`b8f73aa`](https://github.com/Byron/gitoxide/commit/b8f73aa5afe3f7aefa5627d7708e4c7e7da950a2))
    - Merge branch 'cwd-consistency' ([`ea7c6a3`](https://github.com/Byron/gitoxide/commit/ea7c6a3b069c9e13905b51b87538c57ba9182dca))
    - Adapt to changes in `git-discover` and `git-path` and `git-odb` ([`98c2501`](https://github.com/Byron/gitoxide/commit/98c250175a39598b9d37613c43dda2299da8eff3))
    - `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>` ([`7dbab1c`](https://github.com/Byron/gitoxide/commit/7dbab1c62c49822983c59be0443478f7b4fecbca))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Handle `.` specifically in `absolutize()`. ([`9171adb`](https://github.com/Byron/gitoxide/commit/9171adb796b38b08cae9bdd375b16a59a8166a1c))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - Prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - Fix docs ([`4f8e3b1`](https://github.com/Byron/gitoxide/commit/4f8e3b169e57d599439c7abc861c82c08bcd92e3))
    - Thanks clippy ([`7a2a31e`](https://github.com/Byron/gitoxide/commit/7a2a31e5758a2be8434f22cd9401ac00539f2bd9))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Generally avoid using `target_os = "windows"` in favor of `cfg(windows)` and negations ([`91d5402`](https://github.com/Byron/gitoxide/commit/91d54026a61c2aae5e3e1341d271acf16478cd83))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - Avoid unwraps in tests as they are now stable ([`efa1423`](https://github.com/Byron/gitoxide/commit/efa14234c352b6b8417f0a42fc946e88f2eb52d3))
    - Remove canonicalized-path abstraction ([`9496e55`](https://github.com/Byron/gitoxide/commit/9496e5512975825efebe0db86335d0d2dc8c9095))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Fix git-paths tests; improve error handling. ([`9c00504`](https://github.com/Byron/gitoxide/commit/9c0050451f634a54e610c86199b5d7d393378878))
    - Docs for git-path ([`a520092`](https://github.com/Byron/gitoxide/commit/a52009244c9b1059ebb3d5dd472c25f9c49691f3))
    - Remove `git-config` test utilities from `git-path`. ([`c9933c0`](https://github.com/Byron/gitoxide/commit/c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - Update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - Prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - Adjust git-path size limits ([`5ac8a3b`](https://github.com/Byron/gitoxide/commit/5ac8a3b58e0f61d4801a6f4dbd011f757208dbac))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - Remove forbid missing_docs ([`23acebb`](https://github.com/Byron/gitoxide/commit/23acebb8e9e53d89e7f629ab690253610358b0bb))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - Declare `git-path` usable ([`496594d`](https://github.com/Byron/gitoxide/commit/496594d2d8b4216b51cfbd97805834c71c030c75))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Fix create_symlink ([`714db70`](https://github.com/Byron/gitoxide/commit/714db70f02134c7f53dc7ba0461f43a0d6b659e9))
    - Add includeIf test with symlink. ([`5d74404`](https://github.com/Byron/gitoxide/commit/5d744049286632f3141ec07fa3f128093480d1c0))
    - Fix realpath tests. ([`0426f4d`](https://github.com/Byron/gitoxide/commit/0426f4deb5d73fd88529530f9a6d01ba55eeadc4))
    - Refactor real_path tests. ([`b696849`](https://github.com/Byron/gitoxide/commit/b696849e5fd210da397b0e7a3b26a63314d87607))
    - Refactor real_path tests. ([`8ade69f`](https://github.com/Byron/gitoxide/commit/8ade69fbddfa5d0be3bbe761210e49be647c3356))
    - Fix windows (probably) ([`c980014`](https://github.com/Byron/gitoxide/commit/c980014206ff071bc4f351416bb14995ac739e1b))
    - Thanks clippy ([`da13aff`](https://github.com/Byron/gitoxide/commit/da13affabe34c3d691b18a70ce61eb00319668c5))
    - Refactor ([`6bba054`](https://github.com/Byron/gitoxide/commit/6bba054a9a87219a7f94c155058fda5a3e6dffa6))
    - Turn recursion into loop ([`9b83c2c`](https://github.com/Byron/gitoxide/commit/9b83c2c233d41034796694d000bed10d45f40c92))
    - Refactor ([`1ca0540`](https://github.com/Byron/gitoxide/commit/1ca0540d170dcb8066a9141ce97631fcb9f2d5ae))
    - Refactor ([`1f6ecd2`](https://github.com/Byron/gitoxide/commit/1f6ecd2ba91a34171d708ab7cb9414e853face95))
    - Refactor ([`5efb972`](https://github.com/Byron/gitoxide/commit/5efb97251a9bf9e342d28bcbde27b0e69b0b7849))
    - Refactor ([`353c245`](https://github.com/Byron/gitoxide/commit/353c2455dc01cf342b1186f0be263a87952b70be))
    - Put `realpath` into its own module ([`d142e01`](https://github.com/Byron/gitoxide/commit/d142e01445ef545bd8284d3899d7e68f578943e9))
    - Refactor ([`50583f0`](https://github.com/Byron/gitoxide/commit/50583f083be7ba890f7727a6491cbacf8b87ebe4))
    - Rename `real_path()` to `realpath()` ([`478ff6c`](https://github.com/Byron/gitoxide/commit/478ff6caa630970847094fc11af10a6b69d72c34))
    - Refactor ([`8f1daf5`](https://github.com/Byron/gitoxide/commit/8f1daf55c0027ec124bc6672ec545275065af9a7))
    - Fix linux test ([`8a36810`](https://github.com/Byron/gitoxide/commit/8a368102c89161006cad343839105d3a5ff284e2))
    - Fix windows test. ([`1afb2da`](https://github.com/Byron/gitoxide/commit/1afb2daa6704cc0c2efd9437dff5518ea3e64429))
    - Temp ignore real_path_tests. ([`c2f5db9`](https://github.com/Byron/gitoxide/commit/c2f5db9a3fcc7bdcdd84cdda30d970bdcedaff2a))
    - Windows absolute path. ([`8dc33cc`](https://github.com/Byron/gitoxide/commit/8dc33ccd1f5886b5e3f23eac5d6381473c386c2f))
    - Windows absolute path. ([`070f8c7`](https://github.com/Byron/gitoxide/commit/070f8c79a54141d3b3064622ac7b528a24875d4f))
    - Windows absolute path. ([`cefc8fb`](https://github.com/Byron/gitoxide/commit/cefc8fbfbb591fe714ffd87f39d0a7ca00e4a754))
    - Windows absolute path. ([`31a71f3`](https://github.com/Byron/gitoxide/commit/31a71f37d596a3a7a7279d4b6e2508c32383b2b6))
    - Fix merge. ([`f2b46df`](https://github.com/Byron/gitoxide/commit/f2b46dfbf73387d4501a7bf5039cb80ac4cb8d9c))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Add custom tempdir in. ([`8bfd52a`](https://github.com/Byron/gitoxide/commit/8bfd52a65fcecb33ae69917a67c48027f8fb3dff))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - Temp ignore real_path_tests. ([`27f4bfc`](https://github.com/Byron/gitoxide/commit/27f4bfcb2fba45bd02d1977094acb31b7b989cac))
    - Windows fix. ([`ce0b408`](https://github.com/Byron/gitoxide/commit/ce0b408fcdeae80d6c9263955f70a00ead3841e1))
    - Windows fix. ([`25dd319`](https://github.com/Byron/gitoxide/commit/25dd319a2b46327fb553f824619311484726c742))
    - Windows fix. ([`61bc0e7`](https://github.com/Byron/gitoxide/commit/61bc0e776b9b02fdd36df6c0f54aecae63bf5895))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Handle windows path prefix. ([`1723236`](https://github.com/Byron/gitoxide/commit/1723236377db483b09f123a5c24c949afa285b8a))
    - Max symlinks exceeded test. ([`cfff300`](https://github.com/Byron/gitoxide/commit/cfff30075d87045bf9def697c417a3eb46b4b215))
    - Use thiserror in `real_path()` ([`2bd7a44`](https://github.com/Byron/gitoxide/commit/2bd7a441beb7e0a86169ec89ca56a8ba448fbf2b))
    - Input_path is Iterator. ([`c993d78`](https://github.com/Byron/gitoxide/commit/c993d7826fcf76ddaddffca619b4d35555b6636c))
    - Real_path wip ([`3890a61`](https://github.com/Byron/gitoxide/commit/3890a6149683663b16dccdc3b50e2aab7eb4e048))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Release git-path v0.1.0 ([`ca019fc`](https://github.com/Byron/gitoxide/commit/ca019fca03c4ea0d70fabbf09808732925b58077))
</details>

## 0.7.0 (2022-12-19)

<csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/>
<csr-id-37cab07f283a368f323604372c84475d73d6c258/>

### New Features

 - <csr-id-25e795f4fe858d646ae7a3c4706e14a3837c3e66/> Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`.

### Bug Fixes

 - <csr-id-745d92636f8a3436ded0c9da21beb92182341998/> `.` substitution is only done if the input was relative.
   Previously it was possible to have `/a/b/../b` and a CWD of `/a/b`
   replaced with `.` even though that clearly isn't what the user provided.
   
   Now the `.` resubstitution only happens when it's in the interest
   of the caller.
 - <csr-id-92d5d133e17c6b79400ec57b55ccd5337f3796b7/> `normalize()` would fail to interpret `../` correctly and end up in an invalid path.
   This is now fixed and should never happen again thanks to the addition
   of a missing test.

### Refactor

 - <csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/> rename tests/convert/normalize.rs
   This renames the test module to match the new function name.

### Refactor (BREAKING)

 - <csr-id-37cab07f283a368f323604372c84475d73d6c258/> Rename absolutize() to normalize()
   The name absolutize implies strongly that the returned path will be
   absolute, but the function only converts relative paths to absolute under a
   few specific circumstances.
   
   The new name, normalize(), is inspired by Python's os.path.normpath(),
   Java's java.nio.file.Path.normalize(), Node's Path.normalize(), and maybe
   some others which have similar semantics to this function.

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

### Bug Fixes (BREAKING)

 - <csr-id-7dbab1c62c49822983c59be0443478f7b4fecbca/> `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>`
   Previously the function was willing to return an empty path despite it
   being invalid. With the `current_dir` being required, this won't be the
   case anymore and will yield logically consistent results in all cases.
   
   This forces the caller to deal with the relative path being invalid
   or crafted to produce some other path, maybe to bypass sanity checks.

## 0.5.0 (2022-09-20)

### New Features

 - <csr-id-523418f69030faa0add6472b14333e9aafc69f56/> add support for `wasi`
   This allows path conversions there to be just as efficient as on unix.
   
   This was adopted from [a PR in the
   hexlix-editor](https://github.com/helix-editor/helix/pull/3890/files#diff-504515b66023120e75a921cd56a932aed76c0ff62593fbb69d92e0ef65089501R47).

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.2 (2022-08-29)

### New Features

 - <csr-id-f58a043273b8e15afd01aac71f33652783baf462/> add `is_absolute()` for git-style absolute checks
   This essentially means that starting slashes are always absolute, even
   on windows.

## 0.4.1 (2022-08-23)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Bug Fixes

 - <csr-id-9171adb796b38b08cae9bdd375b16a59a8166a1c/> Handle `.` specifically in `absolutize()`.
   Previously, absolutizing `./../../` would lead to one path component
   of the `../` to be ignored as `.` was popped successfully, not realizing
   that it is a no-op.
   
   This could lead to problems with repository discovery if `.` was passed.

## 0.4.0 (2022-07-22)

### Changed (BREAKING)

 - <csr-id-dfa1e05d3c983f1e8b1cb3b80d03608341187883/> `realpath()` handles `cwd` internally
   This makes for more convenient usage in the common case.

## 0.3.0 (2022-06-19)

### Bug Fixes (BREAKING)

 - <csr-id-c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce/> Remove `gix-config` test utilities from `gix-path`.

## 0.2.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.1.3 (2022-05-23)

A maintenance release without user-facing changes.

## 0.1.2 (2022-05-21)

A maintenance release without user-facing changes.

## 0.1.1 (2022-05-18)

<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>

### New Features

 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absolutize' the path later (std::path::absolute()) is
   on the way for that.

## 0.1.0 (2022-04-28)

<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>

### Refactor (BREAKING)

 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

## 0.0.0 (2022-03-31)

An empty crate without any content to reserve the name for the gitoxide project.

