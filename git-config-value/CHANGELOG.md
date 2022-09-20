# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-5f675d387e52a75ff7bd17a38516ce9778ea6b7e/> `Boolean::try_from(OsString)`
   This makes getting booleans from the environment easier.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 21 calendar days.
 - 21 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - `Boolean::try_from(OsString)` ([`5f675d3`](https://github.com/Byron/gitoxide/commit/5f675d387e52a75ff7bd17a38516ce9778ea6b7e))
    - fix windows tests ([`0f11a6d`](https://github.com/Byron/gitoxide/commit/0f11a6dea937903d40833037d063bb82e224d66d))
 * **Uncategorized**
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
    - Merge branch 'main' into filter-refs-by-spec ([`56ba481`](https://github.com/Byron/gitoxide/commit/56ba481f4c48f74f10397feb1b6dc3d7dd3704fb))
    - Merge branch 'main' into filter-refs-by-spec ([`a36c05d`](https://github.com/Byron/gitoxide/commit/a36c05d281269f3f8b297e7adc463bfb3c306663))
</details>

## v0.7.0 (2022-08-29)

### Changed

 - <csr-id-5ad296577d837b0699b4718fa2be3d0978c4e342/> `git-config` now uses `git-config-value`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - add changelog ([`c396ba1`](https://github.com/Byron/gitoxide/commit/c396ba17f3f674c3af7460534860fc0dc462d401))
    - `git-config` now uses `git-config-value`. ([`5ad2965`](https://github.com/Byron/gitoxide/commit/5ad296577d837b0699b4718fa2be3d0978c4e342))
    - port tests over as well ([`9b28df2`](https://github.com/Byron/gitoxide/commit/9b28df22b858b6f1c9ca9b07a5a1c0cc300b50f0))
    - copy all value code from git-config to the dedicated crate ([`edb1162`](https://github.com/Byron/gitoxide/commit/edb1162e284e343e2c575980854b8292de9c968f))
    - add new git-config-value crate ([`f87edf2`](https://github.com/Byron/gitoxide/commit/f87edf26c1cb795142fbe95e12c0dfc1166e4233))
 * **Uncategorized**
    - Release git-config-value v0.7.0 ([`21c0ab9`](https://github.com/Byron/gitoxide/commit/21c0ab9c60ee317f574633081354351b0c7e5d0e))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
</details>

